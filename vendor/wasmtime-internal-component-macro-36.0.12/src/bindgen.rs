use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Token, braced, token};
use wasmtime_wit_bindgen::{
    FunctionConfig, FunctionFilter, FunctionFlags, Opts, Ownership, TrappableError,
};
use wit_parser::{PackageId, Resolve, UnresolvedPackageGroup, WorldId};

pub struct Config {
    opts: Opts,
    resolve: Resolve,
    world: WorldId,
    files: Vec<PathBuf>,
    include_generated_code_from_file: bool,
}

pub fn expand(input: &Config) -> Result<TokenStream> {
    let mut src = match input.opts.generate(&input.resolve, input.world) {
        Ok(s) => s,
        Err(e) => return Err(Error::new(Span::call_site(), e.to_string())),
    };

    if input.opts.stringify {
        return Ok(quote::quote!(#src));
    }

    // If a magical `WASMTIME_DEBUG_BINDGEN` environment variable is set then
    // place a formatted version of the expanded code into a file. This file
    // will then show up in rustc error messages for any codegen issues and can
    // be inspected manually.
    if input.include_generated_code_from_file
        || input.opts.debug
        || std::env::var("WASMTIME_DEBUG_BINDGEN").is_ok()
    {
        static INVOCATION: AtomicUsize = AtomicUsize::new(0);
        let root = Path::new(env!("DEBUG_OUTPUT_DIR"));
        let world_name = &input.resolve.worlds[input.world].name;
        let n = INVOCATION.fetch_add(1, Relaxed);
        let path = root.join(format!("{world_name}{n}.rs"));

        std::fs::write(&path, &src).unwrap();

        // optimistically format the code but don't require success
        drop(
            std::process::Command::new("rustfmt")
                .arg(&path)
                .arg("--edition=2021")
                .output(),
        );

        src = format!("include!({path:?});");
    }
    let mut contents = src.parse::<TokenStream>().unwrap();

    // Include a dummy `include_str!` for any files we read so rustc knows that
    // we depend on the contents of those files.
    for file in input.files.iter() {
        contents.extend(
            format!("const _: &str = include_str!(r#\"{}\"#);\n", file.display())
                .parse::<TokenStream>()
                .unwrap(),
        );
    }

    Ok(contents)
}

impl Parse for Config {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let call_site = Span::call_site();
        let mut opts = Opts::default();
        let mut world = None;
        let mut inline = None;
        let mut paths = Vec::new();
        let mut imports_configured = false;
        let mut exports_configured = false;
        let mut include_generated_code_from_file = false;

        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let fields = Punctuated::<Opt, Token![,]>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                match field.into_value() {
                    Opt::Path(p) => {
                        paths.extend(p.into_iter().map(|p| p.value()));
                    }
                    Opt::World(s) => {
                        if world.is_some() {
                            return Err(Error::new(s.span(), "cannot specify second world"));
                        }
                        world = Some(s.value());
                    }
                    Opt::Inline(s) => {
                        if inline.is_some() {
                            return Err(Error::new(s.span(), "cannot specify second source"));
                        }
                        inline = Some(s.value());
                    }
                    Opt::Debug(val) => opts.debug = val,
                    Opt::TrappableErrorType(val) => opts.trappable_error_type = val,
                    Opt::Ownership(val) => opts.ownership = val,
                    Opt::Interfaces(s) => {
                        if inline.is_some() {
                            return Err(Error::new(s.span(), "cannot specify a second source"));
                        }
                        inline = Some(format!(
                            "
                                package wasmtime:component-macro-synthesized;

                                world interfaces {{
                                    {}
                                }}
                            ",
                            s.value()
                        ));

                        if world.is_some() {
                            return Err(Error::new(
                                s.span(),
                                "cannot specify a world with `interfaces`",
                            ));
                        }
                        world = Some("wasmtime:component-macro-synthesized/interfaces".to_string());

                        opts.only_interfaces = true;
                    }
                    Opt::With(val) => opts.with.extend(val),
                    Opt::AdditionalDerives(paths) => {
                        opts.additional_derive_attributes = paths
                            .into_iter()
                            .map(|p| p.into_token_stream().to_string())
                            .collect()
                    }
                    Opt::Stringify(val) => opts.stringify = val,
                    Opt::SkipMutForwardingImpls(val) => opts.skip_mut_forwarding_impls = val,
                    Opt::RequireStoreDataSend(val) => opts.require_store_data_send = val,
                    Opt::WasmtimeCrate(f) => {
                        opts.wasmtime_crate = Some(f.into_token_stream().to_string())
                    }
                    Opt::IncludeGeneratedCodeFromFile(i) => include_generated_code_from_file = i,
                    Opt::Imports(config, span) => {
                        if imports_configured {
                            return Err(Error::new(span, "cannot specify imports configuration"));
                        }
                        opts.imports = config;
                        imports_configured = true;
                    }
                    Opt::Exports(config, span) => {
                        if exports_configured {
                            return Err(Error::new(span, "cannot specify exports configuration"));
                        }
                        opts.exports = config;
                        exports_configured = true;
                    }
                }
            }
        } else {
            world = input.parse::<Option<syn::LitStr>>()?.map(|s| s.value());
            if input.parse::<Option<syn::token::In>>()?.is_some() {
                paths.push(input.parse::<syn::LitStr>()?.value());
            }
        }
        let (resolve, pkgs, files) = parse_source(&paths, &inline)
            .map_err(|err| Error::new(call_site, format!("{err:?}")))?;

        let world = select_world(&resolve, &pkgs, world.as_deref())
            .map_err(|e| Error::new(call_site, format!("{e:?}")))?;
        Ok(Config {
            opts,
            resolve,
            world,
            files,
            include_generated_code_from_file,
        })
    }
}

