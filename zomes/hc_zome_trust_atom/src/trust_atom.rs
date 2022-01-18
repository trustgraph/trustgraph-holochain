#![allow(clippy::module_name_repetitions)]

use std::collections::BTreeMap;

use hdk::prelude::*;
use holo_hash::EntryHashB64;

enum LinkDirection {
    Forward,
    Reverse,
}

/// Client-facing representation of a Trust Atom
/// We may support JSON in the future to allow for more complex data structures @TODO
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
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

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct SearchInput {
    pub content_starts_with: Option<String>,
    pub min_rating: Option<String>,
    pub source: Option<EntryHashB64>,
    pub target: Option<EntryHashB64>,
}

#[hdk_entry(id = "restaurant", visibility = "public")]
#[derive(Clone)]
pub struct StringTarget(String);

const LINK_TAG_HEADER: &str = "Ŧ";
const LINK_TAG_DIRECTION_FORWARD: &str = "→";
const LINK_TAG_DIRECTION_BACKWARD: &str = "↩";

pub fn create(input: TrustAtomInput) -> ExternResult<()> {
    let unicode_nul = std::str::from_utf8(&[0]).unwrap();
    let agent_info = agent_info()?;
    let agent_address: AnyDhtHash = agent_info.agent_initial_pubkey.clone().into();

    let forward_link_tag_string = format!(
        "{}{}{}{}{}",
        LINK_TAG_HEADER,
        LINK_TAG_DIRECTION_FORWARD,
        input.content.clone(),
        unicode_nul,
        input.value
    );
    let forward_link_tag = link_tag(forward_link_tag_string)?;
    debug!("about to create forward link");
    create_link(
        agent_address.clone().into(),
        input.target.clone().into(),
        forward_link_tag,
    )?;

    let backward_link_tag_string = format!(
        "{}{}{}",
        LINK_TAG_HEADER,
        LINK_TAG_DIRECTION_BACKWARD,
        input.content.clone()
    );
    let backward_link_tag = link_tag(backward_link_tag_string)?;
    create_link(input.target.into(), agent_address.into(), backward_link_tag)?;

    // let trust_atom = TrustAtom {
    //     target: input.target,
    //     content: input.content,
    //     value: input.value,
    //     attributes: input.attributes,
    // };
    // Ok(trust_atom)

    Ok(())
}

/// Required: exactly one of source or target
/// All other arguments are optional
/// Arguments act as additive filters (AND)
pub fn query(
    content_starts_with: Option<String>,
    _min_rating: Option<String>,
    source: Option<EntryHashB64>,
    target: Option<EntryHashB64>,
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

    let link_tag = match content_starts_with {
        Some(content_starts_with) => Some(link_tag(content_starts_with)?),
        None => None,
    };

    let links = get_links(link_base.clone().into(), link_tag)?;

    let trust_atoms = convert_links_to_trust_atoms(links, link_direction, link_base)?;

    Ok(trust_atoms)
}

fn convert_links_to_trust_atoms(
    links: Vec<Link>,
    link_direction: LinkDirection,
    link_base: EntryHashB64,
) -> ExternResult<Vec<TrustAtom>> {
    let trust_atoms: Result<Vec<TrustAtom>, _> = links
        .into_iter()
        .map(
            |link| //foo
                convert_link_to_trust_atom(link, &link_direction, &link_base), // bar
        )
        .collect();
    Ok(trust_atoms?)
}

fn convert_link_to_trust_atom(
    link: Link,
    link_direction: &LinkDirection,
    link_base: &EntryHashB64,
) -> ExternResult<TrustAtom> {
    let link_target = link.target;

    let link_tag = match String::from_utf8(link.tag.clone().into_inner()) {
        Ok(link_tag) => link_tag,
        Err(_) => {
            return Err(WasmError::Guest(format!(
                "Link tag is not valid UTF-8 -- found: {}",
                String::from_utf8_lossy(&link.tag.into_inner())
            )))
        }
    };

    // let link_tag_string_result = String::from_utf8(link.tag.clone().into_inner());
    // let link_tag = link_tag_string_result.or_else(|_| {
    //     Err(WasmError::Guest(format!(
    //         "Link tag is not valid UTF-8 -- found: {}",
    //         String::from_utf8_lossy(&link.tag.into_inner())
    //     )))
    // })?;

    let trust_atom = match link_direction {
        LinkDirection::Forward => {
            TrustAtom {
                source: "".into(),   // TODO
                target: "".into(),   // TODO
                content: link_tag,   // TODO
                value: "999".into(), // TODO
                source_entry_hash: link_base.clone(),
                target_entry_hash: link_target.into(),
                attributes: BTreeMap::new(), // TODO
            }
        }
        LinkDirection::Reverse => {
            TrustAtom {
                source: "".into(),   // TODO
                target: "".into(),   // TODO
                content: link_tag,   // TODO
                value: "999".into(), // TODO
                source_entry_hash: link_target.into(),
                target_entry_hash: link_base.clone(),
                attributes: BTreeMap::new(), // TODO
            }
        }
    };
    Ok(trust_atom)
}

pub fn create_string_target(input: String) -> ExternResult<EntryHashB64> {
    let string_target = StringTarget(input);

    create_entry(string_target.clone())?;

    let target_entry_hash = hash_entry(string_target)?;
    Ok(target_entry_hash.into())
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

pub fn link_tag(tag: String) -> ExternResult<LinkTag> {
    let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
    Ok(LinkTag(serialized_bytes.bytes().clone()))
}
