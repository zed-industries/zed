/*! Overload resolution for builtin functions.

This module defines the [`OverloadSet`] trait, which provides methods the
validator and typifier can use to check the types to builtin functions,
determine their result types, and produce diagnostics that explain why a given
application is not allowed and suggest fixes.

You can call [`MathFunction::overloads`] to obtain an `impl OverloadSet`
representing the given `MathFunction`'s overloads.

[`MathFunction::overloads`]: crate::ir::MathFunction::overloads

*/

mod constructor_set;
mod regular;
mod scalar_set;

mod any_overload_set;
mod list;
mod mathfunction;
mod one_bits_iter;
mod rule;
mod utils;

pub use rule::{Conclusion, MissingSpecialType, Rule};

use crate::ir;
use crate::proc::TypeResolution;

use alloc::vec::Vec;
use core::fmt;

#[expect(rustdoc::private_intra_doc_links)]
/// A trait for types representing of a set of Naga IR type rules.
///
/// Given an expression like `max(x, y)`, there are multiple type rules that
/// could apply, depending on the types of `x` and `y`, like:
///
/// - `max(i32, i32) -> i32`
/// - `max(vec4<f32>, vec4<f32>) -> vec4<f32>`
///
/// and so on. Borrowing WGSL's terminology, Naga calls the full set of type
/// rules that might apply to a given expression its "overload candidates", or
/// "overloads" for short.
///
/// This trait is meant to be implemented by types that represent a set of
/// overload candidates. For example, [`MathFunction::overloads`] returns an
/// `impl OverloadSet` describing the overloads for the given Naga IR math
/// function. Naga's typifier, validator, and WGSL front end use this trait for
/// their work.
///
/// [`MathFunction::overloads`]: ir::MathFunction::overloads
///
/// # Automatic conversions
///
/// In principle, overload sets are easy: you simply list all the overloads the
/// function supports, and then when you're presented with a call to typecheck,
/// you just see if the actual argument types presented in the source code match
/// some overload from the list.
///
/// However, Naga supports languages like WGSL, which apply certain [automatic
/// conversions] if necessary to make a call fit some overload's requirements.
/// This means that the set of calls that are effectively allowed by a given set
/// of overloads can be quite large, since any combination of automatic
/// conversions might be applied.
///
/// For example, if `x` is a `u32`, and `100` is an abstract integer, then even
/// though `max` has no overload like `max(u32, AbstractInt) -> ...`, the
/// expression `max(x, 100)` is still allowed, because AbstractInt automatically
/// converts to `u32`.
///
/// [automatic conversions]: https://gpuweb.github.io/gpuweb/wgsl/#feasible-automatic-conversion
///
/// # How to use `OverloadSet`
///
/// The general process of using an `OverloadSet` is as follows:
///
/// - Obtain an `OverloadSet` for a given operation (say, by calling
///   [`MathFunction::overloads`]). This set is fixed by Naga IR's semantics.
///
/// - Call its [`arg`] method, supplying the type of the argument passed to the
///   operation at a certain index. This returns a new `OverloadSet` containing
///   only those overloads that could accept the given type for the given
///   argument. This includes overloads that only match if automatic conversions
///   are applied.
///
/// - If, at some point, the overload set becomes empty, then the set of
///   arguments is not allowed for this operation, and the program is invalid.
///   The `OverloadSet` trait provides an [`is_empty`] method.
///
/// - After all arguments have been supplied, if the overload set is still
///   non-empty, you can call its [`most_preferred`] method to find out which
///   overload has the least cost in terms of automatic conversions.
///
/// - If the call is rejected, you can use `OverloadSet` to help produce
///   diagnostic messages that explain exactly what went wrong. `OverloadSet`
///   implementations are meant to be cheap to [`Clone`], so it is fine to keep
///   the original overload set value around, and re-run the selection process,
///   attempting to supply the rejected argument at each step to see exactly
///   which preceding argument ruled it out. The [`overload_list`] and
///   [`allowed_args`] methods are helpful for this.
///
/// [`arg`]: OverloadSet::arg
/// [`is_empty`]: OverloadSet::is_empty
/// [`most_preferred`]: OverloadSet::most_preferred
/// [`overload_list`]: OverloadSet::overload_list
/// [`allowed_args`]: OverloadSet::allowed_args
///
/// # Concrete implementations
///
/// This module contains two private implementations of `OverloadSet`:
///
/// - The [`List`] type is a straightforward list of overloads. It is general,
///   but verbose to use. The [`utils`] module exports functions that construct
///   `List` overload sets for the functions that need this.
///
/// - The [`Regular`] type is a compact, efficient representation for the kinds
///   of overload sets commonly seen for Naga IR mathematical functions.
///   However, in return for its simplicity, it is not as flexible as [`List`].
///   This module use the [`regular!`] macro as a legible notation for `Regular`
///   sets.
///
/// [`List`]: list::List
/// [`Regular`]: regular::Regular
/// [`regular!`]: regular::regular
pub trait OverloadSet: Clone {
    /// Return true if `self` is the empty set of overloads.
    fn is_empty(&self) -> bool;

