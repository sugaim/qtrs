use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{
    error::{InvalidValidationError, ValidationError},
    TypeCategory, TypeDb,
};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StructValidationBase {
    typename: String,
}

impl StructValidationBase {
    pub fn new(typename: String, typedb: &TypeDb) -> Result<Self, InvalidValidationError> {
        typedb
            .get_struct_def(&typename)
            .ok_or(InvalidValidationError::TypeDefNotFound {
                typename: typename.to_string(),
            })?;
        Ok(Self { typename })
    }
    pub fn typename(&self) -> &str {
        self.typename.as_str()
    }
}

impl Validate for StructValidationBase {
    type Target = Map<String, Value>;
    fn category(&self) -> TypeCategory {
        TypeCategory::Struct
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        typedb
            .get_struct_def(&self.typename)
            .ok_or(ValidationError::TypeDefNotFound {
                typename: self.typename.to_string(),
            })?
            .validate(value, typedb)
    }
}
