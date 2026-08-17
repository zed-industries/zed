//-
// Copyright 2017, 2019 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::std_facade::fmt;

/// Easily define `proptest` tests.
///
/// Within `proptest!`, define one or more functions without return type
/// normally, except instead of putting `: type` after each parameter, write
/// `in strategy`, where `strategy` is an expression evaluating to some
/// `Strategy`.
///
/// Each function will be wrapped in a function which sets up a `TestRunner`,
/// and then invokes the function body with inputs generated according to the
/// strategies.
///
/// ### Example
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///   # /*
///   #[test]
///   # */
///   fn test_addition(a in 0..10, b in 0..10) {
///     prop_assert!(a + b <= 18);
///   }
///
///   # /*
///   #[test]
///   # */
///   fn test_string_concat(a in ".*", b in ".*") {
///     let cat = format!("{}{}", a, b);
///     prop_assert_eq!(a.len() + b.len(), cat.len());
///   }
/// }
/// #
/// # fn main() { test_addition(); test_string_concat(); }
/// ```
///
/// You can also use the normal argument syntax `pattern: type` as in:
///
/// ```rust
/// use proptest::prelude::*;
///
/// proptest! {
///   # /*
///   #[test]
///   # */
///   fn addition_is_commutative(a: u8, b: u8) {
///     prop_assert_eq!(a as u16 + b as u16, b as u16 + a as u16);
///   }
///
///   # /*
///   #[test]
///   # */
///   fn test_string_concat(a in ".*", b: String) {
///     let cat = format!("{}{}", a, b);
///     prop_assert_eq!(a.len() + b.len(), cat.len());
///   }
/// }
/// #
/// # fn main() { addition_is_commutative(); test_string_concat(); }
/// ```
///
/// As you can see, you can mix `pattern: type` and `pattern in expr`.
/// Due to limitations in `macro_rules!`, `pattern: type` does not work in
/// all circumstances. In such a case, use `(pattern): type` instead.
///
/// To override the default configuration, you can start the `proptest!` block
/// with `#![proptest_config(expr)]`, where `expr` is an expression that
/// evaluates to a `proptest::test_runner::Config` (or a reference to one).
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///   #![proptest_config(ProptestConfig {
///     cases: 99, .. ProptestConfig::default()
///   })]
///   # /*
///   #[test]
///   # */
///   fn test_addition(a in 0..10, b in 0..10) {
///     prop_assert!(a + b <= 18);
///   }
/// }
/// #
/// # fn main() { test_addition(); }
/// ```
///
/// ## Closure-Style Invocation
///
/// As of proptest 0.8.1, an alternative, "closure-style" invocation is
/// supported. In this form, `proptest!` is a function-like macro taking a
/// closure-esque argument. This makes it possible to run multiple tests that
/// require some expensive setup process. Note that the "fork" and "timeout"
/// features are _not_ supported in closure style.
///
/// To use a custom configuration, pass the `Config` object as a first
/// argument.
///
/// ### Example
///
/// ```
/// use proptest::prelude::*;
///
/// #[derive(Debug)]
/// struct BigStruct { /* Lots of fields ... */ }
///
/// fn very_expensive_function() -> BigStruct {
///   // Lots of code...
///   BigStruct { /* fields */ }
/// }
///
/// # /*
/// #[test]
/// # */
/// fn my_test() {
///   // We create just one `BigStruct`
///   let big_struct = very_expensive_function();
///
///   // But now can run multiple tests without needing to build it every time.
///   // Note the extra parentheses around the arguments are currently
///   // required.
///   proptest!(|(x in 0u32..42u32, y in 1000u32..100000u32)| {
///     // Test stuff
///   });
///
///   // `move` closures are also supported
///   proptest!(move |(x in 0u32..42u32)| {
///     // Test other stuff
///   });
///
///   // You can pass a custom configuration as the first argument
///   proptest!(ProptestConfig::with_cases(1000), |(x: i32)| {
///     // Test more stuff
///   });
/// }
/// #
/// # fn main() { my_test(); }
/// ```
#[macro_export]
macro_rules! proptest {
    (#![proptest_config($config:expr)]
     $(
        $(#[$meta:meta])*
       fn $test_name:ident($($parm:pat in $strategy:expr),+ $(,)?) $body:block
    )*) => {
        $(
            $(#[$meta])*
            fn $test_name() {
                let mut config = $crate::test_runner::contextualize_config($config.clone());
                config.test_name = ::core::option::Option::Some(
                    ::core::concat!(::core::module_path!(), "::", ::core::stringify!($test_name)));
                $crate::proptest_helper!(@_BODY config ($($parm in $strategy),+) [] $body);
            }
        )*
    };
    (#![proptest_config($config:expr)]
     $(
        $(#[$meta:meta])*
        fn $test_name:ident($($arg:tt)+) $body:block
    )*) => {
        $(
            $(#[$meta])*
            fn $test_name() {
                let mut config = $crate::test_runner::contextualize_config($config.clone());
                config.test_name = ::core::option::Option::Some(
                    ::core::concat!(::core::module_path!(), "::", ::core::stringify!($test_name)));
                $crate::proptest_helper!(@_BODY2 config ($($arg)+) [] $body);
            }
        )*
    };

    ($(
        $(#[$meta:meta])*
        fn $test_name:ident($($parm:pat in $strategy:expr),+ $(,)?) $body:block
    )*) => { $crate::proptest! {
        #![proptest_config($crate::test_runner::Config::default())]
        $($(#[$meta])*
          fn $test_name($($parm in $strategy),+) $body)*
    } };

    ($(
        $(#[$meta:meta])*
        fn $test_name:ident($($arg:tt)+) $body:block
    )*) => { $crate::proptest! {
        #![proptest_config($crate::test_runner::Config::default())]
        $($(#[$meta])*
          fn $test_name($($arg)+) $body)*
    } };

    (|($($parm:pat in $strategy:expr),+ $(,)?)| $body:expr) => {
        $crate::proptest!(
            $crate::test_runner::Config::default(),
            |($($parm in $strategy),+)| $body)
    };

    (move |($($parm:pat in $strategy:expr),+ $(,)?)| $body:expr) => {
        $crate::proptest!(
            $crate::test_runner::Config::default(),
            move |($($parm in $strategy),+)| $body)
    };

    (|($($arg:tt)+)| $body:expr) => {
        $crate::proptest!(
            $crate::test_runner::Config::default(),
            |($($arg)+)| $body)
    };

    (move |($($arg:tt)+)| $body:expr) => {
        $crate::proptest!(
            $crate::test_runner::Config::default(),
            move |($($arg)+)| $body)
    };

    ($config:expr, |($($parm:pat in $strategy:expr),+ $(,)?)| $body:expr) => { {
        let mut config = $crate::test_runner::contextualize_config($config.__sugar_to_owned());
        $crate::sugar::force_no_fork(&mut config);
        $crate::proptest_helper!(@_BODY config ($($parm in $strategy),+) [] $body)
    } };

    ($config:expr, move |($($parm:pat in $strategy:expr),+ $(,)?)| $body:expr) => { {
        let mut config = $crate::test_runner::contextualize_config($config.__sugar_to_owned());
        $crate::sugar::force_no_fork(&mut config);
        $crate::proptest_helper!(@_BODY config ($($parm in $strategy),+) [move] $body)
    } };

    ($config:expr, |($($arg:tt)+)| $body:expr) => { {
        let mut config = $crate::test_runner::contextualize_config($config.__sugar_to_owned());
        $crate::sugar::force_no_fork(&mut config);
        $crate::proptest_helper!(@_BODY2 config ($($arg)+) [] $body);
    } };

    ($config:expr, move |($($arg:tt)+)| $body:expr) => { {
        let mut config = $crate::test_runner::contextualize_config($config.__sugar_to_owned());
        $crate::sugar::force_no_fork(&mut config);
        $crate::proptest_helper!(@_BODY2 config ($($arg)+) [move] $body);
    } };
}

/// Rejects the test input if assumptions are not met.
///
/// Used directly within a function defined with `proptest!` or in any function
/// returning `Result<_, TestCaseError>`.
///
/// This is invoked as `prop_assume!(condition, format, args...)`. `condition`
/// is evaluated; if it is false, `Err(TestCaseError::Reject)` is returned. The
/// message includes the point of invocation and the format message. `format`
/// and `args` may be omitted to simply use the condition itself as the
/// message.
#[macro_export]
macro_rules! prop_assume {
    ($expr:expr) => {
        $crate::prop_assume!($expr, "{}", ::core::stringify!($expr))
    };

    ($expr:expr, $fmt:tt $(, $fmt_arg:expr),* $(,)?) => {
        if !$expr {
            extern crate alloc;
            return ::core::result::Result::Err(
                $crate::test_runner::TestCaseError::reject(
                    alloc::format!(::core::concat!("{}:{}:{}: ", $fmt),
                            ::core::file!(), ::core::line!(), ::core::column!()
                            $(, $fmt_arg)*)));
        }
    };
}

/// Produce a strategy which picks one of the listed choices.
///
/// This is conceptually equivalent to calling `prop_union` on the first two
/// elements and then chaining `.or()` onto the rest after implicitly boxing
/// all of them. As with `Union`, values shrink across elements on the
/// assumption that earlier ones are "simpler", so they should be listed in
/// order of ascending complexity when possible.
///
/// The macro invocation has two forms. The first is to simply list the
/// strategies separated by commas; this will cause value generation to pick
/// from the strategies uniformly. The other form is to provide a weight in the
/// form of a `u32` before each strategy, separated from the strategy with
/// `=>`.
///
/// Note that the exact type returned by the macro varies depending on how many
/// inputs there are. In particular, if given exactly one option, it will
/// return it unmodified. It is not recommended to depend on the particular
/// type produced by this macro.
///
/// ## Example
///
/// ```rust,no_run
/// use proptest::prelude::*;
///
/// #[derive(Clone, Copy, Debug)]
/// enum MyEnum {
///   Big(u64),
///   Medium(u32),
///   Little(i16),
/// }
///
/// # #[allow(unused_variables)]
/// # fn main() {
/// let my_enum_strategy = prop_oneof![
///   prop::num::i16::ANY.prop_map(MyEnum::Little),
///   prop::num::u32::ANY.prop_map(MyEnum::Medium),
///   prop::num::u64::ANY.prop_map(MyEnum::Big),
/// ];
///
/// let my_weighted_strategy = prop_oneof![
///   1 => prop::num::i16::ANY.prop_map(MyEnum::Little),
///   // Chose `Medium` twice as frequently as either `Little` or `Big`; i.e.,
///   // around 50% of values will be `Medium`, and 25% for each of `Little`
///   // and `Big`.
///   2 => prop::num::u32::ANY.prop_map(MyEnum::Medium),
///   1 => prop::num::u64::ANY.prop_map(MyEnum::Big),
/// ];
/// # }
/// ```
#[macro_export]
macro_rules! prop_oneof {
    ($($item:expr),+ $(,)?) => {
        $crate::prop_oneof![
            $(1 => $item),*
        ]
    };

    ($_weight0:expr => $item0:expr $(,)?) => { $item0 };

    // NOTE: The clippy::arc_with_non_send_sync lint is disabled here because
    // the strategies passed into prop_oneof! are often not Send or Sync, such
    // as BoxedStrategy.
    //
    // The double-curly-braces are not strictly required, but allow the expression
    // to be annotated with an attribute.

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr,
     $weight4:expr => $item4:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3)),
             ($weight4, $crate::std_facade::Arc::new($item4))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr,
     $weight4:expr => $item4:expr,
     $weight5:expr => $item5:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3)),
             ($weight4, $crate::std_facade::Arc::new($item4)),
             ($weight5, $crate::std_facade::Arc::new($item5))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr,
     $weight4:expr => $item4:expr,
     $weight5:expr => $item5:expr,
     $weight6:expr => $item6:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3)),
             ($weight4, $crate::std_facade::Arc::new($item4)),
             ($weight5, $crate::std_facade::Arc::new($item5)),
             ($weight6, $crate::std_facade::Arc::new($item6))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr,
     $weight4:expr => $item4:expr,
     $weight5:expr => $item5:expr,
     $weight6:expr => $item6:expr,
     $weight7:expr => $item7:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3)),
             ($weight4, $crate::std_facade::Arc::new($item4)),
             ($weight5, $crate::std_facade::Arc::new($item5)),
             ($weight6, $crate::std_facade::Arc::new($item6)),
             ($weight7, $crate::std_facade::Arc::new($item7))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr,
     $weight4:expr => $item4:expr,
     $weight5:expr => $item5:expr,
     $weight6:expr => $item6:expr,
     $weight7:expr => $item7:expr,
     $weight8:expr => $item8:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3)),
             ($weight4, $crate::std_facade::Arc::new($item4)),
             ($weight5, $crate::std_facade::Arc::new($item5)),
             ($weight6, $crate::std_facade::Arc::new($item6)),
             ($weight7, $crate::std_facade::Arc::new($item7)),
             ($weight8, $crate::std_facade::Arc::new($item8))))
    }};

    ($weight0:expr => $item0:expr,
     $weight1:expr => $item1:expr,
     $weight2:expr => $item2:expr,
     $weight3:expr => $item3:expr,
     $weight4:expr => $item4:expr,
     $weight5:expr => $item5:expr,
     $weight6:expr => $item6:expr,
     $weight7:expr => $item7:expr,
     $weight8:expr => $item8:expr,
     $weight9:expr => $item9:expr $(,)?) => {{
        #[allow(clippy::arc_with_non_send_sync)]
        $crate::strategy::TupleUnion::new(
            (($weight0, $crate::std_facade::Arc::new($item0)),
             ($weight1, $crate::std_facade::Arc::new($item1)),
             ($weight2, $crate::std_facade::Arc::new($item2)),
             ($weight3, $crate::std_facade::Arc::new($item3)),
             ($weight4, $crate::std_facade::Arc::new($item4)),
             ($weight5, $crate::std_facade::Arc::new($item5)),
             ($weight6, $crate::std_facade::Arc::new($item6)),
             ($weight7, $crate::std_facade::Arc::new($item7)),
             ($weight8, $crate::std_facade::Arc::new($item8)),
             ($weight9, $crate::std_facade::Arc::new($item9))))
    }};

    ($($weight:expr => $item:expr),+ $(,)?) => {
        $crate::strategy::Union::new_weighted(vec![
            $(($weight, $crate::strategy::Strategy::boxed($item))),*
        ])
    };
}

