#![warn(warnings)]

use std::collections::BTreeMap;

use futures::future;

use hc_zome_trust_atom::*;
use hdk::prelude::*;
use holo_hash::AgentPubKey;
use holo_hash::AnyLinkableHash;
use holo_hash::AnyLinkableHashB64;
use holochain::sweettest::{
  SweetAgents, SweetAppBatch, SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile,
};

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom.dna";

#[tokio::test]
pub async fn test_unicode_null() {
  let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
  assert_eq!(unicode_nul.as_bytes(), &[0]);
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

  let trust_atom_input = TrustAtomInput {
    target: AnyLinkableHash::from(target_entry_hash.clone()),
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
  assert_eq!(chunks.len(), 4);
  assert_eq!(chunks[0], "Ŧ→sushi");
  assert_eq!(chunks[1], ".800000000");

  let bucket = chunks[2];

  assert_eq!(bucket.chars().count(), 9);
  assert!(bucket.chars().all(|c| c.is_digit(10)));

  let expected_entry_hash = "uhCEkto76kYgGIZMzU6AbEzCx1HMRNzurwPaOdF2utJaP-33mdcdN";
  let expected_link_tag_string = format!(
    "{}{}{}{}{}{}{}{}{}",
    "Ŧ",
    "→",
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
    "{}{}{}{}{}{}{}{}{}",
    "Ŧ",
    "↩",
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
  assert_eq!(chunks.len(), 4);
  assert_eq!(chunks[0], "Ŧ↩sushi");
  assert_eq!(chunks[1], ".800000000");
  assert_eq!(chunks[2], bucket);
  assert_eq!(chunks[3], expected_entry_hash);
}
#[tokio::test(flavor = "multi_thread")]
pub async fn test_create_trust_atom_with_empty_chunks() {
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

  let trust_atom_input = TrustAtomInput {
    target: AnyLinkableHash::from(target_entry_hash.clone()),
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

  let target_from_link = link.clone().target;
  assert_eq!(
    target_from_link,
    AnyLinkableHash::from(target_entry_hash.clone())
  );

  let link_tag_bytes = link.clone().tag.into_inner();
  let relevant_link_bytes = link_tag_bytes.to_vec();
  let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 4);
  assert_eq!(chunks[0], "Ŧ→");
  assert_eq!(chunks[1], "");

  let bucket = chunks[2];

  assert_eq!(bucket.chars().count(), 9);
  assert!(bucket.chars().all(|c| c.is_digit(10)));

  let expected_link_tag_string = format!(
    "{}{}{}{}{}{}",
    "Ŧ", "→", unicode_nul, unicode_nul, bucket, unicode_nul
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
    "{}{}{}{}{}{}",
    "Ŧ", "↩", unicode_nul, unicode_nul, bucket, unicode_nul
  );
  assert_eq!(relevant_link_string, expected_link_tag_string);

  let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
  assert_eq!(chunks.len(), 4);
  assert_eq!(chunks[0], "Ŧ↩");
  assert_eq!(chunks[1], "");
  assert_eq!(chunks[2], bucket);
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
        content: Some("sushi".to_string()),
        value: Some("0.8".to_string()),
        extra: Some(BTreeMap::new()),
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
      source_entry_hash: AnyLinkableHash::from(agent.clone()),
      target_entry_hash: AnyLinkableHash::from(target_entry_hash),
      content: Some("sushi".to_string()),
      value: Some(".800000000".to_string()),
      extra: Some(BTreeMap::new()),
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
    let _result: TrustAtom = conductor
      .call(
        &cell1.zome("trust_atom"),
        "create_trust_atom",
        TrustAtomInput {
          target: AnyLinkableHash::from(target_entry_hash.clone()),
          content: Some(content.into()),
          value: Some("0.8".into()),
          extra: Some(BTreeMap::new()),
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
    let _result: TrustAtom = conductor
      .call(
        &cell1.zome("trust_atom"),
        "create_trust_atom",
        TrustAtomInput {
          target: AnyLinkableHash::from(target_entry_hash.clone()),
          content: Some(content_full.into()),
          value: Some("0.8".into()),
          extra: Some(BTreeMap::new()),
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
  let (conductor, _agent, cell1) = setup_1_conductor().await;

  let target_entry_hash = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_string_target",
      "Nuka Sushi",
    )
    .await;

  let mock_input = TrustAtomInput {
    target: target_entry_hash,
    content: Some("sushi".to_string()),
    value: Some("0.9871".to_string()),
    extra: Some(BTreeMap::from([
      (
        "extra_stuff".to_string(),
        "Say some extra stuff here".to_string(),
      ),
      (
        "another_thing".to_string(),
        "Put more information here".to_string(),
      ),
    ])),
  };

  let _mock_trust_atom: TrustAtom = conductor
    .call(
      &cell1.zome("trust_atom"),
      "create_trust_atom",
      mock_input.clone(),
    )
    .await;

  let mock_entry = Extra {
    fields: mock_input.extra.unwrap(),
  };
  let mock_extra_entry_hash: EntryHash = conductor
    .call(&cell1.zome("trust_atom"), "calc_extra_hash", mock_entry)
    .await;

  let mock_extra_data: Extra = conductor
    .call(
      &cell1.zome("trust_atom"),
      "get_extra",
      mock_extra_entry_hash,
    )
    .await;

  let field1 = mock_extra_data
    .fields
    .get_key_value(&"extra_stuff".to_string())
    .unwrap();
  let field2 = mock_extra_data
    .fields
    .get_key_value(&"another_thing".to_string())
    .unwrap();

  assert_eq!(
    field1,
    (
      &"extra_stuff".to_string(),
      &"Say some extra stuff here".to_string()
    )
  );
  assert_eq!(
    field2,
    (
      &"another_thing".to_string(),
      &"Put more information here".to_string()
    )
  );
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
