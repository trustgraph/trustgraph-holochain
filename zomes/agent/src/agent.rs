use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

pub struct Stats {
    links: Links,
    score: f32, // avg peer given value
    flags: u32,
    
}
#[derive(Debug, Clone)]
#[hdk_entry(id = "self", visibility = "private")]
pub struct Myself {
    pub_key: AgentPubKeyB64,
    handle: String,
    // data: 
}

#[derive(Debug, Clone, Deserialize, Serialize, SerializedBtyes)]
#[hdk_entry(id = "agent_info", visibility = "private")]
pub enum AgentInfo {
    Trusted(bool),
    Activity(Stats),

}
