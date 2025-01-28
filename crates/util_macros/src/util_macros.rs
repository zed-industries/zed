#![cfg_attr(not(target_os = "windows"), allow(unused))]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

/// This macro replaces the path separator `/` with `\` for Windows.
/// But if the target OS is not Windows, the path is returned as is.
///
/// # Example
/// ```rust
/// # use util_macros::separator;
/// let path = separator!("path/to/file");
/// #[cfg(target_os = "windows")]
/// assert_eq!(path, "path\\to\\file");
/// #[cfg(not(target_os = "windows"))]
/// assert_eq!(path, "path/to/file");
/// ```
#[proc_macro]
pub fn separator(input: TokenStream) -> TokenStream {
    let path = parse_macro_input!(input as LitStr);
    let path = path.value();

    #[cfg(target_os = "windows")]
    let path = path.replace("/", "\\");

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
