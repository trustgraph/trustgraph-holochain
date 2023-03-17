use hdi::prelude::*;
use std::collections::BTreeMap;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct StringTarget(pub String);

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Example {
  pub example_field: String,
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct Extra {
  pub fields: BTreeMap<String, String>,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
  #[entry_def]
  Example(Example),
  #[entry_def]
  StringTarget(StringTarget),
  #[entry_def]
  Extra(Extra),
}

#[hdk_link_types]
pub enum LinkTypes {
  TrustAtom,
}
