//-
// Copyright 2017, 2018 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Provides higher order `Arbitrary` traits.
//! This is mainly for use by `proptest_derive`.
//!
//! ## Stability note
//!
//! This trait is mainly defined for `proptest_derive` to simplify the
//! mechanics of deriving recursive types. If you have custom containers
//! and want to support recursive for those, it is a good idea to implement
//! this trait.
//!
//! There are clearer and terser ways that work better with
//! inference such as using `proptest::collection::vec(..)`
//! to achieve the same result.
//!
//! For these reasons, the traits here are deliberately
//! not exported in a convenient way.

use crate::std_facade::fmt;

use crate::strategy::{BoxedStrategy, Strategy};

/// `ArbitraryF1` lets you lift a [`Strategy`] to unary
/// type constructors such as `Box`, `Vec`, and `Option`.
///
/// The trait corresponds to
/// [Haskell QuickCheck's `Arbitrary1` type class][HaskellQC].
///
/// [HaskellQC]:
/// https://hackage.haskell.org/package/QuickCheck-2.10.1/docs/Test-QuickCheck-Arbitrary.html#t:Arbitrary1
///
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub trait ArbitraryF1<A: fmt::Debug>: fmt::Debug + Sized {
    //==========================================================================
    // Implementation note #1
    //==========================================================================
    // It might be better to do this with generic associated types by
    // having an associated type:
    //
    // `type Strategy<A>: Strategy<Value = Self>;`
    //
    // But with this setup we will likely loose the ability to add bounds
    // such as `Hash + Eq` on `A` which is needed for `HashSet`. We might
    // be able to regain this ability with a ConstraintKinds feature.
    //
    // This alternate formulation will likely work better with type inference.
    //
    //==========================================================================
    // Implementation note #2
    //==========================================================================
    //
    // Until `-> impl Trait` has been stabilized, `BoxedStrategy` must be
    // used. This incurs an unfortunate performance penalty - but since
    // we are dealing with testing, it is better to provide slowed down and
    // somewhat less general functionality than no functionality at all.
    // Implementations should just use `.boxed()` in the end.
    //==========================================================================

    /// The type of parameters that [`lift1_with`] accepts for
    /// configuration of the lifted and generated [`Strategy`]. Parameters
    /// must implement [`Default`].
    ///
    /// [`lift1_with`]:
    ///     trait.ArbitraryF1.html#tymethod.lift1_with
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    /// [`Default`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    type Parameters: Default;

    /// Lifts a given [`Strategy`] to a new [`Strategy`] for the (presumably)
    /// bigger type. This is useful for lifting a `Strategy` for `SomeType`
    /// to a container such as `Vec<SomeType>`.
    ///
    /// Calling this for the type `X` is the equivalent of using
    /// [`X::lift1_with(base, Default::default())`].
    ///
    /// This method is defined in the trait for optimization for the
    /// default if you want to do that. It is a logic error to not
    /// preserve the semantics when overriding.
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`X::lift1_with(base, Default::default())`]:
    ///     trait.ArbitraryF1.html#tymethod.lift1_with
    fn lift1<AS>(base: AS) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static,
    {
        Self::lift1_with(base, Self::Parameters::default())
    }

    /// Lifts a given [`Strategy`] to a new [`Strategy`] for the (presumably)
    /// bigger type. This is useful for lifting a `Strategy` for `SomeType`
    /// to a container such as `Vec` of `SomeType`. The composite strategy is
    /// passed the arguments given in `args`.
    ///
    /// If you wish to use the [`default()`] arguments,
    /// use [`lift1`] instead.
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`lift1`]: trait.ArbitraryF1.html#method.lift1
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    fn lift1_with<AS>(base: AS, args: Self::Parameters) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static;
}

/// `ArbitraryF2` lets you lift [`Strategy`] to binary
/// type constructors such as `Result`, `HashMap`.
///
/// The trait corresponds to
/// [Haskell QuickCheck's `Arbitrary2` type class][HaskellQC].
///
/// [HaskellQC]:
/// https://hackage.haskell.org/package/QuickCheck-2.10.1/docs/Test-QuickCheck-Arbitrary.html#t:Arbitrary2
///
/// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
pub trait ArbitraryF2<A: fmt::Debug, B: fmt::Debug>:
    fmt::Debug + Sized
{
    /// The type of parameters that [`lift2_with`] accepts for
    /// configuration of the lifted and generated [`Strategy`]. Parameters
    /// must implement [`Default`].
    ///
    /// [`lift2_with`]: trait.ArbitraryF2.html#tymethod.lift2_with
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`Default`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    type Parameters: Default;

    /// Lifts two given strategies to a new [`Strategy`] for the (presumably)
    /// bigger type. This is useful for lifting a `Strategy` for `Type1`
    /// and one for `Type2` to a container such as `HashMap<Type1, Type2>`.
    ///
    /// Calling this for the type `X` is the equivalent of using
    /// [`X::lift2_with(base, Default::default())`].
    ///
    /// This method is defined in the trait for optimization for the
    /// default if you want to do that. It is a logic error to not
    /// preserve the semantics when overriding.
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`X::lift2_with(base, Default::default())`]:
    ///     trait.Arbitrary.html#tymethod.lift2_with
    fn lift2<AS, BS>(fst: AS, snd: BS) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static,
        BS: Strategy<Value = B> + 'static,
    {
        Self::lift2_with(fst, snd, Self::Parameters::default())
    }

    /// Lifts two given strategies to a new [`Strategy`] for the (presumably)
    /// bigger type. This is useful for lifting a `Strategy` for `Type1`
    /// and one for `Type2` to a container such as `HashMap<Type1, Type2>`.
    /// The composite strategy is passed the arguments given in `args`.
    ///
    /// If you wish to use the [`default()`] arguments,
    /// use [`lift2`] instead.
    ///
    /// [`Strategy`]: ../proptest/strategy/trait.Strategy.html
    ///
    /// [`lift2`]: trait.ArbitraryF2.html#method.lift2
    ///
    /// [`default()`]:
    ///     https://doc.rust-lang.org/nightly/std/default/trait.Default.html
    fn lift2_with<AS, BS>(
        fst: AS,
        snd: BS,
        args: Self::Parameters,
    ) -> BoxedStrategy<Self>
    where
        AS: Strategy<Value = A> + 'static,
        BS: Strategy<Value = B> + 'static;
}

macro_rules! lift1 {
    ([$($bounds : tt)*] $typ: ty, $params: ty;
     $base: ident, $args: ident => $logic: expr) => {
        impl<A: ::core::fmt::Debug + $($bounds)*>
        $crate::arbitrary::functor::ArbitraryF1<A>
        for $typ {
            type Parameters = $params;

            fn lift1_with<S>($base: S, $args: Self::Parameters)
                -> $crate::strategy::BoxedStrategy<Self>
            where
                S: $crate::strategy::Strategy<Value = A> + 'static
            {
                $crate::strategy::Strategy::boxed($logic)
            }
        }
    };
    ([$($bounds : tt)*] $typ: ty; $base: ident => $logic: expr) => {
        lift1!([$($bounds)*] $typ, (); $base, _args => $logic);
    };
    ([$($bounds : tt)*] $typ: ty; $mapper: expr) => {
        lift1!(['static + $($bounds)*] $typ; base =>
            $crate::strategy::Strategy::prop_map(base, $mapper));
    };
    ([$($bounds : tt)*] $typ: ty) => {
        lift1!(['static + $($bounds)*] $typ; base =>
            $crate::strategy::Strategy::prop_map_into(base));
    };
}
