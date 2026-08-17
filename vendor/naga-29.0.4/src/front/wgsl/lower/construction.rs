use alloc::{boxed::Box, vec, vec::Vec};
use core::num::NonZeroU32;

use crate::common::wgsl::{TryToWgsl, TypeContext};
use crate::front::wgsl::lower::{ExpressionContext, Lowerer};
use crate::front::wgsl::parse::ast;
use crate::front::wgsl::{Error, Result};
use crate::{Handle, Span};

/// A [`constructor built-in function`].
///
/// WGSL has two types of such functions:
///
/// - Those that fully specify the type being constructed, like
///   `vec3<f32>(x,y,z)`, which obviously constructs a `vec3<f32>`.
///
/// - Those that leave the component type of the composite being constructed
///   implicit, to be inferred from the argument types, like `vec3(x,y,z)`,
///   which constructs a `vec3<T>` where `T` is the type of `x`, `y`, and `z`.
///
/// This enum represents both cases. The `PartialFoo` variants
/// represent the second case, where the component type is implicit.
///
/// [`constructor built-in function`]: https://gpuweb.github.io/gpuweb/wgsl/#constructor-builtin-function
pub enum Constructor<T> {
    /// A vector construction whose component type is inferred from the
    /// argument: `vec3(1.0)`.
    PartialVector { size: crate::VectorSize },

    /// A matrix construction whose component type is inferred from the
    /// argument: `mat2x2(1,2,3,4)`.
    PartialMatrix {
        columns: crate::VectorSize,
        rows: crate::VectorSize,
    },

    /// An array whose component type and size are inferred from the arguments:
    /// `array(3,4,5)`.
    PartialArray,

    /// A known Naga type.
    ///
    /// When we match on this type, we need to see the `TypeInner` here, but at
    /// the point that we build this value we'll still need mutable access to
    /// the module later. To avoid borrowing from the module, the type parameter
    /// `T` is `Handle<Type>` initially. Then we use `borrow_inner` to produce a
    /// version holding a tuple `(Handle<Type>, &TypeInner)`.
    Type(T),
}

impl Constructor<Handle<crate::Type>> {
    /// Return an equivalent `Constructor` value that includes borrowed
    /// `TypeInner` values alongside any type handles.
    ///
    /// The returned form is more convenient to match on, since the patterns
    /// can actually see what the handle refers to.
    fn borrow_inner(
        self,
        module: &crate::Module,
    ) -> Constructor<(Handle<crate::Type>, &crate::TypeInner)> {
        match self {
            Constructor::PartialVector { size } => Constructor::PartialVector { size },
            Constructor::PartialMatrix { columns, rows } => {
                Constructor::PartialMatrix { columns, rows }
            }
            Constructor::PartialArray => Constructor::PartialArray,
            Constructor::Type(handle) => Constructor::Type((handle, &module.types[handle].inner)),
        }
    }
}

enum Components<'a> {
    None,
    One {
        component: Handle<crate::Expression>,
        span: Span,
        ty_inner: &'a crate::TypeInner,
    },
    Many {
        components: Vec<Handle<crate::Expression>>,
        spans: Vec<Span>,
    },
}

impl Components<'_> {
    fn into_components_vec(self) -> Vec<Handle<crate::Expression>> {
        match self {
            Self::None => vec![],
            Self::One { component, .. } => vec![component],
            Self::Many { components, .. } => components,
        }
    }
}

