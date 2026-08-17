/*! A representation for highly regular overload sets common in Naga IR.

Many Naga builtin functions' overload sets have a highly regular
structure. For example, many arithmetic functions can be applied to
any floating-point type, or any vector thereof. This module defines a
handful of types for representing such simple overload sets that is
simple and efficient.

*/

use crate::common::{DiagnosticDebug, ForDebugWithTypes};
use crate::ir;
use crate::proc::overloads::constructor_set::{ConstructorSet, ConstructorSize};
use crate::proc::overloads::rule::{Conclusion, Rule};
use crate::proc::overloads::scalar_set::ScalarSet;
use crate::proc::overloads::OverloadSet;
use crate::proc::{GlobalCtx, TypeResolution};
use crate::UniqueArena;

use alloc::vec::Vec;
use core::fmt;

/// Overload sets represented as sets of scalars and constructors.
///
/// This type represents an [`OverloadSet`] using a bitset of scalar
/// types and a bitset of type constructors that might be applied to
/// those scalars. The overload set contains a rule for every possible
/// combination of scalars and constructors, essentially the cartesian
/// product of the two sets.
///
/// For example, if the arity is 2, set of scalars is { AbstractFloat,
/// `f32` }, and the set of constructors is { `vec2`, `vec3` }, then
/// that represents the set of overloads:
///
/// - (`vec2<AbstractFloat>`, `vec2<AbstractFloat>`) -> `vec2<AbstractFloat>`
/// - (`vec2<f32>`, `vec2<f32>`) -> `vec2<f32>`
/// - (`vec3<AbstractFloat>`, `vec3<AbstractFloat>`) -> `vec3<AbstractFloat>`
/// - (`vec3<f32>`, `vec3<f32>`) -> `vec3<f32>`
///
/// The `conclude` value says how to determine the return type from
/// the argument type.
///
/// Restrictions:
///
/// - All overloads must take the same number of arguments.
///
/// - For any given overload, all its arguments must have the same
///   type.
#[derive(Clone)]
pub(in crate::proc::overloads) struct Regular {
    /// The number of arguments in the rules.
    pub arity: usize,

    /// The set of type constructors to apply.
    pub constructors: ConstructorSet,

    /// The set of scalars to apply them to.
    pub scalars: ScalarSet,

    /// How to determine a member rule's return type given the type of
    /// its arguments.
    pub conclude: ConclusionRule,
}

impl Regular {
    pub(in crate::proc::overloads) const EMPTY: Regular = Regular {
        arity: 0,
        constructors: ConstructorSet::empty(),
        scalars: ScalarSet::empty(),
        conclude: ConclusionRule::ArgumentType,
    };

    /// Return an iterator over all the argument types allowed by `self`.
    ///
    /// Return an iterator that produces, for each overload in `self`, the
    /// constructor and scalar of its argument types and return type.
    ///
    /// A [`Regular`] value can only represent overload sets where, in
    /// each overload, all the arguments have the same type, and the
    /// return type is always going to be a determined by the argument
    /// types, so giving the constructor and scalar is sufficient to
    /// characterize the entire rule.
    fn members(&self) -> impl Iterator<Item = (ConstructorSize, ir::Scalar)> {
        let scalars = self.scalars;
        self.constructors.members().flat_map(move |constructor| {
            let size = constructor.size();
            // Technically, we don't need the "most general" `TypeInner` here,
            // but since `ScalarSet::members` only produces singletons anyway,
            // the effect is the same.
            scalars
                .members()
                .map(move |singleton| (size, singleton.most_general_scalar()))
        })
    }

    fn rules(&self) -> impl Iterator<Item = Rule> {
        let arity = self.arity;
        let conclude = self.conclude;
        self.members()
            .map(move |(size, scalar)| make_rule(arity, size, scalar, conclude))
    }
}

impl OverloadSet for Regular {
    fn is_empty(&self) -> bool {
        self.constructors.is_empty() || self.scalars.is_empty()
    }

    fn min_arguments(&self) -> usize {
        assert!(!self.is_empty());
        self.arity
    }

    fn max_arguments(&self) -> usize {
        assert!(!self.is_empty());
        self.arity
    }

    fn arg(&self, i: usize, ty: &ir::TypeInner, types: &UniqueArena<ir::Type>) -> Self {
        if i >= self.arity {
            return Self::EMPTY;
        }

        let constructor = ConstructorSet::singleton(ty);

        let scalars = match ty.scalar_for_conversions(types) {
            Some(ty_scalar) => ScalarSet::convertible_from(ty_scalar),
            None => ScalarSet::empty(),
        };

        Self {
            arity: self.arity,

            // Constrain all member rules' constructors to match `ty`'s.
            constructors: self.constructors & constructor,

            // Constrain all member rules' arguments to be something
            // that `ty` can be converted to.
            scalars: self.scalars & scalars,

            conclude: self.conclude,
        }
    }

