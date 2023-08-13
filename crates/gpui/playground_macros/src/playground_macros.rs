use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, PatType};

#[proc_macro_attribute]
pub fn tailwind_lengths(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_function = parse_macro_input!(item as ItemFn);
    let function_signature = input_function.sig.clone();
    let function_body = input_function.block;

    let argument_name = match function_signature.inputs.iter().nth(1) {
        Some(FnArg::Typed(PatType { pat, .. })) => pat,
        _ => panic!("Couldn't find the second argument in the function signature"),
    };

    let scale_lengths = [
        ("0", quote! { Length::Rems(0.) }),
        ("px", quote! { Length::Pixels(1.) }),
        // ...
        ("auto", quote! { Length::Auto }),
    ];

    let mut output_functions = proc_macro2::TokenStream::new();

    for (length, value) in &scale_lengths {
        let function_name = format_ident!("{}_{}", function_signature.ident, length);
        output_functions.extend(quote! {
            pub fn #function_name(mut self) -> Self {
                let #argument_name = #value;
                #function_body
            }
        });
    }

    output_functions.into()
}
