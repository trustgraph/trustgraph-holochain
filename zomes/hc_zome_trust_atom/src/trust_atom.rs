#![allow(clippy::module_name_repetitions)]

use hdk::prelude::holo_hash::EntryHashB64;
use hdk::prelude::*;
use rust_decimal::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
enum LinkDirection {
  Forward,
  Reverse,
}

/// Client-facing representation of a Trust Atom
/// We may support JSON in the future to allow for more complex data structures @TODO
#[derive(Serialize, Deserialize, SerializedBytes, Debug, Clone, PartialEq)]
pub struct TrustAtom {
  pub source: String, // TODO source_name
  pub target: String,
  pub source_entry_hash: EntryHashB64,
  pub target_entry_hash: EntryHashB64,
  pub content: Option<String>,
  pub value: Option<String>,
  pub extra: Option<BTreeMap<String, String>>,
}

const UNICODE_NUL_STR: &str = "\u{0}"; // Unicode NUL character
const LINK_TAG_HEADER: [u8; 2] = [197, 166]; // Unicode "Ŧ" // hex bytes: [0xC5][0xA6]
const LINK_TAG_ARROW_FORWARD: [u8; 3] = [226, 134, 146]; // Unicode "→" // hex bytes: [0xE2][0x86][0x92]
const LINK_TAG_ARROW_REVERSE: [u8; 3] = [226, 134, 169]; // Unicode "↩" // hex bytes: [0xE2][0x86][0xA9]

#[hdk_entry(id = "extra", visibility = "public")]
#[derive(Clone)]
pub struct Extra {
  pub content_overflow: Option<String>,
  pub extra_fields: Option<BTreeMap<String, String>>, // extra content
}

pub fn create(
  target: EntryHash,
  content: Option<String>,
  value: Option<String>,
  extra: Option<BTreeMap<String, String>>,
) -> ExternResult<TrustAtom> {
  let agent_address: EntryHash = agent_info()?.agent_initial_pubkey.into();

  let bucket = create_bucket()?;

  let overflow: bool = match content.clone() {
    Some(c) => if c.len() > 900 {true} else {false}
    None => false
  };

  let extra_entry_hash_string = match overflow {
    true => match extra.clone() {
      Some(x) => Some(create_extra(content, Some(x))?),
      None => Some(create_extra(content, None)?)
  }
    false => match extra.clone() {
      Some(x) => Some(create_extra(None, Some(x))?),
      None => None
}
  };

  let chunks = vec![
    content.clone(),
    normalize_value(value.clone())?,
    Some(bucket),
    extra_entry_hash_string,
  ];

  let forward_link_tag = create_link_tag(&LinkDirection::Forward, chunks);
  let reverse_link_tag = create_link_tag(&LinkDirection::Reverse, chunks);

  create_link(agent_address.clone(), target.clone(), forward_link_tag)?;
  create_link(target.clone(), agent_address.clone(), reverse_link_tag)?;

  let agent_address_entry: EntryHash = agent_address;

  let trust_atom = TrustAtom {
    source: agent_address_entry.to_string(),
    target: target.to_string(),
    source_entry_hash: agent_address_entry.into(),
    target_entry_hash: target.into(),
    content,
    value,
    extra,
  };
  Ok(trust_atom)
}

fn create_bucket() -> ExternResult<String> {
  let bucket_bytes = random_bytes(9)?.into_vec();
  Ok(create_bucket_string(bucket_bytes))
}

fn create_bucket_string(bucket_bytes: Vec<u8>) -> String {
  let mut bucket = String::new();
  for chunk in bucket_bytes {
    let val = chunk;
    bucket += (val % 10).to_string().as_str();
  }
  bucket
}

fn create_extra(content_overflow: Option<String>, extra_fields: Option<BTreeMap<String, String>>) -> ExternResult<String> {
  let entry = Extra { content_overflow, extra_fields };

  create_entry(entry.clone())?;

  let entry_hash_string = calc_extra_hash(entry)?.to_string();
  Ok(entry_hash_string)
}

