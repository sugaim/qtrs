use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::types::{
    error::{AggregatableError, ValidationError},
    validation::{Validate, Validation},
    TypeCategory, TypeDb,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Field {
    validation: Validation,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    default: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl Field {
    pub fn new(validation: Validation) -> Self {
        Self {
            validation,
            default: None,
            description: None,
        }
    }
    pub fn is_required(&self) -> bool {
        match &self.validation {
            Validation::Optional(_) => true,
            _ => self.default.is_some(),
        }
    }
    pub fn validation(&self) -> &Validation {
        &self.validation
    }
    pub fn take_validation(self) -> Validation {
        self.validation
    }
    pub fn default_value(&self) -> &Option<Value> {
        &self.default
    }
    pub fn take_default(&mut self) -> Option<Value> {
        self.default.take()
    }
    pub fn replace_default(
        &mut self,
        value: Value,
        typedb: &TypeDb,
    ) -> Result<Option<Value>, ValidationError> {
        self.validation.validate(&value, typedb)?;
        Ok(self.default.replace(value))
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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type", content = "value")]
pub enum StructTag {
    Required(String),
    Optional(String),
}

impl StructTag {
    pub fn value(&self) -> &str {
        match self {
            Self::Optional(value) => value.as_str(),
            Self::Required(value) => value.as_str(),
        }
    }
    pub fn is_required(&self) -> bool {
        match self {
            Self::Optional(_) => false,
            Self::Required(_) => true,
        }
    }
    pub fn as_required(&self) -> Option<&str> {
        match self {
            Self::Required(value) => Some(value.as_str()),
            Self::Optional(_) => None,
        }
    }
    pub fn is_optional(&self) -> bool {
        !self.is_required()
    }
    pub fn as_optional(&self) -> Option<&str> {
        match self {
            Self::Optional(value) => Some(value.as_str()),
            Self::Required(_) => None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct StructDef {
    fields: BTreeMap<String, Field>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    tags: BTreeMap<String, StructTag>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    examples: Vec<Value>,
}

impl StructDef {
    pub fn new(fields: BTreeMap<String, Field>, tags: BTreeMap<String, StructTag>) -> Self {
        Self {
            fields,
            tags,
            description: None,
            examples: Vec::default(),
        }
    }
    pub fn fields(&self) -> &BTreeMap<String, Field> {
        &self.fields
    }
    pub fn tags(&self) -> &BTreeMap<String, StructTag> {
        &self.tags
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
}

impl Validate for StructDef {
    type Target = Map<String, Value>;
    fn category(&self) -> TypeCategory {
        TypeCategory::Struct
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        let field_result = ValidationError::collect_err(
            self.fields
                .iter()
                .map(|field| _validate_field(value, field, typedb)),
        );
        let tag_result =
            ValidationError::collect_err(self.tags.iter().map(|tag| _validate_tag(value, tag)));
        ValidationError::merge_result(field_result, tag_result)
    }
}

fn _validate_field(
    value: &Map<String, Value>,
    field: (&String, &Field),
    typedb: &TypeDb,
) -> Result<(), ValidationError> {
    let (name, field) = field;
    if let Some(value) = value.get(name) {
        field
            .validation()
            .validate(value, typedb)
            .map_err(|e| ValidationError::OnPropertyValue {
                name: name.to_string(),
                error: e.into(),
            })
    } else if field.is_required() {
        Err(ValidationError::MissingProperty {
            name: name.to_string(),
        })
    } else {
        Ok(())
    }
}

fn _validate_tag(
    value: &Map<String, Value>,
    tag: (&String, &StructTag),
) -> Result<(), ValidationError> {
    let (name, tag) = tag;
    match value.get(name) {
        Some(Value::String(value)) => {
            if value != tag.value() {
                Err(ValidationError::TagMismatch {
                    name: name.to_string(),
                    expected: tag.value().to_string(),
                    actual: value.to_string(),
                })
            } else {
                Ok(())
            }
        }
        Some(value) => Err(ValidationError::InstanceTypeMismatch {
            value: value.clone(),
            expected: "string",
        }),
        None => {
            if tag.is_required() {
                Err(ValidationError::MissingProperty {
                    name: name.to_string(),
                })
            } else {
                Ok(())
            }
        }
    }
}
