use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    Expr, FnArg, ItemFn, LitInt, LitStr, Token, Type, parse::Parser, punctuated::Punctuated,
    spanned::Spanned,
};

pub fn bench(args: TokenStream, function: TokenStream) -> TokenStream {
    let mut fps: Option<u64> = None;
    let mut inputs: Option<Expr> = None;
    let mut input_name: Option<LitStr> = None;
    let mut group_name: Option<LitStr> = None;
    let mut sample_size: Option<usize> = None;
    let mut explicit_seeds: Option<Vec<u64>> = None;
    let mut iterations: Option<u64> = None;
    if !args.is_empty() {
        let parser = syn::meta::parser(|meta| {
            if meta.path.is_ident("seed") || meta.path.is_ident("seeds") {
                if explicit_seeds.is_some() {
                    return Err(meta
                        .error("#[gpui::bench] accepts one of `seed = N` or `seeds(...)`, once"));
                }
                let seeds = if meta.path.is_ident("seed") {
                    vec![meta.value()?.parse::<LitInt>()?]
                } else {
                    let content;
                    syn::parenthesized!(content in meta.input);
                    Punctuated::<LitInt, Token![,]>::parse_terminated(&content)?
                        .into_iter()
                        .collect()
                };
                explicit_seeds = Some(
                    seeds
                        .iter()
                        .map(|seed| seed.base10_parse::<u64>())
                        .collect::<syn::Result<_>>()?,
                );
                Ok(())
            } else if meta.path.is_ident("iterations") {
                let value: LitInt = meta.value()?.parse()?;
                let value = value.base10_parse::<u64>()?;
                if value == 0 {
                    return Err(meta.error("#[gpui::bench] `iterations` must be greater than zero"));
                }
                iterations = Some(value);
                Ok(())
            } else if meta.path.is_ident("fps") {
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
                    "#[gpui::bench] only accepts `fps = N`, `inputs = EXPR`, `input_name = \"...\"`, `group = \"...\"`, `sample_size = N`, `seed = N`, `seeds(...)`, and `iterations = N`",
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

    // A `StdRng` parameter may appear anywhere; the others are, in order, the input
    // (when `inputs` is given) and the benchmark context.
    let expected_signature = if inputs.is_some() {
        "`(input: &Input, cx: &mut BenchAppContext)`"
    } else {
        "`(cx: &mut BenchAppContext)`"
    };
    let mut call_arguments = Vec::new();
    let mut takes_rng = false;
    let mut positional_count = 0;
    for argument in &inner_fn.sig.inputs {
        let FnArg::Typed(argument) = argument else {
            return error_to_stream(syn::Error::new(
                argument.span(),
                "#[gpui::bench] functions cannot take `self`",
            ));
        };
        if is_std_rng(&argument.ty) {
            if takes_rng {
                return error_to_stream(syn::Error::new(
                    argument.span(),
                    "#[gpui::bench] provides one `StdRng`",
                ));
            }
            takes_rng = true;
            call_arguments.push(quote! { rng });
            continue;
        }
        if let Type::Reference(reference) = &*argument.ty
            && is_std_rng(&reference.elem)
        {
            return error_to_stream(syn::Error::new(
                argument.span(),
                "#[gpui::bench] provides `StdRng` by value; take `rng: StdRng`",
            ));
        }
        call_arguments.push(match (inputs.is_some(), positional_count) {
            (true, 0) => quote! { input },
            (true, 1) | (false, 0) => quote! { &mut cx },
            _ => {
                return error_to_stream(syn::Error::new(
                    argument.span(),
                    format!(
                        "#[gpui::bench] expected {expected_signature}, optionally with a `StdRng`"
                    ),
                ));
            }
        });
        positional_count += 1;
    }
    if positional_count != if inputs.is_some() { 2 } else { 1 } {
        return error_to_stream(syn::Error::new(
            inner_fn.sig.inputs.span(),
            format!("#[gpui::bench] expected {expected_signature}, optionally with a `StdRng`"),
        ));
    }
    if !takes_rng && (explicit_seeds.is_some() || iterations.is_some()) {
        return error_to_stream(syn::Error::new(
            inner_fn.sig.span(),
            "#[gpui::bench] `seed`, `seeds`, and `iterations` require a `StdRng` parameter",
        ));
    }

    // Criterion calls the routine several times (warm-up, then each sample), so the
    // context and the RNG are rebuilt on every call: each one measures the same seed.
    let rng = takes_rng
        .then(|| quote! { let rng = gpui::private::rand::SeedableRng::seed_from_u64(seed); });
    let routine = quote! {
        #rng
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
        #inner_fn_name(#(#call_arguments),*);
        cx.teardown();
    };
    // Each seed is its own benchmark, named after it, so a baseline recorded under
    // `SEED=n` compares against the same tree next time.
    let for_each_seed = |run: proc_macro2::TokenStream| {
        if takes_rng {
            let iterations = iterations.unwrap_or(1);
            let explicit_seeds = explicit_seeds.clone().unwrap_or_default();
            quote! {
                for seed in gpui::calculate_seeds(#iterations, &[#(#explicit_seeds),*]).0 {
                    #run
                }
            }
        } else {
            run
        }
    };

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
        let parameter = if takes_rng {
            quote! { format!("{}/seed-{}", input, seed) }
        } else {
            quote! { input.to_string() }
        };
        let run = for_each_seed(quote! {
            // One report per benchmark: per-iteration metrics differ across
            // inputs and seeds, so blending them would make the summary meaningless.
            let report = #report_expr;
            let parameter = #parameter;
            let report_name = format!("{}/{}/{}", #group_name, #input_name, parameter);
            group.bench_with_input(criterion::BenchmarkId::new(#input_name, &parameter), &input, {
                let report = report.clone();
                move |bencher, input| {
                    #routine
                }
            });
            report.print(&report_name);
        });
        quote! {
            let mut group = criterion.benchmark_group(#group_name);
            #sample_size
            for input in #inputs {
                #run
            }
            group.finish();
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
        let name = if takes_rng {
            quote! { format!("{}/seed-{}", stringify!(#outer_fn_name), seed) }
        } else {
            quote! { stringify!(#outer_fn_name).to_string() }
        };
        for_each_seed(quote! {
            let report = #report_expr;
            let name = #name;
            criterion.bench_function(&name, {
                let report = report.clone();
                move |bencher| {
                    #routine
                }
            });
            report.print(&name);
        })
    };

    TokenStream::from(quote! {
        #inner_fn

        fn #outer_fn_name(criterion: &mut criterion::Criterion<gpui::BenchMeasurement>) {
            #benchmark
        }

    })
}

fn is_std_rng(ty: &Type) -> bool {
    let Type::Path(path) = ty else {
        return false;
    };
    path.path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == "StdRng")
}

fn error_to_stream(error: syn::Error) -> TokenStream {
    TokenStream::from(error.into_compile_error())
}
