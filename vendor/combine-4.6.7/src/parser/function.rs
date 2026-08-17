//! Parsers constructor from regular functions

use crate::{
    error::{ParseResult, StdParseResult},
    lib::marker::PhantomData,
    stream::Stream,
    Parser,
};

impl<'a, Input: Stream, O> Parser<Input>
    for dyn FnMut(&mut Input) -> StdParseResult<O, Input> + 'a
{
    type Output = O;
    type PartialState = ();

    #[inline]
    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<O, Input::Error> {
        self(input).into()
    }
}

#[derive(Copy, Clone)]
pub struct FnParser<Input, F>(F, PhantomData<fn(Input) -> Input>);

/// Wraps a function, turning it into a parser.
///
/// Mainly needed to turn closures into parsers as function types can be casted to function pointers
/// to make them usable as a parser.
///
/// ```
/// extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # use combine::error::{Commit, StreamError};
/// # use combine::stream::easy;
/// # fn main() {
/// let mut even_digit = parser(|input| {
///     // Help type inference out
///     let _: &mut easy::Stream<&str> = input;
///     let position = input.position();
///     let (char_digit, committed) = digit().parse_stream(input).into_result()?;
///     let d = (char_digit as i32) - ('0' as i32);
///     if d % 2 == 0 {
///         Ok((d, committed))
///     }
///     else {
///         //Return an empty error since we only tested the first token of the stream
///         let errors = easy::Errors::new(
///             position,
///             StreamError::expected("even number")
///         );
///         Err(Commit::Peek(errors.into()))
///     }
/// });
/// let result = even_digit
///     .easy_parse("8")
///     .map(|x| x.0);
/// assert_eq!(result, Ok(8));
/// # }
/// ```
pub fn parser<Input, O, F>(f: F) -> FnParser<Input, F>
where
    Input: Stream,
    F: FnMut(&mut Input) -> StdParseResult<O, Input>,
{
    FnParser(f, PhantomData)
}

impl<Input, O, F> Parser<Input> for FnParser<Input, F>
where
    Input: Stream,
    F: FnMut(&mut Input) -> StdParseResult<O, Input>,
{
    type Output = O;
    type PartialState = ();

    #[inline]
    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<O, Input::Error> {
        (self.0)(input).into()
    }
}

impl<Input, O> Parser<Input> for fn(&mut Input) -> StdParseResult<O, Input>
where
    Input: Stream,
{
    type Output = O;
    type PartialState = ();

    #[inline]
    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<O, Input::Error> {
        self(input).into()
    }
}

#[derive(Copy)]
pub struct EnvParser<E, Input, T>
where
    Input: Stream,
{
    env: E,
    parser: fn(E, &mut Input) -> StdParseResult<T, Input>,
}

impl<E, Input, T> Clone for EnvParser<E, Input, T>
where
    Input: Stream,
    E: Clone,
{
    fn clone(&self) -> Self {
        EnvParser {
            env: self.env.clone(),
            parser: self.parser,
        }
    }
}

impl<Input, E, O> Parser<Input> for EnvParser<E, Input, O>
where
    E: Clone,
    Input: Stream,
{
    type Output = O;
    type PartialState = ();

    #[inline]
    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<O, Input::Error> {
        (self.parser)(self.env.clone(), input).into()
    }
}

/// Constructs a parser out of an environment and a function which needs the given environment to
/// do the parsing. This is commonly useful to allow multiple parsers to share some environment
/// while still allowing the parsers to be written in separate functions.
///
/// ```
/// # extern crate combine;
/// # use std::collections::HashMap;
/// # use combine::*;
/// # use combine::parser::function::env_parser;
/// # use combine::parser::char::letter;
/// # fn main() {
/// struct Interner(HashMap<String, u32>);
/// impl Interner {
///     fn string<Input>(&self, input: &mut Input) -> StdParseResult<u32, Input>
///         where Input: Stream<Token = char>,
///     {
///         many(letter())
///             .map(|s: String| self.0.get(&s).cloned().unwrap_or(0))
///             .parse_stream(input)
///             .into_result()
///     }
/// }
///
/// let mut map = HashMap::new();
/// map.insert("hello".into(), 1);
/// map.insert("test".into(), 2);
///
/// let env = Interner(map);
/// let mut parser = env_parser(&env, Interner::string);
///
/// let result = parser.parse("hello");
/// assert_eq!(result, Ok((1, "")));
///
/// let result = parser.parse("world");
/// assert_eq!(result, Ok((0, "")));
/// # }
/// ```
pub fn env_parser<E, Input, O>(
    env: E,
    parser: fn(E, &mut Input) -> StdParseResult<O, Input>,
) -> EnvParser<E, Input, O>
where
    E: Clone,
    Input: Stream,
{
    EnvParser { env, parser }
}
