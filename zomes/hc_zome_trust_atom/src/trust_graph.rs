use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;

use std::collections::BTreeMap;

use crate::trust_atom::{
  _create_trust_atom, convert_link_to_trust_atom, convert_links_to_trust_atoms, create_link_tag,
  query_mine, LinkDirection, TrustAtom,
};

#[derive(Clone, Debug)]
struct RollupData {
  content: String,
  value: String,
  agent_rating: Option<String>,
}

pub fn create_rollup_atoms() -> ExternResult<Vec<TrustAtom>> {
  let me: EntryHash = EntryHash::from(agent_info()?.agent_latest_pubkey);
  let my_atoms: Vec<TrustAtom> = query_mine(None, None, None, None, None)?;
  let agents = build_agent_list(my_atoms.clone())?;

  let rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>> =
    build_rollup_silver(&me, my_atoms, agents)?;
  let rollup_gold: Vec<TrustAtom> = build_rollup_gold(rollup_silver, me)?;
  Ok(rollup_gold)
}

fn build_agent_list(atoms: Vec<TrustAtom>) -> ExternResult<Vec<EntryHash>> {
  let mut agents: Vec<AnyLinkableHash> = Vec::new();

  let chunks = [Some("rollup".to_string())];
  let filter = create_link_tag(&LinkDirection::Forward, &chunks);

  for ta in atoms {
    let entry_hash = AnyLinkableHash::from(ta.target_entry_hash);
    let rollup_links: Vec<Link> = get_links(entry_hash.clone(), Some(filter.clone()))?; // NOTE: Agent must have done at least one rollup
    if !rollup_links.is_empty() && !agents.contains(&entry_hash) {
      // prevent duplicates
      agents.push(entry_hash);
    }
  }
  Ok(agents)
}

#[allow(clippy::needless_pass_by_value)]
fn build_rollup_silver(
  me: &EntryHash,
  atoms: Vec<TrustAtom>,
  agents: Vec<EntryHash>,
) -> ExternResult<BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>>> {
  let mut rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>> = BTreeMap::new(); // K: Target (EntryHash) V: BTreeMap<Agent, RollupData>
  let targets: Vec<EntryHash> = atoms
    .into_iter()
    .map(|x| EntryHash::from(x.target_entry_hash))
    .collect();

  for target in targets.clone() {
    if &target != me && !agents.contains(&target) {
      let links = get_links(target.clone(), None)?;
      let mut links_latest = Vec::new();
      for link in links.clone() {
        let latest = get_latest(&target, &link.target, None)?;
        if let Some(latest) = latest {
          if !links_latest.contains(&latest) {
            // debug!("latest: {:?}", latest);
            links_latest.push(latest);
          }
        }
      }
      let trust_atoms_latest =
        convert_links_to_trust_atoms(links_latest, &LinkDirection::Reverse, &target)?;
      let mut map: BTreeMap<EntryHash, RollupData> = BTreeMap::new();
      for ta in trust_atoms_latest.clone() {
        let source = EntryHash::from(ta.source_entry_hash);
        if agents.contains(&source) {
          // get only Agent TAs
          if let Some(content) = ta.content {
            if let Some(value) = ta.value {
              // ignore content without a rating

              let chunks = [None, Some(content.clone())];

              let filter = create_link_tag(&LinkDirection::Forward, &chunks); // NOTE: filter by content broken if mislabeled
              let agent_rating: Option<String> = get_rating(me, &source, Some(filter))?;
              if let Some(rating) = agent_rating {
                let rating_ok = match rating.parse::<f64>() {
                  Ok(r) => r,
                  Err(_) => return Err(WasmError::Guest("failed to parse".to_string())),
                };
                if rating_ok > 0.0 {
                  // retain only positively rated agents
                  let rollup_data = RollupData {
                    content,
                    value,
                    agent_rating: Some(rating),
                  };
                  map.insert(source.clone(), rollup_data);
                }
              }
            }
          }
        }
      }
      rollup_silver.insert(target, map);
    }
  }
  Ok(rollup_silver)
}

