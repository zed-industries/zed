use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, parse::Parser, spanned::Spanned};

pub fn bench(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut fps: Option<u64> = None;
    let mut platform_text_system = false;
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
            } else if meta.path.is_ident("text_system") {
                let value: syn::Ident = meta.value()?.parse()?;
                match value.to_string().as_str() {
                    "platform" => {
                        platform_text_system = true;
                        Ok(())
                    }
                    "noop" => {
                        platform_text_system = false;
                        Ok(())
                    }
                    _ => {
                        Err(meta.error("#[gpui::bench] `text_system` must be `platform` or `noop`"))
                    }
                }
            } else {
                Err(meta.error("#[gpui::bench] only accepts `fps = N` and `text_system = ...`"))
            }
        });
        if let Err(error) = parser.parse(args) {
            return error_to_stream(error);
        }
    }

    let text_system_expr = if platform_text_system {
        quote! { gpui_platform::current_platform_text_system() }
    } else {
        quote! { None }
    };

    // The frame budget math lives in `BenchReport` so `bench_context` is the
    // single source of truth; `default()` supplies the default frame rate.
    let report_expr = match fps {
        Some(fps) => quote! { gpui::BenchReport::with_fps(#fps) },
        None => quote! { gpui::BenchReport::default() },
    };

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
            let report = #report_expr;
            criterion.bench_function(stringify!(#outer_fn_name), {
                let report = report.clone();
                move |bencher| {
                    let mut cx = gpui::BenchAppContext::new_with_platform_and_report(
                        gpui::bench_platform(
                            Some(Box::new(|| {
                                gpui_platform::current_headless_renderer()
                            })),
                            #text_system_expr,
                        ),
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
