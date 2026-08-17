// Code in this file intentionally uses `for` loops and `.push()` rather than
// `ArrayVec::from_iter`, because the latter is monomorphized by all three of
// the item type, the capacity, and the iterator type, which can easily bloat
// the compiled executable (by ~260 KiB, when it was removed).

use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use core::iter;

use arrayvec::ArrayVec;
use half::f16;
use num_traits::{real::Real, FromPrimitive, One, ToPrimitive, Zero};

use crate::{
    arena::{Arena, Handle, HandleVec, UniqueArena},
    ArraySize, BinaryOperator, Constant, Expression, Literal, Override, RelationalFunction,
    ScalarKind, Span, Type, TypeInner, UnaryOperator,
};

#[cfg(feature = "wgsl-in")]
use crate::common::wgsl::TryToWgsl;

/// A macro that allows dollar signs (`$`) to be emitted by other macros. Useful for generating
/// `macro_rules!` items that, in turn, emit their own `macro_rules!` items.
///
/// Technique stolen directly from
/// <https://github.com/rust-lang/rust/issues/35853#issuecomment-415993963>.
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

macro_rules! gen_component_wise_extractor {
    (
        $ident:ident -> $target:ident,
        literals: [$( $literal:ident => $mapping:ident: $ty:ident ),+ $(,)?],
        scalar_kinds: [$( $scalar_kind:ident ),* $(,)?],
    ) => {
        /// A subset of [`Literal`]s intended to be used for implementing numeric built-ins.
        #[derive(Debug)]
        #[cfg_attr(test, derive(PartialEq))]
        enum $target<const N: usize> {
            $(
                #[doc = concat!(
                    "Maps to [`Literal::",
                    stringify!($literal),
                    "`]",
                )]
                $mapping([$ty; N]),
            )+
        }

        impl From<$target<1>> for Expression {
            fn from(value: $target<1>) -> Self {
                match value {
                    $(
                        $target::$mapping([value]) => {
                            Expression::Literal(Literal::$literal(value))
                        }
                    )+
                }
            }
        }

        #[doc = concat!(
            "Attempts to evaluate multiple `exprs` as a combined [`",
            stringify!($target),
            "`] to pass to `handler`. ",
        )]
        /// If `exprs` are vectors of the same length, `handler` is called for each corresponding
        /// component of each vector.
        ///
        /// `handler`'s output is registered as a new expression. If `exprs` are vectors of the
        /// same length, a new vector expression is registered, composed of each component emitted
        /// by `handler`.
        fn $ident<const N: usize, const M: usize>(
            eval: &mut ConstantEvaluator<'_>,
            span: Span,
            exprs: [Handle<Expression>; N],
            handler: fn($target<N>) -> Result<$target<M>, ConstantEvaluatorError>,
        ) -> Result<Handle<Expression>, ConstantEvaluatorError>
        where
            $target<M>: Into<Expression>,
        {
            assert!(N > 0);
            let err = ConstantEvaluatorError::InvalidMathArg;
            let mut exprs = exprs.into_iter();

            macro_rules! sanitize {
                ($expr:expr) => {
                    eval.eval_zero_value_and_splat($expr, span)
                        .map(|expr| &eval.expressions[expr])
                };
            }

            let new_expr: Result<Expression, ConstantEvaluatorError> = match sanitize!(exprs.next().unwrap())? {
                $(
                    &Expression::Literal(Literal::$literal(x)) => {
                        let mut arr = ArrayVec::<_, N>::new();
                        arr.push(x);
                        for expr in exprs {
                            match sanitize!(expr)? {
                                &Expression::Literal(Literal::$literal(val)) => arr.push(val),
                                _ => return Err(err),
                            }
                        }
                        let comps = $target::$mapping(arr.into_inner().unwrap());
                        Ok(handler(comps)?.into())
                    },
                )+
                &Expression::Compose { ty, ref components } => match &eval.types[ty].inner {
                    &TypeInner::Vector { size, scalar } => match scalar.kind {
                        $(ScalarKind::$scalar_kind)|* => {
                            let first_ty = ty;
                            let mut component_groups =
                                ArrayVec::<ArrayVec<_, { crate::VectorSize::MAX }>, N>::new();
                            {
                                let mut inner = ArrayVec::new();
                                for item in crate::proc::flatten_compose(
                                    first_ty,
                                    components,
                                    eval.expressions,
                                    eval.types,
                                ) {
                                    inner.push(item);
                                }
                                component_groups.push(inner);
                            }
                            for expr in exprs {
                                match sanitize!(expr)? {
                                    &Expression::Compose { ty, ref components }
                                        if &eval.types[ty].inner
                                            == &eval.types[first_ty].inner =>
                                    {
                                        let mut inner = ArrayVec::new();
                                        for item in crate::proc::flatten_compose(
                                            ty,
                                            components,
                                            eval.expressions,
                                            eval.types,
                                        ) {
                                            inner.push(item);
                                        }
                                        component_groups.push(inner);
                                    }
                                    _ => return Err(err),
                                }
                            }
                            let component_groups = component_groups.into_inner().unwrap();
                            let mut new_components =
                                ArrayVec::<_, { crate::VectorSize::MAX }>::new();
                            for idx in 0..(size as u8).into() {
                                let mut group_arr = ArrayVec::<_, N>::new();
                                for cs in component_groups.iter() {
                                    group_arr.push(
                                        cs.get(idx).cloned().ok_or_else(|| err.clone())?,
                                    );
                                }
                                let group = group_arr.into_inner().unwrap();
                                new_components.push($ident(
                                    eval,
                                    span,
                                    group,
                                    handler,
                                )?);
                            }
                            Ok(Expression::Compose {
                                ty: first_ty,
                                components: new_components.into_iter().collect(),
                            })
                        }
                        _ => return Err(err),
                    },
                    _ => return Err(err),
                },
                _ => return Err(err),
            };
            eval.register_evaluated_expr(new_expr?, span)
        }

        with_dollar_sign! {
            ($d:tt) => {
                #[allow(unused)]
                #[doc = concat!(
                    "A convenience macro for using the same RHS for each [`",
                    stringify!($target),
                    "`] variant in a call to [`",
                    stringify!($ident),
                    "`].",
                )]
                macro_rules! $ident {
                    (
                        $eval:expr,
                        $span:expr,
                        [$d ($d expr:expr),+ $d (,)?],
                        |$d ($d arg:ident),+| $d tt:tt
                    ) => {
                        $ident($eval, $span, [$d ($d expr),+], |args| match args {
                            $(
                                $target::$mapping([$d ($d arg),+]) => {
                                    let res = $d tt;
                                    Result::map(res, $target::$mapping)
                                },
                            )+
                        })
                    };
                }
            };
        }
    };
}

gen_component_wise_extractor! {
    component_wise_scalar -> Scalar,
    literals: [
        AbstractFloat => AbstractFloat: f64,
        F32 => F32: f32,
        F16 => F16: f16,
        AbstractInt => AbstractInt: i64,
        U32 => U32: u32,
        I32 => I32: i32,
        U64 => U64: u64,
        I64 => I64: i64,
    ],
    scalar_kinds: [
        Float,
        AbstractFloat,
        Sint,
        Uint,
        AbstractInt,
    ],
}

gen_component_wise_extractor! {
    component_wise_float -> Float,
    literals: [
        AbstractFloat => Abstract: f64,
        F32 => F32: f32,
        F16 => F16: f16,
    ],
    scalar_kinds: [
        Float,
        AbstractFloat,
    ],
}

gen_component_wise_extractor! {
    component_wise_concrete_int -> ConcreteInt,
    literals: [
        U32 => U32: u32,
        I32 => I32: i32,
    ],
    scalar_kinds: [
        Sint,
        Uint,
    ],
}

gen_component_wise_extractor! {
    component_wise_signed -> Signed,
    literals: [
        AbstractFloat => AbstractFloat: f64,
        AbstractInt => AbstractInt: i64,
        F32 => F32: f32,
        F16 => F16: f16,
        I32 => I32: i32,
    ],
    scalar_kinds: [
        Sint,
        AbstractInt,
        Float,
        AbstractFloat,
    ],
}

/// Vectors with a concrete element type.
#[derive(Debug)]
enum LiteralVector {
    F64(ArrayVec<f64, { crate::VectorSize::MAX }>),
    F32(ArrayVec<f32, { crate::VectorSize::MAX }>),
    F16(ArrayVec<f16, { crate::VectorSize::MAX }>),
    U32(ArrayVec<u32, { crate::VectorSize::MAX }>),
    I32(ArrayVec<i32, { crate::VectorSize::MAX }>),
    U64(ArrayVec<u64, { crate::VectorSize::MAX }>),
    I64(ArrayVec<i64, { crate::VectorSize::MAX }>),
    Bool(ArrayVec<bool, { crate::VectorSize::MAX }>),
    AbstractInt(ArrayVec<i64, { crate::VectorSize::MAX }>),
    AbstractFloat(ArrayVec<f64, { crate::VectorSize::MAX }>),
}

impl LiteralVector {
    #[allow(clippy::missing_const_for_fn, reason = "MSRV")]
    fn len(&self) -> usize {
        match *self {
            LiteralVector::F64(ref v) => v.len(),
            LiteralVector::F32(ref v) => v.len(),
            LiteralVector::F16(ref v) => v.len(),
            LiteralVector::U32(ref v) => v.len(),
            LiteralVector::I32(ref v) => v.len(),
            LiteralVector::U64(ref v) => v.len(),
            LiteralVector::I64(ref v) => v.len(),
            LiteralVector::Bool(ref v) => v.len(),
            LiteralVector::AbstractInt(ref v) => v.len(),
            LiteralVector::AbstractFloat(ref v) => v.len(),
        }
    }

    /// Creates [`LiteralVector`] of size 1 from single [`Literal`]
    fn from_literal(literal: Literal) -> Self {
        fn arrayvec_of<T, const N: usize>(val: T) -> ArrayVec<T, N> {
            let mut v = ArrayVec::new();
            v.push(val);
            v
        }
        match literal {
            Literal::F64(e) => Self::F64(arrayvec_of(e)),
            Literal::F32(e) => Self::F32(arrayvec_of(e)),
            Literal::U32(e) => Self::U32(arrayvec_of(e)),
            Literal::I32(e) => Self::I32(arrayvec_of(e)),
            Literal::U64(e) => Self::U64(arrayvec_of(e)),
            Literal::I64(e) => Self::I64(arrayvec_of(e)),
            Literal::Bool(e) => Self::Bool(arrayvec_of(e)),
            Literal::AbstractInt(e) => Self::AbstractInt(arrayvec_of(e)),
            Literal::AbstractFloat(e) => Self::AbstractFloat(arrayvec_of(e)),
            Literal::F16(e) => Self::F16(arrayvec_of(e)),
        }
    }

    /// Creates [`LiteralVector`] from [`ArrayVec`] of [`Literal`]s.
    /// Returns error if components types do not match.
    /// # Panics
    /// Panics if vector is empty
    fn from_literal_vec(
        components: ArrayVec<Literal, { crate::VectorSize::MAX }>,
    ) -> Result<Self, ConstantEvaluatorError> {
        assert!(!components.is_empty());
        // TODO: should a vector of i32 be constructible from abstract int?
        macro_rules! compose_literals {
            ($components:expr, $variant:ident, $self_variant:ident) => {{
                let mut out = ArrayVec::new();
                for l in &$components {
                    match l {
                        &Literal::$variant(v) => out.push(v),
                        _ => return Err(ConstantEvaluatorError::InvalidMathArg),
                    }
                }
                Self::$self_variant(out)
            }};
        }
        Ok(match components[0] {
            Literal::I32(_) => compose_literals!(components, I32, I32),
            Literal::U32(_) => compose_literals!(components, U32, U32),
            Literal::I64(_) => compose_literals!(components, I64, I64),
            Literal::U64(_) => compose_literals!(components, U64, U64),
            Literal::F32(_) => compose_literals!(components, F32, F32),
            Literal::F64(_) => compose_literals!(components, F64, F64),
            Literal::Bool(_) => compose_literals!(components, Bool, Bool),
            Literal::AbstractInt(_) => compose_literals!(components, AbstractInt, AbstractInt),
            Literal::AbstractFloat(_) => {
                compose_literals!(components, AbstractFloat, AbstractFloat)
            }
            Literal::F16(_) => compose_literals!(components, F16, F16),
        })
    }

    #[allow(dead_code)]
    /// Returns [`ArrayVec`] of [`Literal`]s
    fn to_literal_vec(&self) -> ArrayVec<Literal, { crate::VectorSize::MAX }> {
        macro_rules! decompose_literals {
            ($v:expr, $variant:ident) => {{
                let mut out = ArrayVec::new();
                for e in $v {
                    out.push(Literal::$variant(*e));
                }
                out
            }};
        }
        match *self {
            LiteralVector::F64(ref v) => decompose_literals!(v, F64),
            LiteralVector::F32(ref v) => decompose_literals!(v, F32),
            LiteralVector::F16(ref v) => decompose_literals!(v, F16),
            LiteralVector::U32(ref v) => decompose_literals!(v, U32),
            LiteralVector::I32(ref v) => decompose_literals!(v, I32),
            LiteralVector::U64(ref v) => decompose_literals!(v, U64),
            LiteralVector::I64(ref v) => decompose_literals!(v, I64),
            LiteralVector::Bool(ref v) => decompose_literals!(v, Bool),
            LiteralVector::AbstractInt(ref v) => decompose_literals!(v, AbstractInt),
            LiteralVector::AbstractFloat(ref v) => decompose_literals!(v, AbstractFloat),
        }
    }

    #[allow(dead_code)]
    /// Puts self into eval's expressions arena and returns handle to it
    fn register_as_evaluated_expr(
        &self,
        eval: &mut ConstantEvaluator<'_>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let lit_vec = self.to_literal_vec();
        assert!(!lit_vec.is_empty());
        let expr = if lit_vec.len() == 1 {
            Expression::Literal(lit_vec[0])
        } else {
            Expression::Compose {
                ty: eval.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector {
                            size: match lit_vec.len() {
                                2 => crate::VectorSize::Bi,
                                3 => crate::VectorSize::Tri,
                                4 => crate::VectorSize::Quad,
                                _ => unreachable!(),
                            },
                            scalar: lit_vec[0].scalar(),
                        },
                    },
                    Span::UNDEFINED,
                ),
                components: lit_vec
                    .iter()
                    .map(|&l| eval.register_evaluated_expr(Expression::Literal(l), span))
                    .collect::<Result<_, _>>()?,
            }
        };
        eval.register_evaluated_expr(expr, span)
    }
}

/// A macro for matching on [`LiteralVector`] variants.
///
/// `Float` variant expands to `F16`, `F32`, `F64` and `AbstractFloat`.
/// `Integer` variant expands to `I32`, `I64`, `U32`, `U64` and `AbstractInt`.
///
/// For output both [`Literal`] (fold) and [`LiteralVector`] (map) are supported.
///
/// Example usage:
///
/// ```rust,ignore
/// match_literal_vector!(match v => Literal {
///     F16 => |v| {v.sum()},
///     Integer => |v| {v.sum()},
///     U32 => |v| -> I32 {v.sum()}, // optionally override return type
/// })
/// ```
///
/// ```rust,ignore
/// match_literal_vector!(match (e1, e2) => LiteralVector {
///     F16 => |e1, e2| {e1+e2},
///     Integer => |e1, e2| {e1+e2},
///     U32 => |e1, e2| -> I32 {e1+e2}, // optionally override return type
/// })
/// ```
macro_rules! match_literal_vector {
    (match $lit_vec:expr => $out:ident {
        $(
            $ty:ident => |$($var:ident),+| $(-> $ret:ident)? { $body:expr }
        ),+
        $(,)?
    }) => {
        match_literal_vector!(@inner_start $lit_vec; $out; [$($ty),+]; [$({ $($var),+ ; $($ret)? ; $body }),+])
    };

    (@inner_start
        $lit_vec:expr;
        $out:ident;
        [$($ty:ident),+];
        [$({ $($var:ident),+ ; $($ret:ident)? ; $body:expr }),+]
    ) => {
        match_literal_vector!(@inner
            $lit_vec;
            $out;
            [$($ty),+];
            [] <> [$({ $($var),+ ; $($ret)? ; $body }),+]
        )
    };

    (@inner
        $lit_vec:expr;
        $out:ident;
        [$ty:ident $(, $ty1:ident)*];
        [$({$_ty:ident ; $($_var:ident),+ ; $($_ret:ident)? ; $_body:expr}),*] <>
        [$({ $($var:ident),+ ; $($ret:ident)? ; $body:expr }),+]
    ) => {
        match_literal_vector!(@inner
            $ty;
            $lit_vec;
            $out;
            [$($ty1),*];
            [$({$_ty ; $($_var),+ ; $($_ret)? ; $_body}),*] <>
            [$({ $($var),+ ; $($ret)? ; $body }),+]
        )
    };
    (@inner
        Integer;
        $lit_vec:expr;
        $out:ident;
        [$($ty:ident),*];
        [$({$_ty:ident ; $($_var:ident),+ ; $($_ret:ident)? ; $_body:expr}),*] <>
        [
            { $($var:ident),+ ; $($ret:ident)? ; $body:expr }
            $(,{ $($var1:ident),+ ; $($ret1:ident)? ; $body1:expr })*
        ]
    ) => {
        match_literal_vector!(@inner
            $lit_vec;
            $out;
            [U32, I32, U64, I64, AbstractInt $(, $ty)*];
            [$({$_ty ; $($_var),+ ; $($_ret)? ; $_body}),*] <>
            [
                { $($var),+ ; $($ret)? ; $body }, // U32
                { $($var),+ ; $($ret)? ; $body }, // I32
                { $($var),+ ; $($ret)? ; $body }, // U64
                { $($var),+ ; $($ret)? ; $body }, // I64
                { $($var),+ ; $($ret)? ; $body }  // AbstractInt
                $(,{ $($var1),+ ; $($ret1)? ; $body1 })*
            ]
        )
    };
    (@inner
        Float;
        $lit_vec:expr;
        $out:ident;
        [$($ty:ident),*];
        [$({$_ty:ident ; $($_var:ident),+ ; $($_ret:ident)? ; $_body:expr}),*] <>
        [
            { $($var:ident),+ ; $($ret:ident)? ; $body:expr }
            $(,{ $($var1:ident),+ ; $($ret1:ident)? ; $body1:expr })*
        ]
    ) => {
        match_literal_vector!(@inner
            $lit_vec;
            $out;
            [F16, F32, F64, AbstractFloat $(, $ty)*];
            [$({$_ty ; $($_var),+ ; $($_ret)? ; $_body}),*] <>
            [
                { $($var),+ ; $($ret)? ; $body }, // F16
                { $($var),+ ; $($ret)? ; $body }, // F32
                { $($var),+ ; $($ret)? ; $body }, // F64
                { $($var),+ ; $($ret)? ; $body }  // AbstractFloat
                $(,{ $($var1),+ ; $($ret1)? ; $body1 })*
            ]
        )
    };
    (@inner
        $ty:ident;
        $lit_vec:expr;
        $out:ident;
        [$ty1:ident $(,$ty2:ident)*];
        [$({$_ty:ident ; $($_var:ident),+ ; $($_ret:ident)? ; $_body:expr}),*] <> [
            { $($var:ident),+ ; $($ret:ident)? ; $body:expr }
            $(, { $($var1:ident),+ ; $($ret1:ident)? ; $body1:expr })*
        ]
    ) => {
        match_literal_vector!(@inner
            $ty1;
            $lit_vec;
            $out;
            [$($ty2),*];
            [
                $({$_ty ; $($_var),+ ; $($_ret)? ; $_body},)*
                { $ty; $($var),+ ; $($ret)? ; $body }
            ] <>
            [$({ $($var1),+ ; $($ret1)? ; $body1 }),*]

        )
    };
    (@inner
        $ty:ident;
        $lit_vec:expr;
        $out:ident;
        [];
        [$({$_ty:ident ; $($_var:ident),+ ; $($_ret:ident)? ; $_body:expr}),*] <>
        [{ $($var:ident),+ ; $($ret:ident)? ; $body:expr }]
    ) => {
        match_literal_vector!(@inner_finish
            $lit_vec;
            $out;
            [
                $({ $_ty ; $($_var),+ ; $($_ret)? ; $_body },)*
                { $ty; $($var),+ ; $($ret)? ; $body }
            ]
        )
    };
    (@inner_finish
        $lit_vec:expr;
        $out:ident;
        [$({$ty:ident ; $($var:ident),+ ; $($ret:ident)? ; $body:expr}),+]
    ) => {
        match $lit_vec {
            $(
                #[allow(unused_parens)]
                ($(LiteralVector::$ty(ref $var)),+) => { Ok(match_literal_vector!(@expand_ret $out; $ty $(; $ret)? ; $body)) }
            )+
            _ => Err(ConstantEvaluatorError::InvalidMathArg),
        }
    };
    (@expand_ret $out:ident; $ty:ident; $body:expr) => {
        $out::$ty($body)
    };
    (@expand_ret $out:ident; $_ty:ident; $ret:ident; $body:expr) => {
        $out::$ret($body)
    };
}