    fn concrete_only(self, _types: &UniqueArena<ir::Type>) -> Self {
        Self {
            scalars: self.scalars & ScalarSet::CONCRETE,
            ..self
        }
    }

    fn most_preferred(&self) -> Rule {
        assert!(!self.is_empty());

        // If there is more than one constructor allowed, then we must
        // not have had any arguments supplied at all. In any case, we
        // don't have any unambiguously preferred candidate.
        assert!(self.constructors.is_singleton());

        let size = self.constructors.size();
        let scalar = self.scalars.most_general_scalar();
        make_rule(self.arity, size, scalar, self.conclude)
    }

    fn overload_list(&self, _gctx: &GlobalCtx<'_>) -> Vec<Rule> {
        self.rules().collect()
    }

    fn allowed_args(&self, i: usize, _gctx: &GlobalCtx<'_>) -> Vec<TypeResolution> {
        if i >= self.arity {
            return Vec::new();
        }
        self.members()
            .map(|(size, scalar)| TypeResolution::Value(size.to_inner(scalar)))
            .collect()
    }

    fn for_debug(&self, types: &UniqueArena<ir::Type>) -> impl fmt::Debug {
        DiagnosticDebug((self, types))
    }
}

/// Construct a [`Regular`] member [`Rule`] for the given arity and type.
///
/// [`Regular`] can only represent rules where all the argument types and the
/// return type are the same, so just knowing `arity` and `inner` is sufficient.
///
/// [`Rule`]: crate::proc::overloads::Rule
fn make_rule(
    arity: usize,
    size: ConstructorSize,
    scalar: ir::Scalar,
    conclusion_rule: ConclusionRule,
) -> Rule {
    let inner = size.to_inner(scalar);
    let arg = TypeResolution::Value(inner.clone());
    Rule {
        arguments: core::iter::repeat_n(arg.clone(), arity).collect(),
        conclusion: conclusion_rule.conclude(size, scalar),
    }
}

/// Conclusion-computing rules.
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub(in crate::proc::overloads) enum ConclusionRule {
    ArgumentType,
    Scalar,
    Frexp,
    Modf,
    U32,
    I32,
    Vec2F,
    Vec4F,
    Vec4I,
    Vec4U,
}

impl ConclusionRule {
    fn conclude(self, size: ConstructorSize, scalar: ir::Scalar) -> Conclusion {
        match self {
            Self::ArgumentType => Conclusion::Value(size.to_inner(scalar)),
            Self::Scalar => Conclusion::Value(ir::TypeInner::Scalar(scalar)),
            Self::Frexp => Conclusion::for_frexp_modf(ir::MathFunction::Frexp, size, scalar),
            Self::Modf => Conclusion::for_frexp_modf(ir::MathFunction::Modf, size, scalar),
            Self::U32 => Conclusion::Value(ir::TypeInner::Scalar(ir::Scalar::U32)),
            Self::I32 => Conclusion::Value(ir::TypeInner::Scalar(ir::Scalar::I32)),
            Self::Vec2F => Conclusion::Value(ir::TypeInner::Vector {
                size: ir::VectorSize::Bi,
                scalar: ir::Scalar::F32,
            }),
            Self::Vec4F => Conclusion::Value(ir::TypeInner::Vector {
                size: ir::VectorSize::Quad,
                scalar: ir::Scalar::F32,
            }),
            Self::Vec4I => Conclusion::Value(ir::TypeInner::Vector {
                size: ir::VectorSize::Quad,
                scalar: ir::Scalar::I32,
            }),
            Self::Vec4U => Conclusion::Value(ir::TypeInner::Vector {
                size: ir::VectorSize::Quad,
                scalar: ir::Scalar::U32,
            }),
        }
    }
}

impl fmt::Debug for DiagnosticDebug<(&Regular, &UniqueArena<ir::Type>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (regular, types) = self.0;
        let rules: Vec<Rule> = regular.rules().collect();
        f.debug_struct("List")
            .field("rules", &rules.for_debug(types))
            .field("conclude", &regular.conclude)
            .finish()
    }
}

impl ForDebugWithTypes for &Regular {}

impl fmt::Debug for DiagnosticDebug<(&[Rule], &UniqueArena<ir::Type>)> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rules, types) = self.0;
        f.debug_list()
            .entries(rules.iter().map(|rule| rule.for_debug(types)))
            .finish()
    }
}

impl ForDebugWithTypes for &[Rule] {}

