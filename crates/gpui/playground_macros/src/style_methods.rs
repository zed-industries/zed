use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, Ident, Token};

struct Args {
    method_name: Ident,
    method_suffix: Option<Ident>,
    field_name: Ident,
    value: Expr,
}

impl syn::parse::Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let method_name = input.parse()?;
        let method_suffix = if input.peek(Token![::]) {
            input.parse::<Token![::]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        input.parse::<Token![,]>()?;
        let field_name = input.parse()?;
        input.parse::<Token![,]>()?;
        let value = input.parse()?;

        Ok(Self {
            method_name,
            method_suffix,
            field_name,
            value,
        })
    }
}

fn fixed_lengths() -> Vec<(&'static str, proc_macro2::TokenStream)> {
    vec![
        ("0", quote! { DefinedLength::Pixels(0.) }),
        ("px", quote! { DefinedLength::Pixels(1.) }),
        ("0_5", quote! { DefinedLength::Rems(0.125) }),
        // ...
    ]
}

pub fn style_methods(input: TokenStream) -> TokenStream {
    let Args {
        method_name,
        method_suffix,
        field_name,
        value,
    } = parse_macro_input!(input as Args);

    let hover_method_name = format!("hover_{}", method_name);
    let hover_method_ident = syn::Ident::new(&hover_method_name, method_name.span());

    let mut result = quote! {
        fn #method_name(mut self) -> Self
        where
            Self: Sized,
        {
            self.metadata().style.#field_name = #value;
            self
        }

        fn #hover_method_ident(mut self) -> Self
        where
            Self: Sized,
        {
            self.metadata().hover_style.#field_name = Some(#value);
            self
        }
    };

    if let Some(suffix_ident) = method_suffix {
        if suffix_ident == "_" {
            let fixed_lengths = fixed_lengths();

            for (suffix, value) in fixed_lengths {
                let method_ident =
                    syn::Ident::new(&format!("{}_{}", method_name, suffix), method_name.span());
                let method = quote! {
                    fn #method_ident(mut self) -> Self
                    where
                        Self: Sized,
                    {
                        self.metadata().style.#field_name = #value;
                        self
                    }
                };
                result.extend(method);
            }
        }
    }

    result.into()
}
