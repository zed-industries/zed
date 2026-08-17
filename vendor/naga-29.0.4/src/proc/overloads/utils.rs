//! Utility functions for constructing [`List`] overload sets.
//!
//! [`List`]: crate::proc::overloads::list::List

use crate::ir;
use crate::proc::overloads::list::List;
use crate::proc::overloads::rule::{Conclusion, Rule};
use crate::proc::TypeResolution;

use alloc::vec::Vec;

/// Produce all the floating-point [`ir::Scalar`]s.
///
/// Note that `F32` must appear before other sizes; this is how we
/// represent conversion rank.
pub fn float_scalars() -> impl Iterator<Item = ir::Scalar> + Clone {
    [
        ir::Scalar::ABSTRACT_FLOAT,
        ir::Scalar::F32,
        ir::Scalar::F16,
        ir::Scalar::F64,
    ]
    .into_iter()
}

/// Produce all the floating-point [`ir::Scalar`]s, but omit
/// abstract types, for #7405.
pub fn float_scalars_unimplemented_abstract() -> impl Iterator<Item = ir::Scalar> + Clone {
    [ir::Scalar::F32, ir::Scalar::F16, ir::Scalar::F64].into_iter()
}

/// Produce the scalar and vector [`ir::TypeInner`]s that have `s` as
/// their scalar.
pub fn scalar_or_vecn(scalar: ir::Scalar) -> impl Iterator<Item = ir::TypeInner> {
    [
        ir::TypeInner::Scalar(scalar),
        ir::TypeInner::Vector {
            size: ir::VectorSize::Bi,
            scalar,
        },
        ir::TypeInner::Vector {
            size: ir::VectorSize::Tri,
            scalar,
        },
        ir::TypeInner::Vector {
            size: ir::VectorSize::Quad,
            scalar,
        },
    ]
    .into_iter()
}

/// Construct a [`Rule`] for an operation with the given
/// argument types and return type.
pub fn rule<const N: usize>(args: [ir::TypeInner; N], ret: ir::TypeInner) -> Rule {
    Rule {
        arguments: Vec::from_iter(args.into_iter().map(TypeResolution::Value)),
        conclusion: Conclusion::Value(ret),
    }
}

/// Construct a [`List`] from the given rules.
pub fn list(rules: impl Iterator<Item = Rule>) -> List {
    List::from_rules(rules.collect())
}

/// Return the cartesian product of two iterators.
pub fn pairs<T: Clone, U>(
    left: impl Iterator<Item = T>,
    right: impl Iterator<Item = U> + Clone,
) -> impl Iterator<Item = (T, U)> {
    left.flat_map(move |t| right.clone().map(move |u| (t.clone(), u)))
}

/// Return the cartesian product of three iterators.
pub fn triples<T: Clone, U: Clone, V>(
    left: impl Iterator<Item = T>,
    middle: impl Iterator<Item = U> + Clone,
    right: impl Iterator<Item = V> + Clone,
) -> impl Iterator<Item = (T, U, V)> {
    left.flat_map(move |t| {
        let right = right.clone();
        middle.clone().flat_map(move |u| {
            let t = t.clone();
            right.clone().map(move |v| (t.clone(), u.clone(), v))
        })
    })
}
