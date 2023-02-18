use serde::{Deserialize, Serialize};

use crate::types::{
    error::{AggregatableError, InvalidValidationError, ValidationError},
    TypeCategory, TypeDb,
};

use super::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RestrictedValueValidation<V, T> {
    base_validation: V,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    restrictions: Option<Vec<T>>,
}

impl<V, T> RestrictedValueValidation<V, T> {
    pub fn base_validation(&self) -> &V {
        &self.base_validation
    }
    pub fn take_base_validation(self) -> V {
        self.base_validation
    }

    pub fn restrictions(&self) -> &Option<Vec<T>> {
        &self.restrictions
    }
    pub fn take_restrictions(&mut self) -> Option<Vec<T>> {
        self.restrictions.take()
    }

    pub fn new(base_validation: V) -> Self {
        Self {
            base_validation,
            restrictions: None,
        }
    }
}

impl<V, T> RestrictedValueValidation<V, T>
where
    V: Validate<Target = T>,
{
    pub fn new_with_restrictions(
        base_validation: V,
        restrictions: Option<Vec<T>>,
        typedb: &TypeDb,
    ) -> Result<Self, InvalidValidationError> {
        if let Some(restrictions) = &restrictions {
            _validate_restrictions(&base_validation, restrictions, typedb)?;
        }
        Ok(Self {
            base_validation,
            restrictions,
        })
    }
}

fn _validate_restrictions<V: Validate>(
    validate: &V,
    restrictions: &Vec<<V as Validate>::Target>,
    typedb: &TypeDb,
) -> Result<(), InvalidValidationError> {
    let mut errs = Vec::default();
    errs.reserve(restrictions.len());
    for (i, value) in restrictions.iter().enumerate() {
        if let Err(err) = validate.validate(value, typedb) {
            errs.push(InvalidValidationError::Validation {
                for_what: format!("{i}-th restriction"),
                cause: err.into(),
            });
        }
    }
    if errs.is_empty() {
        Ok(())
    } else {
        Err(InvalidValidationError::Aggregated(errs))
    }
}

impl<V, T> Validate for RestrictedValueValidation<V, T>
where
    V: Validate<Target = T>,
    T: PartialEq,
{
    type Target = T;
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        let base_result = self.base_validation.validate(value, typedb);
        if let Some(restrictions) = &self.restrictions {
            if restrictions.iter().all(|r| r != value) {
                return ValidationError::merge_result(
                    base_result,
                    Err(ValidationError::RestrictionNotSatisfied),
                );
            }
        }
        base_result
    }
    fn category(&self) -> TypeCategory {
        self.base_validation.category()
    }
}
