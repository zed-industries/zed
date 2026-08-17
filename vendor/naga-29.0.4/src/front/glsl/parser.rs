use alloc::{string::String, vec};
use core::iter::Peekable;

use pp_rs::token::{PreprocessorError, Token as PPToken, TokenValue as PPTokenValue};

use super::{
    ast::{FunctionKind, Profile, TypeQualifiers},
    context::{Context, ExprPos},
    error::ExpectedToken,
    error::{Error, ErrorKind},
    lex::{Lexer, LexerResultKind},
    token::{Directive, DirectiveKind},
    token::{Token, TokenValue},
    variables::{GlobalOrConstant, VarDeclaration},
    Frontend, Result,
};
use crate::{arena::Handle, proc::ConstValueError, Expression, Module, Span, Type};

mod declarations;
mod expressions;
mod functions;
mod types;

pub struct ParsingContext<'source> {
    lexer: Peekable<Lexer<'source>>,
    /// Used to store tokens already consumed by the parser but that need to be backtracked
    backtracked_token: Option<Token>,
    last_meta: Span,
}

impl<'source> ParsingContext<'source> {
    pub fn new(lexer: Lexer<'source>) -> Self {
        ParsingContext {
            lexer: lexer.peekable(),
            backtracked_token: None,
            last_meta: Span::default(),
        }
    }

    /// Helper method for backtracking from a consumed token
    ///
    /// This method should always be used instead of assigning to `backtracked_token` since
    /// it validates that backtracking hasn't occurred more than one time in a row
    ///
    /// # Panics
    /// - If the parser already backtracked without bumping in between
    pub fn backtrack(&mut self, token: Token) -> Result<()> {
        // This should never happen
        if let Some(ref prev_token) = self.backtracked_token {
            return Err(Error {
                kind: ErrorKind::InternalError("The parser tried to backtrack twice in a row"),
                meta: prev_token.meta,
            });
        }

        self.backtracked_token = Some(token);

        Ok(())
    }

    pub fn expect_ident(&mut self, frontend: &mut Frontend) -> Result<(String, Span)> {
        let token = self.bump(frontend)?;

        match token.value {
            TokenValue::Identifier(name) => Ok((name, token.meta)),
            _ => Err(Error {
                kind: ErrorKind::InvalidToken(token.value, vec![ExpectedToken::Identifier]),
                meta: token.meta,
            }),
        }
    }

    pub fn expect(&mut self, frontend: &mut Frontend, value: TokenValue) -> Result<Token> {
        let token = self.bump(frontend)?;

        if token.value != value {
            Err(Error {
                kind: ErrorKind::InvalidToken(token.value, vec![value.into()]),
                meta: token.meta,
            })
        } else {
            Ok(token)
        }
    }

    pub fn next(&mut self, frontend: &mut Frontend) -> Option<Token> {
        loop {
            if let Some(token) = self.backtracked_token.take() {
                self.last_meta = token.meta;
                break Some(token);
            }

            let res = self.lexer.next()?;

            match res.kind {
                LexerResultKind::Token(token) => {
                    self.last_meta = token.meta;
                    break Some(token);
                }
                LexerResultKind::Directive(directive) => {
                    frontend.handle_directive(directive, res.meta)
                }
                LexerResultKind::Error(error) => frontend.errors.push(Error {
                    kind: ErrorKind::PreprocessorError(error),
                    meta: res.meta,
                }),
            }
        }
    }

    pub fn bump(&mut self, frontend: &mut Frontend) -> Result<Token> {
        self.next(frontend).ok_or(Error {
            kind: ErrorKind::EndOfFile,
            meta: self.last_meta,
        })
    }

    /// Returns None on the end of the file rather than an error like other methods
    pub fn bump_if(&mut self, frontend: &mut Frontend, value: TokenValue) -> Option<Token> {
        if self.peek(frontend).filter(|t| t.value == value).is_some() {
            self.bump(frontend).ok()
        } else {
            None
        }
    }

