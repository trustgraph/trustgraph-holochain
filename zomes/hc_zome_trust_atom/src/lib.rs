#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::missing_errors_doc)] // TODO fix and remove this
#![allow(clippy::or_fun_call)]

// #![warn(clippy::cargo)]

use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;

use std::collections::BTreeMap;

// public for sweettest; TODO can we fix this:
pub mod trust_atom;
pub use trust_atom::*;
pub mod trust_graph;
pub use trust_graph::*;
pub mod test_helpers;
pub use test_helpers::*;
pub mod utils;
pub use utils::*;

entry_defs![test_helpers::StringTarget::entry_def(), Extra::entry_def()];

// INPUT TYPES

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtomInput {
  pub source: EntryHash, //// for testing purposes ////
  pub target: EntryHash, // TODO maybe target_entry_hash?
  pub prefix: Option<String>,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<EntryHashB64, TrustAtom>>, //// map types modified for tests ////
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryInput {
  pub source: Option<EntryHash>,
  pub target: Option<EntryHash>,
  pub prefix: Option<String>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryMineInput {
  pub target: Option<EntryHash>,
  pub prefix: Option<String>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_rollup_atoms(_: ()) -> ExternResult<Vec<TrustAtom>> {
  let trust_graph = trust_graph::create_rollup_atoms()?;
  Ok(trust_graph)
}

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<TrustAtom> {
  let trust_atom = trust_atom::create_trust_atom(
    input.source,
    input.target,
    input.prefix,
    input.content,
    input.value,
    input.extra,
  )?;
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
    input.prefix,
    input.content_full,
    input.content_starts_with,
    input.value_starts_with,
  )
}

#[hdk_extern]
pub fn query_mine(input: QueryMineInput) -> ExternResult<Vec<TrustAtom>> {
  trust_atom::query_mine(
    input.prefix,
    input.content_full,
    input.content_starts_with,
    input.value_starts_with,
  )
}
// TEST HELPERS

#[hdk_extern]
pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  test_helpers::create_string_target(input)
}

#[hdk_extern]
pub fn test_helper_list_links(
  (base, link_tag_text): (EntryHash, Option<String>),
) -> ExternResult<Vec<Link>> {
  test_helpers::list_links(base, link_tag_text)
}

#[hdk_extern]
pub fn test_helper_list_links_for_base(base: EntryHash) -> ExternResult<Vec<Link>> {
  test_helpers::list_links_for_base(base)
}
