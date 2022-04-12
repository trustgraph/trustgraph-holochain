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
  let my_atoms: Vec<TrustAtom> = query_mine(None, None, None, None)?;
  debug!("my_atoms: {:#?}", my_atoms);
  let agents = build_agent_list(my_atoms.clone())?;
  //// for tests only ////
  // let mut vec = Vec::new();
  // for agent in agents.clone() {
  //   let mut vec2 = Vec::new();
  //   let links = get_links(agent.clone(), None)?;
  //   for link in links {
  //     let bytes = link.tag.into_inner();
  //     let tag: Option<String> = String::from_utf8(bytes.clone()).ok();
  //     vec2.push(tag);
  //   }
  //   vec.push(vec2);
  // }
  // debug!("agent_links: {:#?}", vec);
  ////////
  
  // TODO: feature: general agent rating for all things (not just for specific content)

  let rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>> = build_rollup_silver(&me, my_atoms, agents)?;
  let rollup_gold: Vec<TrustAtom> = build_rollup_gold(rollup_silver, me)?;
  Ok(rollup_gold)
}

fn build_agent_list(atoms: Vec<TrustAtom>) -> ExternResult<Vec<EntryHash>> {
  
  let mut agents: Vec<EntryHash> = Vec::new();

  let chunks = [Some("rollup".to_string())];
    let filter = create_link_tag(&LinkDirection::Forward, &chunks);
    // debug!(
    //   "filter: {:?}",
    //   String::from_utf8_lossy(&filter.clone().into_inner())
    // );
  for ta in atoms {
    let entry_hash = EntryHash::from(ta.target_entry_hash);

    let rollup_links: Vec<Link> = get_links(
      entry_hash.clone(),
      Some(filter.clone()),
    )?; // NOTE: Agent must have done at least one rollup

    // debug!("rollup_links: {:?}", rollup_links);
    if rollup_links.len() > 0 {
        // for link in rollup_links {
        // debug!(
        //   "rollup_link.tag: {:?}",
        //   String::from_utf8_lossy(&link.tag.clone().into_inner())
        // );
        // }
        if !agents.contains(&entry_hash) { // prevent duplicates
          agents.push(entry_hash);
        }
      }
    }
  debug!("agents: {:?}", agents);
  Ok(agents)
}

