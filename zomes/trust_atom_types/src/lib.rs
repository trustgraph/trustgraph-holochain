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
use std::collections::BTreeMap;

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
  pub content_not_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct QueryMineInput {
  pub target: Option<AnyLinkableHash>,
  pub content_full: Option<String>,
  pub content_starts_with: Option<String>,
  pub content_not_starts_with: Option<String>,
  pub value_starts_with: Option<String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct DeleteReport {
  pub trust_atoms_deleted: usize,
  pub forward_links_deleted: usize,
  pub backward_links_deleted: usize,
}

/// Client-facing representation of a Trust Atom
/// We may support JSON in the future to allow for more complex data structures @TODO
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq, Eq)]
pub struct TrustAtom {
  pub source_hash: AnyLinkableHash,
  pub target_hash: AnyLinkableHash,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<String, String>>,
}
