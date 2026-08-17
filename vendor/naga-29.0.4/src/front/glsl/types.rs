use alloc::format;

use super::{context::Context, Error, ErrorKind, Result, Span};
use crate::{
    proc::ResolveContext, Expression, Handle, ImageClass, ImageDimension, Scalar, ScalarKind, Type,
    TypeInner, VectorSize,
};

pub fn parse_type(type_name: &str) -> Option<Type> {
    match type_name {
        "bool" => Some(Type {
            name: None,
            inner: TypeInner::Scalar(Scalar::BOOL),
        }),
        "float16_t" => Some(Type {
            name: None,
            inner: TypeInner::Scalar(Scalar::F16),
        }),
        "float" => Some(Type {
            name: None,
            inner: TypeInner::Scalar(Scalar::F32),
        }),
        "double" => Some(Type {
            name: None,
            inner: TypeInner::Scalar(Scalar::F64),
        }),
        "int" => Some(Type {
            name: None,
            inner: TypeInner::Scalar(Scalar::I32),
        }),
        "uint" => Some(Type {
            name: None,
            inner: TypeInner::Scalar(Scalar::U32),
        }),
        "sampler" | "samplerShadow" => Some(Type {
            name: None,
            inner: TypeInner::Sampler {
                comparison: type_name == "samplerShadow",
            },
        }),
        word => {
            fn kind_width_parse(ty: &str) -> Option<Scalar> {
                Some(match ty {
                    "" => Scalar::F32,
                    "b" => Scalar::BOOL,
                    "i" => Scalar::I32,
                    "u" => Scalar::U32,
                    "d" => Scalar::F64,
                    "f16" => Scalar::F16,
                    _ => return None,
                })
            }

            fn size_parse(n: &str) -> Option<VectorSize> {
                Some(match n {
                    "2" => VectorSize::Bi,
                    "3" => VectorSize::Tri,
                    "4" => VectorSize::Quad,
                    _ => return None,
                })
            }

            let vec_parse = |word: &str| {
                let mut iter = word.split("vec");

                let kind = iter.next()?;
                let size = iter.next()?;
                let scalar = kind_width_parse(kind)?;
                let size = size_parse(size)?;

                Some(Type {
                    name: None,
                    inner: TypeInner::Vector { size, scalar },
                })
            };

            let mat_parse = |word: &str| {
                let mut iter = word.split("mat");

                let kind = iter.next()?;
                let size = iter.next()?;
                let scalar = kind_width_parse(kind)?;

                let (columns, rows) = if let Some(size) = size_parse(size) {
                    (size, size)
                } else {
                    let mut iter = size.split('x');
                    match (iter.next()?, iter.next()?, iter.next()) {
                        (col, row, None) => (size_parse(col)?, size_parse(row)?),
                        _ => return None,
                    }
                };

                Some(Type {
                    name: None,
                    inner: TypeInner::Matrix {
                        columns,
                        rows,
                        scalar,
                    },
                })
            };

            let texture_parse = |word: &str| {
                let mut iter = word.split("texture");

                let texture_kind = |ty| {
                    Some(match ty {
                        "" => ScalarKind::Float,
                        "i" => ScalarKind::Sint,
                        "u" => ScalarKind::Uint,
                        _ => return None,
                    })
                };

                let kind = iter.next()?;
                let size = iter.next()?;
                let kind = texture_kind(kind)?;

                let sampled = |multi| ImageClass::Sampled { kind, multi };

                let (dim, arrayed, class) = match size {
                    "1D" => (ImageDimension::D1, false, sampled(false)),
                    "1DArray" => (ImageDimension::D1, true, sampled(false)),
                    "2D" => (ImageDimension::D2, false, sampled(false)),
                    "2DArray" => (ImageDimension::D2, true, sampled(false)),
                    "2DMS" => (ImageDimension::D2, false, sampled(true)),
                    "2DMSArray" => (ImageDimension::D2, true, sampled(true)),
                    "3D" => (ImageDimension::D3, false, sampled(false)),
                    "Cube" => (ImageDimension::Cube, false, sampled(false)),
                    "CubeArray" => (ImageDimension::Cube, true, sampled(false)),
                    _ => return None,
                };

                Some(Type {
                    name: None,
                    inner: TypeInner::Image {
                        dim,
                        arrayed,
                        class,
                    },
                })
            };

            let image_parse = |word: &str| {
                let mut iter = word.split("image");

                let texture_kind = |ty| {
                    Some(match ty {
                        "" => ScalarKind::Float,
                        "i" => ScalarKind::Sint,
                        "u" => ScalarKind::Uint,
                        _ => return None,
                    })
                };

                let kind = iter.next()?;
                let size = iter.next()?;
                // TODO: Check that the texture format and the kind match
                let _ = texture_kind(kind)?;

                let class = ImageClass::Storage {
                    format: crate::StorageFormat::R8Uint,
                    access: crate::StorageAccess::LOAD | crate::StorageAccess::STORE,
                };

                // TODO: glsl support multisampled storage images, naga doesn't
                let (dim, arrayed) = match size {
                    "1D" => (ImageDimension::D1, false),
                    "1DArray" => (ImageDimension::D1, true),
                    "2D" => (ImageDimension::D2, false),
                    "2DArray" => (ImageDimension::D2, true),
                    "3D" => (ImageDimension::D3, false),
                    // Naga doesn't support cube images and it's usefulness
                    // is questionable, so they won't be supported for now
                    // "Cube" => (ImageDimension::Cube, false),
                    // "CubeArray" => (ImageDimension::Cube, true),
                    _ => return None,
                };

                Some(Type {
                    name: None,
                    inner: TypeInner::Image {
                        dim,
                        arrayed,
                        class,
                    },
                })
            };

            vec_parse(word)
                .or_else(|| mat_parse(word))
                .or_else(|| texture_parse(word))
                .or_else(|| image_parse(word))
        }
    }
}