fn build_rollup_silver(
  me: &EntryHash,
  atoms: Vec<TrustAtom>,
  agents: Vec<EntryHash>,
) -> ExternResult<BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>>> {
  let mut rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>> = BTreeMap::new(); // K: Target (EntryHash) V: BTreeMap<Agent, RollupData>
  let targets: Vec<EntryHash> = atoms.into_iter().map(|x| EntryHash::from(x.target_entry_hash)).collect();
  debug!("targets: {:?}", targets); 
  for target in targets.clone() { 
    if &target != me && !agents.contains(&target) {
      let links = get_links(target.clone(), None)?; // OPTION1: filter_map source by agent list
      // debug!("target_links: {:?}", links);
      let mut links_latest = Vec::new();
      //// tests only ////
      let mut vec = Vec::new();
      for link in links.clone() {
        let bytes = link.tag.into_inner();
        let tag: Option<String> = String::from_utf8(bytes.clone()).ok();
        vec.push(tag);
      }
      debug!("silver_links: {:#?}", vec);
      ///////
      for link in links.clone() {
        let latest = get_latest(target.clone(), link.target, None)?;
        if let Some(latest) = latest {
          if !links_latest.contains(&latest) {
            // debug!("latest: {:?}", latest);
          links_latest.push(latest);
          }
        }
      }
      // debug!("links_latest: {:?}", links_latest);
    let trust_atoms_latest =
      convert_links_to_trust_atoms(links_latest, &LinkDirection::Reverse, &target)?;
    let mut map: BTreeMap<EntryHash, RollupData> = BTreeMap::new();
      debug!("TA latest: {:#?}", trust_atoms_latest);
    for ta in trust_atoms_latest.clone() {
      let source = EntryHash::from(ta.source_entry_hash);
      if agents.contains(&source) { // OPTION2: get only Agent TAs
        if let Some(content) = ta.content {
          if let Some(value) = ta.value {
            // ignore content without a rating

            let chunks = [
              None, // ?TODO: agent prefix
              Some(content.clone()),
            ]; 

            let filter = create_link_tag(&LinkDirection::Forward, &chunks); // NOTE: filter by content broken if mislabeled
            // debug!("tag_filter: {:?}", String::from_utf8_lossy(&filter.clone().into_inner()));
            let agent_rating: Option<String> = get_rating(me.clone(), source.clone(), Some(filter))?; 
            // debug!("agent_rating: {:?}", agent_rating);
            if let Some(rating) = agent_rating {
              if rating.parse::<f64>().unwrap() > 0.0 {
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
    debug!("Map: {:?}", map);
    rollup_silver.insert(target, map);
    }
    
  }
  debug!("silver: {:#?}", rollup_silver);
  Ok(rollup_silver)
}

fn build_rollup_gold(
  rollup_silver: BTreeMap<EntryHash, BTreeMap<EntryHash, RollupData>>,
  me: EntryHash,
) -> ExternResult<Vec<TrustAtom>> {
  let mut rollup_gold: Vec<TrustAtom> = Vec::new();
  for (target, map) in rollup_silver.clone() {
    let mut sourced_trust_atoms: BTreeMap<EntryHashB64, TrustAtom> = BTreeMap::new(); // collect to input for rollup extra field
    let mut accumulator: Vec<f64> = Vec::new(); // gather weighted values
    let mut rating_sum: f64 = 0.0;

    for (agent, rollup_data) in map.clone() {
      if let Some(rating) = rollup_data.agent_rating {
        rating_sum += rating.parse::<f64>().expect("Parse Error"); // could ignore parse err and use .ok() to convert result into option 
      }
      let link_latest = get_latest(agent.clone(), target.clone(), None)?;
        if let Some(latest) = link_latest {
        let sourced_atom_latest = convert_link_to_trust_atom(
          latest,
          &LinkDirection::Forward,
          &agent,
        )?;
        sourced_trust_atoms.insert(
          sourced_atom_latest.source_entry_hash.clone(),
          sourced_atom_latest.clone(),
        );
      }
    }

    let sourced_atoms: Option<BTreeMap<EntryHashB64, TrustAtom>> = {
      if sourced_trust_atoms.len() > 0 {
        Some(sourced_trust_atoms)
      }
      else { None }
    };

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
          sourced_atoms.clone(),
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
          sourced_atoms.clone(),
        )?;
        rollup_gold.push(rollup_atom);
      }
    }
  }
  debug!("gold: {:#?}", rollup_gold);
  Ok(rollup_gold)
}

fn get_rating(
  base: EntryHash,
  target: EntryHash,
  tag_filter: Option<LinkTag>,
) -> ExternResult<Option<String>> {
  let link_latest = get_latest(base.clone(), target, tag_filter)?;
  if let Some(latest) = link_latest {
    let trust_atom_latest = convert_link_to_trust_atom(latest, &LinkDirection::Forward, &base)?;
    // debug!("latest rating: {:?}", trust_atom_latest.value);
    return Ok(trust_atom_latest.value)
  }
  Ok(None)
}

fn get_latest(
  base: EntryHash,
  target: EntryHash,
  tag_filter: Option<LinkTag>,
) -> ExternResult<Option<Link>> {
  let mut links: Vec<Link> = get_links(base, tag_filter)?.into_iter().filter(|x| x.target == target).collect();
  // debug!("links: {:?}", links);
    links.sort_by(|a, b| a.timestamp.cmp(&b.timestamp)); // ignoring nanoseconds
    let latest = links.pop();
    match latest {
      Some(link) => return Ok(Some(link)),
      None => return Ok(None)
    }
  }
  // let mut timestamps = Vec::new(); ////
  // for link in links.clone() {
  //   if link.target == target {
  //     timestamps.push(link.timestamp);
  //   }
  // }
  // if !timestamps.is_empty() {
  //   timestamps.sort_by(|a, b| a.cmp(b)); // ignoring nanoseconds
  //   debug!("timestamps: {:?}", timestamps);
  //   let latest = timestamps.pop().expect("Timestamp vec shouldn't be empty");
  //   let mut latest_link: Vec<Link> = links
  //     .into_iter()
  //     .filter(|x| x.timestamp == latest);
  //   if latest_link.len() == 1 {
  //     // should always be one
  //     let link = latest_link.pop().expect("Error");
  //     return Ok(Some(link));
  //   } else {
  //     return Err(WasmError::Guest("Something went wrong".to_string()));
  //   }
  // } ////

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
