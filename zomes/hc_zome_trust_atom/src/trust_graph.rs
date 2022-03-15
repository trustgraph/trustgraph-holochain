use hdk::prelude::*;

use crate::trust_atom::*;
use crate::trust_atom::LinkDirection;
use crate::agents::*;
use crate::agents::*AgentRegistry;

pub fn create_trust_graph_plus_rollup(register: impl Fn(EntryHash) -> AgentRegistry, filter: Option<LinkTag>) -> ExternResult<Vec<TrustAtom>> {
    let base_address: EntryHash = agent_info()?.agent_latest_pubkey.into();
    let mut trust_graph: Vec<TrustAtom> = create_trust_graph_mine(filter.clone())?;
    let mut rollup: Vec<TrustAtom> = create_rollup_atoms(register, filter)?;
    trust_graph.append(&mut rollup);
    Ok(trust_graph)
}

pub fn create_trust_graph_mine(filter: Option<LinkTag>) -> ExternResult<Vec<TrustAtom>> {
    let base_address: EntryHash = agent_info()?.agent_latest_pubkey.into();
    let links: Vec<Link> = get_links(base_address.clone(), filter.clone())?;
    let trust_atoms: Vec<TrustAtom> = convert_links_to_trust_atoms(links, &LinkDirection::Forward, &base_address)?;
    Ok(trust_atoms)
}

fn create_rollup_atoms(registered_agents: EntryHash, filter: Option<LinkTag>) -> ExternResult<Vec<TrustAtom>> {
    let mut trust_atoms_collection: Vec<TrustAtom> = Vec::new();
    let agent_list = get_agent_registry(&registered_agents)?.rated.iter().map(|pk| pk.pubkey.EntryHash::from(pk.pubkey));
    for agent in agent_list.clone() {
        let algo;
        let agent_links = get_links(agent.pubkey.clone(), filter)?;
        let mut agent_trust_atoms = convert_links_to_trust_atoms(agent_links, &LinkDirection::Forward, &agent.pubkey)?;
        //FINISH: combine TAs with same target and calculate weight from agent.stats.score
        let sum;
        for ta in agent_trust_atoms {
            if let Some(val) = ta.value {
                let float_val = val.parse()?;
                if float_val > 0 {
                sum += weight * float_val^1.618; // raised to PHI smooths out weights curve
                }
            }
        }
    trust_atoms_collection.append(&mut algo );
    }
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
        // source: me
        // type: rollup
        // target: HIA Entry hash:
        // value: float
        // content: holochain
    // ]

// }
