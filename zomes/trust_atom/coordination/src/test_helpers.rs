#![allow(clippy::module_name_repetitions)]
#![allow(clippy::use_self)]

use hc_zome_trust_atom_integrity::{EntryTypes, LinkTypes, StringTarget, Test};
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct StringLinkTag(pub String);
holochain_serial!(StringLinkTag);

pub fn list_links_for_base(base: AnyLinkableHash) -> ExternResult<Vec<Link>> {
  let links = get_links(base, LinkTypes::TrustAtom, None)?;

  Ok(links)
}

pub fn list_links(base: AnyLinkableHash, link_tag_text: Option<String>) -> ExternResult<Vec<Link>> {
  let link_tag = match link_tag_text {
    Some(link_tag_text) => Some(link_tag(link_tag_text)?),
    None => None,
  };

  let links = hdk::link::get_links(base, LinkTypes::TrustAtom, link_tag)?;

  Ok(links)
}

fn link_tag(tag: String) -> ExternResult<LinkTag> {
  // let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
  // Ok(LinkTag(serialized_bytes.bytes().clone()))

  let serialized_bytes = SerializedBytes::try_from(StringLinkTag(tag));
  match serialized_bytes {
    Ok(bytes) => Ok(LinkTag(bytes.bytes().clone())),
    Err(e) => Err(wasm_error!(WasmErrorInner::Serialize(e))),
  }
}

pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  create_entry(EntryTypes::StringTarget(StringTarget(input.clone())))?;

  let target_entry_hash = hash_entry(EntryTypes::StringTarget(StringTarget(input)))?;
  Ok(target_entry_hash)
}

pub fn create_test_entry(input: Test) -> ExternResult<ActionHash> {
  create_entry(EntryTypes::Test(input))
}

pub fn get_entry_by_action(action_hash: ActionHash) -> ExternResult<Test> {
  let record = get_record_by_action(action_hash, GetOptions::default())?;
  match record.entry() {
    record::RecordEntry::Present(entry) => {
      Test::try_from(entry.clone()).or(Err(wasm_error!(WasmErrorInner::Guest(format!(
        "Couldn't convert Record entry {:?} into data type {}",
        entry,
        std::any::type_name::<Test>()
      )))))
    }
    _ => Err(wasm_error!(WasmErrorInner::Guest(format!(
      "Record {:?} does not have an entry",
      record
    )))),
  }
}
#[allow(clippy::needless_pass_by_value)]
fn get_record_by_action(action_hash: ActionHash, get_options: GetOptions) -> ExternResult<Record> {
  match get(action_hash.clone(), get_options)? {
    Some(record) => Ok(record),
    None => Err(wasm_error!(WasmErrorInner::Guest(format!(
      "There is no record at the hash {}",
      action_hash
    )))),
  }
}
