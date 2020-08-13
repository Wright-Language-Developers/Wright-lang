
/// Model for Wright's parser system.
pub mod model;
#[cfg(test)]
mod model_tests;

/// Model for Wright's Abstract Syntax Tree.
pub mod ast;

#[derive(Parser)]
#[grammar = "grammar/grammar.pest"]
pub struct WrightGrammar;