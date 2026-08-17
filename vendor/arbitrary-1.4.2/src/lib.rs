// Copyright © 2019 The Rust Fuzz Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! The `Arbitrary` trait crate.
//!
//! This trait provides an [`Arbitrary`] trait to
//! produce well-typed, structured values, from raw, byte buffers. It is
//! generally intended to be used with fuzzers like AFL or libFuzzer. See the
//! [`Arbitrary`] trait's documentation for details on
//! automatically deriving, implementing, and/or using the trait.

#![deny(bad_style)]
#![deny(missing_docs)]
#![deny(future_incompatible)]
#![deny(nonstandard_style)]
#![deny(rust_2018_compatibility)]
#![deny(rust_2018_idioms)]
#![deny(unused)]

mod error;
mod foreign;
pub mod size_hint;
pub mod unstructured;

#[cfg(test)]
mod tests;

pub use error::*;

#[cfg(feature = "derive_arbitrary")]
pub use derive_arbitrary::*;

#[doc(inline)]
pub use unstructured::Unstructured;

/// Error indicating that the maximum recursion depth has been reached while calculating [`Arbitrary::size_hint`]()
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct MaxRecursionReached {}

impl core::fmt::Display for MaxRecursionReached {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("Maximum recursion depth has been reached")
    }
}

impl std::error::Error for MaxRecursionReached {}

