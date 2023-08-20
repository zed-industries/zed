use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

fn tailwind_lengths() -> Vec<(&'static str, TokenStream2)> {
    vec![
        ("0", quote! { DefinedLength::Pixels(0.) }),
        ("1", quote! { DefinedLength::Rems(0.25) }),
        ("2", quote! { DefinedLength::Rems(0.5) }),
        ("3", quote! { DefinedLength::Rems(0.75) }),
        ("4", quote! { DefinedLength::Rems(1.0) }),
        ("5", quote! { DefinedLength::Rems(1.25) }),
        ("6", quote! { DefinedLength::Rems(1.5) }),
        ("8", quote! { DefinedLength::Rems(2.0) }),
        ("10", quote! { DefinedLength::Rems(2.5) }),
        ("12", quote! { DefinedLength::Rems(3.0) }),
        ("16", quote! { DefinedLength::Rems(4.0) }),
        ("20", quote! { DefinedLength::Rems(5.0) }),
        ("24", quote! { DefinedLength::Rems(6.0) }),
        ("32", quote! { DefinedLength::Rems(8.0) }),
        ("40", quote! { DefinedLength::Rems(10.0) }),
        ("48", quote! { DefinedLength::Rems(12.0) }),
        ("56", quote! { DefinedLength::Rems(14.0) }),
        ("64", quote! { DefinedLength::Rems(16.0) }),
        ("auto", quote! { Length::Auto }),
        ("px", quote! { DefinedLength::Pixels(1.0) }),
        ("full", quote! { DefinedLength::Percent(100.0) }),
        // ("screen_50", quote! { DefinedLength::Vh(50.0) }),
        // ("screen_75", quote! { DefinedLength::Vh(75.0) }),
        // ("screen", quote! { DefinedLength::Vh(100.0) }),
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

pub fn styleable_trait(_item: TokenStream) -> TokenStream {
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
                        style.#field_tokens = Some(gpui::geometry::#length_tokens.into());
                    }
                })
                .collect::<Vec<_>>();

            let method = quote! {
                fn #method_name(mut self) -> Self where Self: Sized {
                    let mut style = self.declared_style();
                    #(#field_assignments)*
                    self
                }
            };

            methods.push(method);
        }
    }

    let output = quote! {
        pub trait Styleable {
            type Style: refineable::Refineable;

            fn declared_style(&mut self) -> &mut playground::style::StyleRefinement;

            fn style(&mut self) -> playground::style::Style {
                let mut style = playground::style::Style::default();
                style.refine(self.declared_style());
                style
            }

            #(#methods)*
        }
    };

    output.into()
}
