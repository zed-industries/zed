//! Methods on or related to [`TypeInner`], [`Scalar`], [`ScalarKind`], and [`VectorSize`].
//!
//! [`TypeInner`]: crate::TypeInner
//! [`Scalar`]: crate::Scalar
//! [`ScalarKind`]: crate::ScalarKind
//! [`VectorSize`]: crate::VectorSize

use crate::{ir, valid::MAX_TYPE_SIZE};

use super::TypeResolution;

impl crate::ScalarKind {
    pub const fn is_numeric(self) -> bool {
        match self {
            crate::ScalarKind::Sint
            | crate::ScalarKind::Uint
            | crate::ScalarKind::Float
            | crate::ScalarKind::AbstractInt
            | crate::ScalarKind::AbstractFloat => true,
            crate::ScalarKind::Bool => false,
        }
    }
}

impl crate::Scalar {
    pub const I32: Self = Self {
        kind: crate::ScalarKind::Sint,
        width: 4,
    };
    pub const U32: Self = Self {
        kind: crate::ScalarKind::Uint,
        width: 4,
    };
    pub const F16: Self = Self {
        kind: crate::ScalarKind::Float,
        width: 2,
    };
    pub const F32: Self = Self {
        kind: crate::ScalarKind::Float,
        width: 4,
    };
    pub const F64: Self = Self {
        kind: crate::ScalarKind::Float,
        width: 8,
    };
    pub const I64: Self = Self {
        kind: crate::ScalarKind::Sint,
        width: 8,
    };
    pub const U64: Self = Self {
        kind: crate::ScalarKind::Uint,
        width: 8,
    };
    pub const BOOL: Self = Self {
        kind: crate::ScalarKind::Bool,
        width: crate::BOOL_WIDTH,
    };
    pub const ABSTRACT_INT: Self = Self {
        kind: crate::ScalarKind::AbstractInt,
        width: crate::ABSTRACT_WIDTH,
    };
    pub const ABSTRACT_FLOAT: Self = Self {
        kind: crate::ScalarKind::AbstractFloat,
        width: crate::ABSTRACT_WIDTH,
    };

    pub const fn is_abstract(self) -> bool {
        match self.kind {
            crate::ScalarKind::AbstractInt | crate::ScalarKind::AbstractFloat => true,
            crate::ScalarKind::Sint
            | crate::ScalarKind::Uint
            | crate::ScalarKind::Float
            | crate::ScalarKind::Bool => false,
        }
    }

    /// Construct a float `Scalar` with the given width.
    ///
    /// This is especially common when dealing with
    /// `TypeInner::Matrix`, where the scalar kind is implicit.
    pub const fn float(width: crate::Bytes) -> Self {
        Self {
            kind: crate::ScalarKind::Float,
            width,
        }
    }

    pub const fn to_inner_scalar(self) -> crate::TypeInner {
        crate::TypeInner::Scalar(self)
    }

    pub const fn to_inner_vector(self, size: crate::VectorSize) -> crate::TypeInner {
        crate::TypeInner::Vector { size, scalar: self }
    }

    pub const fn to_inner_atomic(self) -> crate::TypeInner {
        crate::TypeInner::Atomic(self)
    }
}

/// Produce all concrete integer [`ir::Scalar`]s.
///
/// Note that `I32` and `U32` must come first; this represents conversion rank
/// in overload resolution.
pub fn concrete_int_scalars() -> impl Iterator<Item = ir::Scalar> {
    [
        ir::Scalar::I32,
        ir::Scalar::U32,
        ir::Scalar::I64,
        ir::Scalar::U64,
    ]
    .into_iter()
}

/// Produce all vector sizes.
pub fn vector_sizes() -> impl Iterator<Item = ir::VectorSize> + Clone {
    static SIZES: [ir::VectorSize; 3] = [
        ir::VectorSize::Bi,
        ir::VectorSize::Tri,
        ir::VectorSize::Quad,
    ];

    SIZES.iter().cloned()
}

const POINTER_SPAN: u32 = 4;

