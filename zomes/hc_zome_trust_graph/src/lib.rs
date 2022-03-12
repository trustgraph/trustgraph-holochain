use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

pub mod trust_graph;
pub use crate::trust_graph::*;

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_trust_graph() -> ExternResult<TrustGraph> {
  let trust_graph = trust_graph::TrustGraph::create()?;
  Ok(trust_graph)
}