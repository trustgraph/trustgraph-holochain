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

use hdk::prelude::*;

// public for sweettest; TODO can we fix this:
pub mod trust_atom;
pub use crate::trust_atom::*;

pub mod test_helpers;
// pub use crate::test_helpers;

entry_defs![StringTarget::entry_def(), Extra::entry_def()];

// INPUT TYPES

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryInput {
  pub source: Option<EntryHash>,
  pub target: Option<EntryHash>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub min_rating: Option<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryMineInput {
  pub target: Option<EntryHash>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub min_rating: Option<String>,
}

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<()> {
  trust_atom::create(input.target, &input.content, &input.value, input.attributes, input.extra_field)
}

#[hdk_extern]
pub fn query(input: QueryInput) -> ExternResult<Vec<TrustAtom>> {
  trust_atom::query(
    input.source,
    input.target,
    input.content_full,
    input.content_starts_with,
    input.min_rating,
  )
}

#[hdk_extern]
pub fn query_mine(input: QueryMineInput) -> ExternResult<Vec<TrustAtom>> {
  trust_atom::query_mine(
    input.target,
    input.content_full,
    input.content_starts_with,
    input.min_rating,
  )
}

// TEST HELPERS

#[hdk_extern]
pub fn create_string_target(input: String) -> ExternResult<EntryHash> {
  crate::trust_atom::create_string_target(input)
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
