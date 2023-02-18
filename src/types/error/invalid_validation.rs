use url::ParseError;

use super::{ValidationError, _agg::AggregatableError};

///
/// Validation itself is invalid.
///
#[derive(Debug)]
pub enum InvalidValidationError {
    /// Enum definition contains multiple `value`
    DuplcatedEnumValue {
        value: String,
    },

    /// Validation is applied `for_what`, but an error occurs due to `cause`
    Validation {
        for_what: String,
        cause: Box<ValidationError>,
    },

    /// Type definition is tried to registered multiple times for `typename`
    AlreadyDefinedType {
        typename: String,
    },

    /// Type definition of `typename` is not found.
    TypeDefNotFound {
        typename: String,
    },

    // Instance not found
    InstanceNotFound {
        path: String,
    },

    // relative uri is used but base is not found
    RelativeUriWithoutBase {
        relative: String,
    },

    // invalid uri
    InvalidUri {
        cause: ParseError,
    },

    /// Many valudation errors occurs.
    Aggregated(Vec<Self>),
}

impl AggregatableError for InvalidValidationError {
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
