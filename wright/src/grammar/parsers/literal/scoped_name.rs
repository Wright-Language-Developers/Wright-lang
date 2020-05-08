use crate::grammar::ast::{eq::AstEq, Expression, Identifier, ScopedName};
use crate::grammar::model::{Fragment, HasFragment};
use crate::grammar::parsers::whitespace::token_delimiter;
use crate::grammar::parsers::with_input;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::{many0};
use nom::sequence::{delimited, pair, terminated};
use nom::{IResult, Err};

impl<'s> ScopedName<'s> {
    /// The scope separator string.
    pub const SEPARATOR: &'static str = "::";

    /// The name this will appear under when tracing a parse.
    pub const TRACE_NAME: &'static str = "ScopedName";

    /// Parses a ScopedName from the given input fragment.
    pub fn parse(mut input: Fragment<'s>) -> IResult<Fragment<'s>, Self> {
        input.trace_start(Self::TRACE_NAME);
        let res = map(
            with_input(pair(
                many0(terminated(
                    Identifier::parse,
                    delimited(token_delimiter,
                        tag(Self::SEPARATOR),
                        token_delimiter
                    )
                )),
                Identifier::parse,
            )),
            |(frag, (path, name))| Self {
                frag,
                path,
                name,
            },
        )(input);

        res.map(|(mut rem, pr)| {
                rem.trace_end(Self::TRACE_NAME, true);
                (rem, pr)
            })
            .map_err(|e| {
                e.map_input(|mut f| {
                    f.trace_end(Self::TRACE_NAME, false);
                    f
                })
            })
    }
}

impl<'s> HasFragment<'s> for ScopedName<'s> {
    fn get_fragment_reference(&self) -> &Fragment<'s> {
        &self.frag
    }
}

impl<'s> Into<Expression<'s>> for ScopedName<'s> {
    fn into(self) -> Expression<'s> {
        Expression::ScopedName(self)
    }
}

impl<'s> AstEq for ScopedName<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        AstEq::ast_eq(&fst.path, &snd.path) && AstEq::ast_eq(&fst.name, &snd.name)
    }
}
