//! A set of convenience macros for our wasmtime-c-api crate.
//!
//! These are intended to mirror the macros in the `wasm.h` header file and
//! largely facilitate the `declare_ref` macro.
//!
//! > **⚠️ Warning ⚠️**: this crate is an internal-only crate for the Wasmtime
//! > project and is not intended for general use. APIs are not strictly
//! > reviewed for safety and usage outside of Wasmtime may have bugs. If
//! > you're interested in using this feel free to file an issue on the
//! > Wasmtime repository to start a discussion about doing so, but otherwise
//! > be aware that your usage of this crate is not supported.

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;

fn extract_ident(input: proc_macro::TokenStream) -> Ident {
    let input = TokenStream::from(input);
    let i = match input.into_iter().next().unwrap() {
        TokenTree::Ident(i) => i,
        _ => panic!("expected an ident"),
    };
    let name = i.to_string();
    assert!(name.ends_with("_t"));
    return i;
}

#[proc_macro]
pub fn declare_own(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = extract_ident(input);
    let name = ty.to_string();
    let delete = quote::format_ident!("{}_delete", &name[..name.len() - 2]);
    let docs = format!("Deletes the [`{name}`].");

    (quote! {
        #[doc = #docs]
        #[unsafe(no_mangle)]
        pub extern fn #delete(_: Box<#ty>) {}
    })
    .into()
}

#[proc_macro]
pub fn declare_ty(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = extract_ident(input);
    let name = ty.to_string();
    let prefix = &name[..name.len() - 2];
    let copy = quote::format_ident!("{}_copy", &prefix);
    let docs = format!(
        "Creates a new [`{name}`] which matches the provided one.\n\n\
        The caller is responsible for deleting the returned value via [`{prefix}_delete`].\n\n\
    "
    );

    (quote! {
        wasmtime_c_api_macros::declare_own!(#ty);

        #[doc = #docs]
        #[unsafe(no_mangle)]
        pub extern fn #copy(src: &#ty) -> Box<#ty> {
            Box::new(src.clone())
        }
    })
    .into()
}

#[proc_macro]
pub fn declare_ref(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ty = extract_ident(input);
    let name = ty.to_string();
    let prefix = &name[..name.len() - 2];
    let same = quote::format_ident!("{}_same", prefix);
    let same_docs = format!(
        "Returns `true` if the given references are pointing to the same [`{name}`].\n\n\
        This is not yet supported and aborts the process upon use."
    );
    let get_host_info = quote::format_ident!("{}_get_host_info", prefix);
    let get_host_info_docs = format!(
        "Returns the host information of the [`{name}`].\n\n\
        This is not yet supported and always returns `NULL`."
    );
    let set_host_info = quote::format_ident!("{}_set_host_info", prefix);
    let set_host_info_docs = format!(
        "Sets the host information of the [`{name}`].\n\n\
        This is not yet supported and aborts the process upon use."
    );
    let set_host_info_final = quote::format_ident!("{}_set_host_info_with_finalizer", prefix);
    let set_host_info_final_docs = format!(
        "Sets the host information finalizer of the [`{name}`].\n\n\
        This is not yet supported and aborts the process upon use."
    );
    let as_ref = quote::format_ident!("{}_as_ref", prefix);
    let as_ref_docs = format!(
        "Returns the [`{name}`] as mutable reference.\n\n\
        This is not yet supported and aborts the process upon use."
    );
    let as_ref_const = quote::format_ident!("{}_as_ref_const", prefix);
    let as_ref_const_docs = format!(
        "Returns the [`{name}`] as immutable reference.\n\n\
        This is not yet supported and aborts the process upon use."
    );

    (quote! {
        wasmtime_c_api_macros::declare_ty!(#ty);

        #[doc = #same_docs]
        #[unsafe(no_mangle)]
        pub extern fn #same(_a: &#ty, _b: &#ty) -> bool {
            eprintln!("`{}` is not implemented", stringify!(#same));
            std::process::abort();
        }

        #[doc = #get_host_info_docs]
        #[unsafe(no_mangle)]
        pub extern fn #get_host_info(a: &#ty) -> *mut std::os::raw::c_void {
            std::ptr::null_mut()
        }

        #[doc = #set_host_info_docs]
        #[unsafe(no_mangle)]
        pub extern fn #set_host_info(a: &#ty, info: *mut std::os::raw::c_void) {
            eprintln!("`{}` is not implemented", stringify!(#set_host_info));
            std::process::abort();
        }

        #[doc = #set_host_info_final_docs]
        #[unsafe(no_mangle)]
        pub extern fn #set_host_info_final(
            a: &#ty,
            info: *mut std::os::raw::c_void,
            finalizer: Option<extern "C" fn(*mut std::os::raw::c_void)>,
        ) {
            eprintln!("`{}` is not implemented", stringify!(#set_host_info_final));
            std::process::abort();
        }

        #[doc = #as_ref_docs]
        #[unsafe(no_mangle)]
        pub extern fn #as_ref(a: &#ty) -> Box<crate::wasm_ref_t> {
            eprintln!("`{}` is not implemented", stringify!(#as_ref));
            std::process::abort();
        }

        #[doc = #as_ref_const_docs]
        #[unsafe(no_mangle)]
        pub extern fn #as_ref_const(a: &#ty) -> Box<crate::wasm_ref_t> {
            eprintln!("`{}` is not implemented", stringify!(#as_ref_const));
            std::process::abort();
        }

        // TODO: implement `wasm_ref_as_#name#`
        // TODO: implement `wasm_ref_as_#name#_const`
    })
    .into()
}