#[derive(Debug)]
enum Behavior<'a> {
    Wgsl(WgslRestrictions<'a>),
    Glsl(GlslRestrictions<'a>),
}

impl Behavior<'_> {
    /// Returns `true` if the inner WGSL/GLSL restrictions are runtime restrictions.
    const fn has_runtime_restrictions(&self) -> bool {
        matches!(
            self,
            &Behavior::Wgsl(WgslRestrictions::Runtime(_))
                | &Behavior::Glsl(GlslRestrictions::Runtime(_))
        )
    }
}

/// A context for evaluating constant expressions.
///
/// A `ConstantEvaluator` points at an expression arena to which it can append
/// newly evaluated expressions: you pass [`try_eval_and_append`] whatever kind
/// of Naga [`Expression`] you like, and if its value can be computed at compile
/// time, `try_eval_and_append` appends an expression representing the computed
/// value - a tree of [`Literal`], [`Compose`], [`ZeroValue`], and [`Swizzle`]
/// expressions - to the arena. See the [`try_eval_and_append`] method for details.
///
/// A `ConstantEvaluator` also holds whatever information we need to carry out
/// that evaluation: types, other constants, and so on.
///
/// [`try_eval_and_append`]: ConstantEvaluator::try_eval_and_append
/// [`Compose`]: Expression::Compose
/// [`ZeroValue`]: Expression::ZeroValue
/// [`Literal`]: Expression::Literal
/// [`Swizzle`]: Expression::Swizzle
#[derive(Debug)]
pub struct ConstantEvaluator<'a> {
    /// Which language's evaluation rules we should follow.
    behavior: Behavior<'a>,

    /// The module's type arena.
    ///
    /// Because expressions like [`Splat`] contain type handles, we need to be
    /// able to add new types to produce those expressions.
    ///
    /// [`Splat`]: Expression::Splat
    types: &'a mut UniqueArena<Type>,

    /// The module's constant arena.
    constants: &'a Arena<Constant>,

    /// The module's override arena.
    overrides: &'a Arena<Override>,

    /// The arena to which we are contributing expressions.
    expressions: &'a mut Arena<Expression>,

    /// Tracks the constness of expressions residing in [`Self::expressions`]
    expression_kind_tracker: &'a mut ExpressionKindTracker,

    layouter: &'a mut crate::proc::Layouter,
}

#[derive(Debug)]
enum WgslRestrictions<'a> {
    /// - const-expressions will be evaluated and inserted in the arena
    Const(Option<FunctionLocalData<'a>>),
    /// - const-expressions will be evaluated and inserted in the arena
    /// - override-expressions will be inserted in the arena
    Override,
    /// - const-expressions will be evaluated and inserted in the arena
    /// - override-expressions will be inserted in the arena
    /// - runtime-expressions will be inserted in the arena
    Runtime(FunctionLocalData<'a>),
}

#[derive(Debug)]
enum GlslRestrictions<'a> {
    /// - const-expressions will be evaluated and inserted in the arena
    Const,
    /// - const-expressions will be evaluated and inserted in the arena
    /// - override-expressions will be inserted in the arena
    /// - runtime-expressions will be inserted in the arena
    Runtime(FunctionLocalData<'a>),
}

#[derive(Debug)]
struct FunctionLocalData<'a> {
    /// Global constant expressions
    global_expressions: &'a Arena<Expression>,
    emitter: &'a mut super::Emitter,
    block: &'a mut crate::Block,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ExpressionKind {
    Const,
    Override,
    Runtime,
}

#[derive(Debug)]
pub struct ExpressionKindTracker {
    inner: HandleVec<Expression, ExpressionKind>,
}

impl ExpressionKindTracker {
    pub const fn new() -> Self {
        Self {
            inner: HandleVec::new(),
        }
    }

    /// Forces the the expression to not be const
    pub fn force_non_const(&mut self, value: Handle<Expression>) {
        self.inner[value] = ExpressionKind::Runtime;
    }

    pub fn insert(&mut self, value: Handle<Expression>, expr_type: ExpressionKind) {
        self.inner.insert(value, expr_type);
    }

    pub fn is_const(&self, h: Handle<Expression>) -> bool {
        matches!(self.type_of(h), ExpressionKind::Const)
    }

    pub fn is_const_or_override(&self, h: Handle<Expression>) -> bool {
        matches!(
            self.type_of(h),
            ExpressionKind::Const | ExpressionKind::Override
        )
    }

    fn type_of(&self, value: Handle<Expression>) -> ExpressionKind {
        self.inner[value]
    }

    pub fn from_arena(arena: &Arena<Expression>) -> Self {
        let mut tracker = Self {
            inner: HandleVec::with_capacity(arena.len()),
        };
        for (handle, expr) in arena.iter() {
            tracker
                .inner
                .insert(handle, tracker.type_of_with_expr(expr));
        }
        tracker
    }

    fn type_of_with_expr(&self, expr: &Expression) -> ExpressionKind {
        match *expr {
            Expression::Literal(_) | Expression::ZeroValue(_) | Expression::Constant(_) => {
                ExpressionKind::Const
            }
            Expression::Override(_) => ExpressionKind::Override,
            Expression::Compose { ref components, .. } => {
                let mut expr_type = ExpressionKind::Const;
                for component in components {
                    expr_type = expr_type.max(self.type_of(*component))
                }
                expr_type
            }
            Expression::Splat { value, .. } => self.type_of(value),
            Expression::AccessIndex { base, .. } => self.type_of(base),
            Expression::Access { base, index } => self.type_of(base).max(self.type_of(index)),
            Expression::Swizzle { vector, .. } => self.type_of(vector),
            Expression::Unary { expr, .. } => self.type_of(expr),
            Expression::Binary { left, right, .. } => self.type_of(left).max(self.type_of(right)),
            Expression::Math {
                arg,
                arg1,
                arg2,
                arg3,
                ..
            } => self
                .type_of(arg)
                .max(
                    arg1.map(|arg| self.type_of(arg))
                        .unwrap_or(ExpressionKind::Const),
                )
                .max(
                    arg2.map(|arg| self.type_of(arg))
                        .unwrap_or(ExpressionKind::Const),
                )
                .max(
                    arg3.map(|arg| self.type_of(arg))
                        .unwrap_or(ExpressionKind::Const),
                ),
            Expression::As { expr, .. } => self.type_of(expr),
            Expression::Select {
                condition,
                accept,
                reject,
            } => self
                .type_of(condition)
                .max(self.type_of(accept))
                .max(self.type_of(reject)),
            Expression::Relational { argument, .. } => self.type_of(argument),
            Expression::ArrayLength(expr) => self.type_of(expr),
            _ => ExpressionKind::Runtime,
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ConstantEvaluatorError {
    #[error("Constants cannot access function arguments")]
    FunctionArg,
    #[error("Constants cannot access global variables")]
    GlobalVariable,
    #[error("Constants cannot access local variables")]
    LocalVariable,
    #[error("Cannot get the array length of a non array type")]
    InvalidArrayLengthArg,
    #[error("Constants cannot get the array length of a dynamically sized array")]
    ArrayLengthDynamic,
    #[error("Cannot call arrayLength on array sized by override-expression")]
    ArrayLengthOverridden,
    #[error("Constants cannot call functions")]
    Call,
    #[error("Constants don't support workGroupUniformLoad")]
    WorkGroupUniformLoadResult,
    #[error("Constants don't support atomic functions")]
    Atomic,
    #[error("Constants don't support derivative functions")]
    Derivative,
    #[error("Constants don't support load expressions")]
    Load,
    #[error("Constants don't support image expressions")]
    ImageExpression,
    #[error("Constants don't support ray query expressions")]
    RayQueryExpression,
    #[error("Constants don't support subgroup expressions")]
    SubgroupExpression,
    #[error("Cannot access the type")]
    InvalidAccessBase,
    #[error("Cannot access at the index")]
    InvalidAccessIndex,
    #[error("Cannot access with index of type")]
    InvalidAccessIndexTy,
    #[error("Constants don't support array length expressions")]
    ArrayLength,
    #[error("Cannot cast scalar components of expression `{from}` to type `{to}`")]
    InvalidCastArg { from: String, to: String },
    #[error("Cannot apply the unary op to the argument")]
    InvalidUnaryOpArg,
    #[error("Cannot apply the binary op to the arguments")]
    InvalidBinaryOpArgs,
    #[error("Cannot apply math function to type")]
    InvalidMathArg,
    #[error("{0:?} built-in function expects {1:?} arguments but {2:?} were supplied")]
    InvalidMathArgCount(crate::MathFunction, usize, usize),
    #[error("{0} built-in function argument is out of valid range")]
    InvalidMathArgValue(String),
    #[error("Cannot apply relational function to type")]
    InvalidRelationalArg(RelationalFunction),
    #[error("value of `low` is greater than `high` for clamp built-in function")]
    InvalidClamp,
    #[error("Constructor expects {expected} components, found {actual}")]
    InvalidVectorComposeLength { expected: usize, actual: usize },
    #[error("Constructor must only contain vector or scalar arguments")]
    InvalidVectorComposeComponent,
    #[error("Splat is defined only on scalar values")]
    SplatScalarOnly,
    #[error("Can only swizzle vector constants")]
    SwizzleVectorOnly,
    #[error("swizzle component not present in source expression")]
    SwizzleOutOfBounds,
    #[error("Type is not constructible")]
    TypeNotConstructible,
    #[error("Subexpression(s) are not constant")]
    SubexpressionsAreNotConstant,
    #[error("Not implemented as constant expression: {0}")]
    NotImplemented(String),
    #[error("{0} operation overflowed")]
    Overflow(String),
    #[error(
        "the concrete type `{to_type}` cannot represent the abstract value `{value}` accurately"
    )]
    AutomaticConversionLossy {
        value: String,
        to_type: &'static str,
    },
    #[error("Division by zero")]
    DivisionByZero,
    #[error("Remainder by zero")]
    RemainderByZero,
    #[error("RHS of shift operation is greater than or equal to 32")]
    ShiftedMoreThan32Bits,
    #[error(transparent)]
    Literal(#[from] crate::valid::LiteralError),
    #[error("Can't use pipeline-overridable constants in const-expressions")]
    Override,
    #[error("Unexpected runtime-expression")]
    RuntimeExpr,
    #[error("Unexpected override-expression")]
    OverrideExpr,
    #[error("Expected boolean expression for condition argument of `select`, got something else")]
    SelectScalarConditionNotABool,
    #[error(
        "Expected vectors of the same size for reject and accept args., got {:?} and {:?}",
        reject,
        accept
    )]
    SelectVecRejectAcceptSizeMismatch {
        reject: crate::VectorSize,
        accept: crate::VectorSize,
    },
    #[error("Expected boolean vector for condition arg., got something else")]
    SelectConditionNotAVecBool,
    #[error(
        "Expected same number of vector components between condition, accept, and reject args., got something else",
    )]
    SelectConditionVecSizeMismatch,
    #[error(
        "Expected reject and accept args. to be scalars of vectors of the same type, got something else",
    )]
    SelectAcceptRejectTypeMismatch,
    #[error("Cooperative operations can't be constant")]
    CooperativeOperation,
}

impl<'a> ConstantEvaluator<'a> {
    /// Return a [`ConstantEvaluator`] that will add expressions to `module`'s
    /// constant expression arena.
    ///
    /// Report errors according to WGSL's rules for constant evaluation.
    pub const fn for_wgsl_module(
        module: &'a mut crate::Module,
        global_expression_kind_tracker: &'a mut ExpressionKindTracker,
        layouter: &'a mut crate::proc::Layouter,
        in_override_ctx: bool,
    ) -> Self {
        Self::for_module(
            Behavior::Wgsl(if in_override_ctx {
                WgslRestrictions::Override
            } else {
                WgslRestrictions::Const(None)
            }),
            module,
            global_expression_kind_tracker,
            layouter,
        )
    }

    /// Return a [`ConstantEvaluator`] that will add expressions to `module`'s
    /// constant expression arena.
    ///
    /// Report errors according to GLSL's rules for constant evaluation.
    pub const fn for_glsl_module(
        module: &'a mut crate::Module,
        global_expression_kind_tracker: &'a mut ExpressionKindTracker,
        layouter: &'a mut crate::proc::Layouter,
    ) -> Self {
        Self::for_module(
            Behavior::Glsl(GlslRestrictions::Const),
            module,
            global_expression_kind_tracker,
            layouter,
        )
    }

    const fn for_module(
        behavior: Behavior<'a>,
        module: &'a mut crate::Module,
        global_expression_kind_tracker: &'a mut ExpressionKindTracker,
        layouter: &'a mut crate::proc::Layouter,
    ) -> Self {
        Self {
            behavior,
            types: &mut module.types,
            constants: &module.constants,
            overrides: &module.overrides,
            expressions: &mut module.global_expressions,
            expression_kind_tracker: global_expression_kind_tracker,
            layouter,
        }
    }

    /// Return a [`ConstantEvaluator`] that will add expressions to `function`'s
    /// expression arena.
    ///
    /// Report errors according to WGSL's rules for constant evaluation.
    pub const fn for_wgsl_function(
        module: &'a mut crate::Module,
        expressions: &'a mut Arena<Expression>,
        local_expression_kind_tracker: &'a mut ExpressionKindTracker,
        layouter: &'a mut crate::proc::Layouter,
        emitter: &'a mut super::Emitter,
        block: &'a mut crate::Block,
        is_const: bool,
    ) -> Self {
        let local_data = FunctionLocalData {
            global_expressions: &module.global_expressions,
            emitter,
            block,
        };
        Self {
            behavior: Behavior::Wgsl(if is_const {
                WgslRestrictions::Const(Some(local_data))
            } else {
                WgslRestrictions::Runtime(local_data)
            }),
            types: &mut module.types,
            constants: &module.constants,
            overrides: &module.overrides,
            expressions,
            expression_kind_tracker: local_expression_kind_tracker,
            layouter,
        }
    }

    /// Return a [`ConstantEvaluator`] that will add expressions to `function`'s
    /// expression arena.
    ///
    /// Report errors according to GLSL's rules for constant evaluation.
    pub const fn for_glsl_function(
        module: &'a mut crate::Module,
        expressions: &'a mut Arena<Expression>,
        local_expression_kind_tracker: &'a mut ExpressionKindTracker,
        layouter: &'a mut crate::proc::Layouter,
        emitter: &'a mut super::Emitter,
        block: &'a mut crate::Block,
    ) -> Self {
        Self {
            behavior: Behavior::Glsl(GlslRestrictions::Runtime(FunctionLocalData {
                global_expressions: &module.global_expressions,
                emitter,
                block,
            })),
            types: &mut module.types,
            constants: &module.constants,
            overrides: &module.overrides,
            expressions,
            expression_kind_tracker: local_expression_kind_tracker,
            layouter,
        }
    }

