//! WGSL's automatic conversions for abstract types.

use alloc::{boxed::Box, string::String, vec::Vec};

use crate::common::wgsl::{TryToWgsl, TypeContext};
use crate::front::wgsl::error::{
    AutoConversionError, AutoConversionLeafScalarError, ConcretizationFailedError,
};
use crate::front::wgsl::Result;
use crate::{Handle, Span};

impl<'source> super::ExpressionContext<'source, '_, '_> {
    /// Try to use WGSL's automatic conversions to convert `expr` to `goal_ty`.
    ///
    /// If no conversions are necessary, return `expr` unchanged.
    ///
    /// If automatic conversions cannot convert `expr` to `goal_ty`, return an
    /// [`AutoConversion`] error.
    ///
    /// Although the Load Rule is one of the automatic conversions, this
    /// function assumes it has already been applied if appropriate, as
    /// indicated by the fact that the Rust type of `expr` is not `Typed<_>`.
    ///
    /// [`AutoConversion`]: super::Error::AutoConversion
    pub fn try_automatic_conversions(
        &mut self,
        expr: Handle<crate::Expression>,
        goal_ty: &crate::proc::TypeResolution,
        goal_span: Span,
    ) -> Result<'source, Handle<crate::Expression>> {
        let expr_span = self.get_expression_span(expr);
        // Keep the TypeResolution so we can get type names for
        // structs in error messages.
        let expr_resolution = super::resolve!(self, expr);
        let types = &self.module.types;
        let expr_inner = expr_resolution.inner_with(types);
        let goal_inner = goal_ty.inner_with(types);

        // We can only convert abstract types, so if `expr` is not abstract do not even
        // attempt conversion. This allows the validator to catch type errors correctly
        // rather than them being misreported as type conversion errors.
        // If the type is an array (of an array, etc) then we must check whether the
        // type of the innermost array's base type is abstract.
        if !expr_inner.is_abstract(types) {
            return Ok(expr);
        }

        // If `expr` already has the requested type, we're done.
        if self.module.compare_types(expr_resolution, goal_ty) {
            return Ok(expr);
        }

        let (_expr_scalar, goal_scalar) =
            match expr_inner.automatically_converts_to(goal_inner, types) {
                Some(scalars) => scalars,
                None => {
                    let source_type = self.type_resolution_to_string(expr_resolution);
                    let dest_type = self.type_resolution_to_string(goal_ty);

                    return Err(Box::new(super::Error::AutoConversion(Box::new(
                        AutoConversionError {
                            dest_span: goal_span,
                            dest_type,
                            source_span: expr_span,
                            source_type,
                        },
                    ))));
                }
            };