pub fn calc_extra_hash(input: Extra) -> ExternResult<EntryHash> {
  let hash = hash_entry(input)?;
  Ok(hash)
}

fn normalize_value(value_str: Option<String>) -> ExternResult<Option<String>> {
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
              Err(WasmError::Guest(format!(
                "Value must be in the range -1..1, but got: `{}`",
                value_str
              )))
            }
          }
          None => Err(WasmError::Guest(format!(
            "Value could not be processed: `{}`",
            value_str
          ))),
        }
      }
      Err(error) => Err(WasmError::Guest(format!(
        "Value could not be processed: `{}`.  Error: `{}`",
        value_str, error
      ))),
    },
    None => Ok(None),
  }
}

fn create_link_tag(link_direction: &LinkDirection, chunk_options: Vec<Option<String>>) -> LinkTag {
  let mut chunks: Vec<String> = vec![];
  if let Some(content) = chunk_options[0] {
    if content.len() > 900 {
      let mut max_content = content.clone();
      max_content.truncate(898);  // leave 2 bytes for `…`
      max_content.push_str("…");
      chunks.push(max_content);
    }
    else {chunks.push(content);}
}

  for i in 1..chunk_options.len() {
    if let Some(chunk) = chunk_options[i].clone() {
      chunks.push(chunk);
      if i < chunk_options.len() - 1 {
        chunks.push(UNICODE_NUL_STR.to_string());
      }
    }
  }

  create_link_tag_metal(link_direction, chunks)
}

fn create_link_tag_metal(link_direction: &LinkDirection, chunks: Vec<String>) -> LinkTag {
  let link_tag_arrow = match link_direction {
    LinkDirection::Forward => LINK_TAG_ARROW_FORWARD,
    LinkDirection::Reverse => LINK_TAG_ARROW_REVERSE,
  };

  let mut link_tag_bytes = vec![];
  link_tag_bytes.extend_from_slice(&LINK_TAG_HEADER);
  link_tag_bytes.extend_from_slice(&link_tag_arrow);

  for chunk in chunks {
    link_tag_bytes.extend_from_slice(chunk.as_bytes());
  }

  // debug!("link_tag: {:?}", String::from_utf8_lossy(&link_tag_bytes));
  LinkTag(link_tag_bytes)
}

pub fn get_extra(entry_hash: &EntryHash) -> ExternResult<Extra> {
  let element = get_element(entry_hash, GetOptions::default())?;
  match element.entry() {
    element::ElementEntry::Present(entry) => {
      Extra::try_from(entry.clone()).or(Err(WasmError::Guest(format!(
        "Couldn't convert Element entry {:?} into data type {}",
        entry,
        std::any::type_name::<Extra>()
      ))))
    }
    _ => Err(WasmError::Guest(format!(
      "Element {:?} does not have an entry",
      element
    ))),
  }
}

fn get_element(entry_hash: &EntryHash, get_options: GetOptions) -> ExternResult<Element> {
  match get(entry_hash.clone(), get_options)? {
    Some(element) => Ok(element),
    None => Err(WasmError::Guest(format!(
      "There is no element at the hash {}",
      entry_hash
    ))),
  }
}

pub fn query_mine(
  target: Option<EntryHash>,
  content_full: Option<String>,
  content_starts_with: Option<String>,
  value_starts_with: Option<String>,
) -> ExternResult<Vec<TrustAtom>> {
  let agent_address: EntryHash = agent_info()?.agent_initial_pubkey.into();

  let result = query(
    Some(agent_address),
    target,
    content_full,
    content_starts_with,
    value_starts_with,
  )?;

  Ok(result)
}

