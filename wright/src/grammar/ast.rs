use serde::Serialize;

/// Expression nodes in the AST.
pub mod expression;

/// Nodes for literals in AST.
pub mod literal;

/// Statement nodes in the AST.
pub mod statement;

/// Type nodes in the AST.
pub mod types;

/// Pattern matching AST nodes.
pub mod pattern;

/// Check if two AST nodes are equal by serializing them both to JSON and checking the equality
/// of the JSON strings.
pub fn ast_eq<A: Serialize, B: Serialize>(a: &A, b: &B) -> bool {
    serde_json::to_string(a).eq(&serde_json::to_string(b))
}