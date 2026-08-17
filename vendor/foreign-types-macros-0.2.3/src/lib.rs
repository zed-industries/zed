extern crate proc_macro;

use proc_macro::TokenStream;
use syn::parse_macro_input;

use crate::parse::Input;

mod build;
mod parse;

#[proc_macro]
pub fn foreign_type_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Input);
    build::build(input).into()
}