#[allow(clippy::needless_pass_by_value)]
fn build_rollup_gold(
  rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>>,
  me: EntryHash,
) -> ExternResult<Vec<TrustAtom>> {
  let mut rollup_gold: Vec<TrustAtom> = Vec::new();
  for (target, map) in rollup_silver.clone() {
    let mut sourced_trust_atoms: BTreeMap<String, String> = BTreeMap::new(); // collect to input for rollup extra field
    let mut accumulator: Vec<f64> = Vec::new(); // gather weighted values
    let mut agent_rating_phi_sum: f64 = 0.0; // raised to PHI allows for smooth weight curve
                                             // debug!("map: {:#?}", map);
    for (agent, rollup_data) in map.clone() {
      // debug!("data: {:#?}", rollup_data);
      if let Some(rating) = rollup_data.agent_rating {
        agent_rating_phi_sum += rating.parse::<f64>().expect("Parse Error").powf(1.618);
      }
      let link_latest = get_latest(&agent, &target, None)?;
      if let Some(latest) = link_latest {
        let sourced_atom_latest =
          convert_link_to_trust_atom(latest, &LinkDirection::Forward, &agent)?;
        sourced_trust_atoms.insert(
          sourced_atom_latest.source_entry_hash.to_string(),
          sourced_atom_latest.target_entry_hash.to_string(),
        );
      }
    }
    let sourced_atoms: Option<BTreeMap<String, String>> = {
      if sourced_trust_atoms.is_empty() {
        Some(sourced_trust_atoms)
      } else {
        None
      }
    };

    for (_agent, rollup_data) in map.clone() {
      if let Some(rating) = rollup_data.agent_rating {
        let calc: f64 = (rating.parse::<f64>().expect("Parse Error").powf(1.618)
          / agent_rating_phi_sum)
          * rollup_data.value.parse::<f64>().expect("Parse Error");
        accumulator.push(calc);
      }
    }

    let my_rating: Option<String> = get_rating(&me, &target, None)?;
    let weighted_sum: f64 = accumulator.iter().sum();
    let content: Option<String> = {
      // TODO: cleanup get content method by adding TA.target_name String
      let get_latest = get_latest(&me, &target, None)?;
      match get_latest {
        Some(link) => convert_link_to_trust_atom(link, &LinkDirection::Forward, &me)?.content,
        None => None,
      }
    };
    if let Some(rating) = my_rating {
      let parsed: f64 = rating.parse::<f64>().expect("Parse Error");
      let algo: f64 = (weighted_sum - parsed).mul_add(0.20, parsed); // self weight is 80%
      let rollup_atom = _create_trust_atom(
        me.clone(),
        target.clone(),
        Some("rollup".to_string()),
        content.clone(),
        Some(algo.to_string()),
        None, //sourced_atoms.clone(),
      )?;
      rollup_gold.push(rollup_atom);
    } else {
      #[allow(clippy::pedantic)]
      // if no self rating for target then avg the other agents weighted values
      let total = accumulator.len() as f64;

      let algo: f64 = weighted_sum / total;
      let rollup_atom = _create_trust_atom(
        me.clone(),
        target.clone(),
        Some("rollup".to_string()),
        content.clone(),
        Some(algo.to_string()),
        sourced_atoms.clone(),
      )?;
      rollup_gold.push(rollup_atom);
    }
  }
  Ok(rollup_gold)
}

fn get_rating(
  base: &EntryHash,
  target: &EntryHash,
  tag_filter: Option<LinkTag>,
) -> ExternResult<Option<String>> {
  let link_latest = get_latest(base, target, tag_filter)?;
  if let Some(latest) = link_latest {
    let trust_atom_latest = convert_link_to_trust_atom(latest, &LinkDirection::Forward, base)?;
    // debug!("latest rating: {:?}", trust_atom_latest.value);
    return Ok(trust_atom_latest.value);
  }
  Ok(None)
}

fn get_latest(
  base: &EntryHash,
  target: &EntryHash,
  tag_filter: Option<LinkTag>,
) -> ExternResult<Option<Link>> {
  let mut links: Vec<Link> = get_links(base.clone(), tag_filter)?
    .into_iter()
    .filter(|x| x.target == *target)
    .collect();
  // debug!("get_latest_inks: {:?}", links);
  links.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)); // ignoring nanoseconds
  let latest = links.pop();
  // debug!("latest: {:?}", latest);
  match latest {
    Some(link) => Ok(Some(link)),
    None => Ok(None),
  }
}

// fn create_rollup_atoms() {

// rollup_bronze: map = {
//     HIA Entry hash (target): [  // target from my TAs or the rollups of agents in my TG
//
//         {
//             // trust atom:
//             source: zippy
//             value: float
//             content: holochain

//            // plus my rating of the "source" agent
//             agent_rating: float // my rating of zippy on `holochain`
//         }
//     ]
//  }

// rollup_silver: map = {
//     HIA Entry hash (target): [  // target from my TAs or the rollups of agents in my TG
//         {
//             content: [
//                 // latest rating by given agent:
//                  {
//                     source: zippy
//                     value: float
//                     // plus my rating of the "source" agent:
//                     agent_rating: float // my rating of zippy on `holochain`
//               }
//           ]
//        }
//     ]
//  }

// gold:
// rollup_gold: vec<TrustAtom>  = [
//     {
//       source: me
//       type: rollup
//       target: HIA Entry hash:
//       value: float
//       content: holochain
//     }
// ]

// for item in rollup_gold:
//      create_link for each

// }

// ALTERNATE STRATEGY (no agent registry, no ablity to identify whether entry is agentpubkey)

// for TA in my TAs
//   rollup_links = get_links(source: TA.target, type: "rollup")  // returns rollups from agents who have done rollups, but [] from non-agent entries
//
//
