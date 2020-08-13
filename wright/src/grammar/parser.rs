#![allow(missing_docs)]

use pest_consume::{Error, Parser};

/// Wright parser generated using [Pest](https://pest.rs/).
#[derive(Parser, Copy, Clone, Debug)]
#[grammar = "grammar/grammar.pest"]
pub struct WrightParser;

/// Parser result.
pub type ParseResult<T> = Result<T, Error<Rule>>;

pub type Node<'i> = pest_consume::Node<'i, Rule, ()>;



#[pest_consume::parser]
impl WrightParser {

}
