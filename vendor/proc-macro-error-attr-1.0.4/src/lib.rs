//! This is `#[proc_macro_error]` attribute to be used with
//! [`proc-macro-error`](https://docs.rs/proc-macro-error/). There you go.

extern crate proc_macro;

use crate::parse::parse_input;
use crate::parse::Attribute;
use proc_macro::TokenStream;
use proc_macro2::{Literal, Span, TokenStream as TokenStream2, TokenTree};
use quote::{quote, quote_spanned};

use crate::settings::{Setting::*, *};

mod parse;
mod settings;

type Result<T> = std::result::Result<T, Error>;

struct Error {
    span: Span,
    message: String,
}

impl Error {
    fn new(span: Span, message: String) -> Self {
        Error { span, message }
    }

    fn into_compile_error(self) -> TokenStream2 {
        let mut message = Literal::string(&self.message);
        message.set_span(self.span);
        quote_spanned!(self.span=> compile_error!{#message})
    }
}

#[proc_macro_attribute]
pub fn proc_macro_error(attr: TokenStream, input: TokenStream) -> TokenStream {
    match impl_proc_macro_error(attr.into(), input.clone().into()) {
        Ok(ts) => ts,
        Err(e) => {
            let error = e.into_compile_error();
            let input = TokenStream2::from(input);

            quote!(#input #error).into()
        }
    }
}

fn impl_proc_macro_error(attr: TokenStream2, input: TokenStream2) -> Result<TokenStream> {
    let (attrs, signature, body) = parse_input(input)?;
    let mut settings = parse_settings(attr)?;

    let is_proc_macro = is_proc_macro(&attrs);
    if is_proc_macro {
        settings.set(AssertUnwindSafe);
    }

    if detect_proc_macro_hack(&attrs) {
        settings.set(ProcMacroHack);
    }

    if settings.is_set(ProcMacroHack) {
        settings.set(AllowNotMacro);
    }

    if !(settings.is_set(AllowNotMacro) || is_proc_macro) {
        return Err(Error::new(
            Span::call_site(),
            "#[proc_macro_error] attribute can be used only with procedural macros\n\n  \
            = hint: if you are really sure that #[proc_macro_error] should be applied \
            to this exact function, use #[proc_macro_error(allow_not_macro)]\n"
                .into(),
        ));
    }

    let body = gen_body(body, settings);

    let res = quote! {
        #(#attrs)*
        #(#signature)*
        { #body }
    };
    Ok(res.into())
}

#[cfg(not(always_assert_unwind))]
fn gen_body(block: TokenTree, settings: Settings) -> proc_macro2::TokenStream {
    let is_proc_macro_hack = settings.is_set(ProcMacroHack);
    let closure = if settings.is_set(AssertUnwindSafe) {
        quote!(::std::panic::AssertUnwindSafe(|| #block ))
    } else {
        quote!(|| #block)
    };

    quote!( ::proc_macro_error::entry_point(#closure, #is_proc_macro_hack) )
}

// FIXME:
// proc_macro::TokenStream does not implement UnwindSafe until 1.37.0.
// Considering this is the closure's return type the unwind safety check would fail
// for virtually every closure possible, the check is meaningless.
#[cfg(always_assert_unwind)]
fn gen_body(block: TokenTree, settings: Settings) -> proc_macro2::TokenStream {
    let is_proc_macro_hack = settings.is_set(ProcMacroHack);
    let closure = quote!(::std::panic::AssertUnwindSafe(|| #block ));
    quote!( ::proc_macro_error::entry_point(#closure, #is_proc_macro_hack) )
}

fn detect_proc_macro_hack(attrs: &[Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| attr.path_is_ident("proc_macro_hack"))
}

fn is_proc_macro(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.path_is_ident("proc_macro")
            || attr.path_is_ident("proc_macro_derive")
            || attr.path_is_ident("proc_macro_attribute")
    })
}
