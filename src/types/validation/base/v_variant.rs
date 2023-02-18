use std::{cmp::Ordering, collections::BTreeMap};

use itertools::Itertools;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::types::{
    error::{InvalidValidationError, ValidationError},
    typedef::{StructDef, StructTag},
    TypeCategory, TypeDb,
};

use super::super::{Validate, Validation};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub struct VariantValidationBase {
    variants: Vec<Validation>,
}

impl VariantValidationBase {
    pub fn new(variants: Vec<Validation>) -> Self {
        Self { variants }
    }
    pub fn variants(&self) -> &Vec<Validation> {
        &self.variants
    }
    pub fn take_variants(self) -> Vec<Validation> {
        self.variants
    }
    pub fn priored_validations<'a>(
        &'a self,
        typedb: &'a TypeDb,
    ) -> Result<
        BTreeMap<RequiredTagMatcher<'a>, Vec<(TagMatcher<'a>, &'a Validation)>>,
        InvalidValidationError,
    > {
        let mut result = BTreeMap::default();
        let mut errs = Vec::default();
        for validation in self.variants.iter().rev() {
            match validation {
                Validation::Enum(_) => reg_non_struct_validation(&mut result, validation),
                Validation::Newtype(_) => reg_non_struct_validation(&mut result, validation),
                Validation::Struct(v) => {
                    if let Some(typedef) = typedb.get_struct_def(v.base_validation().typename()) {
                        reg_struct_validation(&mut result, validation, typedef);
                    } else {
                        errs.push(InvalidValidationError::TypeDefNotFound {
                            typename: v.base_validation().typename().to_string(),
                        });
                    }
                }
                _ => {}
            }
        }
        if !errs.is_empty() {
            return Err(InvalidValidationError::Aggregated(errs));
        }
        for (_, sub_validations) in &mut result {
            sub_validations.sort_by(|l, r| l.0.cmp(&r.0));
        }
        Ok(result)
    }
}

impl Validate for VariantValidationBase {
    type Target = Value;
    fn category(&self) -> TypeCategory {
        TypeCategory::Variant
    }
    fn validate(&self, value: &Self::Target, typedb: &TypeDb) -> Result<(), ValidationError> {
        let candidates =
            self.priored_validations(typedb)
                .map_err(|e| ValidationError::InvalidValidation {
                    for_what: "generate variant validator".to_owned(),
                    cause: e.into(),
                })?;
        let candidates = candidates
            .iter()
            .filter(|(m, _)| m.is_match(value))
            .flat_map(|(_, deopts)| deopts)
            .filter_map(|(m, d)| if m.is_match(value) { Some(d) } else { None });
        for candidate in candidates {
            if candidate.validate(value, typedb).is_ok() {
                return Ok(());
            }
        }
        Err(ValidationError::VariantMismatch {
            value: value.clone(),
        })
    }
}

fn is_match<'a, I>(requirements: I, value: &Value) -> bool
where
    I: Iterator<Item = (&'a str, &'a str)>,
{
    if let Value::Object(values) = value {
        for (tag, required) in requirements {
            if let Some(Value::String(value)) = values.get(tag) {
                if value != required {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    } else {
        false
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RequiredTagMatcher<'a> {
    tags: &'a BTreeMap<String, StructTag>,
}

impl<'a> RequiredTagMatcher<'a> {
    pub fn requirements(&'a self) -> impl Iterator<Item = (&'a str, &'a str)> {
        self.tags
            .iter()
            .filter(|kv| kv.1.is_required())
            .map(|kv| (kv.0.as_str(), kv.1.value()))
    }
    pub fn is_match(&self, value: &Value) -> bool {
        is_match(self.requirements(), value)
    }
}

impl<'a> PartialEq for RequiredTagMatcher<'a> {
    fn eq(&self, other: &Self) -> bool {
        let mut s = self.requirements();
        let mut o = other.requirements();
        loop {
            match (s.next(), o.next()) {
                (None, None) => return true,
                (Some(_), None) => return false,
                (None, Some(_)) => return false,
                (Some(s), Some(o)) => {
                    if s != o {
                        return false;
                    }
                }
            }
        }
    }
}

fn cmp<'a>(lhs: &RequiredTagMatcher<'a>, rhs: &RequiredTagMatcher<'a>) -> Ordering {
    let mut s = lhs.requirements();
    let mut o = rhs.requirements();
    loop {
        match (s.next(), o.next()) {
            (None, None) => return Ordering::Equal,
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (Some(l), Some(r)) => match l.cmp(&r) {
                Ordering::Equal => {}
                other => return other,
            },
        }
    }
}

impl<'a> PartialOrd for RequiredTagMatcher<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(cmp(self, other))
    }
}
impl<'a> Eq for RequiredTagMatcher<'a> {}
impl<'a> Ord for RequiredTagMatcher<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp(self, other)
    }
}