    /// Return the smallest number of arguments in any type rule in the set.
    ///
    /// # Panics
    ///
    /// Panics if `self` is empty.
    fn min_arguments(&self) -> usize;

    /// Return the largest number of arguments in any type rule in the set.
    ///
    /// # Panics
    ///
    /// Panics if `self` is empty.
    fn max_arguments(&self) -> usize;

    /// Find the overloads that could accept a given argument.
    ///
    /// Return a new overload set containing those members of `self` that could
    /// accept a value of type `ty` for their `i`'th argument, once
    /// feasible automatic conversions have been applied.
    fn arg(&self, i: usize, ty: &ir::TypeInner, types: &crate::UniqueArena<ir::Type>) -> Self;

    /// Limit `self` to overloads whose arguments are all concrete types.
    ///
    /// Naga's overload resolution is based on WGSL's [overload resolution
    /// algorithm][ora], which includes the following step:
    ///
    /// > Eliminate any candidate where one of its subexpressions resolves to
    /// > an abstract type after feasible automatic conversions, but another of
    /// > the candidate’s subexpressions is not a const-expression.
    /// >
    /// > Note: As a consequence, if any subexpression in the phrase is not a
    /// > const-expression, then all subexpressions in the phrase must have a
    /// > concrete type.
    ///
    /// Essentially, if any one of the arguments is not a constant expression,
    /// then the operation is going to be evaluated at runtime, so all its
    /// arguments must be converted to a concrete type. If you determine that an
    /// argument is non-constant, you can use this trait method to toss out any
    /// overloads that would accept abstract types.
    ///
    /// In almost all cases, this operation has no effect. Only constant
    /// expressions can have abstract types, so if any argument is not a
    /// constant expression, it must have a concrete type. Although many
    /// operations in Naga IR have overloads for both abstract types and
    /// concrete types, few operations have overloads that accept a mix of
    /// abstract and concrete types. Thus, a single concrete argument will
    /// usually have eliminated all overloads that accept abstract types anyway.
    /// (The exceptions are oddities like `Expression::Select`, where the
    /// `condition` operand could be a runtime expression even as the `accept`
    /// and `reject` operands have abstract types.)
    ///
    /// Note that it is *not* correct to just [concretize] all arguments once
    /// you've noticed that some argument is non-constant. The type to which
    /// each argument is converted depends on the overloads available, not just
    /// the argument's own type.
    ///
    /// [ora]: https://gpuweb.github.io/gpuweb/wgsl/#overload-resolution-section
    /// [concretize]: https://gpuweb.github.io/gpuweb/wgsl/#concretization
    fn concrete_only(self, types: &crate::UniqueArena<ir::Type>) -> Self;

    /// Return the most preferred candidate.
    ///
    /// Rank the candidates in `self` as described in WGSL's [overload
    /// resolution algorithm][ora], and return a singleton set containing the
    /// most preferred candidate.
    ///
    /// # Simplifications versus WGSL
    ///
    /// Naga's process for selecting the most preferred candidate is simpler
    /// than WGSL's:
    ///
    /// - WGSL allows for the possibility of ambiguous calls, where multiple
    ///   overload candidates exist, no one candidate is clearly better than all
    ///   the others. For example, if a function has the two overloads `(i32,
    ///   f32) -> bool` and `(f32, i32) -> bool`, and the arguments are both
    ///   AbstractInt, neither overload is preferred over the other. Ambiguous
    ///   calls are errors.
    ///
    ///   However, Naga IR includes no operations whose overload sets allow such
    ///   situations to arise, so there is always a most preferred candidate.
    ///   Thus, this method infallibly returns a `Rule`, and has no way to
    ///   indicate ambiguity.
    ///
    /// - WGSL says that the most preferred candidate depends on the conversion
    ///   rank for each argument, as determined by the types of the arguments
    ///   being passed.
    ///
    ///   However, the overloads of every operation in Naga IR can be ranked
    ///   even without knowing the argument types. So this method does not
    ///   require the argument types as a parameter.
    ///
    /// # Panics
    ///
    /// Panics if `self` is empty, or if no argument types have been supplied.
    ///
    /// [ora]: https://gpuweb.github.io/gpuweb/wgsl/#overload-resolution-section
    fn most_preferred(&self) -> Rule;

    /// Return a type rule for each of the overloads in `self`.
    fn overload_list(&self, gctx: &crate::proc::GlobalCtx<'_>) -> Vec<Rule>;

    /// Return a list of the types allowed for argument `i`.
    fn allowed_args(&self, i: usize, gctx: &crate::proc::GlobalCtx<'_>) -> Vec<TypeResolution>;

    /// Return an object that can be formatted with [`core::fmt::Debug`].
    fn for_debug(&self, types: &crate::UniqueArena<ir::Type>) -> impl fmt::Debug;
}
