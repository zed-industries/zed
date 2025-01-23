#![cfg_attr(not(target_os = "windows"), allow(unused))]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitInt, LitStr};

struct ReplacePathInput {
    path: LitStr,
    index: Option<LitInt>,
}

impl Parse for ReplacePathInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        let index = input
            .parse::<syn::Token![,]>()
            .ok()
            .and_then(|_| input.parse().ok());
        Ok(ReplacePathInput { path, index })
    }
}

#[proc_macro]
#[cfg(target_os = "windows")]
pub fn separator(input: TokenStream) -> TokenStream {
    let ReplacePathInput { path, index } = parse_macro_input!(input as ReplacePathInput);
    let path = path.value();
    let index = index.map(|idx| {
        idx.base10_parse::<usize>()
            .expect("Depth must be a positive integer")
    });

    let mut components: Vec<String> = path.split('/').map(String::from).collect();

    let num_take = components.len() - 1;
    if let Some(idx) = index {
        for (i, comp) in components.iter_mut().take(num_take).enumerate() {
            if i == idx {
                comp.push('\\');
            } else {
                comp.push('/');
            }
        }
    } else {
        for comp in components.iter_mut().take(num_take) {
            comp.push('\\');
        }
    }

    let new_path = components.concat();

    TokenStream::from(quote! {
        #new_path
    })
}

#[proc_macro]
#[cfg(not(target_os = "windows"))]
pub fn separator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReplacePathInput);
    let path = path.value();
    TokenStream::from(quote! {
        #path
    })
}

struct UriInput {
    uri: LitStr,
}

impl Parse for UriInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let uri: LitStr = input.parse()?;
        Ok(UriInput { uri })
    }
}

#[proc_macro]
#[cfg(target_os = "windows")]
pub fn uri(input: TokenStream) -> TokenStream {
    let UriInput { uri } = parse_macro_input!(input as UriInput);
    let uri = uri.value();
    let new_uri = uri.replace("file:///", "file:///C:/");

    TokenStream::from(quote! {
        #new_uri
    })
}

#[proc_macro]
#[cfg(not(target_os = "windows"))]
pub fn uri(input: TokenStream) -> TokenStream {
    input
}
