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

#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
  Example(Example),
  StringTarget(StringTarget),
  Extra(Extra),
}
