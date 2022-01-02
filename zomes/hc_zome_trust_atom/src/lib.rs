#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
// #![warn(clippy::cargo)]

use hdk::prelude::*;

mod trust_atom;
// public for sweettest; TODO can we fix this:
pub use crate::trust_atom::TrustAtom;
pub use crate::trust_atom::TrustAtomInput;

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<TrustAtom> {
    Ok(TrustAtom::create(input)?)
}

// TEMP FOR TEST ONLY:
#[hdk_entry(id = "resaraunt", visibility = "public")]
#[derive(Clone)]
pub struct Resaraunt {
    pub website: String,
}
entry_defs![Resaraunt::entry_def()];
