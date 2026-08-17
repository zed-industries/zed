use super::{compose::validate_compose, FunctionInfo, ModuleInfo, ShaderStages, TypeFlags};
use crate::arena::UniqueArena;
use crate::{
    arena::Handle,
    proc::OverloadSet as _,
    proc::{IndexableLengthError, ResolveError},
};

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ExpressionError {
    #[error("Used by a statement before it was introduced into the scope by any of the dominating blocks")]
    NotInScope,
    #[error("Base type {0:?} is not compatible with this expression")]
    InvalidBaseType(Handle<crate::Expression>),
    #[error("Accessing with index {0:?} can't be done")]
    InvalidIndexType(Handle<crate::Expression>),
    #[error("Accessing {0:?} via a negative index is invalid")]
    NegativeIndex(Handle<crate::Expression>),
    #[error("Accessing index {1} is out of {0:?} bounds")]
    IndexOutOfBounds(Handle<crate::Expression>, u32),
    #[error("Function argument {0:?} doesn't exist")]
    FunctionArgumentDoesntExist(u32),
    #[error("Loading of {0:?} can't be done")]
    InvalidPointerType(Handle<crate::Expression>),
    #[error("Array length of {0:?} can't be done")]
    InvalidArrayType(Handle<crate::Expression>),
    #[error("Get intersection of {0:?} can't be done")]
    InvalidRayQueryType(Handle<crate::Expression>),
    #[error("Splatting {0:?} can't be done")]
    InvalidSplatType(Handle<crate::Expression>),
    #[error("Swizzling {0:?} can't be done")]
    InvalidVectorType(Handle<crate::Expression>),
    #[error("Swizzle component {0:?} is outside of vector size {1:?}")]
    InvalidSwizzleComponent(crate::SwizzleComponent, crate::VectorSize),
    #[error(transparent)]
    Compose(#[from] super::ComposeError),
    #[error("Cannot construct zero value of {0:?} because it is not a constructible type")]
    InvalidZeroValue(Handle<crate::Type>),
    #[error(transparent)]
    IndexableLength(#[from] IndexableLengthError),
    #[error("Operation {0:?} can't work with {1:?}")]
    InvalidUnaryOperandType(crate::UnaryOperator, Handle<crate::Expression>),
    #[error(
        "Operation {:?} can't work with {:?} (of type {:?}) and {:?} (of type {:?})",
        op,
        lhs_expr,
        lhs_type,
        rhs_expr,
        rhs_type
    )]
    InvalidBinaryOperandTypes {
        op: crate::BinaryOperator,
        lhs_expr: Handle<crate::Expression>,
        lhs_type: crate::TypeInner,
        rhs_expr: Handle<crate::Expression>,
        rhs_type: crate::TypeInner,
    },
    #[error("Expected selection argument types to match, but reject value of type {reject:?} does not match accept value of value {accept:?}")]
    SelectValuesTypeMismatch {
        accept: crate::TypeInner,
        reject: crate::TypeInner,
    },
    #[error("Expected selection condition to be a boolean value, got {actual:?}")]
    SelectConditionNotABool { actual: crate::TypeInner },
    #[error("Relational argument {0:?} is not a boolean vector")]
    InvalidBooleanVector(Handle<crate::Expression>),
    #[error("Relational argument {0:?} is not a float")]
    InvalidFloatArgument(Handle<crate::Expression>),
    #[error("Type resolution failed")]
    Type(#[from] ResolveError),
    #[error("Not a global variable")]
    ExpectedGlobalVariable,
    #[error("Not a global variable or a function argument")]
    ExpectedGlobalOrArgument,
    #[error("Needs to be an binding array instead of {0:?}")]
    ExpectedBindingArrayType(Handle<crate::Type>),
    #[error("Needs to be an image instead of {0:?}")]
    ExpectedImageType(Handle<crate::Type>),
    #[error("Needs to be an image instead of {0:?}")]
    ExpectedSamplerType(Handle<crate::Type>),
    #[error("Unable to operate on image class {0:?}")]
    InvalidImageClass(crate::ImageClass),
    #[error("Image atomics are not supported for storage format {0:?}")]
    InvalidImageFormat(crate::StorageFormat),
    #[error("Image atomics require atomic storage access, {0:?} is insufficient")]
    InvalidImageStorageAccess(crate::StorageAccess),
    #[error("Derivatives can only be taken from scalar and vector floats")]
    InvalidDerivative,
    #[error("Image array index parameter is misplaced")]
    InvalidImageArrayIndex,
    #[error("Cannot textureLoad from a specific multisample sample on a non-multisampled image.")]
    InvalidImageSampleSelector,
    #[error("Cannot textureLoad from a multisampled image without specifying a sample.")]
    MissingImageSampleSelector,
    #[error("Cannot textureLoad with a specific mip level on a non-mipmapped image.")]
    InvalidImageLevelSelector,
    #[error("Cannot textureLoad from a mipmapped image without specifying a level.")]
    MissingImageLevelSelector,
    #[error("Image array index type of {0:?} is not an integer scalar")]
    InvalidImageArrayIndexType(Handle<crate::Expression>),
    #[error("Image sample or level-of-detail index's type of {0:?} is not an integer scalar")]
    InvalidImageOtherIndexType(Handle<crate::Expression>),
    #[error("Image coordinate type of {1:?} does not match dimension {0:?}")]
    InvalidImageCoordinateType(crate::ImageDimension, Handle<crate::Expression>),
    #[error("Comparison sampling mismatch: image has class {image:?}, but the sampler is comparison={sampler}, and the reference was provided={has_ref}")]
    ComparisonSamplingMismatch {
        image: crate::ImageClass,
        sampler: bool,
        has_ref: bool,
    },
    #[error("Sample offset must be a const-expression")]
    InvalidSampleOffsetExprType,
    #[error("Sample offset constant {1:?} doesn't match the image dimension {0:?}")]
    InvalidSampleOffset(crate::ImageDimension, Handle<crate::Expression>),
    #[error("Depth reference {0:?} is not a scalar float")]
    InvalidDepthReference(Handle<crate::Expression>),
    #[error("Depth sample level can only be Auto or Zero")]
    InvalidDepthSampleLevel,
    #[error("Gather level can only be Zero")]
    InvalidGatherLevel,
    #[error("Gather component {0:?} doesn't exist in the image")]
    InvalidGatherComponent(crate::SwizzleComponent),
    #[error("Gather can't be done for image dimension {0:?}")]
    InvalidGatherDimension(crate::ImageDimension),
    #[error("Sample level (exact) type {0:?} has an invalid type")]
    InvalidSampleLevelExactType(Handle<crate::Expression>),
    #[error("Sample level (bias) type {0:?} is not a scalar float")]
    InvalidSampleLevelBiasType(Handle<crate::Expression>),
    #[error("Bias can't be done for image dimension {0:?}")]
    InvalidSampleLevelBiasDimension(crate::ImageDimension),
    #[error("Sample level (gradient) of {1:?} doesn't match the image dimension {0:?}")]
    InvalidSampleLevelGradientType(crate::ImageDimension, Handle<crate::Expression>),
    #[error("Clamping sample coordinate to edge is not supported with {0}")]
    InvalidSampleClampCoordinateToEdge(alloc::string::String),
    #[error("Unable to cast")]
    InvalidCastArgument,
    #[error("Invalid argument count for {0:?}")]
    WrongArgumentCount(crate::MathFunction),
    #[error("Argument [{1}] to {0:?} as expression {2:?} has an invalid type.")]
    InvalidArgumentType(crate::MathFunction, u32, Handle<crate::Expression>),
    #[error(
        "workgroupUniformLoad result type can't be {0:?}. It can only be a constructible type."
    )]
    InvalidWorkGroupUniformLoadResultType(Handle<crate::Type>),
    #[error("Shader requires capability {0:?}")]
    MissingCapabilities(super::Capabilities),
    #[error(transparent)]
    Literal(#[from] LiteralError),
    #[error("{0:?} is not supported for Width {2} {1:?} arguments yet, see https://github.com/gfx-rs/wgpu/issues/5276")]
    UnsupportedWidth(crate::MathFunction, crate::ScalarKind, crate::Bytes),
    #[error("Invalid operand for cooperative op")]
    InvalidCooperativeOperand(Handle<crate::Expression>),
    #[error("Shift amount exceeds the bit width of {lhs_type:?}")]
    ShiftAmountTooLarge {
        lhs_type: crate::TypeInner,
        rhs_expr: Handle<crate::Expression>,
    },
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum ConstExpressionError {
    #[error("The expression is not a constant or override expression")]
    NonConstOrOverride,
    #[error("The expression is not a fully evaluated constant expression")]
    NonFullyEvaluatedConst,
    #[error(transparent)]
    Compose(#[from] super::ComposeError),
    #[error("Splatting {0:?} can't be done")]
    InvalidSplatType(Handle<crate::Expression>),
    #[error("Type resolution failed")]
    Type(#[from] ResolveError),
    #[error(transparent)]
    Literal(#[from] LiteralError),
    #[error(transparent)]
    Width(#[from] super::r#type::WidthError),
}

#[derive(Clone, Debug, thiserror::Error)]
#[cfg_attr(test, derive(PartialEq))]
pub enum LiteralError {
    #[error("Float literal is NaN")]
    NaN,
    #[error("Float literal is infinite")]
    Infinity,
    #[error(transparent)]
    Width(#[from] super::r#type::WidthError),
}

struct ExpressionTypeResolver<'a> {
    root: Handle<crate::Expression>,
    types: &'a UniqueArena<crate::Type>,
    info: &'a FunctionInfo,
}

impl core::ops::Index<Handle<crate::Expression>> for ExpressionTypeResolver<'_> {
    type Output = crate::TypeInner;

    #[allow(clippy::panic)]
    fn index(&self, handle: Handle<crate::Expression>) -> &Self::Output {
        if handle < self.root {
            self.info[handle].ty.inner_with(self.types)
        } else {
            // `Validator::validate_module_handles` should have caught this.
            panic!(
                "Depends on {:?}, which has not been processed yet",
                self.root
            )
        }
    }
}

impl super::Validator {
    pub(super) fn validate_const_expression(
        &self,
        handle: Handle<crate::Expression>,
        gctx: crate::proc::GlobalCtx,
        mod_info: &ModuleInfo,
        global_expr_kind: &crate::proc::ExpressionKindTracker,
    ) -> Result<(), ConstExpressionError> {
        use crate::Expression as E;

        if !global_expr_kind.is_const_or_override(handle) {
            return Err(ConstExpressionError::NonConstOrOverride);
        }

        match gctx.global_expressions[handle] {
            E::Literal(literal) => {
                self.validate_literal(literal)?;
            }
            E::Constant(_) | E::ZeroValue(_) => {}
            E::Compose { ref components, ty } => {
                validate_compose(
                    ty,
                    gctx,
                    components.iter().map(|&handle| mod_info[handle].clone()),
                )?;
            }
            E::Splat { value, .. } => match *mod_info[value].inner_with(gctx.types) {
                crate::TypeInner::Scalar { .. } => {}
                _ => return Err(ConstExpressionError::InvalidSplatType(value)),
            },
            _ if global_expr_kind.is_const(handle) || self.overrides_resolved => {
                return Err(ConstExpressionError::NonFullyEvaluatedConst)
            }
            // the constant evaluator will report errors about override-expressions
            _ => {}
        }

        Ok(())
    }

    /// Return an error if a constant shift amount in `right` exceeds the bit
    /// width of `left_ty`.
    ///
    /// This function promises to return an error in cases where (1) the
    /// expression is well-typed, (2) `left_ty` is a concrete integer, and
    /// (3) the shift will overflow. It does not return an error in cases where
    /// the expression is not well-typed (e.g. vector dimension mismatch),
    /// because those will be rejected elsewhere.
    fn validate_constant_shift_amounts(
        left_ty: &crate::TypeInner,
        right: Handle<crate::Expression>,
        module: &crate::Module,
        function: &crate::Function,
    ) -> Result<(), ExpressionError> {
        fn is_overflowing_shift(
            left_ty: &crate::TypeInner,
            right: Handle<crate::Expression>,
            module: &crate::Module,
            function: &crate::Function,
        ) -> bool {
            let Some((vec_size, scalar)) = left_ty.vector_size_and_scalar() else {
                return false;
            };
            if !matches!(
                scalar.kind,
                crate::ScalarKind::Sint | crate::ScalarKind::Uint
            ) {
                return false;
            }
            let lhs_bits = u32::from(8 * scalar.width);
            if vec_size.is_none() {
                let shift_amount = module
                    .to_ctx()
                    .get_const_val_from::<u32, _>(right, &function.expressions);
                shift_amount.ok().is_some_and(|s| s >= lhs_bits)
            } else {
                match function.expressions[right] {
                    crate::Expression::ZeroValue(_) => false, // zero shift does not overflow
                    crate::Expression::Splat { value, .. } => module
                        .to_ctx()
                        .get_const_val_from::<u32, _>(value, &function.expressions)
                        .ok()
                        .is_some_and(|s| s >= lhs_bits),
                    crate::Expression::Compose {
                        ty: _,
                        ref components,
                    } => components.iter().any(|comp| {
                        module
                            .to_ctx()
                            .get_const_val_from::<u32, _>(*comp, &function.expressions)
                            .ok()
                            .is_some_and(|s| s >= lhs_bits)
                    }),
                    _ => false,
                }
            }
        }

        if is_overflowing_shift(left_ty, right, module, function) {
            Err(ExpressionError::ShiftAmountTooLarge {
                lhs_type: left_ty.clone(),
                rhs_expr: right,
            })
        } else {
            Ok(())
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn validate_expression(
        &self,
        root: Handle<crate::Expression>,
        expression: &crate::Expression,
        function: &crate::Function,
        module: &crate::Module,
        info: &FunctionInfo,
        mod_info: &ModuleInfo,
        expr_kind: &crate::proc::ExpressionKindTracker,
    ) -> Result<ShaderStages, ExpressionError> {
        use crate::{Expression as E, Scalar as Sc, ScalarKind as Sk, TypeInner as Ti};

        let resolver = ExpressionTypeResolver {
            root,
            types: &module.types,
            info,
        };

        let stages = match *expression {
            E::Access { base, index } => {
                let base_type = &resolver[base];
                match *base_type {
                    Ti::Matrix { .. }
                    | Ti::Vector { .. }
                    | Ti::Array { .. }
                    | Ti::Pointer { .. }
                    | Ti::ValuePointer { size: Some(_), .. }
                    | Ti::BindingArray { .. } => {}
                    ref other => {
                        log::debug!("Indexing of {other:?}");
                        return Err(ExpressionError::InvalidBaseType(base));
                    }
                };
                match resolver[index] {
                    //TODO: only allow one of these
                    Ti::Scalar(Sc {
                        kind: Sk::Sint | Sk::Uint,
                        ..
                    }) => {}
                    ref other => {
                        log::debug!("Indexing by {other:?}");
                        return Err(ExpressionError::InvalidIndexType(index));
                    }
                }

                // If index is const we can do check for non-negative index
                match module
                    .to_ctx()
                    .get_const_val_from(index, &function.expressions)
                {
                    Ok(value) => {
                        let length = if self.overrides_resolved {
                            base_type.indexable_length_resolved(module)
                        } else {
                            base_type.indexable_length_pending(module)
                        }?;
                        // If we know both the length and the index, we can do the
                        // bounds check now.
                        if let crate::proc::IndexableLength::Known(known_length) = length {
                            if value >= known_length {
                                return Err(ExpressionError::IndexOutOfBounds(base, value));
                            }
                        }
                    }
                    Err(crate::proc::ConstValueError::Negative) => {
                        return Err(ExpressionError::NegativeIndex(base))
                    }
                    Err(crate::proc::ConstValueError::NonConst) => {}
                    Err(crate::proc::ConstValueError::InvalidType) => {
                        return Err(ExpressionError::InvalidIndexType(index))
                    }
                }

                ShaderStages::all()
            }
            E::AccessIndex { base, index } => {
                fn resolve_index_limit(
                    module: &crate::Module,
                    top: Handle<crate::Expression>,
                    ty: &crate::TypeInner,
                    top_level: bool,
                ) -> Result<u32, ExpressionError> {
                    let limit = match *ty {
                        Ti::Vector { size, .. }
                        | Ti::ValuePointer {
                            size: Some(size), ..
                        } => size as u32,
                        Ti::Matrix { columns, .. } => columns as u32,
                        Ti::Array {
                            size: crate::ArraySize::Constant(len),
                            ..
                        } => len.get(),
                        Ti::Array { .. } | Ti::BindingArray { .. } => u32::MAX, // can't statically know, but need run-time checks
                        Ti::Pointer { base, .. } if top_level => {
                            resolve_index_limit(module, top, &module.types[base].inner, false)?
                        }
                        Ti::Struct { ref members, .. } => members.len() as u32,
                        ref other => {
                            log::debug!("Indexing of {other:?}");
                            return Err(ExpressionError::InvalidBaseType(top));
                        }
                    };
                    Ok(limit)
                }

                let limit = resolve_index_limit(module, base, &resolver[base], true)?;
                if index >= limit {
                    return Err(ExpressionError::IndexOutOfBounds(base, limit));
                }
                ShaderStages::all()
            }
            E::Splat { size: _, value } => match resolver[value] {
                Ti::Scalar { .. } => ShaderStages::all(),
                ref other => {
                    log::debug!("Splat scalar type {other:?}");
                    return Err(ExpressionError::InvalidSplatType(value));
                }
            },
            E::Swizzle {
                size,
                vector,
                pattern,
            } => {
                let vec_size = match resolver[vector] {
                    Ti::Vector { size: vec_size, .. } => vec_size,
                    ref other => {
                        log::debug!("Swizzle vector type {other:?}");
                        return Err(ExpressionError::InvalidVectorType(vector));
                    }
                };
                for &sc in pattern[..size as usize].iter() {
                    if sc as u8 >= vec_size as u8 {
                        return Err(ExpressionError::InvalidSwizzleComponent(sc, vec_size));
                    }
                }
                ShaderStages::all()
            }
            E::Literal(literal) => {
                self.validate_literal(literal)?;
                ShaderStages::all()
            }
            E::Constant(_) | E::Override(_) => ShaderStages::all(),
            E::ZeroValue(ty) => {
                if !mod_info[ty].contains(TypeFlags::CONSTRUCTIBLE) {
                    return Err(ExpressionError::InvalidZeroValue(ty));
                }
                ShaderStages::all()
            }
            E::Compose { ref components, ty } => {
                validate_compose(
                    ty,
                    module.to_ctx(),
                    components.iter().map(|&handle| info[handle].ty.clone()),
                )?;
                ShaderStages::all()
            }
            E::FunctionArgument(index) => {
                if index >= function.arguments.len() as u32 {
                    return Err(ExpressionError::FunctionArgumentDoesntExist(index));
                }
                ShaderStages::all()
            }
            E::GlobalVariable(_handle) => ShaderStages::all(),
            E::LocalVariable(_handle) => ShaderStages::all(),
            E::Load { pointer } => {
                match resolver[pointer] {
                    Ti::Pointer { base, .. }
                        if self.types[base.index()]
                            .flags
                            .contains(TypeFlags::SIZED | TypeFlags::DATA) => {}
                    Ti::ValuePointer { .. } => {}
                    ref other => {
                        log::debug!("Loading {other:?}");
                        return Err(ExpressionError::InvalidPointerType(pointer));
                    }
                }
                ShaderStages::all()
            }
            E::ImageSample {
                image,
                sampler,
                gather,
                coordinate,
                array_index,
                offset,
                level,
                depth_ref,
                clamp_to_edge,
            } => {
                // check the validity of expressions
                let image_ty = Self::global_var_ty(module, function, image)?;
                let sampler_ty = Self::global_var_ty(module, function, sampler)?;

                let comparison = match module.types[sampler_ty].inner {
                    Ti::Sampler { comparison } => comparison,
                    _ => return Err(ExpressionError::ExpectedSamplerType(sampler_ty)),
                };

                let (class, dim) = match module.types[image_ty].inner {
                    Ti::Image {
                        class,
                        arrayed,
                        dim,
                    } => {
                        // check the array property
                        if arrayed != array_index.is_some() {
                            return Err(ExpressionError::InvalidImageArrayIndex);
                        }
                        if let Some(expr) = array_index {
                            match resolver[expr] {
                                Ti::Scalar(Sc {
                                    kind: Sk::Sint | Sk::Uint,
                                    ..
                                }) => {}
                                _ => return Err(ExpressionError::InvalidImageArrayIndexType(expr)),
                            }
                        }
                        (class, dim)
                    }
                    _ => return Err(ExpressionError::ExpectedImageType(image_ty)),
                };

                // check sampling and comparison properties
                let image_depth = match class {
                    crate::ImageClass::Sampled {
                        kind: crate::ScalarKind::Float,
                        multi: false,
                    } => false,
                    crate::ImageClass::Sampled {
                        kind: crate::ScalarKind::Uint | crate::ScalarKind::Sint,
                        multi: false,
                    } if gather.is_some() => false,
                    crate::ImageClass::External => false,
                    crate::ImageClass::Depth { multi: false } => true,
                    _ => return Err(ExpressionError::InvalidImageClass(class)),
                };
                if comparison != depth_ref.is_some() || (comparison && !image_depth) {
                    return Err(ExpressionError::ComparisonSamplingMismatch {
                        image: class,
                        sampler: comparison,
                        has_ref: depth_ref.is_some(),
                    });
                }

                // check texture coordinates type
                let num_components = match dim {
                    crate::ImageDimension::D1 => 1,
                    crate::ImageDimension::D2 => 2,
                    crate::ImageDimension::D3 | crate::ImageDimension::Cube => 3,
                };
                match resolver[coordinate] {
                    Ti::Scalar(Sc {
                        kind: Sk::Float, ..
                    }) if num_components == 1 => {}
                    Ti::Vector {
                        size,
                        scalar:
                            Sc {
                                kind: Sk::Float, ..
                            },
                    } if size as u32 == num_components => {}
                    _ => return Err(ExpressionError::InvalidImageCoordinateType(dim, coordinate)),
                }

                // check constant offset
                if let Some(const_expr) = offset {
                    if !expr_kind.is_const(const_expr) {
                        return Err(ExpressionError::InvalidSampleOffsetExprType);
                    }

                    match resolver[const_expr] {
                        Ti::Scalar(Sc { kind: Sk::Sint, .. }) if num_components == 1 => {}
                        Ti::Vector {
                            size,
                            scalar: Sc { kind: Sk::Sint, .. },
                        } if size as u32 == num_components => {}
                        _ => {
                            return Err(ExpressionError::InvalidSampleOffset(dim, const_expr));
                        }
                    }
                }

                // check depth reference type
                if let Some(expr) = depth_ref {
                    match resolver[expr] {
                        Ti::Scalar(Sc {
                            kind: Sk::Float, ..
                        }) => {}
                        _ => return Err(ExpressionError::InvalidDepthReference(expr)),
                    }
                    match level {
                        crate::SampleLevel::Auto | crate::SampleLevel::Zero => {}
                        _ => return Err(ExpressionError::InvalidDepthSampleLevel),
                    }
                }

                if let Some(component) = gather {
                    match dim {
                        crate::ImageDimension::D2 | crate::ImageDimension::Cube => {}
                        crate::ImageDimension::D1 | crate::ImageDimension::D3 => {
                            return Err(ExpressionError::InvalidGatherDimension(dim))
                        }
                    };
                    let max_component = match class {
                        crate::ImageClass::Depth { .. } => crate::SwizzleComponent::X,
                        _ => crate::SwizzleComponent::W,
                    };
                    if component > max_component {
                        return Err(ExpressionError::InvalidGatherComponent(component));
                    }
                    match level {
                        crate::SampleLevel::Zero => {}
                        _ => return Err(ExpressionError::InvalidGatherLevel),
                    }
                }

                // Clamping coordinate to edge is only supported with 2d non-arrayed, sampled images
                // when sampling from level Zero without any offset, gather, or depth comparison.
                if clamp_to_edge {
                    if !matches!(
                        class,
                        crate::ImageClass::Sampled {
                            kind: crate::ScalarKind::Float,
                            multi: false
                        } | crate::ImageClass::External
                    ) {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            alloc::format!("image class `{class:?}`"),
                        ));
                    }
                    if dim != crate::ImageDimension::D2 {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            alloc::format!("image dimension `{dim:?}`"),
                        ));
                    }
                    if gather.is_some() {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            "gather".into(),
                        ));
                    }
                    if array_index.is_some() {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            "array index".into(),
                        ));
                    }
                    if offset.is_some() {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            "offset".into(),
                        ));
                    }
                    if level != crate::SampleLevel::Zero {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            "non-zero level".into(),
                        ));
                    }
                    if depth_ref.is_some() {
                        return Err(ExpressionError::InvalidSampleClampCoordinateToEdge(
                            "depth comparison".into(),
                        ));
                    }
                }

                // External textures can only be sampled using clamp_to_edge.
                if matches!(class, crate::ImageClass::External) && !clamp_to_edge {
                    return Err(ExpressionError::InvalidImageClass(class));
                }

                // check level properties
                match level {
                    crate::SampleLevel::Auto => ShaderStages::FRAGMENT,
                    crate::SampleLevel::Zero => ShaderStages::all(),
                    crate::SampleLevel::Exact(expr) => {
                        match class {
                            crate::ImageClass::Depth { .. } => match resolver[expr] {
                                Ti::Scalar(Sc {
                                    kind: Sk::Sint | Sk::Uint,
                                    ..
                                }) => {}
                                _ => {
                                    return Err(ExpressionError::InvalidSampleLevelExactType(expr))
                                }
                            },
                            _ => match resolver[expr] {
                                Ti::Scalar(Sc {
                                    kind: Sk::Float, ..
                                }) => {}
                                _ => {
                                    return Err(ExpressionError::InvalidSampleLevelExactType(expr))
                                }
                            },
                        }
                        ShaderStages::all()
                    }
                    crate::SampleLevel::Bias(expr) => {
                        match resolver[expr] {
                            Ti::Scalar(Sc {
                                kind: Sk::Float, ..
                            }) => {}
                            _ => return Err(ExpressionError::InvalidSampleLevelBiasType(expr)),
                        }
                        match class {
                            crate::ImageClass::Sampled {
                                kind: Sk::Float,
                                multi: false,
                            } => {
                                if dim == crate::ImageDimension::D1 {
                                    return Err(ExpressionError::InvalidSampleLevelBiasDimension(
                                        dim,
                                    ));
                                }
                            }
                            _ => return Err(ExpressionError::InvalidImageClass(class)),
                        }
                        ShaderStages::FRAGMENT
                    }
                    crate::SampleLevel::Gradient { x, y } => {
                        match resolver[x] {
                            Ti::Scalar(Sc {
                                kind: Sk::Float, ..
                            }) if num_components == 1 => {}
                            Ti::Vector {
                                size,
                                scalar:
                                    Sc {
                                        kind: Sk::Float, ..
                                    },
                            } if size as u32 == num_components => {}
                            _ => {
                                return Err(ExpressionError::InvalidSampleLevelGradientType(dim, x))
                            }
                        }
                        match resolver[y] {
                            Ti::Scalar(Sc {
                                kind: Sk::Float, ..
                            }) if num_components == 1 => {}
                            Ti::Vector {
                                size,
                                scalar:
                                    Sc {
                                        kind: Sk::Float, ..
                                    },
                            } if size as u32 == num_components => {}
                            _ => {
                                return Err(ExpressionError::InvalidSampleLevelGradientType(dim, y))
                            }
                        }
                        ShaderStages::all()
                    }
                }
            }
            E::ImageLoad {
                image,
                coordinate,
                array_index,
                sample,
                level,
            } => {
                let ty = Self::global_var_ty(module, function, image)?;
                let Ti::Image {
                    class,
                    arrayed,
                    dim,
                } = module.types[ty].inner
                else {
                    return Err(ExpressionError::ExpectedImageType(ty));
                };

                match resolver[coordinate].image_storage_coordinates() {
                    Some(coord_dim) if coord_dim == dim => {}
                    _ => return Err(ExpressionError::InvalidImageCoordinateType(dim, coordinate)),
                };
                if arrayed != array_index.is_some() {
                    return Err(ExpressionError::InvalidImageArrayIndex);
                }
                if let Some(expr) = array_index {
                    if !matches!(resolver[expr], Ti::Scalar(Sc::I32 | Sc::U32)) {
                        return Err(ExpressionError::InvalidImageArrayIndexType(expr));
                    }
                }

                match (sample, class.is_multisampled()) {
                    (None, false) => {}
                    (Some(sample), true) => {
                        if !matches!(resolver[sample], Ti::Scalar(Sc::I32 | Sc::U32)) {
                            return Err(ExpressionError::InvalidImageOtherIndexType(sample));
                        }
                    }
                    (Some(_), false) => {
                        return Err(ExpressionError::InvalidImageSampleSelector);
                    }
                    (None, true) => {
                        return Err(ExpressionError::MissingImageSampleSelector);
                    }
                }

                match (level, class.is_mipmapped()) {
                    (None, false) => {}
                    (Some(level), true) => match resolver[level] {
                        Ti::Scalar(Sc {
                            kind: Sk::Sint | Sk::Uint,
                            width: _,
                        }) => {}
                        _ => return Err(ExpressionError::InvalidImageArrayIndexType(level)),
                    },
                    (Some(_), false) => {
                        return Err(ExpressionError::InvalidImageLevelSelector);
                    }
                    (None, true) => {
                        return Err(ExpressionError::MissingImageLevelSelector);
                    }
                }
                ShaderStages::all()
            }
            E::ImageQuery { image, query } => {
                let ty = Self::global_var_ty(module, function, image)?;
                match module.types[ty].inner {
                    Ti::Image { class, arrayed, .. } => {
                        let good = match query {
                            crate::ImageQuery::NumLayers => arrayed,
                            crate::ImageQuery::Size { level: None } => true,
                            crate::ImageQuery::Size { level: Some(level) } => {
                                match resolver[level] {
                                    Ti::Scalar(Sc::I32 | Sc::U32) => {}
                                    _ => {
                                        return Err(ExpressionError::InvalidImageOtherIndexType(
                                            level,
                                        ))
                                    }
                                }
                                class.is_mipmapped()
                            }
                            crate::ImageQuery::NumLevels => class.is_mipmapped(),
                            crate::ImageQuery::NumSamples => class.is_multisampled(),
                        };
                        if !good {
                            return Err(ExpressionError::InvalidImageClass(class));
                        }
                    }
                    _ => return Err(ExpressionError::ExpectedImageType(ty)),
                }
                ShaderStages::all()
            }
            E::Unary { op, expr } => {
                use crate::UnaryOperator as Uo;
                let Some((_, scalar)) = resolver[expr].vector_size_and_scalar() else {
                    return Err(ExpressionError::InvalidUnaryOperandType(op, expr));
                };
                match (op, scalar.kind) {
                    (Uo::Negate, Sk::Float | Sk::Sint) => {}
                    (Uo::LogicalNot, Sk::Bool) => {}
                    (Uo::BitwiseNot, Sk::Sint | Sk::Uint) => {}
                    _ => return Err(ExpressionError::InvalidUnaryOperandType(op, expr)),
                }
                ShaderStages::all()
            }
            E::Binary { op, left, right } => {
                use crate::BinaryOperator as Bo;
                let left_inner = &resolver[left];
                let right_inner = &resolver[right];
                let good = match op {
                    Bo::Add | Bo::Subtract => match *left_inner {
                        Ti::Scalar(scalar) | Ti::Vector { scalar, .. } => match scalar.kind {
                            Sk::Uint | Sk::Sint | Sk::Float => left_inner == right_inner,
                            Sk::Bool | Sk::AbstractInt | Sk::AbstractFloat => false,
                        },
                        Ti::Matrix { .. } | Ti::CooperativeMatrix { .. } => {
                            left_inner == right_inner
                        }
                        _ => false,
                    },
                    Bo::Divide | Bo::Modulo => match *left_inner {
                        Ti::Scalar(scalar) | Ti::Vector { scalar, .. } => match scalar.kind {
                            Sk::Uint | Sk::Sint | Sk::Float => left_inner == right_inner,
                            Sk::Bool | Sk::AbstractInt | Sk::AbstractFloat => false,
                        },
                        _ => false,
                    },
                    Bo::Multiply => {
                        let kind_allowed = match left_inner.scalar_kind() {
                            Some(Sk::Uint | Sk::Sint | Sk::Float) => true,
                            Some(Sk::Bool | Sk::AbstractInt | Sk::AbstractFloat) | None => false,
                        };
                        let types_match = match (left_inner, right_inner) {
                            // Straight scalar and mixed scalar/vector.
                            (&Ti::Scalar(scalar1), &Ti::Scalar(scalar2))
                            | (
                                &Ti::Vector {
                                    scalar: scalar1, ..
                                },
                                &Ti::Scalar(scalar2),
                            )
                            | (
                                &Ti::Scalar(scalar1),
                                &Ti::Vector {
                                    scalar: scalar2, ..
                                },
                            ) => scalar1 == scalar2,
                            // Scalar * matrix.
                            (
                                &Ti::Scalar(Sc {
                                    kind: Sk::Float, ..
                                }),
                                &Ti::Matrix { .. },
                            )
                            | (
                                &Ti::Matrix { .. },
                                &Ti::Scalar(Sc {
                                    kind: Sk::Float, ..
                                }),
                            ) => true,
                            // Vector * vector.
                            (
                                &Ti::Vector {
                                    size: size1,
                                    scalar: scalar1,
                                },
                                &Ti::Vector {
                                    size: size2,
                                    scalar: scalar2,
                                },
                            ) => scalar1 == scalar2 && size1 == size2,
                            // Matrix * vector.
                            (
                                &Ti::Matrix { columns, .. },
                                &Ti::Vector {
                                    size,
                                    scalar:
                                        Sc {
                                            kind: Sk::Float, ..
                                        },
                                },
                            ) => columns == size,
                            // Vector * matrix.
                            (
                                &Ti::Vector {
                                    size,
                                    scalar:
                                        Sc {
                                            kind: Sk::Float, ..
                                        },
                                },
                                &Ti::Matrix { rows, .. },
                            ) => size == rows,
                            // Matrix * matrix.
                            (&Ti::Matrix { columns, .. }, &Ti::Matrix { rows, .. }) => {
                                columns == rows
                            }
                            // Scalar * coop matrix.
                            (&Ti::Scalar(s1), &Ti::CooperativeMatrix { scalar: s2, .. })
                            | (&Ti::CooperativeMatrix { scalar: s1, .. }, &Ti::Scalar(s2)) => {
                                s1 == s2
                            }
                            _ => false,
                        };
                        let left_width = left_inner.scalar_width().unwrap_or(0);
                        let right_width = right_inner.scalar_width().unwrap_or(0);
                        kind_allowed && types_match && left_width == right_width
                    }
                    Bo::Equal | Bo::NotEqual => left_inner.is_sized() && left_inner == right_inner,
                    Bo::Less | Bo::LessEqual | Bo::Greater | Bo::GreaterEqual => {
                        match *left_inner {
                            Ti::Scalar(scalar) | Ti::Vector { scalar, .. } => match scalar.kind {
                                Sk::Uint | Sk::Sint | Sk::Float => left_inner == right_inner,
                                Sk::Bool | Sk::AbstractInt | Sk::AbstractFloat => false,
                            },
                            ref other => {
                                log::debug!("Op {op:?} left type {other:?}");
                                false
                            }
                        }
                    }
                    Bo::LogicalAnd | Bo::LogicalOr => match *left_inner {
                        Ti::Scalar(Sc { kind: Sk::Bool, .. })
                        | Ti::Vector {
                            scalar: Sc { kind: Sk::Bool, .. },
                            ..
                        } => left_inner == right_inner,
                        ref other => {
                            log::debug!("Op {op:?} left type {other:?}");
                            false
                        }
                    },
                    Bo::And | Bo::InclusiveOr => match *left_inner {
                        Ti::Scalar(scalar) | Ti::Vector { scalar, .. } => match scalar.kind {
                            Sk::Bool | Sk::Sint | Sk::Uint => left_inner == right_inner,
                            Sk::Float | Sk::AbstractInt | Sk::AbstractFloat => false,
                        },
                        ref other => {
                            log::debug!("Op {op:?} left type {other:?}");
                            false
                        }
                    },
                    Bo::ExclusiveOr => match *left_inner {
                        Ti::Scalar(scalar) | Ti::Vector { scalar, .. } => match scalar.kind {
                            Sk::Sint | Sk::Uint => left_inner == right_inner,
                            Sk::Bool | Sk::Float | Sk::AbstractInt | Sk::AbstractFloat => false,
                        },
                        ref other => {
                            log::debug!("Op {op:?} left type {other:?}");
                            false
                        }
                    },
                    Bo::ShiftLeft | Bo::ShiftRight => {
                        let (base_size, base_scalar) = match *left_inner {
                            Ti::Scalar(scalar) => (Ok(None), scalar),
                            Ti::Vector { size, scalar } => (Ok(Some(size)), scalar),
                            ref other => {
                                log::debug!("Op {op:?} base type {other:?}");
                                (Err(()), Sc::BOOL)
                            }
                        };
                        let shift_size = match *right_inner {
                            Ti::Scalar(Sc { kind: Sk::Uint, .. }) => Ok(None),
                            Ti::Vector {
                                size,
                                scalar: Sc { kind: Sk::Uint, .. },
                            } => Ok(Some(size)),
                            ref other => {
                                log::debug!("Op {op:?} shift type {other:?}");
                                Err(())
                            }
                        };
                        match base_scalar.kind {
                            Sk::Sint | Sk::Uint => base_size.is_ok() && base_size == shift_size,
                            Sk::Float | Sk::AbstractInt | Sk::AbstractFloat | Sk::Bool => false,
                        }
                    }
                };
                if !good {
                    log::debug!(
                        "Left: {:?} of type {:?}",
                        function.expressions[left],
                        left_inner
                    );
                    log::debug!(
                        "Right: {:?} of type {:?}",
                        function.expressions[right],
                        right_inner
                    );
                    return Err(ExpressionError::InvalidBinaryOperandTypes {
                        op,
                        lhs_expr: left,
                        lhs_type: left_inner.clone(),
                        rhs_expr: right,
                        rhs_type: right_inner.clone(),
                    });
                }
                // For shift operations, check if the constant shift amount exceeds the bit width
                if matches!(op, Bo::ShiftLeft | Bo::ShiftRight) {
                    Self::validate_constant_shift_amounts(left_inner, right, module, function)?;
                }
                ShaderStages::all()
            }
            E::Select {
                condition,
                accept,
                reject,
            } => {
                let accept_inner = &resolver[accept];
                let reject_inner = &resolver[reject];
                let condition_ty = &resolver[condition];
                let condition_good = match *condition_ty {
                    Ti::Scalar(Sc {
                        kind: Sk::Bool,
                        width: _,
                    }) => {
                        // When `condition` is a single boolean, `accept` and
                        // `reject` can be vectors or scalars.
                        match *accept_inner {
                            Ti::Scalar { .. } | Ti::Vector { .. } => true,
                            _ => false,
                        }
                    }
                    Ti::Vector {
                        size,
                        scalar:
                            Sc {
                                kind: Sk::Bool,
                                width: _,
                            },
                    } => match *accept_inner {
                        Ti::Vector {
                            size: other_size, ..
                        } => size == other_size,
                        _ => false,
                    },
                    _ => false,
                };
                if accept_inner != reject_inner {
                    return Err(ExpressionError::SelectValuesTypeMismatch {
                        accept: accept_inner.clone(),
                        reject: reject_inner.clone(),
                    });
                }
                if !condition_good {
                    return Err(ExpressionError::SelectConditionNotABool {
                        actual: condition_ty.clone(),
                    });
                }
                ShaderStages::all()
            }
            E::Derivative { expr, .. } => {
                match resolver[expr] {
                    Ti::Scalar(Sc {
                        kind: Sk::Float, ..
                    })
                    | Ti::Vector {
                        scalar:
                            Sc {
                                kind: Sk::Float, ..
                            },
                        ..
                    } => {}
                    _ => return Err(ExpressionError::InvalidDerivative),
                }
                ShaderStages::FRAGMENT
            }
            E::Relational { fun, argument } => {
                use crate::RelationalFunction as Rf;
                let argument_inner = &resolver[argument];
                match fun {
                    Rf::All | Rf::Any => match *argument_inner {
                        Ti::Vector {
                            scalar: Sc { kind: Sk::Bool, .. },
                            ..
                        } => {}
                        ref other => {
                            log::debug!("All/Any of type {other:?}");
                            return Err(ExpressionError::InvalidBooleanVector(argument));
                        }
                    },
                    Rf::IsNan | Rf::IsInf => match *argument_inner {
                        Ti::Scalar(scalar) | Ti::Vector { scalar, .. }
                            if scalar.kind == Sk::Float => {}
                        ref other => {
                            log::debug!("Float test of type {other:?}");
                            return Err(ExpressionError::InvalidFloatArgument(argument));
                        }
                    },
                }
                ShaderStages::all()
            }
            E::Math {
                fun,
                arg,
                arg1,
                arg2,
                arg3,
            } => {
                if matches!(
                    fun,
                    crate::MathFunction::QuantizeToF16
                        | crate::MathFunction::Pack2x16float
                        | crate::MathFunction::Unpack2x16float
                ) && !self
                    .capabilities
                    .contains(crate::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32)
                {
                    return Err(ExpressionError::MissingCapabilities(
                        crate::valid::Capabilities::SHADER_FLOAT16_IN_FLOAT32,
                    ));
                }

                let actuals: &[_] = match (arg1, arg2, arg3) {
                    (None, None, None) => &[arg],
                    (Some(arg1), None, None) => &[arg, arg1],
                    (Some(arg1), Some(arg2), None) => &[arg, arg1, arg2],
                    (Some(arg1), Some(arg2), Some(arg3)) => &[arg, arg1, arg2, arg3],
                    _ => return Err(ExpressionError::WrongArgumentCount(fun)),
                };

                let resolve = |arg| &resolver[arg];
                let actual_types: &[_] = match *actuals {
                    [arg0] => &[resolve(arg0)],
                    [arg0, arg1] => &[resolve(arg0), resolve(arg1)],
                    [arg0, arg1, arg2] => &[resolve(arg0), resolve(arg1), resolve(arg2)],
                    [arg0, arg1, arg2, arg3] => {
                        &[resolve(arg0), resolve(arg1), resolve(arg2), resolve(arg3)]
                    }
                    _ => unreachable!(),
                };

                // Start with the set of all overloads available for `fun`.
                let mut overloads = fun.overloads();
                log::debug!(
                    "initial overloads for {:?}: {:#?}",
                    fun,
                    overloads.for_debug(&module.types)
                );

                // If any argument is not a constant expression, then no
                // overloads that accept abstract values should be considered.
                // `OverloadSet::concrete_only` is supposed to help impose this
                // restriction. However, no `MathFunction` accepts a mix of
                // abstract and concrete arguments, so we don't need to worry
                // about that here.

                for (i, (&expr, &ty)) in actuals.iter().zip(actual_types).enumerate() {
                    // Remove overloads that cannot accept an `i`'th
                    // argument arguments of type `ty`.
                    overloads = overloads.arg(i, ty, &module.types);
                    log::debug!(
                        "overloads after arg {i}: {:#?}",
                        overloads.for_debug(&module.types)
                    );

                    if overloads.is_empty() {
                        log::debug!("all overloads eliminated");
                        return Err(ExpressionError::InvalidArgumentType(fun, i as u32, expr));
                    }
                }

                if actuals.len() < overloads.min_arguments() {
                    return Err(ExpressionError::WrongArgumentCount(fun));
                }

                ShaderStages::all()
            }
            E::As {
                expr,
                kind,
                convert,
            } => {
                let mut base_scalar = match resolver[expr] {
                    crate::TypeInner::Scalar(scalar) | crate::TypeInner::Vector { scalar, .. } => {
                        scalar
                    }
                    crate::TypeInner::Matrix { scalar, .. } => scalar,
                    _ => return Err(ExpressionError::InvalidCastArgument),
                };
                base_scalar.kind = kind;
                if let Some(width) = convert {
                    base_scalar.width = width;
                }
                if self.check_width(base_scalar).is_err() {
                    return Err(ExpressionError::InvalidCastArgument);
                }
                ShaderStages::all()
            }
            E::CallResult(function) => mod_info.functions[function.index()].available_stages,
            E::AtomicResult { .. } => {
                // These expressions are validated when we check the `Atomic` statement
                // that refers to them, because we have all the information we need at
                // that point. The checks driven by `Validator::needs_visit` ensure
                // that this expression is indeed visited by one `Atomic` statement.
                ShaderStages::all()
            }
            E::WorkGroupUniformLoadResult { ty } => {
                if self.types[ty.index()]
                    .flags
                    // Sized | Constructible is exactly the types currently supported by
                    // WorkGroupUniformLoad
                    .contains(TypeFlags::SIZED | TypeFlags::CONSTRUCTIBLE)
                {
                    ShaderStages::COMPUTE_LIKE
                } else {
                    return Err(ExpressionError::InvalidWorkGroupUniformLoadResultType(ty));
                }
            }
            E::ArrayLength(expr) => match resolver[expr] {
                Ti::Pointer { base, .. } => {
                    let base_ty = &resolver.types[base];
                    if let Ti::Array {
                        size: crate::ArraySize::Dynamic,
                        ..
                    } = base_ty.inner
                    {
                        ShaderStages::all()
                    } else {
                        return Err(ExpressionError::InvalidArrayType(expr));
                    }
                }
                ref other => {
                    log::debug!("Array length of {other:?}");
                    return Err(ExpressionError::InvalidArrayType(expr));
                }
            },
            E::RayQueryProceedResult => ShaderStages::all(),
            E::RayQueryGetIntersection {
                query,
                committed: _,
            } => match resolver[query] {
                Ti::Pointer {
                    base,
                    space: crate::AddressSpace::Function,
                } => match resolver.types[base].inner {
                    Ti::RayQuery { .. } => ShaderStages::all(),
                    ref other => {
                        log::debug!("Intersection result of a pointer to {other:?}");
                        return Err(ExpressionError::InvalidRayQueryType(query));
                    }
                },
                ref other => {
                    log::debug!("Intersection result of {other:?}");
                    return Err(ExpressionError::InvalidRayQueryType(query));
                }
            },
            E::RayQueryVertexPositions {
                query,
                committed: _,
            } => match resolver[query] {
                Ti::Pointer {
                    base,
                    space: crate::AddressSpace::Function,
                } => match resolver.types[base].inner {
                    Ti::RayQuery {
                        vertex_return: true,
                    } => ShaderStages::all(),
                    ref other => {
                        log::debug!("Intersection result of a pointer to {other:?}");
                        return Err(ExpressionError::InvalidRayQueryType(query));
                    }
                },
                ref other => {
                    log::debug!("Intersection result of {other:?}");
                    return Err(ExpressionError::InvalidRayQueryType(query));
                }
            },
            E::SubgroupBallotResult | E::SubgroupOperationResult { .. } => self.subgroup_stages,
            E::CooperativeLoad { ref data, .. } => {
                if resolver[data.pointer]
                    .pointer_base_type()
                    .and_then(|tr| tr.inner_with(&module.types).scalar())
                    .is_none()
                {
                    return Err(ExpressionError::InvalidPointerType(data.pointer));
                }
                ShaderStages::COMPUTE
            }
            E::CooperativeMultiplyAdd { a, b, c } => {
                let roles = [
                    crate::CooperativeRole::A,
                    crate::CooperativeRole::B,
                    crate::CooperativeRole::C,
                ];
                for (operand, expected_role) in [a, b, c].into_iter().zip(roles) {
                    match resolver[operand] {
                        Ti::CooperativeMatrix { role, .. } if role == expected_role => {}
                        ref other => {
                            log::debug!("{expected_role:?} operand type: {other:?}");
                            return Err(ExpressionError::InvalidCooperativeOperand(a));
                        }
                    }
                }
                ShaderStages::COMPUTE
            }
        };
        Ok(stages)
    }

    fn global_var_ty(
        module: &crate::Module,
        function: &crate::Function,
        expr: Handle<crate::Expression>,
    ) -> Result<Handle<crate::Type>, ExpressionError> {
        use crate::Expression as Ex;

        match function.expressions[expr] {
            Ex::GlobalVariable(var_handle) => Ok(module.global_variables[var_handle].ty),
            Ex::FunctionArgument(i) => Ok(function.arguments[i as usize].ty),
            Ex::Access { base, .. } | Ex::AccessIndex { base, .. } => {
                match function.expressions[base] {
                    Ex::GlobalVariable(var_handle) => {
                        let array_ty = module.global_variables[var_handle].ty;

                        match module.types[array_ty].inner {
                            crate::TypeInner::BindingArray { base, .. } => Ok(base),
                            _ => Err(ExpressionError::ExpectedBindingArrayType(array_ty)),
                        }
                    }
                    _ => Err(ExpressionError::ExpectedGlobalVariable),
                }
            }
            _ => Err(ExpressionError::ExpectedGlobalVariable),
        }
    }

    pub fn validate_literal(&self, literal: crate::Literal) -> Result<(), LiteralError> {
        let _ = self.check_width(literal.scalar())?;
        check_literal_value(literal)?;

        Ok(())
    }
}

