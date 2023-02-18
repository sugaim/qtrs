use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct StringValidationBase {}

impl Validate for StringValidationBase {
    type Target = String;
    fn category(&self) -> TypeCategory {
        TypeCategory::String
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
