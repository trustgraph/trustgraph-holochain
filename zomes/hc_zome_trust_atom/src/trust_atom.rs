#![allow(clippy::module_name_repetitions)]

use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;
use rust_decimal::prelude::*;
use std::collections::BTreeMap;

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
  pub extra: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtomInput {
  pub target: EntryHash,
  pub content: String,
  pub value: String,
  pub extra: Option<BTreeMap<String, String>>,
}

#[hdk_entry(id = "restaurant", visibility = "public")]
#[derive(Clone)]
pub struct StringTarget(String);

const UNICODE_NUL_STR: &str = "\u{0}"; // Unicode NUL character
const UNICODE_NUL_BYTES: [u8; 1] = [0];
const LINK_TAG_HEADER: [u8; 2] = [197, 166]; // Unicode "Ŧ" // hex bytes: [0xC5][0xA6]
const LINK_TAG_ARROW_FORWARD: [u8; 3] = [226, 134, 146]; // Unicode "→" // hex bytes: [0xE2][0x86][0x92]
const LINK_TAG_ARROW_REVERSE: [u8; 3] = [226, 134, 169]; // Unicode "↩" // hex bytes: [0xE2][0x86][0xA9]

#[hdk_entry(id = "extra", visibility = "public")]
#[derive(Clone)]
pub struct Extra {
  pub field: BTreeMap<String, String>,
}

#[warn(clippy::needless_pass_by_value)] // TODO remove when `extra` is used
pub fn create(
  target: EntryHash,
  content: &str,
  value: &str,
  extra: Option<BTreeMap<String, String>>,
) -> ExternResult<()> {
  let agent_info = agent_info()?;
  let agent_address: EntryHash = agent_info.agent_initial_pubkey.into();

  let normalized_value = normalize_value(value)?;
  //TODO: used Box to consolidate
  if let Some(extra) = extra {
    let xtr_field_eh = create_extra(extra)?.to_string();
    let forward_link_tag = trust_atom_link_tag(
      &LinkDirection::Forward,
      vec![content, &normalized_value, &xtr_field_eh],
    );
    let reverse_link_tag = trust_atom_link_tag(
      &LinkDirection::Reverse,
      vec![content, &normalized_value, &xtr_field_eh],
    );
    create_link(agent_address.clone(), target.clone(), forward_link_tag)?;
    create_link(target, agent_address, reverse_link_tag)?;
  } else {
    let forward_link_tag =
      trust_atom_link_tag(&LinkDirection::Forward, vec![content, &normalized_value]);
    let reverse_link_tag =
      trust_atom_link_tag(&LinkDirection::Reverse, vec![content, &normalized_value]);
    create_link(agent_address.clone(), target.clone(), forward_link_tag)?;
    create_link(target, agent_address, reverse_link_tag)?;
  }
  Ok(())
}

fn normalize_value(value_str: &str) -> ExternResult<String> {
  match Decimal::from_str(value_str) {
    Ok(value_decimal) => {
      match value_decimal.round_sf_with_strategy(9, RoundingStrategy::MidpointAwayFromZero) {
        Some(value_decimal) => {
          if value_decimal == Decimal::ONE {
            Ok(".999999999".to_string())
          } else if value_decimal == Decimal::NEGATIVE_ONE {
            Ok("-.999999999".to_string())
          } else if value_decimal > Decimal::NEGATIVE_ONE && value_decimal < Decimal::ONE {
            let value_zero_stripped = value_decimal.to_string().replace("0.", ".");
            Ok(value_zero_stripped)
          } else {
            Err(WasmError::Guest(format!(
              "Value must be in the range -1..1, but got: `{}`",
              value_str
            )))
          }
        }
        None => Err(WasmError::Guest(format!(
          "Value could not be processed: `{}`",
          value_str
        ))),
      }
    }
    Err(error) => Err(WasmError::Guest(format!(
      "Value could not be processed: `{}`.  Error: `{}`",
      value_str, error
    ))),
  }
}

