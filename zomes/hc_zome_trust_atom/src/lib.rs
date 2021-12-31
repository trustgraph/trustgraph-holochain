#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
#![allow(clippy::missing_errors_doc)]
// #![warn(clippy::cargo)]

mod trust_atom;
pub use crate::trust_atom::TrustAtom; // public for sweettest; TODO can we fix this?
                                      // use crate::trust_atom::{FractalNftId, OwnershipQuery};

// #[hdk_extern]
// pub fn create(input: FractalNftInput) -> ExternResult<FractalNft> {
//     FractalNft::create(input)
// }
