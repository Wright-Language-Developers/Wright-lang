use crate::grammar::ast::{BinaryExpression, BinaryOp, Expression};
use crate::grammar::model::Fragment;
use crate::grammar::parsers::expression::binary_expression::primary::logical::{
    logical_or, logical_or_primary,
};
use crate::grammar::parsers::whitespace::token_delimiter;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{map, value};
use nom::sequence::{delimited, tuple};
use nom::IResult;

/// Parse a child node in a range expression.
pub fn range_primary(input: Fragment) -> IResult<Fragment, Expression> {
    alt((logical_or, logical_or_primary))(input)
}

/// Parse a complete range expression in source code.
pub fn range_expr(input: Fragment) -> IResult<Fragment, Expression> {
    map(
        tuple((
            range_primary,
            delimited(
                token_delimiter,
                alt((
                    value(BinaryOp::Range, tag("..")),
                    value(BinaryOp::RangeInclusive, tag("..=")),
                )),
                token_delimiter,
            ),
            range_primary,
        )),
        |(l, op, r)| BinaryExpression::new_merge(l, op, r).into(),
    )(input)
}
