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
