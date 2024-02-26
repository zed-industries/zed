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

pub fn style_helpers(input: TokenStream) -> TokenStream {
    let _ = parse_macro_input!(input as StyleableMacroInput);
    let methods = generate_methods();
    let output = quote! {
        #(#methods)*
    };

    output.into()
}

fn generate_methods() -> Vec<TokenStream2> {
    let mut methods = Vec::new();

    for (prefix, auto_allowed, fields, prefix_doc_string) in box_prefixes() {
        methods.push(generate_custom_value_setter(
            prefix,
            if auto_allowed {
                quote! { Length }
            } else {
                quote! { DefiniteLength }
            },
            &fields,
            prefix_doc_string,
        ));

        for (suffix, length_tokens, suffix_doc_string) in box_suffixes() {
            if suffix != "auto" || auto_allowed {
                methods.push(generate_predefined_setter(
                    prefix,
                    suffix,
                    &fields,
                    &length_tokens,
                    false,
                    &format!("{prefix_doc_string}\n\n{suffix_doc_string}"),
                ));
            }

            if suffix != "auto" {
                methods.push(generate_predefined_setter(
                    prefix,
                    suffix,
                    &fields,
                    &length_tokens,
                    true,
                    &format!("{prefix_doc_string}\n\n{suffix_doc_string}"),
                ));
            }
        }
    }

    for (prefix, fields, prefix_doc_string) in corner_prefixes() {
        methods.push(generate_custom_value_setter(
            prefix,
            quote! { AbsoluteLength },
            &fields,
            prefix_doc_string,
        ));

        for (suffix, radius_tokens, suffix_doc_string) in corner_suffixes() {
            methods.push(generate_predefined_setter(
                prefix,
                suffix,
                &fields,
                &radius_tokens,
                false,
                &format!("{prefix_doc_string}\n\n{suffix_doc_string}"),
            ));
        }
    }

    for (prefix, fields, prefix_doc_string) in border_prefixes() {
        methods.push(generate_custom_value_setter(
            // The plain method names (e.g., `border`, `border_t`, `border_r`, etc.) are special-cased
            // versions of the 1px variants. This better matches Tailwind, but breaks our existing
            // convention of the suffix-less variant of the method being the one that accepts a custom value
            //
            // To work around this, we're assigning a `_width` suffix here.
            &format!("{prefix}_width"),
            quote! { AbsoluteLength },
            &fields,
            prefix_doc_string,
        ));

        for (suffix, width_tokens, suffix_doc_string) in border_suffixes() {
            methods.push(generate_predefined_setter(
                prefix,
                suffix,
                &fields,
                &width_tokens,
                false,
                &format!("{prefix_doc_string}\n\n{suffix_doc_string}"),
            ));
        }
    }
    methods
}

fn generate_predefined_setter(
    name: &'static str,
    length: &'static str,
    fields: &[TokenStream2],
    length_tokens: &TokenStream2,
    negate: bool,
    doc_string: &str,
) -> TokenStream2 {
    let (negation_prefix, negation_token) = if negate {
        ("neg_", quote! { - })
    } else {
        ("", quote! {})
    };

    let method_name = if length.is_empty() {
        format_ident!("{}{}", negation_prefix, name)
    } else {
        format_ident!("{}{}_{}", negation_prefix, name, length)
    };

    let field_assignments = fields
        .iter()
        .map(|field_tokens| {
            quote! {
                style.#field_tokens = Some((#negation_token gpui::#length_tokens).into());
            }
        })
        .collect::<Vec<_>>();

    let method = quote! {
        #[doc = #doc_string]
        fn #method_name(mut self) -> Self {
            let style = self.style();
            #(#field_assignments)*
            self
        }
    };

    method
}

fn generate_custom_value_setter(
    prefix: &str,
    length_type: TokenStream2,
    fields: &[TokenStream2],
    doc_string: &str,
) -> TokenStream2 {
    let method_name = format_ident!("{}", prefix);

    let mut iter = fields.iter();
    let last = iter.next_back().unwrap();
    let field_assignments = iter
        .map(|field_tokens| {
            quote! {
                style.#field_tokens = Some(length.clone().into());
            }
        })
        .chain(std::iter::once(quote! {
            style.#last = Some(length.into());
        }))
        .collect::<Vec<_>>();

    let method = quote! {
        #[doc = #doc_string]
        fn #method_name(mut self, length: impl std::clone::Clone + Into<gpui::#length_type>) -> Self {
            let style = self.style();
            #(#field_assignments)*
            self
        }
    };

    method
}

