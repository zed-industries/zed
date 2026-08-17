//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::fmt;

use crate::strategy::Strategy;

//==============================================================================
// Arbitrary trait
//==============================================================================

/// Arbitrary determines a canonical [`Strategy`] for the implementing type.
///
/// It provides the method `arbitrary_with` which generates a `Strategy` for
/// producing arbitrary values of the implementing type *(`Self`)*. In general,
/// these strategies will produce the entire set of values possible for the
/// type, up to some size limitation or constraints set by their parameters.
/// When this is not desired, strategies to produce the desired values can be
/// built by combining [`Strategy`]s as described in the crate documentation.
///
/// This trait analogous to
/// [Haskell QuickCheck's implementation of `Arbitrary`][HaskellQC].
/// In this interpretation of `Arbitrary`, `Strategy` is the equivalent of
/// the `Gen` monad. Unlike in QuickCheck, `Arbitrary` is not a core component;
/// types do not need to implement `Arbitrary` unless one wants to use
/// [`any`](fn.any.html) or other free functions in this module.
///
/// `Arbitrary` currently only works for types which represent owned data as
/// opposed to borrowed data. This is a fundamental restriction of `proptest`
/// which may be lifted in the future as the [generic associated types (GAT)]
/// feature of Rust is implemented and stabilized.
///
/// If you do not have unique constraints on how to generate the data for your
/// custom types, consider using [the derive macro] to implement Arbitrary
///
/// [generic associated types (GAT)]: https://github.com/rust-lang/rust/issues/44265
///
/// [`Strategy`]: ../strategy/trait.Strategy.html
///
/// [the derive macro]: https://docs.rs/proptest-derive/latest/proptest_derive/
///
/// [HaskellQC]:
/// https://hackage.haskell.org/package/QuickCheck/docs/Test-QuickCheck-Arbitrary.html
pub trait Arbitrary: Sized + fmt::Debug {
    /// The type of parameters that [`arbitrary_with`] accepts for configuration
    /// of the generated [`Strategy`]. Parameters must implement [`Default`].
    ///
    /// [`arbitrary_with`]: trait.Arbitrary.html#tymethod.arbitrary_with
    ///
    /// [`Strategy`]: ../strategy/trait.Strategy.html
    /// [`Default`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    type Parameters: Default;

    /// Generates a [`Strategy`] for producing arbitrary values
    /// of type the implementing type (`Self`).
    ///
    /// Calling this for the type `X` is the equivalent of using
    /// [`X::arbitrary_with(Default::default())`].
    ///
    /// This method is defined in the trait for optimization for the
    /// default if you want to do that. It is a logic error to not
    /// preserve the semantics when overriding.
    ///
    /// [`Strategy`]: ../strategy/trait.Strategy.html
    /// [`X::arbitrary_with(Default::default())`]:
    ///     trait.Arbitrary.html#tymethod.arbitrary_with
    fn arbitrary() -> Self::Strategy {
        Self::arbitrary_with(Default::default())
    }

    /// Generates a [`Strategy`] for producing arbitrary values of type the
    /// implementing type (`Self`). The strategy is passed the arguments given
    /// in args.
    ///
    /// If you wish to use the [`default()`] arguments,
    /// use [`arbitrary`] instead.
    ///
    /// [`Strategy`]: ../strategy/trait.Strategy.html
    ///
    /// [`arbitrary`]: trait.Arbitrary.html#method.arbitrary
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    fn arbitrary_with(args: Self::Parameters) -> Self::Strategy;

    /// The type of [`Strategy`] used to generate values of type `Self`.
    ///
    /// [`Strategy`]: ../strategy/trait.Strategy.html
    type Strategy: Strategy<Value = Self>;
}

//==============================================================================
// Type aliases for associated types
//==============================================================================

/// `StrategyFor` allows you to mention the type of [`Strategy`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break. This can be especially beneficial when the
/// type of `Strategy` that you are dealing with is very long in name
/// (the case with generics).
///
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../strategy/trait.Strategy.html
pub type StrategyFor<A> = <A as Arbitrary>::Strategy;

/// `ParamsFor` allows you to mention the type of [`Parameters`] for the input
/// type `A` without directly using associated types or without resorting to
/// existential types. This way, if implementation of [`Arbitrary`] changes,
/// your tests should not break.
///
/// [`Parameters`]: trait.Arbitrary.html#associatedtype.Parameters
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../strategy/trait.Strategy.html
pub type ParamsFor<A> = <A as Arbitrary>::Parameters;

//==============================================================================
// Free functions that people should use
//==============================================================================

