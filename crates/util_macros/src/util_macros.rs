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
pub fn separator(input: TokenStream) -> TokenStream {
    let ReplacePathInput { path, index } = parse_macro_input!(input as ReplacePathInput);
    let path = path.value();
    let index = index.map(|idx| {
        idx.base10_parse::<usize>()
            .expect("Depth must be a positive integer")
    });

    let mut components: Vec<String> = path.split('/').into_iter().map(String::from).collect();

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
