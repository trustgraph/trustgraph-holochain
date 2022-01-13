#![warn(warnings)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::unwrap_in_result)]
#![allow(clippy::missing_errors_doc)] // TODO fix and remove this
#![allow(clippy::missing_const_for_fn)]
// #![warn(clippy::cargo)]

use hdk::prelude::*;
use holo_hash::EntryHashB64;

// public for sweettest; TODO can we fix this:
pub mod trust_atom;
pub use crate::trust_atom::SearchInput;
pub use crate::trust_atom::StringTarget;
pub use crate::trust_atom::TrustAtom;
pub use crate::trust_atom::TrustAtomInput;

pub mod test_helpers;
// pub use crate::test_helpers;

entry_defs![StringTarget::entry_def()];

#[hdk_extern]
pub fn create_trust_atom(input: TrustAtomInput) -> ExternResult<()> {
    trust_atom::create(input)
}

#[hdk_extern]
pub fn query(input: SearchInput) -> ExternResult<Vec<TrustAtom>> {
    trust_atom::query(
        input.content_starts_with,
        input.min_rating,
        input.source,
        input.target,
    )
}

#[hdk_extern]
pub fn create_string_target(input: String) -> ExternResult<EntryHashB64> {
    crate::trust_atom::create_string_target(input)
}

// TEST HELPERS

#[hdk_extern]
pub fn test_helper_list_links(
    (base, link_tag_text): (AnyDhtHash, Option<String>),
) -> ExternResult<Vec<Link>> {
    test_helpers::list_links(base, link_tag_text)
}

#[hdk_extern]
pub fn test_helper_list_links_for_base(base: AnyDhtHash) -> ExternResult<Vec<Link>> {
    test_helpers::list_links_for_base(base)
}
