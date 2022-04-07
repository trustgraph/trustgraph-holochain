use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;

use std::collections::BTreeMap;

use crate::*;

#[derive(Clone, Debug)]
struct RollupData {
  content: String,
  value: String,
  agent_rating: Option<String>,
}

pub fn create_rollup_atoms() -> ExternResult<Vec<TrustAtom>> {
  let me: EntryHash = EntryHash::from(agent_info()?.agent_latest_pubkey);
  let agents = build_agent_list()?;

  // TODO: feature: general agent rating for all things (not just for specific content)

  let rollup_silver = build_rollup_silver(agents, &me)?;
  let rollup_gold = build_rollup_gold(rollup_silver, me)?;
  Ok(rollup_gold)
}

fn build_agent_list() -> ExternResult<Vec<HoloHash<holo_hash::hash_type::Entry>>> {
  let my_trust_atoms: Vec<TrustAtom> = query_mine(None, None, None, None)?;
  let mut agents: Vec<EntryHash> = Vec::new();
  for ta in my_trust_atoms.clone() {
    let agent_entry_hash = EntryHash::from(ta.target_entry_hash);
    let chunks = [Some("rollup".to_string())];
    let filter = create_link_tag(&LinkDirection::Forward, &chunks);

    // debug!(
    //   "filter: {:?}",
    //   String::from_utf8_lossy(&filter.clone().into_inner())
    // );

    let rollup_links: Vec<Link> = get_links(
      agent_entry_hash.clone(),
      //Some(filter),
      None
    )?; // Note: Agent must have done at least one rollup

    if rollup_links.len() > 0 {
        // debug!("rollup_link: {:?}", rollup_links);
        // debug!(
        //   "rollup_link.tag: {:?}",
        //   String::from_utf8_lossy(&rollup_link.tag.clone().into_inner())
        // );
        if !agents.contains(&agent_entry_hash) {
          agents.push(agent_entry_hash);
        }
      }
    }
  // debug!("agents: {:?}", agents);
  Ok(agents)
}

fn build_rollup_silver(
  agents: Vec<HoloHash<holo_hash::hash_type::Entry>>,
  me: &HoloHash<holo_hash::hash_type::Entry>,
) -> ExternResult<BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>>> {
  let mut rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>> = BTreeMap::new(); // K: Target (EntryHash) V: BTreeMap<Agent, RollupData>

  for agent in agents {
    let links = get_links(agent.clone(), None)?;

    let mut links_latest = Vec::new();

    for link in links {
      let latest = get_latest(agent.clone(), link.target, None)?;
      if !links_latest.contains(&latest) {
      links_latest.push(latest);
      }
    }
    let trust_atoms_latest =
      convert_links_to_trust_atoms(links_latest, &LinkDirection::Forward, &agent)?;

    for ta in trust_atoms_latest {
      let target_entry_hash = EntryHash::from(ta.target_entry_hash);
      if let Some(content) = ta.content {
        if let Some(value) = ta.value {
          // ignore content without a rating

          let chunks = [
            None, // ?TODO: agent prefix
            Some(content.clone()),
          ];
          let filter = create_link_tag(&LinkDirection::Forward, &chunks);
          let agent_rating: Option<String> = get_rating(me.clone(), agent.clone(), Some(filter))?;

          if let Some(rating) = agent_rating {
            if rating.parse::<f64>().unwrap() > 0.0 {
              // retain only positively rated agents
              let rollup_data = RollupData {
                content,
                value,
                agent_rating: Some(rating),
              };
              let map: BTreeMap<EntryHash, RollupData> =
                BTreeMap::from([(agent.clone(), rollup_data)]);
              rollup_silver.insert(target_entry_hash, map);
            }
          }
        }
      }
    }
  }
  debug!("silver: {:?}", rollup_silver);
  Ok(rollup_silver)
}

