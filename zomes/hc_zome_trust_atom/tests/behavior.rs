#![warn(warnings)]

use futures::future;

use hdk::prelude::*;
use holochain::sweettest::{
    SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};
// use holochain::test_utils::consistency_10s;

use hc_zome_trust_atom::TrustAtom;

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create() {
    let (conductor, _agent, cell1) = setup_1_conductor().await;

    // let target = create_entry("target").await;
    // let target: EntryHashB64 = "...".into(); // TODO
    let result = TrustAtom::spike();

    assert_eq!(result, 42);

    // let target: EntryHashB64 = "...".into(); // TODO
    // let content: String = "sushi".into();
    // let value: f32 = 0.8;
    // let attributes: BTreeMap<String, String> = BTreeMap::from([
    //     ("original_rating_type".into(), "stars".into()),
    //     ("original_rating_min".into(), "1".into()),
    //     ("original_rating_max".into(), "5".into()),
    //     ("original_rating_value".into(), "4".into()),
    // ]);

    // let trust_atom = TrustAtom.create(
    //     target: target,
    //     content: content,
    //     value: value,
    //     attributes: attributes,
    // );
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
