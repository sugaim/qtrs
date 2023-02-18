use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct DateTimeValidationBase {}

impl Validate for DateTimeValidationBase {
    type Target = DateTime<FixedOffset>;
    fn category(&self) -> TypeCategory {
        TypeCategory::DateTime
    }
    fn validate(&self, _: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        Ok(())
    }
}