impl<'source> Lowerer<'source, '_> {
    /// Generate Naga IR for a type constructor expression.
    ///
    /// The `constructor` value represents the head of the constructor
    /// expression, which is at least a hint of which type is being built; if
    /// it's one of the `Partial` variants, we need to consider the argument
    /// types as well.
    ///
    /// This is used for [`Call`] expressions, once we've determined that
    /// the "callable" (in WGSL spec terms) is actually a type.
    ///
    /// [`Call`]: ast::Expression::Call
    pub fn construct(
        &mut self,
        span: Span,
        constructor: Constructor<Handle<crate::Type>>,
        ty_span: Span,
        components: &[Handle<ast::Expression<'source>>],
        ctx: &mut ExpressionContext<'source, '_, '_>,
    ) -> Result<'source, Handle<crate::Expression>> {
        use crate::proc::TypeResolution as Tr;

        let components = match *components {
            [] => Components::None,
            [component] => {
                let span = ctx.ast_expressions.get_span(component);
                let component = self.expression_for_abstract(component, ctx)?;
                let ty_inner = super::resolve_inner!(ctx, component);

                Components::One {
                    component,
                    span,
                    ty_inner,
                }
            }
            ref ast_components @ [_, _, ..] => {
                let components = ast_components
                    .iter()
                    .map(|&expr| self.expression_for_abstract(expr, ctx))
                    .collect::<Result<_>>()?;
                let spans = ast_components
                    .iter()
                    .map(|&expr| ctx.ast_expressions.get_span(expr))
                    .collect();

                for &component in &components {
                    ctx.grow_types(component)?;
                }

                Components::Many { components, spans }
            }
        };

        // Even though we computed `constructor` above, wait until now to borrow
        // a reference to the `TypeInner`, so that the component-handling code
        // above can have mutable access to the type arena.
        let constructor = constructor.borrow_inner(ctx.module);

        let expr;
        match (components, constructor) {
            // Zero-value constructor with explicit type.
            (Components::None, Constructor::Type((result_ty, inner)))
                if inner.is_constructible(&ctx.module.types) =>
            {
                expr = crate::Expression::ZeroValue(result_ty);
            }
            // Zero-value constructor, vector with type inference
            (Components::None, Constructor::PartialVector { size }) => {
                // vec2(), vec3(), vec4() return vectors of abstractInts; the same
                // is not true of the similar constructors for matrices or arrays.
                // See https://www.w3.org/TR/WGSL/#vec2-builtin et seq.
                let result_ty = ctx.module.types.insert(
                    crate::Type {
                        name: None,
                        inner: crate::TypeInner::Vector {
                            size,
                            scalar: crate::Scalar::ABSTRACT_INT,
                        },
                    },
                    span,
                );
                expr = crate::Expression::ZeroValue(result_ty);
            }
            // Zero-value constructor, matrix or array with type inference
            (Components::None, Constructor::PartialMatrix { .. } | Constructor::PartialArray) => {
                // We have no arguments from which to infer the result type, so
                // partial constructors aren't acceptable here.
                return Err(Box::new(Error::TypeNotInferable(ty_span)));
            }

            // Scalar constructor & conversion (scalar -> scalar)
            (
                Components::One {
                    component,
                    ty_inner: &crate::TypeInner::Scalar { .. },
                    ..
                },
                Constructor::Type((_, &crate::TypeInner::Scalar(scalar))),
            ) => {
                expr = crate::Expression::As {
                    expr: component,
                    kind: scalar.kind,
                    convert: Some(scalar.width),
                };
            }

            // Vector conversion (vector -> vector)
            (
                Components::One {
                    component,
                    ty_inner: &crate::TypeInner::Vector { size: src_size, .. },
                    ..
                },
                Constructor::Type((
                    _,
                    &crate::TypeInner::Vector {
                        size: dst_size,
                        scalar: dst_scalar,
                    },
                )),
            ) if dst_size == src_size => {
                expr = crate::Expression::As {
                    expr: component,
                    kind: dst_scalar.kind,
                    convert: Some(dst_scalar.width),
                };
            }

            // Vector conversion (vector -> vector) - partial
            (
                Components::One {
                    component,
                    ty_inner: &crate::TypeInner::Vector { size: src_size, .. },
                    ..
                },
                Constructor::PartialVector { size: dst_size },
            ) if dst_size == src_size => {
                // This is a trivial conversion: the sizes match, and a Partial
                // constructor doesn't specify a scalar type, so nothing can
                // possibly happen.
                return Ok(component);
            }

            // Matrix conversion (matrix -> matrix)
            (
                Components::One {
                    component,
                    ty_inner:
                        &crate::TypeInner::Matrix {
                            columns: src_columns,
                            rows: src_rows,
                            ..
                        },
                    ..
                },
                Constructor::Type((
                    _,
                    &crate::TypeInner::Matrix {
                        columns: dst_columns,
                        rows: dst_rows,
                        scalar: dst_scalar,
                    },
                )),
            ) if dst_columns == src_columns && dst_rows == src_rows => {
                expr = crate::Expression::As {
                    expr: component,
                    kind: dst_scalar.kind,
                    convert: Some(dst_scalar.width),
                };
            }

            // Matrix conversion (matrix -> matrix) - partial
            (
                Components::One {
                    component,
                    ty_inner:
                        &crate::TypeInner::Matrix {
                            columns: src_columns,
                            rows: src_rows,
                            ..
                        },
                    ..
                },
                Constructor::PartialMatrix {
                    columns: dst_columns,
                    rows: dst_rows,
                },
            ) if dst_columns == src_columns && dst_rows == src_rows => {
                // This is a trivial conversion: the sizes match, and a Partial
                // constructor doesn't specify a scalar type, so nothing can
                // possibly happen.
                return Ok(component);
            }

            // Vector constructor (splat) - infer type
            (
                Components::One {
                    component,
                    ty_inner: &crate::TypeInner::Scalar { .. },
                    ..
                },
                Constructor::PartialVector { size },
            ) => {
                expr = crate::Expression::Splat {
                    size,
                    value: component,
                };
            }

            // Vector constructor (splat)
            (
                Components::One {
                    mut component,
                    ty_inner: &crate::TypeInner::Scalar(component_scalar),
                    span,
                },
                Constructor::Type((
                    type_handle,
                    &crate::TypeInner::Vector {
                        size,
                        scalar: vec_scalar,
                    },
                )),
            ) => {
                // Splat only allows automatic conversions of the component's scalar.
                if !component_scalar.automatically_converts_to(vec_scalar) {
                    let component_ty = &ctx.typifier()[component];
                    let arg_ty = ctx.type_resolution_to_string(component_ty);
                    return Err(Box::new(Error::WrongArgumentType {
                        function: ctx.type_to_string(type_handle),
                        call_span: ty_span,
                        arg_span: span,
                        arg_index: 0,
                        arg_ty,
                        allowed: vec![vec_scalar.to_wgsl_for_diagnostics()],
                    }));
                }
                ctx.convert_slice_to_common_leaf_scalar(
                    core::slice::from_mut(&mut component),
                    vec_scalar,
                )?;
                expr = crate::Expression::Splat {
                    size,
                    value: component,
                };
            }

            // Vector constructor (by elements), partial
            (
                Components::Many {
                    mut components,
                    spans,
                },
                Constructor::PartialVector { size },
            ) => {
                let consensus_scalar = ctx
                    .automatic_conversion_consensus(None, &components)
                    .map_err(|index| {
                        Error::InvalidConstructorComponentType(spans[index], index as i32)
                    })?;
                ctx.convert_slice_to_common_leaf_scalar(&mut components, consensus_scalar)?;
                let inner = consensus_scalar.to_inner_vector(size);
                let ty = ctx.ensure_type_exists(inner);
                expr = crate::Expression::Compose { ty, components };
            }

            // Vector constructor (by elements), full type given
            (
                Components::Many { mut components, .. },
                Constructor::Type((ty, &crate::TypeInner::Vector { scalar, .. })),
            ) => {
                ctx.try_automatic_conversions_for_vector(&mut components, scalar, ty_span)?;
                expr = crate::Expression::Compose { ty, components };
            }

            // Matrix constructor (by elements), partial
            (
                Components::Many {
                    mut components,
                    spans,
                },
                Constructor::PartialMatrix { columns, rows },
            ) if components.len() == columns as usize * rows as usize => {
                let consensus_scalar = ctx
                    .automatic_conversion_consensus(
                        Some(crate::Scalar::ABSTRACT_FLOAT),
                        &components,
                    )
                    .map_err(|index| {
                        Error::InvalidConstructorComponentType(spans[index], index as i32)
                    })?;
                ctx.convert_slice_to_common_leaf_scalar(&mut components, consensus_scalar)?;
                let vec_ty = ctx.ensure_type_exists(consensus_scalar.to_inner_vector(rows));

                let components = components
                    .chunks(rows as usize)
                    .map(|vec_components| {
                        ctx.append_expression(
                            crate::Expression::Compose {
                                ty: vec_ty,
                                components: Vec::from(vec_components),
                            },
                            Default::default(),
                        )
                    })
                    .collect::<Result<Vec<_>>>()?;

                let ty = ctx.ensure_type_exists(crate::TypeInner::Matrix {
                    columns,
                    rows,
                    scalar: consensus_scalar,
                });
                expr = crate::Expression::Compose { ty, components };
            }

            // Matrix constructor (by elements), type given
            (
                Components::Many { mut components, .. },
                Constructor::Type((
                    _,
                    &crate::TypeInner::Matrix {
                        columns,
                        rows,
                        scalar,
                    },
                )),
            ) if components.len() == columns as usize * rows as usize => {
                let element = Tr::Value(crate::TypeInner::Scalar(scalar));
                ctx.try_automatic_conversions_slice(&mut components, &element, ty_span)?;
                let vec_ty = ctx.ensure_type_exists(scalar.to_inner_vector(rows));

                let components = components
                    .chunks(rows as usize)
                    .map(|vec_components| {
                        ctx.append_expression(
                            crate::Expression::Compose {
                                ty: vec_ty,
                                components: Vec::from(vec_components),
                            },
                            Default::default(),
                        )
                    })
                    .collect::<Result<Vec<_>>>()?;

                let ty = ctx.ensure_type_exists(crate::TypeInner::Matrix {
                    columns,
                    rows,
                    scalar,
                });
                expr = crate::Expression::Compose { ty, components };
            }

            // Matrix constructor (by columns), partial
            (
                Components::Many {
                    mut components,
                    spans,
                },
                Constructor::PartialMatrix { columns, rows },
            ) if components.len() == columns as usize => {
                let consensus_scalar = ctx
                    .automatic_conversion_consensus(
                        Some(crate::Scalar::ABSTRACT_FLOAT),
                        &components,
                    )
                    .map_err(|index| {
                        Error::InvalidConstructorComponentType(spans[index], index as i32)
                    })?;

                let component_ty = crate::TypeInner::Vector {
                    size: rows,
                    scalar: consensus_scalar,
                };
                ctx.try_automatic_conversions_slice(
                    &mut components,
                    &Tr::Value(component_ty),
                    ty_span,
                )?;

                let ty = ctx.ensure_type_exists(crate::TypeInner::Matrix {
                    columns,
                    rows,
                    scalar: consensus_scalar,
                });
                expr = crate::Expression::Compose { ty, components };
            }

            // Matrix constructor (by columns), type given
            (
                Components::Many { mut components, .. },
                Constructor::Type((
                    ty,
                    &crate::TypeInner::Matrix {
                        columns,
                        rows,
                        scalar,
                    },
                )),
            ) if components.len() == columns as usize => {
                let component_ty = crate::TypeInner::Vector { size: rows, scalar };
                ctx.try_automatic_conversions_slice(
                    &mut components,
                    &Tr::Value(component_ty),
                    ty_span,
                )?;
                expr = crate::Expression::Compose { ty, components };
            }

            // Array constructor - infer type
            (components, Constructor::PartialArray) => {
                let mut components = components.into_components_vec();
                if let Ok(consensus_scalar) = ctx.automatic_conversion_consensus(None, &components)
                {
                    // Note that this will *not* necessarily convert all the
                    // components to the same type! The `automatic_conversion_consensus`
                    // method only considers the parameters' leaf scalar
                    // types; the parameters themselves could be any mix of
                    // vectors, matrices, and scalars.
                    //
                    // But *if* it is possible for this array construction
                    // expression to be well-typed at all, then all the
                    // parameters must have the same type constructors (vec,
                    // matrix, scalar) applied to their leaf scalars, so
                    // reconciling their scalars is always the right thing to
                    // do. And if this array construction is not well-typed,
                    // these conversions will not make it so, and we can let
                    // validation catch the error.
                    ctx.convert_slice_to_common_leaf_scalar(&mut components, consensus_scalar)?;
                } else {
                    // There's no consensus scalar. Emit the `Compose`
                    // expression anyway, and let validation catch the problem.
                }

                let base = ctx.register_type(components[0])?;

                let inner = crate::TypeInner::Array {
                    base,
                    size: crate::ArraySize::Constant(
                        NonZeroU32::new(u32::try_from(components.len()).unwrap()).unwrap(),
                    ),
                    stride: {
                        ctx.layouter.update(ctx.module.to_ctx()).unwrap();
                        ctx.layouter[base].to_stride()
                    },
                };
                let ty = ctx.ensure_type_exists(inner);

                expr = crate::Expression::Compose { ty, components };
            }

            // Array constructor, explicit type.
            (
                components,
                Constructor::Type((ty, inner @ &crate::TypeInner::Array { base, .. })),
            ) if inner.is_constructible(&ctx.module.types) => {
                let mut components = components.into_components_vec();
                ctx.try_automatic_conversions_slice(&mut components, &Tr::Handle(base), ty_span)?;
                expr = crate::Expression::Compose { ty, components };
            }

            // Struct constructor
            (
                components,
                Constructor::Type((ty, inner @ &crate::TypeInner::Struct { ref members, .. })),
            ) if inner.is_constructible(&ctx.module.types) => {
                let mut components = components.into_components_vec();
                let struct_ty_span = ctx.module.types.get_span(ty);

                // Make a vector of the members' type handles in advance, to
                // avoid borrowing `members` from `ctx` while we generate
                // new code.
                let members: Vec<Handle<crate::Type>> = members.iter().map(|m| m.ty).collect();

                for (component, &ty) in components.iter_mut().zip(&members) {
                    *component =
                        ctx.try_automatic_conversions(*component, &Tr::Handle(ty), struct_ty_span)?;
                }
                expr = crate::Expression::Compose { ty, components };
            }

            // ERRORS

            // Bad conversion (type cast)
            (
                Components::One {
                    span, component, ..
                },
                Constructor::Type((
                    ty,
                    &(crate::TypeInner::Scalar { .. }
                    | crate::TypeInner::Vector { .. }
                    | crate::TypeInner::Matrix { .. }),
                )),
            ) => {
                let component_ty = &ctx.typifier()[component];
                let from_type = ctx.type_resolution_to_string(component_ty);
                return Err(Box::new(Error::BadTypeCast {
                    span,
                    from_type,
                    to_type: ctx.type_to_string(ty),
                }));
            }

            // Too many parameters for scalar constructor
            (
                Components::Many { spans, .. },
                Constructor::Type((_, &crate::TypeInner::Scalar { .. })),
            ) => {
                let span = spans[1].until(spans.last().unwrap());
                return Err(Box::new(Error::UnexpectedComponents(span)));
            }

            // Other types can't be constructed
            _ => return Err(Box::new(Error::TypeNotConstructible(ty_span))),
        }

        let expr = ctx.append_expression(expr, span)?;
        Ok(expr)
    }
}
