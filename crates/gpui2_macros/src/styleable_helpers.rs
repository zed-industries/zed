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

    for (prefix, auto_allowed, fields) in box_prefixes() {
        for (suffix, length_tokens, doc_string) in box_suffixes() {
            if auto_allowed || suffix != "auto" {
                let method = generate_method(prefix, suffix, &fields, length_tokens, doc_string);
                methods.push(method);
            }
        }
    }

    for (prefix, fields) in corner_prefixes() {
        for (suffix, radius_tokens, doc_string) in corner_suffixes() {
            let method = generate_method(prefix, suffix, &fields, radius_tokens, doc_string);
            methods.push(method);
        }
    }

    for (prefix, fields) in border_prefixes() {
        for (suffix, width_tokens, doc_string) in border_suffixes() {
            let method = generate_method(prefix, suffix, &fields, width_tokens, doc_string);
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
    doc_string: &'static str,
) -> TokenStream2 {
    let method_name = if suffix.is_empty() {
        format_ident!("{}", prefix)
    } else {
        format_ident!("{}_{}", prefix, suffix)
    };

    let field_assignments = fields
        .iter()
        .map(|field_tokens| {
            quote! {
                style.#field_tokens = Some(gpui::geometry::#length_tokens);
            }
        })
        .collect::<Vec<_>>();

    let method = quote! {
        #[doc = #doc_string]
        fn #method_name(mut self) -> Self where Self: std::marker::Sized {
            let mut style = self.declared_style();
            #(#field_assignments)*
            self
        }
    };

    method
}

fn box_prefixes() -> Vec<(&'static str, bool, Vec<TokenStream2>)> {
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
            vec![
                quote! { margin.top },
                quote! { margin.bottom },
                quote! { margin.left },
                quote! { margin.right },
            ],
        ),
        ("mt", true, vec![quote! { margin.top }]),
        ("mb", true, vec![quote! { margin.bottom }]),
        (
            "my",
            true,
            vec![quote! { margin.top }, quote! { margin.bottom }],
        ),
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
            vec![
                quote! { padding.top },
                quote! { padding.bottom },
                quote! { padding.left },
                quote! { padding.right },
            ],
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

fn box_suffixes() -> Vec<(&'static str, TokenStream2, &'static str)> {
    vec![
        ("0", quote! { pixels(0.) }, "0px"),
        ("0p5", quote! { rems(0.125) }, "2px (0.125rem)"),
        ("1", quote! { rems(0.25) }, "4px (0.25rem)"),
        ("1p5", quote! { rems(0.375) }, "6px (0.375rem)"),
        ("2", quote! { rems(0.5) }, "8px (0.5rem)"),
        ("2p5", quote! { rems(0.625) }, "10px (0.625rem)"),
        ("3", quote! { rems(0.75) }, "12px (0.75rem)"),
        ("3p5", quote! { rems(0.875) }, "14px (0.875rem)"),
        ("4", quote! { rems(1.) }, "16px (1rem)"),
        ("5", quote! { rems(1.25) }, "20px (1.25rem)"),
        ("6", quote! { rems(1.5) }, "24px (1.5rem)"),
        ("7", quote! { rems(1.75) }, "28px (1.77rem)"),
        ("8", quote! { rems(2.0) }, "32px (2rem)"),
        ("9", quote! { rems(2.25) }, "36px (2.25rem)"),
        ("10", quote! { rems(2.5) }, "40px (2.5rem)"),
        ("11", quote! { rems(2.75) }, "44px (2.75rem)"),
        ("12", quote! { rems(3.) }, "48px (3rem)"),
        ("16", quote! { rems(4.) }, "64px (4rem)"),
        ("20", quote! { rems(5.) }, "80px (5rem)"),
        ("24", quote! { rems(6.) }, "96px (6rem)"),
        ("32", quote! { rems(8.) }, "128px (8rem)"),
        ("40", quote! { rems(10.) }, "160px (10rem)"),
        ("48", quote! { rems(12.) }, "192px (12rem)"),
        ("56", quote! { rems(14.) }, "224px (14rem)"),
        ("64", quote! { rems(16.) }, "256px (16rem)"),
        ("72", quote! { rems(18.) }, "288px (18rem)"),
        ("80", quote! { rems(20.) }, "320px (20rem)"),
        ("96", quote! { rems(24.) }, "384px (24rem)"),
        ("auto", quote! { auto() }, "Auto"),
        ("px", quote! { pixels(1.) }, "1px"),
        ("full", quote! { relative(1.) }, "100%"),
        ("1_2", quote! { relative(0.5) }, "50% (1/2)"),
        ("1_3", quote! { relative(1./3.) }, "33% (1/3)"),
        ("2_3", quote! { relative(2./3.) }, "66% (2/3)"),
        ("1_4", quote! { relative(0.25) }, "25% (1/4)"),
        ("2_4", quote! { relative(0.5) }, "50% (2/4)"),
        ("3_4", quote! { relative(0.75) }, "75% (3/4)"),
        ("1_5", quote! { relative(0.2) }, "20% (1/5)"),
        ("2_5", quote! { relative(0.4) }, "40% (2/5)"),
        ("3_5", quote! { relative(0.6) }, "60% (3/5)"),
        ("4_5", quote! { relative(0.8) }, "80% (4/5)"),
        ("1_6", quote! { relative(1./6.) }, "16% (1/6)"),
        ("5_6", quote! { relative(5./6.) }, "80% (5/6)"),
        ("1_12", quote! { relative(1./12.) }, "8% (1/12)"),
    ]
}

