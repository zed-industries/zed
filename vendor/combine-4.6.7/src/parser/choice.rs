//! Combinators which take one or more parsers and attempts to parse successfully with at least one
//! of them.

use crate::{
    error::{
        ParseError,
        ParseResult::{self, *},
        ResultExt, StreamError, Tracked,
    },
    parser::ParseMode,
    ErrorOffset, Parser, Stream, StreamOnce,
};

/// Takes a number of parsers and tries to apply them each in order.
/// Fails if all the parsers fails or if an applied parser fails after it has committed to its
/// parse.
///
/// ```
/// # #[macro_use]
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::{digit, letter, string};
/// # use combine::stream::easy::Error;
/// # fn main() {
/// let mut parser = choice!(
///     many1(digit()),
///     string("let").map(|s| s.to_string()),
///     many1(letter()));
/// assert_eq!(parser.parse("let"), Ok(("let".to_string(), "")));
/// assert_eq!(parser.parse("123abc"), Ok(("123".to_string(), "abc")));
/// assert!(parser.parse(":123").is_err());
/// # }
/// ```
#[macro_export]
macro_rules! choice {
    ($first : expr) => {
        $first
    };
    ($first : expr, $($rest : expr),+) => {
        $first.or(choice!($($rest),+))
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! parse_mode_choice {
    (Input) => {
        fn parse_partial(
            &mut self,
            input: &mut Input,
            state: &mut Self::PartialState,
        ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
            self.parse_mode_choice($crate::parser::PartialMode::default(), input, state)
        }

        fn parse_first(
            &mut self,
            input: &mut Input,
            state: &mut Self::PartialState,
        ) -> ParseResult<Self::Output, Input::Error> {
            self.parse_mode_choice($crate::parser::FirstMode, input, state)
        }
    };
}

/// `ChoiceParser` represents a parser which may parse one of several different choices depending
/// on the input.
///
/// This is an internal trait used to overload the `choice` function.
pub trait ChoiceParser<Input: Stream> {
    type Output;
    type PartialState: Default;

    fn parse_first(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>;

    fn parse_partial(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>;

    fn parse_mode_choice<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
        Self: Sized;

    fn add_error_choice(&mut self, error: &mut Tracked<<Input as StreamOnce>::Error>);
}

impl<'a, Input, P> ChoiceParser<Input> for &'a mut P
where
    Input: Stream,
    P: ?Sized + ChoiceParser<Input>,
{
    type Output = P::Output;
    type PartialState = P::PartialState;

    parse_mode_choice!(Input);
    #[inline]
    fn parse_mode_choice<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        if mode.is_first() {
            (**self).parse_first(input, state)
        } else {
            (**self).parse_partial(input, state)
        }
    }

    fn add_error_choice(&mut self, error: &mut Tracked<<Input as StreamOnce>::Error>) {
        (**self).add_error_choice(error)
    }
}

macro_rules! merge {
    ($head: ident) => {
        $head.error
    };
    ($head: ident $($tail: ident)+) => {
        $head.error.merge(merge!($($tail)+))
    };
}

macro_rules! do_choice {
    (
        $input: ident
        $before_position: ident
        $before: ident
        $partial_state: ident
        $state: ident
        ( )
        $($parser: ident $error: ident)+
    ) => { {
        let mut error = Tracked::from(merge!($($error)+));
        // If offset != 1 then the nested parser is a sequence of parsers where 1 or
        // more parsers returned `PeekOk` before the parser finally failed with
        // `PeekErr`. Since we lose the offsets of the nested parsers when we merge
        // the errors we must first extract the errors before we do the merge.
        // If the offset == 0 on the other hand (which should be the common case) then
        // we can delay the addition of the error since we know for certain that only
        // the first parser in the sequence were tried
        $(
            if $error.offset != ErrorOffset(1) {
                error.offset = $error.offset;
                $parser.add_error(&mut error);
                error.offset = ErrorOffset(0);
            }
        )+
        PeekErr(error)
    } };
    (
        $input: ident
        $before_position: ident
        $before: ident
        $partial_state: ident
        $state: ident
        ( $head: ident $($tail: ident)* )
        $($all: ident)*
    ) => { {
        let parser = $head;
        let mut state = $head::PartialState::default();
        match parser.parse_mode(crate::parser::FirstMode, $input, &mut state) {
            CommitOk(x) => CommitOk(x),
            PeekOk(x) => PeekOk(x),
            CommitErr(err) => {
                // If we get `CommitErr` but the input is the same this is a partial parse we
                // cannot commit to so leave the state as `Peek` to retry all the parsers
                // on the next call to  `parse_partial`
                if $input.position() != $before_position {
                    *$state = self::$partial_state::$head(state);
                }
                CommitErr(err)
            }
            PeekErr($head) => {
                ctry!($input.reset($before.clone()).committed());
                do_choice!(
                    $input
                    $before_position
                    $before
                    $partial_state
                    $state
                    ( $($tail)* )
                    $($all)*
                    parser
                    $head
                )
            }
        }
    } }
}

macro_rules! tuple_choice_parser {
    ($head: ident) => {
        tuple_choice_parser_inner!($head; $head);
    };
    ($head: ident $($id: ident)+) => {
        tuple_choice_parser_inner!($head; $head $($id)+);
        tuple_choice_parser!($($id)+);
    };
}

macro_rules! tuple_choice_parser_inner {
    ($partial_state: ident; $($id: ident)+) => {
        #[doc(hidden)]
        pub enum $partial_state<$($id),+> {
            Peek,
            $(
                $id($id),
            )+
        }

        impl<$($id),+> Default for self::$partial_state<$($id),+> {
            fn default() -> Self {
                self::$partial_state::Peek
            }
        }

        #[allow(non_snake_case)]
        impl<Input, Output $(,$id)+> ChoiceParser<Input> for ($($id,)+)
        where
            Input: Stream,
            $($id: Parser< Input, Output = Output>),+
        {

            type Output = Output;
            type PartialState = self::$partial_state<$($id::PartialState),+>;

            parse_mode_choice!(Input);
            #[inline]
            fn parse_mode_choice<Mode>(
                &mut self,
                mode: Mode,
                input: &mut Input,
                state: &mut Self::PartialState,
            ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
            where
                Mode: ParseMode,
            {
                let ($(ref mut $id,)+) = *self;
                let empty = match *state {
                    self::$partial_state::Peek => true,
                    _ => false,
                };
                if mode.is_first() || empty {
                    let before_position = input.position();
                    let before = input.checkpoint();
                    do_choice!(input before_position before $partial_state state ( $($id)+ ) )
                } else {
                    match *state {
                        self::$partial_state::Peek => unreachable!(),
                        $(
                            self::$partial_state::$id(_) => {
                                let result = match *state {
                                    self::$partial_state::$id(ref mut state) => {
                                        $id.parse_mode(mode, input, state)
                                    }
                                    _ => unreachable!()
                                };
                                if result.is_ok() {
                                    *state = self::$partial_state::Peek;
                                }
                                result
                            }
                        )+
                    }
                }
            }

            fn add_error_choice(
                &mut self,
                error: &mut Tracked<<Input as StreamOnce>::Error>
            ) {
                if error.offset != ErrorOffset(0) {
                    let ($(ref mut $id,)+) = *self;
                    // Reset the offset to 1 on every add so that we always (and only) takes the
                    // error of the first parser. If we don't do this the first parser will consume
                    // the offset to the detriment for all the other parsers.
                    $(
                        error.offset = ErrorOffset(1);
                        $id.add_error(error);
                    )+
                }
            }
        }
    }
}

tuple_choice_parser!(A B C D E F G H I J K L M N O P Q R S T U V X Y Z);

macro_rules! array_choice_parser {
    ($($t: tt)+) => {
        $(
        impl<Input, P> ChoiceParser<Input> for [P; $t]
        where
            Input: Stream,
            P: Parser<Input>,
        {

            type Output = P::Output;
            type PartialState = <[P] as ChoiceParser<Input>>::PartialState;

            parse_mode_choice!(Input);
            #[inline]
            fn parse_mode_choice<M>(
                &mut self,
                mode: M,
                input: &mut Input,
                state: &mut Self::PartialState,
            ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
            where
                M: ParseMode,
            {
                if mode.is_first() {
                    self[..].parse_first(input, state)
                } else {
                    self[..].parse_partial(input, state)
                }
            }
            fn add_error_choice(
                &mut self,
                error: &mut Tracked<<Input as StreamOnce>::Error>
            ) {
                self[..].add_error_choice(error)
            }
        }
        )+
    };
}

#[rustfmt::skip]
array_choice_parser!(
    0 1 2 3 4 5 6 7 8 9
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
);

#[derive(Copy, Clone)]
pub struct Choice<P>(P);

impl<Input, P> Parser<Input> for Choice<P>
where
    Input: Stream,
    P: ChoiceParser<Input>,
{
    type Output = P::Output;
    type PartialState = P::PartialState;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        self.0.parse_mode_choice(mode, input, state)
    }

    fn add_error(&mut self, error: &mut Tracked<<Input as StreamOnce>::Error>) {
        let before = error.offset.0;
        self.0.add_error_choice(error);
        error.offset.0 = before.saturating_sub(1);
    }
}

fn slice_parse_mode<Input, P, M>(
    self_: &mut [P],
    mode: M,
    input: &mut Input,
    state: &mut (usize, P::PartialState),
) -> ParseResult<P::Output, <Input as StreamOnce>::Error>
where
    P: Parser<Input>,
    Input: Stream,
    M: ParseMode,
{
    let mut prev_err = None;
    let mut last_parser_having_non_1_offset = 0;
    let before = input.checkpoint();

    let (ref mut index_state, ref mut child_state) = *state;
    if !mode.is_first() && *index_state != 0 {
        return self_[*index_state - 1]
            .parse_partial(input, child_state)
            .map(|x| {
                *index_state = 0;
                x
            });
    }

    for i in 0..self_.len() {
        ctry!(input.reset(before.clone()).committed());

        match self_[i].parse_mode(mode, input, child_state) {
            committed_err @ CommitErr(_) => {
                *index_state = i + 1;
                return committed_err;
            }
            PeekErr(err) => {
                prev_err = match prev_err {
                    None => Some(err),
                    Some(mut prev_err) => {
                        if prev_err.offset != ErrorOffset(1) {
                            // First add the errors of all the preceding parsers which did not
                            // have a sequence of parsers returning `PeekOk` before failing
                            // with `PeekErr`.
                            let offset = prev_err.offset;
                            for p in &mut self_[last_parser_having_non_1_offset..(i - 1)] {
                                prev_err.offset = ErrorOffset(1);
                                p.add_error(&mut prev_err);
                            }
                            // Then add the errors if the current parser
                            prev_err.offset = offset;
                            self_[i - 1].add_error(&mut prev_err);
                            last_parser_having_non_1_offset = i;
                        }
                        Some(Tracked {
                            error: prev_err.error.merge(err.error),
                            offset: err.offset,
                        })
                    }
                };
            }
            ok @ CommitOk(_) | ok @ PeekOk(_) => {
                *index_state = 0;
                return ok;
            }
        }
    }
    PeekErr(match prev_err {
        None => Input::Error::from_error(
            input.position(),
            StreamError::message_static_message("parser choice is empty"),
        )
        .into(),
        Some(mut prev_err) => {
            if prev_err.offset != ErrorOffset(1) {
                let offset = prev_err.offset;
                let len = self_.len();
                for p in &mut self_[last_parser_having_non_1_offset..(len - 1)] {
                    prev_err.offset = ErrorOffset(1);
                    p.add_error(&mut prev_err);
                }
                prev_err.offset = offset;
                self_.last_mut().unwrap().add_error(&mut prev_err);
                prev_err.offset = ErrorOffset(0);
            }
            prev_err
        }
    })
}

impl<Input, O, P> ChoiceParser<Input> for [P]
where
    Input: Stream,
    P: Parser<Input, Output = O>,
{
    type Output = O;
    type PartialState = (usize, P::PartialState);

    #[inline]
    fn parse_partial(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        slice_parse_mode(self, crate::parser::PartialMode::default(), input, state)
    }

    #[inline]
    fn parse_first(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        slice_parse_mode(self, crate::parser::FirstMode, input, state)
    }

    #[inline]
    fn parse_mode_choice<M>(
        &mut self,
        _mode: M,
        _input: &mut Input,
        _state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        unreachable!()
    }

    fn add_error_choice(&mut self, error: &mut Tracked<<Input as StreamOnce>::Error>) {
        if error.offset != ErrorOffset(0) {
            for p in self {
                error.offset = ErrorOffset(1);
                p.add_error(error);
            }
        }
    }
}

/// Takes a tuple, a slice or an array of parsers and tries to apply them each in order.
/// Fails if all the parsers fails or if an applied parser consumes input before failing.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::{digit, string};
/// # fn main() {
/// // `choice` is overloaded on tuples so that different types of parsers can be used
/// // (each parser must still have the same input and output types)
/// let mut parser = choice((
///     string("Apple").map(|s| s.to_string()),
///     many1(digit()),
///     string("Orange").map(|s| s.to_string()),
/// ));
/// assert_eq!(parser.parse("1234"), Ok(("1234".to_string(), "")));
/// assert_eq!(parser.parse("Orangexx"), Ok(("Orange".to_string(), "xx")));
/// assert!(parser.parse("Appl").is_err());
/// assert!(parser.parse("Pear").is_err());
///
/// // If arrays or slices are used then all parsers must have the same type
/// // (`string` in this case)
/// let mut parser2 = choice([string("one"), string("two"), string("three")]);
/// // Fails as the parser for "two" consumes the first 't' before failing
/// assert!(parser2.parse("three").is_err());
///
/// // Use 'attempt' to make failing parsers always act as if they have not committed any input
/// let mut parser3 = choice([attempt(string("one")), attempt(string("two")), attempt(string("three"))]);
/// assert_eq!(parser3.parse("three"), Ok(("three", "")));
/// # }
/// ```
pub fn choice<Input, P>(ps: P) -> Choice<P>
where
    Input: Stream,
    P: ChoiceParser<Input>,
{
    Choice(ps)
}

#[derive(Copy, Clone)]
pub struct Or<P1, P2>(Choice<(P1, P2)>);
impl<Input, O, P1, P2> Parser<Input> for Or<P1, P2>
where
    Input: Stream,
    P1: Parser<Input, Output = O>,
    P2: Parser<Input, Output = O>,
{
    type Output = O;
    type PartialState = <Choice<(P1, P2)> as Parser<Input>>::PartialState;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        self.0.parse_mode(mode, input, state)
    }

    #[inline]
    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        if errors.offset != ErrorOffset(0) {
            self.0.add_error(errors);
        }
    }
}

/// Equivalent to [`p1.or(p2)`].
///
/// If you are looking to chain 3 or more parsers using `or` you may consider using the
/// [`choice!`] macro instead, which can be clearer and may result in a faster parser.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::choice::or;
/// # use combine::parser::char::{digit, string};
/// # fn main() {
/// let mut parser = or(
///     string("let"),
///     or(digit().map(|_| "digit"), string("led")),
/// );
/// assert_eq!(parser.parse("let"), Ok(("let", "")));
/// assert_eq!(parser.parse("1"), Ok(("digit", "")));
/// assert!(parser.parse("led").is_err());
///
/// let mut parser2 = or(string("two"), string("three"));
/// // Fails as the parser for "two" consumes the first 't' before failing
/// assert!(parser2.parse("three").is_err());
///
/// // Use 'attempt' to make failing parsers always act as if they have not committed any input
/// let mut parser3 = or(attempt(string("two")), attempt(string("three")));
/// assert_eq!(parser3.parse("three"), Ok(("three", "")));
/// # }
/// ```
///
/// [`choice!`]: ../../macro.choice.html
/// [`p1.or(p2)`]: ../trait.Parser.html#method.or
pub fn or<Input, P1, P2>(p1: P1, p2: P2) -> Or<P1, P2>
where
    Input: Stream,
    P1: Parser<Input>,
    P2: Parser<Input, Output = P1::Output>,
{
    Or(choice((p1, p2)))
}

#[derive(Copy, Clone)]
pub struct Optional<P>(P);
impl<Input, P> Parser<Input> for Optional<P>
where
    Input: Stream,
    P: Parser<Input>,
{
    type Output = Option<P::Output>;
    type PartialState = P::PartialState;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        let before = input.checkpoint();
        match self.0.parse_mode(mode, input, state) {
            PeekOk(x) => PeekOk(Some(x)),
            CommitOk(x) => CommitOk(Some(x)),
            CommitErr(err) => CommitErr(err),
            PeekErr(_) => {
                ctry!(input.reset(before).committed());
                PeekOk(None)
            }
        }
    }

    forward_parser!(Input, add_error parser_count, 0);
}

/// Parses `parser` and outputs `Some(value)` if it succeeds, `None` if it fails without
/// consuming any input. Fails if `parser` fails after having committed some input.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::string;
/// # fn main() {
/// let mut parser = optional(string("hello"));
/// assert_eq!(parser.parse("hello"), Ok((Some("hello"), "")));
/// assert_eq!(parser.parse("world"), Ok((None, "world")));
/// assert!(parser.parse("heya").is_err());
/// # }
/// ```
pub fn optional<Input, P>(parser: P) -> Optional<P>
where
    Input: Stream,
    P: Parser<Input>,
{
    Optional(parser)
}

#[macro_export]
#[doc(hidden)]
macro_rules! parse_mode_dispatch {
    () => {
        fn parse_partial(
            &mut self,
            input: &mut Input,
            state: &mut Self::PartialState,
        ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
            self.parse_mode_dispatch($crate::parser::PartialMode::default(), input, state)
        }

        fn parse_first(
            &mut self,
            input: &mut Input,
            state: &mut Self::PartialState,
        ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
            self.parse_mode_dispatch($crate::parser::FirstMode, input, state)
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! dispatch_parser_impl {
    ($parser_name: ident [$first_ident: ident $($id: ident)*] [$($collected_idents: ident)*] $expr: expr, $($rest: expr,)*) => {
        $crate::dispatch_parser_impl!{ $parser_name [ $($id)* ] [$($collected_idents)* $first_ident] $($rest,)*}
    };
    ($parser_name: ident [$($id: ident)*] [$($collected_idents: ident)*]) => {
        $crate::dispatch_parser_impl!{ $parser_name; $($collected_idents)* }
    };

    ($parser_name: ident; $($id: ident)*) => {
        pub enum $parser_name<$($id),*> {
            $(
                $id($id),
            )*
        }

        #[allow(non_snake_case)]
        impl<Input, Output, $($id),*> $crate::Parser<Input> for $parser_name<$($id),*>
            where
                $( $id: $crate::Parser<Input, Output = Output>, )*
                Input: $crate::Stream,
        {
            type Output = Output;
            type PartialState = Option<$parser_name<$($id::PartialState),*>>;

            $crate::parse_mode!(Input);
            fn parse_mode<Mode>(
                &mut self,
                mode: Mode,
                input: &mut Input,
                state: &mut Self::PartialState,
            ) -> $crate::error::ParseResult<Self::Output, <Input as $crate::StreamOnce>::Error>
            where
                Mode: $crate::parser::ParseMode,
            {
                match self {
                    $(
                    $parser_name::$id($id) => {
                        let state = match state {
                            Some($parser_name::$id(s)) => s,
                            _ => {
                                *state = Some($parser_name::$id(Default::default()));
                                match state {
                                    Some($parser_name::$id(s)) => s,
                                    _ => unreachable!(),
                                }
                            }
                        };
                        $id.parse_mode(mode, input, state)
                    }
                    )*
                }
            }

            fn add_error(&mut self, error: &mut $crate::error::Tracked<<Input as $crate::StreamOnce>::Error>) {
                match self {
                    $(
                    $parser_name::$id($id) => $id.add_error(error),
                    )*
                }
            }
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! dispatch_inner {
    ($expr_ident: ident [$first_ident: ident $($id: ident)*] [$($collected: tt)*] $($pat: pat)|+ $(if $pred:expr)? => $expr: expr, $($rest_alt: tt)*) => {
        $crate::dispatch_inner!{ $expr_ident [ $($id)* ] [$($collected)* $first_ident $($pat)|+ $(if $pred)? => $expr,] $($rest_alt)*}
    };
    ($expr_ident: ident [$($id: ident)*] [$($collected: tt)*]) => {
        $crate::dispatch_inner!{ $expr_ident $($collected)* }
    };
    ($expr_ident: ident [$($ident_tt: tt)*]) => {
        unreachable!()
    };
    ($expr_ident: ident $( $ident: ident $($pat: pat)|+ $(if $pred:expr)? => $expr: expr,)+ ) => {
        match $expr_ident {
            $(
                $($pat)|+ $(if $pred)? => Dispatch::$ident(check_parser($expr)),
            )+
        }
    }
}

/// `dispatch!` allows a parser to be constructed depending on earlier input, without forcing each
/// branch to have the same type of parser
///
/// ```
/// use combine::{dispatch, any, token, satisfy, EasyParser, Parser};
///
/// let mut parser = any().then(|e| {
///     dispatch!(e;
///         'a' => token('a'),
///         'b' => satisfy(|b| b == 'b'),
///         t if t == 'c' => any(),
///         _ => token('d')
///     )
/// });
/// assert_eq!(parser.easy_parse("aa"), Ok(('a', "")));
/// assert_eq!(parser.easy_parse("cc"), Ok(('c', "")));
/// assert_eq!(parser.easy_parse("cd"), Ok(('d', "")));
/// assert!(parser.easy_parse("ab").is_err());
/// ```
#[macro_export]
macro_rules! dispatch {
    ($match_expr: expr; $( $($pat: pat)|+ $(if $pred:expr)? => $expr: expr ),+ $(,)? ) => {
        {
            $crate::dispatch_parser_impl!{ Dispatch [A B C D E F G H I J K L M N O P Q R S T U V X Y Z] [] $($expr,)+ }

            fn check_parser<Input, P>(p: P) -> P where P: $crate::Parser<Input>, Input: $crate::Stream { p }

            let e = $match_expr;
            let parser = $crate::dispatch_inner!(e [A B C D E F G H I J K L M N O P Q R S T U V X Y Z] []
                $(
                    $($pat)|+ $(if $pred)? => $expr,
                )*
            );
            parser
        }
    }
}

#[cfg(all(feature = "std", test))]
mod tests {

    use crate::parser::{token::any, EasyParser};

    use super::*;

    #[test]
    fn choice_single_parser() {
        assert!(choice((any(),),).easy_parse("a").is_ok());
    }
}
