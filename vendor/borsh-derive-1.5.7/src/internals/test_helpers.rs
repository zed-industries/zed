use super::cratename::BORSH;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::fmt::Write;
use syn::{Ident, Path};

pub fn pretty_print_syn_str(input: &TokenStream) -> syn::Result<String> {
    let input = format!("{}", quote!(#input));
    let syn_file = syn::parse_str::<syn::File>(&input)?;

    Ok(prettyplease::unparse(&syn_file))
}

pub fn debug_print_vec_of_tokenizable<T: ToTokens>(optional: Option<Vec<T>>) -> String {
    let mut s = String::new();
    if let Some(vec) = optional {
        for element in vec {
            writeln!(&mut s, "{}", element.to_token_stream()).unwrap();
        }
    } else {
        write!(&mut s, "None").unwrap();
    }
    s
}

pub fn debug_print_tokenizable<T: ToTokens>(optional: Option<T>) -> String {
    let mut s = String::new();
    if let Some(type_) = optional {
        writeln!(&mut s, "{}", type_.to_token_stream()).unwrap();
    } else {
        write!(&mut s, "None").unwrap();
    }
    s
}

macro_rules! local_insta_assert_debug_snapshot {
    ($value:expr) => {{

        insta::with_settings!({prepend_module_to_snapshot => false}, {
            insta::assert_debug_snapshot!($value);
        });
    }};
}

macro_rules! local_insta_assert_snapshot {
    ($value:expr) => {{

        insta::with_settings!({prepend_module_to_snapshot => false}, {
            insta::assert_snapshot!($value);
        });
    }};
}

pub(crate) fn default_cratename() -> Path {
    let cratename = Ident::new(BORSH, Span::call_site());
    cratename.into()
}

pub(crate) use {local_insta_assert_debug_snapshot, local_insta_assert_snapshot};