/// Returns a vec of (Property name, has 'auto' suffix, tokens for accessing the property, documentation)
fn box_prefixes() -> Vec<(&'static str, bool, Vec<TokenStream2>, &'static str)> {
    vec![
        (
            "w",
            true,
            vec![quote! { size.width }],
            "Sets the width of the element. [Docs](https://tailwindcss.com/docs/width)",
        ),
        ("h", true, vec![quote! { size.height }], "Sets the height of the element. [Docs](https://tailwindcss.com/docs/height)"),
        (
            "size",
            true,
            vec![quote! {size.width}, quote! {size.height}],
            "Sets the width and height of the element."
        ),
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        (
            "min_w",
            true,
            vec![quote! { min_size.width }],
            "Sets the minimum width of the element. [Docs](https://tailwindcss.com/docs/min-width)",
        ),
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        (
            "min_h",
            true,
            vec![quote! { min_size.height }],
            "Sets the minimum height of the element. [Docs](https://tailwindcss.com/docs/min-height)",
        ),
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        (
            "max_w",
            true,
            vec![quote! { max_size.width }],
            "Sets the maximum width of the element. [Docs](https://tailwindcss.com/docs/max-width)",
        ),
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        (
            "max_h",
            true,
            vec![quote! { max_size.height }],
            "Sets the maximum height of the element. [Docs](https://tailwindcss.com/docs/max-height)",
        ),
        (
            "m",
            true,
            vec![
                quote! { margin.top },
                quote! { margin.bottom },
                quote! { margin.left },
                quote! { margin.right },
            ],
            "Sets the margin of the element. [Docs](https://tailwindcss.com/docs/margin)"
        ),
        ("mt", true, vec![quote! { margin.top }], "Sets the top margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)"),
        (
            "mb",
            true,
            vec![quote! { margin.bottom }],
            "Sets the bottom margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)"
        ),
        (
            "my",
            true,
            vec![quote! { margin.top }, quote! { margin.bottom }],
            "Sets the vertical margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-vertical-margin)"
        ),
        (
            "mx",
            true,
            vec![quote! { margin.left }, quote! { margin.right }],
            "Sets the horizontal margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-horizontal-margin)"
        ),
        ("ml", true, vec![quote! { margin.left }], "Sets the left margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)"),
        (
            "mr",
            true,
            vec![quote! { margin.right }],
            "Sets the right margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)"
        ),
        (
            "p",
            false,
            vec![
                quote! { padding.top },
                quote! { padding.bottom },
                quote! { padding.left },
                quote! { padding.right },
            ],
            "Sets the padding of the element. [Docs](https://tailwindcss.com/docs/padding)"
        ),
        (
            "pt",
            false,
            vec![quote! { padding.top }],
            "Sets the top padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)"
        ),
        (
            "pb",
            false,
            vec![quote! { padding.bottom }],
            "Sets the bottom padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)"
        ),
        (
            "px",
            false,
            vec![quote! { padding.left }, quote! { padding.right }],
            "Sets the horizontal padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-horizontal-padding)"
        ),
        (
            "py",
            false,
            vec![quote! { padding.top }, quote! { padding.bottom }],
            "Sets the vertical padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-vertical-padding)"
        ),
        (
            "pl",
            false,
            vec![quote! { padding.left }],
            "Sets the left padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)"
        ),
        (
            "pr",
            false,
            vec![quote! { padding.right }],
            "Sets the right padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)"
        ),
        (
            "inset",
            true,
            vec![quote! { inset.top }, quote! { inset.right }, quote! { inset.bottom }, quote! { inset.left }],
            "Sets the top, right, bottom, and left values of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        ),
        (
            "top",
            true,
            vec![quote! { inset.top }],
            "Sets the top value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        ),
        (
            "bottom",
            true,
            vec![quote! { inset.bottom }],
            "Sets the bottom value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        ),
        (
            "left",
            true,
            vec![quote! { inset.left }],
            "Sets the left value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        ),
        (
            "right",
            true,
            vec![quote! { inset.right }],
            "Sets the right value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        ),
        (
            "gap",
            false,
            vec![quote! { gap.width }, quote! { gap.height }],
            "Sets the gap between rows and columns in flex layouts. [Docs](https://tailwindcss.com/docs/gap)"
        ),
        (
            "gap_x",
            false,
            vec![quote! { gap.width }],
            "Sets the gap between columns in flex layouts. [Docs](https://tailwindcss.com/docs/gap#changing-row-and-column-gaps-independently)"
        ),
        (
            "gap_y",
            false,
            vec![quote! { gap.height }],
            "Sets the gap between rows in flex layouts. [Docs](https://tailwindcss.com/docs/gap#changing-row-and-column-gaps-independently)"
        ),
    ]
}

/// Returns a vec of (Suffix size, tokens that correspond to this size, documentation)
fn box_suffixes() -> Vec<(&'static str, TokenStream2, &'static str)> {
    vec![
        ("0", quote! { px(0.) }, "0px"),
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
        ("7", quote! { rems(1.75) }, "28px (1.75rem)"),
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
        ("112", quote! { rems(28.) }, "448px (28rem)"),
        ("128", quote! { rems(32.) }, "512px (32rem)"),
        ("auto", quote! { auto() }, "Auto"),
        ("px", quote! { px(1.) }, "1px"),
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

fn corner_prefixes() -> Vec<(&'static str, Vec<TokenStream2>, &'static str)> {
    vec![
        (
            "rounded",
            vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.top_right },
                quote! { corner_radii.bottom_right },
                quote! { corner_radii.bottom_left },
            ],
            "Sets the border radius of the element. [Docs](https://tailwindcss.com/docs/border-radius)"
        ),
        (
            "rounded_t",
            vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.top_right },
            ],
            "Sets the border radius of the top side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)"
        ),
        (
            "rounded_b",
            vec![
                quote! { corner_radii.bottom_left },
                quote! { corner_radii.bottom_right },
            ],
            "Sets the border radius of the bottom side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)"
        ),
        (
            "rounded_r",
            vec![
                quote! { corner_radii.top_right },
                quote! { corner_radii.bottom_right },
            ],
            "Sets the border radius of the right side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)"
        ),
        (
            "rounded_l",
            vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.bottom_left },
            ],
            "Sets the border radius of the left side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)"
        ),
        (
            "rounded_tl",
            vec![quote! { corner_radii.top_left }],
            "Sets the border radius of the top left corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)"
        ),
        (
            "rounded_tr",
            vec![quote! { corner_radii.top_right }],
            "Sets the border radius of the top right corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)"
        ),
        (
            "rounded_bl",
            vec![quote! { corner_radii.bottom_left }],
            "Sets the border radius of the bottom left corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)"
        ),
        (
            "rounded_br",
            vec![quote! { corner_radii.bottom_right }],
            "Sets the border radius of the bottom right corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)"
        ),
    ]
}

fn corner_suffixes() -> Vec<(&'static str, TokenStream2, &'static str)> {
    vec![
        ("none", quote! { px(0.) }, "0px"),
        ("sm", quote! { rems(0.125) }, "2px (0.125rem)"),
        ("md", quote! { rems(0.25) }, "4px (0.25rem)"),
        ("lg", quote! { rems(0.5) }, "8px (0.5rem)"),
        ("xl", quote! { rems(0.75) }, "12px (0.75rem)"),
        ("2xl", quote! { rems(1.) }, "16px (1rem)"),
        ("3xl", quote! { rems(1.5) }, "24px (1.5rem)"),
        ("full", quote! {  px(9999.) }, "9999px"),
    ]
}

fn border_prefixes() -> Vec<(&'static str, Vec<TokenStream2>, &'static str)> {
    vec![
        (
            "border",
            vec![
                quote! { border_widths.top },
                quote! { border_widths.right },
                quote! { border_widths.bottom },
                quote! { border_widths.left },
            ],
            "Sets the border width of the element. [Docs](https://tailwindcss.com/docs/border-width)"
        ),
        (
            "border_t",
            vec![quote! { border_widths.top }],
            "Sets the border width of the top side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        ),
        (
            "border_b",
            vec![quote! { border_widths.bottom }],
            "Sets the border width of the bottom side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        ),
        (
            "border_r",
            vec![quote! { border_widths.right }],
            "Sets the border width of the right side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        ),
        (
            "border_l",
            vec![quote! { border_widths.left }],
            "Sets the border width of the left side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        ),
        (
            "border_x",
            vec![
                quote! { border_widths.left },
                quote! { border_widths.right },
            ],
            "Sets the border width of the vertical sides of the element. [Docs](https://tailwindcss.com/docs/border-width#horizontal-and-vertical-sides)"
        ),
        (
            "border_y",
            vec![
                quote! { border_widths.top },
                quote! { border_widths.bottom },
            ],
            "Sets the border width of the horizontal sides of the element. [Docs](https://tailwindcss.com/docs/border-width#horizontal-and-vertical-sides)"
        ),
    ]
}

fn border_suffixes() -> Vec<(&'static str, TokenStream2, &'static str)> {
    vec![
        ("", quote! { px(1.)}, "1px"),
        ("0", quote! { px(0.)}, "0px"),
        ("1", quote! { px(1.) }, "1px"),
        ("2", quote! { px(2.) }, "2px"),
        ("3", quote! { px(3.) }, "3px"),
        ("4", quote! { px(4.) }, "4px"),
        ("5", quote! { px(5.) }, "5px"),
        ("6", quote! { px(6.) }, "6px"),
        ("7", quote! { px(7.) }, "7px"),
        ("8", quote! { px(8.) }, "8px"),
        ("9", quote! { px(9.) }, "9px"),
        ("10", quote! { px(10.) }, "10px"),
        ("11", quote! { px(11.) }, "11px"),
        ("12", quote! { px(12.) }, "12px"),
        ("16", quote! { px(16.) }, "16px"),
        ("20", quote! { px(20.) }, "20px"),
        ("24", quote! { px(24.) }, "24px"),
        ("32", quote! { px(32.) }, "32px"),
    ]
}
