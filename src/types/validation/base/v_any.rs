use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct AnyValidationBase {}

impl Validate for AnyValidationBase {
    type Target = Value;
    fn category(&self) -> TypeCategory {
        TypeCategory::Any
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
