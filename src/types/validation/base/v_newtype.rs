use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    error::{InvalidValidationError, ValidationError},
    TypeCategory, TypeDb,
};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct NewtypeValidationBase {
    typename: String,
}

impl NewtypeValidationBase {
    pub fn new(typename: String, typedb: &TypeDb) -> Result<Self, InvalidValidationError> {
        typedb
            .get_newtype_def(&typename)
            .ok_or(InvalidValidationError::TypeDefNotFound {
                typename: typename.to_string(),
            })?;
        Ok(Self { typename })
    }
    pub fn typename(&self) -> &str {
        self.typename.as_str()
    }
}

impl Validate for NewtypeValidationBase {
    type Target = Value;
    fn category(&self) -> TypeCategory {
        TypeCategory::Newtype
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        typedb
            .get_newtype_def(&self.typename)
            .ok_or(ValidationError::TypeDefNotFound {
                typename: self.typename.clone(),
            })?
            .validate(value, typedb)
            .map_err(|e| ValidationError::Newtype {
                typename: self.typename.clone(),
                cause: e.into(),
            })
    }
}