/// Construct a [`Regular`] [`OverloadSet`].
///
/// Examples:
///
/// - `regular!(2, SCALAR|VECN of FLOAT)`: An overload set whose rules take two
///   arguments of the same type: a floating-point scalar (possibly abstract) or
///   a vector of such. The return type is the same as the argument type.
///
/// - `regular!(1, VECN of FLOAT -> Scalar)`: An overload set whose rules take
///   one argument that is a vector of floats, and whose return type is the leaf
///   scalar type of the argument type.
///
/// The constructor values (before the `<` angle brackets `>`) are
/// constants from [`ConstructorSet`].
///
/// The scalar values (inside the `<` angle brackets `>`) are
/// constants from [`ScalarSet`].
///
/// When a return type identifier is given, it is treated as a variant
/// of the the [`ConclusionRule`] enum.
macro_rules! regular {
    // regular!(ARITY, CONSTRUCTOR of SCALAR)
    ( $arity:literal , $( $constr:ident )|* of $( $scalar:ident )|*) => {
        {
            use $crate::proc::overloads;
            use overloads::constructor_set::constructor_set;
            use overloads::regular::{Regular, ConclusionRule};
            use overloads::scalar_set::scalar_set;
            Regular {
                arity: $arity,
                constructors: constructor_set!( $( $constr )|* ),
                scalars: scalar_set!( $( $scalar )|* ),
                conclude: ConclusionRule::ArgumentType,
            }
        }
    };

    // regular!(ARITY, CONSTRUCTOR of SCALAR -> CONCLUSION_RULE)
    ( $arity:literal , $( $constr:ident )|* of $( $scalar:ident )|* -> $conclude:ident) => {
        {
            use $crate::proc::overloads;
            use overloads::constructor_set::constructor_set;
            use overloads::regular::{Regular, ConclusionRule};
            use overloads::scalar_set::scalar_set;
            Regular {
                arity: $arity,
                constructors:constructor_set!( $( $constr )|* ),
                scalars: scalar_set!( $( $scalar )|* ),
                conclude: ConclusionRule::$conclude,
            }
        }
    };
}

pub(in crate::proc::overloads) use regular;

#[cfg(test)]
mod test {
    use super::*;
    use crate::ir;

    const fn scalar(scalar: ir::Scalar) -> ir::TypeInner {
        ir::TypeInner::Scalar(scalar)
    }

    const fn vec2(scalar: ir::Scalar) -> ir::TypeInner {
        ir::TypeInner::Vector {
            scalar,
            size: ir::VectorSize::Bi,
        }
    }

    const fn vec3(scalar: ir::Scalar) -> ir::TypeInner {
        ir::TypeInner::Vector {
            scalar,
            size: ir::VectorSize::Tri,
        }
    }

    /// Assert that `set` has a most preferred candidate whose type
    /// conclusion is `expected`.
    #[track_caller]
    fn check_return_type(set: &Regular, expected: &ir::TypeInner, arena: &UniqueArena<ir::Type>) {
        assert!(!set.is_empty());

        let special_types = ir::SpecialTypes::default();

        let preferred = set.most_preferred();
        let conclusion = preferred.conclusion;
        let resolution = conclusion
            .into_resolution(&special_types)
            .expect("special types should have been pre-registered");
        let inner = resolution.inner_with(arena);

        assert!(
            inner.non_struct_equivalent(expected, arena),
            "Expected {:?}, got {:?}",
            expected.for_debug(arena),
            inner.for_debug(arena),
        );
    }

    #[test]
    fn unary_vec_or_scalar_numeric_scalar() {
        let arena = UniqueArena::default();

        let builtin = regular!(1, SCALAR of NUMERIC);

        let ok = builtin.arg(0, &scalar(ir::Scalar::U32), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::U32), &arena);

