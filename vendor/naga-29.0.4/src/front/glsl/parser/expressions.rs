use alloc::{vec, vec::Vec};
use core::num::NonZeroU32;

use crate::{
    front::glsl::{
        ast::{FunctionCall, FunctionCallKind, HirExpr, HirExprKind},
        context::{Context, StmtContext},
        error::{ErrorKind, ExpectedToken},
        parser::ParsingContext,
        token::{Token, TokenValue},
        Error, Frontend, Result, Span,
    },
    ArraySize, BinaryOperator, Handle, Literal, Type, TypeInner, UnaryOperator,
};

impl ParsingContext<'_> {
    pub fn parse_primary(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
    ) -> Result<Handle<HirExpr>> {
        let mut token = self.bump(frontend)?;

        let literal = match token.value {
            TokenValue::IntConstant(int) => {
                if int.width != 32 {
                    frontend.errors.push(Error {
                        kind: ErrorKind::SemanticError("Unsupported non-32bit integer".into()),
                        meta: token.meta,
                    });
                }
                if int.signed {
                    Literal::I32(int.value as i32)
                } else {
                    Literal::U32(int.value as u32)
                }
            }
            TokenValue::FloatConstant(float) => {
                if float.width != 32 {
                    frontend.errors.push(Error {
                        kind: ErrorKind::SemanticError(
                            concat!(
                                "Unsupported floating-point value ",
                                "(expected single-precision floating-point number)"
                            )
                            .into(),
                        ),
                        meta: token.meta,
                    });
                }
                Literal::F32(float.value)
            }
            TokenValue::BoolConstant(value) => Literal::Bool(value),
            TokenValue::LeftParen => {
                let expr = self.parse_expression(frontend, ctx, stmt)?;
                let meta = self.expect(frontend, TokenValue::RightParen)?.meta;

                token.meta.subsume(meta);

                return Ok(expr);
            }
            _ => {
                return Err(Error {
                    kind: ErrorKind::InvalidToken(
                        token.value,
                        vec![
                            TokenValue::LeftParen.into(),
                            ExpectedToken::IntLiteral,
                            ExpectedToken::FloatLiteral,
                            ExpectedToken::BoolLiteral,
                        ],
                    ),
                    meta: token.meta,
                });
            }
        };

        Ok(stmt.hir_exprs.append(
            HirExpr {
                kind: HirExprKind::Literal(literal),
                meta: token.meta,
            },
            Default::default(),
        ))
    }

    pub fn parse_function_call_args(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
        meta: &mut Span,
    ) -> Result<Vec<Handle<HirExpr>>> {
        let mut args = Vec::new();
        if let Some(token) = self.bump_if(frontend, TokenValue::RightParen) {
            meta.subsume(token.meta);
        } else {
            loop {
                args.push(self.parse_assignment(frontend, ctx, stmt)?);

                let token = self.bump(frontend)?;
                match token.value {
                    TokenValue::Comma => {}
                    TokenValue::RightParen => {
                        meta.subsume(token.meta);
                        break;
                    }
                    _ => {
                        return Err(Error {
                            kind: ErrorKind::InvalidToken(
                                token.value,
                                vec![TokenValue::Comma.into(), TokenValue::RightParen.into()],
                            ),
                            meta: token.meta,
                        });
                    }
                }
            }
        }

        Ok(args)
    }

    pub fn parse_postfix(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
    ) -> Result<Handle<HirExpr>> {
        let mut base = if self.peek_type_name(frontend) {
            let (mut handle, mut meta) = self.parse_type_non_void(frontend, ctx)?;

            self.expect(frontend, TokenValue::LeftParen)?;
            let args = self.parse_function_call_args(frontend, ctx, stmt, &mut meta)?;

            if let TypeInner::Array {
                size: ArraySize::Dynamic,
                stride,
                base,
            } = ctx.module.types[handle].inner
            {
                let span = ctx.module.types.get_span(handle);

                let size = u32::try_from(args.len())
                    .ok()
                    .and_then(NonZeroU32::new)
                    .ok_or(Error {
                        kind: ErrorKind::SemanticError(
                            "There must be at least one argument".into(),
                        ),
                        meta,
                    })?;

                handle = ctx.module.types.insert(
                    Type {
                        name: None,
                        inner: TypeInner::Array {
                            stride,
                            base,
                            size: ArraySize::Constant(size),
                        },
                    },
                    span,
                )
            }

            stmt.hir_exprs.append(
                HirExpr {
                    kind: HirExprKind::Call(FunctionCall {
                        kind: FunctionCallKind::TypeConstructor(handle),
                        args,
                    }),
                    meta,
                },
                Default::default(),
            )
        } else if let TokenValue::Identifier(_) = self.expect_peek(frontend)?.value {
            let (name, mut meta) = self.expect_ident(frontend)?;

            let expr = if self.bump_if(frontend, TokenValue::LeftParen).is_some() {
                let args = self.parse_function_call_args(frontend, ctx, stmt, &mut meta)?;

                let kind = match frontend.lookup_type.get(&name) {
                    Some(ty) => FunctionCallKind::TypeConstructor(*ty),
                    None => FunctionCallKind::Function(name),
                };

                HirExpr {
                    kind: HirExprKind::Call(FunctionCall { kind, args }),
                    meta,
                }
            } else {
                let var = match frontend.lookup_variable(ctx, &name, meta)? {
                    Some(var) => var,
                    None => {
                        return Err(Error {
                            kind: ErrorKind::UnknownVariable(name),
                            meta,
                        })
                    }
                };

                HirExpr {
                    kind: HirExprKind::Variable(var),
                    meta,
                }
            };

            stmt.hir_exprs.append(expr, Default::default())
        } else {
            self.parse_primary(frontend, ctx, stmt)?
        };

        while let TokenValue::LeftBracket
        | TokenValue::Dot
        | TokenValue::Increment
        | TokenValue::Decrement = self.expect_peek(frontend)?.value
        {
            let Token { value, mut meta } = self.bump(frontend)?;

            match value {
                TokenValue::LeftBracket => {
                    let index = self.parse_expression(frontend, ctx, stmt)?;
                    let end_meta = self.expect(frontend, TokenValue::RightBracket)?.meta;

                    meta.subsume(end_meta);
                    base = stmt.hir_exprs.append(
                        HirExpr {
                            kind: HirExprKind::Access { base, index },
                            meta,
                        },
                        Default::default(),
                    )
                }
                TokenValue::Dot => {
                    let (field, end_meta) = self.expect_ident(frontend)?;

                    if self.bump_if(frontend, TokenValue::LeftParen).is_some() {
                        let args = self.parse_function_call_args(frontend, ctx, stmt, &mut meta)?;

                        base = stmt.hir_exprs.append(
                            HirExpr {
                                kind: HirExprKind::Method {
                                    expr: base,
                                    name: field,
                                    args,
                                },
                                meta,
                            },
                            Default::default(),
                        );
                        continue;
                    }

                    meta.subsume(end_meta);
                    base = stmt.hir_exprs.append(
                        HirExpr {
                            kind: HirExprKind::Select { base, field },
                            meta,
                        },
                        Default::default(),
                    )
                }
                TokenValue::Increment | TokenValue::Decrement => {
                    base = stmt.hir_exprs.append(
                        HirExpr {
                            kind: HirExprKind::PrePostfix {
                                op: match value {
                                    TokenValue::Increment => BinaryOperator::Add,
                                    _ => BinaryOperator::Subtract,
                                },
                                postfix: true,
                                expr: base,
                            },
                            meta,
                        },
                        Default::default(),
                    )
                }
                _ => unreachable!(),
            }
        }

        Ok(base)
    }

    pub fn parse_unary(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
    ) -> Result<Handle<HirExpr>> {
        Ok(match self.expect_peek(frontend)?.value {
            TokenValue::Plus | TokenValue::Dash | TokenValue::Bang | TokenValue::Tilde => {
                let Token { value, mut meta } = self.bump(frontend)?;

                let expr = self.parse_unary(frontend, ctx, stmt)?;
                let end_meta = stmt.hir_exprs[expr].meta;

                let kind = match value {
                    TokenValue::Dash => HirExprKind::Unary {
                        op: UnaryOperator::Negate,
                        expr,
                    },
                    TokenValue::Bang => HirExprKind::Unary {
                        op: UnaryOperator::LogicalNot,
                        expr,
                    },
                    TokenValue::Tilde => HirExprKind::Unary {
                        op: UnaryOperator::BitwiseNot,
                        expr,
                    },
                    _ => return Ok(expr),
                };

                meta.subsume(end_meta);
                stmt.hir_exprs
                    .append(HirExpr { kind, meta }, Default::default())
            }
            TokenValue::Increment | TokenValue::Decrement => {
                let Token { value, meta } = self.bump(frontend)?;

                let expr = self.parse_unary(frontend, ctx, stmt)?;

                stmt.hir_exprs.append(
                    HirExpr {
                        kind: HirExprKind::PrePostfix {
                            op: match value {
                                TokenValue::Increment => BinaryOperator::Add,
                                _ => BinaryOperator::Subtract,
                            },
                            postfix: false,
                            expr,
                        },
                        meta,
                    },
                    Default::default(),
                )
            }
            _ => self.parse_postfix(frontend, ctx, stmt)?,
        })
    }

    pub fn parse_binary(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
        passthrough: Option<Handle<HirExpr>>,
        min_bp: u8,
    ) -> Result<Handle<HirExpr>> {
        let mut left = passthrough
            .ok_or(ErrorKind::EndOfFile /* Dummy error */)
            .or_else(|_| self.parse_unary(frontend, ctx, stmt))?;
        let mut meta = stmt.hir_exprs[left].meta;

        while let Some((l_bp, r_bp)) = binding_power(&self.expect_peek(frontend)?.value) {
            if l_bp < min_bp {
                break;
            }

            let Token { value, .. } = self.bump(frontend)?;

            let right = self.parse_binary(frontend, ctx, stmt, None, r_bp)?;
            let end_meta = stmt.hir_exprs[right].meta;

            meta.subsume(end_meta);
            left = stmt.hir_exprs.append(
                HirExpr {
                    kind: HirExprKind::Binary {
                        left,
                        op: match value {
                            TokenValue::LogicalOr => BinaryOperator::LogicalOr,
                            TokenValue::LogicalXor => BinaryOperator::NotEqual,
                            TokenValue::LogicalAnd => BinaryOperator::LogicalAnd,
                            TokenValue::VerticalBar => BinaryOperator::InclusiveOr,
                            TokenValue::Caret => BinaryOperator::ExclusiveOr,
                            TokenValue::Ampersand => BinaryOperator::And,
                            TokenValue::Equal => BinaryOperator::Equal,
                            TokenValue::NotEqual => BinaryOperator::NotEqual,
                            TokenValue::GreaterEqual => BinaryOperator::GreaterEqual,
                            TokenValue::LessEqual => BinaryOperator::LessEqual,
                            TokenValue::LeftAngle => BinaryOperator::Less,
                            TokenValue::RightAngle => BinaryOperator::Greater,
                            TokenValue::LeftShift => BinaryOperator::ShiftLeft,
                            TokenValue::RightShift => BinaryOperator::ShiftRight,
                            TokenValue::Plus => BinaryOperator::Add,
                            TokenValue::Dash => BinaryOperator::Subtract,
                            TokenValue::Star => BinaryOperator::Multiply,
                            TokenValue::Slash => BinaryOperator::Divide,
                            TokenValue::Percent => BinaryOperator::Modulo,
                            _ => unreachable!(),
                        },
                        right,
                    },
                    meta,
                },
                Default::default(),
            )
        }

        Ok(left)
    }

    pub fn parse_conditional(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
        passthrough: Option<Handle<HirExpr>>,
    ) -> Result<Handle<HirExpr>> {
        let mut condition = self.parse_binary(frontend, ctx, stmt, passthrough, 0)?;
        let mut meta = stmt.hir_exprs[condition].meta;

        if self.bump_if(frontend, TokenValue::Question).is_some() {
            let accept = self.parse_expression(frontend, ctx, stmt)?;
            self.expect(frontend, TokenValue::Colon)?;
            let reject = self.parse_assignment(frontend, ctx, stmt)?;
            let end_meta = stmt.hir_exprs[reject].meta;

            meta.subsume(end_meta);
            condition = stmt.hir_exprs.append(
                HirExpr {
                    kind: HirExprKind::Conditional {
                        condition,
                        accept,
                        reject,
                    },
                    meta,
                },
                Default::default(),
            )
        }

        Ok(condition)
    }

    pub fn parse_assignment(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
    ) -> Result<Handle<HirExpr>> {
        let tgt = self.parse_unary(frontend, ctx, stmt)?;
        let mut meta = stmt.hir_exprs[tgt].meta;

        Ok(match self.expect_peek(frontend)?.value {
            TokenValue::Assign => {
                self.bump(frontend)?;
                let value = self.parse_assignment(frontend, ctx, stmt)?;
                let end_meta = stmt.hir_exprs[value].meta;

                meta.subsume(end_meta);
                stmt.hir_exprs.append(
                    HirExpr {
                        kind: HirExprKind::Assign { tgt, value },
                        meta,
                    },
                    Default::default(),
                )
            }
            TokenValue::OrAssign
            | TokenValue::AndAssign
            | TokenValue::AddAssign
            | TokenValue::DivAssign
            | TokenValue::ModAssign
            | TokenValue::SubAssign
            | TokenValue::MulAssign
            | TokenValue::LeftShiftAssign
            | TokenValue::RightShiftAssign
            | TokenValue::XorAssign => {
                let token = self.bump(frontend)?;
                let right = self.parse_assignment(frontend, ctx, stmt)?;
                let end_meta = stmt.hir_exprs[right].meta;

                meta.subsume(end_meta);
                let value = stmt.hir_exprs.append(
                    HirExpr {
                        meta,
                        kind: HirExprKind::Binary {
                            left: tgt,
                            op: match token.value {
                                TokenValue::OrAssign => BinaryOperator::InclusiveOr,
                                TokenValue::AndAssign => BinaryOperator::And,
                                TokenValue::AddAssign => BinaryOperator::Add,
                                TokenValue::DivAssign => BinaryOperator::Divide,
                                TokenValue::ModAssign => BinaryOperator::Modulo,
                                TokenValue::SubAssign => BinaryOperator::Subtract,
                                TokenValue::MulAssign => BinaryOperator::Multiply,
                                TokenValue::LeftShiftAssign => BinaryOperator::ShiftLeft,
                                TokenValue::RightShiftAssign => BinaryOperator::ShiftRight,
                                TokenValue::XorAssign => BinaryOperator::ExclusiveOr,
                                _ => unreachable!(),
                            },
                            right,
                        },
                    },
                    Default::default(),
                );

                stmt.hir_exprs.append(
                    HirExpr {
                        kind: HirExprKind::Assign { tgt, value },
                        meta,
                    },
                    Default::default(),
                )
            }
            _ => self.parse_conditional(frontend, ctx, stmt, Some(tgt))?,
        })
    }

    pub fn parse_expression(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
        stmt: &mut StmtContext,
    ) -> Result<Handle<HirExpr>> {
        let mut exprs = Vec::new();
        let mut expr = self.parse_assignment(frontend, ctx, stmt)?;
        exprs.push(expr);

        while let TokenValue::Comma = self.expect_peek(frontend)?.value {
            self.bump(frontend)?;
            expr = self.parse_assignment(frontend, ctx, stmt)?;
            exprs.push(expr);
        }

        if exprs.len() == 1 {
            Ok(expr)
        } else {
            let mut meta = stmt.hir_exprs[exprs[0]].meta;
            for &e in &exprs[1..] {
                meta.subsume(stmt.hir_exprs[e].meta);
            }
            expr = stmt.hir_exprs.append(
                HirExpr {
                    kind: HirExprKind::Sequence { exprs },
                    meta,
                },
                Default::default(),
            );

            Ok(expr)
        }
    }
}

const fn binding_power(value: &TokenValue) -> Option<(u8, u8)> {
    Some(match *value {
        TokenValue::LogicalOr => (1, 2),
        TokenValue::LogicalXor => (3, 4),
        TokenValue::LogicalAnd => (5, 6),
        TokenValue::VerticalBar => (7, 8),
        TokenValue::Caret => (9, 10),
        TokenValue::Ampersand => (11, 12),
        TokenValue::Equal | TokenValue::NotEqual => (13, 14),
        TokenValue::GreaterEqual
        | TokenValue::LessEqual
        | TokenValue::LeftAngle
        | TokenValue::RightAngle => (15, 16),
        TokenValue::LeftShift | TokenValue::RightShift => (17, 18),
        TokenValue::Plus | TokenValue::Dash => (19, 20),
        TokenValue::Star | TokenValue::Slash | TokenValue::Percent => (21, 22),
        _ => return None,
    })
}
