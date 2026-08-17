use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{braced, token, LitStr, Token};
use wit_bindgen_core::wit_parser::{PackageId, Resolve, UnresolvedPackageGroup, WorldId};
use wit_bindgen_rust::{AsyncConfig, Opts, Ownership, WithOption};

#[proc_macro]
pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Config)
        .expand()
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn anyhow_to_syn(span: Span, err: anyhow::Error) -> Error {
    let err = attach_with_context(err);
    let mut msg = err.to_string();
    for cause in err.chain().skip(1) {
        msg.push_str(&format!("\n\nCaused by:\n  {cause}"));
    }
    Error::new(span, msg)
}

fn attach_with_context(err: anyhow::Error) -> anyhow::Error {
    if let Some(e) = err.downcast_ref::<wit_bindgen_rust::MissingWith>() {
        let option = e.0.clone();
        return err.context(format!(
            "missing one of:\n\
            * `generate_all` option\n\
            * `with: {{ \"{option}\": path::to::bindings, }}`\n\
            * `with: {{ \"{option}\": generate, }}`\
            "
        ));
    }
    err
}

struct Config {
    opts: Opts,
    resolve: Resolve,
    world: WorldId,
    files: Vec<PathBuf>,
    debug: bool,
}

/// The source of the wit package definition
enum Source {
    /// A path to a wit directory
    Paths(Vec<PathBuf>),
    /// Inline sources have an optional path to a directory of their dependencies
    Inline(String, Option<Vec<PathBuf>>),
}

impl Parse for Config {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let call_site = Span::call_site();
        let mut opts = Opts::default();
        let mut world = None;
        let mut source = None;
        let mut features = Vec::new();
        let mut async_configured = false;
        let mut debug = false;

        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let fields = Punctuated::<Opt, Token![,]>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                match field.into_value() {
                    Opt::Path(span, p) => {
                        let paths = p.into_iter().map(|f| PathBuf::from(f.value())).collect();

                        source = Some(match source {
                            Some(Source::Paths(_)) | Some(Source::Inline(_, Some(_))) => {
                                return Err(Error::new(span, "cannot specify second source"));
                            }
                            Some(Source::Inline(i, None)) => Source::Inline(i, Some(paths)),
                            None => Source::Paths(paths),
                        })
                    }
                    Opt::World(s) => {
                        if world.is_some() {
                            return Err(Error::new(s.span(), "cannot specify second world"));
                        }
                        world = Some(s.value());
                    }
                    Opt::Inline(s) => {
                        source = Some(match source {
                            Some(Source::Inline(_, _)) => {
                                return Err(Error::new(s.span(), "cannot specify second source"));
                            }
                            Some(Source::Paths(p)) => Source::Inline(s.value(), Some(p)),
                            None => Source::Inline(s.value(), None),
                        })
                    }
                    Opt::UseStdFeature => opts.std_feature = true,
                    Opt::RawStrings => opts.raw_strings = true,
                    Opt::Ownership(ownership) => opts.ownership = ownership,
                    Opt::Skip(list) => opts.skip.extend(list.iter().map(|i| i.value())),
                    Opt::RuntimePath(path) => opts.runtime_path = Some(path.value()),
                    Opt::BitflagsPath(path) => opts.bitflags_path = Some(path.value()),
                    Opt::Stubs => {
                        opts.stubs = true;
                    }
                    Opt::ExportPrefix(prefix) => opts.export_prefix = Some(prefix.value()),
                    Opt::AdditionalDerives(paths) => {
                        opts.additional_derive_attributes = paths
                            .into_iter()
                            .map(|p| p.into_token_stream().to_string())
                            .collect()
                    }
                    Opt::AdditionalDerivesIgnore(list) => {
                        opts.additional_derive_ignore =
                            list.into_iter().map(|i| i.value()).collect()
                    }
                    Opt::With(with) => opts.with.extend(with),
                    Opt::GenerateAll => {
                        opts.generate_all = true;
                    }
                    Opt::TypeSectionSuffix(suffix) => {
                        opts.type_section_suffix = Some(suffix.value());
                    }
                    Opt::DisableRunCtorsOnceWorkaround(enable) => {
                        opts.disable_run_ctors_once_workaround = enable.value();
                    }
                    Opt::DefaultBindingsModule(enable) => {
                        opts.default_bindings_module = Some(enable.value());
                    }
                    Opt::ExportMacroName(name) => {
                        opts.export_macro_name = Some(name.value());
                    }
                    Opt::PubExportMacro(enable) => {
                        opts.pub_export_macro = enable.value();
                    }
                    Opt::GenerateUnusedTypes(enable) => {
                        opts.generate_unused_types = enable.value();
                    }
                    Opt::Features(f) => {
                        features.extend(f.into_iter().map(|f| f.value()));
                    }
                    Opt::DisableCustomSectionLinkHelpers(disable) => {
                        opts.disable_custom_section_link_helpers = disable.value();
                    }
                    Opt::Debug(enable) => {
                        debug = enable.value();
                    }
                    Opt::Async(val, span) => {
                        if async_configured {
                            return Err(Error::new(span, "cannot specify second async config"));
                        }
                        async_configured = true;
                        if !matches!(val, AsyncConfig::None) && !cfg!(feature = "async") {
                            return Err(Error::new(
                                span,
                                "must enable `async` feature to enable async imports and/or exports",
                            ));
                        }
                        opts.async_ = val;
                    }
                }
            }
        } else {
            world = input.parse::<Option<syn::LitStr>>()?.map(|s| s.value());
            if input.parse::<Option<syn::token::In>>()?.is_some() {
                source = Some(Source::Paths(vec![PathBuf::from(
                    input.parse::<syn::LitStr>()?.value(),
                )]));
            }
        }
        let (resolve, pkgs, files) =
            parse_source(&source, &features).map_err(|err| anyhow_to_syn(call_site, err))?;
        let world = select_world(&resolve, &pkgs, world.as_deref())
            .map_err(|e| anyhow_to_syn(call_site, e))?;
        Ok(Config {
            opts,
            resolve,
            world,
            files,
            debug,
        })
    }
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