fn trust_atom_link_tag(link_direction: &LinkDirection, mut chunks: Vec<&str>) -> LinkTag {
  let link_tag_arrow = match link_direction {
    LinkDirection::Forward => LINK_TAG_ARROW_FORWARD,
    LinkDirection::Reverse => LINK_TAG_ARROW_REVERSE,
  };

  let mut link_tag_bytes = vec![];
  link_tag_bytes.extend_from_slice(&LINK_TAG_HEADER);
  link_tag_bytes.extend_from_slice(&link_tag_arrow);
  let content = chunks.remove(0);
  link_tag_bytes.extend_from_slice(content.as_bytes());

  for chunk in &chunks {
    link_tag_bytes.extend_from_slice(&UNICODE_NUL_BYTES);
    link_tag_bytes.extend_from_slice(chunk.as_bytes());
  }

  LinkTag(link_tag_bytes)
}

pub fn query_mine(
  target: Option<EntryHash>,
  content_full: Option<String>,
  content_starts_with: Option<String>,
  min_value: Option<String>,
) -> ExternResult<Vec<TrustAtom>> {
  let agent_address: EntryHash = agent_info()?.agent_initial_pubkey.into();

  let result = query(
    Some(agent_address),
    target,
    content_full,
    content_starts_with,
    min_value,
  )?;

  Ok(result)
}

/// Required: exactly one of source or target
/// All other arguments are optional
/// Arguments act as additive filters (AND)
#[warn(clippy::needless_pass_by_value)]
pub fn query(
  source: Option<EntryHash>,
  target: Option<EntryHash>,
  content_full: Option<String>,
  content_starts_with: Option<String>,
  min_value: Option<String>,
) -> ExternResult<Vec<TrustAtom>> {
  let (full, starts_with, min_val) = match (content_full, content_starts_with, min_value) {
    (Some(_content_full), Some(_content_starts_with), _) => {
      return Err(WasmError::Guest(
        "Exactly one query method must be specified, but not both".into(),
      ))
    }
    (_, Some(_content_starts_with), Some(_min_value)) => {
      return Err(WasmError::Guest(
        "Must be full content to pass min value".into(),
      ))
    }
    (Some(content_full), None, min_value) => (Some(content_full), None, min_value),
    (None, Some(content_starts_with), None) => (None, Some(content_starts_with), None),
    (None, None, min_value) => (None, None, min_value),
  };

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

  let link_tag = match (full, starts_with, link_direction.clone()) {
    (Some(full), None, LinkDirection::Forward) => Some(trust_atom_link_tag(
      &LinkDirection::Forward,
      vec![&full, &min_val.unwrap_or("".to_string())],
    )),
    (Some(full), None, LinkDirection::Reverse) => Some(trust_atom_link_tag(
      &LinkDirection::Reverse,
      vec![&full, &min_val.unwrap_or("".to_string())],
    )),
    (None, Some(starts_with), LinkDirection::Forward) => Some(trust_atom_link_tag(
      &LinkDirection::Forward,
      vec![&starts_with],
    )),
    (None, Some(starts_with), LinkDirection::Reverse) => Some(trust_atom_link_tag(
      &LinkDirection::Reverse,
      vec![&starts_with],
    )),
    (Some(_full), Some(_starts_with), _) => None, // error handled earlier
    (None, None, _) => None,
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
        extra: BTreeMap::new(), // TODO
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
        extra: BTreeMap::new(), // TODO
      }
    }
  };
  Ok(trust_atom)
}

pub fn create_extra(input: BTreeMap<String, String>) -> ExternResult<EntryHash> {
  let entry = Extra { field: input };

  create_entry(entry.clone())?;

  let entry_hash = hash_entry(entry)?;
  Ok(entry_hash)
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

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(non_snake_case)]
mod tests {

  use super::*; // allows testing of private functions