    pub const fn to_ctx(&self) -> crate::proc::GlobalCtx<'_> {
        crate::proc::GlobalCtx {
            types: self.types,
            constants: self.constants,
            overrides: self.overrides,
            global_expressions: match self.function_local_data() {
                Some(data) => data.global_expressions,
                None => self.expressions,
            },
        }
    }

    fn check(&self, expr: Handle<Expression>) -> Result<(), ConstantEvaluatorError> {
        if !self.expression_kind_tracker.is_const(expr) {
            log::debug!("check: SubexpressionsAreNotConstant");
            return Err(ConstantEvaluatorError::SubexpressionsAreNotConstant);
        }
        Ok(())
    }

    fn check_and_get(
        &mut self,
        expr: Handle<Expression>,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.expressions[expr] {
            Expression::Constant(c) => {
                // Are we working in a function's expression arena, or the
                // module's constant expression arena?
                if let Some(function_local_data) = self.function_local_data() {
                    // Deep-copy the constant's value into our arena.
                    self.copy_from(
                        self.constants[c].init,
                        function_local_data.global_expressions,
                    )
                } else {
                    // "See through" the constant and use its initializer.
                    Ok(self.constants[c].init)
                }
            }
            _ => {
                self.check(expr)?;
                Ok(expr)
            }
        }
    }

    /// Try to evaluate `expr` at compile time.
    ///
    /// The `expr` argument can be any sort of Naga [`Expression`] you like. If
    /// we can determine its value at compile time, we append an expression
    /// representing its value - a tree of [`Literal`], [`Compose`],
    /// [`ZeroValue`], and [`Swizzle`] expressions - to the expression arena
    /// `self` contributes to.
    ///
    /// If `expr`'s value cannot be determined at compile time, and `self` is
    /// contributing to some function's expression arena, then append `expr` to
    /// that arena unchanged (and thus unevaluated). Otherwise, `self` must be
    /// contributing to the module's constant expression arena; since `expr`'s
    /// value is not a constant, return an error.
    ///
    /// We only consider `expr` itself, without recursing into its operands. Its
    /// operands must all have been produced by prior calls to
    /// `try_eval_and_append`, to ensure that they have already been reduced to
    /// an evaluated form if possible.
    ///
    /// [`Literal`]: Expression::Literal
    /// [`Compose`]: Expression::Compose
    /// [`ZeroValue`]: Expression::ZeroValue
    /// [`Swizzle`]: Expression::Swizzle
    pub fn try_eval_and_append(
        &mut self,
        expr: Expression,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.expression_kind_tracker.type_of_with_expr(&expr) {
            ExpressionKind::Const => {
                let eval_result = self.try_eval_and_append_impl(&expr, span);
                // We should be able to evaluate `Const` expressions at this
                // point. If we failed to, then that probably means we just
                // haven't implemented that part of constant evaluation. Work
                // around this by simply emitting it as a run-time expression.
                if self.behavior.has_runtime_restrictions()
                    && matches!(
                        eval_result,
                        Err(ConstantEvaluatorError::NotImplemented(_)
                            | ConstantEvaluatorError::InvalidBinaryOpArgs,)
                    )
                {
                    Ok(self.append_expr(expr, span, ExpressionKind::Runtime))
                } else {
                    eval_result
                }
            }
            ExpressionKind::Override => match self.behavior {
                Behavior::Wgsl(WgslRestrictions::Override | WgslRestrictions::Runtime(_)) => {
                    Ok(self.append_expr(expr, span, ExpressionKind::Override))
                }
                Behavior::Wgsl(WgslRestrictions::Const(_)) => {
                    Err(ConstantEvaluatorError::OverrideExpr)
                }

                // GLSL specialization constants (constant_id) become Override expressions
                Behavior::Glsl(GlslRestrictions::Runtime(_)) => {
                    Ok(self.append_expr(expr, span, ExpressionKind::Override))
                }
                Behavior::Glsl(GlslRestrictions::Const) => {
                    Err(ConstantEvaluatorError::OverrideExpr)
                }
            },
            ExpressionKind::Runtime => {
                if self.behavior.has_runtime_restrictions() {
                    Ok(self.append_expr(expr, span, ExpressionKind::Runtime))
                } else {
                    Err(ConstantEvaluatorError::RuntimeExpr)
                }
            }
        }
    }

    /// Is the [`Self::expressions`] arena the global module expression arena?
    const fn is_global_arena(&self) -> bool {
        matches!(
            self.behavior,
            Behavior::Wgsl(WgslRestrictions::Const(None) | WgslRestrictions::Override)
                | Behavior::Glsl(GlslRestrictions::Const)
        )
    }

    const fn function_local_data(&self) -> Option<&FunctionLocalData<'a>> {
        match self.behavior {
            Behavior::Wgsl(
                WgslRestrictions::Runtime(ref function_local_data)
                | WgslRestrictions::Const(Some(ref function_local_data)),
            )
            | Behavior::Glsl(GlslRestrictions::Runtime(ref function_local_data)) => {
                Some(function_local_data)
            }
            _ => None,
        }
    }

    fn try_eval_and_append_impl(
        &mut self,
        expr: &Expression,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        log::trace!("try_eval_and_append: {expr:?}");
        match *expr {
            Expression::Constant(c) if self.is_global_arena() => {
                // "See through" the constant and use its initializer.
                // This is mainly done to avoid having constants pointing to other constants.
                Ok(self.constants[c].init)
            }
            Expression::Override(_) => Err(ConstantEvaluatorError::Override),
            Expression::Literal(_) | Expression::ZeroValue(_) | Expression::Constant(_) => {
                self.register_evaluated_expr(expr.clone(), span)
            }
            Expression::Compose { ty, ref components } => {
                let components = components
                    .iter()
                    .map(|component| self.check_and_get(*component))
                    .collect::<Result<Vec<_>, _>>()?;
                self.register_evaluated_expr(Expression::Compose { ty, components }, span)
            }
            Expression::Splat { size, value } => {
                let value = self.check_and_get(value)?;
                self.register_evaluated_expr(Expression::Splat { size, value }, span)
            }
            Expression::AccessIndex { base, index } => {
                let base = self.check_and_get(base)?;

                self.access(base, index as usize, span)
            }
            Expression::Access { base, index } => {
                let base = self.check_and_get(base)?;
                let index = self.check_and_get(index)?;

                let index_val: u32 = self
                    .to_ctx()
                    .get_const_val_from(index, self.expressions)
                    .map_err(|_| ConstantEvaluatorError::InvalidAccessIndexTy)?;
                self.access(base, index_val as usize, span)
            }
            Expression::Swizzle {
                size,
                vector,
                pattern,
            } => {
                let vector = self.check_and_get(vector)?;

                self.swizzle(size, span, vector, pattern)
            }
            Expression::Unary { expr, op } => {
                let expr = self.check_and_get(expr)?;

                self.unary_op(op, expr, span)
            }
            Expression::Binary { left, right, op } => {
                let left = self.check_and_get(left)?;
                let right = self.check_and_get(right)?;

                self.binary_op(op, left, right, span)
            }
            Expression::Math {
                fun,
                arg,
                arg1,
                arg2,
                arg3,
            } => {
                let arg = self.check_and_get(arg)?;
                let arg1 = arg1.map(|arg| self.check_and_get(arg)).transpose()?;
                let arg2 = arg2.map(|arg| self.check_and_get(arg)).transpose()?;
                let arg3 = arg3.map(|arg| self.check_and_get(arg)).transpose()?;

                self.math(arg, arg1, arg2, arg3, fun, span)
            }
            Expression::As {
                convert,
                expr,
                kind,
            } => {
                let expr = self.check_and_get(expr)?;

                match convert {
                    Some(width) => self.cast(expr, crate::Scalar { kind, width }, span),
                    None => Err(ConstantEvaluatorError::NotImplemented(
                        "bitcast built-in function".into(),
                    )),
                }
            }
            Expression::Select {
                reject,
                accept,
                condition,
            } => {
                let mut arg = |expr| self.check_and_get(expr);

                let reject = arg(reject)?;
                let accept = arg(accept)?;
                let condition = arg(condition)?;

                self.select(reject, accept, condition, span)
            }
            Expression::Relational { fun, argument } => {
                let argument = self.check_and_get(argument)?;
                self.relational(fun, argument, span)
            }
            Expression::ArrayLength(expr) => match self.behavior {
                Behavior::Wgsl(_) => Err(ConstantEvaluatorError::ArrayLength),
                Behavior::Glsl(_) => {
                    let expr = self.check_and_get(expr)?;
                    self.array_length(expr, span)
                }
            },
            Expression::Load { .. } => Err(ConstantEvaluatorError::Load),
            Expression::LocalVariable(_) => Err(ConstantEvaluatorError::LocalVariable),
            Expression::Derivative { .. } => Err(ConstantEvaluatorError::Derivative),
            Expression::CallResult { .. } => Err(ConstantEvaluatorError::Call),
            Expression::WorkGroupUniformLoadResult { .. } => {
                Err(ConstantEvaluatorError::WorkGroupUniformLoadResult)
            }
            Expression::AtomicResult { .. } => Err(ConstantEvaluatorError::Atomic),
            Expression::FunctionArgument(_) => Err(ConstantEvaluatorError::FunctionArg),
            Expression::GlobalVariable(_) => Err(ConstantEvaluatorError::GlobalVariable),
            Expression::ImageSample { .. }
            | Expression::ImageLoad { .. }
            | Expression::ImageQuery { .. } => Err(ConstantEvaluatorError::ImageExpression),
            Expression::RayQueryProceedResult
            | Expression::RayQueryGetIntersection { .. }
            | Expression::RayQueryVertexPositions { .. } => {
                Err(ConstantEvaluatorError::RayQueryExpression)
            }
            Expression::SubgroupBallotResult => Err(ConstantEvaluatorError::SubgroupExpression),
            Expression::SubgroupOperationResult { .. } => {
                Err(ConstantEvaluatorError::SubgroupExpression)
            }
            Expression::CooperativeLoad { .. } | Expression::CooperativeMultiplyAdd { .. } => {
                Err(ConstantEvaluatorError::CooperativeOperation)
            }
        }
    }

    /// Splat `value` to `size`, without using [`Splat`] expressions.
    ///
    /// This constructs [`Compose`] or [`ZeroValue`] expressions to
    /// build a vector with the given `size` whose components are all
    /// `value`.
    ///
    /// Use `span` as the span of the inserted expressions and
    /// resulting types.
    ///
    /// [`Splat`]: Expression::Splat
    /// [`Compose`]: Expression::Compose
    /// [`ZeroValue`]: Expression::ZeroValue
    fn splat(
        &mut self,
        value: Handle<Expression>,
        size: crate::VectorSize,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.expressions[value] {
            Expression::Literal(literal) => {
                let scalar = literal.scalar();
                let ty = self.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector { size, scalar },
                    },
                    span,
                );
                let expr = Expression::Compose {
                    ty,
                    components: vec![value; size as usize],
                };
                self.register_evaluated_expr(expr, span)
            }
            Expression::ZeroValue(ty) => {
                let inner = match self.types[ty].inner {
                    TypeInner::Scalar(scalar) => TypeInner::Vector { size, scalar },
                    _ => return Err(ConstantEvaluatorError::SplatScalarOnly),
                };
                let res_ty = self.types.insert(Type { name: None, inner }, span);
                let expr = Expression::ZeroValue(res_ty);
                self.register_evaluated_expr(expr, span)
            }
            _ => Err(ConstantEvaluatorError::SplatScalarOnly),
        }
    }

    fn swizzle(
        &mut self,
        size: crate::VectorSize,
        span: Span,
        src_constant: Handle<Expression>,
        pattern: [crate::SwizzleComponent; 4],
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let mut get_dst_ty = |ty| match self.types[ty].inner {
            TypeInner::Vector { size: _, scalar } => Ok(self.types.insert(
                Type {
                    name: None,
                    inner: TypeInner::Vector { size, scalar },
                },
                span,
            )),
            _ => Err(ConstantEvaluatorError::SwizzleVectorOnly),
        };

        match self.expressions[src_constant] {
            Expression::ZeroValue(ty) => {
                let dst_ty = get_dst_ty(ty)?;
                let expr = Expression::ZeroValue(dst_ty);
                self.register_evaluated_expr(expr, span)
            }
            Expression::Splat { value, .. } => {
                let expr = Expression::Splat { size, value };
                self.register_evaluated_expr(expr, span)
            }
            Expression::Compose { ty, ref components } => {
                let dst_ty = get_dst_ty(ty)?;

                let mut flattened = [src_constant; 4]; // dummy value
                let len =
                    crate::proc::flatten_compose(ty, components, self.expressions, self.types)
                        .zip(flattened.iter_mut())
                        .map(|(component, elt)| *elt = component)
                        .count();
                let flattened = &flattened[..len];

                let swizzled_components = pattern[..size as usize]
                    .iter()
                    .map(|&sc| {
                        let sc = sc as usize;
                        if let Some(elt) = flattened.get(sc) {
                            Ok(*elt)
                        } else {
                            Err(ConstantEvaluatorError::SwizzleOutOfBounds)
                        }
                    })
                    .collect::<Result<Vec<Handle<Expression>>, _>>()?;
                let expr = Expression::Compose {
                    ty: dst_ty,
                    components: swizzled_components,
                };
                self.register_evaluated_expr(expr, span)
            }
            _ => Err(ConstantEvaluatorError::SwizzleVectorOnly),
        }
    }

    fn math(
        &mut self,
        arg: Handle<Expression>,
        arg1: Option<Handle<Expression>>,
        arg2: Option<Handle<Expression>>,
        arg3: Option<Handle<Expression>>,
        fun: crate::MathFunction,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let expected = fun.argument_count();
        let given = Some(arg)
            .into_iter()
            .chain(arg1)
            .chain(arg2)
            .chain(arg3)
            .count();
        if expected != given {
            return Err(ConstantEvaluatorError::InvalidMathArgCount(
                fun, expected, given,
            ));
        }

        // NOTE: We try to match the declaration order of `MathFunction` here.
        match fun {
            // comparison
            crate::MathFunction::Abs => {
                component_wise_scalar(self, span, [arg], |args| match args {
                    Scalar::AbstractFloat([e]) => Ok(Scalar::AbstractFloat([e.abs()])),
                    Scalar::F32([e]) => Ok(Scalar::F32([e.abs()])),
                    Scalar::F16([e]) => Ok(Scalar::F16([e.abs()])),
                    Scalar::AbstractInt([e]) => Ok(Scalar::AbstractInt([e.wrapping_abs()])),
                    Scalar::I32([e]) => Ok(Scalar::I32([e.wrapping_abs()])),
                    Scalar::U32([e]) => Ok(Scalar::U32([e])), // TODO: just re-use the expression, ezpz
                    Scalar::I64([e]) => Ok(Scalar::I64([e.wrapping_abs()])),
                    Scalar::U64([e]) => Ok(Scalar::U64([e])),
                })
            }
            crate::MathFunction::Min => {
                component_wise_scalar!(self, span, [arg, arg1.unwrap()], |e1, e2| {
                    Ok([e1.min(e2)])
                })
            }
            crate::MathFunction::Max => {
                component_wise_scalar!(self, span, [arg, arg1.unwrap()], |e1, e2| {
                    Ok([e1.max(e2)])
                })
            }
            crate::MathFunction::Clamp => {
                component_wise_scalar!(
                    self,
                    span,
                    [arg, arg1.unwrap(), arg2.unwrap()],
                    |e, low, high| {
                        if low > high {
                            Err(ConstantEvaluatorError::InvalidClamp)
                        } else {
                            Ok([e.clamp(low, high)])
                        }
                    }
                )
            }
            crate::MathFunction::Saturate => component_wise_float(self, span, [arg], |e| match e {
                Float::F16([e]) => Ok(Float::F16(
                    [e.clamp(f16::from_f32(0.0), f16::from_f32(1.0))],
                )),
                Float::F32([e]) => Ok(Float::F32([e.clamp(0., 1.)])),
                Float::Abstract([e]) => Ok(Float::Abstract([e.clamp(0., 1.)])),
            }),

            // trigonometry
            crate::MathFunction::Cos => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.cos()]) })
            }
            crate::MathFunction::Cosh => {
                component_wise_float!(self, span, [arg], |e| {
                    let result = e.cosh();
                    if result.is_finite() {
                        Ok([result])
                    } else {
                        Err(ConstantEvaluatorError::Overflow("cosh".into()))
                    }
                })
            }
            crate::MathFunction::Sin => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.sin()]) })
            }
            crate::MathFunction::Sinh => {
                component_wise_float!(self, span, [arg], |e| {
                    let result = e.sinh();
                    if result.is_finite() {
                        Ok([result])
                    } else {
                        Err(ConstantEvaluatorError::Overflow("sinh".into()))
                    }
                })
            }
            crate::MathFunction::Tan => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.tan()]) })
            }
            crate::MathFunction::Tanh => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.tanh()]) })
            }
            crate::MathFunction::Acos => {
                component_wise_float!(self, span, [arg], |e| {
                    if e.abs() <= One::one() {
                        Ok([e.acos()])
                    } else {
                        Err(ConstantEvaluatorError::InvalidMathArgValue("acos".into()))
                    }
                })
            }
            crate::MathFunction::Asin => {
                component_wise_float!(self, span, [arg], |e| {
                    if e.abs() <= One::one() {
                        Ok([e.asin()])
                    } else {
                        Err(ConstantEvaluatorError::InvalidMathArgValue("asin".into()))
                    }
                })
            }
            crate::MathFunction::Atan => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.atan()]) })
            }
            crate::MathFunction::Atan2 => {
                component_wise_float!(self, span, [arg, arg1.unwrap()], |y, x| {
                    Ok([y.atan2(x)])
                })
            }
            crate::MathFunction::Asinh => component_wise_float(self, span, [arg], |e| match e {
                Float::Abstract([e]) => Ok(Float::Abstract([libm::asinh(e)])),
                Float::F32([e]) => Ok(Float::F32([(e as f64).asinh() as f32])),
                Float::F16([e]) => Ok(Float::F16([e.asinh()])),
            }),
            crate::MathFunction::Acosh => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.acosh()]) })
            }
            crate::MathFunction::Atanh => {
                component_wise_float!(self, span, [arg], |e| {
                    if e.abs() < One::one() {
                        Ok([e.atanh()])
                    } else {
                        Err(ConstantEvaluatorError::InvalidMathArgValue("atanh".into()))
                    }
                })
            }
            crate::MathFunction::Radians => {
                component_wise_float!(self, span, [arg], |e1| { Ok([e1.to_radians()]) })
            }
            crate::MathFunction::Degrees => {
                component_wise_float!(self, span, [arg], |e| {
                    let result = e.to_degrees();
                    if result.is_finite() {
                        Ok([result])
                    } else {
                        Err(ConstantEvaluatorError::Overflow("degrees".into()))
                    }
                })
            }

            // decomposition
            crate::MathFunction::Ceil => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.ceil()]) })
            }
            crate::MathFunction::Floor => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.floor()]) })
            }
            crate::MathFunction::Round => {
                component_wise_float(self, span, [arg], |e| match e {
                    Float::Abstract([e]) => Ok(Float::Abstract([libm::rint(e)])),
                    Float::F32([e]) => Ok(Float::F32([libm::rintf(e)])),
                    Float::F16([e]) => {
                        // TODO: `round_ties_even` is not available on `half::f16` yet.
                        //
                        // This polyfill is shamelessly [~~stolen from~~ inspired by `ndarray-image`][polyfill source],
                        // which has licensing compatible with ours. See also
                        // <https://github.com/rust-lang/rust/issues/96710>.
                        //
                        // [polyfill source]: https://github.com/imeka/ndarray-ndimage/blob/8b14b4d6ecfbc96a8a052f802e342a7049c68d8f/src/lib.rs#L98
                        fn round_ties_even(x: f64) -> f64 {
                            let i = x as i64;
                            let f = (x - i as f64).abs();
                            if f == 0.5 {
                                if i & 1 == 1 {
                                    // -1.5, 1.5, 3.5, ...
                                    (x.abs() + 0.5).copysign(x)
                                } else {
                                    (x.abs() - 0.5).copysign(x)
                                }
                            } else {
                                x.round()
                            }
                        }

                        Ok(Float::F16([(f16::from_f64(round_ties_even(f64::from(e))))]))
                    }
                })
            }
            crate::MathFunction::Fract => {
                component_wise_float!(self, span, [arg], |e| {
                    // N.B., Rust's definition of `fract` is `e - e.trunc()`, so we can't use that
                    // here.
                    Ok([e - e.floor()])
                })
            }
            crate::MathFunction::Trunc => {
                component_wise_float!(self, span, [arg], |e| { Ok([e.trunc()]) })
            }

            // exponent
            crate::MathFunction::Exp => {
                component_wise_float!(self, span, [arg], |e| {
                    let result = e.exp();
                    if result.is_finite() {
                        Ok([result])
                    } else {
                        Err(ConstantEvaluatorError::Overflow("exp".into()))
                    }
                })
            }
            crate::MathFunction::Exp2 => {
                component_wise_float!(self, span, [arg], |e| {
                    let result = e.exp2();
                    if result.is_finite() {
                        Ok([result])
                    } else {
                        Err(ConstantEvaluatorError::Overflow("exp2".into()))
                    }
                })
            }
            crate::MathFunction::Log => {
                component_wise_float!(self, span, [arg], |e| {
                    if e > Zero::zero() {
                        Ok([e.ln()])
                    } else {
                        Err(ConstantEvaluatorError::InvalidMathArgValue("log".into()))
                    }
                })
            }
            crate::MathFunction::Log2 => {
                component_wise_float!(self, span, [arg], |e| {
                    if e > Zero::zero() {
                        Ok([e.log2()])
                    } else {
                        Err(ConstantEvaluatorError::InvalidMathArgValue("log2".into()))
                    }
                })
            }
            crate::MathFunction::Pow => {
                component_wise_float!(self, span, [arg, arg1.unwrap()], |e1, e2| {
                    Ok([e1.powf(e2)])
                })
            }

            // computational
            crate::MathFunction::Sign => {
                component_wise_signed!(self, span, [arg], |e| {
                    Ok([if e.is_zero() {
                        Zero::zero()
                    } else {
                        e.signum()
                    }])
                })
            }
            crate::MathFunction::Fma => {
                component_wise_float!(
                    self,
                    span,
                    [arg, arg1.unwrap(), arg2.unwrap()],
                    |e1, e2, e3| { Ok([e1.mul_add(e2, e3)]) }
                )
            }
            crate::MathFunction::Step => {
                component_wise_float(self, span, [arg, arg1.unwrap()], |x| match x {
                    Float::Abstract([edge, x]) => {
                        Ok(Float::Abstract([if edge <= x { 1.0 } else { 0.0 }]))
                    }
                    Float::F32([edge, x]) => Ok(Float::F32([if edge <= x { 1.0 } else { 0.0 }])),
                    Float::F16([edge, x]) => Ok(Float::F16([if edge <= x {
                        f16::one()
                    } else {
                        f16::zero()
                    }])),
                })
            }
            crate::MathFunction::Sqrt => {
                component_wise_float!(self, span, [arg], |e| {
                    if e >= Zero::zero() {
                        Ok([e.sqrt()])
                    } else {
                        Err(ConstantEvaluatorError::InvalidMathArgValue("sqrt".into()))
                    }
                })
            }
            crate::MathFunction::InverseSqrt => {
                component_wise_float(self, span, [arg], |e| match e {
                    Float::Abstract([e]) => Ok(Float::Abstract([1. / e.sqrt()])),
                    Float::F32([e]) => Ok(Float::F32([1. / e.sqrt()])),
                    Float::F16([e]) => Ok(Float::F16([f16::from_f32(1. / f32::from(e).sqrt())])),
                })
            }

            // bits
            crate::MathFunction::CountTrailingZeros => {
                component_wise_concrete_int!(self, span, [arg], |e| {
                    #[allow(clippy::useless_conversion)]
                    Ok([e
                        .trailing_zeros()
                        .try_into()
                        .expect("bit count overflowed 32 bits, somehow!?")])
                })
            }
            crate::MathFunction::CountLeadingZeros => {
                component_wise_concrete_int!(self, span, [arg], |e| {
                    #[allow(clippy::useless_conversion)]
                    Ok([e
                        .leading_zeros()
                        .try_into()
                        .expect("bit count overflowed 32 bits, somehow!?")])
                })
            }
            crate::MathFunction::CountOneBits => {
                component_wise_concrete_int!(self, span, [arg], |e| {
                    #[allow(clippy::useless_conversion)]
                    Ok([e
                        .count_ones()
                        .try_into()
                        .expect("bit count overflowed 32 bits, somehow!?")])
                })
            }
            crate::MathFunction::ReverseBits => {
                component_wise_concrete_int!(self, span, [arg], |e| { Ok([e.reverse_bits()]) })
            }
            crate::MathFunction::FirstTrailingBit => {
                component_wise_concrete_int(self, span, [arg], |ci| Ok(first_trailing_bit(ci)))
            }
            crate::MathFunction::FirstLeadingBit => {
                component_wise_concrete_int(self, span, [arg], |ci| Ok(first_leading_bit(ci)))
            }

            // vector
            crate::MathFunction::Dot4I8Packed => {
                self.packed_dot_product(arg, arg1.unwrap(), span, true)
            }
            crate::MathFunction::Dot4U8Packed => {
                self.packed_dot_product(arg, arg1.unwrap(), span, false)
            }
            crate::MathFunction::Cross => self.cross_product(arg, arg1.unwrap(), span),
            crate::MathFunction::Dot => {
                // https://www.w3.org/TR/WGSL/#dot-builtin
                let e1 = self.extract_vec(arg, false)?;
                let e2 = self.extract_vec(arg1.unwrap(), false)?;
                if e1.len() != e2.len() {
                    return Err(ConstantEvaluatorError::InvalidMathArg);
                }

                fn float_dot_checked<P>(a: &[P], b: &[P]) -> Result<P, ConstantEvaluatorError>
                where
                    P: num_traits::Float,
                {
                    let result = a
                        .iter()
                        .zip(b.iter())
                        .map(|(&aa, &bb)| aa * bb)
                        .fold(P::zero(), |acc, x| acc + x);
                    if result.is_finite() {
                        Ok(result)
                    } else {
                        Err(ConstantEvaluatorError::Overflow("in dot built-in".into()))
                    }
                }

                fn int_dot_checked<P>(a: &[P], b: &[P]) -> Result<P, ConstantEvaluatorError>
                where
                    P: num_traits::PrimInt + num_traits::CheckedAdd + num_traits::CheckedMul,
                {
                    a.iter()
                        .zip(b.iter())
                        .map(|(&aa, bb)| aa.checked_mul(bb))
                        .try_fold(P::zero(), |acc, x| {
                            if let Some(x) = x {
                                acc.checked_add(&x)
                            } else {
                                None
                            }
                        })
                        .ok_or(ConstantEvaluatorError::Overflow(
                            "in dot built-in".to_string(),
                        ))
                }

                fn int_dot_wrapping<P>(a: &[P], b: &[P]) -> P
                where
                    P: num_traits::PrimInt + num_traits::WrappingAdd + num_traits::WrappingMul,
                {
                    a.iter()
                        .zip(b.iter())
                        .map(|(&aa, bb)| aa.wrapping_mul(bb))
                        .fold(P::zero(), |acc, x| acc.wrapping_add(&x))
                }

                let result = match_literal_vector!(match (e1, e2) => Literal {
                    Float => |e1, e2| { float_dot_checked(e1, e2)? },
                    AbstractInt => |e1, e2 | { int_dot_checked(e1, e2)? },
                    I32 => |e1, e2| { int_dot_wrapping(e1, e2) },
                    U32 => |e1, e2| { int_dot_wrapping(e1, e2) },
                })?;
                self.register_evaluated_expr(Expression::Literal(result), span)
            }
            crate::MathFunction::Length => {
                // https://www.w3.org/TR/WGSL/#length-builtin
                let e1 = self.extract_vec(arg, true)?;

                fn float_length<F>(e: &[F]) -> F
                where
                    F: core::ops::Mul<F>,
                    F: num_traits::Float + iter::Sum,
                {
                    if e.len() == 1 {
                        // Avoids possible overflow in squaring
                        e[0].abs()
                    } else {
                        e.iter().map(|&ei| ei * ei).sum::<F>().sqrt()
                    }
                }

                let result = match_literal_vector!(match e1 => Literal {
                    Float => |e1| { float_length(e1) },
                })?;
                self.register_evaluated_expr(Expression::Literal(result), span)
            }
            crate::MathFunction::Distance => {
                // https://www.w3.org/TR/WGSL/#distance-builtin
                let e1 = self.extract_vec(arg, true)?;
                let e2 = self.extract_vec(arg1.unwrap(), true)?;
                if e1.len() != e2.len() {
                    return Err(ConstantEvaluatorError::InvalidMathArg);
                }

                fn float_distance<F>(a: &[F], b: &[F]) -> F
                where
                    F: core::ops::Mul<F>,
                    F: num_traits::Float + iter::Sum + core::ops::Sub,
                {
                    if a.len() == 1 {
                        // Avoids possible overflow in squaring
                        (a[0] - b[0]).abs()
                    } else {
                        a.iter()
                            .zip(b.iter())
                            .map(|(&aa, &bb)| aa - bb)
                            .map(|ei| ei * ei)
                            .sum::<F>()
                            .sqrt()
                    }
                }
                let result = match_literal_vector!(match (e1, e2) => Literal {
                    Float => |e1, e2| { float_distance(e1, e2) },
                })?;
                self.register_evaluated_expr(Expression::Literal(result), span)
            }
            crate::MathFunction::Normalize => {
                // https://www.w3.org/TR/WGSL/#normalize-builtin
                let e1 = self.extract_vec(arg, true)?;

                fn float_normalize<F>(e: &[F]) -> ArrayVec<F, { crate::VectorSize::MAX }>
                where
                    F: core::ops::Mul<F>,
                    F: num_traits::Float + iter::Sum,
                {
                    let len = e.iter().map(|&ei| ei * ei).sum::<F>().sqrt();
                    let mut out = ArrayVec::new();
                    for &ei in e {
                        out.push(ei / len);
                    }
                    out
                }

                let result = match_literal_vector!(match e1 => LiteralVector {
                    Float => |e1| { float_normalize(e1) },
                })?;
                result.register_as_evaluated_expr(self, span)
            }

            // unimplemented
            crate::MathFunction::Modf
            | crate::MathFunction::Frexp
            | crate::MathFunction::Ldexp
            | crate::MathFunction::Outer
            | crate::MathFunction::FaceForward
            | crate::MathFunction::Reflect
            | crate::MathFunction::Refract
            | crate::MathFunction::Mix
            | crate::MathFunction::SmoothStep
            | crate::MathFunction::Inverse
            | crate::MathFunction::Transpose
            | crate::MathFunction::Determinant
            | crate::MathFunction::QuantizeToF16
            | crate::MathFunction::ExtractBits
            | crate::MathFunction::InsertBits
            | crate::MathFunction::Pack4x8snorm
            | crate::MathFunction::Pack4x8unorm
            | crate::MathFunction::Pack2x16snorm
            | crate::MathFunction::Pack2x16unorm
            | crate::MathFunction::Pack2x16float
            | crate::MathFunction::Pack4xI8
            | crate::MathFunction::Pack4xU8
            | crate::MathFunction::Pack4xI8Clamp
            | crate::MathFunction::Pack4xU8Clamp
            | crate::MathFunction::Unpack4x8snorm
            | crate::MathFunction::Unpack4x8unorm
            | crate::MathFunction::Unpack2x16snorm
            | crate::MathFunction::Unpack2x16unorm
            | crate::MathFunction::Unpack2x16float
            | crate::MathFunction::Unpack4xI8
            | crate::MathFunction::Unpack4xU8 => Err(ConstantEvaluatorError::NotImplemented(
                format!("{fun:?} built-in function"),
            )),
        }
    }

    /// Dot product of two packed vectors (`dot4I8Packed` and `dot4U8Packed`)
    fn packed_dot_product(
        &mut self,
        a: Handle<Expression>,
        b: Handle<Expression>,
        span: Span,
        signed: bool,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let Expression::Literal(Literal::U32(a)) = self.expressions[a] else {
            return Err(ConstantEvaluatorError::InvalidMathArg);
        };
        let Expression::Literal(Literal::U32(b)) = self.expressions[b] else {
            return Err(ConstantEvaluatorError::InvalidMathArg);
        };

        let result = if signed {
            Literal::I32(
                (a & 0xFF) as i8 as i32 * (b & 0xFF) as i8 as i32
                    + ((a >> 8) & 0xFF) as i8 as i32 * ((b >> 8) & 0xFF) as i8 as i32
                    + ((a >> 16) & 0xFF) as i8 as i32 * ((b >> 16) & 0xFF) as i8 as i32
                    + ((a >> 24) & 0xFF) as i8 as i32 * ((b >> 24) & 0xFF) as i8 as i32,
            )
        } else {
            Literal::U32(
                (a & 0xFF) * (b & 0xFF)
                    + ((a >> 8) & 0xFF) * ((b >> 8) & 0xFF)
                    + ((a >> 16) & 0xFF) * ((b >> 16) & 0xFF)
                    + ((a >> 24) & 0xFF) * ((b >> 24) & 0xFF),
            )
        };

        self.register_evaluated_expr(Expression::Literal(result), span)
    }

    /// Vector cross product.
    fn cross_product(
        &mut self,
        a: Handle<Expression>,
        b: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        use Literal as Li;

        let (a, ty) = self.extract_vec_with_size::<3>(a)?;
        let (b, _) = self.extract_vec_with_size::<3>(b)?;

        let product = match (a, b) {
            (
                [Li::AbstractInt(a0), Li::AbstractInt(a1), Li::AbstractInt(a2)],
                [Li::AbstractInt(b0), Li::AbstractInt(b1), Li::AbstractInt(b2)],
            ) => {
                // `cross` has no overload for AbstractInt, so AbstractInt
                // arguments are automatically converted to AbstractFloat. Since
                // `f64` has a much wider range than `i64`, there's no danger of
                // overflow here.
                let p = cross_product(
                    [a0 as f64, a1 as f64, a2 as f64],
                    [b0 as f64, b1 as f64, b2 as f64],
                );
                [
                    Li::AbstractFloat(p[0]),
                    Li::AbstractFloat(p[1]),
                    Li::AbstractFloat(p[2]),
                ]
            }
            (
                [Li::AbstractFloat(a0), Li::AbstractFloat(a1), Li::AbstractFloat(a2)],
                [Li::AbstractFloat(b0), Li::AbstractFloat(b1), Li::AbstractFloat(b2)],
            ) => {
                let p = cross_product([a0, a1, a2], [b0, b1, b2]);
                [
                    Li::AbstractFloat(p[0]),
                    Li::AbstractFloat(p[1]),
                    Li::AbstractFloat(p[2]),
                ]
            }
            ([Li::F16(a0), Li::F16(a1), Li::F16(a2)], [Li::F16(b0), Li::F16(b1), Li::F16(b2)]) => {
                let p = cross_product([a0, a1, a2], [b0, b1, b2]);
                [Li::F16(p[0]), Li::F16(p[1]), Li::F16(p[2])]
            }
            ([Li::F32(a0), Li::F32(a1), Li::F32(a2)], [Li::F32(b0), Li::F32(b1), Li::F32(b2)]) => {
                let p = cross_product([a0, a1, a2], [b0, b1, b2]);
                [Li::F32(p[0]), Li::F32(p[1]), Li::F32(p[2])]
            }
            ([Li::F64(a0), Li::F64(a1), Li::F64(a2)], [Li::F64(b0), Li::F64(b1), Li::F64(b2)]) => {
                let p = cross_product([a0, a1, a2], [b0, b1, b2]);
                [Li::F64(p[0]), Li::F64(p[1]), Li::F64(p[2])]
            }
            _ => return Err(ConstantEvaluatorError::InvalidMathArg),
        };

        let p0 = self.register_evaluated_expr(Expression::Literal(product[0]), span)?;
        let p1 = self.register_evaluated_expr(Expression::Literal(product[1]), span)?;
        let p2 = self.register_evaluated_expr(Expression::Literal(product[2]), span)?;

        self.register_evaluated_expr(
            Expression::Compose {
                ty,
                components: vec![p0, p1, p2],
            },
            span,
        )
    }

    /// Extract the values of a `vecN` from `expr`.
    ///
    /// Return the value of `expr`, whose type is `vecN<S>` for some
    /// vector size `N` and scalar `S`, as an array of `N` [`Literal`]
    /// values.
    ///
    /// Also return the type handle from the `Compose` expression.
    fn extract_vec_with_size<const N: usize>(
        &mut self,
        expr: Handle<Expression>,
    ) -> Result<([Literal; N], Handle<Type>), ConstantEvaluatorError> {
        let span = self.expressions.get_span(expr);
        let expr = self.eval_zero_value_and_splat(expr, span)?;
        let Expression::Compose { ty, ref components } = self.expressions[expr] else {
            return Err(ConstantEvaluatorError::InvalidMathArg);
        };

        let mut value = [Literal::Bool(false); N];
        for (component, elt) in
            crate::proc::flatten_compose(ty, components, self.expressions, self.types)
                .zip(value.iter_mut())
        {
            let Expression::Literal(literal) = self.expressions[component] else {
                return Err(ConstantEvaluatorError::InvalidMathArg);
            };
            *elt = literal;
        }

        Ok((value, ty))
    }

    /// Extract the values of a `vecN` from `expr`.
    ///
    /// Return the value of `expr`, whose type is `vecN<S>` for some
    /// vector size `N` and scalar `S`, as an array of `N` [`Literal`]
    /// values.
    ///
    /// Also return the type handle from the `Compose` expression.
    fn extract_vec(
        &mut self,
        expr: Handle<Expression>,
        allow_single: bool,
    ) -> Result<LiteralVector, ConstantEvaluatorError> {
        let span = self.expressions.get_span(expr);
        let expr = self.eval_zero_value_and_splat(expr, span)?;

        match self.expressions[expr] {
            Expression::Literal(literal) if allow_single => {
                Ok(LiteralVector::from_literal(literal))
            }
            Expression::Compose { ty, ref components } => {
                let mut components_out = ArrayVec::<Literal, { crate::VectorSize::MAX }>::new();
                for expr in
                    crate::proc::flatten_compose(ty, components, self.expressions, self.types)
                {
                    match self.expressions[expr] {
                        Expression::Literal(l) => components_out.push(l),
                        _ => return Err(ConstantEvaluatorError::InvalidMathArg),
                    }
                }
                LiteralVector::from_literal_vec(components_out)
            }
            _ => Err(ConstantEvaluatorError::InvalidMathArg),
        }
    }

    fn array_length(
        &mut self,
        array: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.expressions[array] {
            Expression::ZeroValue(ty) | Expression::Compose { ty, .. } => {
                match self.types[ty].inner {
                    TypeInner::Array { size, .. } => match size {
                        ArraySize::Constant(len) => {
                            let expr = Expression::Literal(Literal::U32(len.get()));
                            self.register_evaluated_expr(expr, span)
                        }
                        ArraySize::Pending(_) => Err(ConstantEvaluatorError::ArrayLengthOverridden),
                        ArraySize::Dynamic => Err(ConstantEvaluatorError::ArrayLengthDynamic),
                    },
                    _ => Err(ConstantEvaluatorError::InvalidArrayLengthArg),
                }
            }
            _ => Err(ConstantEvaluatorError::InvalidArrayLengthArg),
        }
    }

    fn access(
        &mut self,
        base: Handle<Expression>,
        index: usize,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.expressions[base] {
            Expression::ZeroValue(ty) => {
                let ty_inner = &self.types[ty].inner;
                let components = ty_inner
                    .components()
                    .ok_or(ConstantEvaluatorError::InvalidAccessBase)?;

                if index >= components as usize {
                    Err(ConstantEvaluatorError::InvalidAccessBase)
                } else {
                    let ty_res = ty_inner
                        .component_type(index)
                        .ok_or(ConstantEvaluatorError::InvalidAccessIndex)?;
                    let ty = match ty_res {
                        crate::proc::TypeResolution::Handle(ty) => ty,
                        crate::proc::TypeResolution::Value(inner) => {
                            self.types.insert(Type { name: None, inner }, span)
                        }
                    };
                    self.register_evaluated_expr(Expression::ZeroValue(ty), span)
                }
            }
            Expression::Splat { size, value } => {
                if index >= size as usize {
                    Err(ConstantEvaluatorError::InvalidAccessBase)
                } else {
                    Ok(value)
                }
            }
            Expression::Compose { ty, ref components } => {
                let _ = self.types[ty]
                    .inner
                    .components()
                    .ok_or(ConstantEvaluatorError::InvalidAccessBase)?;

                crate::proc::flatten_compose(ty, components, self.expressions, self.types)
                    .nth(index)
                    .ok_or(ConstantEvaluatorError::InvalidAccessIndex)
            }
            _ => Err(ConstantEvaluatorError::InvalidAccessBase),
        }
    }

    /// Lower [`ZeroValue`] and [`Splat`] expressions to [`Literal`] and [`Compose`] expressions.
    ///
    /// [`ZeroValue`]: Expression::ZeroValue
    /// [`Splat`]: Expression::Splat
    /// [`Literal`]: Expression::Literal
    /// [`Compose`]: Expression::Compose
    fn eval_zero_value_and_splat(
        &mut self,
        mut expr: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        // If expr is a Compose expression, eliminate ZeroValue and Splat expressions for
        // each of its components.
        if let Expression::Compose { ty, ref components } = self.expressions[expr] {
            let components = components
                .clone()
                .iter()
                .map(|component| self.eval_zero_value_and_splat(*component, span))
                .collect::<Result<_, _>>()?;
            expr = self.register_evaluated_expr(Expression::Compose { ty, components }, span)?;
        }

        // The result of the splat() for a Splat of a scalar ZeroValue is a
        // vector ZeroValue, so we must call eval_zero_value_impl() after
        // splat() in order to ensure we have no ZeroValues remaining.
        if let Expression::Splat { size, value } = self.expressions[expr] {
            expr = self.splat(value, size, span)?;
        }
        if let Expression::ZeroValue(ty) = self.expressions[expr] {
            expr = self.eval_zero_value_impl(ty, span)?;
        }
        Ok(expr)
    }

    /// Lower [`ZeroValue`] expressions to [`Literal`] and [`Compose`] expressions.
    ///
    /// [`ZeroValue`]: Expression::ZeroValue
    /// [`Literal`]: Expression::Literal
    /// [`Compose`]: Expression::Compose
    fn eval_zero_value(
        &mut self,
        expr: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.expressions[expr] {
            Expression::ZeroValue(ty) => self.eval_zero_value_impl(ty, span),
            _ => Ok(expr),
        }
    }

    /// Lower [`ZeroValue`] expressions to [`Literal`] and [`Compose`] expressions.
    ///
    /// [`ZeroValue`]: Expression::ZeroValue
    /// [`Literal`]: Expression::Literal
    /// [`Compose`]: Expression::Compose
    fn eval_zero_value_impl(
        &mut self,
        ty: Handle<Type>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        match self.types[ty].inner {
            TypeInner::Scalar(scalar) => {
                let expr = Expression::Literal(
                    Literal::zero(scalar).ok_or(ConstantEvaluatorError::TypeNotConstructible)?,
                );
                self.register_evaluated_expr(expr, span)
            }
            TypeInner::Vector { size, scalar } => {
                let scalar_ty = self.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Scalar(scalar),
                    },
                    span,
                );
                let el = self.eval_zero_value_impl(scalar_ty, span)?;
                let expr = Expression::Compose {
                    ty,
                    components: vec![el; size as usize],
                };
                self.register_evaluated_expr(expr, span)
            }
            TypeInner::Matrix {
                columns,
                rows,
                scalar,
            } => {
                let vec_ty = self.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector { size: rows, scalar },
                    },
                    span,
                );
                let el = self.eval_zero_value_impl(vec_ty, span)?;
                let expr = Expression::Compose {
                    ty,
                    components: vec![el; columns as usize],
                };
                self.register_evaluated_expr(expr, span)
            }
            TypeInner::Array {
                base,
                size: ArraySize::Constant(size),
                ..
            } => {
                let el = self.eval_zero_value_impl(base, span)?;
                let expr = Expression::Compose {
                    ty,
                    components: vec![el; size.get() as usize],
                };
                self.register_evaluated_expr(expr, span)
            }
            TypeInner::Struct { ref members, .. } => {
                let types: Vec<_> = members.iter().map(|m| m.ty).collect();
                let mut components = Vec::with_capacity(members.len());
                for ty in types {
                    components.push(self.eval_zero_value_impl(ty, span)?);
                }
                let expr = Expression::Compose { ty, components };
                self.register_evaluated_expr(expr, span)
            }
            _ => Err(ConstantEvaluatorError::TypeNotConstructible),
        }
    }

    /// Convert the scalar components of `expr` to `target`.
    ///
    /// Treat `span` as the location of the resulting expression.
    pub fn cast(
        &mut self,
        expr: Handle<Expression>,
        target: crate::Scalar,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        use crate::Scalar as Sc;

        let expr = self.eval_zero_value(expr, span)?;

        let make_error = || -> Result<_, ConstantEvaluatorError> {
            let from = format!("{:?} {:?}", expr, self.expressions[expr]);

            #[cfg(feature = "wgsl-in")]
            let to = target.to_wgsl_for_diagnostics();

            #[cfg(not(feature = "wgsl-in"))]
            let to = format!("{target:?}");

            Err(ConstantEvaluatorError::InvalidCastArg { from, to })
        };

        use crate::proc::type_methods::IntFloatLimits;

        let expr = match self.expressions[expr] {
            Expression::Literal(literal) => {
                let literal = match target {
                    Sc::I32 => Literal::I32(match literal {
                        Literal::I32(v) => v,
                        Literal::U32(v) => v as i32,
                        Literal::F32(v) => v.clamp(i32::min_float(), i32::max_float()) as i32,
                        Literal::F16(v) => f16::to_i32(&v).unwrap(), //Only None on NaN or Inf
                        Literal::Bool(v) => v as i32,
                        Literal::F64(_) | Literal::I64(_) | Literal::U64(_) => {
                            return make_error();
                        }
                        Literal::AbstractInt(v) => i32::try_from_abstract(v)?,
                        Literal::AbstractFloat(v) => i32::try_from_abstract(v)?,
                    }),
                    Sc::U32 => Literal::U32(match literal {
                        Literal::I32(v) => v as u32,
                        Literal::U32(v) => v,
                        Literal::F32(v) => v.clamp(u32::min_float(), u32::max_float()) as u32,
                        // max(0) avoids None due to negative, therefore only None on NaN or Inf
                        Literal::F16(v) => f16::to_u32(&v.max(f16::ZERO)).unwrap(),
                        Literal::Bool(v) => v as u32,
                        Literal::F64(_) | Literal::I64(_) | Literal::U64(_) => {
                            return make_error();
                        }
                        Literal::AbstractInt(v) => u32::try_from_abstract(v)?,
                        Literal::AbstractFloat(v) => u32::try_from_abstract(v)?,
                    }),
                    Sc::I64 => Literal::I64(match literal {
                        Literal::I32(v) => v as i64,
                        Literal::U32(v) => v as i64,
                        Literal::F32(v) => v.clamp(i64::min_float(), i64::max_float()) as i64,
                        Literal::Bool(v) => v as i64,
                        Literal::F64(v) => v.clamp(i64::min_float(), i64::max_float()) as i64,
                        Literal::I64(v) => v,
                        Literal::U64(v) => v as i64,
                        Literal::F16(v) => f16::to_i64(&v).unwrap(), //Only None on NaN or Inf
                        Literal::AbstractInt(v) => i64::try_from_abstract(v)?,
                        Literal::AbstractFloat(v) => i64::try_from_abstract(v)?,
                    }),
                    Sc::U64 => Literal::U64(match literal {
                        Literal::I32(v) => v as u64,
                        Literal::U32(v) => v as u64,
                        Literal::F32(v) => v.clamp(u64::min_float(), u64::max_float()) as u64,
                        Literal::Bool(v) => v as u64,
                        Literal::F64(v) => v.clamp(u64::min_float(), u64::max_float()) as u64,
                        Literal::I64(v) => v as u64,
                        Literal::U64(v) => v,
                        // max(0) avoids None due to negative, therefore only None on NaN or Inf
                        Literal::F16(v) => f16::to_u64(&v.max(f16::ZERO)).unwrap(),
                        Literal::AbstractInt(v) => u64::try_from_abstract(v)?,
                        Literal::AbstractFloat(v) => u64::try_from_abstract(v)?,
                    }),
                    Sc::F16 => Literal::F16(match literal {
                        Literal::F16(v) => v,
                        Literal::F32(v) => f16::from_f32(v),
                        Literal::F64(v) => f16::from_f64(v),
                        Literal::Bool(v) => f16::from_u32(v as u32).unwrap(),
                        Literal::I64(v) => f16::from_i64(v).unwrap(),
                        Literal::U64(v) => f16::from_u64(v).unwrap(),
                        Literal::I32(v) => f16::from_i32(v).unwrap(),
                        Literal::U32(v) => f16::from_u32(v).unwrap(),
                        Literal::AbstractFloat(v) => f16::try_from_abstract(v)?,
                        Literal::AbstractInt(v) => f16::try_from_abstract(v)?,
                    }),
                    Sc::F32 => Literal::F32(match literal {
                        Literal::I32(v) => v as f32,
                        Literal::U32(v) => v as f32,
                        Literal::F32(v) => v,
                        Literal::Bool(v) => v as u32 as f32,
                        Literal::F64(_) | Literal::I64(_) | Literal::U64(_) => {
                            return make_error();
                        }
                        Literal::F16(v) => f16::to_f32(v),
                        Literal::AbstractInt(v) => f32::try_from_abstract(v)?,
                        Literal::AbstractFloat(v) => f32::try_from_abstract(v)?,
                    }),
                    Sc::F64 => Literal::F64(match literal {
                        Literal::I32(v) => v as f64,
                        Literal::U32(v) => v as f64,
                        Literal::F16(v) => f16::to_f64(v),
                        Literal::F32(v) => v as f64,
                        Literal::F64(v) => v,
                        Literal::Bool(v) => v as u32 as f64,
                        Literal::I64(_) | Literal::U64(_) => return make_error(),
                        Literal::AbstractInt(v) => f64::try_from_abstract(v)?,
                        Literal::AbstractFloat(v) => f64::try_from_abstract(v)?,
                    }),
                    Sc::BOOL => Literal::Bool(match literal {
                        Literal::I32(v) => v != 0,
                        Literal::U32(v) => v != 0,
                        Literal::F32(v) => v != 0.0,
                        Literal::F16(v) => v != f16::zero(),
                        Literal::Bool(v) => v,
                        Literal::AbstractInt(v) => v != 0,
                        Literal::AbstractFloat(v) => v != 0.0,
                        Literal::F64(_) | Literal::I64(_) | Literal::U64(_) => {
                            return make_error();
                        }
                    }),
                    Sc::ABSTRACT_FLOAT => Literal::AbstractFloat(match literal {
                        Literal::AbstractInt(v) => {
                            // Overflow is forbidden, but inexact conversions
                            // are fine. The range of f64 is far larger than
                            // that of i64, so we don't have to check anything
                            // here.
                            v as f64
                        }
                        Literal::AbstractFloat(v) => v,
                        _ => return make_error(),
                    }),
                    Sc::ABSTRACT_INT => Literal::AbstractInt(match literal {
                        Literal::AbstractInt(v) => v,
                        _ => return make_error(),
                    }),
                    _ => {
                        log::debug!("Constant evaluator refused to convert value to {target:?}");
                        return make_error();
                    }
                };
                Expression::Literal(literal)
            }
            Expression::Compose {
                ty,
                components: ref src_components,
            } => {
                let ty_inner = match self.types[ty].inner {
                    TypeInner::Vector { size, .. } => TypeInner::Vector {
                        size,
                        scalar: target,
                    },
                    TypeInner::Matrix { columns, rows, .. } => TypeInner::Matrix {
                        columns,
                        rows,
                        scalar: target,
                    },
                    _ => return make_error(),
                };

                let mut components = src_components.clone();
                for component in &mut components {
                    *component = self.cast(*component, target, span)?;
                }

                let ty = self.types.insert(
                    Type {
                        name: None,
                        inner: ty_inner,
                    },
                    span,
                );

                Expression::Compose { ty, components }
            }
            Expression::Splat { size, value } => {
                let value_span = self.expressions.get_span(value);
                let cast_value = self.cast(value, target, value_span)?;
                Expression::Splat {
                    size,
                    value: cast_value,
                }
            }
            _ => return make_error(),
        };

        self.register_evaluated_expr(expr, span)
    }

    /// Convert the scalar leaves of  `expr` to `target`, handling arrays.
    ///
    /// `expr` must be a `Compose` expression whose type is a scalar, vector,
    /// matrix, or nested arrays of such.
    ///
    /// This is basically the same as the [`cast`] method, except that that
    /// should only handle Naga [`As`] expressions, which cannot convert arrays.
    ///
    /// Treat `span` as the location of the resulting expression.
    ///
    /// [`cast`]: ConstantEvaluator::cast
    /// [`As`]: crate::Expression::As
    pub fn cast_array(
        &mut self,
        expr: Handle<Expression>,
        target: crate::Scalar,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let expr = self.check_and_get(expr)?;

        let Expression::Compose { ty, ref components } = self.expressions[expr] else {
            return self.cast(expr, target, span);
        };

        let TypeInner::Array {
            base: _,
            size,
            stride: _,
        } = self.types[ty].inner
        else {
            return self.cast(expr, target, span);
        };

        let mut components = components.clone();
        for component in &mut components {
            *component = self.cast_array(*component, target, span)?;
        }

        let first = components.first().unwrap();
        let new_base = match self.resolve_type(*first)? {
            crate::proc::TypeResolution::Handle(ty) => ty,
            crate::proc::TypeResolution::Value(inner) => {
                self.types.insert(Type { name: None, inner }, span)
            }
        };
        let mut layouter = core::mem::take(self.layouter);
        layouter.update(self.to_ctx()).unwrap();
        *self.layouter = layouter;

        let new_base_stride = self.layouter[new_base].to_stride();
        let new_array_ty = self.types.insert(
            Type {
                name: None,
                inner: TypeInner::Array {
                    base: new_base,
                    size,
                    stride: new_base_stride,
                },
            },
            span,
        );

        let compose = Expression::Compose {
            ty: new_array_ty,
            components,
        };
        self.register_evaluated_expr(compose, span)
    }

    fn unary_op(
        &mut self,
        op: UnaryOperator,
        expr: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let expr = self.eval_zero_value_and_splat(expr, span)?;

        let expr = match self.expressions[expr] {
            Expression::Literal(value) => Expression::Literal(match op {
                UnaryOperator::Negate => match value {
                    Literal::I32(v) => Literal::I32(v.wrapping_neg()),
                    Literal::I64(v) => Literal::I64(v.wrapping_neg()),
                    Literal::F32(v) => Literal::F32(-v),
                    Literal::F16(v) => Literal::F16(-v),
                    Literal::F64(v) => Literal::F64(-v),
                    Literal::AbstractInt(v) => Literal::AbstractInt(v.wrapping_neg()),
                    Literal::AbstractFloat(v) => Literal::AbstractFloat(-v),
                    _ => return Err(ConstantEvaluatorError::InvalidUnaryOpArg),
                },
                UnaryOperator::LogicalNot => match value {
                    Literal::Bool(v) => Literal::Bool(!v),
                    _ => return Err(ConstantEvaluatorError::InvalidUnaryOpArg),
                },
                UnaryOperator::BitwiseNot => match value {
                    Literal::I32(v) => Literal::I32(!v),
                    Literal::I64(v) => Literal::I64(!v),
                    Literal::U32(v) => Literal::U32(!v),
                    Literal::U64(v) => Literal::U64(!v),
                    Literal::AbstractInt(v) => Literal::AbstractInt(!v),
                    _ => return Err(ConstantEvaluatorError::InvalidUnaryOpArg),
                },
            }),
            Expression::Compose {
                ty,
                components: ref src_components,
            } => {
                match self.types[ty].inner {
                    TypeInner::Vector { .. } | TypeInner::Matrix { .. } => (),
                    _ => return Err(ConstantEvaluatorError::InvalidUnaryOpArg),
                }

                let mut components = src_components.clone();
                for component in &mut components {
                    *component = self.unary_op(op, *component, span)?;
                }

                Expression::Compose { ty, components }
            }
            _ => return Err(ConstantEvaluatorError::InvalidUnaryOpArg),
        };

        self.register_evaluated_expr(expr, span)
    }

    fn binary_op(
        &mut self,
        op: BinaryOperator,
        left: Handle<Expression>,
        right: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let left = self.eval_zero_value_and_splat(left, span)?;
        let right = self.eval_zero_value_and_splat(right, span)?;

        // Note: in most cases constant evaluation checks for overflow, but for
        // i32/u32, it uses wrapping arithmetic. See
        // <https://gpuweb.github.io/gpuweb/wgsl/#integer-types>.

        let expr = match (&self.expressions[left], &self.expressions[right]) {
            (&Expression::Literal(left_value), &Expression::Literal(right_value)) => {
                if !matches!(op, BinaryOperator::ShiftLeft | BinaryOperator::ShiftRight)
                    && core::mem::discriminant(&left_value) != core::mem::discriminant(&right_value)
                {
                    return Err(ConstantEvaluatorError::InvalidBinaryOpArgs);
                }

                let literal = match op {
                    BinaryOperator::Equal => Literal::Bool(left_value == right_value),
                    BinaryOperator::NotEqual => Literal::Bool(left_value != right_value),
                    BinaryOperator::Less => Literal::Bool(left_value < right_value),
                    BinaryOperator::LessEqual => Literal::Bool(left_value <= right_value),
                    BinaryOperator::Greater => Literal::Bool(left_value > right_value),
                    BinaryOperator::GreaterEqual => Literal::Bool(left_value >= right_value),

                    _ => match (left_value, right_value) {
                        (Literal::I32(a), Literal::I32(b)) => Literal::I32(match op {
                            BinaryOperator::Add => a.wrapping_add(b),
                            BinaryOperator::Subtract => a.wrapping_sub(b),
                            BinaryOperator::Multiply => a.wrapping_mul(b),
                            BinaryOperator::Divide => {
                                if b == 0 {
                                    return Err(ConstantEvaluatorError::DivisionByZero);
                                } else {
                                    a.wrapping_div(b)
                                }
                            }
                            BinaryOperator::Modulo => {
                                if b == 0 {
                                    return Err(ConstantEvaluatorError::RemainderByZero);
                                } else {
                                    a.wrapping_rem(b)
                                }
                            }
                            BinaryOperator::And => a & b,
                            BinaryOperator::ExclusiveOr => a ^ b,
                            BinaryOperator::InclusiveOr => a | b,
                            _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                        }),
                        (Literal::I32(a), Literal::U32(b)) => Literal::I32(match op {
                            BinaryOperator::ShiftLeft => {
                                if (if a.is_negative() { !a } else { a }).leading_zeros() <= b {
                                    return Err(ConstantEvaluatorError::Overflow("<<".to_string()));
                                }
                                a.checked_shl(b)
                                    .ok_or(ConstantEvaluatorError::ShiftedMoreThan32Bits)?
                            }
                            BinaryOperator::ShiftRight => a
                                .checked_shr(b)
                                .ok_or(ConstantEvaluatorError::ShiftedMoreThan32Bits)?,
                            _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                        }),
                        (Literal::U32(a), Literal::U32(b)) => Literal::U32(match op {
                            BinaryOperator::Add => a.wrapping_add(b),
                            BinaryOperator::Subtract => a.wrapping_sub(b),
                            BinaryOperator::Multiply => a.wrapping_mul(b),
                            BinaryOperator::Divide => a
                                .checked_div(b)
                                .ok_or(ConstantEvaluatorError::DivisionByZero)?,
                            BinaryOperator::Modulo => a
                                .checked_rem(b)
                                .ok_or(ConstantEvaluatorError::RemainderByZero)?,
                            BinaryOperator::And => a & b,
                            BinaryOperator::ExclusiveOr => a ^ b,
                            BinaryOperator::InclusiveOr => a | b,
                            BinaryOperator::ShiftLeft => a
                                .checked_mul(
                                    1u32.checked_shl(b)
                                        .ok_or(ConstantEvaluatorError::ShiftedMoreThan32Bits)?,
                                )
                                .ok_or(ConstantEvaluatorError::Overflow("<<".to_string()))?,
                            BinaryOperator::ShiftRight => a
                                .checked_shr(b)
                                .ok_or(ConstantEvaluatorError::ShiftedMoreThan32Bits)?,
                            _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                        }),
                        (Literal::F32(a), Literal::F32(b)) => Literal::F32(match op {
                            BinaryOperator::Add => a + b,
                            BinaryOperator::Subtract => a - b,
                            BinaryOperator::Multiply => a * b,
                            BinaryOperator::Divide => a / b,
                            BinaryOperator::Modulo => a % b,
                            _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                        }),
                        (Literal::AbstractInt(a), Literal::U32(b)) => {
                            Literal::AbstractInt(match op {
                                BinaryOperator::ShiftLeft => {
                                    if (if a.is_negative() { !a } else { a }).leading_zeros() <= b {
                                        return Err(ConstantEvaluatorError::Overflow(
                                            "<<".to_string(),
                                        ));
                                    }
                                    a.checked_shl(b).unwrap_or(0)
                                }
                                BinaryOperator::ShiftRight => a.checked_shr(b).unwrap_or(0),
                                _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                            })
                        }
                        (Literal::F16(a), Literal::F16(b)) => {
                            let result = match op {
                                BinaryOperator::Add => a + b,
                                BinaryOperator::Subtract => a - b,
                                BinaryOperator::Multiply => a * b,
                                BinaryOperator::Divide => a / b,
                                BinaryOperator::Modulo => a % b,
                                _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                            };
                            if !result.is_finite() {
                                return Err(ConstantEvaluatorError::Overflow(format!("{op:?}")));
                            }
                            Literal::F16(result)
                        }
                        (Literal::AbstractInt(a), Literal::AbstractInt(b)) => {
                            Literal::AbstractInt(match op {
                                BinaryOperator::Add => a.checked_add(b).ok_or_else(|| {
                                    ConstantEvaluatorError::Overflow("addition".into())
                                })?,
                                BinaryOperator::Subtract => a.checked_sub(b).ok_or_else(|| {
                                    ConstantEvaluatorError::Overflow("subtraction".into())
                                })?,
                                BinaryOperator::Multiply => a.checked_mul(b).ok_or_else(|| {
                                    ConstantEvaluatorError::Overflow("multiplication".into())
                                })?,
                                BinaryOperator::Divide => a.checked_div(b).ok_or_else(|| {
                                    if b == 0 {
                                        ConstantEvaluatorError::DivisionByZero
                                    } else {
                                        ConstantEvaluatorError::Overflow("division".into())
                                    }
                                })?,
                                BinaryOperator::Modulo => a.checked_rem(b).ok_or_else(|| {
                                    if b == 0 {
                                        ConstantEvaluatorError::RemainderByZero
                                    } else {
                                        ConstantEvaluatorError::Overflow("remainder".into())
                                    }
                                })?,
                                BinaryOperator::And => a & b,
                                BinaryOperator::ExclusiveOr => a ^ b,
                                BinaryOperator::InclusiveOr => a | b,
                                _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                            })
                        }
                        (Literal::AbstractFloat(a), Literal::AbstractFloat(b)) => {
                            let result = match op {
                                BinaryOperator::Add => a + b,
                                BinaryOperator::Subtract => a - b,
                                BinaryOperator::Multiply => a * b,
                                BinaryOperator::Divide => a / b,
                                BinaryOperator::Modulo => a % b,
                                _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                            };
                            if !result.is_finite() {
                                return Err(ConstantEvaluatorError::Overflow(format!("{op:?}")));
                            }
                            Literal::AbstractFloat(result)
                        }
                        (Literal::Bool(a), Literal::Bool(b)) => Literal::Bool(match op {
                            BinaryOperator::LogicalAnd => a && b,
                            BinaryOperator::LogicalOr => a || b,
                            _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                        }),
                        _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
                    },
                };
                Expression::Literal(literal)
            }
            (
                &Expression::Compose {
                    components: ref src_components,
                    ty,
                },
                &Expression::Literal(_),
            ) => {
                if !is_allowed_compose_literal_op(&self.types[ty].inner, op) {
                    return Err(ConstantEvaluatorError::InvalidBinaryOpArgs);
                }
                let mut components = src_components.clone();
                for component in &mut components {
                    *component = self.binary_op(op, *component, right, span)?;
                }
                Expression::Compose { ty, components }
            }
            (
                &Expression::Literal(_),
                &Expression::Compose {
                    components: ref src_components,
                    ty,
                },
            ) => {
                if !is_allowed_compose_literal_op(&self.types[ty].inner, op) {
                    return Err(ConstantEvaluatorError::InvalidBinaryOpArgs);
                }
                let mut components = src_components.clone();
                for component in &mut components {
                    *component = self.binary_op(op, left, *component, span)?;
                }
                Expression::Compose { ty, components }
            }
            (
                &Expression::Compose {
                    components: ref left_components,
                    ty: left_ty,
                },
                &Expression::Compose {
                    components: ref right_components,
                    ty: right_ty,
                },
            ) => {
                // We have to make a copy of the component lists, because the
                // call to `binary_op_vector` needs `&mut self`, but `self` owns
                // the component lists.
                let left_flattened = crate::proc::flatten_compose(
                    left_ty,
                    left_components,
                    self.expressions,
                    self.types,
                )
                .collect::<Vec<_>>();
                let right_flattened = crate::proc::flatten_compose(
                    right_ty,
                    right_components,
                    self.expressions,
                    self.types,
                )
                .collect::<Vec<_>>();

                self.binary_op_compose(
                    op,
                    &left_flattened,
                    &right_flattened,
                    left_ty,
                    right_ty,
                    span,
                )?
            }
            _ => return Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
        };

        return self.register_evaluated_expr(expr, span);

        fn is_allowed_compose_literal_op(compose_ty: &TypeInner, op: BinaryOperator) -> bool {
            let is_numeric_vec = matches!(
                compose_ty, TypeInner::Vector { scalar, .. }
                if scalar.kind != ScalarKind::Bool
            );
            let is_allowed_vec_scalar_op = matches!(
                op,
                BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide
                    | BinaryOperator::Modulo
            );
            let is_mat = matches!(compose_ty, TypeInner::Matrix { .. });
            let is_allowed_mat_scalar_op = matches!(op, BinaryOperator::Multiply);
            is_numeric_vec && is_allowed_vec_scalar_op || is_mat && is_allowed_mat_scalar_op
        }
    }

    fn binary_op_compose(
        &mut self,
        op: BinaryOperator,
        left_components: &[Handle<Expression>],
        right_components: &[Handle<Expression>],
        left_ty: Handle<Type>,
        right_ty: Handle<Type>,
        span: Span,
    ) -> Result<Expression, ConstantEvaluatorError> {
        match (&self.types[left_ty].inner, &self.types[right_ty].inner) {
            // Binary operation on vector-vector
            (
                &TypeInner::Vector {
                    size: left_size, ..
                },
                &TypeInner::Vector {
                    size: right_size, ..
                },
            ) if left_size == right_size => self.binary_op_vector(
                op,
                left_size,
                left_components,
                right_components,
                left_ty,
                span,
            ),
            // Binary operation on vector-matrix
            (
                &TypeInner::Vector { size, .. },
                &TypeInner::Matrix {
                    columns,
                    rows,
                    scalar,
                },
            ) if op == BinaryOperator::Multiply && size == rows => self.multiply_vector_matrix(
                left_components,
                right_components,
                columns,
                scalar,
                span,
            ),
            // Binary operation on matrix-vector
            (
                &TypeInner::Matrix {
                    columns,
                    rows,
                    scalar,
                },
                &TypeInner::Vector { size, .. },
            ) if op == BinaryOperator::Multiply && size == columns => {
                self.multiply_matrix_vector(left_components, right_components, rows, scalar, span)
            }
            // Binary operation on matrix-matrix
            (
                &TypeInner::Matrix {
                    columns: left_columns,
                    rows: left_rows,
                    scalar,
                },
                &TypeInner::Matrix {
                    columns: right_columns,
                    rows: right_rows,
                    ..
                },
            ) => match op {
                BinaryOperator::Add | BinaryOperator::Subtract
                    if left_columns == right_columns && left_rows == right_rows =>
                {
                    let components = left_components
                        .iter()
                        .zip(right_components)
                        .map(|(&left, &right)| self.binary_op(op, left, right, span))
                        .collect::<Result<Vec<_>, _>>()?;
                    Ok(Expression::Compose {
                        ty: left_ty,
                        components,
                    })
                }
                BinaryOperator::Multiply if left_columns == right_rows => self
                    .multiply_matrix_matrix(
                        left_components,
                        right_components,
                        left_rows,
                        right_columns,
                        scalar,
                        span,
                    ),
                _ => Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
            },
            _ => Err(ConstantEvaluatorError::InvalidBinaryOpArgs),
        }
    }

    fn binary_op_vector(
        &mut self,
        op: BinaryOperator,
        size: crate::VectorSize,
        left_components: &[Handle<Expression>],
        right_components: &[Handle<Expression>],
        left_ty: Handle<Type>,
        span: Span,
    ) -> Result<Expression, ConstantEvaluatorError> {
        let ty = match op {
            // Relational operators produce vectors of booleans.
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Less
            | BinaryOperator::LessEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterEqual => self.types.insert(
                Type {
                    name: None,
                    inner: TypeInner::Vector {
                        size,
                        scalar: crate::Scalar::BOOL,
                    },
                },
                span,
            ),

            // Other operators produce the same type as their left
            // operand.
            BinaryOperator::Add
            | BinaryOperator::Subtract
            | BinaryOperator::Multiply
            | BinaryOperator::Divide
            | BinaryOperator::Modulo
            | BinaryOperator::And
            | BinaryOperator::ExclusiveOr
            | BinaryOperator::InclusiveOr
            | BinaryOperator::ShiftLeft
            | BinaryOperator::ShiftRight => left_ty,

            BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                // Not supported on vectors
                return Err(ConstantEvaluatorError::InvalidBinaryOpArgs);
            }
        };

        let components = left_components
            .iter()
            .zip(right_components)
            .map(|(&left, &right)| self.binary_op(op, left, right, span))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Expression::Compose { ty, components })
    }

    fn multiply_vector_matrix(
        &mut self,
        vec_components: &[Handle<Expression>],
        mat_components: &[Handle<Expression>],
        mat_columns: crate::VectorSize,
        scalar: crate::Scalar,
        span: Span,
    ) -> Result<Expression, ConstantEvaluatorError> {
        let ty = self.types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: mat_columns,
                    scalar,
                },
            },
            span,
        );
        let components = mat_components
            .iter()
            .map(|&column| {
                let Expression::Compose { ref components, .. } = self.expressions[column] else {
                    unreachable!()
                };
                self.dot_exprs(
                    vec_components.iter().cloned(),
                    components.clone().into_iter(),
                    span,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Expression::Compose { ty, components })
    }

    fn multiply_matrix_vector(
        &mut self,
        mat_components: &[Handle<Expression>],
        vec_components: &[Handle<Expression>],
        mat_rows: crate::VectorSize,
        scalar: crate::Scalar,
        span: Span,
    ) -> Result<Expression, ConstantEvaluatorError> {
        let ty = self.types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: mat_rows,
                    scalar,
                },
            },
            span,
        );

        let flatten = self.flatten_matrix(mat_components);
        let nr = mat_rows as usize;
        let components = (0..nr)
            .map(|r| {
                let row = flatten.iter().skip(r).step_by(nr).cloned();
                self.dot_exprs(row, vec_components.iter().cloned(), span)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Expression::Compose { ty, components })
    }

    fn multiply_matrix_matrix(
        &mut self,
        left_components: &[Handle<Expression>],
        right_components: &[Handle<Expression>],
        left_rows: crate::VectorSize,
        right_columns: crate::VectorSize,
        scalar: crate::Scalar,
        span: Span,
    ) -> Result<Expression, ConstantEvaluatorError> {
        let left_nc = left_components.len();
        let left_nr = left_rows as usize;
        let right_nc = right_columns as usize;
        let right_nr = left_nc;

        let mut result = Vec::with_capacity(right_nc);
        let result_ty = self.types.insert(
            Type {
                name: None,
                inner: TypeInner::Matrix {
                    columns: right_columns,
                    rows: left_rows,
                    scalar,
                },
            },
            span,
        );
        let result_column_ty = self.types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: left_rows,
                    scalar,
                },
            },
            span,
        );

        let left_flattened = self.flatten_matrix(left_components);
        let right_flattened = self.flatten_matrix(right_components);
        for c in 0..right_nc {
            let result_column = (0..left_nr)
                .map(|r| {
                    let row = left_flattened.iter().skip(r).step_by(left_nr);
                    let column = right_flattened.iter().skip(c * right_nr).take(right_nr);
                    self.dot_exprs(row.cloned(), column.cloned(), span)
                })
                .collect::<Result<Vec<_>, _>>()?;
            let expr = Expression::Compose {
                ty: result_column_ty,
                components: result_column,
            };
            let handle = self.register_evaluated_expr(expr, span)?;
            result.push(handle);
        }
        Ok(Expression::Compose {
            ty: result_ty,
            components: result,
        })
    }

    fn flatten_matrix(&self, columns: &[Handle<Expression>]) -> ArrayVec<Handle<Expression>, 16> {
        let mut flattened = ArrayVec::<_, 16>::new();
        for &column in columns {
            let Expression::Compose { ref components, .. } = self.expressions[column] else {
                unreachable!()
            };
            flattened.extend(components.iter().cloned());
        }
        flattened
    }

    fn dot_exprs(
        &mut self,
        left: impl Iterator<Item = Handle<Expression>>,
        right: impl Iterator<Item = Handle<Expression>>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let mut acc = None;
        for (l, r) in left.zip(right) {
            let result = self.binary_op(BinaryOperator::Multiply, l, r, span)?;
            match acc.as_mut() {
                Some(acc) => *acc = self.binary_op(BinaryOperator::Add, *acc, result, span)?,
                None => acc = Some(result),
            }
        }
        Ok(acc.unwrap())
    }

    fn relational(
        &mut self,
        fun: RelationalFunction,
        arg: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let arg = self.eval_zero_value_and_splat(arg, span)?;
        match fun {
            RelationalFunction::All | RelationalFunction::Any => match self.expressions[arg] {
                Expression::Literal(Literal::Bool(_)) => Ok(arg),
                Expression::Compose { ty, ref components }
                    if matches!(self.types[ty].inner, TypeInner::Vector { .. }) =>
                {
                    let mut bool_components = ArrayVec::<bool, { crate::VectorSize::MAX }>::new();
                    for component in
                        crate::proc::flatten_compose(ty, components, self.expressions, self.types)
                    {
                        match self.expressions[component] {
                            Expression::Literal(Literal::Bool(val)) => {
                                bool_components.push(val);
                            }
                            _ => {
                                return Err(ConstantEvaluatorError::InvalidRelationalArg(fun));
                            }
                        }
                    }
                    let components = bool_components;
                    let result = match fun {
                        RelationalFunction::All => components.iter().all(|c| *c),
                        RelationalFunction::Any => components.iter().any(|c| *c),
                        _ => unreachable!(),
                    };
                    self.register_evaluated_expr(Expression::Literal(Literal::Bool(result)), span)
                }
                _ => Err(ConstantEvaluatorError::InvalidRelationalArg(fun)),
            },
            _ => Err(ConstantEvaluatorError::NotImplemented(format!(
                "{fun:?} built-in function"
            ))),
        }
    }

    /// Deep copy `expr` from `expressions` into `self.expressions`.
    ///
    /// Return the root of the new copy.
    ///
    /// This is used when we're evaluating expressions in a function's
    /// expression arena that refer to a constant: we need to copy the
    /// constant's value into the function's arena so we can operate on it.
    fn copy_from(
        &mut self,
        expr: Handle<Expression>,
        expressions: &Arena<Expression>,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let span = expressions.get_span(expr);
        match expressions[expr] {
            ref expr @ (Expression::Literal(_)
            | Expression::Constant(_)
            | Expression::ZeroValue(_)) => self.register_evaluated_expr(expr.clone(), span),
            Expression::Compose { ty, ref components } => {
                let mut components = components.clone();
                for component in &mut components {
                    *component = self.copy_from(*component, expressions)?;
                }
                self.register_evaluated_expr(Expression::Compose { ty, components }, span)
            }
            Expression::Splat { size, value } => {
                let value = self.copy_from(value, expressions)?;
                self.register_evaluated_expr(Expression::Splat { size, value }, span)
            }
            _ => {
                log::debug!("copy_from: SubexpressionsAreNotConstant");
                Err(ConstantEvaluatorError::SubexpressionsAreNotConstant)
            }
        }
    }

    /// Returns the total number of components, after flattening, of a vector compose expression.
    fn vector_compose_flattened_size(
        &self,
        components: &[Handle<Expression>],
    ) -> Result<usize, ConstantEvaluatorError> {
        components
            .iter()
            .try_fold(0, |acc, c| -> Result<_, ConstantEvaluatorError> {
                let size = match *self.resolve_type(*c)?.inner_with(self.types) {
                    TypeInner::Scalar(_) => 1,
                    // We trust that the vector size of `component` is correct,
                    // as it will have already been validated when `component`
                    // was registered.
                    TypeInner::Vector { size, .. } => size as usize,
                    _ => return Err(ConstantEvaluatorError::InvalidVectorComposeComponent),
                };
                Ok(acc + size)
            })
    }

    fn register_evaluated_expr(
        &mut self,
        expr: Expression,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        // It suffices to only check_literal_value() for `Literal` expressions,
        // since we only register one expression at a time, `Compose`
        // expressions can only refer to other expressions, and `ZeroValue`
        // expressions are always okay.
        if let Expression::Literal(literal) = expr {
            crate::valid::check_literal_value(literal)?;
        }

        // Ensure vector composes contain the correct number of components. We
        // do so here when each compose is registered to avoid having to deal
        // with the mess each time the compose is used in another expression.
        if let Expression::Compose { ty, ref components } = expr {
            if let TypeInner::Vector { size, scalar: _ } = self.types[ty].inner {
                let expected = size as usize;
                let actual = self.vector_compose_flattened_size(components)?;
                if expected != actual {
                    return Err(ConstantEvaluatorError::InvalidVectorComposeLength {
                        expected,
                        actual,
                    });
                }
            }
        }

        Ok(self.append_expr(expr, span, ExpressionKind::Const))
    }

    fn append_expr(
        &mut self,
        expr: Expression,
        span: Span,
        expr_type: ExpressionKind,
    ) -> Handle<Expression> {
        let h = match self.behavior {
            Behavior::Wgsl(
                WgslRestrictions::Runtime(ref mut function_local_data)
                | WgslRestrictions::Const(Some(ref mut function_local_data)),
            )
            | Behavior::Glsl(GlslRestrictions::Runtime(ref mut function_local_data)) => {
                let is_running = function_local_data.emitter.is_running();
                let needs_pre_emit = expr.needs_pre_emit();
                if is_running && needs_pre_emit {
                    function_local_data
                        .block
                        .extend(function_local_data.emitter.finish(self.expressions));
                    let h = self.expressions.append(expr, span);
                    function_local_data.emitter.start(self.expressions);
                    h
                } else {
                    self.expressions.append(expr, span)
                }
            }
            _ => self.expressions.append(expr, span),
        };
        self.expression_kind_tracker.insert(h, expr_type);
        h
    }

    /// Resolve the type of `expr` if it is a constant expression.
    ///
    /// If `expr` was evaluated to a constant, returns its type.
    /// Otherwise, returns an error.
    fn resolve_type(
        &self,
        expr: Handle<Expression>,
    ) -> Result<crate::proc::TypeResolution, ConstantEvaluatorError> {
        use crate::proc::TypeResolution as Tr;
        use crate::Expression as Ex;
        let resolution = match self.expressions[expr] {
            Ex::Literal(ref literal) => Tr::Value(literal.ty_inner()),
            Ex::Constant(c) => Tr::Handle(self.constants[c].ty),
            Ex::ZeroValue(ty) | Ex::Compose { ty, .. } => Tr::Handle(ty),
            Ex::Splat { size, value } => {
                let Tr::Value(TypeInner::Scalar(scalar)) = self.resolve_type(value)? else {
                    return Err(ConstantEvaluatorError::SplatScalarOnly);
                };
                Tr::Value(TypeInner::Vector { scalar, size })
            }
            _ => {
                log::debug!("resolve_type: SubexpressionsAreNotConstant");
                return Err(ConstantEvaluatorError::SubexpressionsAreNotConstant);
            }
        };

        Ok(resolution)
    }

    fn select(
        &mut self,
        reject: Handle<Expression>,
        accept: Handle<Expression>,
        condition: Handle<Expression>,
        span: Span,
    ) -> Result<Handle<Expression>, ConstantEvaluatorError> {
        let mut arg = |arg| self.eval_zero_value_and_splat(arg, span);

        let reject = arg(reject)?;
        let accept = arg(accept)?;
        let condition = arg(condition)?;

        let select_single_component =
            |this: &mut Self, reject_scalar, reject, accept, condition| {
                let accept = this.cast(accept, reject_scalar, span)?;
                if condition {
                    Ok(accept)
                } else {
                    Ok(reject)
                }
            };

        match (&self.expressions[reject], &self.expressions[accept]) {
            (&Expression::Literal(reject_lit), &Expression::Literal(_accept_lit)) => {
                let reject_scalar = reject_lit.scalar();
                let &Expression::Literal(Literal::Bool(condition)) = &self.expressions[condition]
                else {
                    return Err(ConstantEvaluatorError::SelectScalarConditionNotABool);
                };
                select_single_component(self, reject_scalar, reject, accept, condition)
            }
            (
                &Expression::Compose {
                    ty: reject_ty,
                    components: ref reject_components,
                },
                &Expression::Compose {
                    ty: accept_ty,
                    components: ref accept_components,
                },
            ) => {
                let ty_deets = |ty| {
                    let (size, scalar) = self.types[ty].inner.vector_size_and_scalar().unwrap();
                    (size.unwrap(), scalar)
                };

                let expected_vec_size = {
                    let [(reject_vec_size, _), (accept_vec_size, _)] =
                        [reject_ty, accept_ty].map(ty_deets);

                    if reject_vec_size != accept_vec_size {
                        return Err(ConstantEvaluatorError::SelectVecRejectAcceptSizeMismatch {
                            reject: reject_vec_size,
                            accept: accept_vec_size,
                        });
                    }
                    reject_vec_size
                };

                let condition_components = match self.expressions[condition] {
                    Expression::Literal(Literal::Bool(condition)) => {
                        vec![condition; (expected_vec_size as u8).into()]
                    }
                    Expression::Compose {
                        ty: condition_ty,
                        components: ref condition_components,
                    } => {
                        let (condition_vec_size, condition_scalar) = ty_deets(condition_ty);
                        if condition_scalar.kind != ScalarKind::Bool {
                            return Err(ConstantEvaluatorError::SelectConditionNotAVecBool);
                        }
                        if condition_vec_size != expected_vec_size {
                            return Err(ConstantEvaluatorError::SelectConditionVecSizeMismatch);
                        }
                        condition_components
                            .iter()
                            .copied()
                            .map(|component| match &self.expressions[component] {
                                &Expression::Literal(Literal::Bool(condition)) => condition,
                                _ => unreachable!(),
                            })
                            .collect()
                    }

                    _ => return Err(ConstantEvaluatorError::SelectConditionNotAVecBool),
                };

                let evaluated = Expression::Compose {
                    ty: reject_ty,
                    components: reject_components
                        .clone()
                        .into_iter()
                        .zip(accept_components.clone().into_iter())
                        .zip(condition_components.into_iter())
                        .map(|((reject, accept), condition)| {
                            let reject_scalar = match &self.expressions[reject] {
                                &Expression::Literal(lit) => lit.scalar(),
                                _ => unreachable!(),
                            };
                            select_single_component(self, reject_scalar, reject, accept, condition)
                        })
                        .collect::<Result<_, _>>()?,
                };
                self.register_evaluated_expr(evaluated, span)
            }
            _ => Err(ConstantEvaluatorError::SelectAcceptRejectTypeMismatch),
        }
    }
}