fn parse_source(
    paths: &Vec<String>,
    inline: &Option<String>,
) -> anyhow::Result<(Resolve, Vec<PackageId>, Vec<PathBuf>)> {
    let mut resolve = Resolve::default();
    resolve.all_features = true;
    let mut files = Vec::new();
    let mut pkgs = Vec::new();
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

    let parse = |resolve: &mut Resolve,
                 files: &mut Vec<PathBuf>,
                 pkgs: &mut Vec<PackageId>,
                 paths: &[String]|
     -> anyhow::Result<_> {
        for path in paths {
            let p = root.join(path);
            // Try to normalize the path to make the error message more understandable when
            // the path is not correct. Fallback to the original path if normalization fails
            // (probably return an error somewhere else).
            let normalized_path = match std::fs::canonicalize(&p) {
                Ok(p) => p,
                Err(_) => p.to_path_buf(),
            };
            let (pkg, sources) = resolve.push_path(normalized_path)?;
            pkgs.push(pkg);
            files.extend(sources.paths().map(|p| p.to_owned()));
        }
        Ok(())
    };

    if !paths.is_empty() {
        parse(&mut resolve, &mut files, &mut pkgs, &paths)?;
    }

    if let Some(inline) = inline {
        pkgs.push(resolve.push_group(UnresolvedPackageGroup::parse("macro-input", inline)?)?);
    }

    if pkgs.is_empty() {
        parse(&mut resolve, &mut files, &mut pkgs, &["wit".into()])?;
    }

    Ok((resolve, pkgs, files))
}

fn select_world(
    resolve: &Resolve,
    pkgs: &[PackageId],
    world: Option<&str>,
) -> anyhow::Result<WorldId> {
    if pkgs.len() == 1 {
        resolve.select_world(pkgs[0], world)
    } else {
        assert!(!pkgs.is_empty());
        match world {
            Some(name) => {
                if !name.contains(":") {
                    anyhow::bail!(
                        "with multiple packages a fully qualified \
                         world name must be specified"
                    )
                }

                // This will ignore the package argument due to the fully
                // qualified name being used.
                resolve.select_world(pkgs[0], world)
            }
            None => {
                let worlds = pkgs
                    .iter()
                    .filter_map(|p| resolve.select_world(*p, None).ok())
                    .collect::<Vec<_>>();
                match &worlds[..] {
                    [] => anyhow::bail!("no packages have a world"),
                    [world] => Ok(*world),
                    _ => anyhow::bail!("multiple packages have a world, must specify which to use"),
                }
            }
        }
    }
}

mod kw {
    syn::custom_keyword!(inline);
    syn::custom_keyword!(path);
    syn::custom_keyword!(tracing);
    syn::custom_keyword!(verbose_tracing);
    syn::custom_keyword!(trappable_error_type);
    syn::custom_keyword!(world);
    syn::custom_keyword!(ownership);
    syn::custom_keyword!(interfaces);
    syn::custom_keyword!(with);
    syn::custom_keyword!(except_imports);
    syn::custom_keyword!(only_imports);
    syn::custom_keyword!(additional_derives);
    syn::custom_keyword!(stringify);
    syn::custom_keyword!(skip_mut_forwarding_impls);
    syn::custom_keyword!(require_store_data_send);
    syn::custom_keyword!(wasmtime_crate);
    syn::custom_keyword!(include_generated_code_from_file);
    syn::custom_keyword!(debug);
    syn::custom_keyword!(imports);
    syn::custom_keyword!(exports);
    syn::custom_keyword!(store);
    syn::custom_keyword!(trappable);
    syn::custom_keyword!(ignore_wit);
    syn::custom_keyword!(exact);
}

