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

// public for sweettest; TODO can we fix this:
pub mod trust_atom;
pub use trust_atom::*;
pub mod trust_graph;
pub use trust_graph::*;
pub mod test_helpers;
pub use test_helpers::Test;
pub mod utils;
pub use utils::*;

entry_defs![
  test_helpers::StringTarget::entry_def(),
  Extra::entry_def(),
  Test::entry_def()
];

// INPUT TYPES

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtomInput {
  pub prefix: Option<String>,
  pub target: AnyLinkableHash,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<String, String>>,
  // for rollups key is "rolled_up_trust_atoms"
  // value is json: '["header hash of atom 1","header hash of atom 2"...]'
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TestHelperTrustAtomInput {
  pub source: AnyLinkableHash,
  pub prefix: Option<String>,
  pub target: AnyLinkableHash,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<String, String>>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryInput {
  pub source: Option<AnyLinkableHash>,
  pub target: Option<AnyLinkableHash>,
  pub prefix: Option<String>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryMineInput {
  pub target: Option<AnyLinkableHash>,
  pub prefix: Option<String>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

// #[hdk_extern]
// fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
//     match op {
//         Op::StoreRecord {
//             record:
//                 Record {
//                     signed_action,
//                     entry: RecordEntry::Present(entry),
//                 },
//         } => {
//             match signed_action.action().entry_type().and_then(|et| match et {
//                 EntryType::App(AppEntryType { id, zome_id, .. }) => Some((zome_id, id)),
//                 _ => None,
//             }) {
//                 Some((zome_id, id)) => {
//                     match EntryTypes::deserialize_from_type(*zome_id, *id, &entry) {
//                         Ok(Some(EntryTypes::Thing(thing))) => Ok(thing.into()),
//                         Ok(None) => Ok(ValidateCallbackResult::Valid),
//                         Err(WasmError {
//                             error: WasmErrorInner::Deserialize(_),
//                             ..
//                         }) => Ok(ValidateCallbackResult::Invalid(
//                             "Failed to deserialize entry".to_string(),
//                         )),
//                         Err(e) => Err(e),
//                     }
//                 }
//                 None => Ok(ValidateCallbackResult::Valid),
//             }
//         }
//         _ => Ok(ValidateCallbackResult::Valid),
//     }
// }

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_rollup_atoms(_: ()) -> ExternResult<Vec<TrustAtom>> {
  trust_graph::create_rollup_atoms()
}

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<TrustAtom> {
  trust_atom::create_mine_trust_atom(
    input.target,
    input.prefix,
    input.content,
    input.value,
    input.extra,
  )
}

#[hdk_extern]
// TEST HELPER move me
pub fn test_helper_create_trust_atom(input: TestHelperTrustAtomInput) -> ExternResult<TrustAtom> {
  trust_atom::create_trust_atom(
    input.source,
    input.target,
    input.prefix,
    input.content,
    input.value,
    input.extra,
  )
}

#[hdk_extern]
#[allow(clippy::needless_pass_by_value)]
pub fn get_extra(entry_hash: EntryHash) -> ExternResult<Extra> {
  trust_atom::get_extra(&entry_hash)
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
    input.target,
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

#[hdk_extern]
pub fn test_helper_calc_extra_hash(input: Extra) -> ExternResult<EntryHash> {
  test_helpers::calc_extra_hash(input)
}
