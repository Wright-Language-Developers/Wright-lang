/// Wright's parser implementation.
pub mod parsers;

/// Model for Wright's parser system.
pub mod model;
#[cfg(test)]
mod model_tests;

/// Model for Wright's Abstract Syntax Tree.
pub mod ast;

/// Utility functions for testing wright's parsing systems.
pub mod testing;

/// Parser prelude. Re-exports all underlying utility parsers.
pub mod prelude;