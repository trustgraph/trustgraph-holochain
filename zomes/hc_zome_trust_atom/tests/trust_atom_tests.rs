#![warn(warnings)]

use std::collections::BTreeMap;

use futures::future;

// use hc_zome_trust_atom::*;

use hc_zome_trust_atom::*;
use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;
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
  let (conductor, agent, cell1) = setup_1_conductor().await;

  // CREATE TARGET ENTRY

  let target_entry_hash: EntryHash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Nuka Sushi",
    )
    .await;

  // CREATE TRUST ATOM

  let content: String = "sushi".into();
  let value: String = ".8".into();
  let attributes: BTreeMap<String, String> = BTreeMap::from([(
    "details".into(),
    "Excellent specials. The regular menu is so-so. Their coconut curry (special) is to die for"
      .into(),
  )]);

  let trust_atom_input = TrustAtomInput {
    target: target_entry_hash.clone(),
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

  // CHECK FORWARD LINK

  let agent_address: EntryHash = agent.clone().into();

  let forward_links: Vec<Link> = conductor
    .call(
      &cell1.zome("trust_atom"),
      "test_helper_list_links_for_base",
      agent_address,
    )
    .await;

  assert_eq!(forward_links.len(), 1);
  let link = &forward_links[0];

  let target_from_link: EntryHash = link.clone().target;
  assert_eq!(target_from_link, target_entry_hash);

  let link_tag_bytes = link.clone().tag.into_inner();
  let relevant_link_bytes = link_tag_bytes.to_vec();
  let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
  let expected_link_tag_string =
    format!("{}{}{}{}{}", "Ŧ", "→", "sushi", unicode_nul, ".800000000");
  assert_eq!(relevant_link_string, expected_link_tag_string);

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 2);
  assert_eq!(chunks[0], "Ŧ→sushi");
  assert_eq!(chunks[1], ".800000000");

  // CHECK BACKWARD LINK

  let backward_links: Vec<Link> = conductor
    .call(
      &cell1.zome("trust_atom"),
      "test_helper_list_links_for_base",
      target_entry_hash.clone(),
    )
    .await;

  assert_eq!(backward_links.len(), 1);
  let link = &backward_links[0];

  // let agent_entry_hash_b64 = EntryHashB64::from(EntryHash::from(agent.clone()));
  // assert_eq!(target_from_link, agent_entry_hash_b64);

  let link_tag_bytes = link.clone().tag.into_inner();
  let relevant_link_bytes = link_tag_bytes.to_vec();
  let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
  let expected_link_tag_string =
    format!("{}{}{}{}{}", "Ŧ", "↩", "sushi", unicode_nul, ".800000000");
  assert_eq!(relevant_link_string, expected_link_tag_string);

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 2);
  assert_eq!(chunks[0], "Ŧ↩sushi");
  assert_eq!(chunks[1], ".800000000");
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_query_mine() {
  let (conductor, agent, cell1) = setup_1_conductor().await;

  // CREATE TARGET ENTRY

  let target_entry_hash: EntryHash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Sushi Ran",
    )
    .await;

  // CREATE TRUST ATOMS

  let _result: () = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_trust_atom",
      TrustAtomInput {
        target: target_entry_hash.clone(),
        content: "sushi".into(),
        value: "0.8".into(),
        attributes: BTreeMap::new(),
        // extra: Some(BTreeMap::new([
        //   ("creator_name".into(), "Bradley Fieldstone Jr.".into()),
        // ])),
      },
    )
    .await;

  // QUERY MY TRUST ATOMS

  let trust_atoms_from_query: Vec<TrustAtom> = conductor
    .call(
      &cell1.zome("trust_atom"),
      "query_mine",
      QueryMineInput {
        target: None,
        content_starts_with: None,
        content_full: None,
        min_rating: None,
      },
    )
    .await;

  assert_eq!(trust_atoms_from_query.len(), 1);

  let source_entry_hash_b64 = EntryHashB64::from(EntryHash::from(agent.clone()));
  let target_entry_hash_b64 = EntryHashB64::from(target_entry_hash);
  let trust_atom = &trust_atoms_from_query[0];

  assert_eq!(
    *trust_atom,
    TrustAtom {
      source: source_entry_hash_b64.to_string(),
      target: target_entry_hash_b64.to_string(),
      content: "sushi".to_string(),
      value: ".800000000".to_string(),
      source_entry_hash: source_entry_hash_b64,
      target_entry_hash: target_entry_hash_b64,
      extra: BTreeMap::new(),
    }
  );
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_query_mine_with_content_starts_with() {
  let (conductor, _agent, cell1) = setup_1_conductor().await;

  // CREATE TARGET ENTRY

  let target_entry_hash: EntryHash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Sushi Ran",
    )
    .await;

  // CREATE TRUST ATOMS

  let contents = vec!["sushi", "sushi joint", "sush"];

  for content in contents {
    let _result: () = conductor
      .call(
        &cell1.zome("trust_atom"),
        "create_trust_atom",
        TrustAtomInput {
          target: target_entry_hash.clone(),
          content: content.into(),
          value: "0.8".into(),
          attributes: BTreeMap::new(),
        },
      )
      .await;
  }
  // QUERY MY TRUST ATOMS

  let trust_atoms_from_query: Vec<TrustAtom> = conductor
    .call(
      &cell1.zome("trust_atom"),
      "query_mine",
      QueryMineInput {
        target: None,
        content_full: None,
        content_starts_with: Some("sushi".into()),
        min_rating: None,
        // min_rating: Some("0.0".into()),
      },
    )
    .await;

  assert_eq!(trust_atoms_from_query.len(), 2);

  let mut actual = [
    trust_atoms_from_query[0].clone().content,
    trust_atoms_from_query[1].clone().content,
  ];
  actual.sort();

  assert_eq!(actual, ["sushi", "sushi joint"]);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_query_mine_with_content_full() {
  let (conductor, _agent, cell1) = setup_1_conductor().await;

  // CREATE TARGET ENTRY

  let target_entry_hash: EntryHash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Sushi Ran",
    )
    .await;

  // CREATE TRUST ATOMS

  let content_fulls = vec!["sushi", "sushi joint", "sush"];

  for content_full in content_fulls {
    let _result: () = conductor
      .call(
        &cell1.zome("trust_atom"),
        "create_trust_atom",
        TrustAtomInput {
          target: target_entry_hash.clone(),
          content: content_full.into(),
          value: "0.8".into(),
          attributes: BTreeMap::new(),
        },
      )
      .await;
  }
  // QUERY MY TRUST ATOMS

  let trust_atoms_from_query: Vec<TrustAtom> = conductor
    .call(
      &cell1.zome("trust_atom"),
      "query_mine",
      QueryMineInput {
        target: None,
        content_starts_with: None,
        content_full: Some("sushi".into()),
        min_rating: None,
        // min_rating: Some("0.0".into()),
      },
    )
    .await;

  assert_eq!(trust_atoms_from_query.len(), 1);

  assert_eq!(
    trust_atoms_from_query[0].clone().content,
    "sushi".to_string()
  );
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