pub const fn check_literal_value(literal: crate::Literal) -> Result<(), LiteralError> {
    let is_nan = match literal {
        crate::Literal::F64(v) => v.is_nan(),
        crate::Literal::F32(v) => v.is_nan(),
        _ => false,
    };
    if is_nan {
        return Err(LiteralError::NaN);
    }

    let is_infinite = match literal {
        crate::Literal::F64(v) => v.is_infinite(),
        crate::Literal::F32(v) => v.is_infinite(),
        _ => false,
    };
    if is_infinite {
        return Err(LiteralError::Infinity);
    }

    Ok(())
}

#[cfg(test)]
/// Validate a module containing the given expression, expecting an error.
fn validate_with_expression(
    expr: crate::Expression,
    caps: super::Capabilities,
) -> Result<ModuleInfo, crate::span::WithSpan<super::ValidationError>> {
    use crate::span::Span;

    let mut function = crate::Function::default();
    function.expressions.append(expr, Span::default());
    function.body.push(
        crate::Statement::Emit(function.expressions.range_from(0)),
        Span::default(),
    );

    let mut module = crate::Module::default();
    module.functions.append(function, Span::default());

    let mut validator = super::Validator::new(super::ValidationFlags::EXPRESSIONS, caps);

    validator.validate(&module)
}

