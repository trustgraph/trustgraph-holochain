use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

pub mod trust_graph;
pub use crate::trust_graph::*;

// INPUT TYPES

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone)]
pub struct Input {
  pub agents: Vec<EntryHash>,
  pub tag_filter: Option<LinkTag>,
}

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_trust_graph(input: Input) -> ExternResult<TrustGraph> {
  let trust_graph = trust_graph::TrustGraph::create()?;
  Ok(trust_graph)
}