use hdk::prelude::*;

pub fn try_get_element(entry_hash: &EntryHash, get_options: GetOptions) -> ExternResult<Element> {
  match get(entry_hash.clone(), get_options)? {
    Some(element) => Ok(element),
    None => Err(WasmError::Guest(format!(
      "There is no element at the hash {}",
      entry_hash
    ))),
  }
}
