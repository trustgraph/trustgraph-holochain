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

    #[must_use]
    pub const fn spike() -> i8 {
        42
    }
}
