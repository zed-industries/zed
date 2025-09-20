#![cfg_attr(not(target_os = "windows"), allow(unused))]
#![allow(clippy::test_attr_in_doctest)]

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
struct PerfArgs {
    /// How many times to loop a test before rerunning the test binary.
    /// If left empty, the test harness will auto-determine this value.
    iterations: Option<syn::Expr>,
}

impl syn::parse::Parse for PerfArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(PerfArgs { iterations: None });
        }

        let mut iterations = None;
        // In principle we only have one possible argument, but leave this as
        // a loop in case we expand this in the future.
        for meta in
            syn::punctuated::Punctuated::<syn::Meta, syn::Token![,]>::parse_terminated(input)?
        {
            match &meta {
                syn::Meta::NameValue(meta_name_value) => {
                    if meta_name_value.path.is_ident("iterations") {
                        iterations = Some(meta_name_value.value.clone());
                    } else {
                        return Err(syn::Error::new_spanned(
                            &meta_name_value.path,
                            "unexpected argument, expected 'iterations'",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "expected name-value argument like 'iterations = 1'",
                    ));
                }
            }
        }

        Ok(PerfArgs { iterations })
    }
}

/// Marks a test as perf-sensitive, to be triaged when checking the performance
/// of a build. This also automatically applies `#[test]`.
///
/// By default, the number of iterations when profiling this test is auto-determined.
/// If this needs to be overwritten, pass the desired iteration count to the macro
/// as a parameter (`#[perf(iterations = n)]`). Note that the actual profiler may still
/// run the test an arbitrary number times; this flag just sets the number of executions
/// before the process is restarted and global state is reset.
///
/// # Usage notes
/// This should probably not be applied to tests that do any significant fs IO, as
/// locks on files may not be released in time when repeating a test many times. This
/// might lead to spurious failures.
///
/// # Examples
/// ```rust
/// use util_macros::perf;
///
/// #[perf]
/// fn expensive_computation_test() {
///     // Test goes here.
/// }
/// ```
///
/// This also works with `#[gpui::test]`s, though in most cases it shouldn't
/// be used with automatic iterations.
/// ```rust,ignore
/// use util_macros::perf;
///
/// #[perf(iterations = 1)]
/// #[gpui::test]
/// fn oneshot_test(_cx: &mut gpui::TestAppContext) {
///     // Test goes here.
/// }
/// ```
#[proc_macro_attribute]
pub fn perf(our_attr: TokenStream, input: TokenStream) -> TokenStream {
    // If any of the below constants are changed, make sure to also update the perf
    // profiler to match!

    /// The suffix on tests marked with `#[perf]`.
    const SUF_NORMAL: &str = "__ZED_PERF";
    /// The suffix on tests marked with `#[perf(iterations = n)]`.
    const SUF_FIXED: &str = "__ZED_PERF_FIXEDITER";
    /// The env var in which we pass the iteration count to our tests.
    const ITER_ENV_VAR: &str = "ZED_PERF_ITER";

    let iter_count = parse_macro_input!(our_attr as PerfArgs).iterations;

    let ItemFn {
        mut attrs,
        vis,
        mut sig,
        block,
    } = parse_macro_input!(input as ItemFn);
    attrs.push(parse_quote!(#[test]));
    attrs.push(parse_quote!(#[allow(non_snake_case)]));

    let block: Box<syn::Block> = if cfg!(perf_enabled) {
        // Make the ident obvious when calling, for the test parser.
        let mut new_ident = sig.ident.to_string();
        if iter_count.is_some() {
            new_ident.push_str(SUF_FIXED);
        } else {
            new_ident.push_str(SUF_NORMAL);
        }

        let new_ident = syn::Ident::new(&new_ident, sig.ident.span());
        sig.ident = new_ident;
        // If we have a preset iteration count, just use that.
        if let Some(iter_count) = iter_count {
            parse_quote!({
               for _ in 0..#iter_count {
                   #block
               }
            })
        } else {
            // Otherwise, the perf harness will pass us the value in an env var.
            parse_quote!({
                let iter_count = std::env::var(#ITER_ENV_VAR).unwrap().parse::<usize>().unwrap();
                for _ in 0..iter_count {
                    #block
                }
            })
        }
    } else {
        block
    };

    ItemFn {
        attrs,
        vis,
        sig,
        block,
    }
    .into_token_stream()
    .into()
}