    pub fn peek(&mut self, frontend: &mut Frontend) -> Option<&Token> {
        loop {
            if let Some(ref token) = self.backtracked_token {
                break Some(token);
            }

            match self.lexer.peek()?.kind {
                LexerResultKind::Token(_) => {
                    let res = self.lexer.peek()?;

                    match res.kind {
                        LexerResultKind::Token(ref token) => break Some(token),
                        _ => unreachable!(),
                    }
                }
                LexerResultKind::Error(_) | LexerResultKind::Directive(_) => {
                    let res = self.lexer.next()?;

                    match res.kind {
                        LexerResultKind::Directive(directive) => {
                            frontend.handle_directive(directive, res.meta)
                        }
                        LexerResultKind::Error(error) => frontend.errors.push(Error {
                            kind: ErrorKind::PreprocessorError(error),
                            meta: res.meta,
                        }),
                        LexerResultKind::Token(_) => unreachable!(),
                    }
                }
            }
        }
    }

    pub fn expect_peek(&mut self, frontend: &mut Frontend) -> Result<&Token> {
        let meta = self.last_meta;
        self.peek(frontend).ok_or(Error {
            kind: ErrorKind::EndOfFile,
            meta,
        })
    }

    pub fn parse(&mut self, frontend: &mut Frontend) -> Result<Module> {
        let mut module = Module::default();
        let mut global_expression_kind_tracker = crate::proc::ExpressionKindTracker::new();

        // Body and expression arena for global initialization
        let mut ctx = Context::new(
            frontend,
            &mut module,
            false,
            &mut global_expression_kind_tracker,
        )?;

        while self.peek(frontend).is_some() {
            self.parse_external_declaration(frontend, &mut ctx)?;
        }

        // Add an `EntryPoint` to `parser.module` for `main`, if a
        // suitable overload exists. Error out if we can't find one.
        if let Some(declaration) = frontend.lookup_function.get("main") {
            for decl in declaration.overloads.iter() {
                if let FunctionKind::Call(handle) = decl.kind {
                    if decl.defined && decl.parameters.is_empty() {
                        frontend.add_entry_point(handle, ctx)?;
                        return Ok(module);
                    }
                }
            }
        }

        Err(Error {
            kind: ErrorKind::SemanticError("Missing entry point".into()),
            meta: Span::default(),
        })
    }

    fn parse_uint_constant(
        &mut self,
        frontend: &mut Frontend,
        ctx: &mut Context,
    ) -> Result<(u32, Span)> {
        let (const_expr, meta) = self.parse_constant_expression(
            frontend,
            ctx.module,
            ctx.global_expression_kind_tracker,
        )?;

        let res = ctx.module.to_ctx().get_const_val(const_expr);

        let int = match res {
            Ok(value) => Ok(value),
            Err(ConstValueError::Negative) => Err(Error {
                kind: ErrorKind::SemanticError("int constant overflows".into()),
                meta,
            }),
            Err(ConstValueError::NonConst | ConstValueError::InvalidType) => Err(Error {
                kind: ErrorKind::SemanticError("Expected a uint constant".into()),
                meta,
            }),
        }?;

        Ok((int, meta))
    }

    fn parse_constant_expression(
        &mut self,
        frontend: &mut Frontend,
        module: &mut Module,
        global_expression_kind_tracker: &mut crate::proc::ExpressionKindTracker,
    ) -> Result<(Handle<Expression>, Span)> {
        let mut ctx = Context::new(frontend, module, true, global_expression_kind_tracker)?;

        let mut stmt_ctx = ctx.stmt_ctx();
        let expr = self.parse_conditional(frontend, &mut ctx, &mut stmt_ctx, None)?;
        let (root, meta) = ctx.lower_expect(stmt_ctx, frontend, expr, ExprPos::Rhs)?;

        Ok((root, meta))
    }
}

