use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TypeCategory {
    Any,
    Array,
    Bool,
    DateTime,
    Date,
    Enum,
    Float,
    Integer,
    Map,
    Newtype,
    Optional,
    Set,
    String,
    Struct,
    Tuple,
    Unsigned,
    Variant,
}

impl Display for TypeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => "any".fmt(f),
            Self::Array => "array".fmt(f),
            Self::Bool => "bool".fmt(f),
            Self::Date => "date".fmt(f),
            Self::DateTime => "date_time".fmt(f),
            Self::Enum => "enum".fmt(f),
            Self::Float => "float".fmt(f),
            Self::Integer => "integer".fmt(f),
            Self::Map => "map".fmt(f),
            Self::Newtype => "newtype".fmt(f),
            Self::Optional => "optional".fmt(f),
            Self::Set => "set".fmt(f),
            Self::String => "string".fmt(f),
            Self::Struct => "struct".fmt(f),
            Self::Tuple => "tuple".fmt(f),
            Self::Unsigned => "unsigned".fmt(f),
            Self::Variant => "variant".fmt(f),
        }
    }
}