/// Convenience to define functions which produce new strategies.
///
/// The macro has two general forms. In the first, you define a function with
/// two argument lists. The first argument list uses the usual syntax and
/// becomes exactly the argument list of the defined function. The second
/// argument list uses the `in strategy` syntax as with `proptest!`, and is
/// used to generate the other inputs for the function. The second argument
/// list has access to all arguments in the first. The return type indicates
/// the type of value being generated; the final return type of the function is
/// `impl Strategy<Value = $type>`.
///
/// ```rust,no_run
/// # #![allow(dead_code)]
/// use proptest::prelude::*;
///
/// #[derive(Clone, Debug)]
/// struct MyStruct {
///   integer: u32,
///   string: String,
/// }
///
/// prop_compose! {
///   fn my_struct_strategy(max_integer: u32)
///                        (integer in 0..max_integer, string in ".*")
///                        -> MyStruct {
///     MyStruct { integer, string }
///   }
/// }
/// #
/// # fn main() { }
/// ```
///
/// This form is simply sugar around making a tuple and then calling `prop_map`
/// on it. You can also use `arg: type` as in `proptest! { .. }`:
///
/// ```rust,no_run
/// # #![allow(dead_code)]
/// # use proptest::prelude::*;
/// #
/// # #[derive(Clone, Debug)]
/// # struct MyStruct {
/// #  integer: u32,
/// #  string: String,
/// # }
///
/// prop_compose! {
///   fn my_struct_strategy(max_integer: u32)
///                        (integer in 0..max_integer, string: String)
///                        -> MyStruct {
///     MyStruct { integer, string }
///   }
/// }
/// #
/// # fn main() { }
/// ```
///
/// The second form is mostly the same, except that it takes _three_ argument
/// lists. The third argument list can see all values in both prior, which
/// permits producing strategies based on other strategies.
///
/// ```rust,no_run
/// # #![allow(dead_code)]
/// use proptest::prelude::*;
///
/// prop_compose! {
///   fn nearby_numbers()(centre in -1000..1000)
///                    (a in centre-10..centre+10,
///                     b in centre-10..centre+10)
///                    -> (i32, i32) {
///     (a, b)
///   }
/// }
/// #
/// # fn main() { }
/// ```
///
/// However, the body of the function does _not_ have access to the second
/// argument list. If the body needs access to those values, they must be
/// passed through explicitly.
///
/// ```rust,no_run
/// # #![allow(dead_code)]
/// use proptest::prelude::*;
///
/// prop_compose! {
///   fn vec_and_index
///     (max_length: usize)
///     (vec in prop::collection::vec(1..10, 1..max_length))
///     (index in 0..vec.len(), vec in Just(vec))
///     -> (Vec<i32>, usize)
///   {
///     (vec, index)
///   }
/// }
/// # fn main() { }
/// ```
///
/// The second form is sugar around making a strategy tuple, calling
/// `prop_flat_map()`, then `prop_map()`.
///
/// To give the function any modifier which isn't a visibility modifier, put it
/// in brackets before the `fn` token but after any visibility modifier.
///
/// ```rust,no_run
/// # #![allow(dead_code)]
/// use proptest::prelude::*;
///
/// prop_compose! {
///   pub(crate) [unsafe] fn pointer()(v in prop::num::usize::ANY)
///                                 -> *const () {
///     v as *const ()
///   }
/// }
/// # fn main() { }
/// ```
///
/// ## Comparison with Hypothesis' `@composite`
///
/// `prop_compose!` makes it easy to do a lot of things you can do with
/// [Hypothesis' `@composite`](https://hypothesis.readthedocs.io/en/latest/data.html#composite-strategies),
/// but not everything.
///
/// - You can't filter via this macro. For filtering, you need to make the
/// strategy the "normal" way and use `prop_filter()`.
///
/// - More than two layers of strategies or arbitrary logic between the two
/// layers. If you need either of these, you can achieve them by calling
/// `prop_flat_map()` by hand.
#[macro_export]
macro_rules! prop_compose {
    ($(#[$meta:meta])*
     $vis:vis
     $([$($modi:tt)*])? fn $name:ident $params:tt
     ($($var:pat in $strategy:expr),+ $(,)?)
       -> $return_type:ty $body:block) =>
    {
        #[must_use = "strategies do nothing unless used"]
        $(#[$meta])*
        $vis
        $($($modi)*)? fn $name $params
                 -> impl $crate::strategy::Strategy<Value = $return_type> {
            let strat = $crate::proptest_helper!(@_WRAP ($($strategy)*));
            $crate::strategy::Strategy::prop_map(strat,
                move |$crate::proptest_helper!(@_WRAPPAT ($($var),*))| $body)
        }
    };

    ($(#[$meta:meta])*
     $vis:vis
     $([$($modi:tt)*])? fn $name:ident $params:tt
     ($($var:pat in $strategy:expr),+ $(,)?)
     ($($var2:pat in $strategy2:expr),+ $(,)?)
       -> $return_type:ty $body:block) =>
    {
        #[must_use = "strategies do nothing unless used"]
        $(#[$meta])*
        $vis
        $($($modi)*)? fn $name $params
                 -> impl $crate::strategy::Strategy<Value = $return_type> {
            let strat = $crate::proptest_helper!(@_WRAP ($($strategy)*));
            let strat = $crate::strategy::Strategy::prop_flat_map(
                strat,
                move |$crate::proptest_helper!(@_WRAPPAT ($($var),*))|
                $crate::proptest_helper!(@_WRAP ($($strategy2)*)));
            $crate::strategy::Strategy::prop_map(strat,
                move |$crate::proptest_helper!(@_WRAPPAT ($($var2),*))| $body)
        }
    };

    ($(#[$meta:meta])*
     $vis:vis
     $([$($modi:tt)*])? fn $name:ident $params:tt
     ($($arg:tt)+)
       -> $return_type:ty $body:block) =>
    {
        #[must_use = "strategies do nothing unless used"]
        $(#[$meta])*
        $vis
        $($($modi)*)? fn $name $params
                 -> impl $crate::strategy::Strategy<Value = $return_type> {
            let strat = $crate::proptest_helper!(@_EXT _STRAT ($($arg)+));
            $crate::strategy::Strategy::prop_map(strat,
                move |$crate::proptest_helper!(@_EXT _PAT ($($arg)+))| $body)
        }
    };

    ($(#[$meta:meta])*
     $vis:vis
     $([$($modi:tt)*])? fn $name:ident $params:tt
     ($($arg:tt)+ $(,)?)
     ($($arg2:tt)+ $(,)?)
       -> $return_type:ty $body:block) =>
    {
        #[must_use = "strategies do nothing unless used"]
        $(#[$meta])*
        $vis
        $($($modi)*)? fn $name $params
                 -> impl $crate::strategy::Strategy<Value = $return_type> {
            let strat = $crate::proptest_helper!(@_WRAP ($($strategy)*));
            let strat = $crate::strategy::Strategy::prop_flat_map(
                strat,
                move |$crate::proptest_helper!(@_EXT _PAT ($($arg)+))|
                $crate::proptest_helper!(@_EXT _STRAT ($($arg2)*)));
            $crate::strategy::Strategy::prop_map(strat,
                move |$crate::proptest_helper!(@_EXT _PAT ($($arg2)*))| $body)
        }
    };
}

/// Similar to `assert!` from std, but returns a test failure instead of
/// panicking if the condition fails.
///
/// This can be used in any function that returns a `Result<_, TestCaseError>`,
/// including the top-level function inside `proptest!`.
///
/// Both panicking via `assert!` and returning a test case failure have the
/// same effect as far as proptest is concerned; however, the Rust runtime
/// implicitly prints every panic to stderr by default (including a backtrace
/// if enabled), which can make test failures unnecessarily noisy. By using
/// `prop_assert!` instead, the only output on a failing test case is the final
/// panic including the minimal test case.
///
/// ## Example
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///   # /*
///   #[test]
///   # */
///   fn triangle_inequality(a in 0.0f64..10.0, b in 0.0f64..10.0) {
///     // Called with just a condition will print the condition on failure
///     prop_assert!((a*a + b*b).sqrt() <= a + b);
///     // You can also provide a custom failure message
///     prop_assert!((a*a + b*b).sqrt() <= a + b,
///                  "Triangle inequality didn't hold for ({}, {})", a, b);
///     // If calling another function that can return failure, don't forget
///     // the `?` to propagate the failure.
///     assert_from_other_function(a, b)?;
///   }
/// }
///
/// // The macro can be used from another function provided it has a compatible
/// // return type.
/// fn assert_from_other_function(a: f64, b: f64) -> Result<(), TestCaseError> {
///   prop_assert!((a*a + b*b).sqrt() <= a + b);
///   Ok(())
/// }
/// #
/// # fn main() { triangle_inequality(); }
/// ```
#[macro_export]
macro_rules! prop_assert {
    ($cond:expr $(,) ?) => {
        $crate::prop_assert!($cond, ::core::concat!("assertion failed: ", ::core::stringify!($cond)))
    };

    ($cond:expr, $($fmt:tt)*) => {
        if !$cond {
            extern crate alloc;
            let message = alloc::format!($($fmt)*);
            let message = alloc::format!("{} at {}:{}", message, ::core::file!(), ::core::line!());
            return ::core::result::Result::Err(
                $crate::test_runner::TestCaseError::fail(message));
        }
    };
}

/// Similar to `assert_eq!` from std, but returns a test failure instead of
/// panicking if the condition fails.
///
/// See `prop_assert!` for a more in-depth discussion.
///
/// ## Example
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///   # /*
///   #[test]
///   # */
///   fn concat_string_length(ref a in ".*", ref b in ".*") {
///     let cat = format!("{}{}", a, b);
///     // Use with default message
///     prop_assert_eq!(a.len() + b.len(), cat.len());
///     // Can also provide custom message (added after the normal
///     // assertion message)
///     prop_assert_eq!(a.len() + b.len(), cat.len(),
///                     "a = {:?}, b = {:?}", a, b);
///   }
/// }
/// #
/// # fn main() { concat_string_length(); }
/// ```
#[macro_export]
macro_rules! prop_assert_eq {
    ($left:expr, $right:expr $(,) ?) => {{
        let left = $left;
        let right = $right;
        $crate::prop_assert!(
            left == right,
            "assertion failed: `(left == right)` \
             \n  left: `{:?}`,\n right: `{:?}`",
            left, right);
    }};

    ($left:expr, $right:expr, $fmt:tt $($args:tt)*) => {{
        let left = $left;
        let right = $right;
        $crate::prop_assert!(
            left == right,
            concat!(
                "assertion failed: `(left == right)` \
                 \n  left: `{:?}`, \n right: `{:?}`: ", $fmt),
            left, right $($args)*);
    }};
}

/// Similar to `assert_ne!` from std, but returns a test failure instead of
/// panicking if the condition fails.
///
/// See `prop_assert!` for a more in-depth discussion.
///
/// ## Example
///
/// ```
/// use proptest::prelude::*;
///
/// proptest! {
///   # /*
///   #[test]
///   # */
///   fn test_addition(a in 0i32..100i32, b in 1i32..100i32) {
///     // Use with default message
///     prop_assert_ne!(a, a + b);
///     // Can also provide custom message added after the common message
///     prop_assert_ne!(a, a + b, "a = {}, b = {}", a, b);
///   }
/// }
/// #
/// # fn main() { test_addition(); }
/// ```
#[macro_export]
macro_rules! prop_assert_ne {
    ($left:expr, $right:expr $(,) ?) => {{
        let left = $left;
        let right = $right;
        $crate::prop_assert!(
            left != right,
            "assertion failed: `(left != right)`\
             \n  left: `{:?}`,\n right: `{:?}`",
            left, right);
    }};

    ($left:expr, $right:expr, $fmt:tt $($args:tt)*) => {{
        let left = $left;
        let right = $right;
        $crate::prop_assert!(left != right, concat!(
                "assertion failed: `(left != right)`\
                 \n  left: `{:?}`,\n right: `{:?}`: ", $fmt),
            left, right $($args)*);
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! proptest_helper {
    (@_WRAP ($a:tt)) => { $a };
    (@_WRAP ($a0:tt $a1:tt)) => { ($a0, $a1) };
    (@_WRAP ($a0:tt $a1:tt $a2:tt)) => { ($a0, $a1, $a2) };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt)) => { ($a0, $a1, $a2, $a3) };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt $a4:tt)) => {
        ($a0, $a1, $a2, $a3, $a4)
    };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt $a4:tt $a5:tt)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5)
    };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt $a4:tt $a5:tt $a6:tt)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6)
    };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt
             $a4:tt $a5:tt $a6:tt $a7:tt)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt $a4:tt
             $a5:tt $a6:tt $a7:tt $a8:tt)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8)
    };
    (@_WRAP ($a0:tt $a1:tt $a2:tt $a3:tt $a4:tt
             $a5:tt $a6:tt $a7:tt $a8:tt $a9:tt)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9)
    };
    (@_WRAP ($a:tt $($rest:tt)*)) => {
        ($a, $crate::proptest_helper!(@_WRAP ($($rest)*)))
    };
    (@_WRAPPAT ($item:pat)) => { $item };
    (@_WRAPPAT ($a0:pat, $a1:pat)) => { ($a0, $a1) };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat)) => { ($a0, $a1, $a2) };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat)) => {
        ($a0, $a1, $a2, $a3)
    };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat)) => {
        ($a0, $a1, $a2, $a3, $a4)
    };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat, $a5:pat)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5)
    };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat,
                $a4:pat, $a5:pat, $a6:pat)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6)
    };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat,
                $a4:pat, $a5:pat, $a6:pat, $a7:pat)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7)
    };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat,
                $a5:pat, $a6:pat, $a7:pat, $a8:pat)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8)
    };
    (@_WRAPPAT ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat,
                $a5:pat, $a6:pat, $a7:pat, $a8:pat, $a9:pat)) => {
        ($a0, $a1, $a2, $a3, $a4, $a5, $a6, $a7, $a8, $a9)
    };
    (@_WRAPPAT ($a:pat, $($rest:pat),*)) => {
        ($a, $crate::proptest_helper!(@_WRAPPAT ($($rest),*)))
    };
    (@_WRAPSTR ($item:pat)) => { ::core::stringify!($item) };
    (@_WRAPSTR ($a0:pat, $a1:pat)) => { (::core::stringify!($a0), ::core::stringify!($a1)) };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2), ::core::stringify!($a3))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2),
         ::core::stringify!($a3), ::core::stringify!($a4))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat, $a5:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2), ::core::stringify!($a3),
         ::core::stringify!($a4), ::core::stringify!($a5))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat,
                $a4:pat, $a5:pat, $a6:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2), ::core::stringify!($a3),
         ::core::stringify!($a4), ::core::stringify!($a5), ::core::stringify!($a6))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat,
                $a4:pat, $a5:pat, $a6:pat, $a7:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2), ::core::stringify!($a3),
         ::core::stringify!($a4), ::core::stringify!($a5), ::core::stringify!($a6), ::core::stringify!($a7))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat,
                $a5:pat, $a6:pat, $a7:pat, $a8:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2), ::core::stringify!($a3),
         ::core::stringify!($a4), ::core::stringify!($a5), ::core::stringify!($a6), ::core::stringify!($a7),
         ::core::stringify!($a8))
    };
    (@_WRAPSTR ($a0:pat, $a1:pat, $a2:pat, $a3:pat, $a4:pat,
                $a5:pat, $a6:pat, $a7:pat, $a8:pat, $a9:pat)) => {
        (::core::stringify!($a0), ::core::stringify!($a1), ::core::stringify!($a2), ::core::stringify!($a3),
         ::core::stringify!($a4), ::core::stringify!($a5), ::core::stringify!($a6), ::core::stringify!($a7),
         ::core::stringify!($a8), ::core::stringify!($a9))
    };
    (@_WRAPSTR ($a:pat, $($rest:pat),*)) => {
        (::core::stringify!($a), $crate::proptest_helper!(@_WRAPSTR ($($rest),*)))
    };
    // build a property testing block that when executed, executes the full property test.
    (@_BODY $config:ident ($($parm:pat in $strategy:expr),+) [$($mod:tt)*] $body:expr) => {{
        $config.source_file = Some(file!());
        let mut runner = $crate::test_runner::TestRunner::new($config);
        let names = $crate::proptest_helper!(@_WRAPSTR ($($parm),*));
        match runner.run(
            &$crate::strategy::Strategy::prop_map(
                $crate::proptest_helper!(@_WRAP ($($strategy)*)),
                |values| $crate::sugar::NamedArguments(names, values)),
            $($mod)* |$crate::sugar::NamedArguments(
                _, $crate::proptest_helper!(@_WRAPPAT ($($parm),*)))|
            {
                let (): () = $body;
                ::core::result::Result::Ok(())
            })
        {
            ::core::result::Result::Ok(()) => (),
            ::core::result::Result::Err(e) => ::core::panic!("{}\n{}", e, runner),
        }
    }};
    // build a property testing block that when executed, executes the full property test.
    (@_BODY2 $config:ident ($($arg:tt)+) [$($mod:tt)*] $body:expr) => {{
        $config.source_file = Some(::core::file!());
        let mut runner = $crate::test_runner::TestRunner::new($config);
        let names = $crate::proptest_helper!(@_EXT _STR ($($arg)*));
        match runner.run(
            &$crate::strategy::Strategy::prop_map(
                $crate::proptest_helper!(@_EXT _STRAT ($($arg)*)),
                |values| $crate::sugar::NamedArguments(names, values)),
            $($mod)* |$crate::sugar::NamedArguments(
                _, $crate::proptest_helper!(@_EXT _PAT ($($arg)*)))|
            {
                let (): () = $body;
                ::core::result::Result::Ok(())
            })
        {
            ::core::result::Result::Ok(()) => (),
            ::core::result::Result::Err(e) => ::core::panic!("{}\n{}", e, runner),
        }
    }};

    // The logic below helps support `pat: type` in the proptest! macro.

    // These matchers define the actual logic:
    (@_STRAT [$s:ty] [$p:pat]) => { $crate::arbitrary::any::<$s>()  };
    (@_PAT [$s:ty] [$p:pat]) => { $p };
    (@_STR [$s:ty] [$p:pat]) => { ::core::stringify!($p) };
    (@_STRAT in [$s:expr] [$p:pat]) => { $s };
    (@_PAT in [$s:expr] [$p:pat]) => { $p };
    (@_STR in [$s:expr] [$p:pat]) => { ::core::stringify!($p) };

    // These matchers rewrite into the above extractors.
    // We have to do this because `:` can't FOLLOW(pat).
    // Note that this is not the full `pat` grammar...
    // See https://docs.rs/syn/0.14.2/syn/enum.Pat.html for that.
    (@_EXT $cmd:ident ($p:pat in $s:expr $(,)?)) => {
        $crate::proptest_helper!(@$cmd in [$s] [$p])
    };
    (@_EXT $cmd:ident (($p:pat) : $s:ty $(,)?)) => {
        // Users can wrap in parens as a last resort.
        $crate::proptest_helper!(@$cmd [$s] [$p])
    };
    (@_EXT $cmd:ident (_ : $s:ty $(,)?)) => {
        $crate::proptest_helper!(@$cmd [$s] [_])
    };
    (@_EXT $cmd:ident (ref mut $p:ident : $s:ty $(,)?)) => {
        $crate::proptest_helper!(@$cmd [$s] [ref mut $p])
    };
    (@_EXT $cmd:ident (ref $p:ident : $s:ty $(,)?)) => {
        $crate::proptest_helper!(@$cmd [$s] [ref $p])
    };
    (@_EXT $cmd:ident (mut $p:ident : $s:ty $(,)?)) => {
        $crate::proptest_helper!(@$cmd [$s] [mut $p])
    };
    (@_EXT $cmd:ident ($p:ident : $s:ty $(,)?)) => {
        $crate::proptest_helper!(@$cmd [$s] [$p])
    };
    (@_EXT $cmd:ident ([$($p:tt)*] : $s:ty $(,)?)) => {
        $crate::proptest_helper!(@$cmd [$s] [[$($p)*]])
    };

    // Rewrite, Inductive case:
    (@_EXT $cmd:ident ($p:pat in $s:expr, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd in [$s] [$p]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident (($p:pat) : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [$p]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident (_ : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [_]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident (ref mut $p:ident : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [ref mut $p]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident (ref $p:ident : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [ref $p]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident (mut $p:ident : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [mut $p]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident ($p:ident : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [$p]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
    (@_EXT $cmd:ident ([$($p:tt)*] : $s:ty, $($r:tt)*)) => {
        ($crate::proptest_helper!(@$cmd [$s] [[$($p)*]]), $crate::proptest_helper!(@_EXT $cmd ($($r)*)))
    };
}

#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct NamedArguments<N, V>(#[doc(hidden)] pub N, #[doc(hidden)] pub V);

impl<V: fmt::Debug> fmt::Debug for NamedArguments<&'static str, V> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} = ", self.0)?;
        self.1.fmt(f)
    }
}

macro_rules! named_arguments_tuple {
    ($($ix:tt $argn:ident $argv:ident)*) => {
        impl<'a, $($argn : Copy),*, $($argv),*> fmt::Debug
        for NamedArguments<($($argn,)*),&'a ($($argv,)*)>
        where $(NamedArguments<$argn, &'a $argv> : fmt::Debug),*,
              $($argv : 'a),*
        {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut first = true;
                $(
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    fmt::Debug::fmt(
                        &NamedArguments((self.0).$ix, &(self.1).$ix), f)?;
                )*
                Ok(())
            }
        }

        impl<$($argn : Copy),*, $($argv),*> fmt::Debug
        for NamedArguments<($($argn,)*), ($($argv,)*)>
        where $(for<'a> NamedArguments<$argn, &'a $argv> : fmt::Debug),*
        {
            #[allow(unused_assignments)]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let mut first = true;
                $(
                    if !first {
                        write!(f, ", ")?;
                    }
                    first = false;
                    fmt::Debug::fmt(
                        &NamedArguments((self.0).$ix, &(self.1).$ix), f)?;
                )*
                Ok(())
            }
        }
    }
}

named_arguments_tuple!(0 AN AV);
named_arguments_tuple!(0 AN AV 1 BN BV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV 4 EN EV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV 4 EN EV
                       5 FN FV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV 4 EN EV
                       5 FN FV 6 GN GV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV 4 EN EV
                       5 FN FV 6 GN GV 7 HN HV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV 4 EN EV
                       5 FN FV 6 GN GV 7 HN HV 8 IN IV);
named_arguments_tuple!(0 AN AV 1 BN BV 2 CN CV 3 DN DV 4 EN EV
                       5 FN FV 6 GN GV 7 HN HV 8 IN IV 9 JN JV);

#[cfg(feature = "std")]
#[doc(hidden)]
pub fn force_no_fork(config: &mut crate::test_runner::Config) {
    if config.fork() {
        eprintln!(
            "proptest: Forking/timeout not supported in closure-style \
             invocations; ignoring"
        );

        #[cfg(feature = "fork")]
        {
            config.fork = false;
        }
        #[cfg(feature = "timeout")]
        {
            config.timeout = 0;
        }
        assert!(!config.fork());
    }
}

#[cfg(not(feature = "std"))]
pub fn force_no_fork(_: &mut crate::test_runner::Config) {}

#[cfg(test)]
mod test {
    use crate::strategy::Just;

    prop_compose! {
        /// These are docs!
        #[allow(dead_code)]
        fn two_ints(relative: i32)(a in 0..relative, b in relative..)
                   -> (i32, i32) {
            (a, b)
        }
    }

    prop_compose! {
        /// These are docs!
        #[allow(dead_code)]
        pub fn two_ints_pub(relative: i32)(a in 0..relative, b in relative..)
                           -> (i32, i32) {
            (a, b)
        }
    }

    prop_compose! {
        /// These are docs!
        #[allow(dead_code, improper_ctypes_definitions)]
        pub [extern "C"] fn two_ints_pub_with_attrs
            (relative: i32)(a in 0..relative, b in relative..)
            -> (i32, i32)
        {
            (a, b)
        }
    }

    prop_compose! {
        // The only modifier we can usefully put here is "unsafe", but we want
        // to keep this crate unsafe-free, even nominally. "const" may
        // eventually work, but is not allowed right now since the generated
        // code contains local variables. `extern "C"` is accepted, even though
        // the result is useless since the return type isn't C-compatible.
        #[allow(dead_code, improper_ctypes_definitions)]
        [extern "C"] fn with_modifier(relative: i32)(a in 0..relative) -> i32 {
            a
        }
    }

    prop_compose! {
        #[allow(dead_code)]
        fn a_less_than_b()(b in 0..1000)(a in 0..b, b in Just(b))
                        -> (i32, i32) {
            (a, b)
        }
    }

    proptest! {
        #[test]
        fn test_something(a in 0u32..42u32, b in 1u32..10u32) {
            prop_assume!(a != 41 || b != 9);
            assert!(a + b < 50);
        }
    }

    prop_compose! {
        #[allow(dead_code)]
        fn single_closure_is_move(base: u64)(off in 0..10u64) -> u64 {
            base + off
        }
    }

    prop_compose! {
        #[allow(dead_code)]
        fn double_closure_is_move
            (base: u64)
            (off1 in 0..10u64)
            (off2 in off1..off1+10)
            -> u64
        {
            base + off2
        }
    }

    #[allow(unused_variables)]
    mod test_arg_counts {
        use crate::strategy::Just;

        proptest! {
            #[test]
            fn test_1_arg(a in Just(0)) { }
            #[test]
            fn test_2_arg(a in Just(0), b in Just(0)) { }
            #[test]
            fn test_3_arg(a in Just(0), b in Just(0), c in Just(0)) { }
            #[test]
            fn test_4_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0)) { }
            #[test]
            fn test_5_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0)) { }
            #[test]
            fn test_6_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0)) { }
            #[test]
            fn test_7_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0),
                          g in Just(0)) { }
            #[test]
            fn test_8_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0),
                          g in Just(0), h in Just(0)) { }
            #[test]
            fn test_9_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0),
                          g in Just(0), h in Just(0), i in Just(0)) { }
            #[test]
            fn test_a_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0),
                          g in Just(0), h in Just(0), i in Just(0),
                          j in Just(0)) { }
            #[test]
            fn test_b_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0),
                          g in Just(0), h in Just(0), i in Just(0),
                          j in Just(0), k in Just(0)) { }
            #[test]
            fn test_c_arg(a in Just(0), b in Just(0), c in Just(0),
                          d in Just(0), e in Just(0), f in Just(0),
                          g in Just(0), h in Just(0), i in Just(0),
                          j in Just(0), k in Just(0), l in Just(0)) { }
        }
    }

    #[test]
    fn named_arguments_is_debug_for_needed_cases() {
        use super::NamedArguments;

        println!("{:?}", NamedArguments("foo", &"bar"));
        println!("{:?}", NamedArguments(("foo",), &(1,)));
        println!("{:?}", NamedArguments(("foo", "bar"), &(1, 2)));
        println!("{:?}", NamedArguments(("a", "b", "c"), &(1, 2, 3)));
        println!("{:?}", NamedArguments(("a", "b", "c", "d"), &(1, 2, 3, 4)));
        println!(
            "{:?}",
            NamedArguments(("a", "b", "c", "d", "e"), &(1, 2, 3, 4, 5))
        );
        println!(
            "{:?}",
            NamedArguments(("a", "b", "c", "d", "e", "f"), &(1, 2, 3, 4, 5, 6))
        );
        println!(
            "{:?}",
            NamedArguments(
                ("a", "b", "c", "d", "e", "f", "g"),
                &(1, 2, 3, 4, 5, 6, 7)
            )
        );
        println!(
            "{:?}",
            NamedArguments(
                ("a", "b", "c", "d", "e", "f", "g", "h"),
                &(1, 2, 3, 4, 5, 6, 7, 8)
            )
        );
        println!(
            "{:?}",
            NamedArguments(
                ("a", "b", "c", "d", "e", "f", "g", "h", "i"),
                &(1, 2, 3, 4, 5, 6, 7, 8, 9)
            )
        );
        println!(
            "{:?}",
            NamedArguments(
                ("a", "b", "c", "d", "e", "f", "g", "h", "i", "j"),
                &(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
            )
        );
        println!(
            "{:?}",
            NamedArguments((("a", "b"), "c", "d"), &((1, 2), 3, 4))
        );
    }

    #[test]
    fn oneof_all_counts() {
        use crate::strategy::{Just as J, Strategy, TupleUnion, Union};

        fn expect_count(n: usize, s: impl Strategy<Value = i32>) {
            use crate::strategy::*;
            use crate::test_runner::*;
            use std::collections::HashSet;

            let mut runner = TestRunner::default();
            let mut seen = HashSet::new();
            for _ in 0..1024 {
                seen.insert(s.new_tree(&mut runner).unwrap().current());
            }

            assert_eq!(n, seen.len());
        }

        fn assert_static<T>(v: TupleUnion<T>) -> TupleUnion<T> {
            v
        }
        fn assert_dynamic<T: Strategy>(v: Union<T>) -> Union<T> {
            v
        }

        expect_count(1, prop_oneof![J(0i32)]);
        expect_count(2, assert_static(prop_oneof![J(0i32), J(1i32),]));
        expect_count(3, assert_static(prop_oneof![J(0i32), J(1i32), J(2i32),]));
        expect_count(
            4,
            assert_static(prop_oneof![J(0i32), J(1i32), J(2i32), J(3i32),]),
        );
        expect_count(
            5,
            assert_static(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
            ]),
        );
        expect_count(
            6,
            assert_static(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
                J(5i32),
            ]),
        );
        expect_count(
            7,
            assert_static(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
                J(5i32),
                J(6i32),
            ]),
        );
        expect_count(
            8,
            assert_static(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
                J(5i32),
                J(6i32),
                J(7i32),
            ]),
        );
        expect_count(
            9,
            assert_static(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
                J(5i32),
                J(6i32),
                J(7i32),
                J(8i32),
            ]),
        );
        expect_count(
            10,
            assert_static(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
                J(5i32),
                J(6i32),
                J(7i32),
                J(8i32),
                J(9i32),
            ]),
        );
        expect_count(
            11,
            assert_dynamic(prop_oneof![
                J(0i32),
                J(1i32),
                J(2i32),
                J(3i32),
                J(4i32),
                J(5i32),
                J(6i32),
                J(7i32),
                J(8i32),
                J(9i32),
                J(10i32),
            ]),
        );
    }
}

