#![warn(missing_copy_implementations)]
#![warn(missing_docs)]

//! The Wright programming language crate.
//!

use codespan::{FileId, Files};
/// The wright interpreter module.
pub mod interpreter;

/// Wright grammar module.
pub mod grammar;

/// Wright backend module.
pub mod backend;
