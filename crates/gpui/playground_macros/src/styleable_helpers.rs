use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
};

struct StyleableMacroInput;

impl Parse for StyleableMacroInput {
    fn parse(_input: ParseStream) -> Result<Self> {
        Ok(StyleableMacroInput)
    }
}

pub fn styleable_helpers(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as StyleableMacroInput);
    let methods = generate_methods();
    let output = quote! {
        #(#methods)*
    };
    output.into()
}

fn generate_methods() -> Vec<TokenStream2> {
    let mut methods = Vec::new();

    for (prefix, auto_allowed, fields) in tailwind_prefixes() {
        for (suffix, length_tokens) in tailwind_lengths() {
            if !auto_allowed && suffix == "auto" {
                // Conditional to skip "auto"
                continue;
            }

            let method_name = format_ident!("{}_{}", prefix, suffix);
            let field_assignments = fields
                .iter()
                .map(|field_tokens| {
                    quote! {
                        style.#field_tokens = Some(gpui::geometry::#length_tokens);
                    }
                })
                .collect::<Vec<_>>();

            let method = quote! {
                fn #method_name(mut self) -> Self where Self: std::marker::Sized {
                    let mut style = self.declared_style();
                    #(#field_assignments)*
                    self
                }
            };

            methods.push(method);
        }
    }

    methods
}

fn tailwind_lengths() -> Vec<(&'static str, TokenStream2)> {
    vec![
        ("0", quote! { pixels(0.) }),
        ("1", quote! { rems(0.25) }),
        ("2", quote! { rems(0.5) }),
        ("3", quote! { rems(0.75) }),
        ("4", quote! { rems(1.) }),
        ("5", quote! { rems(1.25) }),
        ("6", quote! { rems(1.5) }),
        ("8", quote! { rems(2.0) }),
        ("10", quote! { rems(2.5) }),
        ("12", quote! { rems(3.) }),
        ("16", quote! { rems(4.) }),
        ("20", quote! { rems(5.) }),
        ("24", quote! { rems(6.) }),
        ("32", quote! { rems(8.) }),
        ("40", quote! { rems(10.) }),
        ("48", quote! { rems(12.) }),
        ("56", quote! { rems(14.) }),
        ("64", quote! { rems(16.) }),
        ("auto", quote! { auto() }),
        ("px", quote! { pixels(1.) }),
        ("full", quote! { relative(1.) }),
        // ("screen_50", quote! { DefiniteLength::Vh(50.0) }),
        // ("screen_75", quote! { DefiniteLength::Vh(75.0) }),
        // ("screen", quote! { DefiniteLength::Vh(100.0) }),
    ]
}

fn tailwind_prefixes() -> Vec<(&'static str, bool, Vec<TokenStream2>)> {
    vec![
        ("w", true, vec![quote! { size.width }]),
        ("h", true, vec![quote! { size.height }]),
        ("min_w", false, vec![quote! { min_size.width }]),
        ("min_h", false, vec![quote! { min_size.height }]),
        ("max_w", false, vec![quote! { max_size.width }]),
        ("max_h", false, vec![quote! { max_size.height }]),
        (
            "m",
            true,
            vec![quote! { margin.top }, quote! { margin.bottom }],
        ),
        ("mt", true, vec![quote! { margin.top }]),
        ("mb", true, vec![quote! { margin.bottom }]),
        (
            "mx",
            true,
            vec![quote! { margin.left }, quote! { margin.right }],
        ),
        ("ml", true, vec![quote! { margin.left }]),
        ("mr", true, vec![quote! { margin.right }]),
        (
            "p",
            false,
            vec![quote! { padding.top }, quote! { padding.bottom }],
        ),
        ("pt", false, vec![quote! { padding.top }]),
        ("pb", false, vec![quote! { padding.bottom }]),
        (
            "px",
            false,
            vec![quote! { padding.left }, quote! { padding.right }],
        ),
        ("pl", false, vec![quote! { padding.left }]),
        ("pr", false, vec![quote! { padding.right }]),
        ("top", true, vec![quote! { inset.top }]),
        ("bottom", true, vec![quote! { inset.bottom }]),
        ("left", true, vec![quote! { inset.left }]),
        ("right", true, vec![quote! { inset.right }]),
    ]
}
