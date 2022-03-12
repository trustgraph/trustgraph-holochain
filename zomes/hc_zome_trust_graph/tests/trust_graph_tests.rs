#![warn(warnings)]

use std::collections::BTreeMap;
use futures::future;
use hc_zome_trust_atom::*;
use hc_zome_trust_graph::*;
use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;
use holochain::sweettest::{
  SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

const DNA_FILEPATH: &str = "../../workdir/dna/trust_graph.dna";


#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_graph() {
  let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
  let (conductor, agent, cell1) = setup_1_conductor().await;

  let target_entry_hash: EntryHash = conductor
    .call(
      &cell1.zome("trust_graph"),
      "create_trust_graph",
      "Harlan",
    )
    .await;

  let input = Input {
    agents: tg_test_helpers::test_create_links().unwrap(),
    tag_filter: None
  };

  let test_graph = trust_graph::TrustGraph::create(input).unwrap();
}
    
    // TESTING UTILITY FUNCTIONS

async fn setup_1_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
  let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
    .await
    .unwrap();

  let mut conductor = SweetConductor::from_standard_config().await;

  let agent = SweetAgents::one(conductor.keystore()).await;
  let app1 = conductor
    .setup_app_for_agent("app", agent.clone(), &[dna.clone()])
    .await
    .unwrap();

  let cell1 = app1.into_cells()[0].clone();

  (conductor, agent, cell1)
}