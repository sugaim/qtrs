use serde_json::Value;

use super::{InvalidValidationError, _agg::AggregatableError};

///
/// Validation errors.
///
#[derive(Debug)]
pub enum ValidationError {
    /// `value` must be an either of `candidates`
    UnknownEnumValue {
        value: String,
        candidates: Vec<String>,
    },

    /// Property `name` is required but not found.
    MissingProperty { name: String },

    /// An error `error` occurs on a value of property `name`
    OnPropertyValue {
        name: String,
        error: Box<ValidationError>,
    },

    /// Validation error occurs for newtype `typename` due to `cause`
    Newtype {
        typename: String,
        cause: Box<ValidationError>,
    },

    /// An object is expected to have a tag `name`.
    /// Expected value was `expected`, but actual value was `actual`.
    TagMismatch {
        name: String,
        expected: String,
        actual: String,
    },

    /// An instance type of json value `value` is expected to be `expected`
    InstanceTypeMismatch {
        value: Value,
        expected: &'static str,
    },

    /// Type definition for `typename` is not found.
    TypeDefNotFound { typename: String },

    /// A dimension of tuple mismatches.
    /// Expected was `expected` but actual size was `actual`.
    TupleDimensionMismatch { expected: usize, actual: usize },

    /// Json value `value` does not match with any variants
    VariantMismatch { value: Value },

    /// `value` is not a RFC3339 date-time string
    DateTimeParseError { value: String },

    /// `value` is not a ISO8601 date string
    DateParseError { value: String },

    /// Validation itself is tried to use `for_what`, but an error occurs due to `cause`.
    InvalidValidation {
        for_what: String,
        cause: Box<InvalidValidationError>,
    },

    /// Only explicitly restricted value is available.
    RestrictionNotSatisfied,

    /// Many valudation errors occurs.
    Aggregated(Vec<ValidationError>),
}

impl AggregatableError for ValidationError {
    fn aggregate(values: Vec<Self>) -> Self {
        Self::Aggregated(values)
    }
    fn as_many(self) -> Result<Vec<Self>, Self> {
        match self {
            Self::Aggregated(values) => Ok(values),
            value => Err(value),
        }
    }
}
