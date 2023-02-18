use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    error::{AggregatableError, ValidationError},
    TypeCategory, TypeDb,
};

use super::super::{Validate, Validation};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TupleValidationBase {
    values: Vec<Validation>,
}

impl TupleValidationBase {
    pub fn new(values: Vec<Validation>) -> Self {
        Self { values }
    }
    pub fn value_validations(&self) -> &Vec<Validation> {
        &self.values
    }
    pub fn take_value_validations(self) -> Vec<Validation> {
        self.values
    }

    pub fn dimension(&self) -> usize {
        self.values.len()
    }
    pub fn categories<'a>(&'a self) -> impl Iterator<Item = TypeCategory> + 'a {
        self.values.iter().map(|v| v.category())
    }
}

impl Validate for TupleValidationBase {
    type Target = Vec<Value>;
    fn category(&self) -> TypeCategory {
        TypeCategory::Tuple
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        if value.len() != self.dimension() {
            return Err(ValidationError::TupleDimensionMismatch {
                expected: self.dimension(),
                actual: value.len(),
            });
        }
        ValidationError::collect_err(
            value
                .iter()
                .zip(self.values.iter())
                .map(|(v, val)| val.validate(v, typedb)),
        )
    }
}
