#![allow(clippy::module_name_repetitions)]

use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

pub fn list_links_for_base(base: AnyLinkableHash) -> ExternResult<Vec<Link>> {
  let links = hdk::link::get_links(base, None)?;

  Ok(links)
}

pub fn list_links(base: AnyLinkableHash, link_tag_text: Option<String>) -> ExternResult<Vec<Link>> {
  let link_tag = match link_tag_text {
    Some(link_tag_text) => Some(link_tag(link_tag_text)?),
    None => None,
  };

  let links = hdk::link::get_links(base, link_tag)?;

  Ok(links)
}

fn link_tag(tag: String) -> ExternResult<LinkTag> {
  let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
  Ok(LinkTag(serialized_bytes.bytes().clone()))
}

pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  let string_target = StringTarget(input);

  create_entry(string_target.clone())?;

  let target_entry_hash = hash_entry(string_target)?;
  Ok(target_entry_hash)
}

pub fn create_test_entry(input: Test) -> ExternResult<ActionHash> {
  create_entry(input)
}

pub fn get_entry_by_action(action_hash: ActionHash) -> ExternResult<Test> {
  let record = get_record_by_action(action_hash, GetOptions::default())?;
  match record.entry() {
    record::RecordEntry::Present(entry) => {
      Test::try_from(entry.clone()).or(Err(WasmError::Guest(format!(
        "Couldn't convert Record entry {:?} into data type {}",
        entry,
        std::any::type_name::<Test>()
      ))))
    }
    _ => Err(WasmError::Guest(format!(
      "Record {:?} does not have an entry",
      record
    ))),
  }
}
#[allow(clippy::needless_pass_by_value)]
fn get_record_by_action(
  action_hash: ActionHash,
  get_options: GetOptions,
) -> ExternResult<Record> {
  match get(action_hash.clone(), get_options)? {
    Some(record) => Ok(record),
    None => Err(WasmError::Guest(format!(
      "There is no record at the hash {}",
      action_hash
    ))),
  }
}
