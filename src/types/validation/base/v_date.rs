use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct DateValidationBase {}

impl Validate for DateValidationBase {
    type Target = NaiveDate;
    fn category(&self) -> TypeCategory {
        TypeCategory::Date
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
