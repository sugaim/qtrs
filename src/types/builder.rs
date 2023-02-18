use serde_json::Value;

use super::error::InvalidValidationError;

mod ref_visitor;

pub trait VisitReference: Sized {
    type Visited: VisitReference;

    fn visit_reference(&self, reference: &str) -> Result<Self::Visited, InvalidValidationError>;

    fn current_id(&'_ self) -> Option<&'_ str>;
    fn value(&'_ self) -> &'_ Value;
}
