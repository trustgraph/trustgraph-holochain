#![warn(warnings)]

use std::collections::BTreeMap;

use futures::future;
use tokio::time::{sleep, Duration};

use hc_zome_trust_atom::*;
use hdk::prelude::*;
use holo_hash::AgentPubKey;
use holo_hash::AnyLinkableHash;
// use holo_hash::AnyLinkableHashB64;
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
  let extra: BTreeMap<String, String> = BTreeMap::from([(
    "details".into(),
    "Excellent specials. The regular menu is so-so. Their coconut curry (special) is to die for"
      .into(),
  )]);

  let trust_atom_input = TestHelperTrustAtomInput {
    source: AnyLinkableHash::from(agent.clone()),
    target: AnyLinkableHash::from(target_entry_hash.clone()),
    prefix: None,
    content: Some(content.clone()),
    value: Some(value.clone()),
    extra: Some(extra.clone()),
  };

  let _result: TrustAtom = conductor
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

  let target_from_link = link.clone().target;
  assert_eq!(
    target_from_link,
    AnyLinkableHash::from(target_entry_hash.clone())
  );

  let link_tag_bytes = link.clone().tag.into_inner();
  let relevant_link_bytes = link_tag_bytes.to_vec();
  let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 6);
  assert_eq!(chunks[0], "Ŧ→");
  assert_eq!(chunks[1], "");
  assert_eq!(chunks[2], "sushi");
  assert_eq!(chunks[3], ".800000000");

  let bucket = chunks[4];

  assert_eq!(bucket.chars().count(), 9);
  assert!(bucket.chars().all(|c| c.is_digit(10)));

  let expected_entry_hash = "uhCEkZhFjkXulbv_vgbg51u3tNGmGy28XqSz3pIgTcF8ZauOLMcni";
  let expected_link_tag_string = format!(
    "{}{}{}{}{}{}{}{}{}{}{}",
    "Ŧ",
    "→",
    unicode_nul,
    unicode_nul,
    "sushi",
    unicode_nul,
    ".800000000",
    unicode_nul,
    bucket,
    unicode_nul,
    expected_entry_hash
  );

  assert_eq!(relevant_link_string, expected_link_tag_string);

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
  let expected_link_tag_string = format!(
    "{}{}{}{}{}{}{}{}{}{}{}",
    "Ŧ",
    "↩",
    unicode_nul,
    unicode_nul,
    "sushi",
    unicode_nul,
    ".800000000",
    unicode_nul,
    bucket,
    unicode_nul,
    expected_entry_hash
  );
  assert_eq!(relevant_link_string, expected_link_tag_string);

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 6);

  assert_eq!(chunks[0], "Ŧ↩");
  assert_eq!(chunks[1], "");
  assert_eq!(chunks[2], "sushi");
  assert_eq!(chunks[3], ".800000000");
  assert_eq!(chunks[4], bucket);
  assert_eq!(chunks[5], expected_entry_hash);
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

  let _result: TrustAtom = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_trust_atom",
      TrustAtomInput {
        target: AnyLinkableHash::from(target_entry_hash.clone()),
        prefix: None,
        content: Some("sushi".to_string()),
        value: Some("0.8".to_string()),
        extra: None,
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
        prefix: None,
        target: None,
        content_starts_with: None,
        content_full: None,
        value_starts_with: None,
      },
    )
    .await;

  assert_eq!(trust_atoms_from_query.len(), 1);

  // let source_entry_hash_b64 = EntryHashB64::from(EntryHash::from(agent.clone()));
  // let target_entry_hash_b64 = EntryHashB64::from(target_entry_hash);
  let trust_atom = &trust_atoms_from_query[0];

  assert_eq!(
    *trust_atom,
    TrustAtom {
      source_entry_hash: AnyLinkableHash::from(AnyLinkableHash::from(agent.clone())),
      target_entry_hash: AnyLinkableHash::from(AnyLinkableHash::from(target_entry_hash)),
      prefix: None,
      content: Some("sushi".to_string()),
      value: Some(".800000000".to_string()),
      extra: None,
    }
  );
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_query_mine_with_content_starts_with() {
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

  let contents = vec!["sushi", "sushi joint", "sush"];

  for content in contents {
    let result: TrustAtom = conductor
      .call(
        &cell1.zome("trust_atom"),
        "create_trust_atom",
        TrustAtomInput {
          target: AnyLinkableHash::from(target_entry_hash.clone()),
          prefix: None,
          content: Some(content.into()),
          value: Some("0.8".into()),
          extra: None,
        },
      )
      .await;
    println!("result: {:#?}", result);
  }
  // QUERY MY TRUST ATOMS

  let trust_atoms_from_query: Vec<TrustAtom> = conductor
    .call(
      &cell1.zome("trust_atom"),
      "query_mine",
      QueryMineInput {
        prefix: None,
        target: None,
        content_full: None,
        content_starts_with: Some("sushi".into()),
        value_starts_with: None,
        // value_starts_with: Some("0.0".into()),
      },
    )
    .await;

  assert_eq!(trust_atoms_from_query.len(), 2);

  let mut actual = [
    trust_atoms_from_query[0].clone().content,
    trust_atoms_from_query[1].clone().content,
  ];
  actual.sort();

  assert_eq!(
    actual,
    [Some("sushi".to_string()), Some("sushi joint".to_string())]
  );
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_query_mine_with_content_full() {
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

  let content_fulls = vec!["sushi", "sushi joint", "sush"];

  for content_full in content_fulls {
    let _result: TrustAtom = conductor
      .call(
        &cell1.zome("trust_atom"),
        "create_trust_atom",
        TrustAtomInput {
          target: AnyLinkableHash::from(target_entry_hash.clone()),
          prefix: None,
          content: Some(content_full.into()),
          value: Some("0.8".into()),
          extra: None,
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
        prefix: None,
        target: None,
        content_full: Some("sushi".into()),
        content_starts_with: None,
        value_starts_with: None,
        // value_starts_with: Some("0.0".into()),
      },
    )
    .await;

  assert_eq!(trust_atoms_from_query.len(), 1);

  assert_eq!(
    trust_atoms_from_query[0].clone().content,
    Some("sushi".to_string())
  );
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_extra() {
  let (conductor, agent, cell1) = setup_1_conductor().await;

  let target_entry_hash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Nuka Sushi",
    )
    .await;

  let extra_map = BTreeMap::from([
    ("key1".to_string(), "val1".to_string()),
    ("key2".to_string(), "val2".to_string()),
  ]);
  let mock_input = TrustAtomInput {
    target: target_entry_hash,
    prefix: None,
    content: Some("sushi".to_string()),
    value: Some("0.9871".to_string()),
    extra: Some(extra_map.clone()),
  };

  let _mock_trust_atom: TrustAtom = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_trust_atom",
      mock_input.clone(),
    )
    .await;

  let mock_extra_data = Extra {
    field: extra_map.clone(),
  };
  let mock_extra_entry_hash: EntryHash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "test_helper_calc_extra_hash",
      mock_extra_data.clone(),
    )
    .await;

  let mock_entry: Extra = conductor
    .call(
      &cell1.zome("trust_atom"),
      "get_extra",
      mock_extra_entry_hash,
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_get_entry_by_headerhash() {
  let (conductor, _agent, cell1) = setup_1_conductor().await;

  let test_entry = Test {
    example_field: "test".to_string(),
  };

  let header_hash: HeaderHash = conductor
    .call(&cell1.zome("trust_atom"), "create_test_entry", test_entry)
    .await;

  let retrieval: Test = conductor
    .call(
      &cell1.zome("trust_atom"),
      "test_get_entry_by_header",
      header_hash,
    )
    .await;
  assert_eq!("test".to_string(), retrieval.example_field);
}

// #[tokio::test(flavor = "multi_thread")]
// pub async fn test_fetch_external() {
//   let (conductor, agent, cell1) = setup_1_conductor().await;

//   //// WIP

//   let ipfs_address = ExternalHash::from("https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu".to_string());
//   let hc_linked_ipfs_address = AnyLinkableHash::from(ipfs_address.clone());

//   let mock_input = TrustAtomInput {
//     target: AnyLinkableHash::from(ipfs_address),
//     content: Some("ipfs".to_string()),
//     value: None,
//     extra: None,
//   };

//   let mock_trust_atom: TrustAtom = conductor
//     .call(
//       &cell1.zome("trust_atom"),
//       "create_trust_atom",
//       mock_input.clone(),
//     )
//     .await;

//   let external_link = get_links(AnyLinkableHash::from(ipfs_address), None);

//   let fetched_atom = convert_link_to_trust_atom(external_link, LinkDirection::Reverse, AnyLinkableHash::from(agent))?;

//   println!("external_link: {:?}", external_link);
//   assert_eq!(mock_trust_atom, fetched_atom);
//   assert_eq!(fetched_atom.target, ipfs_address);
// }

// =======
// =======
// =======
// =======
//   assert_eq!(extra_map, mock_extra_data.clone().field);
//   assert_eq!(mock_extra_data.clone(), mock_entry.clone());
// }

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_atom_with_empty_chunks() {
  let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
  let (conductor, agent, cell1) = setup_1_conductor().await;

  // CREATE TARGET ENTRY

  let target_entry_hash: AnyLinkableHash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Nuka Sushi",
    )
    .await;

  // CREATE TRUST ATOM

  let trust_atom_input = TrustAtomInput {
    target: target_entry_hash.clone(),
    prefix: None,
    content: None,
    value: None,
    extra: None,
  };

  let _result: TrustAtom = conductor
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

  let target_from_link: AnyLinkableHash = link.clone().target;
  assert_eq!(target_from_link, target_entry_hash);

  let link_tag_bytes = link.clone().tag.into_inner();
  let relevant_link_bytes = link_tag_bytes.to_vec();
  let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 6);
  assert_eq!(chunks[0], "Ŧ→");
  assert_eq!(chunks[1], "");
  assert_eq!(chunks[2], "");
  assert_eq!(chunks[3], "");

  let bucket = chunks[4];

  assert_eq!(bucket.chars().count(), 9);
  assert!(bucket.chars().all(|c| c.is_digit(10)));

  let expected_link_tag_string = format!(
    "{}{}{}{}{}{}{}{}",
    "Ŧ", "→", unicode_nul, unicode_nul, unicode_nul, unicode_nul, bucket, unicode_nul
  );
  assert_eq!(relevant_link_string, expected_link_tag_string);

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

  let link_tag_bytes = link.clone().tag.into_inner();
  let relevant_link_bytes = link_tag_bytes.to_vec();
  let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
  let expected_link_tag_string = format!(
    "{}{}{}{}{}{}{}{}",
    "Ŧ", "↩", unicode_nul, unicode_nul, unicode_nul, unicode_nul, bucket, unicode_nul
  );
  assert_eq!(relevant_link_string, expected_link_tag_string);

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 6);
  assert_eq!(chunks[0], "Ŧ↩");
  assert_eq!(chunks[1], "");
  assert_eq!(chunks[2], "");
  assert_eq!(chunks[3], "");
  assert_eq!(chunks[4], bucket);
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_graph() {
  let (conductors, agents, apps) = setup_conductors(6).await;
  conductors.exchange_peer_info().await;

  let agent_me = AnyLinkableHash::from(agents[0].clone());
  let agent_zippy = AnyLinkableHash::from(agents[1].clone());
  let agent_alice = AnyLinkableHash::from(agents[2].clone());
  let agent_bob = AnyLinkableHash::from(agents[3].clone());
  let agent_charlie = AnyLinkableHash::from(agents[4].clone());
  let agent_spam = AnyLinkableHash::from(agents[5].clone());

  let conductor_me = &conductors[0];
  let conductor_zippy = &conductors[1];
  let conductor_alice = &conductors[2];
  let conductor_bob = &conductors[3];
  let conductor_charlie = &conductors[4];
  let conductor_spam = &conductors[5];

  let cells = apps.cells_flattened();
  let cell_me = cells[0];
  let cell_zippy = cells[1];
  let cell_alice = cells[2];
  let cell_bob = cells[3];
  let cell_charlie = cells[4];
  let cell_spam = cells[5];

  // CREATE TARGET ENTRIES

  let hia_entry_hash: AnyLinkableHash = conductor_me
    .call(&cell_me.zome("trust_atom"), "create_string_target", "HIA")
    .await;

  let telos_entry_hash: AnyLinkableHash = conductor_me
    .call(&cell_me.zome("trust_atom"), "create_string_target", "Telos")
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
      "0.90",
      agent_alice.clone(),
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
      "0.70",
      agent_charlie.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "holochain",
      "-0.99",
      agent_spam.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "engineering",
      "0.80",
      agent_zippy.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "engineering",
      "0.50",
      agent_alice.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "engineering",
      "0.20",
      agent_bob.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "engineering",
      "-0.10",
      agent_charlie.clone(),
    ),
    (
      (agent_me.clone(), conductor_me, cell_me),
      "engineering",
      "-0.99",
      agent_spam.clone(),
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
      "0.83",
      telos_entry_hash.clone(),
    ),
    (
      (agent_zippy.clone(), conductor_zippy, cell_zippy),
      "holochain",
      "0.91",
      hia_entry_hash.clone(),
    ),
    (
      (agent_zippy.clone(), conductor_zippy, cell_zippy),
      "engineering",
      "0.74",
      telos_entry_hash.clone(),
    ),
    (
      (agent_alice.clone(), conductor_alice, cell_alice),
      "holochain",
      "0.85",
      hia_entry_hash.clone(),
    ),
    (
      (agent_alice.clone(), conductor_alice, cell_alice),
      "engineering",
      "0.96",
      telos_entry_hash.clone(),
    ),
    (
      (agent_bob.clone(), conductor_bob, cell_bob),
      "holochain",
      "0.90",
      hia_entry_hash.clone(),
    ),
    (
      (agent_bob.clone(), conductor_bob, cell_bob),
      "engineering",
      "0.99",
      telos_entry_hash.clone(),
    ),
    (
      (agent_charlie.clone(), conductor_charlie, cell_charlie),
      "holochain",
      "0.50",
      hia_entry_hash.clone(),
    ),
    (
      (agent_charlie.clone(), conductor_charlie, cell_charlie),
      "engineering",
      "0.99",
      telos_entry_hash.clone(),
    ),
    (
      (agent_spam.clone(), conductor_spam, cell_spam),
      "holochain",
      "-0.99",
      hia_entry_hash.clone(),
    ),
    (
      (agent_spam.clone(), conductor_spam, cell_spam),
      "engineering",
      "0.99",
      telos_entry_hash.clone(),
    ),
  ];

  // CREATE TEST ENTRIES

  let fake_entry_hash: AnyLinkableHash = conductor_me
    .call(&cell_me.zome("trust_atom"), "create_string_target", "fake")
    .await;

  // CREATE TEST AGENT ROLLUPS  // helps to identify the agents for algorithm

  let zippy_mock_rollup_atom_input = TestHelperTrustAtomInput {
    source: agent_zippy.clone(),
    target: fake_entry_hash.clone(),
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

  let alice_mock_rollup_atom_input = TestHelperTrustAtomInput {
    source: agent_alice.clone(),
    target: fake_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _alice_mock_rollup_atom: TrustAtom = conductor_alice
    .call(
      &cell_alice.zome("trust_atom"),
      "create_trust_atom",
      alice_mock_rollup_atom_input,
    )
    .await;

  let bob_mock_rollup_atom_input = TestHelperTrustAtomInput {
    source: agent_bob.clone(),
    target: fake_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _bob_mock_rollup_atom: TrustAtom = conductor_bob
    .call(
      &cell_bob.zome("trust_atom"),
      "create_trust_atom",
      bob_mock_rollup_atom_input,
    )
    .await;

  let charlie_mock_rollup_atom_input = TestHelperTrustAtomInput {
    source: agent_charlie.clone(),
    target: fake_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _charlie_mock_rollup_atom: TrustAtom = conductor_charlie
    .call(
      &cell_charlie.zome("trust_atom"),
      "create_trust_atom",
      charlie_mock_rollup_atom_input,
    )
    .await;

  let spam_mock_rollup_atom_input = TestHelperTrustAtomInput {
    source: agent_spam.clone(),
    target: fake_entry_hash.clone(),
    prefix: Some("rollup".to_string()),
    content: None,
    value: None,
    extra: None,
  };

  let _spam_mock_rollup_atom: TrustAtom = conductor_spam
    .call(
      &cell_spam.zome("trust_atom"),
      "create_trust_atom",
      spam_mock_rollup_atom_input,
    )
    .await;

  for ((agent, conductor, cell), content, value, target) in data {
    // CREATE TRUST ATOM

    let trust_atom_input = TestHelperTrustAtomInput {
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

  let me = AnyLinkableHash::from(agent_me.clone());

  assert_eq!(
    rollup_atoms,
    vec![
      TrustAtom {
        source_entry_hash: me.clone(),
        target_entry_hash: AnyLinkableHash::from(telos_entry_hash),
        prefix: Some("rollup".to_string()),
        content: Some("engineering".to_string()),
        value: Some("0.828443077244487".to_string()), // YMMV
        extra: None,
      },
      TrustAtom {
        source_entry_hash: me.clone(),
        target_entry_hash: AnyLinkableHash::from(hia_entry_hash),
        prefix: Some("rollup".to_string()),
        content: Some("holochain".to_string()),
        value: Some("0.9393462684191715".to_string()), // YMMV
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
// >>>>>>> 17a26f1 (add label (type) to TA)

// TESTING UTILITY FUNCTIONS

async fn setup_1_conductor() -> (SweetConductor, AgentPubKey, SweetCell) {
  let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
    .await
    .unwrap();

  let mut conductor = SweetConductor::from_standard_config().await;

  let holo_core_agent = SweetAgents::one(conductor.keystore()).await;
  let app1 = conductor
    .setup_app_for_agent("app", holo_core_agent.clone(), &[dna.clone()])
    .await
    .unwrap();

  let cell1 = app1.into_cells()[0].clone();

  let agent_hash = holo_core_agent.into_inner();
  let agent = AgentPubKey::from_raw_39(agent_hash).unwrap();

  (conductor, agent, cell1)
}

pub async fn setup_conductors(n: usize) -> (SweetConductorBatch, Vec<AgentPubKey>, SweetAppBatch) {
  let dna = SweetDnaFile::from_bundle(std::path::Path::new(DNA_FILEPATH))
    .await
    .unwrap();

  let mut conductors = SweetConductorBatch::from_standard_config(n).await;

  let all_agents1: Vec<holochain::core::AgentPubKey> =
    future::join_all(conductors.iter().map(|c| SweetAgents::one(c.keystore()))).await;

  let all_agents2: Vec<AgentPubKey> = all_agents1
    .iter()
    .map(|holo_core_agent| {
      let agent_hash = holo_core_agent.clone().into_inner();
      AgentPubKey::from_raw_39(agent_hash).unwrap()
    })
    .collect();

  let apps = conductors
    .setup_app_for_zipped_agents("app", &all_agents1, &[dna])
    .await
    .unwrap();

  conductors.exchange_peer_info().await;
  (conductors, all_agents2, apps)
}