impl crate::TypeInner {
    /// Return the scalar type of `self`.
    ///
    /// If `inner` is a scalar, vector, or matrix type, return
    /// its scalar type. Otherwise, return `None`.
    ///
    /// Note that this doesn't inspect [`Array`] types, as required
    /// for automatic conversions. For that, see [`scalar_for_conversions`].
    ///
    /// [`Array`]: crate::TypeInner::Array
    /// [`scalar_for_conversions`]: crate::TypeInner::scalar_for_conversions
    pub const fn scalar(&self) -> Option<crate::Scalar> {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Scalar(scalar) | Ti::Vector { scalar, .. } => Some(scalar),
            Ti::Matrix { scalar, .. } => Some(scalar),
            Ti::CooperativeMatrix { scalar, .. } => Some(scalar),
            _ => None,
        }
    }

    pub fn scalar_kind(&self) -> Option<crate::ScalarKind> {
        self.scalar().map(|scalar| scalar.kind)
    }

    /// Returns the scalar width in bytes
    pub fn scalar_width(&self) -> Option<u8> {
        self.scalar().map(|scalar| scalar.width)
    }

    /// Return the leaf scalar type of `self`, as needed for automatic conversions.
    ///
    /// Unlike the [`scalar`] method, which only retrieves scalars for
    /// [`Scalar`], [`Vector`], and [`Matrix`] this also looks into
    /// [`Array`] types to find the leaf scalar.
    ///
    /// [`scalar`]: crate::TypeInner::scalar
    /// [`Scalar`]: crate::TypeInner::Scalar
    /// [`Vector`]: crate::TypeInner::Vector
    /// [`Matrix`]: crate::TypeInner::Matrix
    /// [`Array`]: crate::TypeInner::Array
    pub fn scalar_for_conversions(
        &self,
        types: &crate::UniqueArena<crate::Type>,
    ) -> Option<crate::Scalar> {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Scalar(scalar) | Ti::Vector { scalar, .. } | Ti::Matrix { scalar, .. } => {
                Some(scalar)
            }
            Ti::Array { base, .. } => types[base].inner.scalar_for_conversions(types),
            _ => None,
        }
    }

    pub const fn pointer_space(&self) -> Option<crate::AddressSpace> {
        match *self {
            Self::Pointer { space, .. } => Some(space),
            Self::ValuePointer { space, .. } => Some(space),
            _ => None,
        }
    }

    /// If `self` is a pointer type, return its base type.
    pub const fn pointer_base_type(&self) -> Option<TypeResolution> {
        match *self {
            crate::TypeInner::Pointer { base, .. } => Some(TypeResolution::Handle(base)),
            crate::TypeInner::ValuePointer {
                size: None, scalar, ..
            } => Some(TypeResolution::Value(crate::TypeInner::Scalar(scalar))),
            crate::TypeInner::ValuePointer {
                size: Some(size),
                scalar,
                ..
            } => Some(TypeResolution::Value(crate::TypeInner::Vector {
                size,
                scalar,
            })),
            _ => None,
        }
    }

    pub fn is_atomic_pointer(&self, types: &crate::UniqueArena<crate::Type>) -> bool {
        match *self {
            Self::Pointer { base, .. } => match types[base].inner {
                Self::Atomic { .. } => true,
                _ => false,
            },
            _ => false,
        }
    }

    /// Attempt to calculate the size of this type. Returns `None` if the size
    /// exceeds the limit of [`crate::valid::MAX_TYPE_SIZE`].
    pub fn try_size(&self, gctx: super::GlobalCtx) -> Option<u32> {
        match *self {
            Self::Scalar(scalar) | Self::Atomic(scalar) => Some(scalar.width as u32),
            Self::Vector { size, scalar } => Some(size as u32 * scalar.width as u32),
            // matrices are treated as arrays of aligned columns
            Self::Matrix {
                columns,
                rows,
                scalar,
            } => Some(super::Alignment::from(rows) * scalar.width as u32 * columns as u32),
            Self::CooperativeMatrix {
                columns,
                rows,
                scalar,
                role: _,
            } => Some(columns as u32 * rows as u32 * scalar.width as u32),
            Self::Pointer { .. } | Self::ValuePointer { .. } => Some(POINTER_SPAN),
            Self::Array {
                base: _,
                size,
                stride,
            } => {
                let count = match size.resolve(gctx) {
                    Ok(crate::proc::IndexableLength::Known(count)) => count,
                    // any struct member or array element needing a size at pipeline-creation time
                    // must have a creation-fixed footprint
                    Err(_) => 0,
                    // A dynamically-sized array has to have at least one element
                    Ok(crate::proc::IndexableLength::Dynamic) => 1,
                };
                if count > MAX_TYPE_SIZE {
                    // It shouldn't be possible to have an array of a zero-sized type, but
                    // let's check just in case.
                    None
                } else {
                    count
                        .checked_mul(stride)
                        .filter(|size| *size <= MAX_TYPE_SIZE)
                }
            }
            Self::Struct { span, .. } => Some(span),
            Self::Image { .. }
            | Self::Sampler { .. }
            | Self::AccelerationStructure { .. }
            | Self::RayQuery { .. }
            | Self::BindingArray { .. } => Some(0),
        }
    }

    /// Get the size of this type.
    ///
    /// Panics if the size exceeds the limit of [`crate::valid::MAX_TYPE_SIZE`].
    /// Validated modules should not contain such types. Code working with
    /// modules prior to validation should use [`Self::try_size`] and handle the
    /// error appropriately.
    pub fn size(&self, gctx: super::GlobalCtx) -> u32 {
        self.try_size(gctx).expect("type is too large")
    }

    /// Return the canonical form of `self`, or `None` if it's already in
    /// canonical form.
    ///
    /// Certain types have multiple representations in `TypeInner`. This
    /// function converts all forms of equivalent types to a single
    /// representative of their class, so that simply applying `Eq` to the
    /// result indicates whether the types are equivalent, as far as Naga IR is
    /// concerned.
    pub fn canonical_form(
        &self,
        types: &crate::UniqueArena<crate::Type>,
    ) -> Option<crate::TypeInner> {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Pointer { base, space } => match types[base].inner {
                Ti::Scalar(scalar) => Some(Ti::ValuePointer {
                    size: None,
                    scalar,
                    space,
                }),
                Ti::Vector { size, scalar } => Some(Ti::ValuePointer {
                    size: Some(size),
                    scalar,
                    space,
                }),
                _ => None,
            },
            _ => None,
        }
    }

    /// Compare value type `self` and `rhs` as types.
    ///
    /// This is mostly the same as `<TypeInner as Eq>::eq`, but it treats
    /// [`ValuePointer`] and [`Pointer`] types as equivalent. This method
    /// cannot be used for structs, because it cannot distinguish two
    /// structs with different names but the same members. For structs,
    /// use [`compare_types`].
    ///
    /// When you know that one side of the comparison is never a pointer or
    /// struct, it's fine to not bother with canonicalization, and just
    /// compare `TypeInner` values with `==`.
    ///
    /// # Panics
    ///
    /// If both `self` and `rhs` are structs.
    ///
    /// [`compare_types`]: crate::proc::compare_types
    /// [`ValuePointer`]: ir::TypeInner::ValuePointer
    /// [`Pointer`]: ir::TypeInner::Pointer
    pub fn non_struct_equivalent(
        &self,
        rhs: &ir::TypeInner,
        types: &crate::UniqueArena<crate::Type>,
    ) -> bool {
        let left = self.canonical_form(types);
        let right = rhs.canonical_form(types);

        let left_struct = matches!(*self, ir::TypeInner::Struct { .. });
        let right_struct = matches!(*rhs, ir::TypeInner::Struct { .. });

        assert!(!left_struct || !right_struct);

        left.as_ref().unwrap_or(self) == right.as_ref().unwrap_or(rhs)
    }

    /// Returns true if `self` is runtime- or override-sized.
    pub fn is_dynamically_sized(&self, types: &crate::UniqueArena<crate::Type>) -> bool {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Array {
                size: crate::ArraySize::Constant(_),
                ..
            } => false,
            Ti::Array {
                size: crate::ArraySize::Pending(_) | crate::ArraySize::Dynamic,
                ..
            } => true,
            Ti::Struct { ref members, .. } => members
                .last()
                .map(|last| types[last.ty].inner.is_dynamically_sized(types))
                .unwrap_or(false),
            _ => false,
        }
    }

    /// Returns true if `self` is a constructible type.
    pub fn is_constructible(&self, types: &crate::UniqueArena<crate::Type>) -> bool {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Array { base, size, .. } => {
                let fixed_size = match size {
                    ir::ArraySize::Constant(_) => true,
                    ir::ArraySize::Pending(_) | ir::ArraySize::Dynamic => false,
                };
                fixed_size && types[base].inner.is_constructible(types)
            }
            Ti::Struct { ref members, .. } => members
                .iter()
                .all(|member| types[member.ty].inner.is_constructible(types)),
            Ti::Atomic(_)
            | Ti::Pointer { .. }
            | Ti::ValuePointer { .. }
            | Ti::Image { .. }
            | Ti::Sampler { .. }
            | Ti::AccelerationStructure { .. }
            | Ti::BindingArray { .. } => false,
            Ti::Scalar(_)
            | Ti::Vector { .. }
            | Ti::Matrix { .. }
            | Ti::RayQuery { .. }
            | Ti::CooperativeMatrix { .. } => true,
        }
    }

    pub const fn components(&self) -> Option<u32> {
        Some(match *self {
            Self::Vector { size, .. } => size as u32,
            Self::Matrix { columns, .. } => columns as u32,
            Self::Array {
                size: crate::ArraySize::Constant(len),
                ..
            } => len.get(),
            Self::Struct { ref members, .. } => members.len() as u32,
            _ => return None,
        })
    }

    pub fn component_type(&self, index: usize) -> Option<TypeResolution> {
        Some(match *self {
            Self::Vector { scalar, .. } => TypeResolution::Value(crate::TypeInner::Scalar(scalar)),
            Self::Matrix { rows, scalar, .. } => {
                TypeResolution::Value(crate::TypeInner::Vector { size: rows, scalar })
            }
            Self::Array {
                base,
                size: crate::ArraySize::Constant(_),
                ..
            } => TypeResolution::Handle(base),
            Self::Struct { ref members, .. } => TypeResolution::Handle(members[index].ty),
            _ => return None,
        })
    }

    /// If the type is a Vector or a Scalar return a tuple of the vector size (or None
    /// for Scalars), and the scalar kind. Returns (None, None) for other types.
    pub const fn vector_size_and_scalar(
        &self,
    ) -> Option<(Option<crate::VectorSize>, crate::Scalar)> {
        match *self {
            crate::TypeInner::Scalar(scalar) => Some((None, scalar)),
            crate::TypeInner::Vector { size, scalar } => Some((Some(size), scalar)),
            crate::TypeInner::Matrix { .. }
            | crate::TypeInner::CooperativeMatrix { .. }
            | crate::TypeInner::Atomic(_)
            | crate::TypeInner::Pointer { .. }
            | crate::TypeInner::ValuePointer { .. }
            | crate::TypeInner::Array { .. }
            | crate::TypeInner::Struct { .. }
            | crate::TypeInner::Image { .. }
            | crate::TypeInner::Sampler { .. }
            | crate::TypeInner::AccelerationStructure { .. }
            | crate::TypeInner::RayQuery { .. }
            | crate::TypeInner::BindingArray { .. } => None,
        }
    }

    /// Return true if `self` is an abstract type.
    ///
    /// Use `types` to look up type handles. This is necessary to
    /// recognize abstract arrays.
    pub fn is_abstract(&self, types: &crate::UniqueArena<crate::Type>) -> bool {
        match *self {
            crate::TypeInner::Scalar(scalar)
            | crate::TypeInner::Vector { scalar, .. }
            | crate::TypeInner::Matrix { scalar, .. }
            | crate::TypeInner::Atomic(scalar) => scalar.is_abstract(),
            crate::TypeInner::Array { base, .. } => types[base].inner.is_abstract(types),
            crate::TypeInner::CooperativeMatrix { .. }
            | crate::TypeInner::ValuePointer { .. }
            | crate::TypeInner::Pointer { .. }
            | crate::TypeInner::Struct { .. }
            | crate::TypeInner::Image { .. }
            | crate::TypeInner::Sampler { .. }
            | crate::TypeInner::AccelerationStructure { .. }
            | crate::TypeInner::RayQuery { .. }
            | crate::TypeInner::BindingArray { .. } => false,
        }
    }

    /// Determine whether `self` automatically converts to `goal`.
    ///
    /// If Naga IR's automatic conversions will convert `self` to
    /// `goal`, then return a pair `(from, to)`, where `from` and `to`
    /// are the scalar types of the leaf values of `self` and `goal`.
    ///
    /// If `self` and `goal` are the same type, this will simply return
    /// a pair `(S, S)`.
    ///
    /// If the automatic conversions cannot convert `self` to `goal`,
    /// return `None`.
    ///
    /// Naga IR's automatic conversions will convert:
    ///
    /// - [`AbstractInt`] scalars to [`AbstractFloat`] or any numeric scalar type
    ///
    /// - [`AbstractFloat`] scalars to any floating-point scalar type
    ///
    /// - A [`Vector`] `{ size, scalar: S }` to `{ size, scalar: T }`
    ///   if they would convert `S` to `T`
    ///
    /// - An [`Array`] `{ base: S, size, stride }` to `{ base: T, size, stride }`
    ///   if they would convert `S` to `T`
    ///
    /// [`AbstractInt`]: crate::ScalarKind::AbstractInt
    /// [`AbstractFloat`]: crate::ScalarKind::AbstractFloat
    /// [`Vector`]: crate::TypeInner::Vector
    /// [`Array`]: crate::TypeInner::Array
    pub fn automatically_converts_to(
        &self,
        goal: &Self,
        types: &crate::UniqueArena<crate::Type>,
    ) -> Option<(crate::Scalar, crate::Scalar)> {
        use crate::ScalarKind as Sk;
        use crate::TypeInner as Ti;

        // Automatic conversions only change the scalar type of a value's leaves
        // (e.g., `vec4<AbstractFloat>` to `vec4<f32>`), never the type
        // constructors applied to those scalar types (e.g., never scalar to
        // `vec4`, or `vec2` to `vec3`). So first we check that the type
        // constructors match, extracting the leaf scalar types in the process.
        let expr_scalar;
        let goal_scalar;
        match (self, goal) {
            (&Ti::Scalar(expr), &Ti::Scalar(goal)) => {
                expr_scalar = expr;
                goal_scalar = goal;
            }
            (
                &Ti::Vector {
                    size: expr_size,
                    scalar: expr,
                },
                &Ti::Vector {
                    size: goal_size,
                    scalar: goal,
                },
            ) if expr_size == goal_size => {
                expr_scalar = expr;
                goal_scalar = goal;
            }
            (
                &Ti::Matrix {
                    rows: expr_rows,
                    columns: expr_columns,
                    scalar: expr,
                },
                &Ti::Matrix {
                    rows: goal_rows,
                    columns: goal_columns,
                    scalar: goal,
                },
            ) if expr_rows == goal_rows && expr_columns == goal_columns => {
                expr_scalar = expr;
                goal_scalar = goal;
            }
            (
                &Ti::Array {
                    base: expr_base,
                    size: expr_size,
                    stride: _,
                },
                &Ti::Array {
                    base: goal_base,
                    size: goal_size,
                    stride: _,
                },
            ) if expr_size == goal_size => {
                return types[expr_base]
                    .inner
                    .automatically_converts_to(&types[goal_base].inner, types);
            }
            _ => return None,
        }

        match (expr_scalar.kind, goal_scalar.kind) {
            (Sk::AbstractFloat, Sk::Float) => {}
            (Sk::AbstractInt, Sk::Sint | Sk::Uint | Sk::AbstractFloat | Sk::Float) => {}
            _ => return None,
        }

        log::trace!("      okay: expr {expr_scalar:?}, goal {goal_scalar:?}");
        Some((expr_scalar, goal_scalar))
    }
}

