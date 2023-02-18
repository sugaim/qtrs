use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::types::{
    error::{AggregatableError, InvalidValidationError, ValidationError},
    validation::Validate,
    TypeCategory, TypeDb,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct EnumDef {
    values: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl EnumDef {
    pub fn new(values: Vec<String>) -> Result<Self, InvalidValidationError> {
        InvalidValidationError::collect(
            values
                .iter()
                .counts()
                .iter()
                .filter(|(_, n)| n > &&1)
                .map(|(dup, _)| InvalidValidationError::DuplcatedEnumValue {
                    value: dup.to_string(),
                }),
        )?;
        Ok(Self {
            values,
            description: None,
        })
    }

    pub fn values(&self) -> &Vec<String> {
        &self.values
    }
    pub fn take_values(self) -> Vec<String> {
        self.values
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
}

impl Validate for EnumDef {
    type Target = String;
    fn category(&self) -> TypeCategory {
        TypeCategory::Enum
    }
    fn validate(&self, value: &Self::Target, _: &TypeDb) -> Result<(), ValidationError> {
        if !self.values().contains(value) {
            Err(ValidationError::UnknownEnumValue {
                value: value.clone(),
                candidates: self.values().clone(),
            })
        } else {
            Ok(())
        }
    }
}
