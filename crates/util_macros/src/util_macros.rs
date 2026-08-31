#![cfg_attr(not(target_os = "windows"), allow(unused))]
#![allow(clippy::test_attr_in_doctest)]

use perf::*;
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::{ItemFn, LitStr, parse_macro_input, parse_quote};

/// A macro used in tests for cross-platform path string literals in tests. On Windows it replaces
/// `/` with `\\` and adds `C:` to the beginning of absolute paths. On other platforms, the path is
/// returned unmodified.
///
/// # Example
/// ```rust
/// use util_macros::path;
///
/// let path = path!("/Users/user/file.txt");
/// #[cfg(target_os = "windows")]
/// assert_eq!(path, "C:\\Users\\user\\file.txt");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(path, "/Users/user/file.txt");
/// ```
#[proc_macro]
pub fn path(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);
    let mut path = path.value();

    #[cfg(target_os = "windows")]
    {
        path = path.replace("/", "\\");
        if path.starts_with("\\") {
            path = format!("C:{}", path);
        }
    }

    TokenStream::from(quote! {
        #path
    })
}

/// This macro replaces the path prefix `file:///` with `file:///C:/` for Windows.
/// But if the target OS is not Windows, the URI is returned as is.
///
/// # Example
/// ```rust
/// use util_macros::uri;
///
/// let uri = uri!("file:///path/to/file");
/// #[cfg(target_os = "windows")]
/// assert_eq!(uri, "file:///C:/path/to/file");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(uri, "file:///path/to/file");
/// ```
#[proc_macro]
pub fn uri(input: TokenStream) -> TokenStream {
    let uri = parse_macro_input!(input as LitStr);
    let uri = uri.value();

    #[cfg(target_os = "windows")]
    let uri = uri.replace("file:///", "file:///C:/");

    TokenStream::from(quote! {
        #uri
    })
}

/// This macro replaces the line endings `\n` with `\r\n` for Windows.
/// But if the target OS is not Windows, the line endings are returned as is.
///
/// # Example
/// ```rust
/// use util_macros::line_endings;
///
/// let text = line_endings!("Hello\nWorld");
/// #[cfg(target_os = "windows")]
/// assert_eq!(text, "Hello\r\nWorld");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(text, "Hello\nWorld");
/// ```
#[proc_macro]
pub fn line_endings(input: TokenStream) -> TokenStream {
    let text = parse_macro_input!(input as LitStr);
    let text = text.value();

    #[cfg(target_os = "windows")]
    let text = text.replace("\n", "\r\n");

    TokenStream::from(quote! {
        #text
    })
}

/// Inner data for the perf macro.
#[derive(Default)]
struct PerfArgs {
    /// How many times to loop a test before rerunning the test binary. If left
    /// empty, the test harness will auto-determine this value.
    iterations: Option<syn::Expr>,
    /// How much this test's results should be weighed when comparing across runs.
    /// If unspecified, defaults to `WEIGHT_DEFAULT` (50).
    weight: Option<syn::Expr>,
    /// How relevant a benchmark is to overall performance. See docs on the enum
    /// for details. If unspecified, `Average` is selected.
    importance: Importance,
}

