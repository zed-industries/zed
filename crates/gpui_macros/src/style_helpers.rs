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

    for box_style_prefix in box_prefixes() {
        methods.push(generate_custom_value_setter(
            box_style_prefix.prefix,
            if box_style_prefix.auto_allowed {
                quote! { Length }
            } else {
                quote! { DefiniteLength }
            },
            &box_style_prefix.fields,
            &box_style_prefix.doc_string_prefix,
        ));

        for box_style_suffix in box_suffixes() {
            if box_style_suffix.suffix != "auto" || box_style_prefix.auto_allowed {
                methods.push(generate_predefined_setter(
                    box_style_prefix.prefix,
                    box_style_suffix.suffix,
                    &box_style_prefix.fields,
                    &box_style_suffix.length_tokens,
                    false,
                    &format!(
                        "{prefix}\n\n{suffix}",
                        prefix = box_style_prefix.doc_string_prefix,
                        suffix = box_style_suffix.doc_string_suffix,
                    ),
                ));
            }

            if box_style_suffix.suffix != "auto" {
                methods.push(generate_predefined_setter(
                    box_style_prefix.prefix,
                    box_style_suffix.suffix,
                    &box_style_prefix.fields,
                    &box_style_suffix.length_tokens,
                    true,
                    &format!(
                        "{prefix}\n\n{suffix}",
                        prefix = box_style_prefix.doc_string_prefix,
                        suffix = box_style_suffix.doc_string_suffix,
                    ),
                ));
            }
        }
    }

    for corner_style_prefix in corner_prefixes() {
        methods.push(generate_custom_value_setter(
            corner_style_prefix.prefix,
            quote! { AbsoluteLength },
            &corner_style_prefix.fields,
            corner_style_prefix.doc_string_prefix,
        ));

        for corner_style_suffix in corner_suffixes() {
            methods.push(generate_predefined_setter(
                corner_style_prefix.prefix,
                corner_style_suffix.suffix,
                &corner_style_prefix.fields,
                &corner_style_suffix.radius_tokens,
                false,
                &format!(
                    "{prefix}\n\n{suffix}",
                    prefix = corner_style_prefix.doc_string_prefix,
                    suffix = corner_style_suffix.doc_string_suffix,
                ),
            ));
        }
    }

    for border_style_prefix in border_prefixes() {
        methods.push(generate_custom_value_setter(
            border_style_prefix.prefix,
            quote! { AbsoluteLength },
            &border_style_prefix.fields,
            border_style_prefix.doc_string_prefix,
        ));

        for border_style_suffix in border_suffixes() {
            methods.push(generate_predefined_setter(
                border_style_prefix.prefix,
                border_style_suffix.suffix,
                &border_style_prefix.fields,
                &border_style_suffix.width_tokens,
                false,
                &format!(
                    "{prefix}\n\n{suffix}",
                    prefix = border_style_prefix.doc_string_prefix,
                    suffix = border_style_suffix.doc_string_suffix,
                ),
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
    let (negation_qualifier, negation_token) = if negate {
        ("_neg", quote! { - })
    } else {
        ("", quote! {})
    };

    let method_name = if length.is_empty() {
        format_ident!("{name}{negation_qualifier}")
    } else {
        format_ident!("{name}{negation_qualifier}_{length}")
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

struct BoxStylePrefix {
    prefix: &'static str,
    auto_allowed: bool,
    fields: Vec<TokenStream2>,
    doc_string_prefix: &'static str,
}

fn box_prefixes() -> Vec<BoxStylePrefix> {
    vec![
        BoxStylePrefix {
            prefix: "w",
            auto_allowed: true,
            fields: vec![quote! { size.width }],
            doc_string_prefix: "Sets the width of the element. [Docs](https://tailwindcss.com/docs/width)",
        },
        BoxStylePrefix {
            prefix: "h",
            auto_allowed: true,
            fields: vec![quote! { size.height }],
            doc_string_prefix: "Sets the height of the element. [Docs](https://tailwindcss.com/docs/height)",
        },
        BoxStylePrefix {
            prefix: "size",
            auto_allowed: true,
            fields: vec![quote! {size.width}, quote! {size.height}],
            doc_string_prefix: "Sets the width and height of the element.",
        },
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        BoxStylePrefix {
            prefix: "min_w",
            auto_allowed: true,
            fields: vec![quote! { min_size.width }],
            doc_string_prefix: "Sets the minimum width of the element. [Docs](https://tailwindcss.com/docs/min-width)",
        },
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        BoxStylePrefix {
            prefix: "min_h",
            auto_allowed: true,
            fields: vec![quote! { min_size.height }],
            doc_string_prefix: "Sets the minimum height of the element. [Docs](https://tailwindcss.com/docs/min-height)",
        },
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        BoxStylePrefix {
            prefix: "max_w",
            auto_allowed: true,
            fields: vec![quote! { max_size.width }],
            doc_string_prefix: "Sets the maximum width of the element. [Docs](https://tailwindcss.com/docs/max-width)",
        },
        // TODO: These don't use the same size ramp as the others
        // see https://tailwindcss.com/docs/max-width
        BoxStylePrefix {
            prefix: "max_h",
            auto_allowed: true,
            fields: vec![quote! { max_size.height }],
            doc_string_prefix: "Sets the maximum height of the element. [Docs](https://tailwindcss.com/docs/max-height)",
        },
        BoxStylePrefix {
            prefix: "m",
            auto_allowed: true,
            fields: vec![
                quote! { margin.top },
                quote! { margin.bottom },
                quote! { margin.left },
                quote! { margin.right },
            ],
            doc_string_prefix: "Sets the margin of the element. [Docs](https://tailwindcss.com/docs/margin)",
        },
        BoxStylePrefix {
            prefix: "mt",
            auto_allowed: true,
            fields: vec![quote! { margin.top }],
            doc_string_prefix: "Sets the top margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "mb",
            auto_allowed: true,
            fields: vec![quote! { margin.bottom }],
            doc_string_prefix: "Sets the bottom margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "my",
            auto_allowed: true,
            fields: vec![quote! { margin.top }, quote! { margin.bottom }],
            doc_string_prefix: "Sets the vertical margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-vertical-margin)",
        },
        BoxStylePrefix {
            prefix: "mx",
            auto_allowed: true,
            fields: vec![quote! { margin.left }, quote! { margin.right }],
            doc_string_prefix: "Sets the horizontal margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-horizontal-margin)",
        },
        BoxStylePrefix {
            prefix: "ml",
            auto_allowed: true,
            fields: vec![quote! { margin.left }],
            doc_string_prefix: "Sets the left margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "mr",
            auto_allowed: true,
            fields: vec![quote! { margin.right }],
            doc_string_prefix: "Sets the right margin of the element. [Docs](https://tailwindcss.com/docs/margin#add-margin-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "p",
            auto_allowed: false,
            fields: vec![
                quote! { padding.top },
                quote! { padding.bottom },
                quote! { padding.left },
                quote! { padding.right },
            ],
            doc_string_prefix: "Sets the padding of the element. [Docs](https://tailwindcss.com/docs/padding)",
        },
        BoxStylePrefix {
            prefix: "pt",
            auto_allowed: false,
            fields: vec![quote! { padding.top }],
            doc_string_prefix: "Sets the top padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "pb",
            auto_allowed: false,
            fields: vec![quote! { padding.bottom }],
            doc_string_prefix: "Sets the bottom padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "px",
            auto_allowed: false,
            fields: vec![quote! { padding.left }, quote! { padding.right }],
            doc_string_prefix: "Sets the horizontal padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-horizontal-padding)",
        },
        BoxStylePrefix {
            prefix: "py",
            auto_allowed: false,
            fields: vec![quote! { padding.top }, quote! { padding.bottom }],
            doc_string_prefix: "Sets the vertical padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-vertical-padding)",
        },
        BoxStylePrefix {
            prefix: "pl",
            auto_allowed: false,
            fields: vec![quote! { padding.left }],
            doc_string_prefix: "Sets the left padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "pr",
            auto_allowed: false,
            fields: vec![quote! { padding.right }],
            doc_string_prefix: "Sets the right padding of the element. [Docs](https://tailwindcss.com/docs/padding#add-padding-to-a-single-side)",
        },
        BoxStylePrefix {
            prefix: "inset",
            auto_allowed: true,
            fields: vec![
                quote! { inset.top },
                quote! { inset.right },
                quote! { inset.bottom },
                quote! { inset.left },
            ],
            doc_string_prefix: "Sets the top, right, bottom, and left values of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        },
        BoxStylePrefix {
            prefix: "top",
            auto_allowed: true,
            fields: vec![quote! { inset.top }],
            doc_string_prefix: "Sets the top value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        },
        BoxStylePrefix {
            prefix: "bottom",
            auto_allowed: true,
            fields: vec![quote! { inset.bottom }],
            doc_string_prefix: "Sets the bottom value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        },
        BoxStylePrefix {
            prefix: "left",
            auto_allowed: true,
            fields: vec![quote! { inset.left }],
            doc_string_prefix: "Sets the left value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        },
        BoxStylePrefix {
            prefix: "right",
            auto_allowed: true,
            fields: vec![quote! { inset.right }],
            doc_string_prefix: "Sets the right value of a positioned element. [Docs](https://tailwindcss.com/docs/top-right-bottom-left)",
        },
        BoxStylePrefix {
            prefix: "gap",
            auto_allowed: false,
            fields: vec![quote! { gap.width }, quote! { gap.height }],
            doc_string_prefix: "Sets the gap between rows and columns in flex layouts. [Docs](https://tailwindcss.com/docs/gap)",
        },
        BoxStylePrefix {
            prefix: "gap_x",
            auto_allowed: false,
            fields: vec![quote! { gap.width }],
            doc_string_prefix: "Sets the gap between columns in flex layouts. [Docs](https://tailwindcss.com/docs/gap#changing-row-and-column-gaps-independently)",
        },
        BoxStylePrefix {
            prefix: "gap_y",
            auto_allowed: false,
            fields: vec![quote! { gap.height }],
            doc_string_prefix: "Sets the gap between rows in flex layouts. [Docs](https://tailwindcss.com/docs/gap#changing-row-and-column-gaps-independently)",
        },
    ]
}

struct BoxStyleSuffix {
    suffix: &'static str,
    length_tokens: TokenStream2,
    doc_string_suffix: &'static str,
}

fn box_suffixes() -> Vec<BoxStyleSuffix> {
    vec![
        BoxStyleSuffix {
            suffix: "0",
            length_tokens: quote! { px(0.) },
            doc_string_suffix: "0px",
        },
        BoxStyleSuffix {
            suffix: "0p5",
            length_tokens: quote! { rems(0.125) },
            doc_string_suffix: "2px (0.125rem)",
        },
        BoxStyleSuffix {
            suffix: "1",
            length_tokens: quote! { rems(0.25) },
            doc_string_suffix: "4px (0.25rem)",
        },
        BoxStyleSuffix {
            suffix: "1p5",
            length_tokens: quote! { rems(0.375) },
            doc_string_suffix: "6px (0.375rem)",
        },
        BoxStyleSuffix {
            suffix: "2",
            length_tokens: quote! { rems(0.5) },
            doc_string_suffix: "8px (0.5rem)",
        },
        BoxStyleSuffix {
            suffix: "2p5",
            length_tokens: quote! { rems(0.625) },
            doc_string_suffix: "10px (0.625rem)",
        },
        BoxStyleSuffix {
            suffix: "3",
            length_tokens: quote! { rems(0.75) },
            doc_string_suffix: "12px (0.75rem)",
        },
        BoxStyleSuffix {
            suffix: "3p5",
            length_tokens: quote! { rems(0.875) },
            doc_string_suffix: "14px (0.875rem)",
        },
        BoxStyleSuffix {
            suffix: "4",
            length_tokens: quote! { rems(1.) },
            doc_string_suffix: "16px (1rem)",
        },
        BoxStyleSuffix {
            suffix: "5",
            length_tokens: quote! { rems(1.25) },
            doc_string_suffix: "20px (1.25rem)",
        },
        BoxStyleSuffix {
            suffix: "6",
            length_tokens: quote! { rems(1.5) },
            doc_string_suffix: "24px (1.5rem)",
        },
        BoxStyleSuffix {
            suffix: "7",
            length_tokens: quote! { rems(1.75) },
            doc_string_suffix: "28px (1.75rem)",
        },
        BoxStyleSuffix {
            suffix: "8",
            length_tokens: quote! { rems(2.0) },
            doc_string_suffix: "32px (2rem)",
        },
        BoxStyleSuffix {
            suffix: "9",
            length_tokens: quote! { rems(2.25) },
            doc_string_suffix: "36px (2.25rem)",
        },
        BoxStyleSuffix {
            suffix: "10",
            length_tokens: quote! { rems(2.5) },
            doc_string_suffix: "40px (2.5rem)",
        },
        BoxStyleSuffix {
            suffix: "11",
            length_tokens: quote! { rems(2.75) },
            doc_string_suffix: "44px (2.75rem)",
        },
        BoxStyleSuffix {
            suffix: "12",
            length_tokens: quote! { rems(3.) },
            doc_string_suffix: "48px (3rem)",
        },
        BoxStyleSuffix {
            suffix: "16",
            length_tokens: quote! { rems(4.) },
            doc_string_suffix: "64px (4rem)",
        },
        BoxStyleSuffix {
            suffix: "20",
            length_tokens: quote! { rems(5.) },
            doc_string_suffix: "80px (5rem)",
        },
        BoxStyleSuffix {
            suffix: "24",
            length_tokens: quote! { rems(6.) },
            doc_string_suffix: "96px (6rem)",
        },
        BoxStyleSuffix {
            suffix: "32",
            length_tokens: quote! { rems(8.) },
            doc_string_suffix: "128px (8rem)",
        },
        BoxStyleSuffix {
            suffix: "40",
            length_tokens: quote! { rems(10.) },
            doc_string_suffix: "160px (10rem)",
        },
        BoxStyleSuffix {
            suffix: "48",
            length_tokens: quote! { rems(12.) },
            doc_string_suffix: "192px (12rem)",
        },
        BoxStyleSuffix {
            suffix: "56",
            length_tokens: quote! { rems(14.) },
            doc_string_suffix: "224px (14rem)",
        },
        BoxStyleSuffix {
            suffix: "64",
            length_tokens: quote! { rems(16.) },
            doc_string_suffix: "256px (16rem)",
        },
        BoxStyleSuffix {
            suffix: "72",
            length_tokens: quote! { rems(18.) },
            doc_string_suffix: "288px (18rem)",
        },
        BoxStyleSuffix {
            suffix: "80",
            length_tokens: quote! { rems(20.) },
            doc_string_suffix: "320px (20rem)",
        },
        BoxStyleSuffix {
            suffix: "96",
            length_tokens: quote! { rems(24.) },
            doc_string_suffix: "384px (24rem)",
        },
        BoxStyleSuffix {
            suffix: "112",
            length_tokens: quote! { rems(28.) },
            doc_string_suffix: "448px (28rem)",
        },
        BoxStyleSuffix {
            suffix: "128",
            length_tokens: quote! { rems(32.) },
            doc_string_suffix: "512px (32rem)",
        },
        BoxStyleSuffix {
            suffix: "auto",
            length_tokens: quote! { auto() },
            doc_string_suffix: "Auto",
        },
        BoxStyleSuffix {
            suffix: "px",
            length_tokens: quote! { px(1.) },
            doc_string_suffix: "1px",
        },
        BoxStyleSuffix {
            suffix: "full",
            length_tokens: quote! { relative(1.) },
            doc_string_suffix: "100%",
        },
        BoxStyleSuffix {
            suffix: "1_2",
            length_tokens: quote! { relative(0.5) },
            doc_string_suffix: "50% (1/2)",
        },
        BoxStyleSuffix {
            suffix: "1_3",
            length_tokens: quote! { relative(1./3.) },
            doc_string_suffix: "33% (1/3)",
        },
        BoxStyleSuffix {
            suffix: "2_3",
            length_tokens: quote! { relative(2./3.) },
            doc_string_suffix: "66% (2/3)",
        },
        BoxStyleSuffix {
            suffix: "1_4",
            length_tokens: quote! { relative(0.25) },
            doc_string_suffix: "25% (1/4)",
        },
        BoxStyleSuffix {
            suffix: "2_4",
            length_tokens: quote! { relative(0.5) },
            doc_string_suffix: "50% (2/4)",
        },
        BoxStyleSuffix {
            suffix: "3_4",
            length_tokens: quote! { relative(0.75) },
            doc_string_suffix: "75% (3/4)",
        },
        BoxStyleSuffix {
            suffix: "1_5",
            length_tokens: quote! { relative(0.2) },
            doc_string_suffix: "20% (1/5)",
        },
        BoxStyleSuffix {
            suffix: "2_5",
            length_tokens: quote! { relative(0.4) },
            doc_string_suffix: "40% (2/5)",
        },
        BoxStyleSuffix {
            suffix: "3_5",
            length_tokens: quote! { relative(0.6) },
            doc_string_suffix: "60% (3/5)",
        },
        BoxStyleSuffix {
            suffix: "4_5",
            length_tokens: quote! { relative(0.8) },
            doc_string_suffix: "80% (4/5)",
        },
        BoxStyleSuffix {
            suffix: "1_6",
            length_tokens: quote! { relative(1./6.) },
            doc_string_suffix: "16% (1/6)",
        },
        BoxStyleSuffix {
            suffix: "5_6",
            length_tokens: quote! { relative(5./6.) },
            doc_string_suffix: "80% (5/6)",
        },
        BoxStyleSuffix {
            suffix: "1_12",
            length_tokens: quote! { relative(1./12.) },
            doc_string_suffix: "8% (1/12)",
        },
    ]
}

struct CornerStylePrefix {
    prefix: &'static str,
    fields: Vec<TokenStream2>,
    doc_string_prefix: &'static str,
}

fn corner_prefixes() -> Vec<CornerStylePrefix> {
    vec![
        CornerStylePrefix {
            prefix: "rounded",
            fields: vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.top_right },
                quote! { corner_radii.bottom_right },
                quote! { corner_radii.bottom_left },
            ],
            doc_string_prefix: "Sets the border radius of the element. [Docs](https://tailwindcss.com/docs/border-radius)",
        },
        CornerStylePrefix {
            prefix: "rounded_t",
            fields: vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.top_right },
            ],
            doc_string_prefix: "Sets the border radius of the top side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_b",
            fields: vec![
                quote! { corner_radii.bottom_left },
                quote! { corner_radii.bottom_right },
            ],
            doc_string_prefix: "Sets the border radius of the bottom side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_r",
            fields: vec![
                quote! { corner_radii.top_right },
                quote! { corner_radii.bottom_right },
            ],
            doc_string_prefix: "Sets the border radius of the right side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_l",
            fields: vec![
                quote! { corner_radii.top_left },
                quote! { corner_radii.bottom_left },
            ],
            doc_string_prefix: "Sets the border radius of the left side of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-sides-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_tl",
            fields: vec![quote! { corner_radii.top_left }],
            doc_string_prefix: "Sets the border radius of the top left corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_tr",
            fields: vec![quote! { corner_radii.top_right }],
            doc_string_prefix: "Sets the border radius of the top right corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_bl",
            fields: vec![quote! { corner_radii.bottom_left }],
            doc_string_prefix: "Sets the border radius of the bottom left corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)",
        },
        CornerStylePrefix {
            prefix: "rounded_br",
            fields: vec![quote! { corner_radii.bottom_right }],
            doc_string_prefix: "Sets the border radius of the bottom right corner of the element. [Docs](https://tailwindcss.com/docs/border-radius#rounding-corners-separately)",
        },
    ]
}

struct CornerStyleSuffix {
    suffix: &'static str,
    radius_tokens: TokenStream2,
    doc_string_suffix: &'static str,
}

fn corner_suffixes() -> Vec<CornerStyleSuffix> {
    vec![
        CornerStyleSuffix {
            suffix: "none",
            radius_tokens: quote! { px(0.) },
            doc_string_suffix: "0px",
        },
        CornerStyleSuffix {
            suffix: "sm",
            radius_tokens: quote! { rems(0.125) },
            doc_string_suffix: "2px (0.125rem)",
        },
        CornerStyleSuffix {
            suffix: "md",
            radius_tokens: quote! { rems(0.25) },
            doc_string_suffix: "4px (0.25rem)",
        },
        CornerStyleSuffix {
            suffix: "lg",
            radius_tokens: quote! { rems(0.5) },
            doc_string_suffix: "8px (0.5rem)",
        },
        CornerStyleSuffix {
            suffix: "xl",
            radius_tokens: quote! { rems(0.75) },
            doc_string_suffix: "12px (0.75rem)",
        },
        CornerStyleSuffix {
            suffix: "2xl",
            radius_tokens: quote! { rems(1.) },
            doc_string_suffix: "16px (1rem)",
        },
        CornerStyleSuffix {
            suffix: "3xl",
            radius_tokens: quote! { rems(1.5) },
            doc_string_suffix: "24px (1.5rem)",
        },
        CornerStyleSuffix {
            suffix: "full",
            radius_tokens: quote! {  px(9999.) },
            doc_string_suffix: "9999px",
        },
    ]
}

struct BorderStylePrefix {
    prefix: &'static str,
    fields: Vec<TokenStream2>,
    doc_string_prefix: &'static str,
}

fn border_prefixes() -> Vec<BorderStylePrefix> {
    vec![
        BorderStylePrefix {
            prefix: "border",
            fields: vec![
                quote! { border_widths.top },
                quote! { border_widths.right },
                quote! { border_widths.bottom },
                quote! { border_widths.left },
            ],
            doc_string_prefix: "Sets the border width of the element. [Docs](https://tailwindcss.com/docs/border-width)"
        },
        BorderStylePrefix {
            prefix: "border_t",
            fields: vec![quote! { border_widths.top }],
            doc_string_prefix: "Sets the border width of the top side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        },
        BorderStylePrefix {
            prefix: "border_b",
            fields: vec![quote! { border_widths.bottom }],
            doc_string_prefix: "Sets the border width of the bottom side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        },
        BorderStylePrefix {
            prefix: "border_r",
            fields: vec![quote! { border_widths.right }],
            doc_string_prefix: "Sets the border width of the right side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        },
        BorderStylePrefix {
            prefix: "border_l",
            fields: vec![quote! { border_widths.left }],
            doc_string_prefix: "Sets the border width of the left side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)"
        },
        BorderStylePrefix {
            prefix: "border_x",
            fields: vec![
                quote! { border_widths.left },
                quote! { border_widths.right },
            ],
            doc_string_prefix: "Sets the border width of the vertical sides of the element. [Docs](https://tailwindcss.com/docs/border-width#horizontal-and-vertical-sides)"
        },
        BorderStylePrefix {
            prefix: "border_y",
            fields: vec![
                quote! { border_widths.top },
                quote! { border_widths.bottom },
            ],
            doc_string_prefix: "Sets the border width of the horizontal sides of the element. [Docs](https://tailwindcss.com/docs/border-width#horizontal-and-vertical-sides)"
        },
    ]
}

struct BorderStyleSuffix {
    suffix: &'static str,
    width_tokens: TokenStream2,
    doc_string_suffix: &'static str,
}

fn border_suffixes() -> Vec<BorderStyleSuffix> {
    vec![
        BorderStyleSuffix {
            suffix: "0",
            width_tokens: quote! { px(0.)},
            doc_string_suffix: "0px",
        },
        BorderStyleSuffix {
            suffix: "1",
            width_tokens: quote! { px(1.) },
            doc_string_suffix: "1px",
        },
        BorderStyleSuffix {
            suffix: "2",
            width_tokens: quote! { px(2.) },
            doc_string_suffix: "2px",
        },
        BorderStyleSuffix {
            suffix: "3",
            width_tokens: quote! { px(3.) },
            doc_string_suffix: "3px",
        },
        BorderStyleSuffix {
            suffix: "4",
            width_tokens: quote! { px(4.) },
            doc_string_suffix: "4px",
        },
        BorderStyleSuffix {
            suffix: "5",
            width_tokens: quote! { px(5.) },
            doc_string_suffix: "5px",
        },
        BorderStyleSuffix {
            suffix: "6",
            width_tokens: quote! { px(6.) },
            doc_string_suffix: "6px",
        },
        BorderStyleSuffix {
            suffix: "7",
            width_tokens: quote! { px(7.) },
            doc_string_suffix: "7px",
        },
        BorderStyleSuffix {
            suffix: "8",
            width_tokens: quote! { px(8.) },
            doc_string_suffix: "8px",
        },
        BorderStyleSuffix {
            suffix: "9",
            width_tokens: quote! { px(9.) },
            doc_string_suffix: "9px",
        },
        BorderStyleSuffix {
            suffix: "10",
            width_tokens: quote! { px(10.) },
            doc_string_suffix: "10px",
        },
        BorderStyleSuffix {
            suffix: "11",
            width_tokens: quote! { px(11.) },
            doc_string_suffix: "11px",
        },
        BorderStyleSuffix {
            suffix: "12",
            width_tokens: quote! { px(12.) },
            doc_string_suffix: "12px",
        },
        BorderStyleSuffix {
            suffix: "16",
            width_tokens: quote! { px(16.) },
            doc_string_suffix: "16px",
        },
        BorderStyleSuffix {
            suffix: "20",
            width_tokens: quote! { px(20.) },
            doc_string_suffix: "20px",
        },
        BorderStyleSuffix {
            suffix: "24",
            width_tokens: quote! { px(24.) },
            doc_string_suffix: "24px",
        },
        BorderStyleSuffix {
            suffix: "32",
            width_tokens: quote! { px(32.) },
            doc_string_suffix: "32px",
        },
    ]
}
