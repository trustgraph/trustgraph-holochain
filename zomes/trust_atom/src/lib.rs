#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
#![allow(clippy::missing_errors_doc)] // TODO fix and remove this
#![allow(clippy::missing_const_for_fn)]
#![allow(clippy::or_fun_call)]
#![allow(clippy::option_if_let_else)]
// #![warn(clippy::cargo)]

use hdk::prelude::*;
mod trust_atom;
pub(crate) use trust_atom_integrity::{Example, Extra, LinkTypes};
pub(crate) use trust_atom_types::{QueryInput, QueryMineInput, TrustAtom, TrustAtomInput};
pub(crate) mod test_helpers;

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<TrustAtom> {
  let trust_atom = trust_atom::create(input.target, input.content, input.value, input.extra)?;
  Ok(trust_atom)
}

#[hdk_extern]
pub fn delete_trust_atoms(target: AnyLinkableHash) -> ExternResult<usize> {
  let links = get_links(target, LinkTypes::TrustAtom, None)?;
  let create_link_hashes: Vec<ActionHash> = links
    .into_iter()
    .map(|link| link.create_link_hash)
    .collect();
  for create_link_hash in create_link_hashes.clone() {
    delete_link(create_link_hash)?;
  }
  Ok(create_link_hashes.len())
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
pub fn create_test_entry(input: Example) -> ExternResult<ActionHash> {
  test_helpers::create_test_entry(input)
}

#[hdk_extern]
pub fn test_get_entry_by_action(input: ActionHash) -> ExternResult<Example> {
  test_helpers::get_entry_by_action(input)
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
