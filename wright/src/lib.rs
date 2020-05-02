#![warn(missing_copy_implementations)]
#![warn(missing_docs)]

//! The Wright programming language crate.
//!

#[macro_use]
extern crate enum_iterator;

/// The wright interpreter module.
pub mod interpreter;

/// Wright grammar module.
pub mod grammar;

/// Wright backend module.
pub mod backend;
