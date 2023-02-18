use std::str::FromStr;

use chrono::{DateTime, FixedOffset, NaiveDate};
use serde_json::{Map, Value};

use super::error::ValidationError;

pub(crate) trait FromJson: Sized {
    fn from_json(value: Value) -> Result<Self, ValidationError>;
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError>;
}
impl<T: FromJson> FromJson for Option<T> {
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        if value.is_null() {
            Ok(None)
        } else {
            T::from_json(value).map(Some)
        }
    }
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        if value.is_null() {
            Ok(None)
        } else {
            T::from_json_ref(value).map(Some)
        }
    }
}
impl FromJson for Value {
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        Ok(value)
    }
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        Ok(value.clone())
    }
}
impl FromJson for i64 {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value.as_i64().ok_or(ValidationError::InstanceTypeMismatch {
            value: value.clone(),
            expected: "integer",
        })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        Self::from_json_ref(&value)
    }
}
impl FromJson for u64 {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value.as_u64().ok_or(ValidationError::InstanceTypeMismatch {
            value: value.clone(),
            expected: "integer",
        })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        Self::from_json_ref(&value)
    }
}
impl FromJson for f64 {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value.as_f64().ok_or(ValidationError::InstanceTypeMismatch {
            value: value.clone(),
            expected: "number",
        })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        Self::from_json_ref(&value)
    }
}
impl FromJson for String {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value
            .as_str()
            .map(ToString::to_string)
            .ok_or(ValidationError::InstanceTypeMismatch {
                value: value.clone(),
                expected: "string",
            })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        Self::from_json_ref(&value)
    }
}
impl FromJson for bool {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value
            .as_bool()
            .ok_or(ValidationError::InstanceTypeMismatch {
                value: value.clone(),
                expected: "boolean",
            })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        Self::from_json_ref(&value)
    }
}
impl FromJson for Vec<Value> {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value
            .as_array()
            .cloned()
            .ok_or(ValidationError::InstanceTypeMismatch {
                value: value.clone(),
                expected: "array",
            })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        if let Value::Array(values) = value {
            Ok(values)
        } else {
            Err(ValidationError::InstanceTypeMismatch {
                value: value.clone(),
                expected: "array",
            })
        }
    }
}
impl FromJson for Map<String, Value> {
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        value
            .as_object()
            .cloned()
            .ok_or(ValidationError::InstanceTypeMismatch {
                value: value.clone(),
                expected: "object",
            })
    }
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        if let Value::Object(values) = value {
            Ok(values)
        } else {
            Err(ValidationError::InstanceTypeMismatch {
                value: value.clone(),
                expected: "object",
            })
        }
    }
}
impl FromJson for DateTime<FixedOffset> {
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        let s = String::from_json(value)?;
        DateTime::parse_from_rfc3339(&s).map_err(|_| ValidationError::DateTimeParseError {
            value: s.to_string(),
        })
    }
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        let s = String::from_json_ref(value)?;
        DateTime::parse_from_rfc3339(&s).map_err(|_| ValidationError::DateTimeParseError {
            value: s.to_string(),
        })
    }
}
impl FromJson for NaiveDate {
    fn from_json(value: Value) -> Result<Self, ValidationError> {
        let s = String::from_json(value)?;
        NaiveDate::from_str(s.as_str()).map_err(|_| ValidationError::DateParseError {
            value: s.to_string(),
        })
    }
    fn from_json_ref(value: &Value) -> Result<Self, ValidationError> {
        let s = String::from_json_ref(value)?;
        NaiveDate::from_str(s.as_str()).map_err(|_| ValidationError::DateParseError {
            value: s.to_string(),
        })
    }
}
