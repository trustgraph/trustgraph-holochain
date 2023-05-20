use crate::{EntryTypes, LinkTypes};
use hdi::prelude::*;
use hdk::prelude::agent_info;

const UNICODE_NUL_STR: &str = "\u{0}";
const UNICODE_T_CHAR: &str = "\u{0}";

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  match op.to_type::<EntryTypes, LinkTypes>()? {
    OpType::RegisterCreateLink {
      base_address,
      target_address,
      tag,
      link_type,
      action,
		} => {
            if base_address != agent_info()?.agent_initial_pubkey.into() { 
                    return Ok(ValidateCallbackResult::Invalid(
                		"Creator must be same as base_address".to_string()))
                }
            if base_address != AnyLinkableHash::from(action.author) { 
                return Ok(ValidateCallbackResult::Invalid(
					      "Creator must be same as base_address".to_string()))
            }
            
			let link_tag = String::from_utf8(tag.into_inner());

      match &link_tag {
        Ok(tag) => {
            match tag.matches(UNICODE_NUL_STR).count() {
          4 => { let chunks: Vec<&str> = tag.split(UNICODE_NUL_STR).collect(); 
                // debug!("{:?}", chunks);
                // if chunks[0].as_bytes() != [0xC5][0xA6][0x21][0x92] || [0xC5][0xA6][0x21][0xA9] {
                //   return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - prefix symbol incorrect".to_string())))
                // }
                if chunks[1].as_bytes().len() > 900 {
                  if chunks[4] == "" {
                    return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - content > 900 bytes must have extra entry hash".to_string())))
                  }  
                }
                if chunks[2].chars().count() > 12 {
                  return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - rating must be <= 12 chars".to_string())))
                }
                if chunks[3].chars().count() != 9 {
                  return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - must have 9 char bucket".to_string())))
                }
                else {
                  return Ok(ValidateCallbackResult::Valid)
                }
              },
          _ => return Ok(ValidateCallbackResult::Invalid(String::from(
              "Invalid link tag! Expected 4 null bytes.",
            ))),
            };
        },
        Err(_) => return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error".to_string()))),
          
      }



    },
	OpType::RegisterDeleteLink {
		link_type,
		base_address,
		target_address,
		tag,
		original_action,
		action,
		} => {
			if action.author != original_action.author {
				return Ok(ValidateCallbackResult::Invalid(
					"Only the original author can delete Trust Atom".to_string(),
				));
			}
			Ok(ValidateCallbackResult::Valid)
		},
	_ => { Ok(ValidateCallbackResult::Valid) } // TODO: Exhuast remaining match options
	}
}