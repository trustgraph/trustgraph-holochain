#![allow(clippy::module_name_repetitions)]

use hdk::prelude::*;

pub fn list_links(link_tag_text: String, base: AnyDhtHash) -> ExternResult<Vec<Link>> {
    let link_tag = link_tag(link_tag_text)?;

    let links = hdk::link::get_links(base.into(), Some(link_tag.clone()))?;

    Ok(links)
}

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

fn link_tag(tag: String) -> ExternResult<LinkTag> {
    let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
    Ok(LinkTag(serialized_bytes.bytes().clone()))
}
