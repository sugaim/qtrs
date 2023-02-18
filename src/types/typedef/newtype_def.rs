use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    error::{AggregatableError, ValidationError},
    validation::{Validate, Validation},
    TypeCategory, TypeDb,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct NewtypeDef {
    validation: Validation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    examples: Vec<Value>,
}

impl NewtypeDef {
    pub fn new(validation: Validation) -> Self {
        Self {
            validation,
            description: None,
            examples: Vec::default(),
        }
    }

    pub fn validation(&self) -> &Validation {
        &self.validation
    }
    pub fn take_validation(self) -> Validation {
        self.validation
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }
    pub fn take_description(&mut self) -> Option<String> {
        self.description.take()
    }
    pub fn replace_description(&mut self, desc: String) -> Option<String> {
        self.description.replace(desc)
    }

    pub fn examples(&self) -> &Vec<Value> {
        &self.examples
    }
    pub fn push_example(&mut self, value: Value, typedb: &TypeDb) -> Result<(), ValidationError> {
        self.validate(&value, typedb)?;
        self.examples.push(value);
        Ok(())
    }
    pub fn push_examples<I>(&mut self, values: I, typedb: &TypeDb) -> Result<(), ValidationError>
    where
        I: Iterator<Item = Value>,
    {
        ValidationError::collect_err(values.map(|v| self.push_example(v, typedb)))
    }
    pub fn clear_examples(&mut self) {
        self.examples.clear()
    }
}

impl Validate for NewtypeDef {
    type Target = Value;
    fn category(&self) -> TypeCategory {
        TypeCategory::Newtype
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        self.validation.validate(&value, typedb)
    }
}
