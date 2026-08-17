use alloc::{boxed::Box, vec::Vec};
use directive::enable_extension::ImplementedEnableExtension;

use crate::diagnostic_filter::{
    self, DiagnosticFilter, DiagnosticFilterMap, DiagnosticFilterNode, FilterableTriggeringRule,
    ShouldConflictOnFullDuplicate, StandardFilterableTriggeringRule,
};
use crate::front::wgsl::error::{DiagnosticAttributeNotSupportedPosition, Error, ExpectedToken};
use crate::front::wgsl::parse::directive::enable_extension::{EnableExtension, EnableExtensions};
use crate::front::wgsl::parse::directive::language_extension::LanguageExtension;
use crate::front::wgsl::parse::directive::DirectiveKind;
use crate::front::wgsl::parse::lexer::{Lexer, Token, TokenSpan};
use crate::front::wgsl::parse::number::Number;
use crate::front::wgsl::Result;
use crate::front::SymbolTable;
use crate::{Arena, FastHashSet, FastIndexSet, Handle, ShaderStage, Span};

pub mod ast;
pub mod conv;
pub mod directive;
pub mod lexer;
pub mod number;

/// State for constructing an AST expression.
///
/// Not to be confused with [`lower::ExpressionContext`], which is for producing
/// Naga IR from the AST we produce here.
///
/// [`lower::ExpressionContext`]: super::lower::ExpressionContext
struct ExpressionContext<'input, 'temp, 'out> {
    /// The [`TranslationUnit::expressions`] arena to which we should contribute
    /// expressions.
    ///
    /// [`TranslationUnit::expressions`]: ast::TranslationUnit::expressions
    expressions: &'out mut Arena<ast::Expression<'input>>,

    /// A map from identifiers in scope to the locals/arguments they represent.
    ///
    /// The handles refer to the [`locals`] arena; see that field's
    /// documentation for details.
    ///
    /// [`locals`]: ExpressionContext::locals
    local_table: &'temp mut SymbolTable<&'input str, Handle<ast::Local>>,

    /// Local variable and function argument arena for the function we're building.
    ///
    /// Note that the [`ast::Local`] here is actually a zero-sized type. This
    /// `Arena`'s only role is to assign a unique `Handle` to each local
    /// identifier, and track its definition's span for use in diagnostics. All
    /// the detailed information about locals - names, types, etc. - is kept in
    /// the [`LocalDecl`] statements we parsed from their declarations. For
    /// arguments, that information is kept in [`arguments`].
    ///
    /// In the AST, when an [`Ident`] expression refers to a local variable or
    /// argument, its [`IdentExpr`] holds the referent's `Handle<Local>` in this
    /// arena.
    ///
    /// During lowering, [`LocalDecl`] statements add entries to a per-function
    /// table that maps `Handle<Local>` values to their Naga representations,
    /// accessed via [`StatementContext::local_table`] and
    /// [`LocalExpressionContext::local_table`]. This table is then consulted when
    /// lowering subsequent [`Ident`] expressions.
    ///
    /// [`LocalDecl`]: ast::StatementKind::LocalDecl
    /// [`arguments`]: ast::Function::arguments
    /// [`Ident`]: ast::Expression::Ident
    /// [`IdentExpr`]: ast::IdentExpr
    /// [`StatementContext::local_table`]: super::lower::StatementContext::local_table
    /// [`LocalExpressionContext::local_table`]: super::lower::LocalExpressionContext::local_table
    locals: &'out mut Arena<ast::Local>,

    /// Identifiers used by the current global declaration that have no local definition.
    ///
    /// This becomes the [`GlobalDecl`]'s [`dependencies`] set.
    ///
    /// Note that we don't know at parse time what kind of [`GlobalDecl`] the
    /// name refers to. We can't look up names until we've seen the entire
    /// translation unit.
    ///
    /// [`GlobalDecl`]: ast::GlobalDecl
    /// [`dependencies`]: ast::GlobalDecl::dependencies
    unresolved: &'out mut FastIndexSet<ast::Dependency<'input>>,
}

impl<'a> ExpressionContext<'a, '_, '_> {
    fn parse_binary_op(
        &mut self,
        lexer: &mut Lexer<'a>,
        classifier: impl Fn(Token<'a>) -> Option<crate::BinaryOperator>,
        mut parser: impl FnMut(&mut Lexer<'a>, &mut Self) -> Result<'a, Handle<ast::Expression<'a>>>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        let start = lexer.start_byte_offset();
        let mut accumulator = parser(lexer, self)?;
        while let Some(op) = classifier(lexer.peek().0) {
            let _ = lexer.next();
            let left = accumulator;
            let right = parser(lexer, self)?;
            accumulator = self.expressions.append(
                ast::Expression::Binary { op, left, right },
                lexer.span_from(start),
            );
        }
        Ok(accumulator)
    }

    fn declare_local(&mut self, name: ast::Ident<'a>) -> Result<'a, Handle<ast::Local>> {
        let handle = self.locals.append(ast::Local, name.span);
        if let Some(old) = self.local_table.add(name.name, handle) {
            Err(Box::new(Error::Redefinition {
                previous: self.locals.get_span(old),
                current: name.span,
            }))
        } else {
            Ok(handle)
        }
    }
}

/// Which grammar rule we are in the midst of parsing.
///
/// This is used for error checking. `Parser` maintains a stack of
/// these and (occasionally) checks that it is being pushed and popped
/// as expected.
#[derive(Copy, Clone, Debug, PartialEq)]
enum Rule {
    Attribute,
    VariableDecl,
    FunctionDecl,
    Block,
    Statement,
    PrimaryExpr,
    SingularExpr,
    UnaryExpr,
    GeneralExpr,
    Directive,
    GenericExpr,
    EnclosedExpr,
    LhsExpr,
}

struct ParsedAttribute<T> {
    value: Option<T>,
}

impl<T> Default for ParsedAttribute<T> {
    fn default() -> Self {
        Self { value: None }
    }
}

impl<T> ParsedAttribute<T> {
    fn set(&mut self, value: T, name_span: Span) -> Result<'static, ()> {
        if self.value.is_some() {
            return Err(Box::new(Error::RepeatedAttribute(name_span)));
        }
        self.value = Some(value);
        Ok(())
    }
}

#[derive(Default)]
struct BindingParser<'a> {
    location: ParsedAttribute<Handle<ast::Expression<'a>>>,
    built_in: ParsedAttribute<crate::BuiltIn>,
    interpolation: ParsedAttribute<crate::Interpolation>,
    sampling: ParsedAttribute<crate::Sampling>,
    invariant: ParsedAttribute<bool>,
    blend_src: ParsedAttribute<Handle<ast::Expression<'a>>>,
    per_primitive: ParsedAttribute<()>,
}

impl<'a> BindingParser<'a> {
    fn parse(
        &mut self,
        parser: &mut Parser,
        lexer: &mut Lexer<'a>,
        name: &'a str,
        name_span: Span,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, ()> {
        match name {
            "location" => {
                lexer.expect(Token::Paren('('))?;
                self.location
                    .set(parser.expression(lexer, ctx)?, name_span)?;
                lexer.next_if(Token::Separator(','));
                lexer.expect(Token::Paren(')'))?;
            }
            "builtin" => {
                lexer.expect(Token::Paren('('))?;
                let (raw, span) = lexer.next_ident_with_span()?;
                self.built_in.set(
                    conv::map_built_in(&lexer.enable_extensions, raw, span)?,
                    name_span,
                )?;
                lexer.next_if(Token::Separator(','));
                lexer.expect(Token::Paren(')'))?;
            }
            "interpolate" => {
                lexer.expect(Token::Paren('('))?;
                let (raw, span) = lexer.next_ident_with_span()?;
                self.interpolation
                    .set(conv::map_interpolation(raw, span)?, name_span)?;
                if lexer.next_if(Token::Separator(',')) {
                    let (raw, span) = lexer.next_ident_with_span()?;
                    self.sampling
                        .set(conv::map_sampling(raw, span)?, name_span)?;
                }
                lexer.next_if(Token::Separator(','));
                lexer.expect(Token::Paren(')'))?;
            }

            "invariant" => {
                self.invariant.set(true, name_span)?;
            }
            "blend_src" => {
                lexer.require_enable_extension(
                    ImplementedEnableExtension::DualSourceBlending,
                    name_span,
                )?;

                lexer.expect(Token::Paren('('))?;
                self.blend_src
                    .set(parser.expression(lexer, ctx)?, name_span)?;
                lexer.next_if(Token::Separator(','));
                lexer.expect(Token::Paren(')'))?;
            }
            "per_primitive" => {
                lexer.require_enable_extension(
                    ImplementedEnableExtension::WgpuMeshShader,
                    name_span,
                )?;
                self.per_primitive.set((), name_span)?;
            }
            _ => return Err(Box::new(Error::UnknownAttribute(name_span))),
        }
        Ok(())
    }

    fn finish(self, span: Span) -> Result<'a, Option<ast::Binding<'a>>> {
        match (
            self.location.value,
            self.built_in.value,
            self.interpolation.value,
            self.sampling.value,
            self.invariant.value.unwrap_or_default(),
            self.blend_src.value,
            self.per_primitive.value,
        ) {
            (None, None, None, None, false, None, None) => Ok(None),
            (Some(location), None, interpolation, sampling, false, blend_src, per_primitive) => {
                // Before handing over the completed `Module`, we call
                // `apply_default_interpolation` to ensure that the interpolation and
                // sampling have been explicitly specified on all vertex shader output and fragment
                // shader input user bindings, so leaving them potentially `None` here is fine.
                Ok(Some(ast::Binding::Location {
                    location,
                    interpolation,
                    sampling,
                    blend_src,
                    per_primitive: per_primitive.is_some(),
                }))
            }
            (None, Some(crate::BuiltIn::Position { .. }), None, None, invariant, None, None) => {
                Ok(Some(ast::Binding::BuiltIn(crate::BuiltIn::Position {
                    invariant,
                })))
            }
            (None, Some(built_in), None, None, false, None, None) => {
                Ok(Some(ast::Binding::BuiltIn(built_in)))
            }
            (_, _, _, _, _, _, _) => Err(Box::new(Error::InconsistentBinding(span))),
        }
    }
}

/// Configuration for the whole parser run.
pub struct Options {
    /// Controls whether the parser should parse doc comments.
    pub parse_doc_comments: bool,
    /// Capabilities to enable during parsing.
    pub capabilities: crate::valid::Capabilities,
}

impl Options {
    /// Creates a new default [`Options`].
    pub const fn new() -> Self {
        Options {
            parse_doc_comments: false,
            capabilities: crate::valid::Capabilities::all(),
        }
    }
}

pub struct Parser {
    rules: Vec<(Rule, usize)>,
    recursion_depth: u32,
}

impl Parser {
    pub const fn new() -> Self {
        Parser {
            rules: Vec::new(),
            recursion_depth: 0,
        }
    }

