use std::fmt::{Debug, Display};

use crate::*;


#[track_caller]
pub(crate) fn assert_parse_ok_eq<T: PartialEq + Debug + Display>(
    input: &str,
    result: Result<T, ParseError>,
    expected: T,
    parse_method: &str,
) {
    match result {
        Ok(actual) if actual == expected => {
            if actual.to_string() != input {
                panic!("formatting does not yield original input `{input}`: {actual:?}");
            }
        }
        Ok(actual) => {
            panic!("unexpected parsing result (with `{parse_method}`) for `{input}`:\n\
                actual:    {actual:?}\n\
                expected:  {expected:?}");
        }
        Err(e) => {
            panic!("expected `{input}` to be parsed (with `{parse_method}`) successfully, \
                but it failed: {e:?}");
        }
    }
}

// This is not ideal, but to perform this check we need `proc-macro2`. So we
// just don't do anything if that feature is not enabled.
#[cfg(not(feature = "proc-macro2"))]
pub(crate) fn assert_roundtrip<T>(_: T, _: &str) {}

#[cfg(feature = "proc-macro2")]
#[track_caller]
pub(crate) fn assert_roundtrip<T>(ours: T, input: &str)
where
    T: std::convert::TryFrom<proc_macro2::Literal> + fmt::Debug + PartialEq + Clone,
    proc_macro2::Literal: From<T>,
    <T as std::convert::TryFrom<proc_macro2::Literal>>::Error: std::fmt::Display,
{
    let pm_lit = input.parse::<proc_macro2::Literal>()
        .expect("failed to parse input as proc_macro2::Literal");
    let t_name = std::any::type_name::<T>();

    // Unfortunately, `proc_macro2::Literal` does not implement `PartialEq`, so
    // this is the next best thing.
    if proc_macro2::Literal::from(ours.clone()).to_string() != pm_lit.to_string() {
        panic!(
            "Converting {} to proc_macro2::Literal has unexpected result:\
                \nconverted: {:?}\nexpected:  {:?}",
            t_name,
            proc_macro2::Literal::from(ours),
            pm_lit,
        );
    }

    match T::try_from(pm_lit) {
        Err(e) => {
            panic!("Trying to convert proc_macro2::Literal to {t_name} results in error: {e}");
        }
        Ok(res) => {
            if res != ours {
                panic!("Converting proc_macro2::Literal to {t_name} has unexpected result:\n\
                    actual:    {res:?}\n\
                    expected:  {ours:?}");
            }
        }
    }
}

macro_rules! assert_err {
    ($ty:ident, $input:literal, $kind:ident, $( $span:tt )+ ) => {
        assert_err_single!($ty::parse($input), $kind, $($span)+);
        assert_err_single!($crate::Literal::parse($input), $kind, $($span)+);
    };
}

macro_rules! assert_err_single {
    ($expr:expr, $kind:ident, $( $span:tt )+ ) => {
        let res = $expr;
        let err = match res {
            Err(e) => e,
            Ok(v) => panic!(
                "Expected `{}` to return an error, but it returned Ok({:?})",
                stringify!($expr),
                v,
            ),
        };
        if err.kind != $crate::err::ParseErrorKind::$kind {
            panic!(
                "Expected error kind {} for `{}` but got {:?}",
                stringify!($kind),
                stringify!($expr),
                err.kind,
            )
        }
        let expected_span = assert_err_single!(@span $($span)+);
        if err.span != expected_span {
            panic!(
                "Expected error span {:?} for `{}` but got {:?}",
                expected_span,
                stringify!($expr),
                err.span,
            )
        }
    };
    (@span $start:literal .. $end:literal) => { Some($start .. $end) };
    (@span $at:literal) => { Some($at.. $at + 1) };
    (@span None) => { None };
}
