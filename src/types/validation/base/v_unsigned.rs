use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct UnsignedValidationBase {}

impl Validate for UnsignedValidationBase {
    type Target = u64;
    fn category(&self) -> TypeCategory {
        TypeCategory::Integer
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
