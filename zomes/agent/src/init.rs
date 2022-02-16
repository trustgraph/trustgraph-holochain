use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

use crate::follow::*;
use super::agent::*;

/// Initializes self as the "first" (technically 4th) entry in source chain
[hdk_extern]
pub fn init_account(agent: AgentPubKeyB64, nickname: String) -> ExternResult<Myself> {
    let my_pubkey = agent.info()?.agent_latest_pubkey;
    let me = Myself {
        pubkey: my_pubkey,
        handle: nickname,
        //data: Data::new(),
    }
    create_entry(&me)?;
    OK(me)
}