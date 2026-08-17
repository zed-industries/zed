use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use syn::parse::{Error, Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{braced, token, Token};
use wit_bindgen_core::wit_parser::{PackageId, Resolve, UnresolvedPackage, WorldId};
use wit_bindgen_rust::{Opts, Ownership};

#[proc_macro]
pub fn generate(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Config)
        .expand()
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

fn anyhow_to_syn(span: Span, err: anyhow::Error) -> Error {
    let mut msg = err.to_string();
    for cause in err.chain().skip(1) {
        msg.push_str(&format!("\n\nCaused by:\n  {cause}"));
    }
    Error::new(span, msg)
}

struct Config {
    opts: Opts,
    resolve: Resolve,
    world: WorldId,
    files: Vec<PathBuf>,
}

/// The source of the wit package definition
enum Source {
    /// A path to a wit directory
    Path(String),
    /// Inline sources have an optional path to a directory of their dependencies
    Inline(String, Option<PathBuf>),
}

impl Parse for Config {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let call_site = Span::call_site();
        let mut opts = Opts::default();
        let mut world = None;
        let mut source = None;

        if input.peek(token::Brace) {
            let content;
            syn::braced!(content in input);
            let fields = Punctuated::<Opt, Token![,]>::parse_terminated(&content)?;
            for field in fields.into_pairs() {
                match field.into_value() {
                    Opt::Path(s) => {
                        source = Some(match source {
                            Some(Source::Path(_)) | Some(Source::Inline(_, Some(_))) => {
                                return Err(Error::new(s.span(), "cannot specify second source"));
                            }
                            Some(Source::Inline(i, None)) => {
                                Source::Inline(i, Some(PathBuf::from(s.value())))
                            }
                            None => Source::Path(s.value()),
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
                            Some(Source::Path(p)) => {
                                Source::Inline(s.value(), Some(PathBuf::from(p)))
                            }
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
                    Opt::With(with) => opts.with.extend(with),
                    Opt::TypeSectionSuffix(suffix) => {
                        opts.type_section_suffix = Some(suffix.value());
                    }
                    Opt::RunCtorsOnceWorkaround(enable) => {
                        opts.run_ctors_once_workaround = enable.value();
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
                }
            }
        } else {
            world = input.parse::<Option<syn::LitStr>>()?.map(|s| s.value());
            if input.parse::<Option<syn::token::In>>()?.is_some() {
                source = Some(Source::Path(input.parse::<syn::LitStr>()?.value()));
            }
        }
        let (resolve, pkg, files) =
            parse_source(&source).map_err(|err| anyhow_to_syn(call_site, err))?;
        let world = resolve
            .select_world(pkg, world.as_deref())
            .map_err(|e| anyhow_to_syn(call_site, e))?;
        Ok(Config {
            opts,
            resolve,
            world,
            files,
        })
    }
}

/// Parse the source
fn parse_source(source: &Option<Source>) -> anyhow::Result<(Resolve, PackageId, Vec<PathBuf>)> {
    let mut resolve = Resolve::default();
    let mut files = Vec::new();
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut parse = |path: &Path| -> anyhow::Result<_> {
        let (pkg, sources) = resolve.push_path(path)?;
        files.extend(sources);
        Ok(pkg)
    };
    let pkg = match source {
        Some(Source::Inline(s, path)) => {
            if let Some(p) = path {
                parse(&root.join(p))?;
            }
            resolve.push(UnresolvedPackage::parse("macro-input".as_ref(), s)?)?
        }
        Some(Source::Path(s)) => parse(&root.join(s))?,
        None => parse(&root.join("wit"))?,
    };

    Ok((resolve, pkg, files))
}

impl Config {
    fn expand(self) -> Result<TokenStream> {
        let mut files = Default::default();
        let mut generator = self.opts.build();
        generator
            .generate(&self.resolve, self.world, &mut files)
            .map_err(|e| Error::new(Span::call_site(), e))?;
        let (_, src) = files.iter().next().unwrap();
        let mut src = std::str::from_utf8(src).unwrap().to_string();

        // If a magical `WIT_BINDGEN_DEBUG` environment variable is set then
        // place a formatted version of the expanded code into a file. This file
        // will then show up in rustc error messages for any codegen issues and can
        // be inspected manually.
        if std::env::var("WIT_BINDGEN_DEBUG").is_ok() {
            static INVOCATION: AtomicUsize = AtomicUsize::new(0);
            let root = Path::new(env!("DEBUG_OUTPUT_DIR"));
            let world_name = &self.resolve.worlds[self.world].name;
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
    syn::custom_keyword!(with);
    syn::custom_keyword!(type_section_suffix);
    syn::custom_keyword!(run_ctors_once_workaround);
    syn::custom_keyword!(default_bindings_module);
    syn::custom_keyword!(export_macro_name);
    syn::custom_keyword!(pub_export_macro);
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

enum Opt {
    World(syn::LitStr),
    Path(syn::LitStr),
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
    With(HashMap<String, String>),
    TypeSectionSuffix(syn::LitStr),
    RunCtorsOnceWorkaround(syn::LitBool),
    DefaultBindingsModule(syn::LitStr),
    ExportMacroName(syn::LitStr),
    PubExportMacro(syn::LitBool),
}

impl Parse for Opt {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::path) {
            input.parse::<kw::path>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::Path(input.parse()?))
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
        } else if l.peek(kw::with) {
            input.parse::<kw::with>()?;
            input.parse::<Token![:]>()?;
            let contents;
            let _lbrace = braced!(contents in input);
            let fields: Punctuated<_, Token![,]> =
                contents.parse_terminated(with_field_parse, Token![,])?;
            Ok(Opt::With(HashMap::from_iter(fields.into_iter())))
        } else if l.peek(kw::type_section_suffix) {
            input.parse::<kw::type_section_suffix>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::TypeSectionSuffix(input.parse()?))
        } else if l.peek(kw::run_ctors_once_workaround) {
            input.parse::<kw::run_ctors_once_workaround>()?;
            input.parse::<Token![:]>()?;
            Ok(Opt::RunCtorsOnceWorkaround(input.parse()?))
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
        } else {
            Err(l.error())
        }
    }
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

    Ok((interface, buf))
}
