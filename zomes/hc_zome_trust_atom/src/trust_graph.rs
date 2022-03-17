use hdk::prelude::*;

use std::collections::BTreeMap;

use crate::trust_atom::*;
use crate::trust_atom::LinkDirection;



fn create_rollup_atoms(filter: Option<LinkTag>) -> ExternResult<Vec<TrustAtom>> {
    let mut rollup_silver = BTreeMap::new(); //store TA's where key is AgentPubKey (as EntryHash)
    let mut agents = Vec::new();

    let my_trust_atoms = query_mine(filter)?; //TODO: change query calls
    let mut categories = Vec::new();

    for ta in my_trust_atoms.clone() {
        let rollup_links = get_links(ta.target.clone(), Some("rollup".as_bytes()))?;
            let converted = convert_links_to_trust_atoms(rollup_links.clone(), &LinkDirection::Forward, &ta.target)?;
            if rollup_links.len() > 0 {
                rollup_silver.insert(ta.target.clone(), converted);
                categories.push(ta.content);
                agents.push(ta.target)
            }
    }
    categories.unique_by(|name| name); //get rid of duplicates
    agents.unique_by(|name| name);
    
    let gold_rollup = Vec::new();

    for agent in rollup_silver.clone() {

        let vec_ta = rollup_silver.get(agent.clone()).unique_by(|atom| atom);
        if let Some(vec) = vec_ta {

            let weigh = |val| String::from_utf8_lossy(weight * val); // TODO: algo for weights
            let map = Vec::new();

            for ta in vec {
                let link_latest = get_latest(agent, ta.content);
                let trust_atom_latest = convert_links_to_trust_atoms(content_latest, LinkDirection::Forward, agent)?;
                }
                let trust_atom = create(trust_atom_latest.target, "rollup".to_string(), trust_atom_latest.content, weigh(trust_atom_latest.value), None)?;
                rollup_gold.push(trust_atom);
        }
    }
    Ok(gold_rollup)
} 

fn get_latest(agent, content) -> ExternResult<Link> { 
    let links = get_links(agent, Some("rollup".as_bytes().extend_from_slice(content.as_bytes()))?;
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

    // rollup_silver: map =
    //     key = target  // of my TAs or the rollups of agents in my TG
    //     value = vec of TA data + my rating of agent

    // rollup_data: map = {
        // HIA Entry hash: [
            // {
                // source: zippy
                // value: float
                // content: holochain
                // agent_rating: float // my rating of zippy on `holochain`
            // }
        // ]
    //

    // gold:
    // rollup_gold: vec<TrustAtom>  = [
        // {
        //   source: me
        //   type: rollup
        //   target: HIA Entry hash:
        //   value: float
        //   content: holochain
        // }
    // ]

    // for item in rollup_gold:
    //      create_link for each

// }


// ALTERNATE STRATEGY (no agent registry, no ablity to identify whether entry is agentpubkey)

// for TA in my TAs
//   rollup_links = get_links(source: TA.target, type: "rollup")  // returns rollups from agents who have done rollups, but [] from non-agent entries
//
//
