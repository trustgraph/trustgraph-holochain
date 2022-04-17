#![warn(warnings)]

use std::collections::BTreeMap;

use futures::future;
use tokio::time::{sleep, Duration};

use hc_zome_trust_atom::*;
use hdk::prelude::holo_hash::AgentPubKey;
use hdk::prelude::holo_hash::EntryHash;
use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;
use holochain::sweettest::{
  SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

// #[tokio::test]
// pub async fn test_unicode_null() {
//   let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
//   assert_eq!(
//     unicode_nul.as_bytes(),
//     &[0] // '\u{00}' // .to_string() // .replace("\u{00}", "�")
//          // .as_str()
//   );
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_create_trust_atom() {
//   let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
//   let (conductor, agent, cell1) = setup_1_conductor().await;

//   // CREATE TARGET ENTRY

//   let target_entry_hash: EntryHash = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_string_target",
//       "Nuka Sushi",
//     )
//     .await;

//   // CREATE TRUST ATOM

//   let content: String = "sushi".into();
//   let value: String = ".8".into();
//   // let extra: BTreeMap<String, String> = BTreeMap::from([(
//   //   "details".into(),
//   //   "Excellent specials. The regular menu is so-so. Their coconut curry (special) is to die for"
//   //     .into(),
//   // )]);

//   let trust_atom_input = TrustAtomInput {
//     source: EntryHash::from(agent.clone()),
//     target: target_entry_hash.clone(),
//     prefix: None,
//     content: Some(content.clone()),
//     value: Some(value.clone()),
//     extra: None,
//   };

//   let _result: TrustAtom = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_trust_atom",
//       trust_atom_input,
//     )
//     .await;

//   // CHECK FORWARD LINK

//   let agent_address: EntryHash = agent.clone().into();

//   let forward_links: Vec<Link> = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "test_helper_list_links_for_base",
//       agent_address,
//     )
//     .await;

//   assert_eq!(forward_links.len(), 1);
//   let link = &forward_links[0];

//   let target_from_link: EntryHash = link.clone().target;
//   assert_eq!(target_from_link, target_entry_hash);

//   let link_tag_bytes = link.clone().tag.into_inner();
//   let relevant_link_bytes = link_tag_bytes.to_vec();
//   let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();

//   let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
//   assert_eq!(chunks.len(), 5);
//   assert_eq!(chunks[0], "Ŧ→");
//   assert_eq!(chunks[1], "sushi");
//   assert_eq!(chunks[2], ".800000000");

//   let bucket = chunks[3];

//   assert_eq!(bucket.chars().count(), 9);
//   assert!(bucket.chars().all(|c| c.is_digit(10)));

//   // let expected_entry_hash = "uhCEkto76kYgGIZMzU6AbEzCx1HMRNzurwPaOdF2utJaP-33mdcdN";
//   let expected_link_tag_string = format!(
//     "{}{}{}{}{}{}{}{}{}",
//     "Ŧ", "→", unicode_nul, "sushi", unicode_nul, ".800000000", unicode_nul, bucket, unicode_nul,
//   );
//   println!(
//     "117: {:?}",
//     assert_eq!(relevant_link_string, expected_link_tag_string)
//   );

//   // CHECK BACKWARD LINK

//   let backward_links: Vec<Link> = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "test_helper_list_links_for_base",
//       target_entry_hash.clone(),
//     )
//     .await;

//   assert_eq!(backward_links.len(), 1);
//   let link = &backward_links[0];

//   // let agent_entry_hash_b64 = EntryHashB64::from(EntryHash::from(agent.clone()));
//   // assert_eq!(target_from_link, agent_entry_hash_b64);

//   let link_tag_bytes = link.clone().tag.into_inner();
//   let relevant_link_bytes = link_tag_bytes.to_vec();
//   let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
//   let expected_link_tag_string = format!(
//     "{}{}{}{}{}{}{}{}{}",
//     "Ŧ", "↩", unicode_nul, "sushi", unicode_nul, ".800000000", unicode_nul, bucket, unicode_nul,
//   );
//   assert_eq!(relevant_link_string, expected_link_tag_string);

//   let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
//   println!("153: {:?}", assert_eq!(chunks.len(), 5));
//   assert_eq!(chunks[0], "Ŧ↩");
//   assert_eq!(chunks[1], "sushi");
//   assert_eq!(chunks[2], ".800000000");
//   assert_eq!(chunks[3], bucket);
//   // assert_eq!(chunks[4], expected_entry_hash);
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_query_mine() {
//   let (conductor, agent, cell1) = setup_1_conductor().await;

//   // CREATE TARGET ENTRY

//   let target_entry_hash: EntryHash = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_string_target",
//       "Sushi Ran",
//     )
//     .await;

//   // CREATE TRUST ATOMS

//   let _result: TrustAtom = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_trust_atom",
//       TrustAtomInput {
//         source: agent.clone().into(),
//         target: target_entry_hash.clone(),
//         prefix: None,
//         content: Some("sushi".to_string()),
//         value: Some("0.8".to_string()),
//         extra: None,
//         // extra: Some(BTreeMap::new([
//         //   ("creator_name".into(), "Bradley Fieldstone Jr.".into()),
//         // ])),
//       },
//     )
//     .await;

//   // QUERY MY TRUST ATOMS

//   let trust_atoms_from_query: Vec<TrustAtom> = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "query_mine",
//       QueryMineInput {
//         prefix: None,
//         target: None,
//         content_starts_with: None,
//         content_full: None,
//         value_starts_with: None,
//       },
//     )
//     .await;

//   assert_eq!(trust_atoms_from_query.len(), 1);

//   let source_entry_hash_b64 = EntryHashB64::from(EntryHash::from(agent.clone()));
//   let target_entry_hash_b64 = EntryHashB64::from(target_entry_hash);
//   let trust_atom = &trust_atoms_from_query[0];

//   assert_eq!(
//     *trust_atom,
//     TrustAtom {
//       // source: source_entry_hash_b64.to_string(),
//       // target: target_entry_hash_b64.to_string(),
//       source_entry_hash: source_entry_hash_b64,
//       target_entry_hash: target_entry_hash_b64,
//       prefix: None,
//       content: Some("sushi".to_string()),
//       value: Some(".800000000".to_string()),
//       extra: None,
//     }
//   );
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_query_mine_with_content_starts_with() {
//   let (conductor, agent, cell1) = setup_1_conductor().await;

//   // CREATE TARGET ENTRY

//   let target_entry_hash: EntryHash = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_string_target",
//       "Sushi Ran",
//     )
//     .await;

//   // CREATE TRUST ATOMS

//   let contents = vec!["sushi", "sushi joint", "sush"];

//   for content in contents {
//     let _result: TrustAtom = conductor
//       .call(
//         &cell1.zome("trust_atom"),
//         "create_trust_atom",
//         TrustAtomInput {
//           source: agent.clone().into(),
//           target: target_entry_hash.clone(),
//           prefix: None,
//           content: Some(content.into()),
//           value: Some("0.8".into()),
//           extra: None,
//         },
//       )
//       .await;
//   }
//   // QUERY MY TRUST ATOMS

//   let trust_atoms_from_query: Vec<TrustAtom> = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "query_mine",
//       QueryMineInput {
//         prefix: None,
//         target: None,
//         content_full: None,
//         content_starts_with: Some("sushi".into()),
//         value_starts_with: None,
//         // value_starts_with: Some("0.0".into()),
//       },
//     )
//     .await;

//   assert_eq!(trust_atoms_from_query.len(), 2);

//   let mut actual = [
//     trust_atoms_from_query[0].clone().content,
//     trust_atoms_from_query[1].clone().content,
//   ];
//   actual.sort();

//   assert_eq!(
//     actual,
//     [Some("sushi".to_string()), Some("sushi joint".to_string())]
//   );
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_query_mine_with_content_full() {
//   let (conductor, agent, cell1) = setup_1_conductor().await;

//   // CREATE TARGET ENTRY

//   let target_entry_hash: EntryHash = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_string_target",
//       "Sushi Ran",
//     )
//     .await;

//   // CREATE TRUST ATOMS

//   let content_fulls = vec!["sushi", "sushi joint", "sush"];

//   for content_full in content_fulls {
//     let _result: TrustAtom = conductor
//       .call(
//         &cell1.zome("trust_atom"),
//         "create_trust_atom",
//         TrustAtomInput {
//           source: agent.clone().into(),
//           target: target_entry_hash.clone(),
//           prefix: None,
//           content: Some(content_full.into()),
//           value: Some("0.8".into()),
//           extra: None,
//         },
//       )
//       .await;
//   }
//   // QUERY MY TRUST ATOMS

//   let trust_atoms_from_query: Vec<TrustAtom> = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "query_mine",
//       QueryMineInput {
//         prefix: None,
//         target: None,
//         content_full: Some("sushi".into()),
//         content_starts_with: None,
//         value_starts_with: None,
//         // value_starts_with: Some("0.0".into()),
//       },
//     )
//     .await;

//   assert_eq!(trust_atoms_from_query.len(), 1);

//   assert_eq!(
//     trust_atoms_from_query[0].clone().content,
//     Some("sushi".to_string())
//   );
// }

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_get_extra() {
//   let (conductor, agent, cell1) = setup_1_conductor().await;

//   let target_entry_hash = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_string_target",
//       "Nuka Sushi",
//     )
//     .await;

//   let mock_input = TrustAtomInput {

//     source: agent.clone(),
//     target: target_entry_hash,
//     prefix: None,
//     content: Some("sushi".to_string()),
//     value: Some("0.9871".to_string()),
//     extra: Some("xxx".to_string(),
//   };

//   let _mock_trust_atom: TrustAtom = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_trust_atom",
//       mock_input.clone(),
//     )
//     .await;

//   let mock_entry = Extra {
//     sourced_atoms: Some(mock_input.extra.unwrap()),
//   };
//   let mock_extra_entry_hash: EntryHash = conductor
//     .call(&cell1.zome("trust_atom"), "calc_extra_hash", mock_entry)
//     .await;

//   let mock_extra_data: Extra = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "get_extra",
//       mock_extra_entry_hash,
//     )
//     .await;

//   let field1 = mock_extra_data
//     .fields
//     .get_key_value(&"extra_stuff".to_string())
//     .unwrap();
//   let field2 = mock_extra_data
//     .fields
//     .get_key_value(&"another_thing".to_string())
//     .unwrap();

//   assert_eq!(
//     field1,
//     (
//       &"extra_stuff".to_string(),
//       &"Say some extra stuff here".to_string()
//     )
//   );
//   assert_eq!(
//     field2,
//     (
//       &"another_thing".to_string(),
//       &"Put more information here".to_string()
//     )
//   );
// }
#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_graph() {
  let (conductors, agents, apps) = setup_conductors(3).await;
  conductors.exchange_peer_info().await;

  let agent_me = EntryHash::from(agents[0].clone());
  let agent_zippy = EntryHash::from(agents[1].clone());
  let agent_bob = EntryHash::from(agents[2].clone());

  let conductor_me = &conductors[0];
  let conductor_zippy = &conductors[1];
  let conductor_bob = &conductors[2];

  let cells = apps.cells_flattened();
  let cell_me = cells[0];
  let cell_zippy = cells[1];
  let cell_bob = cells[2];

  // CREATE TARGET ENTRIES

    let hia_entry_hash: EntryHash = conductor_me
      .call(
        &cell_me.zome("trust_atom"),
        "create_string_target",
        "HIA",
      )
      .await;

    let telos_entry_hash: EntryHash = conductor_me
      .call(
        &cell_me.zome("trust_atom"),
        "create_string_target",
        "Telos",
      )
      .await;

    let _ethereum_entry_hash: EntryHash = conductor_bob
      .call(
        &cell_bob.zome("trust_atom"),
        "create_string_target",
        "Ethereum",
      )
      .await;

  // eg : given TAs:
  let data = [
    (
      (agent_me.clone(), conductor_me, cell_me),
      "holochain",
      "0.99",
      agent_zippy.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "holochain",
      "0.80",
      agent_bob.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "holochain",
      "0.97",
      hia_entry_hash.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "engineering",
      "0.88",
      telos_entry_hash.clone(),
    ),
    (
      (agent_zippy.clone(), conductor_zippy, cell_zippy),
      "holochain",
      "0.91",
      hia_entry_hash.clone(),
    ),
    (
      (agent_bob.clone(), conductor_bob, cell_bob),
      "holochain",
      "-0.99",
      hia_entry_hash.clone(), // IDEA: spam count, outliers
    ),
    (
      (agent_bob.clone(), conductor_bob, cell_bob),
      "engineering",
      "0.99",
      telos_entry_hash.clone(),
    ),
  ];

  // CREATE TEST ENTRIES

  let fake1_entry_hash: EntryHash = conductor_me
    .call(&cell_me.zome("trust_atom"), "create_string_target", "fake1")
    .await;

  let fake2_entry_hash: EntryHash = conductor_me
    .call(&cell_me.zome("trust_atom"), "create_string_target", "fake2")
    .await;

  // CREATE TEST AGENT ROLLUPS  // helps to identify the agents for algorithm

  let zippy_mock_rollup_atom_input = TrustAtomInput {
    source: agent_zippy.clone(),
    target: fake1_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _zippy_mock_rollup_atom: TrustAtom = conductor_zippy
    .call(
      &cell_zippy.zome("trust_atom"),
      "create_trust_atom",
      zippy_mock_rollup_atom_input,
    )
    .await;

  // println!("zippy_mock_rollup_atom: {:?}", zippy_mock_rollup_atom);

  let bob_mock1_rollup_atom_input = TrustAtomInput {
    source: agent_bob.clone(),
    target: fake1_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _bob_mock1_rollup_atom: TrustAtom = conductor_bob
    .call(
      &cell_bob.zome("trust_atom"),
      "create_trust_atom",
      bob_mock1_rollup_atom_input,
    )
    .await;

  let bob_mock2_rollup_atom_input = TrustAtomInput {
    source: agent_bob.clone(),
    target: fake2_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _bob_mock2_rollup_atom: TrustAtom = conductor_bob
    .call(
      &cell_bob.zome("trust_atom"),
      "create_trust_atom",
      bob_mock2_rollup_atom_input,
    )
    .await;

  for ((agent, conductor, cell), content, value, target) in data {

    // CREATE TRUST ATOM

    let trust_atom_input = TrustAtomInput {
      source: agent.clone(),
      target,
      prefix: None, //Some("test".to_string()),
      content: Some(content.to_string()),
      value: Some(value.to_string()),
      extra: None,
    };

    let _trust_atom: TrustAtom = conductor
      .call(
        &cell.zome("trust_atom"),
        "create_trust_atom",
        trust_atom_input,
      )
      .await;

    // println!("trust_atom: {:#?}", trust_atom);
  }

  sleep(Duration::from_millis(1000)).await;

  let any = ();
  let rollup_atoms: Vec<TrustAtom> = conductor_me
    .call(&cell_me.zome("trust_atom"), "create_rollup_atoms", any)
    .await;

  // then rollup atoms will be:
  // me -[rollup, holochain, 0.98]-> HIA                  // actual value is TBD
  // me -[rollup, engineering, 0.99]-> Ethereum     // actual value is TBD
  // me -[rollup, engineering, 0.88]-> Telos            // actual value is TBD

  let me_b64 = EntryHashB64::from(agent_me.clone());

  assert_eq!(
    rollup_atoms,
    vec![
      TrustAtom {
        source_entry_hash: me_b64.clone(),
        target_entry_hash: EntryHashB64::from(telos_entry_hash),
        prefix: Some("rollup".to_string()),
        content: Some("engineering".to_string()),
        value: Some("0.7040000000000001".to_string()), // YMMV
        extra: None,
      },
      TrustAtom {
        source_entry_hash: me_b64.clone(),
        target_entry_hash: EntryHashB64::from(hia_entry_hash),
        prefix: Some("rollup".to_string()),
        content: Some("holochain".to_string()),
        value: Some("0.7881675977653632".to_string()), // YMMV
        extra: None,
      },
      // TrustAtom {
      //   source_entry_hash: me_b64.clone(),
      //   target_entry_hash: EntryHashB64::from(ethereum_entry_hash),
      //   prefix: Some("rollup".to_string()),
      //   content: Some("engineering".to_string()),
      //   value: Some(".990000000".to_string()), // YMMV
      //   extra: None,
      // },
    ]
  ); // currently ignore targets not rated by self
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
