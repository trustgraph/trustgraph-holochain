use hdk::prelude::*;

use std::collections::hash_map;

use crate::KeyValue;
use crate::IOType;

#[derive(Debug, Clone, Default, Serialize, Deserialize, SerializedBytes)]
struct TrustGraph {
  trust_atoms: BTreemap<u64, TrustAtom> //listed by id
  ta_placed: BTreeMap<u64,TrustAtom> // list of TA's (K, V) from other agents you gave TA
}

impl TrustGraph {
    fn create() -> ExternResult<Self> {
        let base_address: EntryHashB64 = agent_info()?.agent_latest_pubkey.into();
        let links: Vec<Link> = get_links(base_address, None)?;
        let trust_atoms_vec = convert_links_to_trust_atoms(links);
        let trust_graph = BTreeMap::new();
        for ta in trust_atoms_vec {
          trust_graph.trust_atoms.insert(ta.id, ta);
        }
        Ok(trust_graph)
    };
    fn update(&self, insert: Option<KeyValue>, remove: Option<KeyValue>) -> Self {
      self.insert(insert.key, insert.val);
      self.remove(remove.key, remove.val);
      self
    };
    fn rollup(&self) -> ExternResult<Self> {
      get_ta_links();
    }

}

fn convert_links_to_trust_atoms(links: Vec<Link>) -> ExternResult<Vec<TrustAtom>> {
  let trust_atoms_result = links
    .iter()
    .map(|link| convert_link_to_trust_atom(link))
    .collect();
  let trust_atoms = trust_atoms_result?;
  Ok(trust_atoms)
}

fn convert_link_to_trust_atom(link: Link) -> ExternResult<TrustAtom> {
  let link_tag_bytes = link.tag.clone().into_inner();
  let link_tag = match String::from_utf8(link_tag_bytes) {
    Ok(link_tag) => link_tag,
    Err(_) => {
      return Err(WasmError::Guest(format!(
        "Link tag is not valid UTF-8 -- found: {:?}",
        String::from_utf8_lossy(&link.tag.into_inner())
      )))
    }
  };

  let chunks: Vec<&str> = link_tag.split(UNICODE_NUL_STR).collect();
  let content = chunks[0][tg_link_tag_header_length()..].to_string(); // drop leading "Ŧ→" or "Ŧ↩"
  let value = chunks[1].to_string();
  let extra = chunks[3].clone().to_string();

  let link_base_b64 = EntryHashB64::from(link_base.clone());
  let link_target_b64 = EntryHashB64::from(link.target);

  let trust_atom = match link_direction {
    LinkDirection::Forward => {
      TrustAtom {
        source: link_base_b64.to_string(),
        target: link_target_b64.to_string(),
        source_entry_hash: link_base_b64,
        target_entry_hash: link_target_b64,
        type: Some(type), 
        content: Some(content),
        value: Some(value),
        extra: Some(extra)
      }
    }
    LinkDirection::Reverse => {
      TrustAtom {
        source: link_target_b64.to_string(), 
        target: link_base_b64.to_string(),   
        source_entry_hash: link_target_b64,  
        target_entry_hash: link_base_b64,
        type: Some(type),    
        content: Some(content),
        value: Some(value),
        extra: Some(extra)
      }
    }
  };
  Ok(trust_atom)
}

get_ta_links() -> BTreeMap<Agent, TrustAtoms> {

} 