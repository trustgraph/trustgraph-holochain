use hdk::prelude::*;

pub fn test_create_links() -> ExternResult<Vec<EntryHash>> {
    let agent1: EntryHash = fake_agent_pub_key()?.into();
    let agent2: EntryHash = fake_agent_pub_key()?.into(); 
    create_link( agent1.clone(), fake_entry_hash()?, "HIA".to_string())?;
    create_link( agent2.clone(), fake_entry_hash()?, "HIA".to_string())?;
    create_link(fake_entry_hash()?, fake_entry_hash()?, "tag3".to_string())?;
    let test_agents = Vec::from(agent1, agent2);
    Ok(test_agents)
}