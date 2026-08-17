use proc_macro2::{Span, TokenStream};
use quote::quote;
use spanned::Spanned;
use syn::*;
use syn_expr::{CmpOp, CustomExpr};

mod syn_expr {
    #![allow(dead_code)]

    use syn::{
        custom_keyword, parenthesized, punctuated::Punctuated, token::Paren, BinOp, Expr,
        ExprBinary, Token,
    };

    custom_keyword!(all);
    custom_keyword!(any);

    #[derive(Clone)]
    pub enum CmpOp {
        Custom(Token![:], Expr, Token![:]),
        Approx(Token![~]),
        Eq(Token![==]),
        Ne(Token![!=]),
        Ge(Token![>=]),
        Le(Token![<=]),
        Gt(Token![>]),
        Lt(Token![<]),
    }

    #[derive(Clone)]
    pub enum CustomExpr {
        All {
            all_token: all,
            paren_token: Paren,
            args: Punctuated<CustomExpr, Token![,]>,
        },
        Any {
            any_token: any,
            paren_token: Paren,
            args: Punctuated<CustomExpr, Token![,]>,
        },
        Cmp {
            left: Expr,
            op: CmpOp,
            right: Expr,
        },
        Boolean(Expr),
    }

    impl syn::parse::Parse for CustomExpr {
        fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
            let lookahead = input.lookahead1();
            if lookahead.peek(all) {
                let _: syn::token::Tilde;
                let content;
                Ok(CustomExpr::All {
                    all_token: input.parse()?,
                    paren_token: parenthesized!(content in input),
                    args: content.parse_terminated(Self::parse, Token![,])?,
                })
            } else if lookahead.peek(any) {
                let content;
                Ok(CustomExpr::Any {
                    any_token: input.parse()?,
                    paren_token: parenthesized!(content in input),
                    args: content.parse_terminated(Self::parse, Token![,])?,
                })
            } else {
                let expr = input.parse()?;
                let lookahead = input.lookahead1();

                match expr {
                    Expr::Binary(ExprBinary {
                        attrs: _,
                        left,
                        op:
                            op @ (BinOp::Eq(_)
                            | BinOp::Ne(_)
                            | BinOp::Ge(_)
                            | BinOp::Le(_)
                            | BinOp::Gt(_)
                            | BinOp::Lt(_)),

                        right,
                    }) => match op {
                        BinOp::Eq(op) => Ok(CustomExpr::Cmp {
                            left: *left,
                            op: CmpOp::Eq(op),
                            right: *right,
                        }),
                        BinOp::Ne(op) => Ok(CustomExpr::Cmp {
                            left: *left,
                            op: CmpOp::Ne(op),
                            right: *right,
                        }),
                        BinOp::Ge(op) => Ok(CustomExpr::Cmp {
                            left: *left,
                            op: CmpOp::Ge(op),
                            right: *right,
                        }),
                        BinOp::Le(op) => Ok(CustomExpr::Cmp {
                            left: *left,
                            op: CmpOp::Le(op),
                            right: *right,
                        }),
                        BinOp::Gt(op) => Ok(CustomExpr::Cmp {
                            left: *left,
                            op: CmpOp::Gt(op),
                            right: *right,
                        }),
                        BinOp::Lt(op) => Ok(CustomExpr::Cmp {
                            left: *left,
                            op: CmpOp::Lt(op),
                            right: *right,
                        }),
                        _ => unreachable!(),
                    },
                    expr => {
                        if lookahead.peek(Token![~]) {
                            let left = expr;
                            let op = CmpOp::Approx(input.parse()?);
                            let right: Expr = input.parse()?;
                            return Ok(CustomExpr::Cmp { left, op, right });
                        }

                        if lookahead.peek(Token![:]) {
                            let left = expr;
                            let op = CmpOp::Custom(input.parse()?, input.parse()?, input.parse()?);
                            let right = input.parse()?;
                            return Ok(CustomExpr::Cmp { left, op, right });
                        };

                        Ok(CustomExpr::Boolean(expr))
                    }
                }
            }
        }
    }
}

