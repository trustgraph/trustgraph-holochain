#![warn(warnings)]

// pub mod trust_atom {
//     pub mod behavior {

use std::collections::BTreeMap;

use futures::future;

use hc_zome_trust_atom::{SearchInput, TrustAtom, TrustAtomInput};

use hdk::prelude::*;
use holo_hash::AgentPubKeyB64;
use holo_hash::EntryHashB64;
use holochain::sweettest::{
    SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

#[tokio::test]
pub async fn test_unicode_null() {
    let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
    assert_eq!(
        unicode_nul.as_bytes(),
        &[0] // '\u{00}' // .to_string() // .replace("\u{00}", "�")
             // .as_str()
    );
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_atom() {
    let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
    let (conductor, _agent, cell1) = setup_1_conductor().await;

    // CREATE TARGET ENTRY

    let target_entry_hashb64: EntryHashB64 = conductor
        .call(
            &cell1.zome("trust_atom"),
            "create_string_target",
            "Nuka Sushi",
        )
        .await;

    // CREATE TRUST ATOM

    let content: String = "sushi".into();
    let value: String = "0.8".into();
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

    // CHECK FORWARD LINK

    let forward_links: Vec<Link> = conductor
        .call(
            &cell1.zome("trust_atom"),
            "test_helper_list_links_for_base",
            agent_address,
        )
        .await;

    assert_eq!(forward_links.len(), 1);
    let link = &forward_links[0];

    let target_from_link: EntryHashB64 = link.clone().target.into();
    assert_eq!(target_from_link, target_entry_hashb64);

    // println!("link bytes: {:#?}", link.clone().tag.into_inner());
    let link_tag_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_tag_bytes[1..].to_vec(); // skip the first byte, which may be the link type???  we get 165:u8
    let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
    let expected_link_tag_string = format!("{}{}{}{}{}", "Ŧ", "→", "sushi", unicode_nul, "0.8");
    // println!("expected_link_tag_string: {:#?}", expected_link_tag_string);
    println!("relevant_link_string: {:#?}", relevant_link_string);
    assert_eq!(relevant_link_string, expected_link_tag_string);

    let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], "Ŧ→sushi");
    assert_eq!(chunks[1], "0.8");

    // CHECK BACKWARD LINK

    let backward_links: Vec<Link> = conductor
        .call(
            &cell1.zome("trust_atom"),
            "test_helper_list_links_for_base",
            target_entry_hashb64,
        )
        .await;

    assert_eq!(backward_links.len(), 1);
    let link = &backward_links[0];

    // TODO
    // let target_from_link: EntryHashB64 = link.clone().target.into();
    // let agent_address_b64: AgentPubKeyB64 = agent_address.clone().into();
    // assert_eq!(target_from_link, agent_address_b64);

    // println!("link bytes: {:#?}", link.clone().tag.into_inner());
    let link_tag_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_tag_bytes[1..].to_vec(); // skip the first byte, which may be the link type???  we get 165:u8
    let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
    // println!("relevant link: {:#?}", relevant_link_string);
    assert_eq!(relevant_link_string, "Ŧ↩sushi".to_string());
}

// TEST QUERY:
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
