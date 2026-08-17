mod panic_context;

use crate::util::prelude::*;
use proc_macro2::{Group, TokenTree};
use std::panic::AssertUnwindSafe;
use syn::parse::Parse;

/// Handle the error or panic returned from the macro logic.
///
/// The error may be either a syntax error or a logic error. In either case, we
/// want to return a [`TokenStream`] that still provides good IDE experience.
/// See [`Fallback`] for details.
///
/// This function also catches panics. Importantly, we don't use panics for error
/// handling! A panic is always a bug! However, we still handle it to provide
/// better IDE experience even if there are some bugs in the macro implementation.
///
/// One known bug that may cause panics when using Rust Analyzer is the following one:
/// <https://github.com/rust-lang/rust-analyzer/issues/18244>
pub(crate) fn handle_errors(
    item: TokenStream,
    imp: impl FnOnce() -> Result<TokenStream>,
) -> Result<TokenStream, TokenStream> {
    let panic_listener = panic_context::PanicListener::register();

    std::panic::catch_unwind(AssertUnwindSafe(imp))
        .unwrap_or_else(|err| {
            let msg = panic_context::message_from_panic_payload(err.as_ref())
                .unwrap_or_else(|| "<unknown error message>".to_owned());

            let msg = if msg.contains("unsupported proc macro punctuation character") {
                format!(
                    "known bug in rust-analyzer: {msg};\n\
                    Github issue: https://github.com/rust-lang/rust-analyzer/issues/18244"
                )
            } else {
                let context = panic_listener
                    .get_last_panic()
                    .map(|ctx| format!("\n\n{ctx}"))
                    .unwrap_or_default();

                format!(
                    "proc-macro panicked (may be a bug in the crate `bon`): {msg};\n\
                    please report this issue at our Github repository: \
                    https://github.com/elastio/bon{context}"
                )
            };

            Err(err!(&Span::call_site(), "{msg}"))
        })
        .map_err(|err| {
            let compile_error = err.write_errors();
            let item = strip_invalid_tt(item);

            syn::parse2::<Fallback>(item)
                .map(|fallback| quote!(#compile_error #fallback))
                .unwrap_or_else(|_| compile_error)
        })
}

/// This is used in error handling for better IDE experience. For example, while
/// the developer is writing the function code they'll have a bunch of syntax
/// errors in the process. While that happens the proc macro should output at
/// least some representation of the input code that the developer wrote with
/// a separate compile error entry. This keeps the syntax highlighting and IDE
/// type analysis, completions and other hints features working even if macro
/// fails to parse some syntax or finds some other logic errors.
///
/// This utility does very low-level parsing to strip doc comments from the
/// input. This is to prevent the IDE from showing errors that "doc comments
/// aren't allowed on function arguments". It also removes `#[builder(...)]`
/// attributes that need to be processed by this macro to avoid the IDE from
/// reporting those as well.
struct Fallback {
    output: TokenStream,
}

impl Parse for Fallback {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let mut output = TokenStream::new();

        loop {
            let found_attr = input.step(|cursor| {
                let mut cursor = *cursor;
                while let Some((tt, next)) = cursor.token_tree() {
                    match &tt {
                        TokenTree::Group(group) => {
                            let fallback: Self = syn::parse2(group.stream())?;
                            let new_group = Group::new(group.delimiter(), fallback.output);
                            output.extend([TokenTree::Group(new_group)]);
                        }
                        TokenTree::Punct(punct) if punct.as_char() == '#' => {
                            return Ok((true, cursor));
                        }
                        TokenTree::Punct(_) | TokenTree::Ident(_) | TokenTree::Literal(_) => {
                            output.extend([tt]);
                        }
                    }

                    cursor = next;
                }

                Ok((false, cursor))
            })?;

            if !found_attr {
                return Ok(Self { output });
            }

            input
                .call(syn::Attribute::parse_outer)?
                .into_iter()
                .filter(|attr| !attr.is_doc_expr() && !attr.path().is_ident("builder"))
                .for_each(|attr| attr.to_tokens(&mut output));
        }
    }
}

impl ToTokens for Fallback {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.output.to_tokens(tokens);
    }
}

/// Workaround for the RA bug where it generates an invalid Punct token tree with
/// the character `{`.
///
/// ## Issues
///
/// - [Bug in RA](https://github.com/rust-lang/rust-analyzer/issues/18244)
/// - [Bug in proc-macro2](https://github.com/dtolnay/proc-macro2/issues/470) (already fixed)
fn strip_invalid_tt(tokens: TokenStream) -> TokenStream {
    fn recurse(tt: TokenTree) -> TokenTree {
        match &tt {
            TokenTree::Group(group) => {
                let mut group = Group::new(group.delimiter(), strip_invalid_tt(group.stream()));
                group.set_span(group.span());

                TokenTree::Group(group)
            }
            _ => tt,
        }
    }

    let mut tokens = tokens.into_iter();

    std::iter::from_fn(|| {
        // In newer versions of `proc-macro2` this code panics here (earlier)
        loop {
            // If this panics it means the next token tree is invalid.
            // We can't do anything about it, and we just ignore it.
            // Luckily, `proc-macro2` consumes the invalid token tree
            // so this doesn't cause an infinite loop.
            if let Ok(tt) = std::panic::catch_unwind(AssertUnwindSafe(|| tokens.next())) {
                return tt.map(recurse);
            }
        }
    })
    .collect()
}
