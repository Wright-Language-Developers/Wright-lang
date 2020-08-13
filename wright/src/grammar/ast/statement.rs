use crate::grammar::ast::Expression;
use std::fmt::Debug;
use serde::Serialize;

/// An expression whose results aren't used.
#[derive(Clone, Debug, Serialize)]
pub struct ExpressionStatement<SourceCodeReference: Clone + Debug + Serialize> {
    /// Associated source code.
    pub source: SourceCodeReference,
    /// The expression.
    pub inner: Box<Expression<SourceCodeReference>>,
}

/// A statement in wright source code.
#[allow(missing_docs)]
#[derive(Clone, Debug, Serialize)]
pub enum Statement<SourceCodeReference: Clone + Debug + Serialize> {
    ExpressionStatement(ExpressionStatement<SourceCodeReference>),
}
