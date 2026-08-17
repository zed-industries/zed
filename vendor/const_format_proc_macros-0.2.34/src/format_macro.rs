use crate::{
    format_args::{ExpandInto, FormatArgs, FormatIfArgs, LocalVariable, WriteArgs},
    parse_utils::TokenStream2Ext,
    shared_arg_parsing::{ExprArg, ExprArgs},
    Error,
};

use proc_macro2::{Ident, Span, TokenStream as TokenStream2};

use quote::{quote, quote_spanned};

#[cfg(test)]
mod tests;

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn concatcp_impl(value: ExprArgs) -> Result<TokenStream2, crate::Error> {
    let fmt_var = Ident::new("fmt", Span::mixed_site());

    let concat_args = value.args.iter().map(|ExprArg { expr, span }| {
        quote_spanned!(span.start=>
            __cf_osRcTFl4A::pmr::PConvWrapper(#expr).to_pargument_display(#fmt_var)
        )
    });

    Ok(quote!(({
        // The suffix is to avoid name collisions with identifiers in the passed-in expression.
        #[doc(hidden)]
        #[allow(unused_mut, non_snake_case)]
        const CONCATP_NHPMWYD3NJA : &[__cf_osRcTFl4A::pmr::PArgument] = {
            let #fmt_var = __cf_osRcTFl4A::pmr::FormattingFlags::NEW;

            &[
                #( #concat_args ),*
            ]
        };

        __cf_osRcTFl4A::__concatcp_inner!(CONCATP_NHPMWYD3NJA)
    })))
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn formatcp_if_macro_impl(value: FormatIfArgs) -> Result<TokenStream2, crate::Error> {
    formatcp_impl(value.inner)
}

pub(crate) fn formatcp_impl(fmt_args: FormatArgs) -> Result<TokenStream2, crate::Error> {
    let locals = fmt_args
        .local_variables
        .iter()
        .map(|LocalVariable { ident, expr }| {
            let span = ident.span();
            quote_spanned!(span=> let #ident = #expr;)
        });

    for ei in fmt_args.expanded_into.iter() {
        if let ExpandInto::WithFormatter(wf) = ei {
            return Err(crate::Error::new(
                wf.fmt_ident.span(),
                "Can't do custom formatting in the `formatcp` macro",
            ));
        }
    }

    let parg_constructor = fmt_args.expanded_into.iter().map(|ei| match ei {
        ExpandInto::Str(str, rawness) => {
            let str_tokens = rawness.tokenize_sub(str);
            quote!(
                __cf_osRcTFl4A::pmr::PConvWrapper(#str_tokens)
                    .to_pargument_display(__cf_osRcTFl4A::pmr::FormattingFlags::NEW)
            )
        }
        ExpandInto::Formatted(fmted) => {
            let to_pargument_m = fmted.format.to_pargument_method_name();
            let formatting = fmted.format;
            let local_variable = &fmted.local_variable;
            let span = local_variable.span();
            // I had to use `set_span_recursive` to set the span to that of the argument,
            // quote_span doesn't work for that somehow.
            quote!(
                __cf_osRcTFl4A::pmr::PConvWrapper(#local_variable).#to_pargument_m(#formatting)
            )
            .set_span_recursive(span)
        }
        ExpandInto::WithFormatter { .. } => unreachable!(),
    });

    let fmt_if_true = quote!({
        let mut len = 0usize;

        #( #locals )*

        &[
            #( #parg_constructor ),*
        ]
    });

    if let Some(cond) = fmt_args.condition {
        Ok(quote!(({
            enum __Fooosrctfl4a {}

            // This is generic so that the constant is only evaluated when it's needed.
            impl<T> __cf_osRcTFl4A::pmr::ConcatArgsIf<T, true> for __Fooosrctfl4a {
                #[doc(hidden)]
                const PARGUMENTS : &'static [__cf_osRcTFl4A::pmr::PArgument] = #fmt_if_true;
            }

            __cf_osRcTFl4A::__concatcp_inner!(
                <__Fooosrctfl4a as __cf_osRcTFl4A::pmr::ConcatArgsIf<(), #cond>>::PARGUMENTS
            )
        })))
    } else {
        Ok(quote!(({
            // The suffix is to avoid name collisions with identifiers in the passed-in expression.
            #[doc(hidden)]
            #[allow(unused_mut, non_snake_case)]
            const CONCATP_NHPMWYD3NJA : &[__cf_osRcTFl4A::pmr::PArgument] = #fmt_if_true;

            __cf_osRcTFl4A::__concatcp_inner!(CONCATP_NHPMWYD3NJA)
        })))
    }
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn formatc_if_macro_impl(value: FormatIfArgs) -> Result<TokenStream2, crate::Error> {
    formatc_macro_impl(value.inner)
}

////////////////////////////////////////////////////////////////////////////////

pub(crate) fn formatc_macro_impl(fmt_args: FormatArgs) -> Result<TokenStream2, crate::Error> {
    let locals = fmt_args.local_variables.iter().map(|arg| &arg.ident);
    let expr = fmt_args.local_variables.iter().map(|arg| &arg.expr);

    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let writing_formatted = fmt_args
        .expanded_into
        .iter()
        .map(|ei| ei.fmt_call(&strwriter));

    let cond_a = fmt_args.condition.iter();

    Ok(quote!(({
        #[doc(hidden)]
        #[allow(non_snake_case)]
        const fn fmt_NHPMWYD3NJA(
            mut #strwriter: __cf_osRcTFl4A::fmt::Formatter<'_>,
        ) -> __cf_osRcTFl4A::Result {
            match (#(&(#expr),)*) {
                (#(#locals,)*) => {
                    #(
                        __cf_osRcTFl4A::try_!(#writing_formatted);
                    )*
                },
            }
            __cf_osRcTFl4A::pmr::Ok(())
        }

        __cf_osRcTFl4A::__concatc_inner!(
            fmt_NHPMWYD3NJA,
            #((#cond_a) && )* true,
            ____
        )
    })))
}

pub(crate) fn writec_macro_impl(value: WriteArgs) -> Result<TokenStream2, Error> {
    let writer_expr = value.writer_expr;
    let writer_span = value.writer_span;
    let FormatArgs {
        condition: _,
        expanded_into,
        local_variables,
    } = value.format_args;

    let locals = local_variables.iter().map(|arg| &arg.ident);
    let expr = local_variables.iter().map(|arg| &arg.expr);

    let strwriter = Ident::new("strwriter", Span::mixed_site());

    let writing_formatted = expanded_into.iter().map(|ei| ei.fmt_call(&strwriter));

    let borrow_mutably = quote_spanned!(writer_span=> ((#writer_expr).borrow_mutably()));

    let make_formatter = quote_spanned!(writer_span =>
        let mut marker = __cf_osRcTFl4A::pmr::IsAWriteMarker::NEW;
        if false {
            marker = marker.infer_type(&#strwriter);
        }
        let mut #strwriter = marker.coerce(#strwriter);
        let mut #strwriter =
            #strwriter.make_formatter(__cf_osRcTFl4A::FormattingFlags::NEW);
    );

    Ok(quote! {({

        #[allow(non_snake_case)]
        match (#borrow_mutably, #(&(#expr),)*) {
            (#strwriter, #(#locals,)*) => {
                #make_formatter

                loop {
                    #(
                        __cf_osRcTFl4A::unwrap_or_else!(
                            #writing_formatted,
                            |e| break __cf_osRcTFl4A::pmr::Err(e)
                        );
                    )*
                    break __cf_osRcTFl4A::pmr::Ok(());
                }
            }
        }
    })})
}
