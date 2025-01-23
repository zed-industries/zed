use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, LitInt, LitStr};

struct ReplacePathInput {
    path: LitStr,
    depth: LitInt,
}

impl Parse for ReplacePathInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let path: LitStr = input.parse()?;
        input.parse::<syn::Token![,]>()?;
        let depth: LitInt = input.parse()?;
        Ok(ReplacePathInput { path, depth })
    }
}

#[proc_macro]
pub fn replace_path_slashes(input: TokenStream) -> TokenStream {
    let ReplacePathInput { path, depth } = parse_macro_input!(input as ReplacePathInput);
    let path = path.value();
    let mut depth: usize = depth
        .base10_parse()
        .expect("Depth must be a positive integer");

    let mut components: Vec<String> = path.split('/').into_iter().map(String::from).collect();
    components.reverse();

    if depth == 0 {
        depth = components.len() - 1;
    }
    for comp in components.iter_mut().skip(1) {
        if depth > 0 {
            comp.push('\\');
            depth -= 1;
        } else {
            comp.push('/');
        }
    }

    components.reverse();
    let new_path = components.concat();

    TokenStream::from(quote! {
        #new_path
    })
}