/// Parse the source
fn parse_source(
    source: &Option<Source>,
    features: &[String],
) -> anyhow::Result<(Resolve, Vec<PackageId>, Vec<PathBuf>)> {
    let mut resolve = Resolve::default();
    resolve.features.extend(features.iter().cloned());
    let mut files = Vec::new();
    let mut pkgs = Vec::new();
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut parse = |paths: &[PathBuf]| -> anyhow::Result<()> {
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
    match source {
        Some(Source::Inline(s, path)) => {
            if let Some(p) = path {
                parse(p)?;
            }
            pkgs.push(resolve.push_group(UnresolvedPackageGroup::parse("macro-input", s)?)?);
        }
        Some(Source::Paths(p)) => parse(p)?,
        None => parse(&vec![root.join("wit")])?,
    };

    Ok((resolve, pkgs, files))
}

impl Config {
    fn expand(self) -> Result<TokenStream> {
        let mut files = Default::default();
        let mut generator = self.opts.build();
        generator
            .generate(&self.resolve, self.world, &mut files)
            .map_err(|e| anyhow_to_syn(Span::call_site(), e))?;
        let (_, src) = files.iter().next().unwrap();
        let mut src = std::str::from_utf8(src).unwrap().to_string();

        // If a magical `WIT_BINDGEN_DEBUG` environment variable is set then
        // place a formatted version of the expanded code into a file. This file
        // will then show up in rustc error messages for any codegen issues and can
        // be inspected manually.
        if std::env::var("WIT_BINDGEN_DEBUG").is_ok() || self.debug {
            static INVOCATION: AtomicUsize = AtomicUsize::new(0);
            let root = Path::new(env!("DEBUG_OUTPUT_DIR"));
            let world_name = &self.resolve.worlds[self.world].name;
            let n = INVOCATION.fetch_add(1, Relaxed);
            let path = root.join(format!("{world_name}{n}.rs"));

            // optimistically format the code but don't require success
            let contents = match fmt(&src) {
                Ok(formatted) => formatted,
                Err(_) => src.clone(),
            };
            std::fs::write(&path, contents.as_bytes()).unwrap();

            src = format!("include!({path:?});");
        }
        let mut contents = src.parse::<TokenStream>().unwrap();

        // Include a dummy `include_bytes!` for any files we read so rustc knows that
        // we depend on the contents of those files.
        for file in self.files.iter() {
            contents.extend(
                format!(
                    "const _: &[u8] = include_bytes!(r#\"{}\"#);\n",
                    file.display()
                )
                .parse::<TokenStream>()
                .unwrap(),
            );
        }

        Ok(contents)
    }
}

mod kw {
    syn::custom_keyword!(std_feature);
    syn::custom_keyword!(raw_strings);
    syn::custom_keyword!(skip);
    syn::custom_keyword!(world);
    syn::custom_keyword!(path);
    syn::custom_keyword!(inline);
    syn::custom_keyword!(ownership);
    syn::custom_keyword!(runtime_path);
    syn::custom_keyword!(bitflags_path);
    syn::custom_keyword!(exports);
    syn::custom_keyword!(stubs);
    syn::custom_keyword!(export_prefix);
    syn::custom_keyword!(additional_derives);
    syn::custom_keyword!(additional_derives_ignore);
    syn::custom_keyword!(with);
    syn::custom_keyword!(generate_all);
    syn::custom_keyword!(type_section_suffix);
    syn::custom_keyword!(disable_run_ctors_once_workaround);
    syn::custom_keyword!(default_bindings_module);
    syn::custom_keyword!(export_macro_name);
    syn::custom_keyword!(pub_export_macro);
    syn::custom_keyword!(generate_unused_types);
    syn::custom_keyword!(features);
    syn::custom_keyword!(disable_custom_section_link_helpers);
    syn::custom_keyword!(imports);
    syn::custom_keyword!(debug);
}

#[derive(Clone)]
enum ExportKey {
    World,
    Name(syn::LitStr),
}

impl Parse for ExportKey {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        Ok(if l.peek(kw::world) {
            input.parse::<kw::world>()?;
            Self::World
        } else {
            Self::Name(input.parse()?)
        })
    }
}

