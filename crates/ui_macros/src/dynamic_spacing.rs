use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    LitInt, Token, parse::Parse, parse::ParseStream, parse_macro_input, punctuated::Punctuated,
};

struct DynamicSpacingInput {
    values: Punctuated<DynamicSpacingValue, Token![,]>,
}

// The input for the derive macro is a list of values.
//
// When a single value is provided, the standard spacing formula is
// used to derive the of spacing values.
//
// When a tuple of three values is provided, the values are used as
// the spacing values directly.
enum DynamicSpacingValue {
    Single(LitInt),
    Tuple(LitInt, LitInt, LitInt),
}

impl Parse for DynamicSpacingInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(DynamicSpacingInput {
            values: input.parse_terminated(DynamicSpacingValue::parse)?,
        })
    }
}

impl Parse for DynamicSpacingValue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let a: LitInt = content.parse()?;
            content.parse::<Token![,]>()?;
            let b: LitInt = content.parse()?;
            content.parse::<Token![,]>()?;
            let c: LitInt = content.parse()?;
            Ok(DynamicSpacingValue::Tuple(a, b, c))
        } else {
            Ok(DynamicSpacingValue::Single(input.parse()?))
        }
    }
}

/// Derives the spacing method for the `DynamicSpacing` enum.
pub fn derive_spacing(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DynamicSpacingInput);

    let spacing_ratios: Vec<_> = input
        .values
        .iter()
        .map(|v| {
            let variant = match v {
                DynamicSpacingValue::Single(n) => {
                    format_ident!("Base{:02}", n.base10_parse::<u32>().unwrap())
                }
                DynamicSpacingValue::Tuple(_, b, _) => {
                    format_ident!("Base{:02}", b.base10_parse::<u32>().unwrap())
                }
            };
            match v {
                DynamicSpacingValue::Single(n) => {
                    let n = n.base10_parse::<f32>().unwrap();
                    quote! {
                        DynamicSpacing::#variant => match ThemeSettings::get_global(cx).ui_density {
                            UiDensity::Compact => (#n - 4.0).max(0.0) / BASE_REM_SIZE_IN_PX,
                            UiDensity::Default => #n / BASE_REM_SIZE_IN_PX,
                            UiDensity::Comfortable => (#n + 4.0) / BASE_REM_SIZE_IN_PX,
                        }
                    }
                }
                DynamicSpacingValue::Tuple(a, b, c) => {
                    let a = a.base10_parse::<f32>().unwrap();
                    let b = b.base10_parse::<f32>().unwrap();
                    let c = c.base10_parse::<f32>().unwrap();
                    quote! {
                        DynamicSpacing::#variant => match ThemeSettings::get_global(cx).ui_density {
                            UiDensity::Compact => #a / BASE_REM_SIZE_IN_PX,
                            UiDensity::Default => #b / BASE_REM_SIZE_IN_PX,
                            UiDensity::Comfortable => #c / BASE_REM_SIZE_IN_PX,
                        }
                    }
                }
            }
        })
        .collect();

    let (variant_names, doc_strings): (Vec<_>, Vec<_>) = input
        .values
        .iter()
        .map(|v| {
            let variant = match v {
                DynamicSpacingValue::Single(n) => {
                    format_ident!("Base{:02}", n.base10_parse::<u32>().unwrap())
                }
                DynamicSpacingValue::Tuple(_, b, _) => {
                    format_ident!("Base{:02}", b.base10_parse::<u32>().unwrap())
                }
            };
            let doc_string = match v {
                DynamicSpacingValue::Single(n) => {
                    let n = n.base10_parse::<f32>().unwrap();
                    let compact = (n - 4.0).max(0.0);
                    let comfortable = n + 4.0;
                    format!(
                        "`{}px`|`{}px`|`{}px (@16px/rem)` - Scales with the user's rem size.",
                        compact, n, comfortable
                    )
                }
                DynamicSpacingValue::Tuple(a, b, c) => {
                    let a = a.base10_parse::<f32>().unwrap();
                    let b = b.base10_parse::<f32>().unwrap();
                    let c = c.base10_parse::<f32>().unwrap();
                    format!(
                        "`{}px`|`{}px`|`{}px (@16px/rem)` - Scales with the user's rem size.",
                        a, b, c
                    )
                }
            };
            (quote!(#variant), quote!(#doc_string))
        })
        .unzip();

    let expanded = quote! {
        /// A dynamic spacing system that adjusts spacing based on
        /// [UiDensity].
        ///
        /// The number following "Base" refers to the base pixel size
        /// at the default rem size and spacing settings.
        ///
        /// When possible, [DynamicSpacing] should be used over manual
        /// or built-in spacing values in places dynamic spacing is needed.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum DynamicSpacing {
            #(
                #[doc = #doc_strings]
                #variant_names,
            )*
        }

        impl DynamicSpacing {
            /// Returns the spacing ratio, should only be used internally.
            fn spacing_ratio(&self, cx: &App) -> f32 {
                const BASE_REM_SIZE_IN_PX: f32 = 16.0;
                match self {
                    #(#spacing_ratios,)*
                }
            }

            /// Returns the spacing value in rems.
            pub fn rems(&self, cx: &App) -> Rems {
                rems(self.spacing_ratio(cx))
            }

            /// Returns the spacing value in pixels.
            pub fn px(&self, cx: &App) -> Pixels {
                let ui_font_size_f32: f32 = ThemeSettings::get_global(cx).ui_font_size(cx).into();
                px(ui_font_size_f32 * self.spacing_ratio(cx))
            }
        }
    };

    TokenStream::from(expanded)
}
