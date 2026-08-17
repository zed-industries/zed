use alloc::{string::String, vec, vec::Vec};

use super::{DeclarationContext, ParsingContext, Result};
use crate::{
    front::glsl::{
        ast::{
            GlobalLookup, GlobalLookupKind, Precision, QualifierKey, QualifierValue,
            StorageQualifier, StructLayout, TypeQualifiers,
        },
        context::{Context, ExprPos},
        error::ExpectedToken,
        offset,
        token::{Token, TokenValue},
        types::scalar_components,
        variables::{GlobalOrConstant, VarDeclaration},
        Error, ErrorKind, Frontend, Span,
    },
    proc::Alignment,
    AddressSpace, Expression, FunctionResult, Handle, Scalar, ScalarKind, Statement, StructMember,
    Type, TypeInner,
};

/// Helper method used to retrieve the child type of `ty` at
/// index `i`.
///
/// # Note
///
/// Does not check if the index is valid and returns the same type
/// when indexing out-of-bounds a struct or indexing a non indexable
/// type.
fn element_or_member_type(
    ty: Handle<Type>,
    i: usize,
    types: &mut crate::UniqueArena<Type>,
) -> Handle<Type> {
    match types[ty].inner {
        // The child type of a vector is a scalar of the same kind and width
        TypeInner::Vector { scalar, .. } => types.insert(
            Type {
                name: None,
                inner: TypeInner::Scalar(scalar),
            },
            Default::default(),
        ),
        // The child type of a matrix is a vector of floats with the same
        // width and the size of the matrix rows.
        TypeInner::Matrix { rows, scalar, .. } => types.insert(
            Type {
                name: None,
                inner: TypeInner::Vector { size: rows, scalar },
            },
            Default::default(),
        ),
        // The child type of an array is the base type of the array
        TypeInner::Array { base, .. } => base,
        // The child type of a struct at index `i` is the type of it's
        // member at that same index.
        //
        // In case the index is out of bounds the same type is returned
        TypeInner::Struct { ref members, .. } => {
            members.get(i).map(|member| member.ty).unwrap_or(ty)
        }
        // The type isn't indexable, the same type is returned
        _ => ty,
    }
}

