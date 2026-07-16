use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Expr, ItemFn, LitStr, parse::Parser, spanned::Spanned};

pub fn bench(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut fps: Option<u64> = None;
    let mut inputs: Option<Expr> = None;
    let mut input_name: Option<LitStr> = None;
    let mut group_name: Option<LitStr> = None;
    let mut sample_size: Option<usize> = None;
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
            } else if meta.path.is_ident("inputs") {
                inputs = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("input_name") {
                input_name = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("group") {
                group_name = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("sample_size") {
                let value: syn::LitInt = meta.value()?.parse()?;
                let value = value.base10_parse::<usize>()?;
                if value == 0 {
                    return Err(
                        meta.error("#[gpui::bench] `sample_size` must be greater than zero")
                    );
                }
                sample_size = Some(value);
                Ok(())
            } else {
                Err(meta.error(
                    "#[gpui::bench] only accepts `fps = N`, `inputs = EXPR`, `input_name = \"...\"`, `group = \"...\"`, and `sample_size = N`",
                ))
            }
        });
        if let Err(error) = parser.parse(args) {
            return error_to_stream(error);
        }
    }

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

    let benchmark = if let Some(inputs) = inputs {
        let input_name = match input_name {
            Some(input_name) => quote! { #input_name },
            None => quote! { stringify!(#outer_fn_name) },
        };
        let group_name = match group_name {
            Some(group_name) => quote! { #group_name },
            None => quote! { stringify!(#outer_fn_name) },
        };
        let sample_size =
            sample_size.map(|sample_size| quote! { group.sample_size(#sample_size); });
        quote! {
            let report = #report_expr;
            let mut group = criterion.benchmark_group(#group_name);
            #sample_size
            for input in #inputs {
                group.bench_with_input(criterion::BenchmarkId::new(#input_name, &input), &input, {
                    let report = report.clone();
                    move |bencher, input| {
                        let mut cx = gpui::BenchAppContext::new_with_platform_and_report(
                            gpui::bench_platform(
                                Some(Box::new(|| {
                                    gpui_platform::current_headless_renderer()
                                })),
                                gpui_platform::current_platform(true).text_system(),
                            ),
                            Some(stringify!(#outer_fn_name)),
                            bencher,
                            report.clone(),
                        );
                        #inner_fn_name(input, &mut cx);
                        cx.teardown();
                    }
                });
            }
            group.finish();
            report.print(Some(stringify!(#outer_fn_name)));
        }
    } else {
        if let Some(input_name) = input_name {
            return error_to_stream(syn::Error::new(
                input_name.span(),
                "#[gpui::bench] `input_name` requires `inputs`",
            ));
        }
        if let Some(group_name) = group_name {
            return error_to_stream(syn::Error::new(
                group_name.span(),
                "#[gpui::bench] `group` requires `inputs`",
            ));
        }
        if sample_size.is_some() {
            return error_to_stream(syn::Error::new(
                proc_macro2::Span::call_site(),
                "#[gpui::bench] `sample_size` requires `inputs`",
            ));
        }
        quote! {
            let report = #report_expr;
            criterion.bench_function(stringify!(#outer_fn_name), {
                let report = report.clone();
                move |bencher| {
                    let mut cx = gpui::BenchAppContext::new_with_platform_and_report(
                        gpui::bench_platform(
                            Some(Box::new(|| {
                                gpui_platform::current_headless_renderer()
                            })),
                            gpui_platform::current_platform(true).text_system(),
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
    };

    TokenStream::from(quote! {
        #inner_fn

        fn #outer_fn_name(criterion: &mut criterion::Criterion) {
            #benchmark
        }

    })
}

fn error_to_stream(error: syn::Error) -> TokenStream {
    TokenStream::from(error.into_compile_error())
}
