use serde::{Deserialize, Serialize};

pub use self::{
    enum_def::EnumDef, newtype_def::NewtypeDef, struct_def::Field, struct_def::StructDef,
    struct_def::StructTag,
};

mod enum_def;
mod newtype_def;
mod struct_def;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum TypeDef {
    Enum(EnumDef),
    Newtype(NewtypeDef),
    Struct(StructDef),
}

impl TypeDef {
    pub fn as_enum(&self) -> Option<&EnumDef> {
        match self {
            Self::Enum(def) => Some(def),
            _ => None,
        }
    }
    pub fn as_newtype(&self) -> Option<&NewtypeDef> {
        match self {
            Self::Newtype(def) => Some(def),
            _ => None,
        }
    }
    pub fn as_struct(&self) -> Option<&StructDef> {
        match self {
            Self::Struct(def) => Some(def),
            _ => None,
        }
    }
    pub fn is_enum(&self) -> bool {
        self.as_enum().is_some()
    }
    pub fn is_newtype(&self) -> bool {
        self.as_newtype().is_some()
    }
    pub fn is_struct(&self) -> bool {
        self.as_struct().is_some()
    }
}
