/// Context held while evaluating an AST.
pub mod context;

/// AST evaluation module. Has trait(s) for evaluating AST nodes with
/// a given context.
pub mod ast_eval;

/// Values stored while evaluating an AST using the treewalker backend.
pub mod value;