#[cfg(all(test, feature = "timeout"))]
mod test_timeout {
    proptest! {
        #![proptest_config(crate::test_runner::Config {
            fork: true,
            .. crate::test_runner::Config::default()
        })]

        // Ensure that the macro sets the test name properly. If it doesn't,
        // this test will fail to run correctly.
        #[test]
        fn test_name_set_correctly_for_fork(_ in 0u32..1u32) { }
    }
}

#[cfg(test)]
mod another_test {
    use crate::sugar;

    // Ensure that we can access the `[pub]` composed function above.
    #[allow(dead_code)]
    fn can_access_pub_compose() {
        let _ = sugar::test::two_ints_pub(42);
        let _ = sugar::test::two_ints_pub_with_attrs(42);
    }
}

#[cfg(test)]
mod ownership_tests {
    #[cfg(feature = "std")]
    proptest! {
        #[test]
        fn accept_ref_arg(ref s in "[0-9]") {
            use crate::std_facade::String;
            fn assert_string(_s: &String) {}
            assert_string(s);
        }

        #[test]
        fn accept_move_arg(s in "[0-9]") {
            use crate::std_facade::String;
            fn assert_string(_s: String) {}
            assert_string(s);
        }
    }

    #[derive(Debug)]
    struct NotClone();
    const MK: fn() -> NotClone = NotClone;

