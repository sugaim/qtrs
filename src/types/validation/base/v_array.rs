use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    error::{AggregatableError, ValidationError},
    TypeCategory, TypeDb,
};

use super::super::{Validate, Validation};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct ArrayValidationBase {
    element: Box<Validation>,
}

impl ArrayValidationBase {
    pub fn new(element: Box<Validation>) -> Self {
        Self { element }
    }
    pub fn element_validation(&self) -> &Validation {
        self.element.as_ref()
    }
    pub fn take_element_validation(self) -> Box<Validation> {
        self.element
    }
}

impl Validate for ArrayValidationBase {
    type Target = Vec<Value>;
    fn category(&self) -> TypeCategory {
        TypeCategory::Array
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        ValidationError::collect_err(value.iter().map(|v| self.element.validate(v, typedb)))
    }
}
