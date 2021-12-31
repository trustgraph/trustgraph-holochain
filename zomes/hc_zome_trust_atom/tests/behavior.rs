#![warn(warnings)]

// stdlib:
use std::collections::HashMap;

// rust libs:
use futures::future;

// holochain:
use hdk::prelude::*;
use holochain::sweettest::{
    SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};
// use holochain::test_utils::consistency_10s;

#[cfg(feature = "sweettest")]
use hc_zome_trust_atom::{FractalNft, FractalNftInput, TransferArgs};

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

#[tokio::test(flavor = "multi_thread")]
#[cfg(feature = "sweettest")]
pub async fn test_create() {
    let (conductor, _agent, cell1) = setup_1_conductor().await;

    let mut agent_distributions = HashMap::new();
    agent_distributions.insert("abc".to_string(), "0.02".to_string());
    let trust_atom_input = FractalNftInput {
        transferable: true,
        agent_distributions: agent_distributions.clone(),
        nft_distributions: HashMap::new(),
    };

    let trust_atom: FractalNft = conductor
        .call(&cell1.zome("trust_atom"), "create", trust_atom_input)
        .await;

    assert!(trust_atom.transferable);
    assert_eq!(trust_atom.agent_distributions, agent_distributions);
    // assert_eq!(trust_atom.final_distributions, agent_distributions);
    assert_eq!(trust_atom.id.len(), 52);
}

pub async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
    let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
        .await
        .unwrap();

    let mut conductors = SweetConductorBatch::from_standard_config(n).await;

    let all_agents: Vec<AgentPubKey> =
        future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;
    let apps = conductors
        .setup_app_for_zipped_agents("app", &all_agents, &[dna])
        .await
        .unwrap();

    conductors.exchange_peer_info().await;
    (conductors, all_agents, apps)
}
