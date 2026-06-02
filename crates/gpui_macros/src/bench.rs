use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse::Parser, spanned::Spanned};

/// Default target frame rate when a benchmark doesn't specify `fps = N`.
const DEFAULT_FPS: u64 = 120;

pub fn bench(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut fps: Option<u64> = None;
    if !args.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("fps") {
                let value: syn::LitInt = meta.value()?.parse()?;
                let value = value.base10_parse::<u64>()?;
                if value == 0 {
                    return Err(meta.error("#[gpui::bench] `fps` must be greater than zero"));
                }
                fps = Some(value);
                Ok(())
            } else {
                Err(meta.error("#[gpui::bench] only accepts `fps = N`"))
            }
        });
        if let Err(error) = parser.parse(args) {
            return error_to_stream(error);
        }
    }

    let frame_budget_nanos: u128 = 1_000_000_000 / fps.unwrap_or(DEFAULT_FPS) as u128;

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
            let report = gpui::BenchReport::with_frame_budget_nanos(#frame_budget_nanos);
            criterion.bench_function(stringify!(#outer_fn_name), {
                let report = report.clone();
                move |bencher| {
                    let mut cx = gpui::BenchAppContext::new_with_platform_and_report(
                        gpui_platform::current_platform(true),
                        Some(stringify!(#outer_fn_name)),
                        bencher,
                        report.clone(),
                    );
                    #inner_fn_name(&mut cx);
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
