use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::Underscore;
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::tracing::parsers::tag;
use crate::grammar::tracing::{parsers::map, trace_result};
use nom::IResult;

impl<T: Clone + std::fmt::Debug> Underscore<T> {
    /// The name of this parser as it appears in parse traces.
    pub const TRACE_NAME: &'static str = "Underscore";

    /// The constant for an underscore literal in source code. Unlikely to change.
    pub const UNDERSCORE: &'static str = "_";
}

impl<I: WrightInput> Underscore<I> {
    /// Parse an underscore from source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            map(tag(Self::UNDERSCORE), |source| Self { source })(
                input.trace_start_clone(Self::TRACE_NAME),
            ),
        )
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for Underscore<I> {
    #[inline]
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: Clone + std::fmt::Debug> AstEq for Underscore<I> {
    fn ast_eq(_: &Self, _: &Self) -> bool {
        true
    }
}