enum Opt {
    World(syn::LitStr),
    Path(Vec<syn::LitStr>),
    Inline(syn::LitStr),
    TrappableErrorType(Vec<TrappableError>),
    Ownership(Ownership),
    Interfaces(syn::LitStr),
    With(HashMap<String, String>),
    AdditionalDerives(Vec<syn::Path>),
    Stringify(bool),
    SkipMutForwardingImpls(bool),
    RequireStoreDataSend(bool),
    WasmtimeCrate(syn::Path),
    IncludeGeneratedCodeFromFile(bool),
    Debug(bool),
    Imports(FunctionConfig, Span),
    Exports(FunctionConfig, Span),
}

impl Parse for Opt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::debug) {
            input.parse::<kw::debug>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Debug(input.parse::<syn::LitBool>()?.value))
        } else if l.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![:]>()?;

            let mut paths: Vec<syn::LitStr> = vec![];

            let l = input.lookahead1();
            if l.peek(syn::LitStr) {
                paths.push(input.parse()?);
            } else if l.peek(syn::token::Bracket) {
                let contents;
                syn::bracketed!(contents in input);
                let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;

                paths.extend(list);
            } else {
                return Err(l.error());
            };

            Ok(Opt::Path(paths))
        } else if l.peek(kw::inline) {
            input.parse::<kw::inline>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Inline(input.parse()?))
        } else if l.peek(kw::world) {
            input.parse::<kw::world>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::World(input.parse()?))
        } else if l.peek(kw::ownership) {
            input.parse::<kw::ownership>()?;
            input.parse::<Token![:]>()?;
            let ownership = input.parse::<syn::Ident>()?;
            Ok(Opt::Ownership(match ownership.to_string().as_str() {
                "Owning" => Ownership::Owning,
                "Borrowing" => Ownership::Borrowing {
                    duplicate_if_necessary: {
                        let contents;
                        braced!(contents in input);
                        let field = contents.parse::<syn::Ident>()?;
                        match field.to_string().as_str() {
                            "duplicate_if_necessary" => {
                                contents.parse::<Token![:]>()?;
                                contents.parse::<syn::LitBool>()?.value
                            }
                            name => {
                                return Err(Error::new(
                                    field.span(),
                                    format!(
                                        "unrecognized `Ownership::Borrowing` field: `{name}`; \
                                         expected `duplicate_if_necessary`"
                                    ),
                                ));
                            }
                        }
                    },
                },
                name => {
                    return Err(Error::new(
                        ownership.span(),
                        format!(
                            "unrecognized ownership: `{name}`; \
                             expected `Owning` or `Borrowing`"
                        ),
                    ));
                }
            }))
        } else if l.peek(kw::trappable_error_type) {
            input.parse::<kw::trappable_error_type>()?;
            input.parse::<Token![:]>()?;
            let contents;
            let _lbrace = braced!(contents in input);
            let fields: Punctuated<_, Token![,]> =
                contents.parse_terminated(trappable_error_field_parse, Token![,])?;
            Ok(Opt::TrappableErrorType(Vec::from_iter(fields)))
        } else if l.peek(kw::interfaces) {
            input.parse::<kw::interfaces>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Interfaces(input.parse::<syn::LitStr>()?))
        } else if l.peek(kw::with) {
            input.parse::<kw::with>()?;
            input.parse::<Token![:]>()?;
            let contents;
            let _lbrace = braced!(contents in input);
            let fields: Punctuated<(String, String), Token![,]> =
                contents.parse_terminated(with_field_parse, Token![,])?;
            Ok(Opt::With(HashMap::from_iter(fields)))
        } else if l.peek(kw::additional_derives) {
            input.parse::<kw::additional_derives>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(Opt::AdditionalDerives(list.iter().cloned().collect()))
        } else if l.peek(kw::stringify) {
            input.parse::<kw::stringify>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Stringify(input.parse::<syn::LitBool>()?.value))
        } else if l.peek(kw::skip_mut_forwarding_impls) {
            input.parse::<kw::skip_mut_forwarding_impls>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::SkipMutForwardingImpls(
                input.parse::<syn::LitBool>()?.value,
            ))
        } else if l.peek(kw::require_store_data_send) {
            input.parse::<kw::require_store_data_send>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::RequireStoreDataSend(
                input.parse::<syn::LitBool>()?.value,
            ))
        } else if l.peek(kw::wasmtime_crate) {
            input.parse::<kw::wasmtime_crate>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::WasmtimeCrate(input.parse()?))
        } else if l.peek(kw::include_generated_code_from_file) {
            input.parse::<kw::include_generated_code_from_file>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::IncludeGeneratedCodeFromFile(
                input.parse::<syn::LitBool>()?.value,
            ))
        } else if l.peek(kw::imports) {
            let span = input.parse::<kw::imports>()?.span;
            input.parse::<Token![:]>()?;
            Ok(Opt::Imports(parse_function_config(input)?, span))
        } else if l.peek(kw::exports) {
            let span = input.parse::<kw::exports>()?.span;
            input.parse::<Token![:]>()?;
            Ok(Opt::Exports(parse_function_config(input)?, span))
        } else {
            Err(l.error())
        }
    }
}

