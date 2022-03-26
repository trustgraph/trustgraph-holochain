use hdk::prelude::*;

use std::collections::BTreeMap;

use crate::trust_atom::LinkDirection;
use crate::trust_atom::*;

#[derive(Clone)]
struct RollupData {
  target_entry_hash: EntryHash,
  value: Option<String>,
  agent_rating: Option<String>,
}

pub fn create_rollup_atoms(filter: Option<LinkTag>) -> ExternResult<Vec<TrustAtom>> {
  let me: EntryHash = EntryHash::from(agent_info()?.agent_latest_pubkey);
  
  let mut rollup_silver = BTreeMap::new(); // K: Content (String), V: BTreeMap<Agent, RollupData>                                                   

  let my_trust_atoms: Vec<TrustAtom> = query_mine(filter.clone())?; //TODO: change query calls
  let mut categories = Vec::new(); // list of unique contents
  let mut agents = Vec::new();
  let mut my_ratings_by_content = BTreeMap::new(); // K: Content, V: Float Value

  for ta in my_trust_atoms.clone() {
      let target_entry_hash = EntryHash::from(ta.target_entry_hash);
      let rollup_links = get_links(target_entry_hash.clone(), Some(LinkTag::from("rollup")))?; // FIX: convert rollup str into LinkTag type
          if rollup_links.len() > 0 {
            for agent in agents { 
                match agent {               // prevent duplicates
                    target_entry_hash => break,
                    _ => { agents.push(ta.target_entry_hash); }
                }
            }
          }
        let link_latest = get_latest(me, ta.content.clone())?;
        let trust_atom_latest = convert_link_to_trust_atom(link_latest, &LinkDirection::Forward, &me)?;
          if let Some(val) = trust_atom_latest.value {
            my_ratings_by_content.insert(trust_atom_latest.content, val.parse()?); 
          }
    }

  for agent in agents {
      let agent = EntryHash::from(agent);
      let links = get_links(agent.clone(), filter)?; 
      let trust_atoms = convert_links_to_trust_atoms(links, &LinkDirection::Forward, &agent)?; 

      for ta in trust_atoms {
          if let Some(content) = ta.content.clone() {
              for category in categories {  
                  match category {      // prevent duplicates 
                      content => break,
                      _ => categories.push(ta.content)
                  }
          }
          let link_latest = get_latest(agent.clone(), ta.content.clone())?;
          let trust_atom_latest = convert_link_to_trust_atom(link_latest, &LinkDirection::Forward, &agent)?;

          let rating: Option<String> = {
              for category in categories {
                  match category {
                      trust_atom_latest.content => return trust_atom_latest.value,
                      _ => None
                  }
                }
            };
          let rollup_data = RollupData {
              target_entry_hash: EntryHash::from(trust_atom_latest.target_entry_hash),
              value: trust_atom_latest.value,
              agent_rating: rating
          };
          let map = BTreeMap::from([(agent, rollup_data)]);
          rollup_silver.insert(trust_atom_latest.content, map);
      }
    }
  }

  let gold_rollup: Vec<TrustAtom> = Vec::new();

  for category in rollup_silver.clone() {
      let sourced_trust_atoms = Vec::new(); // collect to input for rollup extra field
      let accumulator = Vec::new(); // gather weighted values
        for agent in category.clone() {
            let rollup_data = get(agent.clone());
            let rating_sum = rollup_data.clone().len();
                if let Some(rating) = rollup_data.rating { 
                        if let Some(val) = rollup_data.value {
                            let calc = rating / rating_sum * val.parse()?; // calculate weight
                            accumulator.push(calc);
                        }
                }
            let sourced_atom_latest = convert_link_to_trust_atom(get_latest(agent, category)?, &LinkDirection::Forward, &agent)?;    
            sourced_trust_atoms.push(sourced_atom_latest);
        }
        // self weight is 80%
        let my_rating = my_ratings_by_content.get(category.clone());
        let algo = my_rating * .80 + accumulator.iter().sum() * .20;
        let rollup_atom = create(rollup_data.target_entry_hash, "rollup".to_string(), category, algo, sourced_trust_atoms)?;
        gold_rollup.push(rollup_atom);
      }

  Ok(gold_rollup)
}

fn get_latest(agent, content) -> ExternResult<Link> {
    let links = get_links(agent, content)?;
    let timestamps = Vec::new();
    for link in links {
        timestamps.push(link.timestamp);
    }
    timestamps.sort_by(|a,b| a.cmp(b)); // ignoring nanoseconds
    let latest = timestamps.pop();
    let latest_link = links.into_iter().filter(|x| latest).collect(); // should always be one
    latest_link
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
