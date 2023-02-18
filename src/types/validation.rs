use serde::{Deserialize, Serialize};
use serde_json::Value;

use self::base::{
    AnyValidationBase, ArrayValidationBase, BoolValidationBase, DateTimeValidationBase,
    DateValidationBase, EnumValidationBase, FloatValidationBase, IntegerValidationBase,
    MapValidationBase, NewtypeValidationBase, OptionalValidationBase, SetValidationBase,
    StringValidationBase, StructValidationBase, TupleValidationBase, UnsignedValidationBase,
    VariantValidationBase,
};
use super::{from_json::FromJson, TypeCategory};
pub use restricted::RestrictedValueValidation;

use super::{error::ValidationError, TypeDb};

pub mod base;
mod restricted;
pub mod update;

pub trait Validate {
    type Target;

    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError>;
    fn category(&self) -> TypeCategory;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Validation {
    Any(AnyValidation),
    Array(ArrayValidation),
    Bool(BoolValidation),
    DateTime(DateTimeValidation),
    Date(DateValidation),
    Enum(EnumValidation),
    Float(FloatValidation),
    Integer(IntegerValidation),
    Map(MapValidation),
    Newtype(NewtypeValidation),
    Optional(OptionalValidation),
    Set(SetValidation),
    String(StringValidation),
    Struct(StructValidation),
    Tuple(TupleValidation),
    Unsigned(UnsignedValidation),
    Variant(VariantValidation),
}

impl Validate for Validation {
    type Target = Value;
    fn category(&self) -> super::TypeCategory {
        macro_rules! _define {
            ($s:ident, $($i:ident), *) => {
                match $s {
                    $(Validation::$i(v) => v.category()), *
                }
            }
        }
        _define!(
            self, Any, Array, Bool, DateTime, Date, Enum, Float, Integer, Map, Newtype, Optional,
            Set, String, Struct, Tuple, Unsigned, Variant
        )
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        macro_rules! _define {
            ($s:ident, $v:ident, $t:ident, $($i:ident), *) => {
                match $s {
                    $(Validation::$i(v) => v.validate(&FromJson::from_json_ref($v)?, $t)), *
                }
            }
        }
        _define!(
            self, value, typedb, Any, Array, Bool, DateTime, Date, Enum, Float, Integer, Map,
            Newtype, Optional, Set, String, Struct, Tuple, Unsigned, Variant
        )
    }
}

impl Default for Validation {
    fn default() -> Self {
        Self::Any(Default::default())
    }
}

macro_rules! define_alias_and_conversion {
    ($t:ident, $base:ty, $i:ident) => {
        pub type $t = RestrictedValueValidation<$base, <$base as Validate>::Target>;

        impl From<$t> for Validation {
            fn from(value: $t) -> Self {
                Self::$i(value)
            }
        }
        impl From<$base> for Validation {
            fn from(value: $base) -> Self {
                Self::$i($t::new(value))
            }
        }
    };
}

define_alias_and_conversion!(AnyValidation, AnyValidationBase, Any);
define_alias_and_conversion!(ArrayValidation, ArrayValidationBase, Array);
define_alias_and_conversion!(BoolValidation, BoolValidationBase, Bool);
define_alias_and_conversion!(DateTimeValidation, DateTimeValidationBase, DateTime);
define_alias_and_conversion!(DateValidation, DateValidationBase, Date);
define_alias_and_conversion!(EnumValidation, EnumValidationBase, Enum);
define_alias_and_conversion!(FloatValidation, FloatValidationBase, Float);
define_alias_and_conversion!(IntegerValidation, IntegerValidationBase, Integer);
define_alias_and_conversion!(MapValidation, MapValidationBase, Map);
define_alias_and_conversion!(NewtypeValidation, NewtypeValidationBase, Newtype);
define_alias_and_conversion!(OptionalValidation, OptionalValidationBase, Optional);
define_alias_and_conversion!(SetValidation, SetValidationBase, Set);
define_alias_and_conversion!(StringValidation, StringValidationBase, String);
define_alias_and_conversion!(StructValidation, StructValidationBase, Struct);
define_alias_and_conversion!(TupleValidation, TupleValidationBase, Tuple);
define_alias_and_conversion!(UnsignedValidation, UnsignedValidationBase, Unsigned);
define_alias_and_conversion!(VariantValidation, VariantValidationBase, Variant);
