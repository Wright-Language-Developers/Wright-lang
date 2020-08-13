use std::fmt::Debug;
use serde::Serialize;
use crate::grammar::ast::ScopedName;

/// A wright primitive type.
/// Wright has almost the same set of primitive types as rust.
/// The ones here all correspond almost equivalently to their rust
/// counterparts. The exception is String, which acts more like a
/// Java String or rust's `&str`.
#[allow(missing_docs)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize)]
pub enum PrimitiveTypeVariant {
    Char,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128,
    Bool,
    String,
}

/// A primitive type in source code.
#[derive(Clone, Debug, Serialize)]
pub struct PrimitiveType<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// Represented variant.
    pub variant: PrimitiveTypeVariant,
}

/// A user defined type.
#[derive(Clone, Debug, Serialize)]
pub struct UserType<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// The type name.
    pub base: ScopedName<SourceCodeReference>,
    /// Generic type parameters
    pub type_params: Vec<Type<SourceCodeReference>>,
}

/// A type in source code.
#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub enum Type<SourceCodeReference: Clone + Debug + Serialize> {
    Primitive(PrimitiveType<SourceCodeReference>),
    User(UserType<SourceCodeReference>)
}
