#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
#![allow(clippy::missing_errors_doc)] // TODO fix and remove this
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::or_fun_call)]

// #![warn(clippy::cargo)]

use hdk::prelude::*;
use std::collections::BTreeMap;

pub mod test_helpers;
pub use test_helpers::Test;

// INPUT TYPES

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtomInput {
  pub target: AnyLinkableHash,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryInput {
  pub source: Option<AnyLinkableHash>,
  pub target: Option<AnyLinkableHash>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryMineInput {
  pub target: Option<AnyLinkableHash>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<TrustAtom> {
  let trust_atom = trust_atom::create(input.target, input.content, input.value, input.extra)?;
  Ok(trust_atom)
}

#[hdk_extern]
#[allow(clippy::needless_pass_by_value)]
pub fn get_extra(entry_hash: EntryHash) -> ExternResult<Extra> {
  let extra = trust_atom::get_extra(&entry_hash)?;
  Ok(extra)
}

#[hdk_extern]
pub fn calc_extra_hash(input: Extra) -> ExternResult<EntryHash> {
  let hash = trust_atom::calc_extra_hash(input)?;
  Ok(hash)
}

#[hdk_extern]
pub fn query(input: QueryInput) -> ExternResult<Vec<TrustAtom>> {
  trust_atom::query(
    input.source,
    input.target,
    input.content_full,
    input.content_starts_with,
    input.value_starts_with,
  )
}

#[hdk_extern]
pub fn query_mine(input: QueryMineInput) -> ExternResult<Vec<TrustAtom>> {
  trust_atom::query_mine(
    input.target,
    input.content_full,
    input.content_starts_with,
    input.value_starts_with,
  )
}
// TEST HELPERS

#[hdk_extern]
pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  crate::test_helpers::create_string_target(input)
}

#[hdk_extern]
pub fn create_test_entry(input: Test) -> ExternResult<HeaderHash> {
  test_helpers::create_test_entry(input)
}

#[hdk_extern]
pub fn test_get_entry_by_header(input: HeaderHash) -> ExternResult<Test> {
  test_helpers::get_entry_by_header(input)
}

#[hdk_extern]
pub fn test_helper_list_links(
  (base, link_tag_text): (AnyLinkableHash, Option<String>),
) -> ExternResult<Vec<Link>> {
  test_helpers::list_links(base, link_tag_text)
}

#[hdk_extern]
pub fn test_helper_list_links_for_base(base: AnyLinkableHash) -> ExternResult<Vec<Link>> {
  test_helpers::list_links_for_base(base)
}
