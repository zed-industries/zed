extern crate pest;

use std::io::{self, Write};

use pest::error::Error;
use pest::iterators::Pairs;
use pest::{state, ParseResult, Parser, ParserState};

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Rule {
    expr,
    paren,
    paren_end,
}

struct ParenParser;

impl Parser<Rule> for ParenParser {
    fn parse(rule: Rule, input: &str) -> Result<Pairs<Rule>, Error<Rule>> {
        fn expr(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.sequence(|s| s.repeat(paren).and_then(|s| s.end_of_input()))
        }

        fn paren(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::paren, |s| {
                s.sequence(|s| {
                    s.match_string("(")
                        .and_then(|s| {
                            s.optional(|s| {
                                s.sequence(|s| {
                                    s.lookahead(true, |s| s.match_string("("))
                                        .and_then(|s| s.repeat(paren))
                                })
                            })
                        })
                        .and_then(|s| s.rule(Rule::paren_end, |s| s.match_string(")")))
                })
            })
        }

        state(input, |state| match rule {
            Rule::expr => expr(state),
            Rule::paren => paren(state),
            _ => unreachable!(),
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Paren(Vec<Paren>);

fn expr(pairs: Pairs<Rule>) -> Vec<Paren> {
    pairs
        .filter(|p| p.as_rule() == Rule::paren)
        .map(|p| Paren(expr(p.into_inner())))
        .collect()
}

fn main() {
    loop {
        let mut line = String::new();

        print!("> ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut line).unwrap();
        line.pop();

        let parsed = ParenParser::parse(Rule::expr, &line);
        #[cfg(feature = "miette-error")]
        let parsed = parsed
            .map_err(Error::into_miette)
            .map_err(miette::Report::from);

        match parsed {
            Ok(pairs) => println!("{:?}", expr(pairs)),
            // To print pest errors, use Display formatting.
            #[cfg(not(feature = "miette-error"))]
            Err(e) => eprintln!("\n{}", e),
            // To print miette errors, use Debug formatting.
            #[cfg(feature = "miette-error")]
            Err(e) => eprintln!("\n{:?}", e),
        };
    }
}
