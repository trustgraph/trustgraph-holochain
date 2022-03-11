use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

pub mod trust_graph;
pub use crate::trust_graph::*;

#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq, Hash)]
pub struct TrustAtom {
  pub id: u64, //hash of source_entry_hash + target_entry_hash + random number
  pub source: String, // TODO source_name
  pub target: String,
  pub source_entry_hash: EntryHashB64,
  pub target_entry_hash: EntryHashB64,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<String, String>>,
}

pub struct KeyValue {
  key: u8,
  val: TrustAtom
}

// ZOME API FUNCTIONS

#[hdk_extern]
pub fn create_trust_graph() -> ExternResult<TrustGraph> {
  let trust_graph = trust_graph::TrustGraph::create()?;
  Ok(trust_graph)
}