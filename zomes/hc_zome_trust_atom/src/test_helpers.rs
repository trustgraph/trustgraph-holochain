#![allow(clippy::module_name_repetitions)]

use crate::trust_atom::Extra;
use hdk::prelude::*;

#[derive(Serialize, Deserialize, Debug, SerializedBytes)]
struct StringLinkTag(String);

#[hdk_entry(id = "restaurant", visibility = "public")]
#[derive(Clone)]
pub struct StringTarget(String);

#[hdk_entry(id = "test", visibility = "public")]
#[derive(Clone)]
pub struct Test {
  pub example_field: String,
  //another_test_field:
}

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

pub fn create_test_entry(input: Test) -> ExternResult<HeaderHash> {
  create_entry(input)
}

pub fn get_entry_by_header(header_hash: HeaderHash) -> ExternResult<Test> {
  let element = get_element_by_header(header_hash, GetOptions::default())?;
  match element.entry() {
    element::ElementEntry::Present(entry) => {
      Test::try_from(entry.clone()).or(Err(WasmError::Guest(format!(
        "Couldn't convert Element entry {:?} into data type {}",
        entry,
        std::any::type_name::<Test>()
      ))))
    }
    _ => Err(WasmError::Guest(format!(
      "Element {:?} does not have an entry",
      element
    ))),
  }
}
#[allow(clippy::needless_pass_by_value)]
fn get_element_by_header(
  header_hash: HeaderHash,
  get_options: GetOptions,
) -> ExternResult<Element> {
  match get(header_hash.clone(), get_options)? {
    Some(element) => Ok(element),
    None => Err(WasmError::Guest(format!(
      "There is no element at the hash {}",
      header_hash
    ))),
  }
pub fn calc_extra_hash(input: Extra) -> ExternResult<EntryHash> {
  hash_entry(input)
}

fn link_tag(tag: String) -> ExternResult<LinkTag> {
  let serialized_bytes: SerializedBytes = StringLinkTag(tag).try_into()?;
  Ok(LinkTag(serialized_bytes.bytes().clone()))
}
