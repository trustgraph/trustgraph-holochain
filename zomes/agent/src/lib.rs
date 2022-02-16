use hdk::prelude::*;



mod init;
mod agent;
pub use init::*;
pub use agent::*;


entry_defs![Myself::entry_defs(), AgentInfo::entry_defs()]

pub enum DataType {

}

pub enum Payload {
    Input(IOType),
    Output(IOType),
}

pub enum IOType {
    User,
    Agent,
    Zome,
    //Error
}