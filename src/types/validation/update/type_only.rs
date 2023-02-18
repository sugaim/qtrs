use serde::{Deserialize, Serialize};

use crate::types::{
    error::InvalidValidationError,
    validation::{
        base::{
            AnyValidationBase, ArrayValidationBase, BoolValidationBase, DateTimeValidationBase,
            DateValidationBase, EnumValidationBase, FloatValidationBase, IntegerValidationBase,
            MapValidationBase, NewtypeValidationBase, OptionalValidationBase, SetValidationBase,
            StringValidationBase, TupleValidationBase, UnsignedValidationBase,
            VariantValidationBase,
        },
        RestrictedValueValidation, StructValidation, Validate,
    },
    TypeDb,
};

use super::{IgnoreRestrictions, UpdateValidation};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct TypeOnly {}

impl<T: Validate> UpdateValidation<RestrictedValueValidation<T, <T as Validate>::Target>>
    for TypeOnly
where
    TypeOnly: UpdateValidation<T>,
    <T as Validate>::Target: PartialEq,
{
    fn update_validation(
        &self,
        validation: RestrictedValueValidation<T, <T as Validate>::Target>,
        typedb: &TypeDb,
    ) -> Result<RestrictedValueValidation<T, <T as Validate>::Target>, InvalidValidationError> {
        let updater = IgnoreRestrictions {
            base_updater: self.clone(),
        };
        updater.update_validation(validation, typedb)
    }
}

impl UpdateValidation<AnyValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: AnyValidationBase,
        _: &TypeDb,
    ) -> Result<AnyValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<ArrayValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: ArrayValidationBase,
        _: &TypeDb,
    ) -> Result<ArrayValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<BoolValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: BoolValidationBase,
        _: &TypeDb,
    ) -> Result<BoolValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<DateTimeValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: DateTimeValidationBase,
        _: &TypeDb,
    ) -> Result<DateTimeValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<DateValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: DateValidationBase,
        _: &TypeDb,
    ) -> Result<DateValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<EnumValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: EnumValidationBase,
        _: &TypeDb,
    ) -> Result<EnumValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<FloatValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        _: FloatValidationBase,
        _: &TypeDb,
    ) -> Result<FloatValidationBase, InvalidValidationError> {
        Ok(Default::default())
    }
}

impl UpdateValidation<IntegerValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        _: IntegerValidationBase,
        _: &TypeDb,
    ) -> Result<IntegerValidationBase, InvalidValidationError> {
        Ok(Default::default())
    }
}

impl UpdateValidation<MapValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: MapValidationBase,
        _: &TypeDb,
    ) -> Result<MapValidationBase, InvalidValidationError> {
        Ok(MapValidationBase::new(validation.take_value_validation()))
    }
}

impl UpdateValidation<NewtypeValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: NewtypeValidationBase,
        _: &TypeDb,
    ) -> Result<NewtypeValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<OptionalValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: OptionalValidationBase,
        _: &TypeDb,
    ) -> Result<OptionalValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<SetValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: SetValidationBase,
        _: &TypeDb,
    ) -> Result<SetValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<StringValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        _: StringValidationBase,
        _: &TypeDb,
    ) -> Result<StringValidationBase, InvalidValidationError> {
        Ok(Default::default())
    }
}

impl UpdateValidation<StructValidation> for TypeOnly {
    fn update_validation(
        &self,
        validation: StructValidation,
        _: &TypeDb,
    ) -> Result<StructValidation, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<TupleValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: TupleValidationBase,
        _: &TypeDb,
    ) -> Result<TupleValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}

impl UpdateValidation<UnsignedValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        _: UnsignedValidationBase,
        _: &TypeDb,
    ) -> Result<UnsignedValidationBase, InvalidValidationError> {
        Ok(Default::default())
    }
}

impl UpdateValidation<VariantValidationBase> for TypeOnly {
    fn update_validation(
        &self,
        validation: VariantValidationBase,
        _: &TypeDb,
    ) -> Result<VariantValidationBase, InvalidValidationError> {
        Ok(validation)
    }
}
