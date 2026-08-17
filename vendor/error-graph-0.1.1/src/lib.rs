//! Allows non-fatal errors in a tree of subfunctions to easily be collected by a caller
//!
//! Provides the [`error_graph::ErrorList<E>`][ErrorList] type to hold a list of non-fatal errors
//! that occurred while a function was running.
//!
//! It has a [`subwriter()`][WriteErrorList::subwriter] method that can be passed as a parameter to
//! a subfunction and allows that subfunction to record all the non-fatal errors it encounters.
//! When the subfunction is done running, its error list will be mapped to the caller's error type
//! and added to the caller's [ErrorList] automatically.
//!
//! Since subfunctions may in-turn also use the [`subwriter()`][WriteErrorList::subwriter]
//! function on the writter given to them by their caller, this creates a tree of non-fatal errors
//! that occurred during the execution of an entire call graph.
//!
//! # Usage
//!
//! ```
//! # use error_graph::{ErrorList, WriteErrorList, strategy::{DontCare, ErrorOccurred}};
//! enum UpperError {
//!     Upper,
//!     Middle(ErrorList<MiddleError>),
//! }
//! enum MiddleError {
//!     Middle,
//!     Lower(ErrorList<LowerError>),
//! }
//! enum LowerError {
//!     Lower,
//! }
//! fn upper() {
//!     let mut errors = ErrorList::default();
//!     errors.push(UpperError::Upper);
//!     // Map the ErrorList<MiddleError> to our UpperError::Middle variant
//!     middle(errors.subwriter(UpperError::Middle));
//!     errors.push(UpperError::Upper);
//!
//!     // Some callers just don't want to know if things went wrong or not
//!     middle(DontCare);
//!
//!     // Some callers are only interested in whether an error occurred or not
//!     let mut error_occurred = ErrorOccurred::default();
//!     middle(&mut error_occurred);
//!     if error_occurred.as_bool() {
//!         errors.push(UpperError::Upper);
//!     }
//! }
//! fn middle(mut errors: impl WriteErrorList<MiddleError>) {
//!     // We can pass a sublist by mutable reference if we need to manipulate it before and after
//!     let mut sublist = errors.sublist(MiddleError::Lower);
//!     lower(&mut sublist);
//!     let num_errors = sublist.len();
//!     sublist.finish();
//!     if num_errors > 10 {
//!         errors.push(MiddleError::Middle);
//!     }
//!     // We can pass a reference directly to our error list for peer functions
//!     middle_2(&mut errors);
//! }
//! fn middle_2(mut errors: impl WriteErrorList<MiddleError>) {
//!     errors.push(MiddleError::Middle);
//! }
//! fn lower(mut errors: impl WriteErrorList<LowerError>) {
//!     errors.push(LowerError::Lower);
//! }
//! ```
//!
//! # Motivation
//!
//! In most call graphs, a function that encounters an error will early-return and pass an
//! error type to its caller. The caller will often respond by passing that error further up the
//! call stack up to its own caller (possibly after wrapping it in its own error type). That
//! continues so-on-and-so-forth until some caller finally handles the error, returns from `main`,
//! or panics. Ultimately, the result is that some interested caller will receive a linear chain of
//! errors that led to the failure.
//!
//! But, not all errors are fatal -- Sometimes, a function might be able to continue working after
//! it encounters an error and still be able to at-least-partially achieve its goals. Calling it
//! again - or calling other functions in the same API - is still permissible and may also result
//! in full or partial functionality.
//!
//! In that case, the function may still choose to return `Result::Ok`; however, that leaves the
//! function with a dilemma -- How can it report the non-fatal errors to the caller?
//!
//! 1.  **Return a tuple in its `Result::Ok` type**: that wouldn't capture the non-fatal errors in
//!     the case that a fatal error occurs, so it would also have to be added to the `Result::Err`
//!     type as well.
//!
//!     That adds a bunch of boilerplate, as the function needs to allocate the list and map it
//!     into the return type for every error return and good return. It also makes the function
//!     signature much more noisy.
//!
//! 2.  **Take a list as a mutable reference?**: Better, but now the caller has to allocate the
//!     list, and there's no way for it to opt out if it doesn't care about the non-fatal errors.
//!
//! 3.  **Maybe add an `Option` to it?** Okay, so a parameter like `errors: Option<&mut Vec<E>>`?
//!     Getting warmer, but now the child has to do a bunch of
//!     `if let Some(v) = errors { v.push(error); }` all over the place.
//!
//! And what about the caller side of it? For a simple caller, the last point isn't too bad: The
//! caller just has to allocate the list, pass `Some(&mut errors)` to the child, and check it upon
//! return.
//!
//! But often, the caller itself is keeping its own list of non-fatal errors and may also be a
//! subfunction to some other caller, and so-on-and-so-forth. In this case, we no longer have
//! a simple chain of errors, but instead we have a tree of errors -- Each level in the tree
//! contains all the non-fatal errors that occurred during execution of a function and all
//! subfunctions in its call graph.
//!
//! # Solution
//!
//! The main behavior we want is captured by the [WriteErrorList] trait in this crate. It can be
//! passed as a parameter to any function that wants to be able to report non-fatal errors to its
//! caller, and it gives the caller flexibility to decide what it wants to do with that
//! information.
//!
//! The main concrete type in this crate is [ErrorList], which stores a list of a single type of
//! error. Any time a list of errors needs to be stored in memory, this is the type to use. It will
//! usually be created by the top-level caller using [ErrorList::default], and any subfunction will
//! give an [ErrorList] of its own error type to the `map_fn` that was passed in by its caller upon
//! return.
//!
//! However, [ErrorList] should rarely be passed as a parameter to a function, as that wouldn't
//! provide the caller with the flexiblity to decide what strategy it actually wants
//! to use when collecting its subfunction's non-fatal errors. The caller may want to pass direct
//! reference to its own error list, it may want to pass a [Sublist] type that automatically
//! pushes the subfunction's error list to its own error list after mapping, or it may want to
//! pass the [DontCare] type if it doesn't want to know anything about the
//! subfunction's non-fatal errors.
//!
//! Instead, subfunctions should take `impl WriteErrorList<E>` as a parameter.
//! This allows any of those types above, as well as mutable references to those types, to be
//! passed in by the caller. This also allows future caller strategies to be implemented, like
//! a caller that only cares how many non-fatal errors occurred but doesn't care about the details.
//!
//! # Serde
//!
//! (This section only applies if the `serde` feature is enabled)
//!
//! [ErrorList] implements the `Serialize` trait if the errors it contains do, and
//! likewise with the `Deserialize` trait. This means that if every error type in the tree
//! implements these traits then the entire tree can be sent over the wire and recreated elsewhere.
//! Very useful if the errors are to be examined remotely!

