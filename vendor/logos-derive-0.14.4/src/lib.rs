use proc_macro::TokenStream;

#[proc_macro_derive(Logos, attributes(logos, extras, error, end, token, regex))]
pub fn logos(input: TokenStream) -> TokenStream {
    logos_codegen::generate(input.into()).into()
}