impl ParsingContext<'_> {
    pub fn parse_external_declaration(
        &mut self,
        frontend: &mut Frontend,
        global_ctx: &mut Context,
    ) -> Result<()> {
        if self
            .parse_declaration(frontend, global_ctx, true, false)?
            .is_none()
        {
            let token = self.bump(frontend)?;
            match token.value {
                TokenValue::Semicolon if frontend.meta.version == 460 => Ok(()),
                _ => {
                    let expected = match frontend.meta.version {
                        460 => vec![TokenValue::Semicolon.into(), ExpectedToken::Eof],
                        _ => vec![ExpectedToken::Eof],
                    };
                    Err(Error {
                        kind: ErrorKind::InvalidToken(token.value, expected),
                        meta: token.meta,
                    })
                }
            }
        } else {
            Ok(())
        }
    }

    pub fn parse_initializer(
        &mut self,
        frontend: &mut Frontend,
        ty: Handle<Type>,
        ctx: &mut Context,
    ) -> Result<(Handle<Expression>, Span)> {
        // initializer:
        //     assignment_expression
        //     LEFT_BRACE initializer_list RIGHT_BRACE
        //     LEFT_BRACE initializer_list COMMA RIGHT_BRACE
        //
        // initializer_list:
        //     initializer
        //     initializer_list COMMA initializer
        if let Some(Token { mut meta, .. }) = self.bump_if(frontend, TokenValue::LeftBrace) {
            // initializer_list
            let mut components = Vec::new();
            loop {
                // The type expected to be parsed inside the initializer list
                let new_ty = element_or_member_type(ty, components.len(), &mut ctx.module.types);

                components.push(self.parse_initializer(frontend, new_ty, ctx)?.0);

                let token = self.bump(frontend)?;
                match token.value {
                    TokenValue::Comma => {
                        if let Some(Token { meta: end_meta, .. }) =
                            self.bump_if(frontend, TokenValue::RightBrace)
                        {
                            meta.subsume(end_meta);
                            break;
                        }
                    }
                    TokenValue::RightBrace => {
                        meta.subsume(token.meta);
                        break;
                    }
                    _ => {
                        return Err(Error {
                            kind: ErrorKind::InvalidToken(
                                token.value,
                                vec![TokenValue::Comma.into(), TokenValue::RightBrace.into()],
                            ),
                            meta: token.meta,
                        })
                    }
                }
            }

            Ok((
                ctx.add_expression(Expression::Compose { ty, components }, meta)?,
                meta,
            ))
        } else {
            let mut stmt = ctx.stmt_ctx();
            let expr = self.parse_assignment(frontend, ctx, &mut stmt)?;
            let (mut init, init_meta) = ctx.lower_expect(stmt, frontend, expr, ExprPos::Rhs)?;

            let scalar_components = scalar_components(&ctx.module.types[ty].inner);
            if let Some(scalar) = scalar_components {
                ctx.implicit_conversion(&mut init, init_meta, scalar)?;
            }

            Ok((init, init_meta))
        }
    }

    // Note: caller preparsed the type and qualifiers
    // Note: caller skips this if the fallthrough token is not expected to be consumed here so this
    // produced Error::InvalidToken if it isn't consumed
    pub fn parse_init_declarator_list(
        &mut self,
        frontend: &mut Frontend,
        mut ty: Handle<Type>,
        ctx: &mut DeclarationContext,
    ) -> Result<()> {
        // init_declarator_list:
        //     single_declaration
        //     init_declarator_list COMMA IDENTIFIER
        //     init_declarator_list COMMA IDENTIFIER array_specifier
        //     init_declarator_list COMMA IDENTIFIER array_specifier EQUAL initializer
        //     init_declarator_list COMMA IDENTIFIER EQUAL initializer
        //
        // single_declaration:
        //     fully_specified_type
        //     fully_specified_type IDENTIFIER
        //     fully_specified_type IDENTIFIER array_specifier
        //     fully_specified_type IDENTIFIER array_specifier EQUAL initializer
        //     fully_specified_type IDENTIFIER EQUAL initializer

        // Consume any leading comma, e.g. this is valid: `float, a=1;`
        if self
            .peek(frontend)
            .is_some_and(|t| t.value == TokenValue::Comma)
        {
            self.next(frontend);
        }

        loop {
            let token = self.bump(frontend)?;
            let name = match token.value {
                TokenValue::Semicolon => break,
                TokenValue::Identifier(name) => name,
                _ => {
                    return Err(Error {
                        kind: ErrorKind::InvalidToken(
                            token.value,
                            vec![ExpectedToken::Identifier, TokenValue::Semicolon.into()],
                        ),
                        meta: token.meta,
                    })
                }
            };
            let mut meta = token.meta;

            // array_specifier
            // array_specifier EQUAL initializer
            // EQUAL initializer

            // parse an array specifier if it exists
            // NOTE: unlike other parse methods this one doesn't expect an array specifier and
            // returns Ok(None) rather than an error if there is not one
            self.parse_array_specifier(frontend, ctx.ctx, &mut meta, &mut ty)?;

            let is_global_const =
                ctx.qualifiers.storage.0 == StorageQualifier::Const && ctx.external;

            let init = self
                .bump_if(frontend, TokenValue::Assign)
                .map::<Result<_>, _>(|_| {
                    let prev_const = ctx.ctx.is_const;
                    ctx.ctx.is_const = is_global_const;

                    let (mut expr, init_meta) = self.parse_initializer(frontend, ty, ctx.ctx)?;

                    let scalar_components = scalar_components(&ctx.ctx.module.types[ty].inner);
                    if let Some(scalar) = scalar_components {
                        ctx.ctx.implicit_conversion(&mut expr, init_meta, scalar)?;
                    }

                    ctx.ctx.is_const = prev_const;

                    meta.subsume(init_meta);

                    Ok(expr)
                })
                .transpose()?;

            let decl_initializer;
            let late_initializer;
            if is_global_const {
                decl_initializer = init;
                late_initializer = None;
            } else if ctx.external {
                decl_initializer =
                    init.and_then(|expr| ctx.ctx.lift_up_const_expression(expr).ok());
                late_initializer = None;
            } else if let Some(init) = init {
                if ctx.is_inside_loop || !ctx.ctx.local_expression_kind_tracker.is_const(init) {
                    decl_initializer = None;
                    late_initializer = Some(init);
                } else {
                    decl_initializer = Some(init);
                    late_initializer = None;
                }
            } else {
                decl_initializer = None;
                late_initializer = None;
            };

            let pointer = ctx.add_var(frontend, ty, name, decl_initializer, meta)?;

            if let Some(value) = late_initializer {
                ctx.ctx.emit_restart();
                ctx.ctx.body.push(Statement::Store { pointer, value }, meta);
            }

            let token = self.bump(frontend)?;
            match token.value {
                TokenValue::Semicolon => break,
                TokenValue::Comma => {}
                _ => {
                    return Err(Error {
                        kind: ErrorKind::InvalidToken(
                            token.value,
                            vec![TokenValue::Comma.into(), TokenValue::Semicolon.into()],
                        ),
                        meta: token.meta,
                    })
                }
            }
        }

        Ok(())
    }

    /// `external` whether or not we are in a global or local context
    pub fn parse_declaration(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        external: bool,
        is_inside_loop: bool,
    ) -> Result<Option<Span>> {
        //declaration:
        //    function_prototype  SEMICOLON
        //
        //    init_declarator_list SEMICOLON
        //    PRECISION precision_qualifier type_specifier SEMICOLON
        //
        //    type_qualifier IDENTIFIER LEFT_BRACE struct_declaration_list RIGHT_BRACE SEMICOLON
        //    type_qualifier IDENTIFIER LEFT_BRACE struct_declaration_list RIGHT_BRACE IDENTIFIER SEMICOLON
        //    type_qualifier IDENTIFIER LEFT_BRACE struct_declaration_list RIGHT_BRACE IDENTIFIER array_specifier SEMICOLON
        //    type_qualifier SEMICOLON type_qualifier IDENTIFIER SEMICOLON
        //    type_qualifier IDENTIFIER identifier_list SEMICOLON

        if self.peek_type_qualifier(frontend) || self.peek_type_name(frontend) {
            let mut qualifiers = self.parse_type_qualifiers(frontend, ctx)?;

            if self.peek_type_name(frontend) {
                // This branch handles variables and function prototypes and if
                // external is true also function definitions
                let (ty, mut meta) = self.parse_type(frontend, ctx)?;

                let token = self.bump(frontend)?;
                let token_fallthrough = match token.value {
                    TokenValue::Identifier(name) => match self.expect_peek(frontend)?.value {
                        TokenValue::LeftParen => {
                            // This branch handles function definition and prototypes
                            self.bump(frontend)?;

                            let result = ty.map(|ty| FunctionResult { ty, binding: None });

                            let mut context = Context::new(
                                frontend,
                                ctx.module,
                                false,
                                ctx.global_expression_kind_tracker,
                            )?;

                            self.parse_function_args(frontend, &mut context)?;

                            let end_meta = self.expect(frontend, TokenValue::RightParen)?.meta;
                            meta.subsume(end_meta);

                            let token = self.bump(frontend)?;
                            return match token.value {
                                TokenValue::Semicolon => {
                                    // This branch handles function prototypes
                                    frontend.add_prototype(context, name, result, meta);

                                    Ok(Some(meta))
                                }
                                TokenValue::LeftBrace if external => {
                                    // This branch handles function definitions
                                    // as you can see by the guard this branch
                                    // only happens if external is also true

                                    // parse the body
                                    self.parse_compound_statement(
                                        token.meta,
                                        frontend,
                                        &mut context,
                                        &mut None,
                                        false,
                                    )?;

                                    frontend.add_function(context, name, result, meta);

                                    Ok(Some(meta))
                                }
                                _ if external => Err(Error {
                                    kind: ErrorKind::InvalidToken(
                                        token.value,
                                        vec![
                                            TokenValue::LeftBrace.into(),
                                            TokenValue::Semicolon.into(),
                                        ],
                                    ),
                                    meta: token.meta,
                                }),
                                _ => Err(Error {
                                    kind: ErrorKind::InvalidToken(
                                        token.value,
                                        vec![TokenValue::Semicolon.into()],
                                    ),
                                    meta: token.meta,
                                }),
                            };
                        }
                        // Pass the token to the init_declarator_list parser
                        _ => Token {
                            value: TokenValue::Identifier(name),
                            meta: token.meta,
                        },
                    },
                    // Pass the token to the init_declarator_list parser
                    _ => token,
                };

                // If program execution has reached here then this will be a
                // init_declarator_list
                // token_fallthrough will have a token that was already bumped
                if let Some(ty) = ty {
                    let mut ctx = DeclarationContext {
                        qualifiers,
                        external,
                        is_inside_loop,
                        ctx,
                    };

                    self.backtrack(token_fallthrough)?;
                    self.parse_init_declarator_list(frontend, ty, &mut ctx)?;
                } else {
                    frontend.errors.push(Error {
                        kind: ErrorKind::SemanticError("Declaration cannot have void type".into()),
                        meta,
                    })
                }

                Ok(Some(meta))
            } else {
                // This branch handles struct definitions and modifiers like
                // ```glsl
                // layout(early_fragment_tests);
                // ```
                let token = self.bump(frontend)?;
                match token.value {
                    TokenValue::Identifier(ty_name) => {
                        if self.bump_if(frontend, TokenValue::LeftBrace).is_some() {
                            self.parse_block_declaration(
                                frontend,
                                ctx,
                                &mut qualifiers,
                                ty_name,
                                token.meta,
                            )
                            .map(Some)
                        } else {
                            if qualifiers.invariant.take().is_some() {
                                frontend.make_variable_invariant(ctx, &ty_name, token.meta)?;

                                qualifiers.unused_errors(&mut frontend.errors);
                                self.expect(frontend, TokenValue::Semicolon)?;
                                return Ok(Some(qualifiers.span));
                            }

                            //TODO: declaration
                            // type_qualifier IDENTIFIER SEMICOLON
                            // type_qualifier IDENTIFIER identifier_list SEMICOLON
                            Err(Error {
                                kind: ErrorKind::NotImplemented("variable qualifier"),
                                meta: token.meta,
                            })
                        }
                    }
                    TokenValue::Semicolon => {
                        if let Some(value) =
                            qualifiers.uint_layout_qualifier("local_size_x", &mut frontend.errors)
                        {
                            frontend.meta.workgroup_size[0] = value;
                        }
                        if let Some(value) =
                            qualifiers.uint_layout_qualifier("local_size_y", &mut frontend.errors)
                        {
                            frontend.meta.workgroup_size[1] = value;
                        }
                        if let Some(value) =
                            qualifiers.uint_layout_qualifier("local_size_z", &mut frontend.errors)
                        {
                            frontend.meta.workgroup_size[2] = value;
                        }

                        frontend.meta.early_fragment_tests |= qualifiers
                            .none_layout_qualifier("early_fragment_tests", &mut frontend.errors);

                        qualifiers.unused_errors(&mut frontend.errors);

                        Ok(Some(qualifiers.span))
                    }
                    _ => Err(Error {
                        kind: ErrorKind::InvalidToken(
                            token.value,
                            vec![ExpectedToken::Identifier, TokenValue::Semicolon.into()],
                        ),
                        meta: token.meta,
                    }),
                }
            }
        } else {
            match self.peek(frontend).map(|t| &t.value) {
                Some(&TokenValue::Precision) => {
                    // PRECISION precision_qualifier type_specifier SEMICOLON
                    self.bump(frontend)?;

                    let token = self.bump(frontend)?;
                    let _ = match token.value {
                        TokenValue::PrecisionQualifier(p) => p,
                        _ => {
                            return Err(Error {
                                kind: ErrorKind::InvalidToken(
                                    token.value,
                                    vec![
                                        TokenValue::PrecisionQualifier(Precision::High).into(),
                                        TokenValue::PrecisionQualifier(Precision::Medium).into(),
                                        TokenValue::PrecisionQualifier(Precision::Low).into(),
                                    ],
                                ),
                                meta: token.meta,
                            })
                        }
                    };

                    let (ty, meta) = self.parse_type_non_void(frontend, ctx)?;

                    match ctx.module.types[ty].inner {
                        TypeInner::Scalar(Scalar {
                            kind: ScalarKind::Float | ScalarKind::Sint,
                            ..
                        }) => {}
                        _ => frontend.errors.push(Error {
                            kind: ErrorKind::SemanticError(
                                "Precision statement can only work on floats and ints".into(),
                            ),
                            meta,
                        }),
                    }

                    self.expect(frontend, TokenValue::Semicolon)?;

                    Ok(Some(meta))
                }
                _ => Ok(None),
            }
        }
    }

    pub fn parse_block_declaration(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        qualifiers: &mut TypeQualifiers,
        ty_name: String,
        mut meta: Span,
    ) -> Result<Span> {
        let layout = match qualifiers.layout_qualifiers.remove(&QualifierKey::Layout) {
            Some((QualifierValue::Layout(l), _)) => l,
            None => {
                if let StorageQualifier::AddressSpace(AddressSpace::Storage { .. }) =
                    qualifiers.storage.0
                {
                    StructLayout::Std430
                } else {
                    StructLayout::Std140
                }
            }
            _ => unreachable!(),
        };

        let mut members = Vec::new();
        let span = self.parse_struct_declaration_list(frontend, ctx, &mut members, layout)?;
        self.expect(frontend, TokenValue::RightBrace)?;

        let mut ty = ctx.module.types.insert(
            Type {
                name: Some(ty_name),
                inner: TypeInner::Struct {
                    members: members.clone(),
                    span,
                },
            },
            Default::default(),
        );

        let token = self.bump(frontend)?;
        let name = match token.value {
            TokenValue::Semicolon => None,
            TokenValue::Identifier(name) => {
                self.parse_array_specifier(frontend, ctx, &mut meta, &mut ty)?;

                self.expect(frontend, TokenValue::Semicolon)?;

                Some(name)
            }
            _ => {
                return Err(Error {
                    kind: ErrorKind::InvalidToken(
                        token.value,
                        vec![ExpectedToken::Identifier, TokenValue::Semicolon.into()],
                    ),
                    meta: token.meta,
                })
            }
        };

        let global = frontend.add_global_var(
            ctx,
            VarDeclaration {
                qualifiers,
                ty,
                name,
                init: None,
                meta,
            },
        )?;

        for (i, k, ty) in members.into_iter().enumerate().filter_map(|(i, m)| {
            let ty = m.ty;
            m.name.map(|s| (i as u32, s, ty))
        }) {
            let lookup = GlobalLookup {
                kind: match global {
                    GlobalOrConstant::Global(handle) => GlobalLookupKind::BlockSelect(handle, i),
                    GlobalOrConstant::Constant(handle) => GlobalLookupKind::Constant(handle, ty),
                    GlobalOrConstant::Override(handle) => GlobalLookupKind::Override(handle, ty),
                },
                entry_arg: None,
                mutable: true,
            };
            ctx.add_global(&k, lookup)?;

            frontend.global_variables.push((k, lookup));
        }

        Ok(meta)
    }

    // TODO: Accept layout arguments
    pub fn parse_struct_declaration_list(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        members: &mut Vec<StructMember>,
        layout: StructLayout,
    ) -> Result<u32> {
        let mut span = 0;
        let mut align = Alignment::ONE;

        loop {
            // TODO: type_qualifier

            let (base_ty, mut meta) = self.parse_type_non_void(frontend, ctx)?;

            loop {
                let (name, name_meta) = self.expect_ident(frontend)?;
                let mut ty = base_ty;
                self.parse_array_specifier(frontend, ctx, &mut meta, &mut ty)?;

                meta.subsume(name_meta);

                let info = offset::calculate_offset(
                    ty,
                    meta,
                    layout,
                    &mut ctx.module.types,
                    &mut frontend.errors,
                );

                let member_alignment = info.align;
                span = member_alignment.round_up(span);
                align = member_alignment.max(align);

                members.push(StructMember {
                    name: Some(name),
                    ty: info.ty,
                    binding: None,
                    offset: span,
                });

                span += info.span;

                if self.bump_if(frontend, TokenValue::Comma).is_none() {
                    break;
                }
            }

            self.expect(frontend, TokenValue::Semicolon)?;

            if let TokenValue::RightBrace = self.expect_peek(frontend)?.value {
                break;
            }
        }

        span = align.round_up(span);

        Ok(span)
    }
}