#[cfg(test)]
/// Validate a module containing the given constant expression, expecting an error.
fn validate_with_const_expression(
    expr: crate::Expression,
    caps: super::Capabilities,
) -> Result<ModuleInfo, crate::span::WithSpan<super::ValidationError>> {
    use crate::span::Span;

    let mut module = crate::Module::default();
    module.global_expressions.append(expr, Span::default());

    let mut validator = super::Validator::new(super::ValidationFlags::CONSTANTS, caps);

    validator.validate(&module)
}

/// Using F64 in a function's expression arena is forbidden.
#[test]
fn f64_runtime_literals() {
    let result = validate_with_expression(
        crate::Expression::Literal(crate::Literal::F64(0.57721_56649)),
        super::Capabilities::default(),
    );
    let error = result.unwrap_err().into_inner();
    assert!(matches!(
        error,
        crate::valid::ValidationError::Function {
            source: super::FunctionError::Expression {
                source: ExpressionError::Literal(LiteralError::Width(
                    super::r#type::WidthError::MissingCapability {
                        name: "f64",
                        flag: "FLOAT64",
                    }
                ),),
                ..
            },
            ..
        }
    ));

    let result = validate_with_expression(
        crate::Expression::Literal(crate::Literal::F64(0.57721_56649)),
        super::Capabilities::default() | super::Capabilities::FLOAT64,
    );
    assert!(result.is_ok());
}

/// Using F64 in a module's constant expression arena is forbidden.
#[test]
fn f64_const_literals() {
    let result = validate_with_const_expression(
        crate::Expression::Literal(crate::Literal::F64(0.57721_56649)),
        super::Capabilities::default(),
    );
    let error = result.unwrap_err().into_inner();
    assert!(matches!(
        error,
        crate::valid::ValidationError::ConstExpression {
            source: ConstExpressionError::Literal(LiteralError::Width(
                super::r#type::WidthError::MissingCapability {
                    name: "f64",
                    flag: "FLOAT64",
                }
            )),
            ..
        }
    ));

    let result = validate_with_const_expression(
        crate::Expression::Literal(crate::Literal::F64(0.57721_56649)),
        super::Capabilities::default() | super::Capabilities::FLOAT64,
    );
    assert!(result.is_ok());
}
