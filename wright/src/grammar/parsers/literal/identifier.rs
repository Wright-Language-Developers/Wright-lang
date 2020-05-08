use crate::grammar::ast::{
    eq::AstEq, BinaryOp, Conditional, SelfLit, Underscore,
};
use crate::grammar::ast::{BooleanLit, Identifier};
use crate::grammar::model::{Fragment, HasFragment};
use nom::bytes::complete::take_while;
use nom::character::complete::anychar;
use nom::combinator::{map, recognize, verify};
use nom::error::context;
use nom::sequence::pair;
use nom::IResult;

impl<I> Identifier<I> {
    /// Reserved words that an identifier must not match.
    pub const RESERVED_WORDS: [&'static str; 8] = [
        BooleanLit::FALSE,
        BooleanLit::TRUE,
        SelfLit::SELF,
        Conditional::IF,
        Conditional::ELSE,
        Underscore::UNDERSCORE,
        BinaryOp::LOGICAL_AND,
        BinaryOp::LOGICAL_OR,
    ];

    fn new(source: I) -> Self {
        Self {  }
    }

    fn raw_ident(input: Fragment<'s>) -> IResult<Fragment<'s>, Fragment<'s>> {
        verify(
            recognize(pair(
                verify(anychar, |c| c.is_ascii_alphabetic() || *c == '_'),
                take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
            )),
            |fr: &Fragment<'s>| {
                Self::RESERVED_WORDS
                    .iter()
                    .all(|s: &&str| *s != fr.source())
            },
        )(input)
    }

    /// Parse an identifier from source code. Identifiers may include
    /// ASCII alphanumerics and underscores, but must not start with a number.
    /// An Identifier also must not be a reserved word.
    pub fn parse(input: I) -> IResult<Fragment<'s>, Self> {
        context("expected identifier", map(Self::raw_ident, Self::new))(input)
    }
}

impl<'s> HasFragment<'s> for Identifier<'s> {
    fn get_fragment_reference(&self) -> &Fragment<'s> {
        &self.source
    }
}

impl<'s> AstEq for Identifier<'s> {
    fn ast_eq(fst: &Self, snd: &Self) -> bool {
        fst.source.source() == snd.source.source()
    }
}
