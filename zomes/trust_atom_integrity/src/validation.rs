use crate::{EntryTypes, LinkTypes};
use hdi::prelude::*;

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  match op.to_type::<EntryTypes, LinkTypes>()? {
    OpType::RegisterCreateLink {
      base_address,
      target_address,
      tag,
      link_type,
      action,
		} => match link_type {
			match null_byte_count {
				4 => // Ok(ValidateCallbackResult::Valid)
			 	null_bytes =>
						Ok(ValidateCallbackResult::Invalid(String::from(
							"Invalid link tag!  Expected 4 null bytes, but got {}", null_bytes,
						)))

		},
			// from https://github.com/holochain-immersive/private-publication.git
			// LinkTypes::PathToPost => {
			// 		validate_create_link_all_posts(action, base_address, target_address, tag)
			// }
    }
    _ => Ok(ValidateCallbackResult::Valid),
  }
}

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
    } => match link_type {
      LinkTypes::TrustAtom => {}
    },
    _ => {}
  }
}