use {
    std::{
        error::Error,
        fmt::{self, Debug, Display, Formatter},
    },
    strategy::*,
};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod strategy;

/// Types that are capable of having errors and sublists of errors pushed onto them
///
/// This is the main trait that allows a function to record a list of non-fatal errors it
/// encounters during its execution. Generally, the [WriteErrorList::push] method will be used when
/// such an error occurs to record the error and any relevant information.
///
/// Often, a function will want to call a subfunction and add any non-fatal errors encountered
/// to its own list of errors. There are 2 strategies it could use:
///
/// 1.  **Let the subfunction directly push onto its error list** For functions that are at the same
///     level of abstraction and use the same error type, it might make the most sense for them
///     to just share an error list. In this case, simply pass a mutable reference to the error
///     list. For any type that implements this trait, a mutable reference to it implements the
///     trait too. This allows a single function to be a composition of a bunch of functions that
///     each share a single flat error list.
///
/// 2.  **Map the subfunction's error list to the caller** For a subfunction that is at a different
///     level of abstraction than the caller and uses its own error type, this makes the most sense;
///     consume the subfunction's entire error list and store it as a single error of the
///     caller's higher-level error type. Of course, those subfunctions may implement this same
///     strategy for subfunctions they call, creating a hierarchy of errors.
///
///     In this case, call the [WriteErrorList::subwriter()] function as a parameter to
///     the subfunction. If you need to manipulate the list after the subfunction has returned,
///     instead call [WriteErrorList::sublist] and pass a mutable reference as a parameter.
///
/// Function parameters should always prefer to take an object by this trait, and should rarely
/// take parameters as concrete types like [ErrorList] or [Sublist].
/// Doing so would prevent callers from being able to decide what strategy they want to use
/// to merge the subfunction's errors with its own, and would also prevent them from using the
/// [DontCare] call if they want to opt out of receiving non-fatal error information.
///
/// Passing by this trait may also help prevent logic errors: Directly passing a [Sublist] allows
/// the subfunction to query the contents of the list it's passed. Functions may incorrectly rely on
/// the fact that they are always passed an empty list, and will suddenly break if that assumption
/// doesn't hold.
pub trait WriteErrorList<E>: Sized + private::Sealed<E> {
    /// Add an error to the list of errors
    fn push(&mut self, error: E);
    /// Create a new mapping error writer with this as its parent
    ///
    /// Creates a error writer for use by a subfunction. When the subfunction is finished,
    /// either by explicitly calling [WriteErrorList::finish] or by letting it drop, the list
    /// of errors it has written using [WriteErrorList::push] will be passed as an
    /// [`ErrorList<SubErr>`][ErrorList] to the given `map_fn`, which is expected to map it to
    /// our error type, `E`.
    ///
    /// Use of this function should always be preferred to [WriteErrorList::sublist] when the
    /// caller does not need to inspect or manipulate the list returned by the subfunction and
    /// simply wants to pass it upward to its own caller, as this function will pass forward
    /// alternate strategies for collecting the errors, like [DontCare] (which turns
    /// [WriteErrorList::push] into a no-op). In constrast, [WriteErrorList::sublist] actually
    /// materializes a list that will collect all the errors of all the lists below it, even
    /// if the caller above it passed in a [DontCare].
    fn subwriter<'sub, SubMapFn, SubErr: 'sub>(
        &'sub mut self,
        map_fn: SubMapFn,
    ) -> impl WriteErrorList<SubErr> + 'sub
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E + 'sub;
    /// Start a new error list with this error list as its parent
    ///
    /// This works in a very similar manner to [WriteErrorList::subwriter], but it materializes
    /// an actual concrete [Sublist] type. This function
    /// should only be used if the function needs to be able to inspect or manipulate the errors
    /// returned by the subfunction, as it always collects all errors written by the subfunction's
    /// call graph. Otherwise, [WriteErrorList::subwriter] should be used.
    fn sublist<SubMapFn, SubErr>(
        &mut self,
        map_fn: SubMapFn,
    ) -> Sublist<'_, SubErr, SubMapFn, Self, E>
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E,
    {
        Sublist::new(map_fn, self)
    }
    /// Finish this error list
    ///
    /// This doesn't normally need to be called, as the [Drop] implementation will take care of
    /// all the details of cleaning up and ensuring that sublists are mapped up to their parent.
    ///
    /// This is mostly useful when a caller maintains a binding to a subfunction's error list
    /// and passes it by mutable reference instead of by value. Before the caller can continue
    /// to use its own error list, the sublist must release its exclusive reference.
    ///
    /// This function simply calls [drop()], but it's just a bit more clear about the intent.
    fn finish(self) {
        drop(self)
    }
}