impl From<ExportKey> for wit_bindgen_rust::ExportKey {
    fn from(key: ExportKey) -> Self {
        match key {
            ExportKey::World => Self::World,
            ExportKey::Name(s) => Self::Name(s.value()),
        }
    }
}

enum AsyncConfigSomeKind {
    Imports,
    Exports,
}

enum Opt {
    World(syn::LitStr),
    Path(Span, Vec<syn::LitStr>),
    Inline(syn::LitStr),
    UseStdFeature,
    RawStrings,
    Skip(Vec<syn::LitStr>),
    Ownership(Ownership),
    RuntimePath(syn::LitStr),
    BitflagsPath(syn::LitStr),
    Stubs,
    ExportPrefix(syn::LitStr),
    // Parse as paths so we can take the concrete types/macro names rather than raw strings
    AdditionalDerives(Vec<syn::Path>),
    AdditionalDerivesIgnore(Vec<syn::LitStr>),
    With(HashMap<String, WithOption>),
    GenerateAll,
    TypeSectionSuffix(syn::LitStr),
    DisableRunCtorsOnceWorkaround(syn::LitBool),
    DefaultBindingsModule(syn::LitStr),
    ExportMacroName(syn::LitStr),
    PubExportMacro(syn::LitBool),
    GenerateUnusedTypes(syn::LitBool),
    Features(Vec<syn::LitStr>),
    DisableCustomSectionLinkHelpers(syn::LitBool),
    Async(AsyncConfig, Span),
    Debug(syn::LitBool),
}

