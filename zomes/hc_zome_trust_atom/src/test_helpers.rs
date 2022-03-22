#![allow(clippy::module_name_repetitions)]

use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

#[hdk_entry(id = "restaurant", visibility = "public")]
#[derive(Clone)]
pub struct StringTarget(String);

pub fn list_links_for_base(base: EntryHash) -> ExternResult<Vec<Link>> {
  let links = hdk::link::get_links(base, None)?;

  Ok(links)
}

pub fn list_links(base: EntryHash, link_tag_text: Option<String>) -> ExternResult<Vec<Link>> {
  let link_tag = match link_tag_text {
    Some(link_tag_text) => Some(link_tag(link_tag_text)?),
    None => None,
  };

  let links = hdk::link::get_links(base, link_tag)?;

  Ok(links)
}

pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  let string_target = StringTarget(input);

  create_entry(string_target.clone())?;

  let target_entry_hash = hash_entry(string_target)?;
  Ok(target_entry_hash)
}

fn link_tag(tag: String) -> ExternResult<LinkTag> {
  let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
  Ok(LinkTag(serialized_bytes.bytes().clone()))
}

// pub fn test_create_links() -> ExternResult<Vec<EntryHash>> {
//   let agent1: EntryHash = fake_agent_pub_key()?.into();
//   let agent2: EntryHash = fake_agent_pub_key()?.into();
//   create_link(agent1.clone(), fake_entry_hash()?, "HIA".to_string())?;
//   create_link(agent2.clone(), fake_entry_hash()?, "HIA".to_string())?;
//   create_link(fake_entry_hash()?, fake_entry_hash()?, "tag3".to_string())?;
//   let test_agents = Vec::from(agent1, agent2);
//   Ok(test_agents)
// }
