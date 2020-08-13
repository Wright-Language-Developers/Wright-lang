use crate::grammar::ast::{BooleanLit, CharLit, Identifier, NumLit, ScopedName, StringLit};
use std::fmt::Debug;
use serde::Serialize;

/// A Pattern used in pattern matching.
#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub enum Pattern<SourceCodeReference: Clone + Debug + Serialize> {
    NumLit(NumLitPattern<SourceCodeReference>),
    CharLit(CharLit<SourceCodeReference>),
    StringLit(StringLit<SourceCodeReference>),
    BooleanLit(BooleanLit<SourceCodeReference>),
    Identifier(Identifier<SourceCodeReference>),
    ScopedName(ScopedName<SourceCodeReference>),
    Underscore(Underscore<SourceCodeReference>),
}

/// An underscore pattern in source code.
#[derive(Clone, Debug, Serialize)]
pub struct Underscore<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
}

/// Number literal pattern
#[derive(Clone, Debug, Serialize)]
pub struct NumLitPattern<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code reference.
    pub source: SourceCodeReference,
    /// Whether the number literal pattern has '-' in front
    pub negative: bool,
    /// Inner number literal value
    pub inner: NumLit<SourceCodeReference>,
}
