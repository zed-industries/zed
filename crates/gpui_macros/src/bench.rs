use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, spanned::Spanned};

pub fn bench(args: TokenStream, function: TokenStream) -> TokenStream {
    if !args.is_empty() {
        return error_to_stream(syn::Error::new(
            proc_macro2::TokenStream::from(args).span(),
            "#[gpui::bench] does not accept arguments yet",
        ));
    }

    let mut inner_fn = match syn::parse::<ItemFn>(function) {
        Ok(function) => function,
        Err(error) => return error_to_stream(error),
    };

    if let Some(asyncness) = &inner_fn.sig.asyncness {
        return error_to_stream(syn::Error::new(
            asyncness.span(),
            "#[gpui::bench] does not support async benchmark functions yet",
        ));
    }

    let outer_fn_name = inner_fn.sig.ident.clone();
    let inner_fn_name = format_ident!("__gpui_bench_{}", outer_fn_name);
    inner_fn.sig.ident = inner_fn_name.clone();

    TokenStream::from(quote! {
        #inner_fn

        fn #outer_fn_name(criterion: &mut criterion::Criterion) {
            let report = gpui::BenchReport::default();
            criterion.bench_function(stringify!(#outer_fn_name), {
                let report = report.clone();
                move |bencher| {
                    let mut cx = gpui::BenchAppContext::new_with_report(
                        Some(stringify!(#outer_fn_name)),
                        report.clone(),
                    );
                    #inner_fn_name(bencher, &mut cx);
                    cx.teardown();
                }
            });
            report.print(Some(stringify!(#outer_fn_name)));
        }

    })
}

fn error_to_stream(error: syn::Error) -> TokenStream {
    TokenStream::from(error.into_compile_error())
}
