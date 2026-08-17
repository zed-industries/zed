use crate::spanned::Spans;

use proc_macro2::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream as TokenStream2,
    TokenTree as TokenTree2,
};

use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Error {
    messages: Vec<CompileError>,
}

#[derive(Debug, Clone)]
enum CompileError {
    Basic {
        start_span: Span,
        end_span: Span,
        msg: String,
    },
    #[cfg(feature = "derive")]
    Syn(TokenStream2),
}

impl Error {
    pub fn new<T: Display>(span: Span, msg: T) -> Self {
        Error {
            messages: vec![CompileError::Basic {
                start_span: span,
                end_span: span,
                msg: msg.to_string(),
            }],
        }
    }

    pub fn spanned<T: Display>(spans: Spans, msg: T) -> Self {
        Error {
            messages: vec![CompileError::Basic {
                start_span: spans.start,
                end_span: spans.end,
                msg: msg.to_string(),
            }],
        }
    }

    pub fn to_compile_error(&self) -> TokenStream2 {
        macro_rules!  tokenstream{
            ($($tt:expr),* $(,)*) => ({
                let list: Vec<TokenTree2> = vec![
                    $($tt.into(),)*
                ];
                list.into_iter().collect::<TokenStream2>()
            })
        }

        self.messages
            .iter()
            .map(|em| match em {
                CompileError::Basic {
                    start_span,
                    end_span,
                    msg,
                } => {
                    let ts = tokenstream![
                        Ident::new("compile_error", *start_span),
                        {
                            let mut this = Punct::new('!', Spacing::Alone);
                            this.set_span(*start_span);
                            this
                        },
                        {
                            let mut group = Group::new(
                                Delimiter::Parenthesis,
                                tokenstream![{
                                    let mut lit = Literal::string(msg);
                                    lit.set_span(*end_span);
                                    TokenTree2::Literal(lit)
                                }],
                            );
                            group.set_span(*end_span);
                            group
                        },
                    ];

                    // Still have no idea why the compile_error has to be wrapped in parentheses
                    // so that the spans point at the stuff between start_span and end_span.
                    let mut this = Group::new(Delimiter::Parenthesis, ts);
                    this.set_span(*end_span);
                    tokenstream![this]
                }
                #[cfg(feature = "derive")]
                CompileError::Syn(x) => x.clone(),
            })
            .collect()
    }

    pub fn combine(&mut self, another: Error) {
        self.messages.extend(another.messages)
    }
}

#[cfg(feature = "derive")]
impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Self {
            messages: vec![CompileError::Syn(err.to_compile_error())],
        }
    }
}