impl Frontend {
    fn handle_directive(&mut self, directive: Directive, meta: Span) {
        let mut tokens = directive.tokens.into_iter();

        match directive.kind {
            DirectiveKind::Version { is_first_directive } => {
                if !is_first_directive {
                    self.errors.push(Error {
                        kind: ErrorKind::SemanticError(
                            "#version must occur first in shader".into(),
                        ),
                        meta,
                    })
                }

                match tokens.next() {
                    Some(PPToken {
                        value: PPTokenValue::Integer(int),
                        location,
                    }) => match int.value {
                        440 | 450 | 460 => self.meta.version = int.value as u16,
                        _ => self.errors.push(Error {
                            kind: ErrorKind::InvalidVersion(int.value),
                            meta: location.into(),
                        }),
                    },
                    Some(PPToken { value, location }) => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                            value,
                        )),
                        meta: location.into(),
                    }),
                    None => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedNewLine),
                        meta,
                    }),
                };

                match tokens.next() {
                    Some(PPToken {
                        value: PPTokenValue::Ident(name),
                        location,
                    }) => match name.as_str() {
                        "core" => self.meta.profile = Profile::Core,
                        _ => self.errors.push(Error {
                            kind: ErrorKind::InvalidProfile(name),
                            meta: location.into(),
                        }),
                    },
                    Some(PPToken { value, location }) => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                            value,
                        )),
                        meta: location.into(),
                    }),
                    None => {}
                };

                if let Some(PPToken { value, location }) = tokens.next() {
                    self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                            value,
                        )),
                        meta: location.into(),
                    })
                }
            }
            DirectiveKind::Extension => {
                // TODO: Proper extension handling
                // - Checking for extension support in the compiler
                // - Handle behaviors such as warn
                // - Handle the all extension
                let name = match tokens.next() {
                    Some(PPToken {
                        value: PPTokenValue::Ident(name),
                        ..
                    }) => Some(name),
                    Some(PPToken { value, location }) => {
                        self.errors.push(Error {
                            kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                                value,
                            )),
                            meta: location.into(),
                        });

                        None
                    }
                    None => {
                        self.errors.push(Error {
                            kind: ErrorKind::PreprocessorError(
                                PreprocessorError::UnexpectedNewLine,
                            ),
                            meta,
                        });

                        None
                    }
                };

                match tokens.next() {
                    Some(PPToken {
                        value: PPTokenValue::Punct(pp_rs::token::Punct::Colon),
                        ..
                    }) => {}
                    Some(PPToken { value, location }) => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                            value,
                        )),
                        meta: location.into(),
                    }),
                    None => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedNewLine),
                        meta,
                    }),
                };

                match tokens.next() {
                    Some(PPToken {
                        value: PPTokenValue::Ident(behavior),
                        location,
                    }) => match behavior.as_str() {
                        "require" | "enable" | "warn" | "disable" => {
                            if let Some(name) = name {
                                self.meta.extensions.insert(name);
                            }
                        }
                        _ => self.errors.push(Error {
                            kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                                PPTokenValue::Ident(behavior),
                            )),
                            meta: location.into(),
                        }),
                    },
                    Some(PPToken { value, location }) => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                            value,
                        )),
                        meta: location.into(),
                    }),
                    None => self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedNewLine),
                        meta,
                    }),
                }

                if let Some(PPToken { value, location }) = tokens.next() {
                    self.errors.push(Error {
                        kind: ErrorKind::PreprocessorError(PreprocessorError::UnexpectedToken(
                            value,
                        )),
                        meta: location.into(),
                    })
                }
            }
            DirectiveKind::Pragma => {
                // TODO: handle some common pragmas?
            }
        }
    }
}

pub struct DeclarationContext<'ctx, 'qualifiers, 'a> {
    qualifiers: TypeQualifiers<'qualifiers>,
    /// Indicates a global declaration
    external: bool,
    is_inside_loop: bool,
    ctx: &'ctx mut Context<'a>,
}

impl DeclarationContext<'_, '_, '_> {
    fn add_var(
        &mut self,
        frontend: &mut Frontend,
        ty: Handle<Type>,
        name: String,
        init: Option<Handle<Expression>>,
        meta: Span,
    ) -> Result<Handle<Expression>> {
        let decl = VarDeclaration {
            qualifiers: &mut self.qualifiers,
            ty,
            name: Some(name),
            init,
            meta,
        };

        match self.external {
            true => {
                let global = frontend.add_global_var(self.ctx, decl)?;
                let expr = match global {
                    GlobalOrConstant::Global(handle) => Expression::GlobalVariable(handle),
                    GlobalOrConstant::Constant(handle) => Expression::Constant(handle),
                    GlobalOrConstant::Override(handle) => Expression::Override(handle),
                };
                Ok(self.ctx.add_expression(expr, meta)?)
            }
            false => frontend.add_local_var(self.ctx, decl),
        }
    }
}