/// Required: exactly one of source or target
/// All other arguments are optional
/// Arguments act as additive filters (AND)
#[warn(clippy::needless_pass_by_value)]
pub fn query(
  source: Option<EntryHash>,
  target: Option<EntryHash>,
  content_full: Option<String>,
  content_starts_with: Option<String>,
  value_starts_with: Option<String>,
) -> ExternResult<Vec<TrustAtom>> {
  let (link_direction, link_base) = match (source, target) {
    (Some(source), None) => (LinkDirection::Forward, source),
    (None, Some(target)) => (LinkDirection::Reverse, target),
    (None, None) => {
      return Err(WasmError::Guest(
        "Either source or target must be specified".into(),
      ))
    }
    (Some(_source), Some(_target)) => {
      return Err(WasmError::Guest(
        "Exactly one of source or target must be specified, but not both".into(),
      ))
    }
  };

  let link_tag = match (content_full, content_starts_with, value_starts_with) {
    (Some(_content_full), Some(_content_starts_with), _) => {
      return Err(WasmError::Guest("Only one of `content_full` or `content_starts_with` can be used".into()))
    }
    (_, Some(_content_starts_with), Some(_value_starts_with)) => {
      return Err(WasmError::Guest(
        "Cannot use `value_starts_with` and `content_starts_with` arguments together; maybe try `content_full` instead?".into(),
      ))
    }
    (Some(content_full), None, Some(value_starts_with)) => Some(create_link_tag(
      &link_direction,
      Some(content_full), Some(value_starts_with),
    )),
    (Some(content_full), None, None) => {
      Some(create_link_tag_metal(&link_direction, vec![content_full, UNICODE_NUL_STR.to_string()]))
    }
    (None, Some(content_starts_with), None) => Some(create_link_tag(
      &link_direction,
      Some(content_starts_with),
    )),
    (None, None, Some(value_starts_with)) => Some(create_link_tag(&link_direction, Some(value_starts_with))),
    (None, None, None) => None,
  };
  let links = get_links(link_base.clone(), link_tag)?;

  let trust_atoms = convert_links_to_trust_atoms(links, &link_direction, &link_base)?;

  Ok(trust_atoms)
}

fn convert_links_to_trust_atoms(
  links: Vec<Link>,
  link_direction: &LinkDirection,
  link_base: &EntryHash,
) -> ExternResult<Vec<TrustAtom>> {
  let trust_atoms_result: Result<Vec<TrustAtom>, _> = links
    .into_iter()
    .map(|link| convert_link_to_trust_atom(link, link_direction, link_base))
    .collect();
  let trust_atoms = trust_atoms_result?;
  Ok(trust_atoms)
  // .ok_or_else(|_| WasmError::Guest("Failure in converting links to trust atoms".to_string()))?;
  //   Ok(trust_atoms.or_else(|_| WasmError::Guest("erro"))?)
}

fn convert_link_to_trust_atom(
  link: Link,
  link_direction: &LinkDirection,
  link_base: &EntryHash,
) -> ExternResult<TrustAtom> {
  let link_tag_bytes = link.tag.clone().into_inner();
  let link_tag = match String::from_utf8(link_tag_bytes) {
    Ok(link_tag) => link_tag,
    Err(_) => {
      return Err(WasmError::Guest(format!(
        "Link tag is not valid UTF-8 -- found: {:?}",
        String::from_utf8_lossy(&link.tag.into_inner())
      )))
    }
  };

  let chunks: Vec<&str> = link_tag.split(UNICODE_NUL_STR).collect();
  let content = chunks[0][tg_link_tag_header_length()..].to_string(); // drop leading "Ŧ→" or "Ŧ↩"
  let value = chunks[1].to_string();

  let link_base_b64 = EntryHashB64::from(link_base.clone());
  let link_target_b64 = EntryHashB64::from(link.target);

  let trust_atom = match link_direction {
    LinkDirection::Forward => {
      TrustAtom {
        source: link_base_b64.to_string(),
        target: link_target_b64.to_string(),
        source_entry_hash: link_base_b64,
        target_entry_hash: link_target_b64,
        content: Some(content),
        value: Some(value),
        extra: Some(BTreeMap::new()), // TODO
      }
    }
    LinkDirection::Reverse => {
      TrustAtom {
        source: link_target_b64.to_string(), // flipped for Reverse direction
        target: link_base_b64.to_string(),   // flipped for Reverse direction
        source_entry_hash: link_target_b64,  // flipped for Reverse direction
        target_entry_hash: link_base_b64,    // flipped for Reverse direction
        content: Some(content),
        value: Some(value),
        extra: Some(BTreeMap::new()), // TODO
      }
    }
  };
  Ok(trust_atom)
}

