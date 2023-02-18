use serde::{Deserialize, Serialize};

use crate::types::{error::InvalidValidationError, validation::Validate, TypeDb};

use super::UpdateValidation;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default)]
pub struct NoUpdate {}

impl<T: Validate> UpdateValidation<T> for NoUpdate {
    fn update_validation(&self, validation: T, _: &TypeDb) -> Result<T, InvalidValidationError> {
        Ok(validation)
    }
}