    fn reset(&mut self) {
        self.rules.clear();
        self.recursion_depth = 0;
    }

    fn push_rule_span(&mut self, rule: Rule, lexer: &mut Lexer<'_>) {
        self.rules.push((rule, lexer.start_byte_offset()));
    }

    fn pop_rule_span(&mut self, lexer: &Lexer<'_>) -> Span {
        let (_, initial) = self.rules.pop().unwrap();
        lexer.span_from(initial)
    }

    fn peek_rule_span(&mut self, lexer: &Lexer<'_>) -> Span {
        let &(_, initial) = self.rules.last().unwrap();
        lexer.span_from(initial)
    }

    fn race_rules(&self, rule0: Rule, rule1: Rule) -> Option<Rule> {
        Some(
            self.rules
                .iter()
                .rev()
                .find(|&x| x.0 == rule0 || x.0 == rule1)?
                .0,
        )
    }

    fn track_recursion<'a, F, R>(&mut self, f: F) -> Result<'a, R>
    where
        F: FnOnce(&mut Self) -> Result<'a, R>,
    {
        self.recursion_depth += 1;
        if self.recursion_depth >= 256 {
            return Err(Box::new(Error::Internal("Parser recursion limit exceeded")));
        }
        let ret = f(self);
        self.recursion_depth -= 1;
        ret
    }

    fn switch_value<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, ast::SwitchValue<'a>> {
        if lexer.next_if(Token::Word("default")) {
            return Ok(ast::SwitchValue::Default);
        }

        let expr = self.expression(lexer, ctx)?;
        Ok(ast::SwitchValue::Expr(expr))
    }

    /// Expects `name` to be consumed (not in lexer).
    fn arguments<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Vec<Handle<ast::Expression<'a>>>> {
        self.push_rule_span(Rule::EnclosedExpr, lexer);
        lexer.open_arguments()?;
        let mut arguments = Vec::new();
        loop {
            if !arguments.is_empty() {
                if !lexer.next_argument()? {
                    break;
                }
            } else if lexer.next_if(Token::Paren(')')) {
                break;
            }
            let arg = self.expression(lexer, ctx)?;
            arguments.push(arg);
        }

        self.pop_rule_span(lexer);
        Ok(arguments)
    }

