#![allow(clippy::module_name_repetitions)]

use std::collections::BTreeMap;

use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;

#[derive(Debug, Clone)]
enum LinkDirection {
  Forward,
  Reverse,
}

/// Client-facing representation of a Trust Atom
/// We may support JSON in the future to allow for more complex data structures @TODO
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
pub struct TrustAtom {
  pub source: String,
  pub target: String,
  pub content: String,
  pub value: String,
  pub source_entry_hash: EntryHashB64,
  pub target_entry_hash: EntryHashB64,
  pub attributes: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtomInput {
  pub target: EntryHash,
  pub content: String,
  pub value: String,
  pub attributes: BTreeMap<String, String>,
}

#[hdk_entry(id = "restaurant", visibility = "public")]
#[derive(Clone)]
pub struct StringTarget(String);

const UNICODE_NUL_STR: &str = "\u{0}"; // Unicode NUL character
const UNICODE_NUL_BYTES: [u8; 1] = [0];
const LINK_TAG_HEADER: [u8; 2] = [197, 166]; // Unicode "Ŧ" // hex bytes: [0xC5][0xA6]
const LINK_TAG_ARROW_FORWARD: [u8; 3] = [226, 134, 146]; // Unicode "→" // hex bytes: [0xE2][0x86][0x92]
const LINK_TAG_ARROW_REVERSE: [u8; 3] = [226, 134, 169]; // Unicode "↩" // hex bytes: [0xE2][0x86][0xA9]

pub fn create(input: TrustAtomInput) -> ExternResult<()> {
  let agent_info = agent_info()?;
  let agent_address: EntryHash = agent_info.agent_initial_pubkey.into();

  let forward_link_tag = trust_atom_link_tag(&LinkDirection::Forward, &input.content, &input.value);

  create_link(
    agent_address.clone(),
    input.target.clone(),
    forward_link_tag,
  )?;

  let reverse_link_tag = trust_atom_link_tag(&LinkDirection::Reverse, &input.content, &input.value);
  create_link(input.target, agent_address, reverse_link_tag)?;

  Ok(())
}

fn trust_atom_link_tag(link_direction: &LinkDirection, content: &str, value: &str) -> LinkTag {
  let link_tag_arrow = match link_direction {
    LinkDirection::Forward => LINK_TAG_ARROW_FORWARD,
    LinkDirection::Reverse => LINK_TAG_ARROW_REVERSE,
  };

  let mut link_tag_bytes = vec![];
  link_tag_bytes.extend_from_slice(&LINK_TAG_HEADER);
  link_tag_bytes.extend_from_slice(&link_tag_arrow);
  link_tag_bytes.extend_from_slice(content.as_bytes());
  link_tag_bytes.extend_from_slice(&UNICODE_NUL_BYTES);
  link_tag_bytes.extend_from_slice(value.as_bytes());

  LinkTag(link_tag_bytes)
}

fn trust_atom_link_tag_leading_bytes(
  link_direction: &LinkDirection,
  content: &str,
  // value: Option<String>, // TODO
) -> LinkTag {
  let link_tag_arrow = match link_direction {
    LinkDirection::Forward => LINK_TAG_ARROW_FORWARD,
    LinkDirection::Reverse => LINK_TAG_ARROW_REVERSE,
  };

  let mut link_tag_bytes = vec![];
  link_tag_bytes.extend_from_slice(&LINK_TAG_HEADER);
  link_tag_bytes.extend_from_slice(&link_tag_arrow);
  link_tag_bytes.extend_from_slice(content.as_bytes());

  LinkTag(link_tag_bytes)
}

pub fn query_mine(
  target: Option<EntryHash>,
  content_starts_with: Option<String>,
  min_rating: Option<String>,
) -> ExternResult<Vec<TrustAtom>> {
  let agent_address: EntryHash = agent_info()?.agent_initial_pubkey.into();

  let result = query(Some(agent_address), target, content_starts_with, min_rating)?;

  Ok(result)
}

/// Required: exactly one of source or target
/// All other arguments are optional
/// Arguments act as additive filters (AND)
#[warn(clippy::needless_pass_by_value)]
pub fn query(
  source: Option<EntryHash>,
  target: Option<EntryHash>,
  content_starts_with: Option<String>,
  _min_rating: Option<String>,
) -> ExternResult<Vec<TrustAtom>> {
  // let link_direction: LinkDirection;

  let (link_direction, link_base) = match (source, target) {
    (Some(source), None) => (LinkDirection::Forward, source),
    (None, Some(target)) => (LinkDirection::Reverse, target),
    (None, None) => {
      return Err(WasmError::Guest(
        "Either source or target must be specified".into(),
      ))
    }
    (Some(_source), Some(_target)) => {
      return Err(WasmError::Guest(
        "Exactly one of source or target must be specified, but not both".into(),
      ))
    }
  };

  let link_tag =
    match (content_starts_with, link_direction.clone()) {
      (Some(content_starts_with), LinkDirection::Forward) => Some(
        trust_atom_link_tag_leading_bytes(&LinkDirection::Forward, &content_starts_with),
      ),
      (Some(content_starts_with), LinkDirection::Reverse) => Some(
        trust_atom_link_tag_leading_bytes(&LinkDirection::Reverse, &content_starts_with),
      ),
      (None, _) => None,
    };

  let links = get_links(link_base.clone(), link_tag)?;

  let trust_atoms = convert_links_to_trust_atoms(links, &link_direction, &link_base)?;

  Ok(trust_atoms)
}

fn convert_links_to_trust_atoms(
  links: Vec<Link>,
  link_direction: &LinkDirection,
  link_base: &EntryHash,
) -> ExternResult<Vec<TrustAtom>> {
  let trust_atoms_result: Result<Vec<TrustAtom>, _> = links
    .into_iter()
    .map(|link| convert_link_to_trust_atom(link, link_direction, link_base))
    .collect();
  let trust_atoms = trust_atoms_result?;
  Ok(trust_atoms)
  // .ok_or_else(|_| WasmError::Guest("Failure in converting links to trust atoms".to_string()))?;
  //   Ok(trust_atoms.or_else(|_| WasmError::Guest("erro"))?)
}

fn convert_link_to_trust_atom(
  link: Link,
  link_direction: &LinkDirection,
  link_base: &EntryHash,
) -> ExternResult<TrustAtom> {
  let link_tag_bytes = link.tag.clone().into_inner();
  let link_tag = match String::from_utf8(link_tag_bytes) {
    Ok(link_tag) => link_tag,
    Err(_) => {
      return Err(WasmError::Guest(format!(
        "Link tag is not valid UTF-8 -- found: {:?}",
        String::from_utf8_lossy(&link.tag.into_inner())
      )))
    }
  };

  let chunks: Vec<&str> = link_tag.split(UNICODE_NUL_STR).collect();
  let content = chunks[0][tg_link_tag_header_length()..].to_string(); // drop leading "Ŧ→" or "Ŧ↩"
  let value = chunks[1].to_string();

  let link_base_b64 = EntryHashB64::from(link_base.clone());
  let link_target_b64 = EntryHashB64::from(link.target);

  let trust_atom = match link_direction {
    LinkDirection::Forward => {
      TrustAtom {
        source: link_base_b64.to_string(),
        target: link_target_b64.to_string(),
        content,
        value,
        source_entry_hash: link_base_b64,
        target_entry_hash: link_target_b64,
        attributes: BTreeMap::new(), // TODO
      }
    }
    LinkDirection::Reverse => {
      TrustAtom {
        source: "".into(),   // TODO
        target: "".into(),   // TODO
        content: link_tag,   // TODO
        value: "999".into(), // TODO
        source_entry_hash: link_target_b64,
        target_entry_hash: link_base.clone().into(),
        attributes: BTreeMap::new(), // TODO
      }
    }
  };
  Ok(trust_atom)
}

pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  let string_target = StringTarget(input);

  create_entry(string_target.clone())?;

  let target_entry_hash = hash_entry(string_target)?;
  Ok(target_entry_hash)
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

pub fn link_tag(tag: String) -> ExternResult<LinkTag> {
  let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
  Ok(LinkTag(serialized_bytes.bytes().clone()))
}

const fn tg_link_tag_header_length() -> usize {
  LINK_TAG_HEADER.len() + LINK_TAG_ARROW_FORWARD.len()
}