        self.convert_leaf_scalar(expr, expr_span, goal_scalar)
    }

    /// Try to convert `expr`'s leaf scalar to `goal_scalar` using automatic conversions.
    ///
    /// If no conversions are necessary, return `expr` unchanged.
    ///
    /// If automatic conversions cannot convert `expr` to `goal_scalar`, return
    /// an [`AutoConversionLeafScalar`] error.
    ///
    /// Although the Load Rule is one of the automatic conversions, this
    /// function assumes it has already been applied if appropriate, as
    /// indicated by the fact that the Rust type of `expr` is not `Typed<_>`.
    ///
    /// [`AutoConversionLeafScalar`]: super::Error::AutoConversionLeafScalar
    pub fn try_automatic_conversion_for_leaf_scalar(
        &mut self,
        expr: Handle<crate::Expression>,
        goal_scalar: crate::Scalar,
        goal_span: Span,
    ) -> Result<'source, Handle<crate::Expression>> {
        let expr_span = self.get_expression_span(expr);
        let expr_resolution = super::resolve!(self, expr);
        let types = &self.module.types;
        let expr_inner = expr_resolution.inner_with(types);

        let make_error = || {
            let source_type = self.type_resolution_to_string(expr_resolution);
            super::Error::AutoConversionLeafScalar(Box::new(AutoConversionLeafScalarError {
                dest_span: goal_span,
                dest_scalar: goal_scalar.to_wgsl_for_diagnostics(),
                source_span: expr_span,
                source_type,
            }))
        };

        let expr_scalar = match expr_inner.automatically_convertible_scalar(&self.module.types) {
            Some(scalar) => scalar,
            None => return Err(Box::new(make_error())),
        };

        if expr_scalar == goal_scalar {
            return Ok(expr);
        }

        if !expr_scalar.automatically_converts_to(goal_scalar) {
            return Err(Box::new(make_error()));
        }

        assert!(expr_scalar.is_abstract());

        self.convert_leaf_scalar(expr, expr_span, goal_scalar)
    }

    fn convert_leaf_scalar(
        &mut self,
        expr: Handle<crate::Expression>,
        expr_span: Span,
        goal_scalar: crate::Scalar,
    ) -> Result<'source, Handle<crate::Expression>> {
        let expr_inner = super::resolve_inner!(self, expr);
        if let crate::TypeInner::Array { .. } = *expr_inner {
            self.as_const_evaluator()
                .cast_array(expr, goal_scalar, expr_span)
                .map_err(|err| {
                    Box::new(super::Error::ConstantEvaluatorError(err.into(), expr_span))
                })
        } else {
            let cast = crate::Expression::As {
                expr,
                kind: goal_scalar.kind,
                convert: Some(goal_scalar.width),
            };
            self.append_expression(cast, expr_span)
        }
    }

    /// Try to convert `exprs` to `goal_ty` using WGSL's automatic conversions.
    pub fn try_automatic_conversions_slice(
        &mut self,
        exprs: &mut [Handle<crate::Expression>],
        goal_ty: &crate::proc::TypeResolution,
        goal_span: Span,
    ) -> Result<'source, ()> {
        for expr in exprs.iter_mut() {
            *expr = self.try_automatic_conversions(*expr, goal_ty, goal_span)?;
        }

        Ok(())
    }

    /// Apply WGSL's automatic conversions to a vector constructor's arguments.
    ///
    /// When calling a vector constructor like `vec3<f32>(...)`, the parameters
    /// can be a mix of scalars and vectors, with the latter being spread out to
    /// contribute each of their components as a component of the new value.
    /// When the element type is explicit, as with `<f32>` in the example above,
    /// WGSL's automatic conversions should convert abstract scalar and vector
    /// parameters to the constructor's required scalar type.
    pub fn try_automatic_conversions_for_vector(
        &mut self,
        exprs: &mut [Handle<crate::Expression>],
        goal_scalar: crate::Scalar,
        goal_span: Span,
    ) -> Result<'source, ()> {
        use crate::proc::TypeResolution as Tr;
        use crate::TypeInner as Ti;
        let goal_scalar_res = Tr::Value(Ti::Scalar(goal_scalar));

        for (i, expr) in exprs.iter_mut().enumerate() {
            // Keep the TypeResolution so we can get full type names
            // in error messages.
            let expr_resolution = super::resolve!(self, *expr);
            let types = &self.module.types;
            let expr_inner = expr_resolution.inner_with(types);

            match *expr_inner {
                Ti::Scalar(_) => {
                    *expr = self.try_automatic_conversions(*expr, &goal_scalar_res, goal_span)?;
                }
                Ti::Vector { size, scalar: _ } => {
                    let goal_vector_res = Tr::Value(Ti::Vector {
                        size,
                        scalar: goal_scalar,
                    });
                    *expr = self.try_automatic_conversions(*expr, &goal_vector_res, goal_span)?;
                }
                _ => {
                    let span = self.get_expression_span(*expr);
                    return Err(Box::new(super::Error::InvalidConstructorComponentType(
                        span, i as i32,
                    )));
                }
            }
        }

        Ok(())
    }

    /// Convert `expr` to the leaf scalar type `scalar`.
    pub fn convert_to_leaf_scalar(
        &mut self,
        expr: &mut Handle<crate::Expression>,
        goal: crate::Scalar,
    ) -> Result<'source, ()> {
        let inner = super::resolve_inner!(self, *expr);
        // Do nothing if `inner` doesn't even have leaf scalars;
        // it's a type error that validation will catch.
        if inner.scalar() != Some(goal) {
            let cast = crate::Expression::As {
                expr: *expr,
                kind: goal.kind,
                convert: Some(goal.width),
            };
            let expr_span = self.get_expression_span(*expr);
            *expr = self.append_expression(cast, expr_span)?;
        }

        Ok(())
    }

    /// Convert all expressions in `exprs` to a common scalar type.
    ///
    /// Note that the caller is responsible for making sure these
    /// conversions are actually justified. This function simply
    /// generates `As` expressions, regardless of whether they are
    /// permitted WGSL automatic conversions. Callers intending to
    /// implement automatic conversions need to determine for
    /// themselves whether the casts we we generate are justified,
    /// perhaps by calling `TypeInner::automatically_converts_to` or
    /// `Scalar::automatic_conversion_combine`.
    pub fn convert_slice_to_common_leaf_scalar(
        &mut self,
        exprs: &mut [Handle<crate::Expression>],
        goal: crate::Scalar,
    ) -> Result<'source, ()> {
        for expr in exprs.iter_mut() {
            self.convert_to_leaf_scalar(expr, goal)?;
        }

        Ok(())
    }

    /// Return an expression for the concretized value of `expr`.
    ///
    /// If `expr` is already concrete, return it unchanged.
    pub fn concretize(
        &mut self,
        expr: Handle<crate::Expression>,
    ) -> Result<'source, Handle<crate::Expression>> {
        let inner = super::resolve_inner!(self, expr);
        if let Some(scalar) = inner.automatically_convertible_scalar(&self.module.types) {
            use crate::ScalarKind as Sk;
            let concretization_preferences = match scalar.kind {
                // already concrete
                Sk::Sint | Sk::Uint | Sk::Float | Sk::Bool => return Ok(expr),
                Sk::AbstractInt => {
                    [crate::Scalar::I32, crate::Scalar::U32, crate::Scalar::F32].as_slice()
                }
                Sk::AbstractFloat => [crate::Scalar::F32].as_slice(),
            };
            let expr_span = self.get_expression_span(expr);
            let mut errors = Vec::new();
            for concrete_scalar in concretization_preferences {
                match self
                    .as_const_evaluator()
                    .cast_array(expr, *concrete_scalar, expr_span)
                {
                    Ok(expr) => return Ok(expr),
                    Err(err) => {
                        errors.push((concrete_scalar.to_wgsl_for_diagnostics(), err));
                    }
                }
            }
            if !errors.is_empty() {
                // A `TypeResolution` includes the type's full name, if
                // it has one. Also, avoid holding the borrow of `inner`
                // across the call to `cast_array`.
                let expr_type = &self.typifier()[expr];
                return Err(Box::new(super::Error::ConcretizationFailed(Box::new(
                    ConcretizationFailedError {
                        expr_span,
                        expr_type: self.type_resolution_to_string(expr_type),
                        concretization_preferences: errors,
                    },
                ))));
            }
        }

        Ok(expr)
    }

    /// Find the consensus scalar of `components` under WGSL's automatic
    /// conversions.
    ///
    /// If `components` can all be converted to any common scalar via
    /// WGSL's automatic conversions, return the best such scalar.
    ///
    /// The `components` slice must not be empty. All elements' types must
    /// have been resolved.
    ///
    /// If `components` are definitely not acceptable as arguments to such
    /// constructors, return `Err(i)`, where `i` is the index in
    /// `components` of some problematic argument.
    ///
    /// If `base` is `Some(scalar)`, the consensus scalar must also be
    /// compatible with that `scalar`. This is used to restrict matrix
    /// initializers to floating-point types.
    ///
    /// This function doesn't fully type-check the arguments - it only
    /// considers their leaf scalar types. This means it may return `Ok`
    /// even when the Naga validator will reject the resulting
    /// construction expression later.
    pub fn automatic_conversion_consensus<'handle, I>(
        &self,
        base: Option<crate::Scalar>,
        components: I,
    ) -> core::result::Result<crate::Scalar, usize>
    where
        I: IntoIterator<Item = &'handle Handle<crate::Expression>>,
        I::IntoIter: Clone, // for debugging
    {
        let types = &self.module.types;
        let components_iter = components.into_iter();
        log::debug!(
            "wgsl automatic_conversion_consensus: {}",
            components_iter
                .clone()
                .map(|&expr| {
                    let res = &self.typifier()[expr];
                    self.type_resolution_to_string(res)
                })
                .collect::<Vec<String>>()
                .join(", ")
        );
        let mut components_iter = components_iter
            .map(|&c| self.typifier()[c].inner_with(types).scalar())
            .enumerate();
        let base = base
            .or_else(|| components_iter.next().unwrap().1)
            .ok_or(0usize)?;
        let best = components_iter.try_fold(base, |best, (i, scalar)| {
            scalar
                .and_then(|scalar| best.automatic_conversion_combine(scalar))
                .ok_or(i)
        })?;
        log::debug!("    consensus: {}", best.to_wgsl_for_diagnostics());
        Ok(best)
    }
}

