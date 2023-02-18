use std::collections::BTreeMap;

use serde_json::Value;
use url::Url;

use crate::types::error::InvalidValidationError;

use super::VisitReference;

pub struct RefVisitor {
    root_id: Option<String>,
    root_schema: Value,
    resources: BTreeMap<String, Value>,
}

pub struct RefVisitorCursor<'a> {
    master: &'a RefVisitor,
    root: &'a Value,
    value: &'a Value,
    id: Option<&'a str>,
}

impl<'a> VisitReference for &'a RefVisitor {
    type Visited = RefVisitorCursor<'a>;
    fn current_id(&'_ self) -> Option<&'_ str> {
        self.root_id.as_ref().map(|id| id.as_str())
    }
    fn value(&'_ self) -> &'_ Value {
        &self.root_schema
    }
    fn visit_reference(&self, reference: &str) -> Result<Self::Visited, InvalidValidationError> {
        if reference.starts_with("#/") {
            return visit_internal(
                self,
                &self.root_schema,
                self.root_id.map(|s| s.as_str()),
                &reference[1..],
            );
        }
        let mut uri =
            Url::parse(reference).map_err(|e| InvalidValidationError::InvalidUri { cause: e })?;
        match uri.fragment().map(|s| s.to_owned()) {
            None => self.visit_reference(uri.as_str()),
            Some(fragment) => {
                uri.set_fragment(None);
                self.visit_reference(uri.as_str())?
                    .visit_reference(&fragment)
            }
        }
    }
}

impl<'a> VisitReference for RefVisitorCursor<'a> {
    type Visited = Self;
    fn current_id(&'_ self) -> Option<&'_ str> {
        self.id.clone()
    }
    fn value(&'_ self) -> &'_ Value {
        self.value
    }
    fn visit_reference(&self, reference: &str) -> Result<Self::Visited, InvalidValidationError> {
        visit_reference_impl(self, self.master, self.root, self.id, reference)
    }
}

fn visit_internal<'a>(
    master: &'a RefVisitor,
    root: &'a Value,
    id: Option<&'a str>,
    reference: &'a str,
) -> Result<RefVisitorCursor<'a>, InvalidValidationError> {
    return match root.pointer(&reference) {
        None => Err(InvalidValidationError::InstanceNotFound {
            path: id
                .as_ref()
                .map(|id| id.to_string() + reference)
                .unwrap_or(reference.to_owned()),
        }),
        Some(value) => Ok(RefVisitorCursor {
            master,
            root,
            value,
            id,
        }),
    };
}

fn visit_anchored<'a>(
    master: &'a RefVisitor,
    id: Option<&'a str>,
    reference: &'a str,
) -> Result<RefVisitorCursor<'a>, InvalidValidationError> {
    let anchored_id = id
        .map(|id| id.to_string() + reference)
        .unwrap_or(reference.to_owned());

    // because anchor does not have path-information,
    // anchored schema is assumed to be collected apriori.
    return match master.resources.get(&anchored_id) {
        None => Err(InvalidValidationError::InstanceNotFound { path: anchored_id }),
        Some(value) => Ok(RefVisitorCursor {
            id,
            master,
            value,
            root: value,
        }),
    };
}

fn visit_relative_uri<'a>(
    master: &'a RefVisitor,
    id: Option<&'a str>,
    reference: &'a str,
) -> Result<RefVisitorCursor<'a>, InvalidValidationError> {
    if id.is_none() {
        return Err(InvalidValidationError::RelativeUriWithoutBase {
            relative: reference.to_owned(),
        });
    }
    let uri = Url::parse(id.unwrap())
        .map_err(|e| InvalidValidationError::InvalidUri { cause: e })?
        .join(reference)
        .map_err(|e| InvalidValidationError::InvalidUri { cause: e })?;
    return match master.resources.get(uri.as_str()) {
        None => Err(InvalidValidationError::InstanceNotFound {
            path: uri.as_str().to_owned(),
        }),
        Some(value) => Ok(RefVisitorCursor {
            id,
            master,
            value,
            root: value,
        }),
    };
}

fn visit_reference_impl<'a, T: VisitReference<Visited = RefVisitorCursor<'a>>>(
    visitor: &'a T,
    master: &'a RefVisitor,
    root: &'a Value,
    id: Option<&'a str>,
    reference: &'a str,
) -> Result<RefVisitorCursor<'a>, InvalidValidationError> {
    if reference.starts_with("#/") {
        // inside self
        return match root.pointer(&reference[1..]) {
            None => Err(InvalidValidationError::InstanceNotFound {
                path: id
                    .as_ref()
                    .map(|id| id.to_string() + reference)
                    .unwrap_or(reference.to_owned()),
            }),
            Some(value) => Ok(RefVisitorCursor {
                master,
                root,
                value,
                id,
            }),
        };
    }
    if reference.starts_with("#") {
        // anchored
        let anchored_id = id
            .map(|id| id.to_string() + reference)
            .unwrap_or(reference.to_owned());

        // because anchor does not have path-information,
        // anchored schema is assumed to be collected apriori.
        return match master.resources.get(&anchored_id) {
            None => Err(InvalidValidationError::InstanceNotFound { path: anchored_id }),
            Some(value) => Ok(RefVisitorCursor {
                id,
                master,
                value,
                root: value,
            }),
        };
    }
    if reference.starts_with("/") {
        // relative uri
        if id.is_none() {
            return Err(InvalidValidationError::RelativeUriWithoutBase {
                relative: reference.to_owned(),
            });
        }
        let uri = Url::parse(id.unwrap())
            .map_err(|e| InvalidValidationError::InvalidUri { cause: e })?
            .join(reference)
            .map_err(|e| InvalidValidationError::InvalidUri { cause: e })?;
        return match master.resources.get(uri.as_str()) {
            None => Err(InvalidValidationError::InstanceNotFound {
                path: uri.as_str().to_owned(),
            }),
            Some(value) => Ok(RefVisitorCursor {
                id,
                master,
                value,
                root: value,
            }),
        };
    }
    let mut uri =
        Url::parse(reference).map_err(|e| InvalidValidationError::InvalidUri { cause: e })?;
    match uri.fragment().map(|s| s.to_owned()) {
        None => visitor.visit_reference(uri.as_str()),
        Some(fragment) => {
            uri.set_fragment(None);
            visitor
                .visit_reference(uri.as_str())?
                .visit_reference(&fragment)
        }
    }
}
