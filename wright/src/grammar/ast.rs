/// Expression nodes in the AST.
pub mod expression;

/// Reexport expression nodes publicly.
pub use expression::*;

/// Nodes for literals in AST.
pub mod literal;

/// Reexport literal nodes publicly.
pub use literal::*;

/// Statement nodes in the AST.
pub mod statement;

/// Reexport statement nodes publicly.
pub use statement::*;

/// Type nodes in the AST.
pub mod types;

/// Reexport type nodes publicly.
pub use types::*;

/// Pattern matching AST nodes.
pub mod pattern;

/// Reexport pattern nodes publicly.
pub use pattern::*;
use serde::Serialize;

/// Check whether two AST node are structurally equal by
/// serializing each to JSON and then comparing the JSON.
pub fn ast_eq<T: Serialize>(a: &T, b: &T) -> bool {
    serde_json::to_string(a)
        .and_then(|fst| serde_json::to_string(b).map(|snd| snd == fst))
        .unwrap_or(false)
}