fn first_trailing_bit(concrete_int: ConcreteInt<1>) -> ConcreteInt<1> {
    // NOTE: Bit indices for this built-in start at 0 at the "right" (or LSB). For example, a value
    // of 1 means the least significant bit is set. Therefore, an input of `0x[80 00…]` would
    // return a right-to-left bit index of 0.
    let trailing_zeros_to_bit_idx = |e: u32| -> u32 {
        match e {
            idx @ 0..=31 => idx,
            32 => u32::MAX,
            _ => unreachable!(),
        }
    };
    match concrete_int {
        ConcreteInt::U32([e]) => ConcreteInt::U32([trailing_zeros_to_bit_idx(e.trailing_zeros())]),
        ConcreteInt::I32([e]) => {
            ConcreteInt::I32([trailing_zeros_to_bit_idx(e.trailing_zeros()) as i32])
        }
    }
}

#[test]
fn first_trailing_bit_smoke() {
    assert_eq!(
        first_trailing_bit(ConcreteInt::I32([0])),
        ConcreteInt::I32([-1])
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::I32([1])),
        ConcreteInt::I32([0])
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::I32([2])),
        ConcreteInt::I32([1])
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::I32([-1])),
        ConcreteInt::I32([0]),
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::I32([i32::MIN])),
        ConcreteInt::I32([31]),
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::I32([i32::MAX])),
        ConcreteInt::I32([0]),
    );
    for idx in 0..32 {
        assert_eq!(
            first_trailing_bit(ConcreteInt::I32([1 << idx])),
            ConcreteInt::I32([idx])
        )
    }

    assert_eq!(
        first_trailing_bit(ConcreteInt::U32([0])),
        ConcreteInt::U32([u32::MAX])
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::U32([1])),
        ConcreteInt::U32([0])
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::U32([2])),
        ConcreteInt::U32([1])
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::U32([1 << 31])),
        ConcreteInt::U32([31]),
    );
    assert_eq!(
        first_trailing_bit(ConcreteInt::U32([u32::MAX])),
        ConcreteInt::U32([0]),
    );
    for idx in 0..32 {
        assert_eq!(
            first_trailing_bit(ConcreteInt::U32([1 << idx])),
            ConcreteInt::U32([idx])
        )
    }
}

