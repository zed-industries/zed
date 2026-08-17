/// Initialize a struct or tuple out of an unordered sequences of parsers
///
/// Unlike normal struct initialization syntax:
/// - `_` fields can exist to run a parser but ignore the result
/// - Parse results for a field can later be referenced using the field name
///
/// Unlike normal tuple initialization syntax:
/// - Struct-style initialization (`{ 0: _, 1: _}`) is not supported
/// - `_: <parser>` fields can exist to run a parser but ignore the result
///
/// To stop on an error, rather than trying further permutations, see
/// [`cut_err`][crate::combinator::cut_err] ([example][crate::_tutorial::chapter_7]).
///
/// # Example
///
/// ```rust
/// # #[cfg(feature = "ascii")] {
/// # use winnow::{error::ErrMode, error::Needed};
/// # use winnow::prelude::*;
/// use winnow::ascii::{alpha1, digit1};
/// use winnow::combinator::unordered_seq;
/// fn parser<'i>(input: &mut &'i str) -> ModalResult<(&'i str, &'i str)> {
///   unordered_seq!((alpha1, digit1)).parse_next(input)
/// }
///
/// // permutation takes alphabetic characters then digit
/// assert_eq!(parser.parse_peek("abc123"), Ok(("", ("abc", "123"))));
///
/// // but also in inverse order
/// assert_eq!(parser.parse_peek("123abc"), Ok(("", ("abc", "123"))));
///
/// // it will fail if one of the parsers failed
/// assert!(parser.parse_peek("abc;").is_err());
/// # }
/// ```
///
/// The parsers are applied greedily: if there are multiple unapplied parsers
/// that could parse the next slice of input, the first one is used.
/// ```rust
/// # #[cfg(feature = "parser")] {
/// # use winnow::error::ErrMode;
/// # use winnow::prelude::*;
/// use winnow::combinator::unordered_seq;
/// use winnow::token::any;
///
/// fn parser(input: &mut &str) -> ModalResult<(char, char)> {
///   unordered_seq!((any, 'a')).parse_next(input)
/// }
///
/// // any parses 'b', then char('a') parses 'a'
/// assert_eq!(parser.parse_peek("ba"), Ok(("", ('b', 'a'))));
///
/// // any parses 'a', then char('a') fails on 'b',
/// // even though char('a') followed by any would succeed
/// assert!(parser.parse_peek("ab").is_err());
/// # }
/// ```
#[macro_export]
#[doc(alias = "permutation")]
#[doc(hidden)] // forced to be visible in intended location
macro_rules! unordered_seq {
    ($($name: ident)::* { $($fields: tt)* }) => {
        $crate::combinator::trace(stringify!($($name)::*), move |input: &mut _| {
            $crate::unordered_seq_parse_struct_body!(
                ( $($name)::* );
                ( $($fields)* );
                ( _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20 );
                ( input );
            )
        })
    };
    (( $($fields: tt)* )) => {
        $crate::combinator::trace("tuple", move |input: &mut _| {
            $crate::unordered_seq_parse_tuple_body!(
                ( );
                ( $($fields)* );
                ( _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20 );
                ( input );
            )
        })
    };
    ($($name: ident)::* ( $($fields: tt)* )) => {
        $crate::combinator::trace(stringify!($($name)::*), move |input: &mut _| {
            $crate::unordered_seq_parse_tuple_body!(
                ( $($name)::* );
                ( $($fields)* );
                ( _0, _1, _2, _3, _4, _5, _6, _7, _8, _9, _10, _11, _12, _13, _14, _15, _16, _17, _18, _19, _20 );
                ( input );
            )
        })
    };
    ($($fields: tt)*) => {
        $crate::unordered_seq!((
            $($fields)*
        ))
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_struct_body {
    (
        ( $($name: ident)::* );
        ( $($fields: tt)* );
        ( $($unnamed: ident),* );
        ( $input: expr );
    ) => {{
        // Workaround a type inference issue by hinting to the compiler `I` within `ParserError` calls
        fn recover_err<I, E>(_input: &I, curr: E, acc: &mut Option<E>) -> Option<E>
        where
            I: $crate::stream::Stream,
            E: $crate::error::ParserError<I>,
        {
            if $crate::error::ParserError::is_backtrack(&curr) {
                *acc = Some(match acc.take() {
                    Some(err) => $crate::error::ParserError::or(err, curr),
                    None => curr,
                });
                None
            } else {
                Some(curr)
            }
        }

        $crate::unordered_seq_parse_struct_init!(
            ( $($fields)* );
            ( $($unnamed),* );
        );

        loop {
            let mut err = ::core::option::Option::None;
            let start = $crate::stream::Stream::checkpoint($input);

            $crate::unordered_seq_parse_struct_each!(
                ( $($fields)* );
                ( $($unnamed),* );
                ( $input );
                ( start );
                ( err );
            );

            // If we reach here, every iterator has either been applied before,
            // or errored on the remaining input
            if let ::core::option::Option::Some(err) = err {
                // There are remaining parsers, and all errored on the remaining input
                break Err($crate::error::ParserError::append(err, $input, &start));
            }

            // All parsers were applied
            break Ok(
                $crate::unordered_seq_parse_struct_collect!(
                    ( $($fields)* );
                    ( $($unnamed),* );
                    ( $($name)::* );
                )
            );
        }
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_struct_init {
    (
        ( _ : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
    ) => {
        let mut $unnamed1 = ::core::option::Option::None;
        $crate::unordered_seq_parse_struct_init!(
            ( $($fields)* );
            ( $($unnamed),* );
        );
    };
    (
        ( $named: ident : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
    ) => {
        let mut $named = ::core::option::Option::None;
        $crate::unordered_seq_parse_struct_init!(
            ( $($fields)* );
            ( $($unnamed),* );
        );
    };
    (
        ( _ : $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
    ) => {
        let mut $unnamed1 = ::core::option::Option::None;
    };
    (
        ( $named: ident : $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
    ) => {
        let mut $named = ::core::option::Option::None;
    };
    (
        ( .. $update: expr  $(,)? );
        ( $($unnamed: ident),* );
    ) => {
    };
    (
        ( $(,)? );
        ( $($unnamed: ident),* );
    ) => {
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_struct_collect {
    (
        ( $(,)? );
        ( $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $($name)::* { $($inits)* }
    };
    (
        ( .. $update: expr $(,)? );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
       $crate::unordered_seq_parse_struct_collect!(
            ( , );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)* .. $update
        )
    };
    (
        ( _ : $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_struct_collect!(
            ( , );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)*
        )
    };
    (
        ( _ : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_struct_collect!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)*
        )
    };
    (
        ( $named: ident : $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_struct_collect!(
            ( , );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)* $named: $named.unwrap(),
        )
    };
    (
        ( $named: ident : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_struct_collect!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)* $named: $named.unwrap(),
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_struct_each(
    (
        ( _ : $head_parser: expr $(,)? );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        $crate::unordered_seq_parse_next!(
            ( $head_parser );
            ( $unnamed1 );
            ( $input );
            ( $start );
            ( $err );
        );
    );
    (
        ( $named: ident : $head_parser: expr $(,)? );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        $crate::unordered_seq_parse_next!(
            ( $head_parser );
            ( $named );
            ( $input );
            ( $start );
            ( $err );
        );
    );
    (
        ( _ : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        $crate::unordered_seq_parse_next!(
            ( $head_parser );
            ( $unnamed1 );
            ( $input );
            ( $start );
            ( $err );
        );
        $crate::unordered_seq_parse_struct_each!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $input );
            ( $start );
            ( $err );
        );
    );
    (
        ( $named: ident : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        $crate::unordered_seq_parse_next!(
            ( $head_parser );
            ( $named );
            ( $input );
            ( $start );
            ( $err );
        );
        $crate::unordered_seq_parse_struct_each!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $input );
            ( $start );
            ( $err );
        );
    );
    (
        ( .. $update: expr $(,)? );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
    );
);

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_tuple_body {
    (
        ( $($name: ident)::* );
        ( $($fields: tt)* );
        ( $($unnamed: ident),* );
        ( $input: expr );
    ) => {{
        // Workaround a type inference issue by hinting to the compiler `I` within `ParserError` calls
        fn recover_err<I, E>(_input: &I, curr: E, acc: &mut Option<E>) -> Option<E>
        where
            I: $crate::stream::Stream,
            E: $crate::error::ParserError<I>,
        {
            if $crate::error::ParserError::is_backtrack(&curr) {
                *acc = Some(match acc.take() {
                    Some(err) => $crate::error::ParserError::or(err, curr),
                    None => curr,
                });
                None
            } else {
                Some(curr)
            }
        }

        $crate::unordered_seq_parse_tuple_init!(
            ( $($fields)* );
            ( $($unnamed),* );
        );

        loop {
            let mut err = ::core::option::Option::None;
            let start = $crate::stream::Stream::checkpoint($input);

            $crate::unordered_seq_parse_tuple_each!(
                ( $($fields)* );
                ( $($unnamed),* );
                ( $input );
                ( start );
                ( err );
            );

            // If we reach here, every iterator has either been applied before,
            // or errored on the remaining input
            if let ::core::option::Option::Some(err) = err {
                // There are remaining parsers, and all errored on the remaining input
                break Err($crate::error::ParserError::append(err, $input, &start));
            }

            // All parsers were applied
            break Ok(
                $crate::unordered_seq_parse_tuple_collect!(
                    ( $($fields)* );
                    ( $($unnamed),* );
                    ( $($name)::* );
                )
            );
        }
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_tuple_init {
    (
        ( $(_ :)? $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
    ) => {
        let mut $unnamed1 = ::core::option::Option::None;
        $crate::unordered_seq_parse_tuple_init!(
            ( $($fields)* );
            ( $($unnamed),* );
        );
    };
    (
        ( $(_ :)? $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
    ) => {
        let mut $unnamed1 = ::core::option::Option::None;
    };
    (
        ( $(,)? );
        ( $($unnamed: ident),* );
    ) => {
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_tuple_collect {
    (
        ( $(,)? );
        ( $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $($name)::* ( $($inits)* )
    };
    (
        ( _ : $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_tuple_collect!(
            ( , );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)*
        )
    };
    (
        ( _ : $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_tuple_collect!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)*
        )
    };
    (
        ( $head_parser: expr );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_tuple_collect!(
            ( , );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)* $unnamed1.unwrap(),
        )
    };
    (
        ( $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $($name: ident)::* );
        $($inits: tt)*
    ) => {
        $crate::unordered_seq_parse_tuple_collect!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $($name)::* );
            $($inits)* $unnamed1.unwrap(),
        )
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_tuple_each(
    (
        ( $(_ :)? $head_parser: expr $(,)? );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        $crate::unordered_seq_parse_next!(
            ( $head_parser );
            ( $unnamed1 );
            ( $input );
            ( $start );
            ( $err );
        );
    );
    (
        ( $(_ :)? $head_parser: expr, $($fields: tt)* );
        ( $unnamed1: ident, $($unnamed: ident),* );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        $crate::unordered_seq_parse_next!(
            ( $head_parser );
            ( $unnamed1 );
            ( $input );
            ( $start );
            ( $err );
        );
        $crate::unordered_seq_parse_tuple_each!(
            ( $($fields)* );
            ( $($unnamed),* );
            ( $input );
            ( $start );
            ( $err );
        );
    );
);

#[macro_export]
#[doc(hidden)]
macro_rules! unordered_seq_parse_next(
    (
        ( $field: tt );
        ( $unnamed: ident );
        ( $input: expr );
        ( $start: expr );
        ( $err: expr );
    ) => (
        if $unnamed.is_none() {
            $crate::stream::Stream::reset($input, &$start);

            match $crate::Parser::parse_next(&mut $field, $input) {
                Ok(o) => {
                    $unnamed = Some(o);
                    continue;
                }
                Err(e) => {
                    if let Some(cut) = recover_err($input, e, &mut $err) {
                        return Err(cut);
                    }
                }
            };
        }
    );
);
