use hdi::prelude::*;

#[hdk_entry_defs]
#[hdk_entry_helper]
#[derive(Clone)]
pub struct StringTarget(String);

#[hdk_entry_defs]
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Test {
  pub example_field: String,
  //another_test_field:
}

#[hdk_entry_defs]
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Extra {
  pub fields: BTreeMap<String, String>, // extra content
}

#[hdk_link_types]
pub enum LinkTypes {
  TrustAtom 
}