/// Helper trait for providing the min and max values exactly representable by
/// the integer type `Self` and floating point type `F`.
pub trait IntFloatLimits<F>
where
    F: num_traits::Float,
{
    /// Returns the minimum value exactly representable by the integer type
    /// `Self` and floating point type `F`.
    fn min_float() -> F;
    /// Returns the maximum value exactly representable by the integer type
    /// `Self` and floating point type `F`.
    fn max_float() -> F;
}

macro_rules! define_int_float_limits {
    ($int:ty, $float:ty, $min:expr, $max:expr) => {
        impl IntFloatLimits<$float> for $int {
            fn min_float() -> $float {
                $min
            }
            fn max_float() -> $float {
                $max
            }
        }
    };
}

define_int_float_limits!(i32, half::f16, half::f16::MIN, half::f16::MAX);
define_int_float_limits!(u32, half::f16, half::f16::ZERO, half::f16::MAX);
define_int_float_limits!(i64, half::f16, half::f16::MIN, half::f16::MAX);
define_int_float_limits!(u64, half::f16, half::f16::ZERO, half::f16::MAX);
define_int_float_limits!(i32, f32, -2147483648.0f32, 2147483520.0f32);
define_int_float_limits!(u32, f32, 0.0f32, 4294967040.0f32);
define_int_float_limits!(
    i64,
    f32,
    -9223372036854775808.0f32,
    9223371487098961920.0f32
);
define_int_float_limits!(u64, f32, 0.0f32, 18446742974197923840.0f32);
define_int_float_limits!(i32, f64, -2147483648.0f64, 2147483647.0f64);
define_int_float_limits!(u32, f64, 0.0f64, 4294967295.0f64);
define_int_float_limits!(
    i64,
    f64,
    -9223372036854775808.0f64,
    9223372036854774784.0f64
);
define_int_float_limits!(u64, f64, 0.0f64, 18446744073709549568.0f64);