impl Parse for Opt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![:]>()?;
            // the `path` supports two forms:
            // * path: "xxx"
            // * path: ["aaa", "bbb"]
            if input.peek(token::Bracket) {
                let contents;
                syn::bracketed!(contents in input);
                let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
                Ok(Opt::Path(list.span(), list.into_iter().collect()))
            } else {
                let path: LitStr = input.parse()?;
                Ok(Opt::Path(path.span(), vec![path]))
            }
        } else if l.peek(kw::inline) {
            input.parse::<kw::inline>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Inline(input.parse()?))
        } else if l.peek(kw::world) {
            input.parse::<kw::world>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::World(input.parse()?))
        } else if l.peek(kw::std_feature) {
            input.parse::<kw::std_feature>()?;
            Ok(Opt::UseStdFeature)
        } else if l.peek(kw::raw_strings) {
            input.parse::<kw::raw_strings>()?;
            Ok(Opt::RawStrings)
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
        } else if l.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(Opt::Skip(list.iter().cloned().collect()))
        } else if l.peek(kw::runtime_path) {
            input.parse::<kw::runtime_path>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::RuntimePath(input.parse()?))
        } else if l.peek(kw::bitflags_path) {
            input.parse::<kw::bitflags_path>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::BitflagsPath(input.parse()?))
        } else if l.peek(kw::stubs) {
            input.parse::<kw::stubs>()?;
            Ok(Opt::Stubs)
        } else if l.peek(kw::export_prefix) {
            input.parse::<kw::export_prefix>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::ExportPrefix(input.parse()?))
        } else if l.peek(kw::additional_derives) {
            input.parse::<kw::additional_derives>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(Opt::AdditionalDerives(list.iter().cloned().collect()))
        } else if l.peek(kw::additional_derives_ignore) {
            input.parse::<kw::additional_derives_ignore>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(Opt::AdditionalDerivesIgnore(list.iter().cloned().collect()))
        } else if l.peek(kw::with) {
            input.parse::<kw::with>()?;
            input.parse::<Token![:]>()?;
            let contents;
            let _lbrace = braced!(contents in input);
            let fields: Punctuated<_, Token![,]> =
                contents.parse_terminated(with_field_parse, Token![,])?;
            Ok(Opt::With(HashMap::from_iter(fields.into_iter())))
        } else if l.peek(kw::generate_all) {
            input.parse::<kw::generate_all>()?;
            Ok(Opt::GenerateAll)
        } else if l.peek(kw::type_section_suffix) {
            input.parse::<kw::type_section_suffix>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::TypeSectionSuffix(input.parse()?))
        } else if l.peek(kw::disable_run_ctors_once_workaround) {
            input.parse::<kw::disable_run_ctors_once_workaround>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::DisableRunCtorsOnceWorkaround(input.parse()?))
        } else if l.peek(kw::default_bindings_module) {
            input.parse::<kw::default_bindings_module>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::DefaultBindingsModule(input.parse()?))
        } else if l.peek(kw::export_macro_name) {
            input.parse::<kw::export_macro_name>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::ExportMacroName(input.parse()?))
        } else if l.peek(kw::pub_export_macro) {
            input.parse::<kw::pub_export_macro>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::PubExportMacro(input.parse()?))
        } else if l.peek(kw::generate_unused_types) {
            input.parse::<kw::generate_unused_types>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::GenerateUnusedTypes(input.parse()?))
        } else if l.peek(kw::features) {
            input.parse::<kw::features>()?;
            input.parse::<Token![:]>()?;
            let contents;
            syn::bracketed!(contents in input);
            let list = Punctuated::<_, Token![,]>::parse_terminated(&contents)?;
            Ok(Opt::Features(list.into_iter().collect()))
        } else if l.peek(kw::disable_custom_section_link_helpers) {
            input.parse::<kw::disable_custom_section_link_helpers>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::DisableCustomSectionLinkHelpers(input.parse()?))
        } else if l.peek(kw::debug) {
            input.parse::<kw::debug>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Debug(input.parse()?))
        } else if l.peek(Token![async]) {
            let span = input.parse::<Token![async]>()?.span;
            input.parse::<Token![:]>()?;
            if input.peek(syn::LitBool) {
                if input.parse::<syn::LitBool>()?.value {
                    Ok(Opt::Async(AsyncConfig::All, span))
                } else {
                    Ok(Opt::Async(AsyncConfig::None, span))
                }
            } else {
                let mut imports = Vec::new();
                let mut exports = Vec::new();
                let contents;
                syn::braced!(contents in input);
                for (kind, values) in
                    contents.parse_terminated(parse_async_some_field, Token![,])?
                {
                    match kind {
                        AsyncConfigSomeKind::Imports => imports = values,
                        AsyncConfigSomeKind::Exports => exports = values,
                    }
                }
                Ok(Opt::Async(AsyncConfig::Some { imports, exports }, span))
            }
        } else {
            Err(l.error())
        }
    }
}

fn with_field_parse(input: ParseStream<'_>) -> Result<(String, WithOption)> {
    let interface = input.parse::<syn::LitStr>()?.value();
    input.parse::<Token![:]>()?;
    let start = input.span();
    let path = input.parse::<syn::Path>()?;

    // It's not possible for the segments of a path to be empty
    let span = start
        .join(path.segments.last().unwrap().ident.span())
        .unwrap_or(start);

    if path.is_ident("generate") {
        return Ok((interface, WithOption::Generate));
    }

    let mut buf = String::new();
    let append = |buf: &mut String, segment: syn::PathSegment| -> Result<()> {
        if !segment.arguments.is_none() {
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

    Ok((interface, WithOption::Path(buf)))
}

/// Format a valid Rust string
fn fmt(input: &str) -> Result<String> {
    let syntax_tree = syn::parse_file(&input)?;
    Ok(prettyplease::unparse(&syntax_tree))
}

fn parse_async_some_field(input: ParseStream<'_>) -> Result<(AsyncConfigSomeKind, Vec<String>)> {
    let lookahead = input.lookahead1();
    let kind = if lookahead.peek(kw::imports) {
        input.parse::<kw::imports>()?;
        input.parse::<Token![:]>()?;
        AsyncConfigSomeKind::Imports
    } else if lookahead.peek(kw::exports) {
        input.parse::<kw::exports>()?;
        input.parse::<Token![:]>()?;
        AsyncConfigSomeKind::Exports
    } else {
        return Err(lookahead.error());
    };

    let list;
    syn::bracketed!(list in input);
    let fields = list.parse_terminated(Parse::parse, Token![,])?;

    Ok((
        kind,
        fields.iter().map(|s: &syn::LitStr| s.value()).collect(),
    ))
}
