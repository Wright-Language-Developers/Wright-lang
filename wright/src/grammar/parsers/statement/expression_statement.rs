use crate::grammar::ast::eq::AstEq;
use crate::grammar::ast::Statement;
use crate::grammar::ast::{Expression, ExpressionStatement};
use crate::grammar::model::{HasSourceReference, WrightInput};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use crate::grammar::tracing::parsers::map;
use crate::grammar::tracing::trace_result;
use nom::character::complete::char as ch;
use nom::sequence::{pair, terminated};
use nom::IResult;

impl<T: Clone + std::fmt::Debug> ExpressionStatement<T> {
    /// Name that appears in parse traces.
    pub const TRACE_NAME: &'static str = "ExpressionStatement";
}

impl<I: WrightInput> ExpressionStatement<I> {
    /// Parse an expression followed by a semicolon in source code.
    pub fn parse(input: I) -> IResult<I, Self> {
        trace_result(
            Self::TRACE_NAME,
            map(
                with_input(terminated(
                    Expression::parse,
                    pair(token_delimiter, ch(Statement::<I>::TERMINATOR)),
                )),
                move |(consumed, result)| Self {
                    source: consumed,
                    inner: Box::new(result),
                },
            )(input.trace_start_clone(Self::TRACE_NAME)),
        )
    }
}

impl<I: std::fmt::Debug + Clone> HasSourceReference<I> for ExpressionStatement<I> {
    #[inline]
    fn get_source_ref(&self) -> &I {
        &self.source
    }
}

impl<I: Clone + std::fmt::Debug + PartialEq> AstEq for ExpressionStatement<I> {
    #[inline]
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.inner, &snd.inner)
    }
}