fn first_leading_bit(concrete_int: ConcreteInt<1>) -> ConcreteInt<1> {
    // NOTE: Bit indices for this built-in start at 0 at the "right" (or LSB). For example, 1 means
    // the least significant bit is set. Therefore, an input of 1 would return a right-to-left bit
    // index of 0.
    let rtl_to_ltr_bit_idx = |e: u32| -> u32 {
        match e {
            idx @ 0..=31 => 31 - idx,
            32 => u32::MAX,
            _ => unreachable!(),
        }
    };
    match concrete_int {
        ConcreteInt::I32([e]) => ConcreteInt::I32([{
            let rtl_bit_index = if e.is_negative() {
                e.leading_ones()
            } else {
                e.leading_zeros()
            };
            rtl_to_ltr_bit_idx(rtl_bit_index) as i32
        }]),
        ConcreteInt::U32([e]) => ConcreteInt::U32([rtl_to_ltr_bit_idx(e.leading_zeros())]),
    }
}

#[test]
fn first_leading_bit_smoke() {
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([-1])),
        ConcreteInt::I32([-1])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([0])),
        ConcreteInt::I32([-1])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([1])),
        ConcreteInt::I32([0])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([-2])),
        ConcreteInt::I32([0])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([1234 + 4567])),
        ConcreteInt::I32([12])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([i32::MAX])),
        ConcreteInt::I32([30])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::I32([i32::MIN])),
        ConcreteInt::I32([30])
    );
    // NOTE: Ignore the sign bit, which is a separate (above) case.
    for idx in 0..(32 - 1) {
        assert_eq!(
            first_leading_bit(ConcreteInt::I32([1 << idx])),
            ConcreteInt::I32([idx])
        );
    }
    for idx in 1..(32 - 1) {
        assert_eq!(
            first_leading_bit(ConcreteInt::I32([-(1 << idx)])),
            ConcreteInt::I32([idx - 1])
        );
    }

    assert_eq!(
        first_leading_bit(ConcreteInt::U32([0])),
        ConcreteInt::U32([u32::MAX])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::U32([1])),
        ConcreteInt::U32([0])
    );
    assert_eq!(
        first_leading_bit(ConcreteInt::U32([u32::MAX])),
        ConcreteInt::U32([31])
    );
    for idx in 0..32 {
        assert_eq!(
            first_leading_bit(ConcreteInt::U32([1 << idx])),
            ConcreteInt::U32([idx])
        )
    }
}

