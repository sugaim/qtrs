use serde::{Deserialize, Serialize};

use crate::types::{
    error::InvalidValidationError,
    validation::{RestrictedValueValidation, Validate},
    TypeDb,
};

use super::UpdateValidation;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct IgnoreRestrictions<U> {
    pub base_updater: U,
}

impl<T: Validate, U> UpdateValidation<RestrictedValueValidation<T, <T as Validate>::Target>>
    for IgnoreRestrictions<U>
where
    U: UpdateValidation<T>,
    <T as Validate>::Target: PartialEq,
{
    fn update_validation(
        &self,
        validation: RestrictedValueValidation<T, <T as Validate>::Target>,
        typedb: &TypeDb,
    ) -> Result<RestrictedValueValidation<T, <T as Validate>::Target>, InvalidValidationError> {
        let updated = self
            .base_updater
            .update_validation(validation.take_base_validation(), typedb)?;
        Ok(RestrictedValueValidation::new(updated))
    }
}