struct Operand {
    placeholder_id: Ident,
    diagnostic_expr: TokenStream,
}

enum AssertExpr {
    BoolExpr(Operand),
    CmpExpr {
        custom: bool,
        cmp: Operand,
        left: Operand,
        right: Operand,
    },
    AndExpr(Box<(AssertExpr, AssertExpr)>),
    OrExpr(Box<(AssertExpr, AssertExpr)>),
}

struct Code {
    assert_expr: TokenStream,
    source: TokenStream,
    source_type: TokenStream,
    debug_lhs: TokenStream,
    debug_rhs: TokenStream,
    debug_cmp: TokenStream,
}

impl AssertExpr {
    fn code(&self, crate_name: &Path) -> Code {
        match self {
            AssertExpr::BoolExpr(Operand {
                placeholder_id,
                diagnostic_expr: expr,
            }) => Code {
                assert_expr: quote! { (#placeholder_id).0.0.0 },
                source: quote! { #expr },
                source_type: quote! { &'static ::core::primitive::str },
                debug_lhs: quote! { () },
                debug_rhs: quote! { () },
                debug_cmp: quote! { #placeholder_id.0.0.0 },
            },
            AssertExpr::CmpExpr {
                custom,
                cmp:
                    Operand {
                        placeholder_id: cmp_placeholder_id,
                        diagnostic_expr: _,
                    },
                left:
                    Operand {
                        placeholder_id: left_placeholder_id,
                        diagnostic_expr: left_expr,
                    },
                right:
                    Operand {
                        placeholder_id: right_placeholder_id,
                        diagnostic_expr: right_expr,
                    },
            } => {
                let name = if *custom {
                    quote! { CustomCmpExpr }
                } else {
                    quote! { CmpExpr }
                };
                Code {
                    assert_expr: quote! {
                        #crate_name::expr::#name {
                            cmp: #cmp_placeholder_id,
                            lhs: #left_placeholder_id,
                            rhs: #right_placeholder_id,
                        }
                    },
                    source: quote! {
                        #crate_name::expr::#name {
                            cmp: (),
                            lhs: #left_expr,
                            rhs: #right_expr,
                        }
                    },
                    source_type: quote! {
                        #crate_name::expr::#name<
                            (),
                            &'static ::core::primitive::str,
                            &'static ::core::primitive::str,
                        >
                    },
                    debug_lhs: quote! { (#left_placeholder_id).get_ptr() },
                    debug_rhs: quote! { (#right_placeholder_id).get_ptr() },
                    debug_cmp: if *custom {
                        quote! { (&(#cmp_placeholder_id).0.0.0) as *const _ as *const () }
                    } else {
                        quote! { () }
                    },
                }
            }
            AssertExpr::AndExpr(inner) => {
                let (left, right) = &**inner;
                let Code {
                    assert_expr: left_assert_expr,
                    source: left_source,
                    source_type: left_source_type,
                    debug_lhs: left_debug_lhs,
                    debug_rhs: left_debug_rhs,
                    debug_cmp: left_debug_cmp,
                } = left.code(crate_name);
                let Code {
                    assert_expr: right_assert_expr,
                    source: right_source,
                    source_type: right_source_type,
                    debug_lhs: right_debug_lhs,
                    debug_rhs: right_debug_rhs,
                    debug_cmp: right_debug_cmp,
                } = right.code(crate_name);
                Code {
                    assert_expr: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_assert_expr),
                            rhs: (#right_assert_expr),
                        }
                    },
                    source: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_source),
                            rhs: (#right_source),
                        }
                    },
                    source_type: quote! {
                        #crate_name::expr::AndExpr<#left_source_type, #right_source_type>
                    },
                    debug_lhs: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_debug_lhs),
                            rhs: (#right_debug_lhs),
                        }
                    },
                    debug_rhs: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_debug_rhs),
                            rhs: (#right_debug_rhs),
                        }
                    },
                    debug_cmp: quote! {
                        #crate_name::expr::AndExpr {
                            lhs: (#left_debug_cmp),
                            rhs: (#right_debug_cmp),
                        }
                    },
                }
            }
            AssertExpr::OrExpr(inner) => {
                let (left, right) = &**inner;
                let Code {
                    assert_expr: left_assert_expr,
                    source: left_source,
                    source_type: left_source_type,
                    debug_lhs: left_debug_lhs,
                    debug_rhs: left_debug_rhs,
                    debug_cmp: left_debug_cmp,
                } = left.code(crate_name);
                let Code {
                    assert_expr: right_assert_expr,
                    source: right_source,
                    source_type: right_source_type,
                    debug_lhs: right_debug_lhs,
                    debug_rhs: right_debug_rhs,
                    debug_cmp: right_debug_cmp,
                } = right.code(crate_name);
                Code {
                    assert_expr: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_assert_expr),
                            rhs: (#right_assert_expr),
                        }
                    },
                    source: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_source),
                            rhs: (#right_source),
                        }
                    },
                    source_type: quote! {
                        #crate_name::expr::OrExpr<#left_source_type, #right_source_type>
                    },
                    debug_lhs: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_debug_lhs),
                            rhs: (#right_debug_lhs),
                        }
                    },
                    debug_rhs: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_debug_rhs),
                            rhs: (#right_debug_rhs),
                        }
                    },
                    debug_cmp: quote! {
                        #crate_name::expr::OrExpr {
                            lhs: (#left_debug_cmp),
                            rhs: (#right_debug_cmp),
                        }
                    },
                }
            }
        }
    }
}

