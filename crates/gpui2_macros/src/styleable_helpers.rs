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

    for (prefix, auto_allowed, fields) in tailwind_length_prefixes() {
        for (suffix, length_tokens) in tailwind_lengths() {
            if auto_allowed || suffix != "auto" {
                let method = generate_method(prefix, suffix, &fields, length_tokens);
                methods.push(method);
            }
        }
    }

    for (prefix, fields) in tailwind_corner_prefixes() {
        for (suffix, radius_tokens) in tailwind_corner_radii() {
            let method = generate_method(prefix, suffix, &fields, radius_tokens);
            methods.push(method);
        }
    }

    methods
}

fn generate_method(
    prefix: &'static str,
    suffix: &'static str,
    fields: &Vec<TokenStream2>,
    length_tokens: TokenStream2,
) -> TokenStream2 {
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

    method
}

fn tailwind_length_prefixes() -> Vec<(&'static str, bool, Vec<TokenStream2>)> {
    vec![
        ("w", true, vec![quote! { size.width }]),
        ("h", true, vec![quote! { size.height }]),
        (
            "size",
            true,
            vec![quote! {size.width}, quote! {size.height}],
        ),
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
        (
            "py",
            false,
            vec![quote! { padding.top }, quote! { padding.bottom }],
        ),
        ("pl", false, vec![quote! { padding.left }]),
        ("pr", false, vec![quote! { padding.right }]),
        ("top", true, vec![quote! { inset.top }]),
        ("bottom", true, vec![quote! { inset.bottom }]),
        ("left", true, vec![quote! { inset.left }]),
        ("right", true, vec![quote! { inset.right }]),
        (
            "gap",
            false,
            vec![quote! { gap.width }, quote! { gap.height }],
        ),
        ("gap_x", false, vec![quote! { gap.width }]),
        ("gap_y", false, vec![quote! { gap.height }]),
    ]
}

fn tailwind_lengths() -> Vec<(&'static str, TokenStream2)> {
    vec![
        ("0", quote! { pixels(0.) }),
        ("0p5", quote! { rems(0.125) }),
        ("1", quote! { rems(0.25) }),
        ("1p5", quote! { rems(0.375) }),
        ("2", quote! { rems(0.5) }),
        ("2p5", quote! { rems(0.625) }),
        ("3", quote! { rems(0.75) }),
        ("3p5", quote! { rems(0.875) }),
        ("4", quote! { rems(1.) }),
        ("5", quote! { rems(1.25) }),
        ("6", quote! { rems(1.5) }),
        ("7", quote! { rems(1.75) }),
        ("8", quote! { rems(2.0) }),
        ("9", quote! { rems(2.25) }),
        ("10", quote! { rems(2.5) }),
        ("11", quote! { rems(2.75) }),
        ("12", quote! { rems(3.) }),
        ("16", quote! { rems(4.) }),
        ("20", quote! { rems(5.) }),
        ("24", quote! { rems(6.) }),
        ("32", quote! { rems(8.) }),
        ("40", quote! { rems(10.) }),
        ("48", quote! { rems(12.) }),
        ("56", quote! { rems(14.) }),
        ("64", quote! { rems(16.) }),
        ("72", quote! { rems(18.) }),
        ("80", quote! { rems(20.) }),
        ("96", quote! { rems(24.) }),
        ("auto", quote! { auto() }),
        ("px", quote! { pixels(1.) }),
        ("full", quote! { relative(1.) }),
        ("1_2", quote! { relative(0.5) }),
        ("1_3", quote! { relative(1./3.) }),
        ("2_3", quote! { relative(2./3.) }),
        ("1_4", quote! { relative(0.25) }),
        ("2_4", quote! { relative(0.5) }),
        ("3_4", quote! { relative(0.75) }),
        ("1_5", quote! { relative(0.2) }),
        ("2_5", quote! { relative(0.4) }),
        ("3_5", quote! { relative(0.6) }),
        ("4_5", quote! { relative(0.8) }),
        ("1_6", quote! { relative(1./6.) }),
        ("5_6", quote! { relative(5./6.) }),
        ("1_12", quote! { relative(1./12.) }),
        // ("screen_50", quote! { DefiniteLength::Vh(50.0) }),
        // ("screen_75", quote! { DefiniteLength::Vh(75.0) }),
        // ("screen", quote! { DefiniteLength::Vh(100.0) }),
    ]
}

fn tailwind_corner_prefixes() -> Vec<(&'static str, Vec<TokenStream2>)> {
    vec![
        (
            "rounded",
            vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.top_right },
                quote! { corner_radii.bottom_right },
                quote! { corner_radii.bottom_left },
            ],
        ),
        (
            "rounded_t",
            vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.top_right },
            ],
        ),
        (
            "rounded_b",
            vec![
                quote! { corner_radii.bottom_left },
                quote! { corner_radii.bottom_right },
            ],
        ),
        (
            "rounded_r",
            vec![
                quote! { corner_radii.top_right },
                quote! { corner_radii.bottom_right },
            ],
        ),
        (
            "rounded_l",
            vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.bottom_left },
            ],
        ),
        ("rounded_tl", vec![quote! { corner_radii.top_left }]),
        ("rounded_tr", vec![quote! { corner_radii.top_right }]),
        ("rounded_bl", vec![quote! { corner_radii.bottom_left }]),
        ("rounded_br", vec![quote! { corner_radii.bottom_right }]),
    ]
}

fn tailwind_corner_radii() -> Vec<(&'static str, TokenStream2)> {
    vec![
        ("none", quote! { pixels(0.) }),
        ("sm", quote! { rems(0.125) }),
        ("md", quote! { rems(0.25) }),
        ("lg", quote! { rems(0.5) }),
        ("xl", quote! { rems(0.75) }),
        ("2xl", quote! { rems(1.) }),
        ("3xl", quote! { rems(1.5) }),
        ("full", quote! {  pixels(9999.) }),
    ]
}
