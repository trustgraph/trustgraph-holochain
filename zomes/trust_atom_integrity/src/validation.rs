use crate::{EntryTypes, LinkTypes};
use hdi::prelude::*;
use hdk::prelude::agent_info;
use rust_decimal::prelude::*;
use std::str;

pub const UNICODE_NUL_STR: &str = "\u{0}"; // Unicode NUL character
pub const LINK_TAG_HEADER: [u8; 2] = [197, 166]; // Unicode "Ŧ" // hex bytes: [0xC5][0xA6]
pub const LINK_TAG_ARROW_FORWARD: [u8; 3] = [226, 134, 146]; // Unicode "→" // hex bytes: [0xE2][0x86][0x92]
pub const LINK_TAG_ARROW_REVERSE: [u8; 3] = [226, 134, 169]; // Unicode "↩" // hex bytes: [0xE2][0x86][0xA9]

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
  match op.flattened::<(), LinkTypes>()? {
    FlatOp::RegisterCreateLink {
      base_address,
      target_address,
      tag,
      link_type,
      action,
      // TODO: how to handle forward vs reverse base
    } => {
      if base_address != agent_info()?.agent_initial_pubkey.into() {
        return Ok(ValidateCallbackResult::Invalid(
          "Creator must be same as base_address".to_string(),
        ));
      }
      if base_address != AnyLinkableHash::from(action.author) {
        return Ok(ValidateCallbackResult::Invalid(
          "Creator must be same as base_address".to_string(),
        ));
      }

      let link_tag = match String::from_utf8(tag.clone().into_inner()) {
        Ok(link_tag) => link_tag,
        Err(_) => {
          return Err(wasm_error!(
            "Link tag is not valid UTF-8 -- found: {:?}",
            String::from_utf8_lossy(&tag.into_inner())
          ))
        }
      };

      let chunks: Vec<&str> = link_tag.split(UNICODE_NUL_STR).collect();

      if chunks.len() != 4 {
        return Ok(ValidateCallbackResult::Invalid(format!(
          "Invalid link tag! Expected 4 NUL bytes, got {} NUL bytes",
          chunks.len().to_string(),
        )));
      }

      // debug!("{:?}", chunks);
      if chunks[0].as_bytes() != build_forward_header()
        && chunks[0].as_bytes() != build_reverse_header()
      {
        return Err(wasm_error!(
          "LinkTag format error - header bytes incorrect, must be either {:?}, or {:?}, but got {}",
          convert_bytes_to_string(&LINK_TAG_ARROW_FORWARD),
          convert_bytes_to_string(&LINK_TAG_ARROW_REVERSE),
          chunks[0]
        ));
      }
      // TODO: Let's make sure there is a test with 900 bytes and 901 bytes
      // if chunks[1].as_bytes().len() > 900 && chunks[4] == "" {
      //   return Err(wasm_error!(
      //     "LinkTag format error - content > 900 bytes must have extra entry hash"
      //   ));
      // }
      // TODO: pass chunk through normalize_value()
      if chunks[2].chars().count() > 12 {
        return Err(wasm_error!(
          "LinkTag format error - rating must be <= 12 chars"
        ));
      }
      // TODO: check that chars are numbers only
      if chunks[3].chars().count() != 9 {
        return Err(wasm_error!(
          "LinkTag format error - must have 9 char bucket"
        ));
      } else {
        return Ok(ValidateCallbackResult::Valid);
      }
    }

    // match link_tag.matches(UNICODE_NUL_STR).count() {
    //   4 => { let chunks: Vec<&str> = link_tag.split(UNICODE_NUL_STR).collect();
    //         // debug!("{:?}", chunks);
    //         if chunks[0].as_bytes() != build_forward_header() && chunks[0].as_bytes() != build_reverse_header() {
    //           return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - prefix symbol incorrect".to_string())))
    //         }
    //         if chunks[1].as_bytes().len() > 900 {
    //           if chunks[4] == "" {
    //             return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - content > 900 bytes must have extra entry hash".to_string())))
    //           }
    //         }
    //         if chunks[2].chars().count() > 12 {
    //           return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - rating must be <= 12 chars".to_string())))
    //         }
    //         if chunks[3].chars().count() != 9 {
    //           return Err(wasm_error!(WasmErrorInner::Guest("LinkTag format error - must have 9 char bucket".to_string())))
    //         }
    //         else {
    //           return Ok(ValidateCallbackResult::Valid)
    //         }
    //       },
    //   _ => return Ok(ValidateCallbackResult::Invalid(String::from(
    //       "Invalid link tag! Expected 4 null bytes.",
    //     ))),
    //     };
    FlatOp::RegisterDeleteLink {
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
      if base_address != AnyLinkableHash::from(agent_info()?.agent_initial_pubkey) {
        return Ok(ValidateCallbackResult::Invalid(
          "Base address of Trust Atom must be original author".to_string(),
        ));
      }

      Ok(ValidateCallbackResult::Valid)
    }
    _ => Ok(ValidateCallbackResult::Valid), // TODO: Exhuast remaining match options
  }
}

fn build_forward_header() -> Vec<u8> {
  let mut forward_bytes = vec![];
  forward_bytes.extend_from_slice(&LINK_TAG_HEADER);
  forward_bytes.extend_from_slice(&LINK_TAG_ARROW_FORWARD);

  forward_bytes
}

fn build_reverse_header() -> Vec<u8> {
  let mut reverse_bytes = vec![];
  reverse_bytes.extend_from_slice(&LINK_TAG_HEADER);
  reverse_bytes.extend_from_slice(&LINK_TAG_ARROW_REVERSE);

  reverse_bytes
}

fn convert_bytes_to_string(byte_array: &[u8]) -> ExternResult<String> {
  let conversion = String::from_utf8(byte_array.to_vec());

  match conversion {
    Ok(string_slice) => Ok(string_slice),
    Err(e) => Err(wasm_error!(
      "Failed to convert byte slice to string slice: {:?}",
      e
    )),
  }
}

pub fn normalize_value(value_str: Option<String>) -> ExternResult<Option<String>> {
  match value_str {
    Some(value_str) => match Decimal::from_str(value_str.as_str()) {
      Ok(value_decimal) => {
        match value_decimal.round_sf_with_strategy(9, RoundingStrategy::MidpointAwayFromZero) {
          Some(value_decimal) => {
            if value_decimal == Decimal::ONE {
              Ok(Some(".999999999".to_string()))
            } else if value_decimal == Decimal::NEGATIVE_ONE {
              Ok(Some("-.999999999".to_string()))
            } else if value_decimal > Decimal::NEGATIVE_ONE && value_decimal < Decimal::ONE {
              let value_zero_stripped = value_decimal.to_string().replace("0.", ".");
              Ok(Some(value_zero_stripped))
            } else {
              Err(wasm_error!(
                "Value must be in the range -1..1, but got: `{}`",
                value_str
              ))
            }
          }
          None => Err(wasm_error!("Value could not be processed: `{}`", value_str)),
        }
      }
      Err(error) => Err(wasm_error!(
        "Value could not be processed: `{}`.  Error: `{}`",
        value_str,
        error
      )),
    },
    None => Ok(None),
  }
}
