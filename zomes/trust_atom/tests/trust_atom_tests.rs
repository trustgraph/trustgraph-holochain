#![warn(warnings)]

use std::collections::BTreeMap;

use futures::future;
use serial_test::serial;

use hdk::prelude::*;
use holochain::conductor::config::ConductorConfig;
use holochain::sweettest::{SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile, SweetZome};
use holochain::test_utils::consistency_10s;

use trust_atom_integrity::Example;
use trust_atom_types::TrustAtomInput;

const DNA_FILEPATH: &str = "../../workdir/dna/trust_atom_dna.dna";
const ZOME_NAME: &str = "trust_atom";

#[tokio::test]
pub async fn test_unicode_null() {
    let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
    assert_eq!(unicode_nul.as_bytes(), &[0]);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_create_trust_atom() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    // let (conductor, agent, cell1): (SweetConductor, AgentPubKey, SweetCell) = setup_1_conductor().await;

    let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();
    // CREATE TARGET ENTRY

    let target_hash = ann.create_string_target("Nuka Sushi").await;

    // CREATE TRUST ATOM

    let content: String = "sushi".into();
    let value: String = ".8".into();
    let extra: BTreeMap<String, String> = BTreeMap::from([("details".into(), "Excellent specials. The regular menu is so-so. Their coconut curry (special) is to die for".into())]);

    let trust_atom_input = TrustAtomInput {
        target: AnyLinkableHash::from(target_hash.clone()),
        content: Some(content.clone()),
        value: Some(value.clone()),
        extra: Some(extra.clone()),
    };

    let _result: trust_atom_types::TrustAtom = ann.create_trust_atom(trust_atom_input).await;

    // CHECK FORWARD LINK

    let agent_address: EntryHash = ann.pubkey.clone().into();

    let forward_links: Vec<Link> = ann.test_helper_list_links_for_base(agent_address).await;

    assert_eq!(forward_links.len(), 1);
    let link = &forward_links[0];

    let target_from_link = link.clone().target;
    assert_eq!(target_from_link, AnyLinkableHash::from(target_hash.clone()));

    let link_tag_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_tag_bytes.to_vec();
    let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();

    let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0], "Ŧ→sushi");
    assert_eq!(chunks[1], ".800000000");

    let bucket = chunks[2];

    assert_eq!(bucket.chars().count(), 9);
    assert!(bucket.chars().all(|c| c.is_ascii_digit()));

    let expected_entry_hash = "uhCEkto76kYgGIZMzU6AbEzCx1HMRNzurwPaOdF2utJaP-33mdcdN";
    let expected_link_tag_string = format!("{}{}{}{}{}{}{}{}{}", "Ŧ", "→", "sushi", unicode_nul, ".800000000", unicode_nul, bucket, unicode_nul, expected_entry_hash);
    assert_eq!(relevant_link_string, expected_link_tag_string);

    // CHECK BACKWARD LINK

    let backward_links: Vec<Link> = ann.test_helper_list_links_for_base(target_hash.clone()).await;

    assert_eq!(backward_links.len(), 1);
    let link = &backward_links[0];

    // let agent_entry_hash = EntryHash::from(EntryHash::from(ann.pubkey.clone()));
    // assert_eq!(target_from_link, agent_entry_hash);

    let link_tag_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_tag_bytes.to_vec();
    let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
    let expected_link_tag_string = format!("{}{}{}{}{}{}{}{}{}", "Ŧ", "↩", "sushi", unicode_nul, ".800000000", unicode_nul, bucket, unicode_nul, expected_entry_hash);
    assert_eq!(relevant_link_string, expected_link_tag_string);

    let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0], "Ŧ↩sushi");
    assert_eq!(chunks[1], ".800000000");
    assert_eq!(chunks[2], bucket);
    assert_eq!(chunks[3], expected_entry_hash);
}
#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_create_trust_atom_with_empty_chunks() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    let unicode_nul: &str = std::str::from_utf8(&[0]).unwrap();

    // CREATE TARGET ENTRY

    let target_hash = ann.create_string_target("Nuka Sushi").await;

    // CREATE TRUST ATOM

    let trust_atom_input = TrustAtomInput {
        target: AnyLinkableHash::from(target_hash.clone()),
        content: None,
        value: None,
        extra: None,
    };

    let _result: trust_atom_types::TrustAtom = ann.create_trust_atom(trust_atom_input).await;

    // CHECK FORWARD LINK

    let agent_address: EntryHash = ann.pubkey.clone().into();

    let forward_links: Vec<Link> = ann.test_helper_list_links_for_base(agent_address).await;

    assert_eq!(forward_links.len(), 1);
    let link = &forward_links[0];

    let target_from_link = link.clone().target;
    assert_eq!(target_from_link, AnyLinkableHash::from(target_hash.clone()));

    let link_tag_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_tag_bytes.to_vec();
    let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();

    let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0], "Ŧ→");
    assert_eq!(chunks[1], "");

    let bucket = chunks[2];

    assert_eq!(bucket.chars().count(), 9);
    assert!(bucket.chars().all(|c| c.is_ascii_digit()));

    let expected_link_tag_string = format!("{}{}{}{}{}{}", "Ŧ", "→", unicode_nul, unicode_nul, bucket, unicode_nul);
    assert_eq!(relevant_link_string, expected_link_tag_string);

    // CHECK BACKWARD LINK

    let backward_links: Vec<Link> = ann.test_helper_list_links_for_base(target_hash.clone()).await;

    assert_eq!(backward_links.len(), 1);
    let link = &backward_links[0];

    let link_tag_bytes = link.clone().tag.into_inner();
    let relevant_link_bytes = link_tag_bytes.to_vec();
    let relevant_link_string = String::from_utf8(relevant_link_bytes).unwrap();
    let expected_link_tag_string = format!("{}{}{}{}{}{}", "Ŧ", "↩", unicode_nul, unicode_nul, bucket, unicode_nul);
    assert_eq!(relevant_link_string, expected_link_tag_string);

    let chunks: Vec<&str> = relevant_link_string.split(unicode_nul).collect();
    assert_eq!(chunks.len(), 4);
    assert_eq!(chunks[0], "Ŧ↩");
    assert_eq!(chunks[1], "");
    assert_eq!(chunks[2], bucket);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_query_mine() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    // CREATE TARGET ENTRY

    let target_hash = ann.create_string_target("Sushi Ran").await;

    // CREATE TRUST ATOMS

    let _result: trust_atom_types::TrustAtom = ann
        .create_trust_atom(TrustAtomInput {
            target: AnyLinkableHash::from(target_hash.clone()),
            content: Some("sushi".to_string()),
            value: Some("0.8".to_string()),
            extra: Some(BTreeMap::new()),
        })
        .await;

    // QUERY MY TRUST ATOMS

    let trust_atoms_from_query: Vec<trust_atom_types::TrustAtom> = ann
        .query_mine(trust_atom_types::QueryMineInput {
            target: None,
            content_starts_with: None,
            content_full: None,
            value_starts_with: None,
        })
        .await;

    assert_eq!(trust_atoms_from_query.len(), 1);

    // let source_hash = EntryHash::from(EntryHash::from(ann.pubkey.clone()));
    // let target_hash = EntryHash::from(target_hash);
    let trust_atom = &trust_atoms_from_query[0];

    assert_eq!(
        *trust_atom,
        trust_atom_types::TrustAtom {
            source_hash: AnyLinkableHash::from(ann.pubkey.clone()),
            target_hash: AnyLinkableHash::from(target_hash),
            content: Some("sushi".to_string()),
            value: Some(".800000000".to_string()),
            extra: Some(BTreeMap::new()),
        }
    );
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_query_mine_with_content_starts_with() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    // CREATE TARGET ENTRY

    let target_hash = ann.create_string_target("Sushi Ran").await;

    // CREATE TRUST ATOMS

    let contents = vec!["sushi", "sushi joint", "sush"];

    for content in contents {
        let _result: trust_atom_types::TrustAtom = ann
            .create_trust_atom(TrustAtomInput {
                target: AnyLinkableHash::from(target_hash.clone()),
                content: Some(content.into()),
                value: Some("0.8".into()),
                extra: Some(BTreeMap::new()),
            })
            .await;
    }
    // QUERY MY TRUST ATOMS

    let trust_atoms_from_query: Vec<trust_atom_types::TrustAtom> = ann
        .query_mine(trust_atom_types::QueryMineInput {
            target: None,
            content_full: None,
            content_starts_with: Some("sushi".into()),
            value_starts_with: None,
            // value_starts_with: Some("0.0".into()),
        })
        .await;

    assert_eq!(trust_atoms_from_query.len(), 2);

    let mut actual = [trust_atoms_from_query[0].clone().content, trust_atoms_from_query[1].clone().content];
    actual.sort();

    assert_eq!(actual, [Some("sushi".to_string()), Some("sushi joint".to_string())]);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_query_mine_with_content_full() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    // CREATE TARGET ENTRY

    let target_hash = ann.create_string_target("Sushi Ran").await;

    // CREATE TRUST ATOMS

    let content_fulls = vec!["sushi", "sushi joint", "sush"];

    for content_full in content_fulls {
        let _result: trust_atom_types::TrustAtom = ann
            .create_trust_atom(TrustAtomInput {
                target: AnyLinkableHash::from(target_hash.clone()),
                content: Some(content_full.into()),
                value: Some("0.8".into()),
                extra: Some(BTreeMap::new()),
            })
            .await;
    }
    // QUERY MY TRUST ATOMS

    let trust_atoms_from_query: Vec<trust_atom_types::TrustAtom> = ann
        .query_mine(trust_atom_types::QueryMineInput {
            target: None,
            content_full: Some("sushi".into()),
            content_starts_with: None,
            value_starts_with: None,
            // value_starts_with: Some("0.0".into()),
        })
        .await;

    assert_eq!(trust_atoms_from_query.len(), 1);

    assert_eq!(trust_atoms_from_query[0].clone().content, Some("sushi".to_string()));
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_get_extra() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    let target_hash = ann.create_string_target("Nuka Sushi").await;

    let mock_input = TrustAtomInput {
        target: target_hash,
        content: Some("sushi".to_string()),
        value: Some("0.9871".to_string()),
        extra: Some(BTreeMap::from([("extra_stuff".to_string(), "Say some extra stuff here".to_string()), ("another_thing".to_string(), "Put more information here".to_string())])),
    };

    let _mock_trust_atom: trust_atom_types::TrustAtom = ann.create_trust_atom(mock_input.clone()).await;

    let mock_entry = trust_atom_integrity::Extra { fields: mock_input.extra.unwrap() };
    let mock_extra_entry_hash: EntryHash = ann.calc_extra_hash(mock_entry).await;

    let mock_extra_data: trust_atom_integrity::Extra = ann.get_extra(mock_extra_entry_hash).await;

    let field1 = mock_extra_data.fields.get_key_value(&"extra_stuff".to_string()).unwrap();
    let field2 = mock_extra_data.fields.get_key_value(&"another_thing".to_string()).unwrap();

    assert_eq!(field1, (&"extra_stuff".to_string(), &"Say some extra stuff here".to_string()));
    assert_eq!(field2, (&"another_thing".to_string(), &"Put more information here".to_string()));
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
pub async fn test_get_entry_by_actionhash() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    let test_entry = trust_atom_integrity::Example { example_field: "test".to_string() };

    let action_hash: ActionHash = ann.create_test_entry(test_entry).await;

    let retrieval: trust_atom_integrity::Example = ann.test_get_entry_by_action(action_hash).await;
    assert_eq!("test".to_string(), retrieval.example_field);
}

