use hdk::prelude::*;

use crate::utils::*;

#[hdk_entry(id = "agent_registry", visibility = "private")]
#[derive(Clone)]
pub enum AgentRegistry {
    rated: Vec<Agent>, // any positive TA value (rating)
    no_rating: Vec<Agent>,
    spam: Vec<Agent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pubkey: AgentInfo,
    handle: String,
    //trusted: bool,
    stats: Stats,
    // data: 
}

impl Agent {
    fn new(pubkey: AgentInfo, name: String) -> Self {
        Self {
            pubkey,
            handle: name,
            stats: Default::default(),
            //data: Default::default(),
        }
    }
    // fn update(&self, agent: Agent,  ) -> Self {
    //   Self {
    //     pubkey: 
    //     handle:
    //     stats:
    //     //data:
    //   }
    // }
}

#[derive(Clone, Debug, Default)]
pub struct Stats {
    links: Vec<Link>,
    score: f32, // avg peer given value, or could be matrix, or struct with categories
    flags: u32, //warrants
    // metadata:
}

pub fn get_agent_registry(entry_hash: &EntryHash) -> ExternResult<AgentRegistry> {
  let element = try_get_element(entry_hash, GetOptions::default())?;
  match element.entry() {
    element::ElementEntry::Present(entry) => {
      AgentRegistry::try_from(entry.clone()).or(Err(WasmError::Guest(format!(
        "Couldn't convert Element entry {:?} into data label {}",
        entry,
        std::any::type_name::<AgentRegistry>()
      ))))
    }
    _ => Err(WasmError::Guest(format!(
      "Element {:?} does not have an entry",
      element
    ))),
  }
}
