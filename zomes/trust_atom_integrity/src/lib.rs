use hdi::prelude::*;
pub mod entries;
pub mod headers;
pub mod validation;

#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
  TrustAtom,
}