/// Returns a tuple of [`crate::Literal`]s representing the minimum and maximum
/// float values exactly representable by the provided float and integer types.
/// Panics if `float` is not one of `F16`, `F32`, or `F64`, or `int` is
/// not one of `I32`, `U32`, `I64`, or `U64`.
pub fn min_max_float_representable_by(
    float: crate::Scalar,
    int: crate::Scalar,
) -> (crate::Literal, crate::Literal) {
    match (float, int) {
        (crate::Scalar::F16, crate::Scalar::I32) => (
            crate::Literal::F16(i32::min_float()),
            crate::Literal::F16(i32::max_float()),
        ),
        (crate::Scalar::F16, crate::Scalar::U32) => (
            crate::Literal::F16(u32::min_float()),
            crate::Literal::F16(u32::max_float()),
        ),
        (crate::Scalar::F16, crate::Scalar::I64) => (
            crate::Literal::F16(i64::min_float()),
            crate::Literal::F16(i64::max_float()),
        ),
        (crate::Scalar::F16, crate::Scalar::U64) => (
            crate::Literal::F16(u64::min_float()),
            crate::Literal::F16(u64::max_float()),
        ),
        (crate::Scalar::F32, crate::Scalar::I32) => (
            crate::Literal::F32(i32::min_float()),
            crate::Literal::F32(i32::max_float()),
        ),
        (crate::Scalar::F32, crate::Scalar::U32) => (
            crate::Literal::F32(u32::min_float()),
            crate::Literal::F32(u32::max_float()),
        ),
        (crate::Scalar::F32, crate::Scalar::I64) => (
            crate::Literal::F32(i64::min_float()),
            crate::Literal::F32(i64::max_float()),
        ),
        (crate::Scalar::F32, crate::Scalar::U64) => (
            crate::Literal::F32(u64::min_float()),
            crate::Literal::F32(u64::max_float()),
        ),
        (crate::Scalar::F64, crate::Scalar::I32) => (
            crate::Literal::F64(i32::min_float()),
            crate::Literal::F64(i32::max_float()),
        ),
        (crate::Scalar::F64, crate::Scalar::U32) => (
            crate::Literal::F64(u32::min_float()),
            crate::Literal::F64(u32::max_float()),
        ),
        (crate::Scalar::F64, crate::Scalar::I64) => (
            crate::Literal::F64(i64::min_float()),
            crate::Literal::F64(i64::max_float()),
        ),
        (crate::Scalar::F64, crate::Scalar::U64) => (
            crate::Literal::F64(u64::min_float()),
            crate::Literal::F64(u64::max_float()),
        ),
        _ => unreachable!(),
    }
}

/// Helper function that returns the string corresponding to the [`VectorSize`](crate::VectorSize)
pub const fn vector_size_str(size: crate::VectorSize) -> &'static str {
    match size {
        crate::VectorSize::Bi => "2",
        crate::VectorSize::Tri => "3",
        crate::VectorSize::Quad => "4",
    }
}