/// Trait for conversions of abstract values to concrete types.
trait TryFromAbstract<T>: Sized {
    /// Convert an abstract literal `value` to `Self`.
    ///
    /// Since Naga's [`AbstractInt`] and [`AbstractFloat`] exist to support
    /// WGSL, we follow WGSL's conversion rules here:
    ///
    /// - WGSL §6.1.2. Conversion Rank says that automatic conversions
    ///   from [`AbstractInt`] to an integer type are either lossless or an
    ///   error.
    ///
    /// - WGSL §15.7.6 Floating Point Conversion says that conversions
    ///   to floating point in constant expressions and override
    ///   expressions are errors if the value is out of range for the
    ///   destination type, but rounding is okay.
    ///
    /// - WGSL §17.1.2 i32()/u32() constructors treat AbstractFloat as any
    ///   other floating point type, following the scalar floating point to
    ///   integral conversion algorithm (§15.7.6). There is no automatic
    ///   conversion from AbstractFloat to integer types.
    ///
    /// [`AbstractInt`]: crate::Literal::AbstractInt
    /// [`AbstractFloat`]: crate::Literal::AbstractFloat
    fn try_from_abstract(value: T) -> Result<Self, ConstantEvaluatorError>;
}

impl TryFromAbstract<i64> for i32 {
    fn try_from_abstract(value: i64) -> Result<i32, ConstantEvaluatorError> {
        i32::try_from(value).map_err(|_| ConstantEvaluatorError::AutomaticConversionLossy {
            value: format!("{value:?}"),
            to_type: "i32",
        })
    }
}

