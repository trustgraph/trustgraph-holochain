use hdk::prelude::*;
use hc_zome_trust_atom::*;
use hc_zome_trust_atom::TrustAtom;
use std::collections::hash_map;

#[derive(Debug, Clone, Default, Serialize, Deserialize, SerializedBytes)]
struct TrustGraph {
  trust_atoms: BTreemap<u64, TrustAtom>, //listed by id
  rollup: TrustAtom, // algorithm from agents you placed TA for given filter
}

impl TrustGraph {
    fn create(agents: Vec<EntryHash>, tag_filter: Option<LinkTag>) -> ExternResult<Self> { // TODO: construct agent list for input. Enum?
        let base_address: EntryHashB64 = agent_info()?.agent_latest_pubkey.into();
        let links: Vec<Link> = get_links(base_address.clone(), tag_filter.clone())?;
        let trust_atoms_vec = convert_links_to_trust_atoms(base_address.clone(), links.clone())?;
        let trust_atoms = BTreeMap::new();
        for ta in trust_atoms_vec.clone() {
          trust_atoms.insert(ta.id, ta);
        }
        let trust_atom = trust_atoms_vec.pop(); //any TA should give target info.. TODO: verify
        let algo;
        let trust_atoms_collection = BTreeMap::new();
        for agent in agents {
          let agent_links = get_links(agent.clone(), tag_filter)?;
          let agent_trust_atoms = convert_links_to_trust_atoms(agent.clone() agent_links)?;
          trust_atoms_collection.insert(agent.clone(), agent_trust_atoms.clone());
          let sum;
          for ta in agent_trust_atoms { // why would there ever be more than one? is there a risk of other agents mislabeling?
            let float_val = ta.value.parse()?;
            if float_val > 0 {
            sum += float_val^1.618; // raised to PHI smooths out weights curve
            }
          }
            }
          algo += sum;
        };
        let rollup = trust_atom::create(
          trust_atom.target_entry_hash, trust_atom.label, None, algo, trust_atoms_collection
        );
        let trust_graph = TrustGraph {
          trust_atoms,
          rollup,
        }
        Ok(trust_graph)
    fn update(&self, insert: Option<KeyValue>, remove: Option<KeyValue>) -> Self {
      self.insert(insert.key, insert.val);
      self.remove(remove.key, remove.val);
      self
    }
  }

fn convert_links_to_trust_atoms(base: EntryHash, links: Vec<Link>) -> ExternResult<Vec<TrustAtom>> {
  let trust_atoms_result = links
    .iter()
    .map(|link| convert_link_to_trust_atom(base,link))
    .collect();
  let trust_atoms = trust_atoms_result?;
  Ok(trust_atoms)
}

fn convert_link_to_trust_atom(base: EntryHash, link: Link) -> ExternResult<TrustAtom> {
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
  let label = chunks[0].to_string();
  let content = chunks[1][tg_link_tag_header_length()..].to_string(); // drop leading "Ŧ→" or "Ŧ↩"
  let value = chunks[2].to_string();
  let extra = chunks[3].clone().to_string();

  let trust_atom = TrustAtom {
        source_name, //TODO: method for associating name with entryhash
        target_name, 
        source_entry_hash: base,
        target_entry_hash: link.target,
        label: Some(label), 
        content: Some(content),
        value: Some(value),
        extra: Some(extra)
      };
  Ok(trust_atom)
}