#![allow(clippy::module_name_repetitions)]

use std::collections::BTreeMap;

use holo_hash::EntryHashB64;

pub struct TrustAtom {
    pub target: EntryHashB64,
    pub content: String,
    pub value: f32,
    pub attributes: BTreeMap<String, String>,
}

impl TrustAtom {
    #[must_use]
    pub const fn spike() -> i8 {
        42
    }
    // pub fn create(target: EntryHashB64) -> Self {
    //     Self {
    //         target,
    //         content: "".into(),
    //         value: 0.0,
    //         attributes: BTreeMap::new(),
    //     }
    // }
}