    fn enclosed_expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        self.push_rule_span(Rule::EnclosedExpr, lexer);
        let expr = self.expression(lexer, ctx)?;
        self.pop_rule_span(lexer);
        Ok(expr)
    }

    fn ident_expr<'a>(
        &mut self,
        name: &'a str,
        name_span: Span,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> ast::IdentExpr<'a> {
        match ctx.local_table.lookup(name) {
            Some(&local) => ast::IdentExpr::Local(local),
            None => {
                ctx.unresolved.insert(ast::Dependency {
                    ident: name,
                    usage: name_span,
                });
                ast::IdentExpr::Unresolved(name)
            }
        }
    }

    fn primary_expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        token: TokenSpan<'a>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        self.push_rule_span(Rule::PrimaryExpr, lexer);

        const fn literal_ray_flag<'b>(flag: crate::RayFlag) -> ast::Expression<'b> {
            ast::Expression::Literal(ast::Literal::Number(Number::U32(flag.bits())))
        }
        const fn literal_ray_intersection<'b>(
            intersection: crate::RayQueryIntersection,
        ) -> ast::Expression<'b> {
            ast::Expression::Literal(ast::Literal::Number(Number::U32(intersection as u32)))
        }

        let expr = match token {
            (Token::Paren('('), _) => {
                let expr = self.enclosed_expression(lexer, ctx)?;
                lexer.expect(Token::Paren(')'))?;
                self.pop_rule_span(lexer);
                return Ok(expr);
            }
            (Token::Word("true"), _) => ast::Expression::Literal(ast::Literal::Bool(true)),
            (Token::Word("false"), _) => ast::Expression::Literal(ast::Literal::Bool(false)),
            (Token::Number(res), span) => {
                let num = res.map_err(|err| Error::BadNumber(span, err))?;

                if let Some(enable_extension) = num.requires_enable_extension() {
                    lexer.require_enable_extension(enable_extension, span)?;
                }

                ast::Expression::Literal(ast::Literal::Number(num))
            }
            (Token::Word("RAY_FLAG_NONE"), _) => literal_ray_flag(crate::RayFlag::empty()),
            (Token::Word("RAY_FLAG_FORCE_OPAQUE"), _) => {
                literal_ray_flag(crate::RayFlag::FORCE_OPAQUE)
            }
            (Token::Word("RAY_FLAG_FORCE_NO_OPAQUE"), _) => {
                literal_ray_flag(crate::RayFlag::FORCE_NO_OPAQUE)
            }
            (Token::Word("RAY_FLAG_TERMINATE_ON_FIRST_HIT"), _) => {
                literal_ray_flag(crate::RayFlag::TERMINATE_ON_FIRST_HIT)
            }
            (Token::Word("RAY_FLAG_SKIP_CLOSEST_HIT_SHADER"), _) => {
                literal_ray_flag(crate::RayFlag::SKIP_CLOSEST_HIT_SHADER)
            }
            (Token::Word("RAY_FLAG_CULL_BACK_FACING"), _) => {
                literal_ray_flag(crate::RayFlag::CULL_BACK_FACING)
            }
            (Token::Word("RAY_FLAG_CULL_FRONT_FACING"), _) => {
                literal_ray_flag(crate::RayFlag::CULL_FRONT_FACING)
            }
            (Token::Word("RAY_FLAG_CULL_OPAQUE"), _) => {
                literal_ray_flag(crate::RayFlag::CULL_OPAQUE)
            }
            (Token::Word("RAY_FLAG_CULL_NO_OPAQUE"), _) => {
                literal_ray_flag(crate::RayFlag::CULL_NO_OPAQUE)
            }
            (Token::Word("RAY_FLAG_SKIP_TRIANGLES"), _) => {
                literal_ray_flag(crate::RayFlag::SKIP_TRIANGLES)
            }
            (Token::Word("RAY_FLAG_SKIP_AABBS"), _) => literal_ray_flag(crate::RayFlag::SKIP_AABBS),
            (Token::Word("RAY_QUERY_INTERSECTION_NONE"), _) => {
                literal_ray_intersection(crate::RayQueryIntersection::None)
            }
            (Token::Word("RAY_QUERY_INTERSECTION_TRIANGLE"), _) => {
                literal_ray_intersection(crate::RayQueryIntersection::Triangle)
            }
            (Token::Word("RAY_QUERY_INTERSECTION_GENERATED"), _) => {
                literal_ray_intersection(crate::RayQueryIntersection::Generated)
            }
            (Token::Word("RAY_QUERY_INTERSECTION_AABB"), _) => {
                literal_ray_intersection(crate::RayQueryIntersection::Aabb)
            }
            (Token::Word(word), span) => {
                let ident = self.template_elaborated_ident(word, span, lexer, ctx)?;

                if let Token::Paren('(') = lexer.peek().0 {
                    let arguments = self.arguments(lexer, ctx)?;
                    ast::Expression::Call(ast::CallPhrase {
                        function: ident,
                        arguments,
                    })
                } else {
                    ast::Expression::Ident(ident)
                }
            }
            other => {
                return Err(Box::new(Error::Unexpected(
                    other.1,
                    ExpectedToken::PrimaryExpression,
                )))
            }
        };

        self.pop_rule_span(lexer);
        let span = lexer.span_with_start(token.1);
        let expr = ctx.expressions.append(expr, span);
        Ok(expr)
    }

    fn component_or_swizzle_specifier<'a>(
        &mut self,
        expr_start: Span,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        expr: Handle<ast::Expression<'a>>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        let mut expr = expr;

        loop {
            let expression = match lexer.peek().0 {
                Token::Separator('.') => {
                    let _ = lexer.next();
                    let field = lexer.next_ident()?;

                    ast::Expression::Member { base: expr, field }
                }
                Token::Paren('[') => {
                    let _ = lexer.next();
                    let index = self.enclosed_expression(lexer, ctx)?;
                    lexer.expect(Token::Paren(']'))?;

                    ast::Expression::Index { base: expr, index }
                }
                _ => break,
            };

            let span = lexer.span_with_start(expr_start);
            expr = ctx.expressions.append(expression, span);
        }

        Ok(expr)
    }

    /// Parse a `unary_expression`.
    fn unary_expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        self.push_rule_span(Rule::UnaryExpr, lexer);

        enum UnaryOp {
            Negate,
            LogicalNot,
            BitwiseNot,
            Deref,
            AddrOf,
        }

        let mut ops = Vec::new();
        let mut expr;

        loop {
            match lexer.next() {
                (Token::Operation('-'), span) => {
                    ops.push((UnaryOp::Negate, span));
                }
                (Token::Operation('!'), span) => {
                    ops.push((UnaryOp::LogicalNot, span));
                }
                (Token::Operation('~'), span) => {
                    ops.push((UnaryOp::BitwiseNot, span));
                }
                (Token::Operation('*'), span) => {
                    ops.push((UnaryOp::Deref, span));
                }
                (Token::Operation('&'), span) => {
                    ops.push((UnaryOp::AddrOf, span));
                }
                token => {
                    expr = self.singular_expression(lexer, ctx, token)?;
                    break;
                }
            };
        }

        for (op, span) in ops.into_iter().rev() {
            let e = match op {
                UnaryOp::Negate => ast::Expression::Unary {
                    op: crate::UnaryOperator::Negate,
                    expr,
                },
                UnaryOp::LogicalNot => ast::Expression::Unary {
                    op: crate::UnaryOperator::LogicalNot,
                    expr,
                },
                UnaryOp::BitwiseNot => ast::Expression::Unary {
                    op: crate::UnaryOperator::BitwiseNot,
                    expr,
                },
                UnaryOp::Deref => ast::Expression::Deref(expr),
                UnaryOp::AddrOf => ast::Expression::AddrOf(expr),
            };
            let span = lexer.span_with_start(span);
            expr = ctx.expressions.append(e, span);
        }

        self.pop_rule_span(lexer);
        Ok(expr)
    }

    /// Parse a `lhs_expression`.
    ///
    /// LHS expressions only support the `&` and `*` operators and
    /// the `[]` and `.` postfix selectors.
    fn lhs_expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        token: Option<TokenSpan<'a>>,
        expected_token: ExpectedToken<'a>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        self.track_recursion(|this| {
            this.push_rule_span(Rule::LhsExpr, lexer);
            let token = token.unwrap_or_else(|| lexer.next());
            let expr = match token {
                (Token::Operation('*'), _) => {
                    let expr =
                        this.lhs_expression(lexer, ctx, None, ExpectedToken::LhsExpression)?;
                    let expr = ast::Expression::Deref(expr);
                    let span = this.peek_rule_span(lexer);
                    ctx.expressions.append(expr, span)
                }
                (Token::Operation('&'), _) => {
                    let expr =
                        this.lhs_expression(lexer, ctx, None, ExpectedToken::LhsExpression)?;
                    let expr = ast::Expression::AddrOf(expr);
                    let span = this.peek_rule_span(lexer);
                    ctx.expressions.append(expr, span)
                }
                (Token::Paren('('), span) => {
                    let expr =
                        this.lhs_expression(lexer, ctx, None, ExpectedToken::LhsExpression)?;
                    lexer.expect(Token::Paren(')'))?;
                    this.component_or_swizzle_specifier(span, lexer, ctx, expr)?
                }
                (Token::Word(word), span) => {
                    let ident = this.ident_expr(word, span, ctx);
                    let ident = ast::TemplateElaboratedIdent {
                        ident,
                        ident_span: span,
                        template_list: Vec::new(),
                        template_list_span: Span::UNDEFINED,
                    };
                    let ident = ctx.expressions.append(ast::Expression::Ident(ident), span);
                    this.component_or_swizzle_specifier(span, lexer, ctx, ident)?
                }
                (_, span) => {
                    return Err(Box::new(Error::Unexpected(span, expected_token)));
                }
            };

            this.pop_rule_span(lexer);
            Ok(expr)
        })
    }

    /// Parse a `singular_expression`.
    fn singular_expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        token: TokenSpan<'a>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        self.push_rule_span(Rule::SingularExpr, lexer);
        let primary_expr = self.primary_expression(lexer, ctx, token)?;
        let singular_expr =
            self.component_or_swizzle_specifier(token.1, lexer, ctx, primary_expr)?;
        self.pop_rule_span(lexer);

        Ok(singular_expr)
    }

    fn equality_expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        context: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        // equality_expression
        context.parse_binary_op(
            lexer,
            |token| match token {
                Token::LogicalOperation('=') => Some(crate::BinaryOperator::Equal),
                Token::LogicalOperation('!') => Some(crate::BinaryOperator::NotEqual),
                _ => None,
            },
            // relational_expression
            |lexer, context| {
                let enclosing = self.race_rules(Rule::GenericExpr, Rule::EnclosedExpr);
                context.parse_binary_op(
                    lexer,
                    match enclosing {
                        Some(Rule::GenericExpr) => |token| match token {
                            Token::LogicalOperation('<') => Some(crate::BinaryOperator::LessEqual),
                            _ => None,
                        },
                        _ => |token| match token {
                            Token::Paren('<') => Some(crate::BinaryOperator::Less),
                            Token::Paren('>') => Some(crate::BinaryOperator::Greater),
                            Token::LogicalOperation('<') => Some(crate::BinaryOperator::LessEqual),
                            Token::LogicalOperation('>') => {
                                Some(crate::BinaryOperator::GreaterEqual)
                            }
                            _ => None,
                        },
                    },
                    // shift_expression
                    |lexer, context| {
                        context.parse_binary_op(
                            lexer,
                            match enclosing {
                                Some(Rule::GenericExpr) => |token| match token {
                                    Token::ShiftOperation('<') => {
                                        Some(crate::BinaryOperator::ShiftLeft)
                                    }
                                    _ => None,
                                },
                                _ => |token| match token {
                                    Token::ShiftOperation('<') => {
                                        Some(crate::BinaryOperator::ShiftLeft)
                                    }
                                    Token::ShiftOperation('>') => {
                                        Some(crate::BinaryOperator::ShiftRight)
                                    }
                                    _ => None,
                                },
                            },
                            // additive_expression
                            |lexer, context| {
                                context.parse_binary_op(
                                    lexer,
                                    |token| match token {
                                        Token::Operation('+') => Some(crate::BinaryOperator::Add),
                                        Token::Operation('-') => {
                                            Some(crate::BinaryOperator::Subtract)
                                        }
                                        _ => None,
                                    },
                                    // multiplicative_expression
                                    |lexer, context| {
                                        context.parse_binary_op(
                                            lexer,
                                            |token| match token {
                                                Token::Operation('*') => {
                                                    Some(crate::BinaryOperator::Multiply)
                                                }
                                                Token::Operation('/') => {
                                                    Some(crate::BinaryOperator::Divide)
                                                }
                                                Token::Operation('%') => {
                                                    Some(crate::BinaryOperator::Modulo)
                                                }
                                                _ => None,
                                            },
                                            |lexer, context| self.unary_expression(lexer, context),
                                        )
                                    },
                                )
                            },
                        )
                    },
                )
            },
        )
    }

    fn expression<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        context: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Handle<ast::Expression<'a>>> {
        self.push_rule_span(Rule::GeneralExpr, lexer);
        // logical_or_expression
        let handle = context.parse_binary_op(
            lexer,
            |token| match token {
                Token::LogicalOperation('|') => Some(crate::BinaryOperator::LogicalOr),
                _ => None,
            },
            // logical_and_expression
            |lexer, context| {
                context.parse_binary_op(
                    lexer,
                    |token| match token {
                        Token::LogicalOperation('&') => Some(crate::BinaryOperator::LogicalAnd),
                        _ => None,
                    },
                    // inclusive_or_expression
                    |lexer, context| {
                        context.parse_binary_op(
                            lexer,
                            |token| match token {
                                Token::Operation('|') => Some(crate::BinaryOperator::InclusiveOr),
                                _ => None,
                            },
                            // exclusive_or_expression
                            |lexer, context| {
                                context.parse_binary_op(
                                    lexer,
                                    |token| match token {
                                        Token::Operation('^') => {
                                            Some(crate::BinaryOperator::ExclusiveOr)
                                        }
                                        _ => None,
                                    },
                                    // and_expression
                                    |lexer, context| {
                                        context.parse_binary_op(
                                            lexer,
                                            |token| match token {
                                                Token::Operation('&') => {
                                                    Some(crate::BinaryOperator::And)
                                                }
                                                _ => None,
                                            },
                                            |lexer, context| {
                                                self.equality_expression(lexer, context)
                                            },
                                        )
                                    },
                                )
                            },
                        )
                    },
                )
            },
        )?;
        self.pop_rule_span(lexer);
        Ok(handle)
    }

    fn optionally_typed_ident<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, (ast::Ident<'a>, Option<ast::TemplateElaboratedIdent<'a>>)> {
        let name = lexer.next_ident()?;

        let ty = if lexer.next_if(Token::Separator(':')) {
            Some(self.type_specifier(lexer, ctx)?)
        } else {
            None
        };

        Ok((name, ty))
    }

    /// 'var' _disambiguate_template template_list? optionally_typed_ident
    fn variable_decl<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, ast::GlobalVariable<'a>> {
        self.push_rule_span(Rule::VariableDecl, lexer);
        let (template_list, _) = self.maybe_template_list(lexer, ctx)?;
        let (name, ty) = self.optionally_typed_ident(lexer, ctx)?;

        let init = if lexer.next_if(Token::Operation('=')) {
            let handle = self.expression(lexer, ctx)?;
            Some(handle)
        } else {
            None
        };
        lexer.expect(Token::Separator(';'))?;
        self.pop_rule_span(lexer);

        Ok(ast::GlobalVariable {
            name,
            template_list,
            binding: None,
            ty,
            init,
            doc_comments: Vec::new(),
            memory_decorations: crate::MemoryDecorations::empty(),
        })
    }

    fn struct_body<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Vec<ast::StructMember<'a>>> {
        let mut members = Vec::new();
        let mut member_names = FastHashSet::default();

        lexer.expect(Token::Paren('{'))?;
        let mut ready = true;
        while !lexer.next_if(Token::Paren('}')) {
            if !ready {
                return Err(Box::new(Error::Unexpected(
                    lexer.next().1,
                    ExpectedToken::Token(Token::Separator(',')),
                )));
            }

            let doc_comments = lexer.accumulate_doc_comments();

            let (mut size, mut align) = (ParsedAttribute::default(), ParsedAttribute::default());
            self.push_rule_span(Rule::Attribute, lexer);
            let mut bind_parser = BindingParser::default();
            while lexer.next_if(Token::Attribute) {
                match lexer.next_ident_with_span()? {
                    ("size", name_span) => {
                        lexer.expect(Token::Paren('('))?;
                        let expr = self.expression(lexer, ctx)?;
                        lexer.next_if(Token::Separator(','));
                        lexer.expect(Token::Paren(')'))?;
                        size.set(expr, name_span)?;
                    }
                    ("align", name_span) => {
                        lexer.expect(Token::Paren('('))?;
                        let expr = self.expression(lexer, ctx)?;
                        lexer.next_if(Token::Separator(','));
                        lexer.expect(Token::Paren(')'))?;
                        align.set(expr, name_span)?;
                    }
                    (word, word_span) => bind_parser.parse(self, lexer, word, word_span, ctx)?,
                }
            }

            let bind_span = self.pop_rule_span(lexer);
            let binding = bind_parser.finish(bind_span)?;

            let name = lexer.next_ident()?;
            lexer.expect(Token::Separator(':'))?;
            let ty = self.type_specifier(lexer, ctx)?;
            ready = lexer.next_if(Token::Separator(','));

            members.push(ast::StructMember {
                name,
                ty,
                binding,
                size: size.value,
                align: align.value,
                doc_comments,
            });

            if !member_names.insert(name.name) {
                return Err(Box::new(Error::Redefinition {
                    previous: members
                        .iter()
                        .find(|x| x.name.name == name.name)
                        .map(|x| x.name.span)
                        .unwrap(),
                    current: name.span,
                }));
            }
        }

        Ok(members)
    }

    fn maybe_template_list<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, (Vec<Handle<ast::Expression<'a>>>, Span)> {
        let start = lexer.start_byte_offset();
        if lexer.next_if(Token::TemplateArgsStart) {
            let mut args = Vec::new();
            args.push(self.expression(lexer, ctx)?);
            while lexer.next_if(Token::Separator(',')) && lexer.peek().0 != Token::TemplateArgsEnd {
                args.push(self.expression(lexer, ctx)?);
            }
            lexer.expect(Token::TemplateArgsEnd)?;
            let span = lexer.span_from(start);
            Ok((args, span))
        } else {
            Ok((Vec::new(), Span::UNDEFINED))
        }
    }

    fn template_elaborated_ident<'a>(
        &mut self,
        word: &'a str,
        span: Span,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, ast::TemplateElaboratedIdent<'a>> {
        let ident = self.ident_expr(word, span, ctx);
        let (template_list, template_list_span) = self.maybe_template_list(lexer, ctx)?;
        Ok(ast::TemplateElaboratedIdent {
            ident,
            ident_span: span,
            template_list,
            template_list_span,
        })
    }

    fn type_specifier<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, ast::TemplateElaboratedIdent<'a>> {
        let (name, span) = lexer.next_ident_with_span()?;
        self.template_elaborated_ident(name, span, lexer, ctx)
    }

    /// Parses assignment, increment and decrement statements
    ///
    /// This does not consume or require a final `;` token. In the update
    /// expression of a C-style `for` loop header, there is no terminating `;`.
    fn variable_updating_statement<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        block: &mut ast::Block<'a>,
        token: TokenSpan<'a>,
        expected_token: ExpectedToken<'a>,
    ) -> Result<'a, ()> {
        match token {
            (Token::Word("_"), span) => {
                lexer.expect(Token::Operation('='))?;
                let expr = self.expression(lexer, ctx)?;
                let span = lexer.span_with_start(span);
                block.stmts.push(ast::Statement {
                    kind: ast::StatementKind::Phony(expr),
                    span,
                });
                return Ok(());
            }
            _ => {}
        }
        let target = self.lhs_expression(lexer, ctx, Some(token), expected_token)?;

        let (op, value) = match lexer.next() {
            (Token::Operation('='), _) => {
                let value = self.expression(lexer, ctx)?;
                (None, value)
            }
            (Token::AssignmentOperation(c), _) => {
                use crate::BinaryOperator as Bo;
                let op = match c {
                    '<' => Bo::ShiftLeft,
                    '>' => Bo::ShiftRight,
                    '+' => Bo::Add,
                    '-' => Bo::Subtract,
                    '*' => Bo::Multiply,
                    '/' => Bo::Divide,
                    '%' => Bo::Modulo,
                    '&' => Bo::And,
                    '|' => Bo::InclusiveOr,
                    '^' => Bo::ExclusiveOr,
                    // Note: `consume_token` shouldn't produce any other assignment ops
                    _ => unreachable!(),
                };

                let value = self.expression(lexer, ctx)?;
                (Some(op), value)
            }
            op_token @ (Token::IncrementOperation | Token::DecrementOperation, _) => {
                let op = match op_token.0 {
                    Token::IncrementOperation => ast::StatementKind::Increment,
                    Token::DecrementOperation => ast::StatementKind::Decrement,
                    _ => unreachable!(),
                };

                let span = lexer.span_with_start(token.1);
                block.stmts.push(ast::Statement {
                    kind: op(target),
                    span,
                });
                return Ok(());
            }
            (_, span) => return Err(Box::new(Error::Unexpected(span, ExpectedToken::Assignment))),
        };

        let span = lexer.span_with_start(token.1);
        block.stmts.push(ast::Statement {
            kind: ast::StatementKind::Assign { target, op, value },
            span,
        });
        Ok(())
    }

    /// Parse a function call statement.
    ///
    /// This assumes that `token` has been consumed from the lexer.
    ///
    /// This does not consume or require a final `;` token. In the update
    /// expression of a C-style `for` loop header, there is no terminating `;`.
    fn maybe_func_call_statement<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        context: &mut ExpressionContext<'a, '_, '_>,
        block: &mut ast::Block<'a>,
        token: TokenSpan<'a>,
    ) -> Result<'a, bool> {
        let (name, name_span) = match token {
            (Token::Word(name), span) => (name, span),
            _ => return Ok(false),
        };
        let ident = self.template_elaborated_ident(name, name_span, lexer, context)?;
        if ident.template_list.is_empty() && !matches!(lexer.peek(), (Token::Paren('('), _)) {
            return Ok(false);
        }

        self.push_rule_span(Rule::SingularExpr, lexer);

        let arguments = self.arguments(lexer, context)?;
        let span = lexer.span_with_start(name_span);

        block.stmts.push(ast::Statement {
            kind: ast::StatementKind::Call(ast::CallPhrase {
                function: ident,
                arguments,
            }),
            span,
        });

        self.pop_rule_span(lexer);

        Ok(true)
    }

    /// Parses func_call_statement and variable_updating_statement
    ///
    /// This does not consume or require a final `;` token. In the update
    /// expression of a C-style `for` loop header, there is no terminating `;`.
    fn func_call_or_variable_updating_statement<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        context: &mut ExpressionContext<'a, '_, '_>,
        block: &mut ast::Block<'a>,
        token: TokenSpan<'a>,
        expected_token: ExpectedToken<'a>,
    ) -> Result<'a, ()> {
        if !self.maybe_func_call_statement(lexer, context, block, token)? {
            self.variable_updating_statement(lexer, context, block, token, expected_token)?;
        }
        Ok(())
    }

    /// Parses variable_or_value_statement, func_call_statement and variable_updating_statement.
    ///
    /// This is equivalent to the `for_init` production in the WGSL spec,
    /// but it's also used for parsing these forms when they appear within a block,
    /// hence the longer name.
    ///
    /// This does not consume the following `;` token.
    fn variable_or_value_or_func_call_or_variable_updating_statement<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        block: &mut ast::Block<'a>,
        token: TokenSpan<'a>,
        expected_token: ExpectedToken<'a>,
    ) -> Result<'a, ()> {
        let local_decl = match token {
            (Token::Word("let"), _) => {
                let (name, given_ty) = self.optionally_typed_ident(lexer, ctx)?;

                lexer.expect(Token::Operation('='))?;
                let expr_id = self.expression(lexer, ctx)?;

                let handle = ctx.declare_local(name)?;
                ast::LocalDecl::Let(ast::Let {
                    name,
                    ty: given_ty,
                    init: expr_id,
                    handle,
                })
            }
            (Token::Word("const"), _) => {
                let (name, given_ty) = self.optionally_typed_ident(lexer, ctx)?;

                lexer.expect(Token::Operation('='))?;
                let expr_id = self.expression(lexer, ctx)?;

                let handle = ctx.declare_local(name)?;
                ast::LocalDecl::Const(ast::LocalConst {
                    name,
                    ty: given_ty,
                    init: expr_id,
                    handle,
                })
            }
            (Token::Word("var"), _) => {
                if lexer.next_if(Token::TemplateArgsStart) {
                    let (class_str, span) = lexer.next_ident_with_span()?;
                    if class_str != "function" {
                        return Err(Box::new(Error::InvalidLocalVariableAddressSpace(span)));
                    }
                    lexer.expect(Token::TemplateArgsEnd)?;
                }

                let (name, ty) = self.optionally_typed_ident(lexer, ctx)?;

                let init = if lexer.next_if(Token::Operation('=')) {
                    let init = self.expression(lexer, ctx)?;
                    Some(init)
                } else {
                    None
                };

                let handle = ctx.declare_local(name)?;
                ast::LocalDecl::Var(ast::LocalVariable {
                    name,
                    ty,
                    init,
                    handle,
                })
            }
            token => {
                return self.func_call_or_variable_updating_statement(
                    lexer,
                    ctx,
                    block,
                    token,
                    expected_token,
                );
            }
        };

        let span = lexer.span_with_start(token.1);
        block.stmts.push(ast::Statement {
            kind: ast::StatementKind::LocalDecl(local_decl),
            span,
        });

        Ok(())
    }

    fn statement<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        block: &mut ast::Block<'a>,
        brace_nesting_level: u8,
    ) -> Result<'a, ()> {
        self.track_recursion(|this| {
            this.push_rule_span(Rule::Statement, lexer);

            // We peek here instead of eagerly getting the next token since
            // `Parser::block` expects its first token to be `{`.
            //
            // Most callers have a single path leading to the start of the block;
            // `statement` is the only exception where there are multiple choices.
            match lexer.peek() {
                (token, _) if is_start_of_compound_statement(token) => {
                    let (inner, span) = this.block(lexer, ctx, brace_nesting_level)?;
                    block.stmts.push(ast::Statement {
                        kind: ast::StatementKind::Block(inner),
                        span,
                    });
                    this.pop_rule_span(lexer);
                    return Ok(());
                }
                _ => {}
            }

            let kind = match lexer.next() {
                (Token::Separator(';'), _) => {
                    this.pop_rule_span(lexer);
                    return Ok(());
                }
                (Token::Word("return"), _) => {
                    let value = if lexer.peek().0 != Token::Separator(';') {
                        let handle = this.expression(lexer, ctx)?;
                        Some(handle)
                    } else {
                        None
                    };
                    lexer.expect(Token::Separator(';'))?;
                    ast::StatementKind::Return { value }
                }
                (Token::Word("if"), _) => {
                    let condition = this.expression(lexer, ctx)?;

                    let accept = this.block(lexer, ctx, brace_nesting_level)?.0;

                    let mut elsif_stack = Vec::new();
                    let mut elseif_span_start = lexer.start_byte_offset();
                    let mut reject = loop {
                        if !lexer.next_if(Token::Word("else")) {
                            break ast::Block::default();
                        }

                        if !lexer.next_if(Token::Word("if")) {
                            // ... else { ... }
                            break this.block(lexer, ctx, brace_nesting_level)?.0;
                        }

                        // ... else if (...) { ... }
                        let other_condition = this.expression(lexer, ctx)?;
                        let other_block = this.block(lexer, ctx, brace_nesting_level)?;
                        elsif_stack.push((elseif_span_start, other_condition, other_block));
                        elseif_span_start = lexer.start_byte_offset();
                    };

                    // reverse-fold the else-if blocks
                    //Note: we may consider uplifting this to the IR
                    for (other_span_start, other_cond, other_block) in elsif_stack.into_iter().rev()
                    {
                        let sub_stmt = ast::StatementKind::If {
                            condition: other_cond,
                            accept: other_block.0,
                            reject,
                        };
                        reject = ast::Block::default();
                        let span = lexer.span_from(other_span_start);
                        reject.stmts.push(ast::Statement {
                            kind: sub_stmt,
                            span,
                        })
                    }

                    ast::StatementKind::If {
                        condition,
                        accept,
                        reject,
                    }
                }
                (Token::Word("switch"), _) => {
                    let selector = this.expression(lexer, ctx)?;
                    let brace_span = lexer.expect_span(Token::Paren('{'))?;
                    let brace_nesting_level =
                        Self::increase_brace_nesting(brace_nesting_level, brace_span)?;
                    let mut cases = Vec::new();

                    loop {
                        // cases + default
                        match lexer.next() {
                            (Token::Word("case"), _) => {
                                // parse a list of values
                                let value = loop {
                                    let value = this.switch_value(lexer, ctx)?;
                                    if lexer.next_if(Token::Separator(',')) {
                                        // list of values ends with ':' or a compound statement
                                        let next_token = lexer.peek().0;
                                        if next_token == Token::Separator(':')
                                            || is_start_of_compound_statement(next_token)
                                        {
                                            break value;
                                        }
                                    } else {
                                        break value;
                                    }
                                    cases.push(ast::SwitchCase {
                                        value,
                                        body: ast::Block::default(),
                                        fall_through: true,
                                    });
                                };

                                lexer.next_if(Token::Separator(':'));

                                let body = this.block(lexer, ctx, brace_nesting_level)?.0;

                                cases.push(ast::SwitchCase {
                                    value,
                                    body,
                                    fall_through: false,
                                });
                            }
                            (Token::Word("default"), _) => {
                                lexer.next_if(Token::Separator(':'));
                                let body = this.block(lexer, ctx, brace_nesting_level)?.0;
                                cases.push(ast::SwitchCase {
                                    value: ast::SwitchValue::Default,
                                    body,
                                    fall_through: false,
                                });
                            }
                            (Token::Paren('}'), _) => break,
                            (_, span) => {
                                return Err(Box::new(Error::Unexpected(
                                    span,
                                    ExpectedToken::SwitchItem,
                                )))
                            }
                        }
                    }

                    ast::StatementKind::Switch { selector, cases }
                }
                (Token::Word("loop"), _) => this.r#loop(lexer, ctx, brace_nesting_level)?,
                (Token::Word("while"), _) => {
                    let mut body = ast::Block::default();

                    let (condition, span) =
                        lexer.capture_span(|lexer| this.expression(lexer, ctx))?;
                    let mut reject = ast::Block::default();
                    reject.stmts.push(ast::Statement {
                        kind: ast::StatementKind::Break,
                        span,
                    });

                    body.stmts.push(ast::Statement {
                        kind: ast::StatementKind::If {
                            condition,
                            accept: ast::Block::default(),
                            reject,
                        },
                        span,
                    });

                    let (block, span) = this.block(lexer, ctx, brace_nesting_level)?;
                    body.stmts.push(ast::Statement {
                        kind: ast::StatementKind::Block(block),
                        span,
                    });

                    ast::StatementKind::Loop {
                        body,
                        continuing: ast::Block::default(),
                        break_if: None,
                    }
                }
                (Token::Word("for"), _) => {
                    lexer.expect(Token::Paren('('))?;

                    ctx.local_table.push_scope();

                    if !lexer.next_if(Token::Separator(';')) {
                        let token = lexer.next();
                        this.variable_or_value_or_func_call_or_variable_updating_statement(
                            lexer,
                            ctx,
                            block,
                            token,
                            ExpectedToken::ForInit,
                        )?;
                        lexer.expect(Token::Separator(';'))?;
                    };

                    let mut body = ast::Block::default();
                    if !lexer.next_if(Token::Separator(';')) {
                        let (condition, span) = lexer.capture_span(|lexer| -> Result<'_, _> {
                            let condition = this.expression(lexer, ctx)?;
                            lexer.expect(Token::Separator(';'))?;
                            Ok(condition)
                        })?;
                        let mut reject = ast::Block::default();
                        reject.stmts.push(ast::Statement {
                            kind: ast::StatementKind::Break,
                            span,
                        });
                        body.stmts.push(ast::Statement {
                            kind: ast::StatementKind::If {
                                condition,
                                accept: ast::Block::default(),
                                reject,
                            },
                            span,
                        });
                    };

                    let mut continuing = ast::Block::default();
                    if !lexer.next_if(Token::Paren(')')) {
                        let token = lexer.next();
                        this.func_call_or_variable_updating_statement(
                            lexer,
                            ctx,
                            &mut continuing,
                            token,
                            ExpectedToken::ForUpdate,
                        )?;
                        lexer.expect(Token::Paren(')'))?;
                    }

                    let (block, span) = this.block(lexer, ctx, brace_nesting_level)?;
                    body.stmts.push(ast::Statement {
                        kind: ast::StatementKind::Block(block),
                        span,
                    });

                    ctx.local_table.pop_scope();

                    ast::StatementKind::Loop {
                        body,
                        continuing,
                        break_if: None,
                    }
                }
                (Token::Word("break"), span) => {
                    // Check if the next token is an `if`, this indicates
                    // that the user tried to type out a `break if` which
                    // is illegal in this position.
                    let (peeked_token, peeked_span) = lexer.peek();
                    if let Token::Word("if") = peeked_token {
                        let span = span.until(&peeked_span);
                        return Err(Box::new(Error::InvalidBreakIf(span)));
                    }
                    lexer.expect(Token::Separator(';'))?;
                    ast::StatementKind::Break
                }
                (Token::Word("continue"), _) => {
                    lexer.expect(Token::Separator(';'))?;
                    ast::StatementKind::Continue
                }
                (Token::Word("discard"), _) => {
                    lexer.expect(Token::Separator(';'))?;
                    ast::StatementKind::Kill
                }
                // https://www.w3.org/TR/WGSL/#const-assert-statement
                (Token::Word("const_assert"), _) => {
                    // parentheses are optional
                    let paren = lexer.next_if(Token::Paren('('));

                    let condition = this.expression(lexer, ctx)?;

                    if paren {
                        lexer.expect(Token::Paren(')'))?;
                    }
                    lexer.expect(Token::Separator(';'))?;
                    ast::StatementKind::ConstAssert(condition)
                }
                token => {
                    this.variable_or_value_or_func_call_or_variable_updating_statement(
                        lexer,
                        ctx,
                        block,
                        token,
                        ExpectedToken::Statement,
                    )?;
                    lexer.expect(Token::Separator(';'))?;
                    this.pop_rule_span(lexer);
                    return Ok(());
                }
            };

            let span = this.pop_rule_span(lexer);
            block.stmts.push(ast::Statement { kind, span });

            Ok(())
        })
    }

    fn r#loop<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        brace_nesting_level: u8,
    ) -> Result<'a, ast::StatementKind<'a>> {
        let mut body = ast::Block::default();
        let mut continuing = ast::Block::default();
        let mut break_if = None;

        let brace_span = lexer.expect_span(Token::Paren('{'))?;
        let brace_nesting_level = Self::increase_brace_nesting(brace_nesting_level, brace_span)?;

        ctx.local_table.push_scope();

        loop {
            if lexer.next_if(Token::Word("continuing")) {
                // Branch for the `continuing` block, this must be
                // the last thing in the loop body

                // Expect a opening brace to start the continuing block
                let brace_span = lexer.expect_span(Token::Paren('{'))?;
                let brace_nesting_level =
                    Self::increase_brace_nesting(brace_nesting_level, brace_span)?;
                loop {
                    if lexer.next_if(Token::Word("break")) {
                        // Branch for the `break if` statement, this statement
                        // has the form `break if <expr>;` and must be the last
                        // statement in a continuing block

                        // The break must be followed by an `if` to form
                        // the break if
                        lexer.expect(Token::Word("if"))?;

                        let condition = self.expression(lexer, ctx)?;
                        // Set the condition of the break if to the newly parsed
                        // expression
                        break_if = Some(condition);

                        // Expect a semicolon to close the statement
                        lexer.expect(Token::Separator(';'))?;
                        // Expect a closing brace to close the continuing block,
                        // since the break if must be the last statement
                        lexer.expect(Token::Paren('}'))?;
                        // Stop parsing the continuing block
                        break;
                    } else if lexer.next_if(Token::Paren('}')) {
                        // If we encounter a closing brace it means we have reached
                        // the end of the continuing block and should stop processing
                        break;
                    } else {
                        // Otherwise try to parse a statement
                        self.statement(lexer, ctx, &mut continuing, brace_nesting_level)?;
                    }
                }
                // Since the continuing block must be the last part of the loop body,
                // we expect to see a closing brace to end the loop body
                lexer.expect(Token::Paren('}'))?;
                break;
            }
            if lexer.next_if(Token::Paren('}')) {
                // If we encounter a closing brace it means we have reached
                // the end of the loop body and should stop processing
                break;
            }
            // Otherwise try to parse a statement
            self.statement(lexer, ctx, &mut body, brace_nesting_level)?;
        }

        ctx.local_table.pop_scope();

        Ok(ast::StatementKind::Loop {
            body,
            continuing,
            break_if,
        })
    }

    /// compound_statement
    fn block<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
        brace_nesting_level: u8,
    ) -> Result<'a, (ast::Block<'a>, Span)> {
        self.push_rule_span(Rule::Block, lexer);

        ctx.local_table.push_scope();

        let mut diagnostic_filters = DiagnosticFilterMap::new();

        self.push_rule_span(Rule::Attribute, lexer);
        while lexer.next_if(Token::Attribute) {
            let (name, name_span) = lexer.next_ident_with_span()?;
            if let Some(DirectiveKind::Diagnostic) = DirectiveKind::from_ident(name) {
                let filter = self.diagnostic_filter(lexer)?;
                let span = self.peek_rule_span(lexer);
                diagnostic_filters
                    .add(filter, span, ShouldConflictOnFullDuplicate::Yes)
                    .map_err(|e| Box::new(e.into()))?;
            } else {
                return Err(Box::new(Error::Unexpected(
                    name_span,
                    ExpectedToken::DiagnosticAttribute,
                )));
            }
        }
        self.pop_rule_span(lexer);

        if !diagnostic_filters.is_empty() {
            return Err(Box::new(
                Error::DiagnosticAttributeNotYetImplementedAtParseSite {
                    site_name_plural: "compound statements",
                    spans: diagnostic_filters.spans().collect(),
                },
            ));
        }

        let brace_span = lexer.expect_span(Token::Paren('{'))?;
        let brace_nesting_level = Self::increase_brace_nesting(brace_nesting_level, brace_span)?;
        let mut block = ast::Block::default();
        while !lexer.next_if(Token::Paren('}')) {
            self.statement(lexer, ctx, &mut block, brace_nesting_level)?;
        }

        ctx.local_table.pop_scope();

        let span = self.pop_rule_span(lexer);
        Ok((block, span))
    }

    fn varying_binding<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        ctx: &mut ExpressionContext<'a, '_, '_>,
    ) -> Result<'a, Option<ast::Binding<'a>>> {
        let mut bind_parser = BindingParser::default();
        self.push_rule_span(Rule::Attribute, lexer);

        while lexer.next_if(Token::Attribute) {
            let (word, span) = lexer.next_ident_with_span()?;
            bind_parser.parse(self, lexer, word, span, ctx)?;
        }

        let span = self.pop_rule_span(lexer);
        bind_parser.finish(span)
    }

    fn function_decl<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        diagnostic_filter_leaf: Option<Handle<DiagnosticFilterNode>>,
        must_use: Option<Span>,
        out: &mut ast::TranslationUnit<'a>,
        dependencies: &mut FastIndexSet<ast::Dependency<'a>>,
    ) -> Result<'a, ast::Function<'a>> {
        self.push_rule_span(Rule::FunctionDecl, lexer);
        // read function name
        let fun_name = lexer.next_ident()?;

        let mut locals = Arena::new();

        let mut ctx = ExpressionContext {
            expressions: &mut out.expressions,
            local_table: &mut SymbolTable::default(),
            locals: &mut locals,
            unresolved: dependencies,
        };

        // start a scope that contains arguments as well as the function body
        ctx.local_table.push_scope();
        // Reduce lookup scope to parse the parameter list and return type
        // avoiding identifier lookup to match newly declared param names.
        ctx.local_table.reduce_lookup_scope();

        // read parameter list
        let mut arguments = Vec::new();
        lexer.expect(Token::Paren('('))?;
        let mut ready = true;
        while !lexer.next_if(Token::Paren(')')) {
            if !ready {
                return Err(Box::new(Error::Unexpected(
                    lexer.next().1,
                    ExpectedToken::Token(Token::Separator(',')),
                )));
            }
            let binding = self.varying_binding(lexer, &mut ctx)?;

            let param_name = lexer.next_ident()?;

            lexer.expect(Token::Separator(':'))?;
            let param_type = self.type_specifier(lexer, &mut ctx)?;

            let handle = ctx.declare_local(param_name)?;
            arguments.push(ast::FunctionArgument {
                name: param_name,
                ty: param_type,
                binding,
                handle,
            });
            ready = lexer.next_if(Token::Separator(','));
        }
        // read return type
        let result = if lexer.next_if(Token::Arrow) {
            let binding = self.varying_binding(lexer, &mut ctx)?;
            let ty = self.type_specifier(lexer, &mut ctx)?;
            let must_use = must_use.is_some();
            Some(ast::FunctionResult {
                ty,
                binding,
                must_use,
            })
        } else if let Some(must_use) = must_use {
            return Err(Box::new(Error::FunctionMustUseReturnsVoid(
                must_use,
                self.peek_rule_span(lexer),
            )));
        } else {
            None
        };

        ctx.local_table.reset_lookup_scope();

        // do not use `self.block` here, since we must not push a new scope
        lexer.expect(Token::Paren('{'))?;
        let brace_nesting_level = 1;
        let mut body = ast::Block::default();
        while !lexer.next_if(Token::Paren('}')) {
            self.statement(lexer, &mut ctx, &mut body, brace_nesting_level)?;
        }

        ctx.local_table.pop_scope();

        let fun = ast::Function {
            entry_point: None,
            name: fun_name,
            arguments,
            result,
            body,
            diagnostic_filter_leaf,
            doc_comments: Vec::new(),
        };

        // done
        self.pop_rule_span(lexer);

        Ok(fun)
    }

    fn directive_ident_list<'a>(
        &self,
        lexer: &mut Lexer<'a>,
        handler: impl FnMut(&'a str, Span) -> Result<'a, ()>,
    ) -> Result<'a, ()> {
        let mut handler = handler;
        'next_arg: loop {
            let (ident, span) = lexer.next_ident_with_span()?;
            handler(ident, span)?;

            let expected_token = match lexer.peek().0 {
                Token::Separator(',') => {
                    let _ = lexer.next();
                    if matches!(lexer.peek().0, Token::Word(..)) {
                        continue 'next_arg;
                    }
                    ExpectedToken::AfterIdentListComma
                }
                _ => ExpectedToken::AfterIdentListArg,
            };

            if !matches!(lexer.next().0, Token::Separator(';')) {
                return Err(Box::new(Error::Unexpected(span, expected_token)));
            }

            break Ok(());
        }
    }

    fn global_decl<'a>(
        &mut self,
        lexer: &mut Lexer<'a>,
        out: &mut ast::TranslationUnit<'a>,
    ) -> Result<'a, ()> {
        let doc_comments = lexer.accumulate_doc_comments();

        // read attributes
        let mut binding = None;
        let mut stage = ParsedAttribute::default();
        // Span in case we need to report an error for a shader stage missing something (e.g. its workgroup size).
        // Doesn't need to be set in the vertex and fragment stages because they don't have errors like that.
        let mut shader_stage_error_span = Span::new(0, 0);
        let mut workgroup_size = ParsedAttribute::default();
        let mut early_depth_test = ParsedAttribute::default();
        let (mut bind_index, mut bind_group) =
            (ParsedAttribute::default(), ParsedAttribute::default());
        let mut id = ParsedAttribute::default();
        // the payload variable for a mesh shader
        let mut payload = ParsedAttribute::default();
        // the incoming payload from a traceRay call
        let mut incoming_payload = ParsedAttribute::default();
        let mut mesh_output = ParsedAttribute::default();

        let mut must_use: ParsedAttribute<Span> = ParsedAttribute::default();
        let mut memory_decorations = crate::MemoryDecorations::empty();

        let mut dependencies = FastIndexSet::default();
        let mut ctx = ExpressionContext {
            expressions: &mut out.expressions,
            local_table: &mut SymbolTable::default(),
            locals: &mut Arena::new(),
            unresolved: &mut dependencies,
        };
        let mut diagnostic_filters = DiagnosticFilterMap::new();
        let ensure_no_diag_attrs = |on_what, filters: DiagnosticFilterMap| -> Result<()> {
            if filters.is_empty() {
                Ok(())
            } else {
                Err(Box::new(Error::DiagnosticAttributeNotSupported {
                    on_what,
                    spans: filters.spans().collect(),
                }))
            }
        };

        self.push_rule_span(Rule::Attribute, lexer);
        while lexer.next_if(Token::Attribute) {
            let (name, name_span) = lexer.next_ident_with_span()?;
            if let Some(DirectiveKind::Diagnostic) = DirectiveKind::from_ident(name) {
                let filter = self.diagnostic_filter(lexer)?;
                let span = self.peek_rule_span(lexer);
                diagnostic_filters
                    .add(filter, span, ShouldConflictOnFullDuplicate::Yes)
                    .map_err(|e| Box::new(e.into()))?;
                continue;
            }
            match name {
                "binding" => {
                    lexer.expect(Token::Paren('('))?;
                    bind_index.set(self.expression(lexer, &mut ctx)?, name_span)?;
                    lexer.next_if(Token::Separator(','));
                    lexer.expect(Token::Paren(')'))?;
                }
                "group" => {
                    lexer.expect(Token::Paren('('))?;
                    bind_group.set(self.expression(lexer, &mut ctx)?, name_span)?;
                    lexer.next_if(Token::Separator(','));
                    lexer.expect(Token::Paren(')'))?;
                }
                "id" => {
                    lexer.expect(Token::Paren('('))?;
                    id.set(self.expression(lexer, &mut ctx)?, name_span)?;
                    lexer.next_if(Token::Separator(','));
                    lexer.expect(Token::Paren(')'))?;
                }
                "vertex" => {
                    stage.set(ShaderStage::Vertex, name_span)?;
                }
                "fragment" => {
                    stage.set(ShaderStage::Fragment, name_span)?;
                }
                "compute" => {
                    stage.set(ShaderStage::Compute, name_span)?;
                    shader_stage_error_span = name_span;
                }
                "task" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuMeshShader,
                        name_span,
                    )?;
                    stage.set(ShaderStage::Task, name_span)?;
                    shader_stage_error_span = name_span;
                }
                "mesh" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuMeshShader,
                        name_span,
                    )?;
                    stage.set(ShaderStage::Mesh, name_span)?;
                    shader_stage_error_span = name_span;

                    lexer.expect(Token::Paren('('))?;
                    mesh_output.set(lexer.next_ident_with_span()?, name_span)?;
                    lexer.expect(Token::Paren(')'))?;
                }
                "ray_generation" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuRayTracingPipeline,
                        name_span,
                    )?;
                    stage.set(ShaderStage::RayGeneration, name_span)?;
                    shader_stage_error_span = name_span;
                }
                "any_hit" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuRayTracingPipeline,
                        name_span,
                    )?;
                    stage.set(ShaderStage::AnyHit, name_span)?;
                    shader_stage_error_span = name_span;
                }
                "closest_hit" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuRayTracingPipeline,
                        name_span,
                    )?;
                    stage.set(ShaderStage::ClosestHit, name_span)?;
                    shader_stage_error_span = name_span;
                }
                "miss" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuRayTracingPipeline,
                        name_span,
                    )?;
                    stage.set(ShaderStage::Miss, name_span)?;
                    shader_stage_error_span = name_span;
                }
                "payload" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuMeshShader,
                        name_span,
                    )?;
                    lexer.expect(Token::Paren('('))?;
                    payload.set(lexer.next_ident_with_span()?, name_span)?;
                    lexer.expect(Token::Paren(')'))?;
                }
                "incoming_payload" => {
                    lexer.require_enable_extension(
                        ImplementedEnableExtension::WgpuRayTracingPipeline,
                        name_span,
                    )?;
                    lexer.expect(Token::Paren('('))?;
                    incoming_payload.set(lexer.next_ident_with_span()?, name_span)?;
                    lexer.expect(Token::Paren(')'))?;
                }
                "workgroup_size" => {
                    lexer.expect(Token::Paren('('))?;
                    let mut new_workgroup_size = [None; 3];
                    for size in new_workgroup_size.iter_mut() {
                        *size = Some(self.expression(lexer, &mut ctx)?);
                        match lexer.next() {
                            (Token::Paren(')'), _) => break,
                            (Token::Separator(','), _) => {
                                if lexer.next_if(Token::Paren(')')) {
                                    break;
                                }
                            }
                            other => {
                                return Err(Box::new(Error::Unexpected(
                                    other.1,
                                    ExpectedToken::WorkgroupSizeSeparator,
                                )))
                            }
                        }
                    }
                    workgroup_size.set(new_workgroup_size, name_span)?;
                }
                "early_depth_test" => {
                    lexer.expect(Token::Paren('('))?;
                    let (ident, ident_span) = lexer.next_ident_with_span()?;
                    let value = if ident == "force" {
                        crate::EarlyDepthTest::Force
                    } else {
                        crate::EarlyDepthTest::Allow {
                            conservative: conv::map_conservative_depth(ident, ident_span)?,
                        }
                    };
                    lexer.expect(Token::Paren(')'))?;
                    early_depth_test.set(value, name_span)?;
                }
                "must_use" => {
                    must_use.set(name_span, name_span)?;
                }
                "coherent" => {
                    memory_decorations |= crate::MemoryDecorations::COHERENT;
                }
                "volatile" => {
                    memory_decorations |= crate::MemoryDecorations::VOLATILE;
                }
                _ => return Err(Box::new(Error::UnknownAttribute(name_span))),
            }
        }

        let attrib_span = self.pop_rule_span(lexer);
        match (bind_group.value, bind_index.value) {
            (Some(group), Some(index)) => {
                binding = Some(ast::ResourceBinding {
                    group,
                    binding: index,
                });
            }
            (Some(_), None) => {
                return Err(Box::new(Error::MissingAttribute("binding", attrib_span)))
            }
            (None, Some(_)) => return Err(Box::new(Error::MissingAttribute("group", attrib_span))),
            (None, None) => {}
        }

        // read item
        let start = lexer.start_byte_offset();
        let kind = match lexer.next() {
            (Token::Separator(';'), _) => {
                ensure_no_diag_attrs(
                    DiagnosticAttributeNotSupportedPosition::SemicolonInModulePosition,
                    diagnostic_filters,
                )?;
                None
            }
            (Token::Word(word), directive_span) if DirectiveKind::from_ident(word).is_some() => {
                return Err(Box::new(Error::DirectiveAfterFirstGlobalDecl {
                    directive_span,
                }));
            }
            (Token::Word("struct"), _) => {
                ensure_no_diag_attrs("`struct`s".into(), diagnostic_filters)?;

                let name = lexer.next_ident()?;

                let members = self.struct_body(lexer, &mut ctx)?;

                Some(ast::GlobalDeclKind::Struct(ast::Struct {
                    name,
                    members,
                    doc_comments,
                }))
            }
            (Token::Word("alias"), _) => {
                ensure_no_diag_attrs("`alias`es".into(), diagnostic_filters)?;

                let name = lexer.next_ident()?;

                lexer.expect(Token::Operation('='))?;
                let ty = self.type_specifier(lexer, &mut ctx)?;
                lexer.expect(Token::Separator(';'))?;
                Some(ast::GlobalDeclKind::Type(ast::TypeAlias { name, ty }))
            }
            (Token::Word("const"), _) => {
                ensure_no_diag_attrs("`const`s".into(), diagnostic_filters)?;

                let (name, ty) = self.optionally_typed_ident(lexer, &mut ctx)?;

                lexer.expect(Token::Operation('='))?;
                let init = self.expression(lexer, &mut ctx)?;
                lexer.expect(Token::Separator(';'))?;

                Some(ast::GlobalDeclKind::Const(ast::Const {
                    name,
                    ty,
                    init,
                    doc_comments,
                }))
            }
            (Token::Word("override"), _) => {
                ensure_no_diag_attrs("`override`s".into(), diagnostic_filters)?;

                let (name, ty) = self.optionally_typed_ident(lexer, &mut ctx)?;

                let init = if lexer.next_if(Token::Operation('=')) {
                    Some(self.expression(lexer, &mut ctx)?)
                } else {
                    None
                };

                lexer.expect(Token::Separator(';'))?;

                Some(ast::GlobalDeclKind::Override(ast::Override {
                    name,
                    id: id.value,
                    ty,
                    init,
                }))
            }
            (Token::Word("var"), _) => {
                ensure_no_diag_attrs("`var`s".into(), diagnostic_filters)?;

                let mut var = self.variable_decl(lexer, &mut ctx)?;
                var.binding = binding.take();
                var.doc_comments = doc_comments;
                var.memory_decorations = memory_decorations;
                Some(ast::GlobalDeclKind::Var(var))
            }
            (Token::Word("fn"), _) => {
                let diagnostic_filter_leaf = Self::write_diagnostic_filters(
                    &mut out.diagnostic_filters,
                    diagnostic_filters,
                    out.diagnostic_filter_leaf,
                );

                let function = self.function_decl(
                    lexer,
                    diagnostic_filter_leaf,
                    must_use.value,
                    out,
                    &mut dependencies,
                )?;
                Some(ast::GlobalDeclKind::Fn(ast::Function {
                    entry_point: if let Some(stage) = stage.value {
                        if stage.compute_like() && workgroup_size.value.is_none() {
                            return Err(Box::new(Error::MissingWorkgroupSize(
                                shader_stage_error_span,
                            )));
                        }

                        match stage {
                            ShaderStage::AnyHit | ShaderStage::ClosestHit | ShaderStage::Miss => {
                                if incoming_payload.value.is_none() {
                                    return Err(Box::new(Error::MissingIncomingPayload(
                                        shader_stage_error_span,
                                    )));
                                }
                            }
                            _ => {}
                        }

                        Some(ast::EntryPoint {
                            stage,
                            early_depth_test: early_depth_test.value,
                            workgroup_size: workgroup_size.value,
                            mesh_output_variable: mesh_output.value,
                            task_payload: payload.value,
                            ray_incoming_payload: incoming_payload.value,
                        })
                    } else {
                        None
                    },
                    doc_comments,
                    ..function
                }))
            }
            (Token::Word("const_assert"), _) => {
                ensure_no_diag_attrs("`const_assert`s".into(), diagnostic_filters)?;

                // parentheses are optional
                let paren = lexer.next_if(Token::Paren('('));

                let condition = self.expression(lexer, &mut ctx)?;

                if paren {
                    lexer.expect(Token::Paren(')'))?;
                }
                lexer.expect(Token::Separator(';'))?;
                Some(ast::GlobalDeclKind::ConstAssert(condition))
            }
            (Token::End, _) => return Ok(()),
            other => {
                return Err(Box::new(Error::Unexpected(
                    other.1,
                    ExpectedToken::GlobalItem,
                )))
            }
        };

        if let Some(kind) = kind {
            out.decls.append(
                ast::GlobalDecl { kind, dependencies },
                lexer.span_from(start),
            );
        }

        if !self.rules.is_empty() {
            log::error!("Reached the end of global decl, but rule stack is not empty");
            log::error!("Rules: {:?}", self.rules);
            return Err(Box::new(Error::Internal("rule stack is not empty")));
        };

        match binding {
            None => Ok(()),
            Some(_) => Err(Box::new(Error::Internal(
                "we had the attribute but no var?",
            ))),
        }
    }

    pub fn parse<'a>(
        &mut self,
        source: &'a str,
        options: &Options,
    ) -> Result<'a, ast::TranslationUnit<'a>> {
        self.reset();

        let mut lexer = Lexer::new(source, !options.parse_doc_comments);
        let mut tu = ast::TranslationUnit::default();
        let mut enable_extensions = EnableExtensions::empty();
        let mut diagnostic_filters = DiagnosticFilterMap::new();

        // Parse module doc comments.
        tu.doc_comments = lexer.accumulate_module_doc_comments();

        // Parse directives.
        while let (Token::Word(word), _) = lexer.peek() {
            if let Some(kind) = DirectiveKind::from_ident(word) {
                self.push_rule_span(Rule::Directive, &mut lexer);
                let _ = lexer.next_ident_with_span().unwrap();
                match kind {
                    DirectiveKind::Diagnostic => {
                        let diagnostic_filter = self.diagnostic_filter(&mut lexer)?;
                        let span = self.peek_rule_span(&lexer);
                        diagnostic_filters
                            .add(diagnostic_filter, span, ShouldConflictOnFullDuplicate::No)
                            .map_err(|e| Box::new(e.into()))?;
                        lexer.expect(Token::Separator(';'))?;
                    }
                    DirectiveKind::Enable => {
                        self.directive_ident_list(&mut lexer, |ident, span| {
                            let kind = EnableExtension::from_ident(ident, span)?;
                            let extension = match kind {
                                EnableExtension::Implemented(kind) => kind,
                                EnableExtension::Unimplemented(kind) => {
                                    return Err(Box::new(Error::EnableExtensionNotYetImplemented {
                                        kind,
                                        span,
                                    }))
                                }
                            };
                            // Check if the required capability is supported
                            let required_capability = extension.capability();
                            if !options.capabilities.contains(required_capability) {
                                return Err(Box::new(Error::EnableExtensionNotSupported {
                                    kind,
                                    span,
                                }));
                            }
                            enable_extensions.add(extension);
                            Ok(())
                        })?;
                    }
                    DirectiveKind::Requires => {
                        self.directive_ident_list(&mut lexer, |ident, span| {
                            match LanguageExtension::from_ident(ident) {
                                Some(LanguageExtension::Implemented(_kind)) => {
                                    // NOTE: No further validation is needed for an extension, so
                                    // just throw parsed information away. If we ever want to apply
                                    // what we've parsed to diagnostics, maybe we'll want to refer
                                    // to enabled extensions later?
                                    Ok(())
                                }
                                Some(LanguageExtension::Unimplemented(kind)) => {
                                    Err(Box::new(Error::LanguageExtensionNotYetImplemented {
                                        kind,
                                        span,
                                    }))
                                }
                                None => Err(Box::new(Error::UnknownLanguageExtension(span, ident))),
                            }
                        })?;
                    }
                }
                self.pop_rule_span(&lexer);
            } else {
                break;
            }
        }

        lexer.enable_extensions = enable_extensions;
        tu.enable_extensions = enable_extensions;
        tu.diagnostic_filter_leaf =
            Self::write_diagnostic_filters(&mut tu.diagnostic_filters, diagnostic_filters, None);

        loop {
            match self.global_decl(&mut lexer, &mut tu) {
                Err(error) => return Err(error),
                Ok(()) => {
                    if lexer.peek().0 == Token::End {
                        break;
                    }
                }
            }
        }

        Ok(tu)
    }

    fn increase_brace_nesting(brace_nesting_level: u8, brace_span: Span) -> Result<'static, u8> {
        // From [spec.](https://gpuweb.github.io/gpuweb/wgsl/#limits):
        //
        // > § 2.4. Limits
        // >
        // > …
        // >
        // > Maximum nesting depth of brace-enclosed statements in a function[:] 127
        const BRACE_NESTING_MAXIMUM: u8 = 127;
        if brace_nesting_level + 1 > BRACE_NESTING_MAXIMUM {
            return Err(Box::new(Error::ExceededLimitForNestedBraces {
                span: brace_span,
                limit: BRACE_NESTING_MAXIMUM,
            }));
        }
        Ok(brace_nesting_level + 1)
    }

    fn diagnostic_filter<'a>(&self, lexer: &mut Lexer<'a>) -> Result<'a, DiagnosticFilter> {
        lexer.expect(Token::Paren('('))?;

        let (severity_control_name, severity_control_name_span) = lexer.next_ident_with_span()?;
        let new_severity = diagnostic_filter::Severity::from_wgsl_ident(severity_control_name)
            .ok_or(Error::DiagnosticInvalidSeverity {
                severity_control_name_span,
            })?;

        lexer.expect(Token::Separator(','))?;

        let (diagnostic_name_token, diagnostic_name_token_span) = lexer.next_ident_with_span()?;
        let triggering_rule = if lexer.next_if(Token::Separator('.')) {
            let (ident, _span) = lexer.next_ident_with_span()?;
            FilterableTriggeringRule::User(Box::new([diagnostic_name_token.into(), ident.into()]))
        } else {
            let diagnostic_rule_name = diagnostic_name_token;
            let diagnostic_rule_name_span = diagnostic_name_token_span;
            if let Some(triggering_rule) =
                StandardFilterableTriggeringRule::from_wgsl_ident(diagnostic_rule_name)
            {
                FilterableTriggeringRule::Standard(triggering_rule)
            } else {
                diagnostic_filter::Severity::Warning.report_wgsl_parse_diag(
                    Box::new(Error::UnknownDiagnosticRuleName(diagnostic_rule_name_span)),
                    lexer.source,
                )?;
                FilterableTriggeringRule::Unknown(diagnostic_rule_name.into())
            }
        };
        let filter = DiagnosticFilter {
            triggering_rule,
            new_severity,
        };
        lexer.next_if(Token::Separator(','));
        lexer.expect(Token::Paren(')'))?;

        Ok(filter)
    }

    pub(crate) fn write_diagnostic_filters(
        arena: &mut Arena<DiagnosticFilterNode>,
        filters: DiagnosticFilterMap,
        parent: Option<Handle<DiagnosticFilterNode>>,
    ) -> Option<Handle<DiagnosticFilterNode>> {
        filters
            .into_iter()
            .fold(parent, |parent, (triggering_rule, (new_severity, span))| {
                Some(arena.append(
                    DiagnosticFilterNode {
                        inner: DiagnosticFilter {
                            new_severity,
                            triggering_rule,
                        },
                        parent,
                    },
                    span,
                ))
            })
    }
}

const fn is_start_of_compound_statement<'a>(token: Token<'a>) -> bool {
    matches!(token, Token::Attribute | Token::Paren('{'))
}