        let err = builtin.arg(0, &scalar(ir::Scalar::BOOL), &arena);
        assert!(err.is_empty());
    }

    #[test]
    fn unary_vec_or_scalar_numeric_vector() {
        let arena = UniqueArena::default();

        let builtin = regular!(1, VECN|SCALAR of NUMERIC);

        let ok = builtin.arg(0, &vec3(ir::Scalar::F64), &arena);
        check_return_type(&ok, &vec3(ir::Scalar::F64), &arena);

        let err = builtin.arg(0, &vec3(ir::Scalar::BOOL), &arena);
        assert!(err.is_empty());
    }

    #[test]
    fn unary_vec_or_scalar_numeric_matrix() {
        let arena = UniqueArena::default();

        let builtin = regular!(1, VECN|SCALAR of NUMERIC);

        let err = builtin.arg(
            0,
            &ir::TypeInner::Matrix {
                columns: ir::VectorSize::Tri,
                rows: ir::VectorSize::Tri,
                scalar: ir::Scalar::F32,
            },
            &arena,
        );
        assert!(err.is_empty());
    }

    #[test]
    #[rustfmt::skip]
    fn binary_vec_or_scalar_numeric_scalar() {
        let arena = UniqueArena::default();

        let builtin = regular!(2, VECN|SCALAR of NUMERIC);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::F32), &arena)
            .arg(1, &scalar(ir::Scalar::F32), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::F32), &arena);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::ABSTRACT_FLOAT), &arena)
            .arg(1, &scalar(ir::Scalar::F32), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::F32), &arena);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::F32), &arena)
            .arg(1, &scalar(ir::Scalar::ABSTRACT_INT), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::F32), &arena);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::U32), &arena)
            .arg(1, &scalar(ir::Scalar::U32), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::U32), &arena);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::U32), &arena)
            .arg(1, &scalar(ir::Scalar::ABSTRACT_INT), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::U32), &arena);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::ABSTRACT_INT), &arena)
            .arg(1, &scalar(ir::Scalar::U32), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::U32), &arena);

        // Not numeric.
        let err = builtin
            .arg(0, &scalar(ir::Scalar::BOOL), &arena)
            .arg(1, &scalar(ir::Scalar::BOOL), &arena);
        assert!(err.is_empty());

        // Different floating-point types.
        let err = builtin
            .arg(0, &scalar(ir::Scalar::F32), &arena)
            .arg(1, &scalar(ir::Scalar::F64), &arena);
        assert!(err.is_empty());

        // Different constructor.
        let err = builtin
            .arg(0, &scalar(ir::Scalar::F32), &arena)
            .arg(1, &vec2(ir::Scalar::F32), &arena);
        assert!(err.is_empty());

        // Different vector size
        let err = builtin
            .arg(0, &vec2(ir::Scalar::F32), &arena)
            .arg(1, &vec3(ir::Scalar::F32), &arena);
        assert!(err.is_empty());
    }

    #[test]
    #[rustfmt::skip]
    fn binary_vec_or_scalar_numeric_vector() {
        let arena = UniqueArena::default();

        let builtin = regular!(2, VECN|SCALAR of NUMERIC);

        let ok = builtin
            .arg(0, &vec3(ir::Scalar::F32), &arena)
            .arg(1, &vec3(ir::Scalar::F32), &arena);
        check_return_type(&ok, &vec3(ir::Scalar::F32), &arena);

        // Different vector sizes.
        let err = builtin
            .arg(0, &vec2(ir::Scalar::F32), &arena)
            .arg(1, &vec3(ir::Scalar::F32), &arena);
        assert!(err.is_empty());

        // Different vector scalars.
        let err = builtin
            .arg(0, &vec3(ir::Scalar::F32), &arena)
            .arg(1, &vec3(ir::Scalar::F64), &arena);
        assert!(err.is_empty());

        // Mix of vectors and scalars.
        let err = builtin
            .arg(0, &scalar(ir::Scalar::F32), &arena)
            .arg(1, &vec3(ir::Scalar::F32), &arena);
        assert!(err.is_empty());
    }

    #[test]
    #[rustfmt::skip]
    fn binary_vec_or_scalar_numeric_vector_abstract() {
        let arena = UniqueArena::default();

        let builtin = regular!(2, VECN|SCALAR of NUMERIC);

        let ok = builtin
            .arg(0, &vec2(ir::Scalar::ABSTRACT_INT), &arena)
            .arg(1, &vec2(ir::Scalar::U32), &arena);
        check_return_type(&ok, &vec2(ir::Scalar::U32), &arena);

        let ok = builtin
            .arg(0, &vec3(ir::Scalar::ABSTRACT_INT), &arena)
            .arg(1, &vec3(ir::Scalar::F32), &arena);
        check_return_type(&ok, &vec3(ir::Scalar::F32), &arena);

        let ok = builtin
            .arg(0, &scalar(ir::Scalar::ABSTRACT_FLOAT), &arena)
            .arg(1, &scalar(ir::Scalar::F32), &arena);
        check_return_type(&ok, &scalar(ir::Scalar::F32), &arena);

        let err = builtin
            .arg(0, &scalar(ir::Scalar::ABSTRACT_FLOAT), &arena)
            .arg(1, &scalar(ir::Scalar::U32), &arena);
        assert!(err.is_empty());

        let err = builtin
            .arg(0, &scalar(ir::Scalar::I32), &arena)
            .arg(1, &scalar(ir::Scalar::U32), &arena);
        assert!(err.is_empty());
    }
}