fn usize_to_ident(idx: usize) -> Ident {
    Ident::new(&format!("__operand_{idx}"), Span::call_site())
}

fn cmp_usize_to_ident(idx: usize) -> Ident {
    Ident::new(&format!("__cmp_{idx}"), Span::call_site())
}

fn handle_expr(
    crate_name: &Path,
    atomics: &mut Vec<Expr>,
    cmp_atomics: &mut Vec<Expr>,
    diagnostics: &mut Vec<TokenStream>,
    mut placeholder_id: usize,
    mut cmp_placeholder_id: usize,
    expr: CustomExpr,
) -> (AssertExpr, usize, usize) {
    match expr {
        CustomExpr::All {
            all_token: _,
            paren_token: _,
            args,
        } => {
            let mut args = args.into_iter().collect::<Vec<_>>();
            if args.is_empty() {
                let expr = Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Bool(LitBool {
                        value: true,
                        span: Span::call_site(),
                    }),
                });
                let diagnostic_expr = quote! { ::core::stringify!(#expr) };
                atomics.push(expr);
                (
                    AssertExpr::BoolExpr(Operand {
                        placeholder_id: usize_to_ident(placeholder_id),
                        diagnostic_expr,
                    }),
                    placeholder_id + 1,
                    cmp_placeholder_id,
                )
            } else {
                let mut assert_expr;
                let mut arg_expr;
                (assert_expr, placeholder_id, cmp_placeholder_id) = handle_expr(
                    crate_name,
                    atomics,
                    cmp_atomics,
                    diagnostics,
                    placeholder_id,
                    cmp_placeholder_id,
                    args.pop().unwrap(),
                );
                while let Some(arg) = args.pop() {
                    (arg_expr, placeholder_id, cmp_placeholder_id) = handle_expr(
                        crate_name,
                        atomics,
                        cmp_atomics,
                        diagnostics,
                        placeholder_id,
                        cmp_placeholder_id,
                        arg,
                    );
                    assert_expr = AssertExpr::AndExpr(Box::new((arg_expr, assert_expr)));
                }
                (assert_expr, placeholder_id, cmp_placeholder_id)
            }
        }
        CustomExpr::Any {
            any_token: _,
            paren_token: _,
            args,
        } => {
            let mut args = args.into_iter().collect::<Vec<_>>();
            if args.is_empty() {
                let expr = Expr::Lit(ExprLit {
                    attrs: Vec::new(),
                    lit: Lit::Bool(LitBool {
                        value: false,
                        span: Span::call_site(),
                    }),
                });
                let diagnostic_expr = quote! { ::core::stringify!(#expr) };
                atomics.push(expr);
                (
                    AssertExpr::BoolExpr(Operand {
                        placeholder_id: usize_to_ident(placeholder_id),
                        diagnostic_expr,
                    }),
                    placeholder_id + 1,
                    cmp_placeholder_id,
                )
            } else {
                let mut assert_expr;
                let mut arg_expr;
                (assert_expr, placeholder_id, cmp_placeholder_id) = handle_expr(
                    crate_name,
                    atomics,
                    cmp_atomics,
                    diagnostics,
                    placeholder_id,
                    cmp_placeholder_id,
                    args.pop().unwrap(),
                );
                while let Some(arg) = args.pop() {
                    (arg_expr, placeholder_id, cmp_placeholder_id) = handle_expr(
                        crate_name,
                        atomics,
                        cmp_atomics,
                        diagnostics,
                        placeholder_id,
                        cmp_placeholder_id,
                        arg,
                    );
                    assert_expr = AssertExpr::OrExpr(Box::new((arg_expr, assert_expr)));
                }
                (assert_expr, placeholder_id, cmp_placeholder_id)
            }
        }
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Custom(_, cmp, _),
        } => handle_cmp(
            true,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |crate_name, cmp, lhs, rhs| quote! { #crate_name::Cmp::test(#cmp, #lhs, #rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            cmp,
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Approx(op),
        } => handle_cmp(
            true,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |crate_name, cmp, lhs, rhs| quote! { #crate_name::Cmp::test(#cmp, #lhs, #rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            Expr::Path(ExprPath {
                attrs: vec![],
                qself: None,
                path: Ident::new("approx_eq", op.spans[0]).into(),
            }),
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Eq(_),
        } => handle_cmp(
            false,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |_, _, lhs, rhs| quote! { *(#lhs) == *(#rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            make_cmp(crate_name, "Eq"),
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Ne(_),
        } => handle_cmp(
            false,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |_, _, lhs, rhs| quote! { *(#lhs) != *(#rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            make_cmp(crate_name, "Ne"),
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Lt(_),
        } => handle_cmp(
            false,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |_, _, lhs, rhs| quote! { *(#lhs) < *(#rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            make_cmp(crate_name, "Lt"),
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Gt(_),
        } => handle_cmp(
            false,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |_, _, lhs, rhs| quote! { *(#lhs) > *(#rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            make_cmp(crate_name, "Gt"),
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Le(_),
        } => handle_cmp(
            false,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |_, _, lhs, rhs| quote! { *(#lhs) <= *(#rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            make_cmp(crate_name, "Le"),
        ),
        CustomExpr::Cmp {
            left,
            right,
            op: CmpOp::Ge(_),
        } => handle_cmp(
            false,
            crate_name,
            atomics,
            cmp_atomics,
            diagnostics,
            |_, _, lhs, rhs| quote! { *(#lhs) >= *(#rhs) },
            placeholder_id,
            cmp_placeholder_id,
            left,
            right,
            make_cmp(crate_name, "Ge"),
        ),
        CustomExpr::Boolean(expr) => (
            AssertExpr::BoolExpr(Operand {
                placeholder_id: usize_to_ident(placeholder_id),
                diagnostic_expr: quote! { ::core::stringify!(#expr) },
            }),
            {
                let val = usize_to_ident(placeholder_id);
                diagnostics.push(quote! { *#val });
                atomics.push(expr);
                placeholder_id + 1
            },
            cmp_placeholder_id,
        ),
    }
}

fn make_cmp(crate_name: &Path, name: &str) -> Expr {
    let span = crate_name.span();
    let mut path = crate_name.clone();
    path.segments.push_punct(Token![::](crate_name.span()));
    path.segments.push_value(PathSegment {
        ident: Ident::new(name, span),
        arguments: PathArguments::None,
    });
    Expr::Path(ExprPath {
        attrs: vec![],
        qself: None,
        path,
    })
}

fn handle_cmp(
    custom: bool,
    crate_name: &Path,
    atomics: &mut Vec<Expr>,
    cmp_atomics: &mut Vec<Expr>,
    diagnostics: &mut Vec<TokenStream>,
    diagnose: fn(crate_name: &Path, cmp: Ident, lhs: Ident, rhs: Ident) -> TokenStream,
    placeholder_id: usize,
    cmp_placeholder_id: usize,
    left: Expr,
    right: Expr,
    cmp: Expr,
) -> (AssertExpr, usize, usize) {
    (
        AssertExpr::CmpExpr {
            custom,
            cmp: Operand {
                placeholder_id: cmp_usize_to_ident(cmp_placeholder_id),
                diagnostic_expr: quote! {},
            },
            left: Operand {
                placeholder_id: usize_to_ident(placeholder_id),
                diagnostic_expr: quote! { ::core::stringify!(#left) },
            },
            right: Operand {
                placeholder_id: usize_to_ident(placeholder_id + 1),
                diagnostic_expr: quote! { ::core::stringify!(#right) },
            },
        },
        {
            {
                let cmp = cmp_usize_to_ident(cmp_placeholder_id);
                let lhs = usize_to_ident(placeholder_id);
                let rhs = usize_to_ident(placeholder_id + 1);
                diagnostics.push(diagnose(crate_name, cmp, lhs, rhs));
            }
            cmp_atomics.push(cmp);
            atomics.push(left);
            atomics.push(right);
            placeholder_id + 2
        },
        cmp_placeholder_id + 1,
    )
}

type FormatArgs = syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>;

struct Args {
    crate_name: Path,
    expr: CustomExpr,
    format_args: Option<FormatArgs>,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let crate_name = input.parse()?;
        let _comma: syn::token::Comma = input.parse()?;
        let expr = input.parse()?;
        let format_args = if input.is_empty() {
            FormatArgs::new()
        } else {
            input.parse::<syn::token::Comma>()?;
            FormatArgs::parse_terminated(input)?
        };

        let format_args = Some(format_args).filter(|x| !x.is_empty());
        Ok(Self {
            crate_name,
            expr,
            format_args,
        })
    }
}

#[proc_macro]
pub fn assert(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as Args);

    let crate_name = &input.crate_name;
    let args = input.format_args;
    let body = input.expr;

    let mut atomics = Vec::new();
    let mut cmp_atomics = Vec::new();
    let mut diagnostics = Vec::new();
    let assert_expr = handle_expr(
        crate_name,
        &mut atomics,
        &mut cmp_atomics,
        &mut diagnostics,
        0,
        0,
        body.clone(),
    )
    .0;
    let atomics = atomics;
    let cmp_atomics = cmp_atomics;
    let placeholders = &*atomics
        .iter()
        .enumerate()
        .map(|(idx, _)| Ident::new(&format!("__operand_{idx}"), Span::call_site()))
        .collect::<Vec<_>>();

    let cmp_placeholders = &*cmp_atomics
        .iter()
        .enumerate()
        .map(|(idx, _)| Ident::new(&format!("__cmp_{idx}"), Span::call_site()))
        .collect::<Vec<_>>();

    let Code {
        assert_expr,
        source,
        source_type,
        debug_cmp,
        debug_lhs,
        debug_rhs,
    } = assert_expr.code(crate_name);

    let message = match args {
        Some(args) => quote! { #crate_name::Message(::core::format_args!(#args)) },
        None => quote! { #crate_name::NoMessage },
    };

    let outer_block = {
        quote! {
            match (#(&(#atomics),)* #(&(#cmp_atomics),)*) {
                (#(#placeholders,)* #(#cmp_placeholders,)*) => {
                    if false {
                        #(let _ = #diagnostics;)*
                    }
                    use #crate_name::spec::debug::TryDebugWrap;
                    use #crate_name::spec::sized::TrySizedWrap;
                    use #crate_name::spec::by_val::TryByValWrap;
                    use #crate_name::traits::Expr;

                    #(let #placeholders = (&&#crate_name::spec::Wrapper(#placeholders)).wrap_debug().do_wrap(#placeholders);)*
                    #(let #placeholders = (&&#crate_name::spec::Wrapper(#placeholders)).wrap_sized().do_wrap(#placeholders);)*
                    #(let #placeholders = (#placeholders).get();)*
                    #(let #placeholders = (&&#crate_name::spec::Wrapper(#placeholders)).wrap_by_val().do_wrap(#placeholders);)*

                    #(let #cmp_placeholders = #crate_name::spec::debug::CmpDebugWrapper(#cmp_placeholders);)*
                    #(let #cmp_placeholders = #crate_name::spec::sized::CmpSizedWrapper(#cmp_placeholders);)*
                    #(let #cmp_placeholders = #crate_name::spec::by_val::CmpByValWrapper(#cmp_placeholders).__wrap_ref();)*

                    let __assert_expr = #crate_name::structures::Finalize {
                        inner: #assert_expr,
                    };

                    if !(&&&__assert_expr).eval_expr() {
                        struct Source<'a, V>(pub &'a V);
                        impl<V: #crate_name::traits::DynInfoType> #crate_name::traits::DynInfoType for &Source<'_, V> {
                            type VTable = #crate_name::structures::WithSource<#source_type, &'static V::VTable>;
                            const NULL_VTABLE: &'static Self::VTable = &#crate_name::structures::WithSource {
                                source: #source,
                                file: ::core::file!(),
                                line: ::core::line!(),
                                col: ::core::column!(),
                                vtable: V::NULL_VTABLE,
                            };
                        }
                        impl<V: #crate_name::traits::DynInfo> #crate_name::traits::DynInfo for &Source<'_, V> {
                            const VTABLE: &'static Self::VTable = &#crate_name::structures::WithSource {
                                source: #source,
                                file: ::core::file!(),
                                line: ::core::line!(),
                                col: ::core::column!(),
                                vtable: V::VTABLE,
                            };
                        }
                        impl<V> #crate_name::traits::DynInfoType for Source<'_, V> {
                            type VTable = #crate_name::structures::WithSource<&'static str, &'static ()>;
                            const NULL_VTABLE: &'static Self::VTable = &#crate_name::structures::WithSource {
                                source: "",
                                file: ::core::file!(),
                                line: ::core::line!(),
                                col: ::core::column!(),
                                vtable: &(),
                            };
                        }
                        impl<V> #crate_name::traits::DynInfo for Source<'_, V> {
                            const VTABLE: &'static Self::VTable = <Self as #crate_name::traits::DynInfoType>::NULL_VTABLE;
                        }

                        #[allow(clippy::useless_transmute)]
                        #crate_name::panic_failed_assert(
                            (&&&__assert_expr).__marker(),
                            unsafe { ::core::mem::transmute(#debug_lhs) },
                            unsafe { ::core::mem::transmute(#debug_rhs) },
                            unsafe { ::core::mem::transmute(#debug_cmp) },
                            {
                                use #crate_name::traits::DynInfo;
                                (&&Source(&__assert_expr)).vtable()
                            },
                            #message,
                        );
                    }
                }
            }
        }
    };

    outer_block.into()
}