impl crate::TypeInner {
    fn automatically_convertible_scalar(
        &self,
        types: &crate::UniqueArena<crate::Type>,
    ) -> Option<crate::Scalar> {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Scalar(scalar) | Ti::Vector { scalar, .. } | Ti::Matrix { scalar, .. } => {
                Some(scalar)
            }
            Ti::CooperativeMatrix { .. } => None,
            Ti::Array { base, .. } => types[base].inner.automatically_convertible_scalar(types),
            Ti::Atomic(_)
            | Ti::Pointer { .. }
            | Ti::ValuePointer { .. }
            | Ti::Struct { .. }
            | Ti::Image { .. }
            | Ti::Sampler { .. }
            | Ti::AccelerationStructure { .. }
            | Ti::RayQuery { .. }
            | Ti::BindingArray { .. } => None,
        }
    }

    /// Return the leaf scalar type of `pointer`.
    ///
    /// `pointer` must be a `TypeInner` representing a pointer type.
    pub fn pointer_automatically_convertible_scalar(
        &self,
        types: &crate::UniqueArena<crate::Type>,
    ) -> Option<crate::Scalar> {
        use crate::TypeInner as Ti;
        match *self {
            Ti::Scalar(scalar) | Ti::Vector { scalar, .. } | Ti::Matrix { scalar, .. } => {
                Some(scalar)
            }
            Ti::CooperativeMatrix { .. } => None,
            Ti::Atomic(_) => None,
            Ti::Pointer { base, .. } | Ti::Array { base, .. } => {
                types[base].inner.automatically_convertible_scalar(types)
            }
            Ti::ValuePointer { scalar, .. } => Some(scalar),
            Ti::Struct { .. }
            | Ti::Image { .. }
            | Ti::Sampler { .. }
            | Ti::AccelerationStructure { .. }
            | Ti::RayQuery { .. }
            | Ti::BindingArray { .. } => None,
        }
    }
}