//
// ^^^ TESTS: ^^^
//
// vvv TEST HELPERS: vvv
//

pub struct Agent<'a> {
    pub cell: SweetCell,
    pub conductor: &'a SweetConductor,
    pub pubkey: AgentPubKey,
    pub zome: SweetZome,
}

impl Agent<'_> {
    // public zome functions

    pub async fn create_trust_atom(&self, input: TrustAtomInput) -> trust_atom_types::TrustAtom {
        self.conductor.call(&self.zome, "create_trust_atom", input).await
    }

    pub async fn query_mine(&self, input: trust_atom_types::QueryMineInput) -> Vec<trust_atom_types::TrustAtom> {
        self.conductor.call(&self.zome, "query_mine", input).await
    }

    // test helpers

    pub async fn calc_extra_hash(&self, input: trust_atom_integrity::Extra) -> EntryHash {
        self.conductor.call(&self.zome, "calc_extra_hash", input).await
    }

    pub async fn create_string_target<I: Into<String>>(&self, input: I) -> AnyLinkableHash {
        let payload: String = input.into();
        let result: EntryHash = self.conductor.call(&self.zome, "create_string_target", payload).await;
        AnyLinkableHash::from(result)
    }

    pub async fn create_test_entry(&self, input: trust_atom_integrity::Example) -> ActionHash {
        self.conductor.call(&self.zome, "create_test_entry", input).await
    }

    pub async fn get_extra(&self, input: EntryHash) -> trust_atom_integrity::Extra {
        self.conductor.call(&self.zome, "get_extra", input).await
    }

    pub async fn test_get_entry_by_action(&self, input: ActionHash) -> Example {
        self.conductor.call(&self.zome, "test_get_entry_by_action", input).await
    }

    pub async fn test_helper_list_links_for_base<I: Into<AnyLinkableHash>>(&self, input: I) -> Vec<Link> {
        let payload: AnyLinkableHash = input.into();
        self.conductor.call(&self.zome, "test_helper_list_links_for_base", payload).await
    }

    // pub async fn recommended(&self, input: RecommendedInput) -> Vec<TrustFeedMew> {
    //     self.conductor.call(&self.zome, "recommended", input).await
    // }
}

