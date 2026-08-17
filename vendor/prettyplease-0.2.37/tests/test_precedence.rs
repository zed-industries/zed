use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens as _;
use std::mem;
use std::process::ExitCode;
use syn::punctuated::Punctuated;
use syn::visit_mut::{self, VisitMut};
use syn::{
    token, AngleBracketedGenericArguments, Arm, BinOp, Block, Expr, ExprArray, ExprAssign,
    ExprAsync, ExprAwait, ExprBinary, ExprBlock, ExprBreak, ExprCall, ExprCast, ExprClosure,
    ExprConst, ExprContinue, ExprField, ExprForLoop, ExprIf, ExprIndex, ExprLet, ExprLit, ExprLoop,
    ExprMacro, ExprMatch, ExprMethodCall, ExprPath, ExprRange, ExprRawAddr, ExprReference,
    ExprReturn, ExprStruct, ExprTry, ExprTryBlock, ExprUnary, ExprUnsafe, ExprWhile, ExprYield,
    File, GenericArgument, Generics, Item, ItemConst, Label, Lifetime, LifetimeParam, Lit, LitInt,
    Macro, MacroDelimiter, Member, Pat, PatWild, Path, PathArguments, PathSegment,
    PointerMutability, QSelf, RangeLimits, ReturnType, Stmt, StmtMacro, Token, Type, TypeInfer,
    TypeParam, TypePath, UnOp, Visibility,
};

struct FlattenParens;

impl VisitMut for FlattenParens {
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        while let Expr::Paren(paren) = e {
            *e = mem::replace(&mut *paren.expr, Expr::PLACEHOLDER);
        }
        visit_mut::visit_expr_mut(self, e);
    }
}

struct AsIfPrinted;

impl VisitMut for AsIfPrinted {
    fn visit_generics_mut(&mut self, generics: &mut Generics) {
        if generics.params.is_empty() {
            generics.lt_token = None;
            generics.gt_token = None;
        }
        if let Some(where_clause) = &generics.where_clause {
            if where_clause.predicates.is_empty() {
                generics.where_clause = None;
            }
        }
        visit_mut::visit_generics_mut(self, generics);
    }

    fn visit_lifetime_param_mut(&mut self, param: &mut LifetimeParam) {
        if param.bounds.is_empty() {
            param.colon_token = None;
        }
        visit_mut::visit_lifetime_param_mut(self, param);
    }

    fn visit_stmt_mut(&mut self, stmt: &mut Stmt) {
        if let Stmt::Expr(expr, semi) = stmt {
            if let Expr::Macro(e) = expr {
                if match e.mac.delimiter {
                    MacroDelimiter::Brace(_) => true,
                    MacroDelimiter::Paren(_) | MacroDelimiter::Bracket(_) => semi.is_some(),
                } {
                    let expr = match mem::replace(expr, Expr::PLACEHOLDER) {
                        Expr::Macro(expr) => expr,
                        _ => unreachable!(),
                    };
                    *stmt = Stmt::Macro(StmtMacro {
                        attrs: expr.attrs,
                        mac: expr.mac,
                        semi_token: *semi,
                    });
                }
            }
        }
        visit_mut::visit_stmt_mut(self, stmt);
    }

    fn visit_type_param_mut(&mut self, param: &mut TypeParam) {
        if param.bounds.is_empty() {
            param.colon_token = None;
        }
        visit_mut::visit_type_param_mut(self, param);
    }
}

