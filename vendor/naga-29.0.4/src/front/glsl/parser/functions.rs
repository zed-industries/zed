use alloc::{vec, vec::Vec};

use crate::front::glsl::context::ExprPos;
use crate::front::glsl::Span;
use crate::Literal;
use crate::{
    front::glsl::{
        ast::ParameterQualifier,
        context::Context,
        parser::ParsingContext,
        token::{Token, TokenValue},
        variables::VarDeclaration,
        Error, ErrorKind, Frontend, Result,
    },
    Block, Expression, Statement, SwitchCase, UnaryOperator,
};

impl ParsingContext<'_> {
    pub fn peek_parameter_qualifier(&mut self, frontend: &mut Frontend) -> bool {
        self.peek(frontend).is_some_and(|t| match t.value {
            TokenValue::In | TokenValue::Out | TokenValue::InOut | TokenValue::Const => true,
            _ => false,
        })
    }

    /// Returns the parsed `ParameterQualifier` or `ParameterQualifier::In`
    pub fn parse_parameter_qualifier(&mut self, frontend: &mut Frontend) -> ParameterQualifier {
        if self.peek_parameter_qualifier(frontend) {
            match self.bump(frontend).unwrap().value {
                TokenValue::In => ParameterQualifier::In,
                TokenValue::Out => ParameterQualifier::Out,
                TokenValue::InOut => ParameterQualifier::InOut,
                TokenValue::Const => ParameterQualifier::Const,
                _ => unreachable!(),
            }
        } else {
            ParameterQualifier::In
        }
    }

    pub fn parse_statement(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        terminator: &mut Option<usize>,
        is_inside_loop: bool,
    ) -> Result<Option<Span>> {
        // Type qualifiers always identify a declaration statement
        if self.peek_type_qualifier(frontend) {
            return self.parse_declaration(frontend, ctx, false, is_inside_loop);
        }

        // Type names can identify either declaration statements or type constructors
        // depending on whether the token following the type name is a `(` (LeftParen)
        if self.peek_type_name(frontend) {
            // Start by consuming the type name so that we can peek the token after it
            let token = self.bump(frontend)?;
            // Peek the next token and check if it's a `(` (LeftParen) if so the statement
            // is a constructor, otherwise it's a declaration. We need to do the check
            // beforehand and not in the if since we will backtrack before the if
            let declaration = TokenValue::LeftParen != self.expect_peek(frontend)?.value;

            self.backtrack(token)?;

            if declaration {
                return self.parse_declaration(frontend, ctx, false, is_inside_loop);
            }
        }

        let new_break = || {
            let mut block = Block::new();
            block.push(Statement::Break, Span::default());
            block
        };

        let &Token {
            ref value,
            mut meta,
        } = self.expect_peek(frontend)?;

        let meta_rest = match *value {
            TokenValue::Continue => {
                let meta = self.bump(frontend)?.meta;
                ctx.body.push(Statement::Continue, meta);
                terminator.get_or_insert(ctx.body.len());
                self.expect(frontend, TokenValue::Semicolon)?.meta
            }
            TokenValue::Break => {
                let meta = self.bump(frontend)?.meta;
                ctx.body.push(Statement::Break, meta);
                terminator.get_or_insert(ctx.body.len());
                self.expect(frontend, TokenValue::Semicolon)?.meta
            }
            TokenValue::Return => {
                self.bump(frontend)?;
                let (value, meta) = match self.expect_peek(frontend)?.value {
                    TokenValue::Semicolon => (None, self.bump(frontend)?.meta),
                    _ => {
                        // TODO: Implicit conversions
                        let mut stmt = ctx.stmt_ctx();
                        let expr = self.parse_expression(frontend, ctx, &mut stmt)?;
                        self.expect(frontend, TokenValue::Semicolon)?;
                        let (handle, meta) =
                            ctx.lower_expect(stmt, frontend, expr, ExprPos::Rhs)?;
                        (Some(handle), meta)
                    }
                };

                ctx.emit_restart();

                ctx.body.push(Statement::Return { value }, meta);
                terminator.get_or_insert(ctx.body.len());

                meta
            }
            TokenValue::Discard => {
                let meta = self.bump(frontend)?.meta;
                ctx.body.push(Statement::Kill, meta);
                terminator.get_or_insert(ctx.body.len());

                self.expect(frontend, TokenValue::Semicolon)?.meta
            }
            TokenValue::If => {
                let mut meta = self.bump(frontend)?.meta;

                self.expect(frontend, TokenValue::LeftParen)?;
                let condition = {
                    let mut stmt = ctx.stmt_ctx();
                    let expr = self.parse_expression(frontend, ctx, &mut stmt)?;
                    let (handle, more_meta) =
                        ctx.lower_expect(stmt, frontend, expr, ExprPos::Rhs)?;
                    meta.subsume(more_meta);
                    handle
                };
                self.expect(frontend, TokenValue::RightParen)?;

                let accept = ctx.new_body(|ctx| {
                    if let Some(more_meta) =
                        self.parse_statement(frontend, ctx, &mut None, is_inside_loop)?
                    {
                        meta.subsume(more_meta);
                    }
                    Ok(())
                })?;

                let reject = ctx.new_body(|ctx| {
                    if self.bump_if(frontend, TokenValue::Else).is_some() {
                        if let Some(more_meta) =
                            self.parse_statement(frontend, ctx, &mut None, is_inside_loop)?
                        {
                            meta.subsume(more_meta);
                        }
                    }
                    Ok(())
                })?;

                ctx.body.push(
                    Statement::If {
                        condition,
                        accept,
                        reject,
                    },
                    meta,
                );

                meta
            }
            TokenValue::Switch => {
                let mut meta = self.bump(frontend)?.meta;
                let end_meta;

                self.expect(frontend, TokenValue::LeftParen)?;

                let (selector, uint) = {
                    let mut stmt = ctx.stmt_ctx();
                    let expr = self.parse_expression(frontend, ctx, &mut stmt)?;
                    let (root, meta) = ctx.lower_expect(stmt, frontend, expr, ExprPos::Rhs)?;
                    let uint = ctx.resolve_type(root, meta)?.scalar_kind()
                        == Some(crate::ScalarKind::Uint);
                    (root, uint)
                };

                self.expect(frontend, TokenValue::RightParen)?;

                ctx.emit_restart();

                let mut cases = Vec::new();
                // Track if any default case is present in the switch statement.
                let mut default_present = false;

                self.expect(frontend, TokenValue::LeftBrace)?;
                loop {
                    let value = match self.expect_peek(frontend)?.value {
                        TokenValue::Case => {
                            self.bump(frontend)?;

                            let (const_expr, meta) = self.parse_constant_expression(
                                frontend,
                                ctx.module,
                                ctx.global_expression_kind_tracker,
                            )?;

                            match ctx.module.global_expressions[const_expr] {
                                Expression::Literal(Literal::I32(value)) => match uint {
                                    // This unchecked cast isn't good, but since
                                    // we only reach this code when the selector
                                    // is unsigned but the case label is signed,
                                    // verification will reject the module
                                    // anyway (which also matches GLSL's rules).
                                    true => crate::SwitchValue::U32(value as u32),
                                    false => crate::SwitchValue::I32(value),
                                },
                                Expression::Literal(Literal::U32(value)) => {
                                    crate::SwitchValue::U32(value)
                                }
                                _ => {
                                    frontend.errors.push(Error {
                                        kind: ErrorKind::SemanticError(
                                            "Case values can only be integers".into(),
                                        ),
                                        meta,
                                    });

                                    crate::SwitchValue::I32(0)
                                }
                            }
                        }
                        TokenValue::Default => {
                            self.bump(frontend)?;
                            default_present = true;
                            crate::SwitchValue::Default
                        }
                        TokenValue::RightBrace => {
                            end_meta = self.bump(frontend)?.meta;
                            break;
                        }
                        _ => {
                            let Token { value, meta } = self.bump(frontend)?;
                            return Err(Error {
                                kind: ErrorKind::InvalidToken(
                                    value,
                                    vec![
                                        TokenValue::Case.into(),
                                        TokenValue::Default.into(),
                                        TokenValue::RightBrace.into(),
                                    ],
                                ),
                                meta,
                            });
                        }
                    };

                    self.expect(frontend, TokenValue::Colon)?;

                    let mut fall_through = true;

                    let body = ctx.new_body(|ctx| {
                        let mut case_terminator = None;
                        loop {
                            match self.expect_peek(frontend)?.value {
                                TokenValue::Case | TokenValue::Default | TokenValue::RightBrace => {
                                    break
                                }
                                _ => {
                                    self.parse_statement(
                                        frontend,
                                        ctx,
                                        &mut case_terminator,
                                        is_inside_loop,
                                    )?;
                                }
                            }
                        }

                        if let Some(mut idx) = case_terminator {
                            if let Statement::Break = ctx.body[idx - 1] {
                                fall_through = false;
                                idx -= 1;
                            }

                            ctx.body.cull(idx..)
                        }

                        Ok(())
                    })?;

                    cases.push(SwitchCase {
                        value,
                        body,
                        fall_through,
                    })
                }

                meta.subsume(end_meta);

                // NOTE: do not unwrap here since a switch statement isn't required
                // to have any cases.
                if let Some(case) = cases.last_mut() {
                    // GLSL requires that the last case not be empty, so we check
                    // that here and produce an error otherwise (fall_through must
                    // also be checked because `break`s count as statements but
                    // they aren't added to the body)
                    if case.body.is_empty() && case.fall_through {
                        frontend.errors.push(Error {
                            kind: ErrorKind::SemanticError(
                                "last case/default label must be followed by statements".into(),
                            ),
                            meta,
                        })
                    }

                    // GLSL allows the last case to not have any `break` statement,
                    // this would mark it as fall through but naga's IR requires that
                    // the last case must not be fall through, so we mark need to mark
                    // the last case as not fall through always.
                    case.fall_through = false;
                }

                // Add an empty default case in case non was present, this is needed because
                // naga's IR requires that all switch statements must have a default case but
                // GLSL doesn't require that, so we might need to add an empty default case.
                if !default_present {
                    cases.push(SwitchCase {
                        value: crate::SwitchValue::Default,
                        body: Block::new(),
                        fall_through: false,
                    })
                }

                ctx.body.push(Statement::Switch { selector, cases }, meta);

                meta
            }
            TokenValue::While => {
                let mut meta = self.bump(frontend)?.meta;

                let loop_body = ctx.new_body(|ctx| {
                    let mut stmt = ctx.stmt_ctx();
                    self.expect(frontend, TokenValue::LeftParen)?;
                    let root = self.parse_expression(frontend, ctx, &mut stmt)?;
                    meta.subsume(self.expect(frontend, TokenValue::RightParen)?.meta);

                    let (expr, expr_meta) = ctx.lower_expect(stmt, frontend, root, ExprPos::Rhs)?;
                    let condition = ctx.add_expression(
                        Expression::Unary {
                            op: UnaryOperator::LogicalNot,
                            expr,
                        },
                        expr_meta,
                    )?;

                    ctx.emit_restart();

                    ctx.body.push(
                        Statement::If {
                            condition,
                            accept: new_break(),
                            reject: Block::new(),
                        },
                        Span::default(),
                    );

                    meta.subsume(expr_meta);

                    if let Some(body_meta) = self.parse_statement(frontend, ctx, &mut None, true)? {
                        meta.subsume(body_meta);
                    }
                    Ok(())
                })?;

                ctx.body.push(
                    Statement::Loop {
                        body: loop_body,
                        continuing: Block::new(),
                        break_if: None,
                    },
                    meta,
                );

                meta
            }
            TokenValue::Do => {
                let mut meta = self.bump(frontend)?.meta;

                let loop_body = ctx.new_body(|ctx| {
                    let mut terminator = None;
                    self.parse_statement(frontend, ctx, &mut terminator, true)?;

                    let mut stmt = ctx.stmt_ctx();

                    self.expect(frontend, TokenValue::While)?;
                    self.expect(frontend, TokenValue::LeftParen)?;
                    let root = self.parse_expression(frontend, ctx, &mut stmt)?;
                    let end_meta = self.expect(frontend, TokenValue::RightParen)?.meta;

                    meta.subsume(end_meta);

                    let (expr, expr_meta) = ctx.lower_expect(stmt, frontend, root, ExprPos::Rhs)?;
                    let condition = ctx.add_expression(
                        Expression::Unary {
                            op: UnaryOperator::LogicalNot,
                            expr,
                        },
                        expr_meta,
                    )?;

                    ctx.emit_restart();

                    ctx.body.push(
                        Statement::If {
                            condition,
                            accept: new_break(),
                            reject: Block::new(),
                        },
                        Span::default(),
                    );

                    if let Some(idx) = terminator {
                        ctx.body.cull(idx..)
                    }
                    Ok(())
                })?;

                ctx.body.push(
                    Statement::Loop {
                        body: loop_body,
                        continuing: Block::new(),
                        break_if: None,
                    },
                    meta,
                );

                meta
            }
            TokenValue::For => {
                let mut meta = self.bump(frontend)?.meta;

                ctx.symbol_table.push_scope();
                self.expect(frontend, TokenValue::LeftParen)?;

                if self.bump_if(frontend, TokenValue::Semicolon).is_none() {
                    if self.peek_type_name(frontend) || self.peek_type_qualifier(frontend) {
                        self.parse_declaration(frontend, ctx, false, is_inside_loop)?;
                    } else {
                        let mut stmt = ctx.stmt_ctx();
                        let expr = self.parse_expression(frontend, ctx, &mut stmt)?;
                        ctx.lower(stmt, frontend, expr, ExprPos::Rhs)?;
                        self.expect(frontend, TokenValue::Semicolon)?;
                    }
                }

                let loop_body = ctx.new_body(|ctx| {
                    if self.bump_if(frontend, TokenValue::Semicolon).is_none() {
                        let (expr, expr_meta) = if self.peek_type_name(frontend)
                            || self.peek_type_qualifier(frontend)
                        {
                            let mut qualifiers = self.parse_type_qualifiers(frontend, ctx)?;
                            let (ty, mut meta) = self.parse_type_non_void(frontend, ctx)?;
                            let name = self.expect_ident(frontend)?.0;

                            self.expect(frontend, TokenValue::Assign)?;

                            let (value, end_meta) = self.parse_initializer(frontend, ty, ctx)?;
                            meta.subsume(end_meta);

                            let decl = VarDeclaration {
                                qualifiers: &mut qualifiers,
                                ty,
                                name: Some(name),
                                init: None,
                                meta,
                            };

                            let pointer = frontend.add_local_var(ctx, decl)?;

                            ctx.emit_restart();

                            ctx.body.push(Statement::Store { pointer, value }, meta);

                            (value, end_meta)
                        } else {
                            let mut stmt = ctx.stmt_ctx();
                            let root = self.parse_expression(frontend, ctx, &mut stmt)?;
                            ctx.lower_expect(stmt, frontend, root, ExprPos::Rhs)?
                        };

                        let condition = ctx.add_expression(
                            Expression::Unary {
                                op: UnaryOperator::LogicalNot,
                                expr,
                            },
                            expr_meta,
                        )?;

                        ctx.emit_restart();

                        ctx.body.push(
                            Statement::If {
                                condition,
                                accept: new_break(),
                                reject: Block::new(),
                            },
                            Span::default(),
                        );

                        self.expect(frontend, TokenValue::Semicolon)?;
                    }
                    Ok(())
                })?;

                let continuing = ctx.new_body(|ctx| {
                    match self.expect_peek(frontend)?.value {
                        TokenValue::RightParen => {}
                        _ => {
                            let mut stmt = ctx.stmt_ctx();
                            let rest = self.parse_expression(frontend, ctx, &mut stmt)?;
                            ctx.lower(stmt, frontend, rest, ExprPos::Rhs)?;
                        }
                    }
                    Ok(())
                })?;

                meta.subsume(self.expect(frontend, TokenValue::RightParen)?.meta);

                let loop_body = ctx.with_body(loop_body, |ctx| {
                    if let Some(stmt_meta) = self.parse_statement(frontend, ctx, &mut None, true)? {
                        meta.subsume(stmt_meta);
                    }
                    Ok(())
                })?;

                ctx.body.push(
                    Statement::Loop {
                        body: loop_body,
                        continuing,
                        break_if: None,
                    },
                    meta,
                );

                ctx.symbol_table.pop_scope();

                meta
            }
            TokenValue::LeftBrace => {
                let mut meta = self.bump(frontend)?.meta;

                let mut block_terminator = None;

                let block = ctx.new_body(|ctx| {
                    let block_meta = self.parse_compound_statement(
                        meta,
                        frontend,
                        ctx,
                        &mut block_terminator,
                        is_inside_loop,
                    )?;
                    meta.subsume(block_meta);
                    Ok(())
                })?;

                ctx.body.push(Statement::Block(block), meta);
                if block_terminator.is_some() {
                    terminator.get_or_insert(ctx.body.len());
                }

                meta
            }
            TokenValue::Semicolon => self.bump(frontend)?.meta,
            _ => {
                // Attempt to force expression parsing for remainder of the
                // tokens. Unknown or invalid tokens will be caught there and
                // turned into an error.
                let mut stmt = ctx.stmt_ctx();
                let expr = self.parse_expression(frontend, ctx, &mut stmt)?;
                ctx.lower(stmt, frontend, expr, ExprPos::Rhs)?;
                self.expect(frontend, TokenValue::Semicolon)?.meta
            }
        };

        meta.subsume(meta_rest);
        Ok(Some(meta))
    }

    pub fn parse_compound_statement(
        &mut self,
        mut meta: Span,
        frontend: &mut Frontend,
        ctx: &mut Context,
        terminator: &mut Option<usize>,
        is_inside_loop: bool,
    ) -> Result<Span> {
        ctx.symbol_table.push_scope();

        loop {
            if let Some(Token {
                meta: brace_meta, ..
            }) = self.bump_if(frontend, TokenValue::RightBrace)
            {
                meta.subsume(brace_meta);
                break;
            }

            let stmt = self.parse_statement(frontend, ctx, terminator, is_inside_loop)?;

            if let Some(stmt_meta) = stmt {
                meta.subsume(stmt_meta);
            }
        }

        if let Some(idx) = *terminator {
            ctx.body.cull(idx..)
        }

        ctx.symbol_table.pop_scope();

        Ok(meta)
    }

    pub fn parse_function_args(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
    ) -> Result<()> {
        if self.bump_if(frontend, TokenValue::Void).is_some() {
            return Ok(());
        }

        loop {
            if self.peek_type_name(frontend) || self.peek_parameter_qualifier(frontend) {
                let qualifier = self.parse_parameter_qualifier(frontend);
                let mut ty = self.parse_type_non_void(frontend, ctx)?.0;

                match self.expect_peek(frontend)?.value {
                    TokenValue::Comma => {
                        self.bump(frontend)?;
                        ctx.add_function_arg(None, ty, qualifier)?;
                        continue;
                    }
                    TokenValue::Identifier(_) => {
                        let mut name = self.expect_ident(frontend)?;
                        self.parse_array_specifier(frontend, ctx, &mut name.1, &mut ty)?;

                        ctx.add_function_arg(Some(name), ty, qualifier)?;

                        if self.bump_if(frontend, TokenValue::Comma).is_some() {
                            continue;
                        }

                        break;
                    }
                    _ => break,
                }
            }

            break;
        }

        Ok(())
    }
}