#[warn(clippy::all, clippy::pedantic)]
impl PerfArgs {
    /// Parses attribute arguments into a `PerfArgs`.
    fn parse_into(&mut self, meta: syn::meta::ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("iterations") {
            self.iterations = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("weight") {
            self.weight = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("critical") {
            self.importance = Importance::Critical;
        } else if meta.path.is_ident("important") {
            self.importance = Importance::Important;
        } else if meta.path.is_ident("average") {
            // This shouldn't be specified manually, but oh well.
            self.importance = Importance::Average;
        } else if meta.path.is_ident("iffy") {
            self.importance = Importance::Iffy;
        } else if meta.path.is_ident("fluff") {
            self.importance = Importance::Fluff;
        } else {
            return Err(syn::Error::new_spanned(meta.path, "unexpected identifier"));
        }
        Ok(())
    }
}

/// Marks a test as perf-sensitive, to be triaged when checking the performance
/// of a build. This also automatically applies `#[test]`.
///
/// # Usage
/// Applying this attribute to a test marks it as average importance by default.
/// There are 5 levels of importance (`Critical`, `Important`, `Average`, `Iffy`,
/// `Fluff`); see the documentation on `Importance` for details. Add the importance
/// as a parameter to override the default (e.g. `#[perf(important)]`).
///
/// Each test also has a weight factor. This is irrelevant on its own, but is considered
/// when comparing results across different runs. By default, this is set to 50;
/// pass `weight = n` as a parameter to override this. Note that this value is only
/// relevant within its importance category.
///
/// By default, the number of iterations when profiling this test is auto-determined.
/// If this needs to be overwritten, pass the desired iteration count as a parameter
/// (`#[perf(iterations = n)]`). Note that the actual profiler may still run the test
/// an arbitrary number times; this flag just sets the number of executions before the
/// process is restarted and global state is reset.
///
/// This attribute should probably not be applied to tests that do any significant
/// disk IO, as locks on files may not be released in time when repeating a test many
/// times. This might lead to spurious failures.
///
/// # Examples
/// ```rust
/// use util_macros::perf;
///
/// #[perf]
/// fn generic_test() {
///     // Test goes here.
/// }
///
/// #[perf(fluff, weight = 30)]
/// fn cold_path_test() {
///     // Test goes here.
/// }
/// ```
///
/// This also works with `#[gpui::test]`s, though in most cases it shouldn't
/// be used with automatic iterations.
/// ```rust,ignore
/// use util_macros::perf;
///
/// #[perf(iterations = 1, critical)]
/// #[gpui::test]
/// fn oneshot_test(_cx: &mut gpui::TestAppContext) {
///     // Test goes here.
/// }
/// ```
#[proc_macro_attribute]
#[warn(clippy::all, clippy::pedantic)]
pub fn perf(our_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut args = PerfArgs::default();
    let parser = syn::meta::parser(|meta| PerfArgs::parse_into(&mut args, meta));
    parse_macro_input!(our_attr with parser);

    let ItemFn {
        attrs: mut attrs_main,
        vis,
        sig: mut sig_main,
        block,
    } = parse_macro_input!(input as ItemFn);
    if !attrs_main
        .iter()
        .any(|a| Some(&parse_quote!(test)) == a.path().segments.last())
    {
        attrs_main.push(parse_quote!(#[test]));
    }
    attrs_main.push(parse_quote!(#[allow(non_snake_case)]));

    let fns = if cfg!(perf_enabled) {
        #[allow(clippy::wildcard_imports, reason = "We control the other side")]
        use consts::*;

        // Make the ident obvious when calling, for the test parser.
        // Also set up values for the second metadata-returning "test".
        let mut new_ident_main = sig_main.ident.to_string();
        let mut new_ident_meta = new_ident_main.clone();
        new_ident_main.push_str(SUF_NORMAL);
        new_ident_meta.push_str(SUF_MDATA);

        let new_ident_main = syn::Ident::new(&new_ident_main, sig_main.ident.span());
        sig_main.ident = new_ident_main;

        // We don't want any nonsense if the original test had a weird signature.
        let new_ident_meta = syn::Ident::new(&new_ident_meta, sig_main.ident.span());
        let sig_meta = parse_quote!(fn #new_ident_meta());
        let attrs_meta = parse_quote!(#[test] #[allow(non_snake_case)]);

        // Make the test loop as the harness instructs it to.
        let block_main = {
            // The perf harness will pass us the value in an env var. Even if we
            // have a preset value, just do this to keep the code paths unified.
            parse_quote!({
                let iter_count = std::env::var(#ITER_ENV_VAR).unwrap().parse::<usize>().unwrap();
                for _ in 0..iter_count {
                    #block
                }
            })
        };
        let importance = format!("{}", args.importance);
        let block_meta = {
            // This function's job is to just print some relevant info to stdout,
            // based on the params this attr is passed. It's not an actual test.
            // Since we use a custom attr set on our metadata fn, it shouldn't
            // cause problems with xfail tests.
            let q_iter = if let Some(iter) = args.iterations {
                quote! {
                    println!("{} {} {}", #MDATA_LINE_PREF, #ITER_COUNT_LINE_NAME, #iter);
                }
            } else {
                quote! {}
            };
            let weight = args
                .weight
                .unwrap_or_else(|| parse_quote! { #WEIGHT_DEFAULT });
            parse_quote!({
                #q_iter
                println!("{} {} {}", #MDATA_LINE_PREF, #WEIGHT_LINE_NAME, #weight);
                println!("{} {} {}", #MDATA_LINE_PREF, #IMPORTANCE_LINE_NAME, #importance);
                println!("{} {} {}", #MDATA_LINE_PREF, #VERSION_LINE_NAME, #MDATA_VER);
            })
        };

        vec![
            // The real test.
            ItemFn {
                attrs: attrs_main,
                vis: vis.clone(),
                sig: sig_main,
                block: block_main,
            },
            // The fake test.
            ItemFn {
                attrs: attrs_meta,
                vis,
                sig: sig_meta,
                block: block_meta,
            },
        ]
    } else {
        vec![ItemFn {
            attrs: attrs_main,
            vis,
            sig: sig_main,
            block,
        }]
    };

    fns.into_iter()
        .flat_map(|f| TokenStream::from(f.into_token_stream()))
        .collect()
}

/// Parsed input for [`fs_embed`]: `<vis> struct <Name>, "<repo-relative dir>"`
/// followed by optional `include = [..]` and `exclude = [..]` glob lists.
struct FsEmbedInput {
    vis: syn::Visibility,
    name: syn::Ident,
    dir: LitStr,
    includes: Vec<LitStr>,
    excludes: Vec<LitStr>,
}

impl syn::parse::Parse for FsEmbedInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let vis: syn::Visibility = input.parse()?;
        input.parse::<syn::Token![struct]>()?;
        let name: syn::Ident = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let dir: LitStr = input.parse()?;
        let mut includes = Vec::new();
        let mut excludes = Vec::new();
        while input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?;
            if input.is_empty() {
                break;
            }
            let key: syn::Ident = input.parse()?;
            input.parse::<syn::Token![=]>()?;
            let content;
            syn::bracketed!(content in input);
            let globs =
                syn::punctuated::Punctuated::<LitStr, syn::Token![,]>::parse_terminated(&content)?;
            let globs = globs.into_iter().collect();
            if key == "include" {
                includes = globs;
            } else if key == "exclude" {
                excludes = globs;
            } else {
                return Err(syn::Error::new(
                    key.span(),
                    "unexpected key; expected `include` or `exclude`",
                ));
            }
        }
        Ok(Self {
            vis,
            name,
            dir,
            includes,
            excludes,
        })
    }
}

/// Implementation of `util::fs_embed!` (re-exported from `util`; see its docs for
/// the contract). Defines a `rust_embed` asset source that embeds files in
/// release builds and reads them from the checkout at runtime in dev builds,
/// from a single repository-root-relative directory.
///
/// rust_embed's `#[derive(RustEmbed)]` needs a `#[folder]` relative to the
/// crate's `Cargo.toml`, which `macro_rules!` cannot synthesize from a
/// repo-relative path (attribute values must be string literals). This
/// proc-macro derives it: it climbs from `CARGO_MANIFEST_DIR` up to the
/// repository root (the parent of `crates/`) and descends via the given path.
/// Only a climb *count* is read from the manifest dir, so no absolute build
/// path is baked into the output.
#[proc_macro]
pub fn fs_embed(input: TokenStream) -> TokenStream {
    let FsEmbedInput {
        vis,
        name,
        dir,
        includes,
        excludes,
    } = parse_macro_input!(input as FsEmbedInput);

    // Every asset crate lives at `crates/<name>` (climb 2); deriving the count
    // from the manifest dir generalizes to any depth under `crates/`, and falls
    // back to 2 when the env is unavailable (e.g. a sandboxed proc-macro host).
    let climb = std::env::var("CARGO_MANIFEST_DIR")
        .ok()
        .and_then(|manifest_dir| {
            let manifest_dir = manifest_dir.replace('\\', "/");
            manifest_dir
                .rsplit_once("/crates/")
                .map(|(_root, rest)| 1 + rest.split('/').count())
        })
        .unwrap_or(2);
    let folder = format!("{}{}", "../".repeat(climb), dir.value());
    let folder = LitStr::new(&folder, dir.span());

    let include_attrs: Vec<_> = includes
        .iter()
        .map(|glob| quote! { #[include = #glob] })
        .collect();
    let exclude_attrs: Vec<_> = excludes
        .iter()
        .map(|glob| quote! { #[exclude = #glob] })
        .collect();
    let includes_arr = quote! { &[#(#includes),*] };
    let excludes_arr = quote! { &[#(#excludes),*] };

    quote! {
        // `crate_path` points the derive's generated code at util's rust_embed
        // re-export, so the caller needs no direct `rust_embed` dependency.
        #[cfg(not(debug_assertions))]
        #[derive(::util::__rust_embed::RustEmbed)]
        #[crate_path = "::util::__rust_embed"]
        #[folder = #folder]
        #(#include_attrs)*
        #(#exclude_attrs)*
        #vis struct #name;

        #[cfg(debug_assertions)]
        #vis struct #name;

        // Mirror the derive's public surface: inherent `get`/`iter` (callable
        // without the trait in scope) plus the trait impl (for generic bounds
        // like `util::asset_str` and `handlebars::register_embed_templates`), so
        // the two arms are interchangeable at call sites.
        #[cfg(debug_assertions)]
        impl #name {
            pub fn get(
                file_path: &str,
            ) -> ::core::option::Option<::util::__rust_embed::EmbeddedFile> {
                ::util::__fs_embed_get(#dir, file_path, #includes_arr, #excludes_arr)
            }

            pub fn iter(
            ) -> impl ::core::iter::Iterator<Item = ::std::borrow::Cow<'static, str>> + 'static
            {
                ::util::__fs_embed_iter(#dir, #includes_arr, #excludes_arr)
            }
        }

        #[cfg(debug_assertions)]
        impl ::util::__rust_embed::RustEmbed for #name {
            fn get(
                file_path: &str,
            ) -> ::core::option::Option<::util::__rust_embed::EmbeddedFile> {
                <#name>::get(file_path)
            }

            fn iter() -> ::util::__rust_embed::Filenames {
                ::util::__fs_embed_iter(#dir, #includes_arr, #excludes_arr)
            }
        }
    }
    .into()
}
