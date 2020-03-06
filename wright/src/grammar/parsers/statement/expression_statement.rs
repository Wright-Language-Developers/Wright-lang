use crate::grammar::ast::{Expression, ExpressionSt};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::with_input;
use nom::character::complete::{char as ch, multispace0};
use nom::combinator::map;
use nom::sequence::{preceded, terminated};
use nom::IResult;

impl<'s> ExpressionSt<'s> {
    fn inner(frag: Fragment<'s>) -> IResult<Fragment<'s>, Expression<'s>> {
        terminated(Expression::parse, preceded(multispace0, ch(';')))(frag)
    }

    /// Matches an expression followed by any amount of whitespace, then a semicolon.
    pub fn parse(frag: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        map(with_input(Self::inner), |(parse, expr)| ExpressionSt {
            frag: parse,
            inner: expr,
        })(frag)
    }
}

impl<'s> HasFragment<'s> for ExpressionSt<'s> {
    fn get_fragment(&self) -> Fragment<'s> {
        self.frag
    }
}