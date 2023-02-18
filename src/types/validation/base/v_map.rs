use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{
    error::{AggregatableError, ValidationError},
    TypeCategory, TypeDb,
};

use super::super::{Validate, Validation};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct MapValidationBase {
    value: Box<Validation>,
}

impl MapValidationBase {
    pub fn new(value: Box<Validation>) -> Self {
        Self { value }
    }
    pub fn value_validation(&self) -> &Validation {
        self.value.as_ref()
    }
    pub fn take_value_validation(self) -> Box<Validation> {
        self.value
    }
}

impl Validate for MapValidationBase {
    type Target = Map<String, Value>;
    fn category(&self) -> TypeCategory {
        TypeCategory::Map
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        ValidationError::collect_err(value.iter().map(|(_, v)| self.value.validate(v, typedb)))
    }
}