/// Generates a [`Strategy`] producing [`Arbitrary`][trait Arbitrary] values of
/// `A`. Unlike [`arbitrary`][fn arbitrary], it should be used for being
/// explicit on what `A` is. For clarity, this may be a good idea.
///
/// Use this version instead of [`arbitrary`][fn arbitrary] if you want to be
/// clear which type you want to generate a `Strategy` for, or if you don't
/// have an anchoring type for type inference to work with.
///
/// If you want to customize how the strategy is generated, use
/// [`any_with::<A>(args)`] where `args` are any arguments accepted by
/// the `Arbitrary` impl in question.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// use proptest::prelude::*;
///
/// proptest! {
///     fn reverse_reverse_is_identity(ref vec in any::<Vec<u32>>()) {
///         let vec2 = vec.iter().cloned().rev().rev().collect::<Vec<u32>>();
///         prop_assert_eq!(vec, &vec2);
///     }
/// }
///
/// fn main() {
///     reverse_reverse_is_identity();
/// }
/// ```
///
/// [`any_with::<A>(args)`]: fn.any_with.html
/// [fn arbitrary]: fn.arbitrary.html
/// [trait Arbitrary]: trait.Arbitrary.html
/// [`Strategy`]: ../strategy/trait.Strategy.html
#[must_use = "strategies do nothing unless used"]
pub fn any<A: Arbitrary>() -> StrategyFor<A> {
    // ^-- We use a shorter name so that turbofish becomes more ergonomic.
    A::arbitrary()
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A` with the
/// given configuration arguments passed in `args`. Unlike [`arbitrary_with`],
/// it should be used for being explicit on what `A` is.
/// For clarity, this may be a good idea.
///
/// Use this version instead of [`arbitrary_with`] if you want to be clear which
/// type you want to generate a `Strategy` for, or if you don't have an anchoring
/// type for type inference to work with.
///
/// If you don't want to specify any arguments and instead use the default
/// behavior, you should use [`any::<A>()`].
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// use proptest::prelude::*;
/// use proptest::collection::size_range;
///
/// proptest! {
///     fn reverse_reverse_is_identity
///         (ref vec in any_with::<Vec<u32>>(size_range(1000).lift()))
///     {
///         let vec2 = vec.iter().cloned().rev().rev().collect::<Vec<u32>>();
///         prop_assert_eq!(vec, &vec2);
///     }
/// }
///
/// fn main() {
///     reverse_reverse_is_identity();
/// }
/// ```
///
/// [`any::<A>()`]: fn.any.html
/// [`arbitrary_with`]: fn.arbitrary_with.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../strategy/trait.Strategy.html
#[must_use = "strategies do nothing unless used"]
pub fn any_with<A: Arbitrary>(args: ParamsFor<A>) -> StrategyFor<A> {
    // ^-- We use a shorter name so that turbofish becomes more ergonomic.
    A::arbitrary_with(args)
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A`.
/// Works better with type inference than [`any::<A>()`].
///
/// With this version, you shouldn't need to specify any of the (many) type
/// parameters explicitly. This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use [`any::<A>()`] instead.
///
/// For clarity, it is often a good idea to specify the type generated, and
/// so using [`any::<A>()`] can be a good idea.
///
/// If you want to customize how the strategy is generated, use
/// [`arbitrary_with(args)`] where `args` is of type
/// `<A as Arbitrary>::Parameters`.
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest;
/// use proptest::arbitrary::{arbitrary, StrategyFor};
///
/// fn gen_vec_usize() -> StrategyFor<Vec<usize>> {
///     arbitrary()
/// }
///
/// # fn main() {}
/// ```
///
/// [`arbitrary_with(args)`]: fn.arbitrary_with.html
/// [`any::<A>()`]: fn.any.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../strategy/trait.Strategy.html
#[must_use = "strategies do nothing unless used"]
pub fn arbitrary<A, S>() -> S
where
    // The backlinking here cause an injection which helps type inference.
    S: Strategy<Value = A>,
    A: Arbitrary<Strategy = S>,
{
    A::arbitrary()
}

/// Generates a [`Strategy`] producing [`Arbitrary`] values of `A` with the
/// given configuration arguments passed in `args`.
/// Works better with type inference than [`any_with::<A>(args)`].
///
/// With this version, you shouldn't need to specify any of the (many) type
/// parameters explicitly. This can have a positive effect on type inference.
/// However, if you want specify `A`, you should use
/// [`any_with::<A>(args)`] instead.
///
/// For clarity, it is often a good idea to specify the type generated, and
/// so using [`any_with::<A>(args)`] can be a good idea.
///
/// If you don't want to specify any arguments and instead use the default
/// behavior, you should use [`arbitrary()`].
///
/// # Example
///
/// The function can be used as:
///
/// ```rust
/// extern crate proptest;
/// use proptest::arbitrary::{arbitrary_with, StrategyFor};
/// use proptest::collection::size_range;
///
/// fn gen_vec_10_u32() -> StrategyFor<Vec<u32>> {
///     arbitrary_with(size_range(10).lift())
/// }
///
/// # fn main() {}
/// ```
///
/// [`any_with::<A>(args)`]: fn.any_with.html
/// [`arbitrary()`]: fn.arbitrary.html
/// [`Arbitrary`]: trait.Arbitrary.html
/// [`Strategy`]: ../strategy/trait.Strategy.html
#[must_use = "strategies do nothing unless used"]
pub fn arbitrary_with<A, S, P>(args: P) -> S
where
    P: Default,
    // The backlinking here cause an injection which helps type inference.
    S: Strategy<Value = A>,
    A: Arbitrary<Strategy = S, Parameters = P>,
{
    A::arbitrary_with(args)
}