impl crate::Scalar {
    /// Find the common type of `self` and `other` under WGSL's
    /// automatic conversions.
    ///
    /// If there are any scalars to which WGSL's automatic conversions
    /// will convert both `self` and `other`, return the best such
    /// scalar. Otherwise, return `None`.
    pub const fn automatic_conversion_combine(self, other: Self) -> Option<crate::Scalar> {
        use crate::ScalarKind as Sk;

        match (self.kind, other.kind) {
            // When the kinds match...
            (Sk::AbstractFloat, Sk::AbstractFloat)
            | (Sk::AbstractInt, Sk::AbstractInt)
            | (Sk::Sint, Sk::Sint)
            | (Sk::Uint, Sk::Uint)
            | (Sk::Float, Sk::Float)
            | (Sk::Bool, Sk::Bool) => {
                if self.width == other.width {
                    // ... either no conversion is necessary ...
                    Some(self)
                } else {
                    // ... or no conversion is possible.
                    // We never convert concrete to concrete, and
                    // abstract types should have only one size.
                    None
                }
            }

            // AbstractInt converts to AbstractFloat.
            (Sk::AbstractFloat, Sk::AbstractInt) => Some(self),
            (Sk::AbstractInt, Sk::AbstractFloat) => Some(other),

            // AbstractFloat converts to Float.
            (Sk::AbstractFloat, Sk::Float) => Some(other),
            (Sk::Float, Sk::AbstractFloat) => Some(self),

            // AbstractInt converts to concrete integer or float.
            (Sk::AbstractInt, Sk::Uint | Sk::Sint | Sk::Float) => Some(other),
            (Sk::Uint | Sk::Sint | Sk::Float, Sk::AbstractInt) => Some(self),

            // AbstractFloat can't be reconciled with concrete integer types.
            (Sk::AbstractFloat, Sk::Uint | Sk::Sint) | (Sk::Uint | Sk::Sint, Sk::AbstractFloat) => {
                None
            }

            // Nothing can be reconciled with `bool`.
            (Sk::Bool, _) | (_, Sk::Bool) => None,

            // Different concrete types cannot be reconciled.
            (Sk::Sint | Sk::Uint | Sk::Float, Sk::Sint | Sk::Uint | Sk::Float) => None,
        }
    }

    /// Return `true` if automatic conversions will covert `self` to `goal`.
    pub fn automatically_converts_to(self, goal: Self) -> bool {
        self.automatic_conversion_combine(goal) == Some(goal)
    }

    pub(in crate::front::wgsl) const fn concretize(self) -> Self {
        use crate::ScalarKind as Sk;
        match self.kind {
            Sk::Sint | Sk::Uint | Sk::Float | Sk::Bool => self,
            Sk::AbstractInt => Self::I32,
            Sk::AbstractFloat => Self::F32,
        }
    }
}
