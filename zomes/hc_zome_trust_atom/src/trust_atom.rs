#![allow(clippy::module_name_repetitions)]

use std::collections::BTreeMap;

use hdk::prelude::*;
use holo_hash::EntryHashB64;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtom {
  pub target: EntryHashB64,
  pub content: String,
  pub value: f32,
  pub attributes: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct TrustAtomInput {
  pub target: EntryHashB64,
  pub content: String,
  pub value: f32,
  pub attributes: BTreeMap<String, String>,
}

#[hdk_entry(id = "restaurant", visibility = "public")]
#[derive(Clone)]
pub struct StringTarget(String);

impl TrustAtom {
  pub fn create(input: TrustAtomInput) -> ExternResult<Self> {
    let trust_atom = Self {
      target: input.target,
      content: input.content,
      value: input.value,
      attributes: input.attributes,
    };
    Ok(trust_atom)
  }

  pub fn create_string_target(input: String) -> ExternResult<EntryHashB64> {
    let string_target = StringTarget(input);

    create_entry(string_target.clone())?;

    let target_entry_hash = hash_entry(string_target)?;
    let target_entry_hashb64: EntryHashB64 = target_entry_hash.into();
    debug!("target_entry_hashb64: {:#?}", target_entry_hashb64);
    Ok(target_entry_hashb64)
  }
}

#[must_use]
pub const fn spike() -> i8 {
  42
}