pub const fn scalar_components(ty: &TypeInner) -> Option<Scalar> {
    match *ty {
        TypeInner::Scalar(scalar)
        | TypeInner::Vector { scalar, .. }
        | TypeInner::ValuePointer { scalar, .. }
        | TypeInner::Matrix { scalar, .. } => Some(scalar),
        _ => None,
    }
}

pub const fn type_power(scalar: Scalar) -> Option<u32> {
    Some(match scalar.kind {
        ScalarKind::Sint => 0,
        ScalarKind::Uint => 1,
        ScalarKind::Float if scalar.width == 4 => 2,
        ScalarKind::Float => 3,
        ScalarKind::Bool | ScalarKind::AbstractInt | ScalarKind::AbstractFloat => return None,
    })
}

impl Context<'_> {
    /// Resolves the types of the expressions until `expr` (inclusive)
    ///
    /// This needs to be done before the [`typifier`] can be queried for
    /// the types of the expressions in the range between the last grow and `expr`.
    ///
    /// # Note
    ///
    /// The `resolve_type*` methods (like [`resolve_type`]) automatically
    /// grow the [`typifier`] so calling this method is not necessary when using
    /// them.
    ///
    /// [`typifier`]: Context::typifier
    /// [`resolve_type`]: Self::resolve_type
    pub(crate) fn typifier_grow(&mut self, expr: Handle<Expression>, meta: Span) -> Result<()> {
        let resolve_ctx = ResolveContext::with_locals(self.module, &self.locals, &self.arguments);

        let typifier = if self.is_const {
            &mut self.const_typifier
        } else {
            &mut self.typifier
        };

        let expressions = if self.is_const {
            &self.module.global_expressions
        } else {
            &self.expressions
        };

        typifier
            .grow(expr, expressions, &resolve_ctx)
            .map_err(|error| Error {
                kind: ErrorKind::SemanticError(format!("Can't resolve type: {error:?}").into()),
                meta,
            })
    }

    pub(crate) fn get_type(&self, expr: Handle<Expression>) -> &TypeInner {
        let typifier = if self.is_const {
            &self.const_typifier
        } else {
            &self.typifier
        };

        typifier.get(expr, &self.module.types)
    }

    /// Gets the type for the result of the `expr` expression
    ///
    /// Automatically grows the [`typifier`] to `expr` so calling
    /// [`typifier_grow`] is not necessary
    ///
    /// [`typifier`]: Context::typifier
    /// [`typifier_grow`]: Self::typifier_grow
    pub(crate) fn resolve_type(
        &mut self,
        expr: Handle<Expression>,
        meta: Span,
    ) -> Result<&TypeInner> {
        self.typifier_grow(expr, meta)?;
        Ok(self.get_type(expr))
    }

    /// Gets the type handle for the result of the `expr` expression
    ///
    /// Automatically grows the [`typifier`] to `expr` so calling
    /// [`typifier_grow`] is not necessary
    ///
    /// # Note
    ///
    /// Consider using [`resolve_type`] whenever possible
    /// since it doesn't require adding each type to the [`types`] arena
    /// and it doesn't need to mutably borrow the [`Parser`][Self]
    ///
    /// [`types`]: crate::Module::types
    /// [`typifier`]: Context::typifier
    /// [`typifier_grow`]: Self::typifier_grow
    /// [`resolve_type`]: Self::resolve_type
    pub(crate) fn resolve_type_handle(
        &mut self,
        expr: Handle<Expression>,
        meta: Span,
    ) -> Result<Handle<Type>> {
        self.typifier_grow(expr, meta)?;

        let typifier = if self.is_const {
            &mut self.const_typifier
        } else {
            &mut self.typifier
        };

        Ok(typifier.register_type(expr, &mut self.module.types))
    }

    /// Invalidates the cached type resolution for `expr` forcing a recomputation
    pub(crate) fn invalidate_expression(
        &mut self,
        expr: Handle<Expression>,
        meta: Span,
    ) -> Result<()> {
        let resolve_ctx = ResolveContext::with_locals(self.module, &self.locals, &self.arguments);

        let typifier = if self.is_const {
            &mut self.const_typifier
        } else {
            &mut self.typifier
        };

        typifier
            .invalidate(expr, &self.expressions, &resolve_ctx)
            .map_err(|error| Error {
                kind: ErrorKind::SemanticError(format!("Can't resolve type: {error:?}").into()),
                meta,
            })
    }

    pub(crate) fn lift_up_const_expression(
        &mut self,
        expr: Handle<Expression>,
    ) -> Result<Handle<Expression>> {
        let meta = self.expressions.get_span(expr);
        let h = match self.expressions[expr] {
            ref expr @ (Expression::Literal(_)
            | Expression::Constant(_)
            | Expression::ZeroValue(_)) => {
                self.module.global_expressions.append(expr.clone(), meta)
            }
            Expression::Compose { ty, ref components } => {
                let mut components = components.clone();
                for component in &mut components {
                    *component = self.lift_up_const_expression(*component)?;
                }
                self.module
                    .global_expressions
                    .append(Expression::Compose { ty, components }, meta)
            }
            Expression::Splat { size, value } => {
                let value = self.lift_up_const_expression(value)?;
                self.module
                    .global_expressions
                    .append(Expression::Splat { size, value }, meta)
            }
            _ => {
                return Err(Error {
                    kind: ErrorKind::SemanticError("Expression is not const-expression".into()),
                    meta,
                })
            }
        };
        self.global_expression_kind_tracker
            .insert(h, crate::proc::ExpressionKind::Const);
        Ok(h)
    }
}