#[test]
fn test_permutations() -> ExitCode {
    fn iter(depth: usize, f: &mut dyn FnMut(Expr)) {
        let span = Span::call_site();

        // Expr::Path
        f(Expr::Path(ExprPath {
            // `x`
            attrs: Vec::new(),
            qself: None,
            path: Path::from(Ident::new("x", span)),
        }));
        if false {
            f(Expr::Path(ExprPath {
                // `x::<T>`
                attrs: Vec::new(),
                qself: None,
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([PathSegment {
                        ident: Ident::new("x", span),
                        arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: Some(Token![::](span)),
                            lt_token: Token![<](span),
                            args: Punctuated::from_iter([GenericArgument::Type(Type::Path(
                                TypePath {
                                    qself: None,
                                    path: Path::from(Ident::new("T", span)),
                                },
                            ))]),
                            gt_token: Token![>](span),
                        }),
                    }]),
                },
            }));
            f(Expr::Path(ExprPath {
                // `<T as Trait>::CONST`
                attrs: Vec::new(),
                qself: Some(QSelf {
                    lt_token: Token![<](span),
                    ty: Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Path::from(Ident::new("T", span)),
                    })),
                    position: 1,
                    as_token: Some(Token![as](span)),
                    gt_token: Token![>](span),
                }),
                path: Path {
                    leading_colon: None,
                    segments: Punctuated::from_iter([
                        PathSegment::from(Ident::new("Trait", span)),
                        PathSegment::from(Ident::new("CONST", span)),
                    ]),
                },
            }));
        }

        let depth = match depth.checked_sub(1) {
            Some(depth) => depth,
            None => return,
        };

        // Expr::Assign
        iter(depth, &mut |expr| {
            iter(0, &mut |simple| {
                f(Expr::Assign(ExprAssign {
                    // `x = $expr`
                    attrs: Vec::new(),
                    left: Box::new(simple.clone()),
                    eq_token: Token![=](span),
                    right: Box::new(expr.clone()),
                }));
                f(Expr::Assign(ExprAssign {
                    // `$expr = x`
                    attrs: Vec::new(),
                    left: Box::new(expr.clone()),
                    eq_token: Token![=](span),
                    right: Box::new(simple),
                }));
            });
        });

        // Expr::Binary
        iter(depth, &mut |expr| {
            iter(0, &mut |simple| {
                for op in [
                    BinOp::Add(Token![+](span)),
                    //BinOp::Sub(Token![-](span)),
                    //BinOp::Mul(Token![*](span)),
                    //BinOp::Div(Token![/](span)),
                    //BinOp::Rem(Token![%](span)),
                    //BinOp::And(Token![&&](span)),
                    //BinOp::Or(Token![||](span)),
                    //BinOp::BitXor(Token![^](span)),
                    //BinOp::BitAnd(Token![&](span)),
                    //BinOp::BitOr(Token![|](span)),
                    //BinOp::Shl(Token![<<](span)),
                    //BinOp::Shr(Token![>>](span)),
                    //BinOp::Eq(Token![==](span)),
                    BinOp::Lt(Token![<](span)),
                    //BinOp::Le(Token![<=](span)),
                    //BinOp::Ne(Token![!=](span)),
                    //BinOp::Ge(Token![>=](span)),
                    //BinOp::Gt(Token![>](span)),
                    BinOp::ShlAssign(Token![<<=](span)),
                ] {
                    f(Expr::Binary(ExprBinary {
                        // `x + $expr`
                        attrs: Vec::new(),
                        left: Box::new(simple.clone()),
                        op,
                        right: Box::new(expr.clone()),
                    }));
                    f(Expr::Binary(ExprBinary {
                        // `$expr + x`
                        attrs: Vec::new(),
                        left: Box::new(expr.clone()),
                        op,
                        right: Box::new(simple.clone()),
                    }));
                }
            });
        });

        // Expr::Block
        f(Expr::Block(ExprBlock {
            // `{}`
            attrs: Vec::new(),
            label: None,
            block: Block {
                brace_token: token::Brace(span),
                stmts: Vec::new(),
            },
        }));

        // Expr::Break
        f(Expr::Break(ExprBreak {
            // `break`
            attrs: Vec::new(),
            break_token: Token![break](span),
            label: None,
            expr: None,
        }));
        iter(depth, &mut |expr| {
            f(Expr::Break(ExprBreak {
                // `break $expr`
                attrs: Vec::new(),
                break_token: Token![break](span),
                label: None,
                expr: Some(Box::new(expr)),
            }));
        });

        // Expr::Call
        iter(depth, &mut |expr| {
            f(Expr::Call(ExprCall {
                // `$expr()`
                attrs: Vec::new(),
                func: Box::new(expr),
                paren_token: token::Paren(span),
                args: Punctuated::new(),
            }));
        });

        // Expr::Cast
        iter(depth, &mut |expr| {
            f(Expr::Cast(ExprCast {
                // `$expr as T`
                attrs: Vec::new(),
                expr: Box::new(expr),
                as_token: Token![as](span),
                ty: Box::new(Type::Path(TypePath {
                    qself: None,
                    path: Path::from(Ident::new("T", span)),
                })),
            }));
        });

        // Expr::Closure
        iter(depth, &mut |expr| {
            f(Expr::Closure(ExprClosure {
                // `|| $expr`
                attrs: Vec::new(),
                lifetimes: None,
                constness: None,
                movability: None,
                asyncness: None,
                capture: None,
                or1_token: Token![|](span),
                inputs: Punctuated::new(),
                or2_token: Token![|](span),
                output: ReturnType::Default,
                body: Box::new(expr),
            }));
        });

        // Expr::Field
        iter(depth, &mut |expr| {
            f(Expr::Field(ExprField {
                // `$expr.field`
                attrs: Vec::new(),
                base: Box::new(expr),
                dot_token: Token![.](span),
                member: Member::Named(Ident::new("field", span)),
            }));
        });

        // Expr::If
        iter(depth, &mut |expr| {
            f(Expr::If(ExprIf {
                // `if $expr {}`
                attrs: Vec::new(),
                if_token: Token![if](span),
                cond: Box::new(expr),
                then_branch: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
                else_branch: None,
            }));
        });

        // Expr::Let
        iter(depth, &mut |expr| {
            f(Expr::Let(ExprLet {
                attrs: Vec::new(),
                let_token: Token![let](span),
                pat: Box::new(Pat::Wild(PatWild {
                    attrs: Vec::new(),
                    underscore_token: Token![_](span),
                })),
                eq_token: Token![=](span),
                expr: Box::new(expr),
            }));
        });

        // Expr::Range
        f(Expr::Range(ExprRange {
            // `..`
            attrs: Vec::new(),
            start: None,
            limits: RangeLimits::HalfOpen(Token![..](span)),
            end: None,
        }));
        iter(depth, &mut |expr| {
            f(Expr::Range(ExprRange {
                // `..$expr`
                attrs: Vec::new(),
                start: None,
                limits: RangeLimits::HalfOpen(Token![..](span)),
                end: Some(Box::new(expr.clone())),
            }));
            f(Expr::Range(ExprRange {
                // `$expr..`
                attrs: Vec::new(),
                start: Some(Box::new(expr)),
                limits: RangeLimits::HalfOpen(Token![..](span)),
                end: None,
            }));
        });

        // Expr::Reference
        iter(depth, &mut |expr| {
            f(Expr::Reference(ExprReference {
                // `&$expr`
                attrs: Vec::new(),
                and_token: Token![&](span),
                mutability: None,
                expr: Box::new(expr),
            }));
        });

        // Expr::Return
        f(Expr::Return(ExprReturn {
            // `return`
            attrs: Vec::new(),
            return_token: Token![return](span),
            expr: None,
        }));
        iter(depth, &mut |expr| {
            f(Expr::Return(ExprReturn {
                // `return $expr`
                attrs: Vec::new(),
                return_token: Token![return](span),
                expr: Some(Box::new(expr)),
            }));
        });

        // Expr::Try
        iter(depth, &mut |expr| {
            f(Expr::Try(ExprTry {
                // `$expr?`
                attrs: Vec::new(),
                expr: Box::new(expr),
                question_token: Token![?](span),
            }));
        });

        // Expr::Unary
        iter(depth, &mut |expr| {
            for op in [
                UnOp::Deref(Token![*](span)),
                //UnOp::Not(Token![!](span)),
                //UnOp::Neg(Token![-](span)),
            ] {
                f(Expr::Unary(ExprUnary {
                    // `*$expr`
                    attrs: Vec::new(),
                    op,
                    expr: Box::new(expr.clone()),
                }));
            }
        });

        if false {
            // Expr::Array
            f(Expr::Array(ExprArray {
                // `[]`
                attrs: Vec::new(),
                bracket_token: token::Bracket(span),
                elems: Punctuated::new(),
            }));

            // Expr::Async
            f(Expr::Async(ExprAsync {
                // `async {}`
                attrs: Vec::new(),
                async_token: Token![async](span),
                capture: None,
                block: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));

            // Expr::Await
            iter(depth, &mut |expr| {
                f(Expr::Await(ExprAwait {
                    // `$expr.await`
                    attrs: Vec::new(),
                    base: Box::new(expr),
                    dot_token: Token![.](span),
                    await_token: Token![await](span),
                }));
            });

            // Expr::Block
            f(Expr::Block(ExprBlock {
                // `'a: {}`
                attrs: Vec::new(),
                label: Some(Label {
                    name: Lifetime::new("'a", span),
                    colon_token: Token![:](span),
                }),
                block: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));
            iter(depth, &mut |expr| {
                f(Expr::Block(ExprBlock {
                    // `{ $expr }`
                    attrs: Vec::new(),
                    label: None,
                    block: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::from([Stmt::Expr(expr.clone(), None)]),
                    },
                }));
                f(Expr::Block(ExprBlock {
                    // `{ $expr; }`
                    attrs: Vec::new(),
                    label: None,
                    block: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::from([Stmt::Expr(expr, Some(Token![;](span)))]),
                    },
                }));
            });

            // Expr::Break
            f(Expr::Break(ExprBreak {
                // `break 'a`
                attrs: Vec::new(),
                break_token: Token![break](span),
                label: Some(Lifetime::new("'a", span)),
                expr: None,
            }));
            iter(depth, &mut |expr| {
                f(Expr::Break(ExprBreak {
                    // `break 'a $expr`
                    attrs: Vec::new(),
                    break_token: Token![break](span),
                    label: Some(Lifetime::new("'a", span)),
                    expr: Some(Box::new(expr)),
                }));
            });

            // Expr::Closure
            f(Expr::Closure(ExprClosure {
                // `|| -> T {}`
                attrs: Vec::new(),
                lifetimes: None,
                constness: None,
                movability: None,
                asyncness: None,
                capture: None,
                or1_token: Token![|](span),
                inputs: Punctuated::new(),
                or2_token: Token![|](span),
                output: ReturnType::Type(
                    Token![->](span),
                    Box::new(Type::Path(TypePath {
                        qself: None,
                        path: Path::from(Ident::new("T", span)),
                    })),
                ),
                body: Box::new(Expr::Block(ExprBlock {
                    attrs: Vec::new(),
                    label: None,
                    block: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::new(),
                    },
                })),
            }));

            // Expr::Const
            f(Expr::Const(ExprConst {
                // `const {}`
                attrs: Vec::new(),
                const_token: Token![const](span),
                block: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));

            // Expr::Continue
            f(Expr::Continue(ExprContinue {
                // `continue`
                attrs: Vec::new(),
                continue_token: Token![continue](span),
                label: None,
            }));
            f(Expr::Continue(ExprContinue {
                // `continue 'a`
                attrs: Vec::new(),
                continue_token: Token![continue](span),
                label: Some(Lifetime::new("'a", span)),
            }));

            // Expr::ForLoop
            iter(depth, &mut |expr| {
                f(Expr::ForLoop(ExprForLoop {
                    // `for _ in $expr {}`
                    attrs: Vec::new(),
                    label: None,
                    for_token: Token![for](span),
                    pat: Box::new(Pat::Wild(PatWild {
                        attrs: Vec::new(),
                        underscore_token: Token![_](span),
                    })),
                    in_token: Token![in](span),
                    expr: Box::new(expr.clone()),
                    body: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::new(),
                    },
                }));
                f(Expr::ForLoop(ExprForLoop {
                    // `'a: for _ in $expr {}`
                    attrs: Vec::new(),
                    label: Some(Label {
                        name: Lifetime::new("'a", span),
                        colon_token: Token![:](span),
                    }),
                    for_token: Token![for](span),
                    pat: Box::new(Pat::Wild(PatWild {
                        attrs: Vec::new(),
                        underscore_token: Token![_](span),
                    })),
                    in_token: Token![in](span),
                    expr: Box::new(expr),
                    body: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::new(),
                    },
                }));
            });

            // Expr::Index
            iter(depth, &mut |expr| {
                f(Expr::Index(ExprIndex {
                    // `$expr[0]`
                    attrs: Vec::new(),
                    expr: Box::new(expr),
                    bracket_token: token::Bracket(span),
                    index: Box::new(Expr::Lit(ExprLit {
                        attrs: Vec::new(),
                        lit: Lit::Int(LitInt::new("0", span)),
                    })),
                }));
            });

            // Expr::Loop
            f(Expr::Loop(ExprLoop {
                // `loop {}`
                attrs: Vec::new(),
                label: None,
                loop_token: Token![loop](span),
                body: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));
            f(Expr::Loop(ExprLoop {
                // `'a: loop {}`
                attrs: Vec::new(),
                label: Some(Label {
                    name: Lifetime::new("'a", span),
                    colon_token: Token![:](span),
                }),
                loop_token: Token![loop](span),
                body: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));

            // Expr::Macro
            f(Expr::Macro(ExprMacro {
                // `m!()`
                attrs: Vec::new(),
                mac: Macro {
                    path: Path::from(Ident::new("m", span)),
                    bang_token: Token![!](span),
                    delimiter: MacroDelimiter::Paren(token::Paren(span)),
                    tokens: TokenStream::new(),
                },
            }));
            f(Expr::Macro(ExprMacro {
                // `m! {}`
                attrs: Vec::new(),
                mac: Macro {
                    path: Path::from(Ident::new("m", span)),
                    bang_token: Token![!](span),
                    delimiter: MacroDelimiter::Brace(token::Brace(span)),
                    tokens: TokenStream::new(),
                },
            }));

            // Expr::Match
            iter(depth, &mut |expr| {
                f(Expr::Match(ExprMatch {
                    // `match $expr {}`
                    attrs: Vec::new(),
                    match_token: Token![match](span),
                    expr: Box::new(expr.clone()),
                    brace_token: token::Brace(span),
                    arms: Vec::new(),
                }));
                f(Expr::Match(ExprMatch {
                    // `match x { _ => $expr }`
                    attrs: Vec::new(),
                    match_token: Token![match](span),
                    expr: Box::new(Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: Path::from(Ident::new("x", span)),
                    })),
                    brace_token: token::Brace(span),
                    arms: Vec::from([Arm {
                        attrs: Vec::new(),
                        pat: Pat::Wild(PatWild {
                            attrs: Vec::new(),
                            underscore_token: Token![_](span),
                        }),
                        guard: None,
                        fat_arrow_token: Token![=>](span),
                        body: Box::new(expr.clone()),
                        comma: None,
                    }]),
                }));
                f(Expr::Match(ExprMatch {
                    // `match x { _ if $expr => {} }`
                    attrs: Vec::new(),
                    match_token: Token![match](span),
                    expr: Box::new(Expr::Path(ExprPath {
                        attrs: Vec::new(),
                        qself: None,
                        path: Path::from(Ident::new("x", span)),
                    })),
                    brace_token: token::Brace(span),
                    arms: Vec::from([Arm {
                        attrs: Vec::new(),
                        pat: Pat::Wild(PatWild {
                            attrs: Vec::new(),
                            underscore_token: Token![_](span),
                        }),
                        guard: Some((Token![if](span), Box::new(expr))),
                        fat_arrow_token: Token![=>](span),
                        body: Box::new(Expr::Block(ExprBlock {
                            attrs: Vec::new(),
                            label: None,
                            block: Block {
                                brace_token: token::Brace(span),
                                stmts: Vec::new(),
                            },
                        })),
                        comma: None,
                    }]),
                }));
            });

            // Expr::MethodCall
            iter(depth, &mut |expr| {
                f(Expr::MethodCall(ExprMethodCall {
                    // `$expr.method()`
                    attrs: Vec::new(),
                    receiver: Box::new(expr.clone()),
                    dot_token: Token![.](span),
                    method: Ident::new("method", span),
                    turbofish: None,
                    paren_token: token::Paren(span),
                    args: Punctuated::new(),
                }));
                f(Expr::MethodCall(ExprMethodCall {
                    // `$expr.method::<T>()`
                    attrs: Vec::new(),
                    receiver: Box::new(expr),
                    dot_token: Token![.](span),
                    method: Ident::new("method", span),
                    turbofish: Some(AngleBracketedGenericArguments {
                        colon2_token: Some(Token![::](span)),
                        lt_token: Token![<](span),
                        args: Punctuated::from_iter([GenericArgument::Type(Type::Path(
                            TypePath {
                                qself: None,
                                path: Path::from(Ident::new("T", span)),
                            },
                        ))]),
                        gt_token: Token![>](span),
                    }),
                    paren_token: token::Paren(span),
                    args: Punctuated::new(),
                }));
            });

            // Expr::RawAddr
            iter(depth, &mut |expr| {
                f(Expr::RawAddr(ExprRawAddr {
                    // `&raw const $expr`
                    attrs: Vec::new(),
                    and_token: Token![&](span),
                    raw: Token![raw](span),
                    mutability: PointerMutability::Const(Token![const](span)),
                    expr: Box::new(expr),
                }));
            });

            // Expr::Struct
            f(Expr::Struct(ExprStruct {
                // `Struct {}`
                attrs: Vec::new(),
                qself: None,
                path: Path::from(Ident::new("Struct", span)),
                brace_token: token::Brace(span),
                fields: Punctuated::new(),
                dot2_token: None,
                rest: None,
            }));

            // Expr::TryBlock
            f(Expr::TryBlock(ExprTryBlock {
                // `try {}`
                attrs: Vec::new(),
                try_token: Token![try](span),
                block: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));

            // Expr::Unsafe
            f(Expr::Unsafe(ExprUnsafe {
                // `unsafe {}`
                attrs: Vec::new(),
                unsafe_token: Token![unsafe](span),
                block: Block {
                    brace_token: token::Brace(span),
                    stmts: Vec::new(),
                },
            }));

            // Expr::While
            iter(depth, &mut |expr| {
                f(Expr::While(ExprWhile {
                    // `while $expr {}`
                    attrs: Vec::new(),
                    label: None,
                    while_token: Token![while](span),
                    cond: Box::new(expr.clone()),
                    body: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::new(),
                    },
                }));
                f(Expr::While(ExprWhile {
                    // `'a: while $expr {}`
                    attrs: Vec::new(),
                    label: Some(Label {
                        name: Lifetime::new("'a", span),
                        colon_token: Token![:](span),
                    }),
                    while_token: Token![while](span),
                    cond: Box::new(expr),
                    body: Block {
                        brace_token: token::Brace(span),
                        stmts: Vec::new(),
                    },
                }));
            });

            // Expr::Yield
            f(Expr::Yield(ExprYield {
                // `yield`
                attrs: Vec::new(),
                yield_token: Token![yield](span),
                expr: None,
            }));
            iter(depth, &mut |expr| {
                f(Expr::Yield(ExprYield {
                    // `yield $expr`
                    attrs: Vec::new(),
                    yield_token: Token![yield](span),
                    expr: Some(Box::new(expr)),
                }));
            });
        }
    }

    let mut failures = 0;
    macro_rules! fail {
        ($($message:tt)*) => {{
            eprintln!($($message)*);
            failures += 1;
            return;
        }};
    }
    let mut assert = |mut original: Expr| {
        let span = Span::call_site();
        // `const _: () = $expr;`
        let pretty = prettyplease::unparse(&File {
            shebang: None,
            attrs: Vec::new(),
            items: Vec::from([Item::Const(ItemConst {
                attrs: Vec::new(),
                vis: Visibility::Inherited,
                const_token: Token![const](span),
                ident: Ident::from(Token![_](span)),
                generics: Generics::default(),
                colon_token: Token![:](span),
                ty: Box::new(Type::Infer(TypeInfer {
                    underscore_token: Token![_](span),
                })),
                eq_token: Token![=](span),
                expr: Box::new(original.clone()),
                semi_token: Token![;](span),
            })]),
        });
        let mut parsed = match syn::parse_file(&pretty) {
            Ok(parsed) => parsed,
            _ => fail!("failed to parse: {pretty}{original:#?}"),
        };
        let item = match parsed.items.as_mut_slice() {
            [Item::Const(item)] => item,
            _ => unreachable!(),
        };
        let mut parsed = mem::replace(&mut *item.expr, Expr::PLACEHOLDER);
        AsIfPrinted.visit_expr_mut(&mut original);
        FlattenParens.visit_expr_mut(&mut parsed);
        if original != parsed {
            fail!(
                "before: {}\n{:#?}\nafter: {}\n{:#?}",
                original.to_token_stream(),
                original,
                parsed.to_token_stream(),
                parsed,
            );
        }
        if pretty.contains("(||") {
            // https://github.com/dtolnay/prettyplease/issues/99
            return;
        }
        let no_paren = pretty.replace(['(', ')'], "");
        if pretty != no_paren {
            if let Ok(mut parsed2) = syn::parse_file(&no_paren) {
                let item = match parsed2.items.as_mut_slice() {
                    [Item::Const(item)] => item,
                    _ => unreachable!(),
                };
                if original == *item.expr {
                    fail!("redundant parens: {}", pretty);
                }
            }
        }
    };

    iter(if cfg!(debug_assertions) { 3 } else { 4 }, &mut assert);
    if failures > 0 {
        eprintln!("FAILURES: {failures}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
