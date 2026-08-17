use crate::{
    format_str::FormatStr, formatting::FormattingFlags, parse_utils::StrRawness,
    parse_utils::TokenStream2Ext, shared_arg_parsing::ExprArg, spanned::Spans,
};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use quote::{quote_spanned, TokenStreamExt};

////////////////////////////////////////////////

mod parsing;

////////////////////////////////////////////////

struct UncheckedFormatArgs {
    literal: FormatStr,
    args: Vec<UncheckedFormatArg>,
}

struct UncheckedFormatArg {
    pub(crate) spans: Spans,
    pub(crate) ident: Option<Ident>,
    // The identifier for the Formatter passed to format the argument.
    // If this is Some, then `expr` is expanded directly,
    pub(crate) fmt_ident: Option<Ident>,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    pub(crate) expr: TokenStream2,
}

pub(crate) struct FormatArgs {
    pub(crate) condition: Option<ExprArg>,
    pub(crate) local_variables: Vec<LocalVariable>,
    pub(crate) expanded_into: Vec<ExpandInto>,
}

pub(crate) struct FormatIfArgs {
    pub(crate) inner: FormatArgs,
}

/// The arguments of `writec`
pub(crate) struct WriteArgs {
    pub(crate) writer_expr: TokenStream2,
    pub(crate) writer_span: Span,
    pub(crate) format_args: FormatArgs,
}

pub(crate) enum ExpandInto {
    Str(String, StrRawness),
    Formatted(ExpandFormatted),
    WithFormatter(ExpandWithFormatter),
}

pub(crate) struct ExpandFormatted {
    pub(crate) format: FormattingFlags,
    pub(crate) local_variable: Ident,
}

pub(crate) struct ExpandWithFormatter {
    pub(crate) format: FormattingFlags,
    pub(crate) fmt_ident: Ident,
    pub(crate) expr: TokenStream2,
}

pub(crate) struct LocalVariable {
    // The local variable that the macro will output for this argument,
    // so that it is not evaluated multiple times when it's used multiple times
    // in the format string.
    pub(crate) ident: Ident,
    /// Using a TokenStream2 because it is validated to be a valid expression in
    /// the macro_rules! macros that call these proc macros.
    pub(crate) expr: TokenStream2,
}

pub(crate) enum FormatArg {
    WithFormatter {
        // The identifier for the Formatter passed to format the argument.
        // If this is Some, then `expr` is expanded directly,
        fmt_ident: Ident,
        /// Using a TokenStream2 because it is validated to be a valid expression in
        /// the macro_rules! macros that call these proc macros.
        expr: TokenStream2,
    },
    WithLocal(Ident),
}

////////////////////////////////////////////////

impl ExpandInto {
    pub(crate) fn fmt_call(&self, formatter: &Ident) -> TokenStream2 {
        match self {
            ExpandInto::Str(str, rawness) => {
                let str_tokens = rawness.tokenize_sub(str);

                quote_spanned!(rawness.span()=> #formatter.write_str(#str_tokens) )
            }
            ExpandInto::Formatted(fmted) => {
                let flags = fmted.format;
                let fmt_method = fmted.format.fmt_method_name();
                let local_variable = &fmted.local_variable;
                let span = local_variable.span();

                let mut tokens = quote::quote!(
                    __cf_osRcTFl4A::coerce_to_fmt!(&#local_variable)
                        .#fmt_method
                )
                .set_span_recursive(span);

                tokens.append_all(quote::quote!( (&mut #formatter.make_formatter(#flags)) ));

                tokens
            }
            ExpandInto::WithFormatter(ExpandWithFormatter {
                format,
                fmt_ident,
                expr,
            }) => quote::quote!({
                let #fmt_ident = &mut #formatter.make_formatter(#format);
                __cf_osRcTFl4A::pmr::ToResult( #expr ).to_result()
            }),
        }
    }
}