#[derive(Debug, Clone)]
pub struct TagMatcher<'a> {
    tags: Vec<(&'a str, &'a str)>,
}
impl<'a> PartialEq for TagMatcher<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.tags.eq(&other.tags)
    }
}
impl<'a> TagMatcher<'a> {
    pub fn requirements<'b>(&'b self) -> impl Iterator<Item = (&'b str, &'b str)> {
        self.tags.iter().copied()
    }
    pub fn is_match(&self, value: &Value) -> bool {
        is_match(self.requirements(), value)
    }
}

fn cmp_tag_matcher<'a>(lhs: &TagMatcher<'a>, rhs: &TagMatcher<'a>) -> Ordering {
    match lhs.tags.len().cmp(&rhs.tags.len()) {
        Ordering::Equal => {}
        other => return other,
    }
    if lhs.tags.iter().tuple_windows().all(|(f, s)| f < s)
        && rhs.tags.iter().tuple_windows().all(|(f, s)| f < s)
    {
        return lhs.tags.cmp(&rhs.tags);
    } else {
        let mut lhs = lhs.tags.clone();
        let mut rhs = rhs.tags.clone();
        lhs.sort();
        rhs.sort();
        return lhs.cmp(&rhs);
    }
}
impl<'a> PartialOrd for TagMatcher<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp_tag_matcher(&self, &other))
    }
}
impl<'a> Eq for TagMatcher<'a> {}
impl<'a> Ord for TagMatcher<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_tag_matcher(&self, &other)
    }
}

fn reg_struct_validation<'a>(
    result: &mut BTreeMap<RequiredTagMatcher<'a>, Vec<(TagMatcher<'a>, &'a Validation)>>,
    validation: &'a Validation,
    typedef: &'a StructDef,
) {
    let required = RequiredTagMatcher {
        tags: typedef.tags(),
    };
    if !result.contains_key(&required) {
        result.insert(required, Vec::default());
    }
    let candidates = result.get_mut(&required).unwrap();
    let opt_tags = typedef
        .tags()
        .iter()
        .filter(|kv| !kv.1.is_required())
        .map(|kv| (kv.0.as_str(), kv.1.value()));
    for i in 0..opt_tags.clone().count() {
        for opt_tags in opt_tags.clone().combinations(i) {
            candidates.push((TagMatcher { tags: opt_tags }, validation));
        }
    }
}

fn reg_non_struct_validation<'a>(
    result: &mut BTreeMap<RequiredTagMatcher<'a>, Vec<(TagMatcher<'a>, &'a Validation)>>,
    validation: &'a Validation,
) {
    static EMPTY: Lazy<BTreeMap<String, StructTag>> = Lazy::new(|| BTreeMap::default());
    let required = RequiredTagMatcher { tags: &*EMPTY };
    if !result.contains_key(&required) {
        result.insert(required, Vec::default());
    }
    let candidates = result.get_mut(&required).unwrap();
    candidates.push((
        TagMatcher {
            tags: Vec::default(),
        },
        validation,
    ));
}
