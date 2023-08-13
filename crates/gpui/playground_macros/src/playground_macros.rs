use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
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

    let mut output_functions = TokenStream2::new();

    for (length, value) in fixed_lengths() {
        let function_name = format_ident!("{}_{}", function_signature.ident, length);
        output_functions.extend(quote! {
            pub fn #function_name(mut self) -> Self {
                let #argument_name = #value.into();
                #function_body
            }
        });
    }

    output_functions.into()
}

fn fixed_lengths() -> Vec<(&'static str, TokenStream2)> {
    vec![
        ("0", quote! { Length::Pixels(0.) }),
        ("px", quote! { Length::Pixels(1.) }),
        ("0_5", quote! { Length::Rems(0.125) }),
        ("1", quote! { Length::Rems(0.25) }),
        ("1_5", quote! { Length::Rems(0.375) }),
        ("2", quote! { Length::Rems(0.5) }),
        ("2_5", quote! { Length::Rems(0.625) }),
        ("3", quote! { Length::Rems(0.75) }),
        ("3_5", quote! { Length::Rems(0.875) }),
        ("4", quote! { Length::Rems(1.) }),
        ("5", quote! { Length::Rems(1.25) }),
        ("6", quote! { Length::Rems(1.5) }),
        ("7", quote! { Length::Rems(1.75) }),
        ("8", quote! { Length::Rems(2.) }),
        ("9", quote! { Length::Rems(2.25) }),
        ("10", quote! { Length::Rems(2.5) }),
        ("11", quote! { Length::Rems(2.75) }),
        ("12", quote! { Length::Rems(3.) }),
        ("14", quote! { Length::Rems(3.5) }),
        ("16", quote! { Length::Rems(4.) }),
        ("20", quote! { Length::Rems(5.) }),
        ("24", quote! { Length::Rems(6.) }),
        ("28", quote! { Length::Rems(7.) }),
        ("32", quote! { Length::Rems(8.) }),
        ("36", quote! { Length::Rems(9.) }),
        ("40", quote! { Length::Rems(10.) }),
        ("44", quote! { Length::Rems(11.) }),
        ("48", quote! { Length::Rems(12.) }),
        ("52", quote! { Length::Rems(13.) }),
        ("56", quote! { Length::Rems(14.) }),
        ("60", quote! { Length::Rems(15.) }),
        ("64", quote! { Length::Rems(16.) }),
        ("72", quote! { Length::Rems(18.) }),
        ("80", quote! { Length::Rems(20.) }),
        ("96", quote! { Length::Rems(24.) }),
        ("half", quote! { Length::Percent(50.) }),
        ("1_3rd", quote! { Length::Percent(33.333333) }),
        ("2_3rd", quote! { Length::Percent(66.666667) }),
        ("1_4th", quote! { Length::Percent(25.) }),
        ("2_4th", quote! { Length::Percent(50.) }),
        ("3_4th", quote! { Length::Percent(75.) }),
        ("1_5th", quote! { Length::Percent(20.) }),
        ("2_5th", quote! { Length::Percent(40.) }),
        ("3_5th", quote! { Length::Percent(60.) }),
        ("4_5th", quote! { Length::Percent(80.) }),
        ("1_6th", quote! { Length::Percent(16.666667) }),
        ("2_6th", quote! { Length::Percent(33.333333) }),
        ("3_6th", quote! { Length::Percent(50.) }),
        ("4_6th", quote! { Length::Percent(66.666667) }),
        ("5_6th", quote! { Length::Percent(83.333333) }),
        ("1_12th", quote! { Length::Percent(8.333333) }),
        ("2_12th", quote! { Length::Percent(16.666667) }),
        ("3_12th", quote! { Length::Percent(25.) }),
        ("4_12th", quote! { Length::Percent(33.333333) }),
        ("5_12th", quote! { Length::Percent(41.666667) }),
        ("6_12th", quote! { Length::Percent(50.) }),
        ("7_12th", quote! { Length::Percent(58.333333) }),
        ("8_12th", quote! { Length::Percent(66.666667) }),
        ("9_12th", quote! { Length::Percent(75.) }),
        ("10_12th", quote! { Length::Percent(83.333333) }),
        ("11_12th", quote! { Length::Percent(91.666667) }),
        ("full", quote! { Length::Percent(100.) }),
    ]
}