impl TryFromAbstract<i64> for u32 {
    fn try_from_abstract(value: i64) -> Result<u32, ConstantEvaluatorError> {
        u32::try_from(value).map_err(|_| ConstantEvaluatorError::AutomaticConversionLossy {
            value: format!("{value:?}"),
            to_type: "u32",
        })
    }
}

impl TryFromAbstract<i64> for u64 {
    fn try_from_abstract(value: i64) -> Result<u64, ConstantEvaluatorError> {
        u64::try_from(value).map_err(|_| ConstantEvaluatorError::AutomaticConversionLossy {
            value: format!("{value:?}"),
            to_type: "u64",
        })
    }
}

impl TryFromAbstract<i64> for i64 {
    fn try_from_abstract(value: i64) -> Result<i64, ConstantEvaluatorError> {
        Ok(value)
    }
}

impl TryFromAbstract<i64> for f32 {
    fn try_from_abstract(value: i64) -> Result<Self, ConstantEvaluatorError> {
        let f = value as f32;
        // The range of `i64` is roughly ±18 × 10¹⁸, whereas the range of
        // `f32` is roughly ±3.4 × 10³⁸, so there's no opportunity for
        // overflow here.
        Ok(f)
    }
}

impl TryFromAbstract<f64> for f32 {
    fn try_from_abstract(value: f64) -> Result<f32, ConstantEvaluatorError> {
        let f = value as f32;
        if f.is_infinite() {
            return Err(ConstantEvaluatorError::AutomaticConversionLossy {
                value: format!("{value:?}"),
                to_type: "f32",
            });
        }
        Ok(f)
    }
}

impl TryFromAbstract<i64> for f64 {
    fn try_from_abstract(value: i64) -> Result<Self, ConstantEvaluatorError> {
        let f = value as f64;
        // The range of `i64` is roughly ±18 × 10¹⁸, whereas the range of
        // `f64` is roughly ±1.8 × 10³⁰⁸, so there's no opportunity for
        // overflow here.
        Ok(f)
    }
}

impl TryFromAbstract<f64> for f64 {
    fn try_from_abstract(value: f64) -> Result<f64, ConstantEvaluatorError> {
        Ok(value)
    }
}

impl TryFromAbstract<f64> for i32 {
    fn try_from_abstract(value: f64) -> Result<Self, ConstantEvaluatorError> {
        // https://www.w3.org/TR/WGSL/#floating-point-conversion
        // To convert a floating point scalar value X to an integer scalar type T:
        // * If X is a NaN, the result is an indeterminate value in T.
        // * If X is exactly representable in the target type T, then the
        //   result is that value.
        // * Otherwise, the result is the value in T closest to truncate(X) and
        //   also exactly representable in the original floating point type.
        //
        // A rust cast satisfies these requirements apart from "the result
        // is... exactly representable in the original floating point type".
        // However, i32::MIN and i32::MAX are exactly representable by f64, so
        // we're all good.
        Ok(value as i32)
    }
}

impl TryFromAbstract<f64> for u32 {
    fn try_from_abstract(value: f64) -> Result<Self, ConstantEvaluatorError> {
        // As above, u32::MIN and u32::MAX are exactly representable by f64,
        // so a simple rust cast is sufficient.
        Ok(value as u32)
    }
}

impl TryFromAbstract<f64> for i64 {
    fn try_from_abstract(value: f64) -> Result<Self, ConstantEvaluatorError> {
        // As above, except we clamp to the minimum and maximum values
        // representable by both f64 and i64.
        use crate::proc::type_methods::IntFloatLimits;
        Ok(value.clamp(i64::min_float(), i64::max_float()) as i64)
    }
}

impl TryFromAbstract<f64> for u64 {
    fn try_from_abstract(value: f64) -> Result<Self, ConstantEvaluatorError> {
        // As above, this time clamping to the minimum and maximum values
        // representable by both f64 and u64.
        use crate::proc::type_methods::IntFloatLimits;
        Ok(value.clamp(u64::min_float(), u64::max_float()) as u64)
    }
}

impl TryFromAbstract<f64> for f16 {
    fn try_from_abstract(value: f64) -> Result<f16, ConstantEvaluatorError> {
        let f = f16::from_f64(value);
        if f.is_infinite() {
            return Err(ConstantEvaluatorError::AutomaticConversionLossy {
                value: format!("{value:?}"),
                to_type: "f16",
            });
        }
        Ok(f)
    }
}

impl TryFromAbstract<i64> for f16 {
    fn try_from_abstract(value: i64) -> Result<f16, ConstantEvaluatorError> {
        let f = f16::from_i64(value);
        if f.is_none() {
            return Err(ConstantEvaluatorError::AutomaticConversionLossy {
                value: format!("{value:?}"),
                to_type: "f16",
            });
        }
        Ok(f.unwrap())
    }
}