fn build_rollup_gold(
  rollup_silver: BTreeMap<
    HoloHash<holo_hash::hash_type::Entry>,
    BTreeMap<HoloHash<holo_hash::hash_type::Entry>, RollupData>,
  >,
  me: HoloHash<holo_hash::hash_type::Entry>,
) -> ExternResult<Vec<TrustAtom>> {
  let mut rollup_gold: Vec<TrustAtom> = Vec::new();
  for (target, map) in rollup_silver.clone() {
    let mut sourced_trust_atoms: BTreeMap<EntryHashB64, TrustAtom> = BTreeMap::new(); // collect to input for rollup extra field
    let mut accumulator: Vec<f64> = Vec::new(); // gather weighted values
    let mut rating_sum: f64 = 0.0;

    for (agent, rollup_data) in map.clone() {
      if let Some(rating) = rollup_data.agent_rating {
        rating_sum += rating.parse::<f64>().expect("Parse Error");
      }
      let sourced_atom_latest = convert_link_to_trust_atom(
        get_latest(agent.clone(), target.clone(), None)?,
        &LinkDirection::Forward,
        &agent,
      )?;
      sourced_trust_atoms.insert(
        sourced_atom_latest.source_entry_hash.clone(),
        sourced_atom_latest.clone(),
      );
    }

    for (_agent, rollup_data) in map.clone() {
      if let Some(rating) = rollup_data.agent_rating {
        let calc: f64 = (rating.parse::<f64>().expect("Parse Error") / rating_sum)
          * rollup_data.value.parse::<f64>().expect("Parse Error");
        accumulator.push(calc);
      }

      let my_rating: Option<String> = get_rating(me.clone(), target.clone(), None)?;
      let sum: f64 = accumulator.iter().sum();

      if let Some(rating) = my_rating {
        let parsed: f64 = rating.parse::<f64>().expect("Parse Error");
        let algo: f64 = parsed * 0.80 + sum * 0.20; // self weight is 80%
        let rollup_atom = create_trust_atom(
          me.clone(),
          target.clone(),
          Some("rollup".to_string()),
          Some(rollup_data.content),
          Some(algo.to_string()),
          Some(sourced_trust_atoms.clone()),
        )?;
        rollup_gold.push(rollup_atom);
      } else {
        // if no self rating for target then avg the other agents weighted values
        let total = accumulator.len() as f64;
        let algo: f64 = sum / total;
        let rollup_atom = create_trust_atom(
          me.clone(),
          target.clone(),
          Some("rollup".to_string()),
          Some(rollup_data.content),
          Some(algo.to_string()),
          Some(sourced_trust_atoms.clone()),
        )?;
        rollup_gold.push(rollup_atom);
      }
    }
  }
  // debug!("gold: {:?}", rollup_gold);
  Ok(rollup_gold)
}

fn get_rating(
  base: EntryHash,
  target: EntryHash,
  tag_filter: Option<LinkTag>,
) -> ExternResult<Option<String>> {
  let link_latest = get_latest(base.clone(), target, tag_filter)?;
  let trust_atom_latest = convert_link_to_trust_atom(link_latest, &LinkDirection::Forward, &base)?;
  Ok(trust_atom_latest.value)
}

fn get_latest(
  base: EntryHash,
  target: EntryHash,
  tag_filter: Option<LinkTag>,
) -> ExternResult<Link> {
  let links = get_links(base, tag_filter)?;
  let mut timestamps = Vec::new();
  for link in links.clone() {
    if link.target == target {
      timestamps.push(link.timestamp);
    }
  }

  timestamps.sort_by(|a, b| a.cmp(b)); // ignoring nanoseconds
  let latest = timestamps.pop().expect("Error");
  let mut latest_link: Vec<Link> = links
    .into_iter()
    .filter(|x| x.timestamp == latest)
    .collect();
  if latest_link.len() == 1 {
    // should always be one
    let link = latest_link.pop().expect("Error");
    return Ok(link);
  } else {
    return Err(WasmError::Guest("Something went wrong".to_string()));
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