fn trappable_error_field_parse(input: ParseStream<'_>) -> Result<TrappableError> {
    let wit_path = input.parse::<syn::LitStr>()?.value();
    input.parse::<Token![=>]>()?;
    let rust_type_name = input.parse::<syn::Path>()?.to_token_stream().to_string();
    Ok(TrappableError {
        wit_path,
        rust_type_name,
    })
}

fn with_field_parse(input: ParseStream<'_>) -> Result<(String, String)> {
    let interface = input.parse::<syn::LitStr>()?.value();
    input.parse::<Token![:]>()?;
    let start = input.span();
    let path = input.parse::<syn::Path>()?;

    // It's not possible for the segments of a path to be empty
    let span = start
        .join(path.segments.last().unwrap().ident.span())
        .unwrap_or(start);

    let mut buf = String::new();
    let append = |buf: &mut String, segment: syn::PathSegment| -> Result<()> {
        if segment.arguments != syn::PathArguments::None {
            return Err(Error::new(
                span,
                "Module path must not contain angles or parens",
            ));
        }

        buf.push_str(&segment.ident.to_string());

        Ok(())
    };

    if path.leading_colon.is_some() {
        buf.push_str("::");
    }

    let mut segments = path.segments.into_iter();

    if let Some(segment) = segments.next() {
        append(&mut buf, segment)?;
    }

    for segment in segments {
        buf.push_str("::");
        append(&mut buf, segment)?;
    }

    Ok((interface, buf))
}

fn parse_function_config(input: ParseStream<'_>) -> Result<FunctionConfig> {
    let content;
    syn::braced!(content in input);
    let mut ret = FunctionConfig::new();

    let list = Punctuated::<FunctionConfigSyntax, Token![,]>::parse_terminated(&content)?;
    for item in list.into_iter() {
        ret.push(item.filter, item.flags);
    }

    return Ok(ret);

    struct FunctionConfigSyntax {
        filter: FunctionFilter,
        flags: FunctionFlags,
    }

    impl Parse for FunctionConfigSyntax {
        fn parse(input: ParseStream<'_>) -> Result<Self> {
            let l = input.lookahead1();
            let filter = if l.peek(syn::LitStr) {
                FunctionFilter::Name(input.parse::<syn::LitStr>()?.value())
            } else if l.peek(Token![default]) {
                input.parse::<Token![default]>()?;
                FunctionFilter::Default
            } else {
                return Err(l.error());
            };

            input.parse::<Token![:]>()?;

            let mut flags = FunctionFlags::empty();
            while !input.is_empty() {
                let l = input.lookahead1();
                if l.peek(Token![async]) {
                    input.parse::<Token![async]>()?;
                    flags |= FunctionFlags::ASYNC;
                } else if l.peek(kw::tracing) {
                    input.parse::<kw::tracing>()?;
                    flags |= FunctionFlags::TRACING;
                } else if l.peek(kw::verbose_tracing) {
                    input.parse::<kw::verbose_tracing>()?;
                    flags |= FunctionFlags::VERBOSE_TRACING;
                } else if l.peek(kw::store) {
                    input.parse::<kw::store>()?;
                    flags |= FunctionFlags::STORE;
                } else if l.peek(kw::trappable) {
                    input.parse::<kw::trappable>()?;
                    flags |= FunctionFlags::TRAPPABLE;
                } else if l.peek(kw::ignore_wit) {
                    input.parse::<kw::ignore_wit>()?;
                    flags |= FunctionFlags::IGNORE_WIT;
                } else if l.peek(kw::exact) {
                    input.parse::<kw::exact>()?;
                    flags |= FunctionFlags::EXACT;
                } else {
                    return Err(l.error());
                }

                if input.peek(Token![|]) {
                    input.parse::<Token![|]>()?;
                } else {
                    break;
                }
            }

            Ok(FunctionConfigSyntax { filter, flags })
        }
    }
}
