use crate::types::{error::InvalidValidationError, TypeDb};

use super::{
    AnyValidation, ArrayValidation, BoolValidation, DateTimeValidation, DateValidation,
    EnumValidation, FloatValidation, IntegerValidation, MapValidation, NewtypeValidation,
    OptionalValidation, SetValidation, StringValidation, StructValidation, TupleValidation,
    UnsignedValidation, Validate, Validation, VariantValidation,
};

pub use self::{
    ignore_restrictions::IgnoreRestrictions, keep_restrictions::KeepRestrictions,
    no_update::NoUpdate, type_only::TypeOnly,
};

mod ignore_restrictions;
mod keep_restrictions;
mod no_update;
mod type_only;

pub trait UpdateValidation<T: Validate> {
    fn update_validation(
        &self,
        validation: T,
        typedb: &TypeDb,
    ) -> Result<T, InvalidValidationError>;
}

pub fn update_validation<T>(
    updater: &T,
    validation: Validation,
    typedb: &TypeDb,
) -> Result<Validation, InvalidValidationError>
where
    T: UpdateValidation<AnyValidation>
        + UpdateValidation<ArrayValidation>
        + UpdateValidation<BoolValidation>
        + UpdateValidation<DateTimeValidation>
        + UpdateValidation<DateValidation>
        + UpdateValidation<EnumValidation>
        + UpdateValidation<FloatValidation>
        + UpdateValidation<IntegerValidation>
        + UpdateValidation<MapValidation>
        + UpdateValidation<NewtypeValidation>
        + UpdateValidation<OptionalValidation>
        + UpdateValidation<SetValidation>
        + UpdateValidation<StringValidation>
        + UpdateValidation<StructValidation>
        + UpdateValidation<TupleValidation>
        + UpdateValidation<UnsignedValidation>
        + UpdateValidation<VariantValidation>,
{
    macro_rules! update_validation_impl {
        ($modifier:ident, $validation:ident, $($c:ident), *) => {
            match $validation {
                $(Validation::$c(validation)
                    => Ok(updater.update_validation(validation, typedb)?.into())), *
            }
        }
    }
    update_validation_impl!(
        updater, validation, Any, Array, Bool, DateTime, Date, Enum, Float, Integer, Map, Newtype,
        Optional, Set, String, Struct, Tuple, Unsigned, Variant
    )
}
