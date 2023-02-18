use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct IntegerValidationBase {}

impl Validate for IntegerValidationBase {
    type Target = i64;
    fn category(&self) -> TypeCategory {
        TypeCategory::Integer
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
