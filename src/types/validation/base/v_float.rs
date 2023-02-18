use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct FloatValidationBase {}

impl Validate for FloatValidationBase {
    type Target = f64;
    fn category(&self) -> TypeCategory {
        TypeCategory::Float
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
