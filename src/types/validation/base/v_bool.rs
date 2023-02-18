use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct BoolValidationBase {}

impl Validate for BoolValidationBase {
    type Target = bool;
    fn category(&self) -> TypeCategory {
        TypeCategory::Bool
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
