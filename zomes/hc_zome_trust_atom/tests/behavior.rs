#![warn(warnings)]

use std::collections::BTreeMap;

use futures::future;

use hc_zome_trust_atom::Resaraunt; // TEMP FOR TEST ONLY
use hc_zome_trust_atom::{create_trust_atom, TrustAtom, TrustAtomInput};

use hdk::prelude::*;
use holo_hash::EntryHashB64;
use holochain::sweettest::{
    SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};
// use holochain::test_utils::consistency_10s;

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create() {
    let (conductor, _agent, cell1) = setup_1_conductor().await;

    // let result = TrustAtom::spike();
    // assert_eq!(result, 42);

    let sushi_joint = Resaraunt {
        website: "https://www.nukamaui.com/".into(),
    };

    let result = create_entry(sushi_joint.clone());
    error!("{:#?}", 111);
    warn!("{:#?}", result);

    let target_entry_hash_result = hash_entry(sushi_joint);
    error!("{:#?}", 222);
    warn!("{:#?}", target_entry_hash_result);

    let target_entry_hash = target_entry_hash_result.unwrap();
    error!("{:#?}", 333);
    warn!("{:#?}", target_entry_hash);

    let target_entry_hashb64: EntryHashB64 = target_entry_hash.into();

    let content: String = "sushi".into();
    let value: f32 = 0.8;
    let attributes: BTreeMap<String, String> =
        BTreeMap::from([("comment".into(), "From a longtime regular customer".into())]);

    let trust_atom = create_trust_atom(TrustAtomInput {
        target: target_entry_hashb64.clone(),
        content: content,
        value: value,
        attributes: attributes,
    })
    .unwrap();

    assert_eq!(trust_atom.clone().target, target_entry_hashb64.clone());
}

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
