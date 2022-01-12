#![allow(clippy::module_name_repetitions)]

use hdk::prelude::*;

pub fn list_links_for_base(base: AnyDhtHash) -> ExternResult<Vec<Link>> {
    let links = hdk::link::get_links(base.into(), None)?;

    Ok(links)
}

pub fn list_links(base: AnyDhtHash, link_tag_text: Option<String>) -> ExternResult<Vec<Link>> {
    let link_tag = match link_tag_text {
        Some(link_tag_text) => Some(link_tag(link_tag_text)?),
        None => None,
    };

    let links = hdk::link::get_links(base.into(), link_tag)?;

    Ok(links)
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

fn link_tag(tag: String) -> ExternResult<LinkTag> {
    let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
    Ok(LinkTag(serialized_bytes.bytes().clone()))
}