/// Generate arbitrary structured values from raw, unstructured data.
///
/// The `Arbitrary` trait allows you to generate valid structured values, like
/// `HashMap`s, or ASTs, or `MyTomlConfig`, or any other data structure from
/// raw, unstructured bytes provided by a fuzzer.
///
/// # Deriving `Arbitrary`
///
/// Automatically deriving the `Arbitrary` trait is the recommended way to
/// implement `Arbitrary` for your types.
///
/// Using the custom derive requires that you enable the `"derive"` cargo
/// feature in your `Cargo.toml`:
///
/// ```toml
/// [dependencies]
/// arbitrary = { version = "1", features = ["derive"] }
/// ```
///
/// Then, you add the `#[derive(Arbitrary)]` annotation to your `struct` or
/// `enum` type definition:
///
/// ```
/// # #[cfg(feature = "derive")] mod foo {
/// use arbitrary::Arbitrary;
/// use std::collections::HashSet;
///
/// #[derive(Arbitrary)]
/// pub struct AddressBook {
///     friends: HashSet<Friend>,
/// }
///
/// #[derive(Arbitrary, Hash, Eq, PartialEq)]
/// pub enum Friend {
///     Buddy { name: String },
///     Pal { age: usize },
/// }
/// # }
/// ```
///
/// Every member of the `struct` or `enum` must also implement `Arbitrary`.
///
/// It is also possible to change the default bounds added by the derive:
///
/// ```
/// # #[cfg(feature = "derive")] mod foo {
/// use arbitrary::Arbitrary;
///
/// trait Trait {
///     type Assoc: for<'a> Arbitrary<'a>;
/// }
///
/// #[derive(Arbitrary)]
/// // The bounds are used verbatim, so any existing trait bounds will need to be repeated.
/// #[arbitrary(bound = "T: Trait")]
/// struct Point<T: Trait> {
///     x: T::Assoc,
/// }
/// # }
/// ```
///
/// # Implementing `Arbitrary` By Hand
///
/// Implementing `Arbitrary` mostly involves nested calls to other `Arbitrary`
/// arbitrary implementations for each of your `struct` or `enum`'s members. But
/// sometimes you need some amount of raw data, or you need to generate a
/// variably-sized collection type, or something of that sort. The
/// [`Unstructured`] type helps you with these tasks.
///
/// ```
/// # #[cfg(feature = "derive")] mod foo {
/// # pub struct MyCollection<T> { _t: std::marker::PhantomData<T> }
/// # impl<T> MyCollection<T> {
/// #     pub fn new() -> Self { MyCollection { _t: std::marker::PhantomData } }
/// #     pub fn insert(&mut self, element: T) {}
/// # }
/// use arbitrary::{Arbitrary, Result, Unstructured};
///
/// impl<'a, T> Arbitrary<'a> for MyCollection<T>
/// where
///     T: Arbitrary<'a>,
/// {
///     fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
///         // Get an iterator of arbitrary `T`s.
///         let iter = u.arbitrary_iter::<T>()?;
///
///         // And then create a collection!
///         let mut my_collection = MyCollection::new();
///         for elem_result in iter {
///             let elem = elem_result?;
///             my_collection.insert(elem);
///         }
///
///         Ok(my_collection)
///     }
/// }
/// # }
/// ```
///
/// # A Note On Output Distributions
///
/// There is no requirement for a particular distribution of the values. For
/// example, it is not required that every value appears with the same
/// probability. That being said, the main use for `Arbitrary` is for fuzzing,
/// so in many cases a uniform distribution will make the most sense in order to
/// provide the best coverage of the domain. In other cases this is not
/// desirable or even possible, for example when sampling from a uniform
/// distribution is computationally expensive or in the case of collections that
/// may grow indefinitely.
pub trait Arbitrary<'a>: Sized {
    /// Generate an arbitrary value of `Self` from the given unstructured data.
    ///
    /// Calling `Arbitrary::arbitrary` requires that you have some raw data,
    /// perhaps given to you by a fuzzer like AFL or libFuzzer. You wrap this
    /// raw data in an `Unstructured`, and then you can call `<MyType as
    /// Arbitrary>::arbitrary` to construct an arbitrary instance of `MyType`
    /// from that unstructured data.
    ///
    /// Implementations may return an error if there is not enough data to
    /// construct a full instance of `Self`, or they may fill out the rest of
    /// `Self` with dummy values. Using dummy values when the underlying data is
    /// exhausted can help avoid accidentally "defeating" some of the fuzzer's
    /// mutations to the underlying byte stream that might otherwise lead to
    /// interesting runtime behavior or new code coverage if only we had just a
    /// few more bytes. However, it also requires that implementations for
    /// recursive types (e.g. `struct Foo(Option<Box<Foo>>)`) avoid infinite
    /// recursion when the underlying data is exhausted.
    ///
    /// ```
    /// # #[cfg(feature = "derive")] fn foo() {
    /// use arbitrary::{Arbitrary, Unstructured};
    ///
    /// #[derive(Arbitrary)]
    /// pub struct MyType {
    ///     // ...
    /// }
    ///
    /// // Get the raw data from the fuzzer or wherever else.
    /// # let get_raw_data_from_fuzzer = || &[];
    /// let raw_data: &[u8] = get_raw_data_from_fuzzer();
    ///
    /// // Wrap that raw data in an `Unstructured`.
    /// let mut unstructured = Unstructured::new(raw_data);
    ///
    /// // Generate an arbitrary instance of `MyType` and do stuff with it.
    /// if let Ok(value) = MyType::arbitrary(&mut unstructured) {
    /// #   let do_stuff = |_| {};
    ///     do_stuff(value);
    /// }
    /// # }
    /// ```
    ///
    /// See also the documentation for [`Unstructured`].
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self>;

    /// Generate an arbitrary value of `Self` from the entirety of the given
    /// unstructured data.
    ///
    /// This is similar to Arbitrary::arbitrary, however it assumes that it is
    /// the last consumer of the given data, and is thus able to consume it all
    /// if it needs.  See also the documentation for
    /// [`Unstructured`].
    fn arbitrary_take_rest(mut u: Unstructured<'a>) -> Result<Self> {
        Self::arbitrary(&mut u)
    }

    /// Get a size hint for how many bytes out of an `Unstructured` this type
    /// needs to construct itself.
    ///
    /// This is useful for determining how many elements we should insert when
    /// creating an arbitrary collection.
    ///
    /// The return value is similar to [`Iterator::size_hint`]: it returns a
    /// tuple where the first element is a lower bound on the number of bytes
    /// required, and the second element is an optional upper bound.
    ///
    /// The default implementation return `(0, None)` which is correct for any
    /// type, but not ultimately that useful. Using `#[derive(Arbitrary)]` will
    /// create a better implementation. If you are writing an `Arbitrary`
    /// implementation by hand, and your type can be part of a dynamically sized
    /// collection (such as `Vec`), you are strongly encouraged to override this
    /// default with a better implementation, and also override
    /// [`try_size_hint`].
    ///
    /// ## How to implement this
    ///
    /// If the size hint calculation is a trivial constant and does not recurse
    /// into any other `size_hint` call, you should implement it in `size_hint`:
    ///
    /// ```
    /// use arbitrary::{size_hint, Arbitrary, Result, Unstructured};
    ///
    /// struct SomeStruct(u8);
    ///
    /// impl<'a> Arbitrary<'a> for SomeStruct {
    ///     fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
    ///         let buf = &mut [0];
    ///         u.fill_buffer(buf)?;
    ///         Ok(SomeStruct(buf[0]))
    ///     }
    ///
    ///     #[inline]
    ///     fn size_hint(depth: usize) -> (usize, Option<usize>) {
    ///         let _ = depth;
    ///         (1, Some(1))
    ///     }
    /// }
    /// ```
    ///
    /// Otherwise, it should instead be implemented in [`try_size_hint`],
    /// and the `size_hint` implementation should forward to it:
    ///
    /// ```
    /// use arbitrary::{size_hint, Arbitrary, MaxRecursionReached, Result, Unstructured};
    ///
    /// struct SomeStruct<A, B> {
    ///     a: A,
    ///     b: B,
    /// }
    ///
    /// impl<'a, A: Arbitrary<'a>, B: Arbitrary<'a>> Arbitrary<'a> for SomeStruct<A, B> {
    ///     fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self> {
    ///         // ...
    /// #       todo!()
    ///     }
    ///
    ///     fn size_hint(depth: usize) -> (usize, Option<usize>) {
    ///         // Return the value of try_size_hint
    ///         //
    ///         // If the recursion fails, return the default, always valid `(0, None)`
    ///         Self::try_size_hint(depth).unwrap_or_default()
    ///     }
    ///
    ///     fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
    ///         // Protect against potential infinite recursion with
    ///         // `try_recursion_guard`.
    ///         size_hint::try_recursion_guard(depth, |depth| {
    ///             // If we aren't too deep, then `recursion_guard` calls
    ///             // this closure, which implements the natural size hint.
    ///             // Don't forget to use the new `depth` in all nested
    ///             // `try_size_hint` calls! We recommend shadowing the
    ///             // parameter, like what is done here, so that you can't
    ///             // accidentally use the wrong depth.
    ///             Ok(size_hint::and(
    ///                 <A as Arbitrary>::try_size_hint(depth)?,
    ///                 <B as Arbitrary>::try_size_hint(depth)?,
    ///             ))
    ///         })
    ///     }
    /// }
    /// ```
    ///
    /// ## Invariant
    ///
    /// It must be possible to construct every possible output using only inputs
    /// of lengths bounded by these parameters. This applies to both
    /// [`Arbitrary::arbitrary`] and [`Arbitrary::arbitrary_take_rest`].
    ///
    /// This is trivially true for `(0, None)`. To restrict this further, it
    /// must be proven that all inputs that are now excluded produced redundant
    /// outputs which are still possible to produce using the reduced input
    /// space.
    ///
    /// [iterator-size-hint]: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html#method.size_hint
    /// [`try_size_hint`]: Arbitrary::try_size_hint
    #[inline]
    fn size_hint(depth: usize) -> (usize, Option<usize>) {
        let _ = depth;
        (0, None)
    }

    /// Get a size hint for how many bytes out of an `Unstructured` this type
    /// needs to construct itself.
    ///
    /// Unlike [`size_hint`], this function keeps the information that the
    /// recursion limit was reached. This is required to "short circuit" the
    /// calculation and avoid exponential blowup with recursive structures.
    ///
    /// If you are implementing [`size_hint`] for a struct that could be
    /// recursive, you should implement `try_size_hint` and call the
    /// `try_size_hint` when recursing
    ///
    ///
    /// The return value is similar to [`core::iter::Iterator::size_hint`]: it
    /// returns a tuple where the first element is a lower bound on the number
    /// of bytes required, and the second element is an optional upper bound.
    ///
    /// The default implementation returns the value of [`size_hint`] which is
    /// correct for any type, but might lead to exponential blowup when dealing
    /// with recursive types.
    ///
    /// ## Invariant
    ///
    /// It must be possible to construct every possible output using only inputs
    /// of lengths bounded by these parameters. This applies to both
    /// [`Arbitrary::arbitrary`] and [`Arbitrary::arbitrary_take_rest`].
    ///
    /// This is trivially true for `(0, None)`. To restrict this further, it
    /// must be proven that all inputs that are now excluded produced redundant
    /// outputs which are still possible to produce using the reduced input
    /// space.
    ///
    /// ## When to implement `try_size_hint`
    ///
    /// If you 100% know that the type you are implementing `Arbitrary` for is
    /// not a recursive type, or your implementation is not transitively calling
    /// any other `size_hint` methods, you may implement [`size_hint`], and the
    /// default `try_size_hint` implementation will use it.
    ///
    /// Note that if you are implementing `Arbitrary` for a generic type, you
    /// cannot guarantee the lack of type recursion!
    ///
    /// Otherwise, when there is possible type recursion, you should implement
    /// `try_size_hint` instead.
    ///
    /// ## The `depth` parameter
    ///
    /// When implementing `try_size_hint`, you need to use
    /// [`arbitrary::size_hint::try_recursion_guard(depth)`][crate::size_hint::try_recursion_guard]
    /// to prevent potential infinite recursion when calculating size hints for
    /// potentially recursive types:
    ///
    /// ```
    /// use arbitrary::{size_hint, Arbitrary, MaxRecursionReached, Unstructured};
    ///
    /// // This can potentially be a recursive type if `L` or `R` contain
    /// // something like `Box<Option<MyEither<L, R>>>`!
    /// enum MyEither<L, R> {
    ///     Left(L),
    ///     Right(R),
    /// }
    ///
    /// impl<'a, L, R> Arbitrary<'a> for MyEither<L, R>
    /// where
    ///     L: Arbitrary<'a>,
    ///     R: Arbitrary<'a>,
    /// {
    ///     fn arbitrary(u: &mut Unstructured) -> arbitrary::Result<Self> {
    ///         // ...
    /// #       unimplemented!()
    ///     }
    ///
    ///     fn size_hint(depth: usize) -> (usize, Option<usize>) {
    ///         // Return the value of `try_size_hint`
    ///         //
    ///         // If the recursion fails, return the default `(0, None)` range,
    ///         // which is always valid.
    ///         Self::try_size_hint(depth).unwrap_or_default()
    ///     }
    ///
    ///     fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
    ///         // Protect against potential infinite recursion with
    ///         // `try_recursion_guard`.
    ///         size_hint::try_recursion_guard(depth, |depth| {
    ///             // If we aren't too deep, then `recursion_guard` calls
    ///             // this closure, which implements the natural size hint.
    ///             // Don't forget to use the new `depth` in all nested
    ///             // `try_size_hint` calls! We recommend shadowing the
    ///             // parameter, like what is done here, so that you can't
    ///             // accidentally use the wrong depth.
    ///             Ok(size_hint::or(
    ///                 <L as Arbitrary>::try_size_hint(depth)?,
    ///                 <R as Arbitrary>::try_size_hint(depth)?,
    ///             ))
    ///         })
    ///     }
    /// }
    /// ```
    #[inline]
    fn try_size_hint(depth: usize) -> Result<(usize, Option<usize>), MaxRecursionReached> {
        Ok(Self::size_hint(depth))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn exhausted_entropy() {
        let mut u = Unstructured::new(&[]);
        assert_eq!(u.arbitrary::<bool>().unwrap(), false);
        assert_eq!(u.arbitrary::<u8>().unwrap(), 0);
        assert_eq!(u.arbitrary::<usize>().unwrap(), 0);
        assert_eq!(u.arbitrary::<f32>().unwrap(), 0.0);
        assert_eq!(u.arbitrary::<f64>().unwrap(), 0.0);
        assert_eq!(u.arbitrary::<Option<u32>>().unwrap(), None);
        assert_eq!(u.int_in_range(4..=100).unwrap(), 4);
        assert_eq!(u.choose_index(10).unwrap(), 0);
        assert_eq!(u.ratio(5, 7).unwrap(), true);
    }
}

