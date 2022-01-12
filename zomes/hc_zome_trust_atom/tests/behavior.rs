#![warn(warnings)]

// pub mod trust_atom {
//     pub mod behavior {

use std::collections::BTreeMap;

use futures::future;

use hc_zome_trust_atom::{SearchInput, TrustAtom, TrustAtomInput};

use hdk::prelude::*;
use holo_hash::EntryHashB64;
use holochain::sweettest::{
    SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

// #[ignore]
// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_spike() {
//     let (conductor, _agent, cell1) = setup_1_conductor().await;

//     let result = hc_zome_trust_atom::spike();
//     assert_eq!(result, 42);
// }

// #[ignore]
#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_atom() {
    std::env::set_var("RUST_LOG", "debug");

    let (conductor, _agent, cell1) = setup_1_conductor().await;

    let target_entry_hashb64: EntryHashB64 = conductor
        .call(
            &cell1.zome("trust_atom"),
            "create_string_target",
            "Nuka Sushi",
        )
        .await;

    let content: String = "sushi".into();
    let value: f32 = 0.8;
    let attributes: BTreeMap<String, String> =
        BTreeMap::from([("details".into(), "Excellent specials. The regular menu is so-so. Their coconut curry (special) is to die for".into())]);

    let trust_atom_input = TrustAtomInput {
        target: target_entry_hashb64.clone(),
        content: content.clone(),
        value,
        attributes: attributes.clone(),
    };

    let _result: () = conductor
        .call(
            &cell1.zome("trust_atom"),
            "create_trust_atom",
            trust_atom_input,
        )
        .await;

    let agent_address: AnyDhtHash = _agent.clone().into();

    // let agentB64: EntryHashB64 = _agent.as_hash().into();

    let links: Vec<Link> = conductor
        .call(
            &cell1.zome("trust_atom"),
            "test_helper_list_links",
            ("sushi".to_string(), agent_address),
        )
        .await;

    assert_eq!(links.len(), 1);
    let link = links.first().unwrap();

    let target_from_link: EntryHashB64 = link.clone().target.into();
    assert_eq!(target_from_link, target_entry_hashb64);

    // println!("link bytes: {:#?}", link.clone().tag.into_inner());
    let link_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_bytes[1..].to_vec(); // skip the first byte, which may be the link type???  we get 165:u8
    assert_eq!(
        String::from_utf8(relevant_link_bytes).unwrap(),
        "Ŧ→sushi".to_string()
    );

    // assert_eq!(trust_atom.target, target_entry_hashb64.clone());
    // assert_eq!(trust_atom.content, content);
    // assert_eq!(trust_atom.value, value);
    // assert_eq!(trust_atom.attributes, attributes);

    // let search_input: SearchInput = SearchInput {
    //     content_starts_with: Some("sushi".into()),
    //     min_rating: Some("0.0".into()),
    //     source: None,
    //     target: None,
    // };

    // let trust_atoms_from_query: Vec<TrustAtom> = conductor
    //     .call(&cell1.zome("trust_atom"), "query", search_input)
    //     .await;

    // assert_eq!(trust_atoms_from_query.len(), 1);
    // // assert_eq!(trust_atoms_from_query[0].target, target_entry_hashb64);
    // assert_eq!(trust_atoms_from_query[0].content, content);
    // // assert_eq!(trust_atoms_from_query[0].value, value);
    // assert_eq!(trust_atoms_from_query[0].attributes, attributes);
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
//     }
// }