const fn tg_link_tag_header_length() -> usize {
  LINK_TAG_HEADER.len() + LINK_TAG_ARROW_FORWARD.len()
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
#[allow(non_snake_case)]
mod tests {

  use super::*; // allows testing of private functions

  #[test]
  fn test_normalize_value__valid_value() {
    let valid_values = [
      "1.0",
      "1.000000000000000000000000000",
      "1",
      "0.534857395723489529357489283",
      "0.0",
      "0.000000000000000000000000000",
      "0",
      "-1.0",
      "-1",
      "-1.00000000000000000000000000",
    ];

    for value in valid_values {
      normalize_value(Some(value.to_string())).unwrap();
    }
  }

  #[test]
  fn test_normalize_value__values_out_of_range() {
    let out_of_range_values = [
      "100000000000000000",
      "-100000000000000000",
      "2",
      "1.000000005",
      "1.00000001",
      "-1.00000001",
      "-1.000000005",
      "-2",
    ];

    for value in out_of_range_values {
      let expected_error_message = "Value must be in the range -1..1";
      let actual_error_message = normalize_value(Some(value.to_string()))
        .expect_err(&format!("expected error for value `{}`, got", value))
        .to_string();
      assert!(
        actual_error_message.contains(expected_error_message),
        "Expected error message: `...{}...`, but got: `{}`",
        expected_error_message,
        actual_error_message
      );
    }
  }

  #[test]
  fn test_normalize_value__values_not_numeric() {
    #[rustfmt::skip]
    let non_numeric_values = [
      " ",
      " 0 ",
      " 0",
      "-.",
      "-",
      "-100000000000000000000000000000.0",
      "-1e",
      "-1e0",
      "-e0",
      "!",
      ".",
      "",
      "",
      "\u{1f9d0}",
      "0 ",
      "100000000000000000000000000000.0",
      "1e",
      "1e0",
      "e",
      "e0",
      "foo",
     ];

    for value in non_numeric_values {
      let expected_error_message = "Value could not be processed";
      let actual_error_message = normalize_value(Some(value.to_string()))
        .expect_err(&format!("expected error for value `{}`, got", value))
        .to_string();
      assert!(
        actual_error_message.contains(expected_error_message),
        "Expected error message: `...{}...`, but got: `{}`",
        expected_error_message,
        actual_error_message
      );
    }
  }

  #[test]
  fn test_normalize_value() {
    let input_and_expected = [
      ["-.9", "-.900000000"],
      ["-.9000", "-.900000000"],
      ["-.900000000", "-.900000000"],
      ["-.9000000004", "-.900000000"],
      ["-.9000000005", "-.900000001"],
      ["-0.900000000", "-.900000000"],
      ["0.8999999995", ".900000000"],
      ["0.7999999995", ".800000000"],
      ["-0.8999999995", "-.900000000"],
      ["-0.7999999995", "-.800000000"],
      ["0.8999999994", ".899999999"],
      ["0.7999999994", ".799999999"],
      ["-0.8999999994", "-.899999999"],
      ["-0.7999999994", "-.799999999"],
      [".9", ".900000000"],
      [".9000", ".900000000"],
      [".900000000", ".900000000"],
      ["0.900000000", ".900000000"],
      //
      ["1", ".999999999"],
      ["1.0", ".999999999"],
      ["-1", "-.999999999"],
      ["-1.0", "-.999999999"],
    ];

    for [input, expected] in input_and_expected {
      let normalized_value = normalize_value(Some(input.to_string())).unwrap().unwrap();
      assert_eq!(normalized_value, expected.to_string());
    }
  }

  #[test]
  fn test_bucket_val() {
    let bytes: [u8; 9] = [9, 10, 11, 12, 13, 14, 15, 16, 17];
    let bucket = create_bucket_string(&bytes);
    assert_eq!(bucket, "901234567".to_string());
  }
}
