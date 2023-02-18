use serde::{Deserialize, Serialize};

use crate::types::{
    error::{InvalidValidationError, ValidationError},
    TypeCategory, TypeDb,
};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct EnumValidationBase {
    typename: String,
}

impl EnumValidationBase {
    pub fn new(typename: String, typedb: &TypeDb) -> Result<Self, InvalidValidationError> {
        if typedb.get_enum_def(&typename).is_none() {
            return Err(InvalidValidationError::TypeDefNotFound {
                typename: typename.to_string(),
            });
        }
        Ok(Self { typename })
    }
    pub fn typename(&self) -> &str {
        &self.typename
    }
}

impl Validate for EnumValidationBase {
    type Target = String;
    fn category(&self) -> TypeCategory {
        TypeCategory::Enum
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        typedb
            .get_enum_def(&self.typename)
            .ok_or(ValidationError::TypeDefNotFound {
                typename: self.typename.clone(),
            })?
            .validate(value, typedb)
    }
}
