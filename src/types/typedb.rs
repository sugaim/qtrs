use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use super::{
    error::InvalidValidationError,
    typedef::{EnumDef, NewtypeDef, StructDef, TypeDef},
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(transparent)]
pub struct TypeDb {
    records: BTreeMap<String, TypeDef>,
}

impl TypeDb {
    pub fn get(&self, typename: &str) -> Option<&TypeDef> {
        self.records.get(typename)
    }
    pub fn get_enum_def(&self, typename: &str) -> Option<&EnumDef> {
        self.records.get(typename).and_then(|d| match d {
            TypeDef::Enum(d) => Some(d),
            _ => None,
        })
    }
    pub fn get_struct_def(&self, typename: &str) -> Option<&StructDef> {
        self.records.get(typename).and_then(|d| match d {
            TypeDef::Struct(d) => Some(d),
            _ => None,
        })
    }
    pub fn get_newtype_def(&self, typename: &str) -> Option<&NewtypeDef> {
        self.records.get(typename).and_then(|d| match d {
            TypeDef::Newtype(d) => Some(d),
            _ => None,
        })
    }
    pub fn contains(&self, typename: &str) -> bool {
        self.records.contains_key(typename)
    }

    pub fn reg(&mut self, typename: &str, typedef: TypeDef) -> Result<(), InvalidValidationError> {
        if self.records.contains_key(typename) {
            Err(InvalidValidationError::AlreadyDefinedType {
                typename: typename.to_string(),
            })
        } else {
            self.records.insert(typename.to_string(), typedef);
            Ok(())
        }
    }
}