impl<E, T: WriteErrorList<E>> private::Sealed<E> for &mut T {}

impl<E, T: WriteErrorList<E>> WriteErrorList<E> for &mut T {
    fn push(&mut self, error: E) {
        WriteErrorList::push(*self, error)
    }
    fn subwriter<'sub, SubMapFn, SubErr: 'sub>(
        &'sub mut self,
        map_fn: SubMapFn,
    ) -> impl WriteErrorList<SubErr> + 'sub
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E + 'sub,
    {
        WriteErrorList::subwriter(*self, map_fn)
    }
}

/// The main type that holds a list of errors.
///
/// See the module-level docs and the docs for [WriteErrorList].
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ErrorList<E> {
    errors: Vec<E>,
}

#[cfg(feature = "serde")]
impl<E: Serialize> Serialize for ErrorList<E> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Serialize::serialize(&self.errors, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, E: Deserialize<'de>> Deserialize<'de> for ErrorList<E> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ErrorList {
            errors: Deserialize::deserialize(deserializer)?,
        })
    }
}

impl<E> ErrorList<E> {
    /// Returns whether the error list is empty
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }
    /// Return the length of the error list
    pub fn len(&self) -> usize {
        self.errors.len()
    }
    /// Iterate the error list, returning immutable references
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a E>
    where
        E: 'a,
    {
        self.errors.iter()
    }
    /// Iterate the error list, returning mutable references
    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut E>
    where
        E: 'a,
    {
        self.errors.iter_mut()
    }
}

impl<E> private::Sealed<E> for ErrorList<E> {}

impl<E> WriteErrorList<E> for ErrorList<E> {
    fn push(&mut self, error: E) {
        self.errors.push(error);
    }
    fn subwriter<'sub, SubMapFn, SubErr: 'sub>(
        &'sub mut self,
        map_fn: SubMapFn,
    ) -> impl WriteErrorList<SubErr> + 'sub
    where
        SubMapFn: FnOnce(ErrorList<SubErr>) -> E + 'sub,
    {
        self.sublist(map_fn)
    }
}

impl<E> Default for ErrorList<E> {
    fn default() -> Self {
        Self { errors: Vec::new() }
    }
}

impl<E: Error> Display for ErrorList<E> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "one or more errors occurred:")?;
        writeln!(f)?;
        for (i, e) in self.errors.iter().enumerate() {
            writeln!(f, "  {i}:")?;

            for line in e.to_string().lines() {
                writeln!(f, "    {line}")?;
            }

            writeln!(f)?;

            let mut source = e.source();
            while let Some(e) = source {
                writeln!(f, "    caused by:")?;

                for line in e.to_string().lines() {
                    writeln!(f, "      {line}")?;
                }

                writeln!(f)?;

                source = e.source();
            }
        }
        Ok(())
    }
}

impl<E: Error> Error for ErrorList<E> {}

impl<E> IntoIterator for ErrorList<E> {
    type Item = <Vec<E> as IntoIterator>::Item;
    type IntoIter = <Vec<E> as IntoIterator>::IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}

mod private {
    /// Prevent users of this crate from implementing traits for their own types
    pub trait Sealed<E> {}
}