    proptest! {
        #[test]
        fn accept_noclone_arg(nc in MK) {
            let _nc2: NotClone = nc;
        }

        #[test]
        fn accept_noclone_ref_arg(ref nc in MK) {
            let _nc2: &NotClone = nc;
        }
    }
}

#[cfg(test)]
mod closure_tests {
    #[test]
    fn test_simple() {
        let x = 420;

        proptest!(|(y: i32)| {
            assert!(x != y);
        });

        proptest!(|(y in 0..100)| {
            println!("{}", y);
            assert!(x != y);
        });

        proptest!(|(y: i32,)| {
            assert!(x != y);
        });

        proptest!(|(y in 0..100,)| {
            println!("{}", y);
            assert!(x != y);
        });
    }

    #[test]
    fn test_move() {
        let foo = Foo;

        proptest!(move |(x in 1..100, y in 0..100)| {
            assert!(x + y > 0, "foo: {:?}", foo);
        });

        let foo = Foo;
        proptest!(move |(x: (), y: ())| {
            assert!(x == y, "foo: {:?}", foo);
        });

        #[derive(Debug)]
        struct Foo;
    }

    #[test]
    #[should_panic]
    #[allow(unreachable_code)]
    fn fails_if_closure_panics() {
        proptest!(|(_ in 0..1)| {
            panic!()
        });
    }