fn cross_product<T>(a: [T; 3], b: [T; 3]) -> [T; 3]
where
    T: Copy,
    T: core::ops::Mul<T, Output = T>,
    T: core::ops::Sub<T, Output = T>,
{
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

#[cfg(test)]
mod tests {
    use alloc::{vec, vec::Vec};

    use crate::{
        Arena, BinaryOperator, Constant, Expression, FastHashMap, Handle, Literal, ScalarKind,
        Type, TypeInner, UnaryOperator, UniqueArena, VectorSize,
    };

    use super::{Behavior, ConstantEvaluator, ExpressionKindTracker, WgslRestrictions};

    #[test]
    fn unary_op() {
        let mut types = UniqueArena::new();
        let mut constants = Arena::new();
        let overrides = Arena::new();
        let mut global_expressions = Arena::new();

        let scalar_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(crate::Scalar::I32),
            },
            Default::default(),
        );

        let vec_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: VectorSize::Bi,
                    scalar: crate::Scalar::I32,
                },
            },
            Default::default(),
        );

        let h = constants.append(
            Constant {
                name: None,
                ty: scalar_ty,
                init: global_expressions
                    .append(Expression::Literal(Literal::I32(4)), Default::default()),
            },
            Default::default(),
        );

        let h1 = constants.append(
            Constant {
                name: None,
                ty: scalar_ty,
                init: global_expressions
                    .append(Expression::Literal(Literal::I32(8)), Default::default()),
            },
            Default::default(),
        );

        let vec_h = constants.append(
            Constant {
                name: None,
                ty: vec_ty,
                init: global_expressions.append(
                    Expression::Compose {
                        ty: vec_ty,
                        components: vec![constants[h].init, constants[h1].init],
                    },
                    Default::default(),
                ),
            },
            Default::default(),
        );

        let expr = global_expressions.append(Expression::Constant(h), Default::default());
        let expr1 = global_expressions.append(Expression::Constant(vec_h), Default::default());

        let expr2 = Expression::Unary {
            op: UnaryOperator::Negate,
            expr,
        };

        let expr3 = Expression::Unary {
            op: UnaryOperator::BitwiseNot,
            expr,
        };

        let expr4 = Expression::Unary {
            op: UnaryOperator::BitwiseNot,
            expr: expr1,
        };

        let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&global_expressions);
        let mut solver = ConstantEvaluator {
            behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
            types: &mut types,
            constants: &constants,
            overrides: &overrides,
            expressions: &mut global_expressions,
            expression_kind_tracker,
            layouter: &mut crate::proc::Layouter::default(),
        };

        let res1 = solver
            .try_eval_and_append(expr2, Default::default())
            .unwrap();
        let res2 = solver
            .try_eval_and_append(expr3, Default::default())
            .unwrap();
        let res3 = solver
            .try_eval_and_append(expr4, Default::default())
            .unwrap();

        assert_eq!(
            global_expressions[res1],
            Expression::Literal(Literal::I32(-4))
        );

        assert_eq!(
            global_expressions[res2],
            Expression::Literal(Literal::I32(!4))
        );

        let res3_inner = &global_expressions[res3];

        match *res3_inner {
            Expression::Compose {
                ref ty,
                ref components,
            } => {
                assert_eq!(*ty, vec_ty);
                let mut components_iter = components.iter().copied();
                assert_eq!(
                    global_expressions[components_iter.next().unwrap()],
                    Expression::Literal(Literal::I32(!4))
                );
                assert_eq!(
                    global_expressions[components_iter.next().unwrap()],
                    Expression::Literal(Literal::I32(!8))
                );
                assert!(components_iter.next().is_none());
            }
            _ => panic!("Expected vector"),
        }
    }

    #[test]
    fn matrix_op() {
        let mut helper = MatrixTestHelper::new();

        for nc in 2..=4 {
            for nr in 2..=4 {
                // Validates multiplication on vector-matrix.
                // vecR(0, 1, .., r) * matCxR(0, 1, .., nc * nr)
                let evaluated = helper.eval_vector_multiply_matrix(nc, nr);
                let expected = (0..nc)
                    .map(|c| (0..nr).map(|r| (r * (c * nr + r)) as f32).sum())
                    .collect::<Vec<f32>>();
                assert_eq!(evaluated, expected);

                // Validates multiplication on matrix-vector.
                // matCxR(0, 1, .., nc * nr) * vecC(0, 1, .., nc)
                let evaluated = helper.eval_matrix_multiply_vector(nc, nr);
                let expected = (0..nr)
                    .map(|r| (0..nc).map(|c| (c * (c * nr + r)) as f32).sum())
                    .collect::<Vec<f32>>();
                assert_eq!(evaluated, expected);

                for k in 2..=4 {
                    // Validates multiplication on matrix-matrix.
                    // matKxR(0, 1, .., k * nr) * matCxK(0, 1, .., nc * k)
                    let evaluated = helper.eval_matrix_multiply_matrix(nr, nc, k);
                    let expected = (0..nc)
                        .flat_map(|c| {
                            (0..nr).map(move |r| {
                                (0..k).map(|v| ((v * nr + r) * (c * k + v)) as f32).sum()
                            })
                        })
                        .collect::<Vec<f32>>();
                    assert_eq!(evaluated, expected);
                }
            }
        }
    }

    /// Test fixture providing pre-built f32 vector and matrix constant
    /// expressions with sequential element values, used to evaluate and verify
    /// matrix operations.
    struct MatrixTestHelper {
        types: UniqueArena<Type>,
        expressions: Arena<Expression>,
        /// Vector expressions from [0, 1] to [0, 1, 2, 3].
        vec_exprs: FastHashMap<usize, Handle<Expression>>,
        /// Matrix expressions from [0, .., 3] to [0, .., 15].
        mat_exprs: FastHashMap<(usize, usize), Handle<Expression>>,
    }

    impl MatrixTestHelper {
        fn new() -> Self {
            let mut types = UniqueArena::new();
            let mut expressions = Arena::new();
            let span = crate::Span::default();

            let (mut vec_tys, mut mat_tys) = (FastHashMap::default(), FastHashMap::default());
            for c in 2..=4 {
                let vec_ty = types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Vector {
                            size: Self::int_to_vector_size(c),
                            scalar: crate::Scalar::F32,
                        },
                    },
                    span,
                );
                vec_tys.insert(c, vec_ty);
                for r in 2..=4 {
                    let mat_ty = types.insert(
                        Type {
                            name: None,
                            inner: TypeInner::Matrix {
                                columns: Self::int_to_vector_size(c),
                                rows: Self::int_to_vector_size(r),
                                scalar: crate::Scalar::F32,
                            },
                        },
                        span,
                    );
                    mat_tys.insert((c, r), mat_ty);
                }
            }

            let mut lit_exprs = FastHashMap::default();
            for i in 0..16 {
                let expr = expressions.append(Expression::Literal(Literal::F32(i as f32)), span);
                lit_exprs.insert(i, expr);
            }

            let mut vec_exprs = FastHashMap::default();
            for c in 2..=4 {
                let expr = expressions.append(
                    Expression::Compose {
                        ty: *vec_tys.get(&c).unwrap(),
                        components: (0..c)
                            .map(|i| *lit_exprs.get(&i).unwrap())
                            .collect::<Vec<_>>(),
                    },
                    span,
                );
                vec_exprs.insert(c, expr);
            }

            let mut mat_exprs = FastHashMap::default();
            for c in 2..=4 {
                for r in 2..=4 {
                    let mut columns = Vec::with_capacity(c);
                    for cc in 0..c {
                        let start = cc * r;
                        let expr = expressions.append(
                            Expression::Compose {
                                ty: *vec_tys.get(&r).unwrap(),
                                components: (start..start + r)
                                    .map(|i| *lit_exprs.get(&i).unwrap())
                                    .collect::<Vec<_>>(),
                            },
                            span,
                        );
                        columns.push(expr);
                    }

                    let expr = expressions.append(
                        Expression::Compose {
                            ty: *mat_tys.get(&(c, r)).unwrap(),
                            components: columns,
                        },
                        span,
                    );
                    mat_exprs.insert((c, r), expr);
                }
            }

            Self {
                types,
                expressions,
                vec_exprs,
                mat_exprs,
            }
        }

        /// Evaluates vec[0..nr] * mat[0..nc*nr] and returns the result as f32s.
        fn eval_vector_multiply_matrix(&mut self, nc: usize, nr: usize) -> Vec<f32> {
            let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&self.expressions);
            let mut solver = ConstantEvaluator {
                behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
                types: &mut self.types,
                constants: &Arena::new(),
                overrides: &Arena::new(),
                expressions: &mut self.expressions,
                expression_kind_tracker,
                layouter: &mut crate::proc::Layouter::default(),
            };

            let result = solver
                .try_eval_and_append(
                    Expression::Binary {
                        op: BinaryOperator::Multiply,
                        left: *self.vec_exprs.get(&nr).unwrap(),
                        right: *self.mat_exprs.get(&(nc, nr)).unwrap(),
                    },
                    Default::default(),
                )
                .unwrap();
            self.flatten(result)
        }

        /// Evaluates mat[0..nc*nr] * vec[0..nc] and returns the result as f32s.
        fn eval_matrix_multiply_vector(&mut self, nc: usize, nr: usize) -> Vec<f32> {
            let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&self.expressions);
            let mut solver = ConstantEvaluator {
                behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
                types: &mut self.types,
                constants: &Arena::new(),
                overrides: &Arena::new(),
                expressions: &mut self.expressions,
                expression_kind_tracker,
                layouter: &mut crate::proc::Layouter::default(),
            };

            let result = solver
                .try_eval_and_append(
                    Expression::Binary {
                        op: BinaryOperator::Multiply,
                        left: *self.mat_exprs.get(&(nc, nr)).unwrap(),
                        right: *self.vec_exprs.get(&nc).unwrap(),
                    },
                    Default::default(),
                )
                .unwrap();
            self.flatten(result)
        }

        /// Evaluates mat[0..k*l_nr] * mat[0..r_nc*k] and returns the result as
        /// f32s.
        fn eval_matrix_multiply_matrix(&mut self, l_nr: usize, r_nc: usize, k: usize) -> Vec<f32> {
            let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&self.expressions);
            let mut solver = ConstantEvaluator {
                behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
                types: &mut self.types,
                constants: &Arena::new(),
                overrides: &Arena::new(),
                expressions: &mut self.expressions,
                expression_kind_tracker,
                layouter: &mut crate::proc::Layouter::default(),
            };

            let result = solver
                .try_eval_and_append(
                    Expression::Binary {
                        op: BinaryOperator::Multiply,
                        left: *self.mat_exprs.get(&(k, l_nr)).unwrap(),
                        right: *self.mat_exprs.get(&(r_nc, k)).unwrap(),
                    },
                    Default::default(),
                )
                .unwrap();
            self.flatten(result)
        }

        fn flatten(&self, expr: Handle<Expression>) -> Vec<f32> {
            let Expression::Compose {
                ref components,
                ref ty,
            } = self.expressions[expr]
            else {
                unreachable!()
            };

            match self.types[*ty].inner {
                TypeInner::Vector { .. } => components
                    .iter()
                    .map(|&comp| {
                        let Expression::Literal(Literal::F32(v)) = self.expressions[comp] else {
                            unreachable!()
                        };
                        v
                    })
                    .collect(),
                TypeInner::Matrix { .. } => components
                    .iter()
                    .flat_map(|&comp| self.flatten(comp))
                    .collect(),
                _ => unreachable!(),
            }
        }

        fn int_to_vector_size(int: usize) -> VectorSize {
            match int {
                2 => VectorSize::Bi,
                3 => VectorSize::Tri,
                4 => VectorSize::Quad,
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn cast() {
        let mut types = UniqueArena::new();
        let mut constants = Arena::new();
        let overrides = Arena::new();
        let mut global_expressions = Arena::new();

        let scalar_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(crate::Scalar::I32),
            },
            Default::default(),
        );

        let h = constants.append(
            Constant {
                name: None,
                ty: scalar_ty,
                init: global_expressions
                    .append(Expression::Literal(Literal::I32(4)), Default::default()),
            },
            Default::default(),
        );

        let expr = global_expressions.append(Expression::Constant(h), Default::default());

        let root = Expression::As {
            expr,
            kind: ScalarKind::Bool,
            convert: Some(crate::BOOL_WIDTH),
        };

        let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&global_expressions);
        let mut solver = ConstantEvaluator {
            behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
            types: &mut types,
            constants: &constants,
            overrides: &overrides,
            expressions: &mut global_expressions,
            expression_kind_tracker,
            layouter: &mut crate::proc::Layouter::default(),
        };

        let res = solver
            .try_eval_and_append(root, Default::default())
            .unwrap();

        assert_eq!(
            global_expressions[res],
            Expression::Literal(Literal::Bool(true))
        );
    }

    #[test]
    fn access() {
        let mut types = UniqueArena::new();
        let mut constants = Arena::new();
        let overrides = Arena::new();
        let mut global_expressions = Arena::new();

        let matrix_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Matrix {
                    columns: VectorSize::Bi,
                    rows: VectorSize::Tri,
                    scalar: crate::Scalar::F32,
                },
            },
            Default::default(),
        );

        let vec_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: VectorSize::Tri,
                    scalar: crate::Scalar::F32,
                },
            },
            Default::default(),
        );

        let mut vec1_components = Vec::with_capacity(3);
        let mut vec2_components = Vec::with_capacity(3);

        for i in 0..3 {
            let h = global_expressions.append(
                Expression::Literal(Literal::F32(i as f32)),
                Default::default(),
            );

            vec1_components.push(h)
        }

        for i in 3..6 {
            let h = global_expressions.append(
                Expression::Literal(Literal::F32(i as f32)),
                Default::default(),
            );

            vec2_components.push(h)
        }

        let vec1 = constants.append(
            Constant {
                name: None,
                ty: vec_ty,
                init: global_expressions.append(
                    Expression::Compose {
                        ty: vec_ty,
                        components: vec1_components,
                    },
                    Default::default(),
                ),
            },
            Default::default(),
        );

        let vec2 = constants.append(
            Constant {
                name: None,
                ty: vec_ty,
                init: global_expressions.append(
                    Expression::Compose {
                        ty: vec_ty,
                        components: vec2_components,
                    },
                    Default::default(),
                ),
            },
            Default::default(),
        );

        let h = constants.append(
            Constant {
                name: None,
                ty: matrix_ty,
                init: global_expressions.append(
                    Expression::Compose {
                        ty: matrix_ty,
                        components: vec![constants[vec1].init, constants[vec2].init],
                    },
                    Default::default(),
                ),
            },
            Default::default(),
        );

        let base = global_expressions.append(Expression::Constant(h), Default::default());

        let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&global_expressions);
        let mut solver = ConstantEvaluator {
            behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
            types: &mut types,
            constants: &constants,
            overrides: &overrides,
            expressions: &mut global_expressions,
            expression_kind_tracker,
            layouter: &mut crate::proc::Layouter::default(),
        };

        let root1 = Expression::AccessIndex { base, index: 1 };

        let res1 = solver
            .try_eval_and_append(root1, Default::default())
            .unwrap();

        let root2 = Expression::AccessIndex {
            base: res1,
            index: 2,
        };

        let res2 = solver
            .try_eval_and_append(root2, Default::default())
            .unwrap();

        match global_expressions[res1] {
            Expression::Compose {
                ref ty,
                ref components,
            } => {
                assert_eq!(*ty, vec_ty);
                let mut components_iter = components.iter().copied();
                assert_eq!(
                    global_expressions[components_iter.next().unwrap()],
                    Expression::Literal(Literal::F32(3.))
                );
                assert_eq!(
                    global_expressions[components_iter.next().unwrap()],
                    Expression::Literal(Literal::F32(4.))
                );
                assert_eq!(
                    global_expressions[components_iter.next().unwrap()],
                    Expression::Literal(Literal::F32(5.))
                );
                assert!(components_iter.next().is_none());
            }
            _ => panic!("Expected vector"),
        }

        assert_eq!(
            global_expressions[res2],
            Expression::Literal(Literal::F32(5.))
        );
    }

    #[test]
    fn compose_of_constants() {
        let mut types = UniqueArena::new();
        let mut constants = Arena::new();
        let overrides = Arena::new();
        let mut global_expressions = Arena::new();

        let i32_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(crate::Scalar::I32),
            },
            Default::default(),
        );

        let vec2_i32_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: VectorSize::Bi,
                    scalar: crate::Scalar::I32,
                },
            },
            Default::default(),
        );

        let h = constants.append(
            Constant {
                name: None,
                ty: i32_ty,
                init: global_expressions
                    .append(Expression::Literal(Literal::I32(4)), Default::default()),
            },
            Default::default(),
        );

        let h_expr = global_expressions.append(Expression::Constant(h), Default::default());

        let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&global_expressions);
        let mut solver = ConstantEvaluator {
            behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
            types: &mut types,
            constants: &constants,
            overrides: &overrides,
            expressions: &mut global_expressions,
            expression_kind_tracker,
            layouter: &mut crate::proc::Layouter::default(),
        };

        let solved_compose = solver
            .try_eval_and_append(
                Expression::Compose {
                    ty: vec2_i32_ty,
                    components: vec![h_expr, h_expr],
                },
                Default::default(),
            )
            .unwrap();
        let solved_negate = solver
            .try_eval_and_append(
                Expression::Unary {
                    op: UnaryOperator::Negate,
                    expr: solved_compose,
                },
                Default::default(),
            )
            .unwrap();

        let pass = match global_expressions[solved_negate] {
            Expression::Compose { ty, ref components } => {
                ty == vec2_i32_ty
                    && components.iter().all(|&component| {
                        let component = &global_expressions[component];
                        matches!(*component, Expression::Literal(Literal::I32(-4)))
                    })
            }
            _ => false,
        };
        if !pass {
            panic!("unexpected evaluation result")
        }
    }

    #[test]
    fn splat_of_constant() {
        let mut types = UniqueArena::new();
        let mut constants = Arena::new();
        let overrides = Arena::new();
        let mut global_expressions = Arena::new();

        let i32_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(crate::Scalar::I32),
            },
            Default::default(),
        );

        let vec2_i32_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: VectorSize::Bi,
                    scalar: crate::Scalar::I32,
                },
            },
            Default::default(),
        );

        let h = constants.append(
            Constant {
                name: None,
                ty: i32_ty,
                init: global_expressions
                    .append(Expression::Literal(Literal::I32(4)), Default::default()),
            },
            Default::default(),
        );

        let h_expr = global_expressions.append(Expression::Constant(h), Default::default());

        let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&global_expressions);
        let mut solver = ConstantEvaluator {
            behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
            types: &mut types,
            constants: &constants,
            overrides: &overrides,
            expressions: &mut global_expressions,
            expression_kind_tracker,
            layouter: &mut crate::proc::Layouter::default(),
        };

        let solved_compose = solver
            .try_eval_and_append(
                Expression::Splat {
                    size: VectorSize::Bi,
                    value: h_expr,
                },
                Default::default(),
            )
            .unwrap();
        let solved_negate = solver
            .try_eval_and_append(
                Expression::Unary {
                    op: UnaryOperator::Negate,
                    expr: solved_compose,
                },
                Default::default(),
            )
            .unwrap();

        let pass = match global_expressions[solved_negate] {
            Expression::Compose { ty, ref components } => {
                ty == vec2_i32_ty
                    && components.iter().all(|&component| {
                        let component = &global_expressions[component];
                        matches!(*component, Expression::Literal(Literal::I32(-4)))
                    })
            }
            _ => false,
        };
        if !pass {
            panic!("unexpected evaluation result")
        }
    }

    #[test]
    fn splat_of_zero_value() {
        let mut types = UniqueArena::new();
        let constants = Arena::new();
        let overrides = Arena::new();
        let mut global_expressions = Arena::new();

        let f32_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(crate::Scalar::F32),
            },
            Default::default(),
        );

        let vec2_f32_ty = types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector {
                    size: VectorSize::Bi,
                    scalar: crate::Scalar::F32,
                },
            },
            Default::default(),
        );

        let five =
            global_expressions.append(Expression::Literal(Literal::F32(5.0)), Default::default());
        let five_splat = global_expressions.append(
            Expression::Splat {
                size: VectorSize::Bi,
                value: five,
            },
            Default::default(),
        );
        let zero = global_expressions.append(Expression::ZeroValue(f32_ty), Default::default());
        let zero_splat = global_expressions.append(
            Expression::Splat {
                size: VectorSize::Bi,
                value: zero,
            },
            Default::default(),
        );

        let expression_kind_tracker = &mut ExpressionKindTracker::from_arena(&global_expressions);
        let mut solver = ConstantEvaluator {
            behavior: Behavior::Wgsl(WgslRestrictions::Const(None)),
            types: &mut types,
            constants: &constants,
            overrides: &overrides,
            expressions: &mut global_expressions,
            expression_kind_tracker,
            layouter: &mut crate::proc::Layouter::default(),
        };

        let solved_add = solver
            .try_eval_and_append(
                Expression::Binary {
                    op: BinaryOperator::Add,
                    left: zero_splat,
                    right: five_splat,
                },
                Default::default(),
            )
            .unwrap();

        let pass = match global_expressions[solved_add] {
            Expression::Compose { ty, ref components } => {
                ty == vec2_f32_ty
                    && components.iter().all(|&component| {
                        let component = &global_expressions[component];
                        matches!(*component, Expression::Literal(Literal::F32(5.0)))
                    })
            }
            _ => false,
        };
        if !pass {
            panic!("unexpected evaluation result")
        }
    }
}
