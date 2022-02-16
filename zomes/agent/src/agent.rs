use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

pub struct Stats {
    followers: u32,
    remewed: u32,
    flags: u32, //how many users dont trust an agent
    
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
    Trusted(bool), //allows for blocking
    Followed(bool),
    Activity(Stats),

}