/// Multiple conflicting arbitrary attributes are used on the same field:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(value = 2)]
///     #[arbitrary(value = 2)]
///     x: i32,
/// }
/// ```
///
/// An unknown attribute:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(unknown_attr)]
///     x: i32,
/// }
/// ```
///
/// An unknown attribute with a value:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(unknown_attr = 13)]
///     x: i32,
/// }
/// ```
///
/// `value` without RHS:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(value)]
///     x: i32,
/// }
/// ```
///
/// `with` without RHS:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// struct Point {
///     #[arbitrary(with)]
///     x: i32,
/// }
/// ```
///
/// Multiple conflicting bounds at the container-level:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default")]
/// #[arbitrary(bound = "T: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Multiple conflicting bounds in a single bound attribute:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default, T: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Multiple conflicting bounds in multiple bound attributes:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default", bound = "T: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Too many bounds supplied:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default")]
/// struct Point {
///     x: i32,
/// }
/// ```
///
/// Too many bounds supplied across multiple attributes:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// #[arbitrary(bound = "T: Default")]
/// #[arbitrary(bound = "U: Default")]
/// struct Point<T: Default> {
///     #[arbitrary(default)]
///     x: T,
/// }
/// ```
///
/// Attempt to use the derive attribute on an enum variant:
/// ```compile_fail
/// #[derive(::arbitrary::Arbitrary)]
/// enum Enum<T: Default> {
///     #[arbitrary(default)]
///     Variant(T),
/// }
/// ```
#[cfg(all(doctest, feature = "derive"))]
pub struct CompileFailTests;

