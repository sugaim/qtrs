use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{error::ValidationError, TypeCategory, TypeDb};

use super::super::{Validate, Validation};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct OptionalValidationBase {
    element: Box<Validation>,
}

impl OptionalValidationBase {
    pub fn new(element: Box<Validation>) -> Self {
        Self { element }
    }
    pub fn element_validation(&self) -> &Validation {
        &self.element
    }
    pub fn take_element_validation(self) -> Box<Validation> {
        self.element
    }
}

impl Validate for OptionalValidationBase {
    type Target = Option<Value>;
    fn category(&self) -> TypeCategory {
        TypeCategory::Optional
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        if let Some(value) = value {
            self.element.validate(value, typedb)?;
            Ok(())
        } else {
            Ok(())
        }
    }
}