    #[test]
    fn accepts_unblocked_syntax() {
        proptest!(|(x in 0u32..10, y in 10u32..20)| assert!(x < y));
        proptest!(|(x in 0u32..10, y in 10u32..20,)| assert!(x < y));
    }

    #[test]
    fn accepts_custom_config() {
        let conf = crate::test_runner::Config::default();

        proptest!(conf, |(x in 0u32..10, y in 10u32..20)| assert!(x < y));
        proptest!(&conf, |(x in 0u32..10, y in 10u32..20)| assert!(x < y));
        proptest!(conf, move |(x in 0u32..10, y in 10u32..20)| assert!(x < y));
        proptest!(conf, |(_x: u32, _y: u32)| { });
        proptest!(conf, move |(_x: u32, _y: u32)| { });

        // Same as above, but with extra trailing comma
        proptest!(conf, |(x in 0u32..10, y in 10u32..20,)| assert!(x < y));
        proptest!(&conf, |(x in 0u32..10, y in 10u32..20,)| assert!(x < y));
        proptest!(conf, move |(x in 0u32..10, y in 10u32..20,)| assert!(x < y));
        proptest!(conf, |(_x: u32, _y: u32,)| { });
        proptest!(conf, move |(_x: u32, _y: u32,)| { });
    }
}

