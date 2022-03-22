use hdk::prelude::*;

// use itertools::Itertools;
use std::collections::BTreeMap;

use crate::trust_atom::LinkDirection;
use crate::trust_atom::*;

struct RollupData {
  target_hash: EntryHash,
  value: f64,
  rating: f64,
}

pub(crate) fn create_rollup_atoms(filter: Option<LinkTag>) -> ExternResult<Vec<TrustAtom>> {
  Ok(vec![])
  // let mut rollup_silver = BTreeMap::new(); // K: AgentPubKey (EntryHash), V: BTreeMap<Content, RollupData>
  //                                                                    // suggest: top level key is the target hash; see also silver/gold notes
  // let mut agents: Vec<EntryHash> = Vec::new();

  // let my_trust_atoms = query_mine(filter.clone())?; //TODO: change query calls
  // let mut categories = Vec::new();
  // let my_ratings = BTreeMap::new(); // K: Content, V: Float Value

  // for ta in my_trust_atoms.clone() {
  //     let rollup_links = get_links(ta.target_entry_hash.clone(), "rollup".into())?;
  //         if rollup_links.len() > 0 {
  //             agents.push(ta.target_entry_hash)
  //         }
  //         if let Some(val) = ta.value {
  //             my_ratings.insert(ta.content, val.parse()?);  // maybe get latest rating, if conflict?
  //         }
  //     }

  // agents.iter().unique(); // get rid of duplicates  // TODO does this edit agents in place?

  // for agent in agents {
  //     let links = get_links(agent.clone(), filter)?;
  //     let trust_atoms = convert_links_to_trust_atoms(links, &LinkDirection::Forward, &agent)?;
  //     let rollup_collection = Vec::new();
  //     for ta in trust_atoms {
  //         if let Some(content) = ta.content.clone() {
  //             categories.push(content); // (optional) could be a map, then push in map[key] = true, then you have a list of unique keys
  //         }
  //         let link_latest = get_latest(agent.clone(), ta.content.clone())?;
  //         let trust_atom_latest = convert_link_to_trust_atom(link_latest, &LinkDirection::Forward, &agent)?;
  //         categories.iter().unique();

  //         let rating = {
  //             let i = 0;
  //             while i < categories.len() {
  //                 match trust_atom_latest.content {
  //                     categories[i] => { trust_atom_latest.value,
  //                                     break; },
  //                     _ => { i += 1; }
  //                 }
  //             }
  //         };
  //         let rollup_data = RollupData {
  //             target_hash: trust_atom_latest.target_entry_hash,
  //             value: trust_atom_latest.value.parse()?,
  //             rating: rating.parse()?
  //         };
  //         let map = BTreeMap::from(trust_atom_latest.content, rollup_data);
  //         rollup_silver.insert(agent, map);
  //     }
  // }
  // }

  // let gold_rollup: Vec<TrustAtom> = Vec::new();

  //     let amalgamation = BTreeMap::new(); // K: Content, V: Float Value

  // for rollup in category { // TODO: calc category

  //     let sum;
  //     let weight = rollup.rating;
  //     let algo = mine + (mine - rollup.value) * weight;
  //     sum += 1;

  //     ////
  //         let weigh = |val| String::from_utf8_lossy(weight * val);
  //         let map = Vec::new();

  //             let trust_atom_rolled = create(trust_atom_latest.target, "rollup".to_string(), trust_atom_latest.content, weigh(trust_atom_latest.value), None)?;
  //             rollup_gold.push(trust_atom);
  //     }

  // Ok(gold_rollup)
}

// fn get_latest(agent, content) -> ExternResult<Link> {
//     let links = get_links(agent, Some("rollup".as_bytes().extend_from_slice(content.as_bytes()))?;
//     let timestamps = Vec::new();
//     for link in links {
//         timestamps.push(link.timestamp);
//     }
//     timestamps.sort_by(|a,b| a.cmp(b)); // ignoring nanoseconds
//     let latest = timestamps.pop();
//     let latest_link = links.into_iter().filter(|x| latest).collect(); // should always be one
//     latest_link
// }

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