  #[test]
  fn test_normalize_value__valid_value() {
    let valid_values = [
      "1.0",
      "1.000000000000000000000000000",
      "1",
      "0.534857395723489529357489283",
      "0.0",
      "0.000000000000000000000000000",
      "0",
      "-1.0",
      "-1",
      "-1.00000000000000000000000000",
    ];

    for value in valid_values {
      normalize_value(value).unwrap();
    }
  }

  #[test]
  fn test_normalize_value__values_out_of_range() {
    let out_of_range_values = [
      "100000000000000000",
      "-100000000000000000",
      "2",
      "1.000000005",
      "1.00000001",
      "-1.00000001",
      "-1.000000005",
      "-2",
    ];

    for value in out_of_range_values {
      let expected_error_message = "Value must be in the range -1..1";
      let actual_error_message = normalize_value(value)
        .expect_err(&format!("expected error for value `{}`, got", value))
        .to_string();
      assert!(
        actual_error_message.contains(expected_error_message),
        "Expected error message: `...{}...`, but got: `{}`",
        expected_error_message,
        actual_error_message
      );
    }
  }

  #[test]
  fn test_normalize_value__values_not_numeric() {
    #[rustfmt::skip]
    let non_numeric_values = [
      " ",
      " 0 ",
      " 0",
      "-.",
      "-",
      "-100000000000000000000000000000.0",
      "-1e",
      "-1e0",
      "-e0",
      "!",
      ".",
      "",
      "",
      "\u{1f9d0}",
      "0 ",
      "100000000000000000000000000000.0",
      "1e",
      "1e0",
      "e",
      "e0",
      "foo",
     ];

    for value in non_numeric_values {
      let expected_error_message = "Value could not be processed";
      let actual_error_message = normalize_value(value)
        .expect_err(&format!("expected error for value `{}`, got", value))
        .to_string();
      assert!(
        actual_error_message.contains(expected_error_message),
        "Expected error message: `...{}...`, but got: `{}`",
        expected_error_message,
        actual_error_message
      );
    }
  }

  #[test]
  fn test_normalize_value() {
    assert_eq!(normalize_value("-.9").unwrap(), "-.900000000");
    assert_eq!(normalize_value("-.9000").unwrap(), "-.900000000");
    assert_eq!(normalize_value("-.900000000").unwrap(), "-.900000000");
    assert_eq!(normalize_value("-.9000000004").unwrap(), "-.900000000");
    assert_eq!(normalize_value("-.9000000005").unwrap(), "-.900000001");
    assert_eq!(normalize_value("-0.900000000").unwrap(), "-.900000000");

    assert_eq!(normalize_value("0.8999999995").unwrap(), ".900000000");
    assert_eq!(normalize_value("0.7999999995").unwrap(), ".800000000");
    assert_eq!(normalize_value("-0.8999999995").unwrap(), "-.900000000");
    assert_eq!(normalize_value("-0.7999999995").unwrap(), "-.800000000");

    assert_eq!(normalize_value("0.8999999994").unwrap(), ".899999999");
    assert_eq!(normalize_value("0.7999999994").unwrap(), ".799999999");
    assert_eq!(normalize_value("-0.8999999994").unwrap(), "-.899999999");
    assert_eq!(normalize_value("-0.7999999994").unwrap(), "-.799999999");

    assert_eq!(normalize_value(".9").unwrap(), ".900000000");
    assert_eq!(normalize_value(".9000").unwrap(), ".900000000");
    assert_eq!(normalize_value(".900000000").unwrap(), ".900000000");
    assert_eq!(normalize_value("0.900000000").unwrap(), ".900000000");

    //

    assert_eq!(normalize_value("1").unwrap(), ".999999999");
    assert_eq!(normalize_value("1.0").unwrap(), ".999999999");

    assert_eq!(normalize_value("-1").unwrap(), "-.999999999");
    assert_eq!(normalize_value("-1.0").unwrap(), "-.999999999");
  }
}