#[cfg(test)]
mod any_tests {
    proptest! {
        #[test]
        fn test_something
            (
                a: bool,
                b in 25u8..,
                c in 25u8..,
                _d: (),
                mut _e: (),
                ref _f: (),
                ref mut _g: (),
                [_, _]: [(); 2],
            ) {
            if a {} // Assert bool.
            assert!(b as usize + c as usize >= 50);
        }
    }

    // Test that the macro accepts some of the inputs we expect it to:
    #[test]
    fn proptest_ext_test() {
        struct Y(pub u8);

        let _ = proptest_helper!(@_EXT _STRAT( _ : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( x : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( ref x : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( mut x : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( ref mut x : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( [_, _] : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( (&mut &Y(ref x)) : u8 ));
        let _ = proptest_helper!(@_EXT _STRAT( x in 1..2 ));

        let proptest_helper!(@_EXT _PAT( _ : u8 )) = 1;
        let proptest_helper!(@_EXT _PAT( _x : u8 )) = 1;
        let proptest_helper!(@_EXT _PAT( mut _x : u8 )) = 1;
        let proptest_helper!(@_EXT _PAT( ref _x : u8 )) = 1;
        let proptest_helper!(@_EXT _PAT( ref mut _x : u8 )) = 1;
        let proptest_helper!(@_EXT _PAT( [_, _] : u8 )) = [1, 2];
        let proptest_helper!(@_EXT _PAT( (&mut &Y(ref _x)) : u8 )) = &mut &Y(1);
        let proptest_helper!(@_EXT _PAT( _x in 1..2 )) = 1;
    }
}
