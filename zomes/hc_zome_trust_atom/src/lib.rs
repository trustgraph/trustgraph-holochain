#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
// #![warn(clippy::cargo)]

use hdk::prelude::*;
use holo_hash::EntryHashB64;

mod trust_atom;
// public for sweettest; TODO can we fix this:
pub use crate::trust_atom::spike;
pub use crate::trust_atom::StringTarget;
pub use crate::trust_atom::TrustAtom;
pub use crate::trust_atom::TrustAtomInput;

entry_defs![StringTarget::entry_def()];

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<TrustAtom> {
    TrustAtom::create(input)
}

#[hdk_extern]
pub fn create_string_target(input: String) -> ExternResult<EntryHashB64> {
    TrustAtom::create_string_target(input)
}

// // TEMP FOR TEST ONLY:
// #[hdk_entry(id = "restaurant", visibility = "public")]
// #[derive(Clone)]
// pub struct Restaurant {
//     pub website: String,
// }
// entry_defs![Restaurant::entry_def()];