// Support for `#[derive(Arbitrary)]`.
#[doc(hidden)]
#[cfg(feature = "derive")]
pub mod details {
    use super::*;

    // Hidden trait that papers over the difference between `&mut Unstructured` and
    // `Unstructured` arguments so that `with_recursive_count` can be used for both
    // `arbitrary` and `arbitrary_take_rest`.
    pub trait IsEmpty {
        fn is_empty(&self) -> bool;
    }

    impl IsEmpty for Unstructured<'_> {
        fn is_empty(&self) -> bool {
            Unstructured::is_empty(self)
        }
    }

    impl IsEmpty for &mut Unstructured<'_> {
        fn is_empty(&self) -> bool {
            Unstructured::is_empty(self)
        }
    }

    // Calls `f` with a recursive count guard.
    #[inline]
    pub fn with_recursive_count<U: IsEmpty, R>(
        u: U,
        recursive_count: &'static std::thread::LocalKey<std::cell::Cell<u32>>,
        f: impl FnOnce(U) -> Result<R>,
    ) -> Result<R> {
        let guard_against_recursion = u.is_empty();
        if guard_against_recursion {
            recursive_count.with(|count| {
                if count.get() > 0 {
                    return Err(Error::NotEnoughData);
                }
                count.set(count.get() + 1);
                Ok(())
            })?;
        }

        let result = f(u);

        if guard_against_recursion {
            recursive_count.with(|count| {
                count.set(count.get() - 1);
            });
        }

        result
    }
}
