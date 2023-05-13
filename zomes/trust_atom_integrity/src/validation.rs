use hdi::prelude::*;
use crate::{EntryTypes, LinkTypes};




#[hdk_extern]
pub fn validate_create_trust_atom(op: Op) -> ExternResult<ValidateCallbackResult> {

	match op.flattened::<EntryTypes, LinkTypes>()? {

		FlatOp::RegisterCreateLink {
			            link_type,
			            base_address,
			            target_address,
			            tag,
			            action,
			        } => {
			            match link_type {
			                LinkTypes::TrustAtom => {
			                    
			                }
			               
			                
			            }
			        }
			                
			            }
			        }
		_ => {}
		

#[hdk_extern]
pub fn validate_delete_trust_atom(op: Op) -> ExternResult<ValidateCallbackResult> {
	let agent = agent_info().agent_initial_pubkey?;
	match op.flattened::<EntryTypes, LinkTypes>()? {
		
			FlatOp::RegisterDeleteLink {
				link_type,
				base_address,
				target_address,
				tag,
				original_action,
				action,
			} => {
				match link_type {
					LinkTypes::TrustAtom => {
						
					}
					
				}
			}
		_ => {}
	}


}

