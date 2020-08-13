
use codespan::{ByteIndex, ByteOffset, FileId, Files, Span};
use std::fmt::Debug;
use std::ops::{Range, RangeFrom, RangeFull, RangeTo};
use std::str::FromStr;

/// A piece of source code. Generally used to replace strings in the nom parser,
/// this structure stores extra information about the location of a fragment of
/// source code.
#[derive(Debug, Clone)]
pub struct Fragment<'source> {
    /// A reference to the parent Files object, which stores all source code
    /// being processed.
    files: &'source Files<String>,
    handle: FileId,
    span: Span,
    /// The fragment of source code represented by this object.
    source: &'source str,
}

/// An error when attempting to merge two fragments.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum FragmentError {
    /// Fragments are from different files, and cannot be merged.
    HandleMismatch,
    /// Fragments are not in the same `Files<String>` object, and cannot be merged.
    FilesRefMismatch,
}

impl<'s> Fragment<'s> {
    /// Construct a new parser input from a handle into a
    /// [Files](https://docs.rs/codespan/0.5.0/codespan/struct.Files.html)
    /// object.
    pub fn new(files: &'s Files<String>, handle: FileId) -> Self {
        let source = files.source(handle);
        let span = files.source_span(handle);
        Self {
            files,
            handle,
            span,
            source,
        }
    }

    /// Get the span associated with this fragment of source code.
    #[inline]
    pub fn get_span(&self) -> Span {
        self.span
    }

    /// Get the ending index of this fragment.
    /// Identical to `self.get_span().end()`.
    #[inline]
    pub fn end(&self) -> ByteIndex {
        self.span.end()
    }

    /// Get the starting index of this fragment.
    /// Identical to `self.get_span().start()`.
    #[inline]
    pub fn start(&self) -> ByteIndex {
        self.span.start()
    }

    /// Get the length of this fragment.
    /// Identical to `self.source.len()`.
    #[inline]
    pub fn len(&self) -> usize {
        self.source.len()
    }

    /// Get the source code of this fragment.
    #[inline]
    pub fn source(&self) -> &'s str {
        self.source
    }

    /// Get reference to files object.
    #[inline]
    pub fn files(&self) -> &'s Files<String> {
        self.files
    }

    /// Get the handle of this fragment's file in the parent
    /// [Files](https://docs.rs/codespan/0.5.0/codespan/struct.Files.html)
    /// object.
    #[inline]
    pub fn get_handle(&self) -> FileId {
        self.handle
    }
}