pub struct AgentGroup {
    conductors: SweetConductorBatch,
}

impl AgentGroup {
    #[allow(clippy::needless_lifetimes)]
    pub async fn create_agents<'a>(&'a mut self) -> Vec<Agent<'a>> {
        let dna_path = std::env::current_dir().unwrap().join(DNA_FILEPATH);
        let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

        let apps = self.conductors.setup_app(ZOME_NAME, &[dna]).await.unwrap();
        self.conductors.exchange_peer_info().await;

        let ((ann_cell,), (bob_cell,), (cat_cell,)) = apps.into_tuples();

        let ann = Agent {
            cell: ann_cell.clone(),
            conductor: self.conductors.get(0).unwrap(),
            pubkey: ann_cell.agent_pubkey().clone(),
            zome: ann_cell.zome(ZOME_NAME),
        };
        let bob = Agent {
            cell: bob_cell.clone(),
            conductor: self.conductors.get(1).unwrap(),
            pubkey: bob_cell.agent_pubkey().clone(),
            zome: bob_cell.zome(ZOME_NAME),
        };
        let cat = Agent {
            cell: cat_cell.clone(),
            conductor: self.conductors.get(2).unwrap(),
            pubkey: cat_cell.agent_pubkey().clone(),
            zome: cat_cell.zome(ZOME_NAME),
        };

        vec![ann, bob, cat]
    }
}

pub async fn setup() -> AgentGroup {
    let conductors = SweetConductorBatch::from_config(3, ConductorConfig::default()).await;
    AgentGroup { conductors }
}