fn corner_prefixes() -> Vec<(&'static str, Vec<TokenStream2>)> {
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

fn corner_suffixes() -> Vec<(&'static str, TokenStream2, &'static str)> {
    vec![
        ("none", quote! { pixels(0.) }, "0px"),
        ("sm", quote! { rems(0.125) }, "2px (0.125rem)"),
        ("md", quote! { rems(0.25) }, "4px (0.25rem)"),
        ("lg", quote! { rems(0.5) }, "8px (0.5rem)"),
        ("xl", quote! { rems(0.75) }, "12px (0.75rem)"),
        ("2xl", quote! { rems(1.) }, "16px (1rem)"),
        ("3xl", quote! { rems(1.5) }, "24px (1.5rem)"),
        ("full", quote! {  pixels(9999.) }, "9999px"),
    ]
}

fn border_prefixes() -> Vec<(&'static str, Vec<TokenStream2>)> {
    vec![
        (
            "border",
            vec![
                quote! { border_widths.top },
                quote! { border_widths.right },
                quote! { border_widths.bottom },
                quote! { border_widths.left },
            ],
        ),
        ("border_t", vec![quote! { border_widths.top }]),
        ("border_b", vec![quote! { border_widths.bottom }]),
        ("border_r", vec![quote! { border_widths.right }]),
        ("border_l", vec![quote! { border_widths.left }]),
        (
            "border_x",
            vec![
                quote! { border_widths.left },
                quote! { border_widths.right },
            ],
        ),
        (
            "border_y",
            vec![
                quote! { border_widths.top },
                quote! { border_widths.bottom },
            ],
        ),
    ]
}

fn border_suffixes() -> Vec<(&'static str, TokenStream2, &'static str)> {
    vec![
        ("", quote! { pixels(1.)}, "1px"),
        ("0", quote! { pixels(0.)}, "0px"),
        ("1", quote! { pixels(1.) }, "1px"),
        ("2", quote! { pixels(2.) }, "2px"),
        ("3", quote! { pixels(3.) }, "3px"),
        ("4", quote! { pixels(4.) }, "4px"),
        ("5", quote! { pixels(5.) }, "5px"),
        ("6", quote! { pixels(6.) }, "6px"),
        ("7", quote! { pixels(7.) }, "7px"),
        ("8", quote! { pixels(8.) }, "8px"),
        ("9", quote! { pixels(9.) }, "9px"),
        ("10", quote! { pixels(10.) }, "10px"),
        ("11", quote! { pixels(11.) }, "11px"),
        ("12", quote! { pixels(12.) }, "12px"),
        ("16", quote! { pixels(16.) }, "16px"),
        ("20", quote! { pixels(20.) }, "20px"),
        ("24", quote! { pixels(24.) }, "24px"),
        ("32", quote! { pixels(32.) }, "32px"),
    ]
}
