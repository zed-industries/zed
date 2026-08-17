extern crate proc_macro;

use std::iter::FromIterator;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn cold_for_target_arch(attr: TokenStream, item: TokenStream) -> TokenStream {
    let arch_list = attr.to_string();
    let mut out: Vec<TokenStream> = arch_list.split(",").map(|a| {
        let a = a.trim().split("\"").nth(1).expect("A ','-separated list of \"arguments\" expected");
        (quote! {
            #[cfg_attr(target_arch = #a, cold)]
        }).into()
    }).collect();

    out.push(item);

    TokenStream::from_iter(out.into_iter())
}


