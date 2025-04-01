use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Token, Visibility, braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input,
};

#[derive(Debug)]
struct StyleableMacroInput {
    method_visibility: Visibility,
}

impl Parse for StyleableMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if !input.peek(syn::token::Brace) {
            return Ok(Self {
                method_visibility: Visibility::Inherited,
            });
        }

        let content;
        braced!(content in input);

        let mut method_visibility = None;

        let ident: syn::Ident = content.parse()?;
        if ident == "visibility" {
            let _colon: Token![:] = content.parse()?;
            method_visibility = Some(content.parse()?);
        }

        Ok(Self {
            method_visibility: method_visibility.unwrap_or(Visibility::Inherited),
        })
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

pub fn visibility_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let visibility = input.method_visibility;
    let output = quote! {
        /// Sets the visibility of the element to `visible`.
        /// [Docs](https://tailwindcss.com/docs/visibility)
        #visibility fn visible(mut self) -> Self {
            self.style().visibility = Some(gpui::Visibility::Visible);
            self
        }

        /// Sets the visibility of the element to `hidden`.
        /// [Docs](https://tailwindcss.com/docs/visibility)
        #visibility fn invisible(mut self) -> Self {
            self.style().visibility = Some(gpui::Visibility::Hidden);
            self
        }
    };

    output.into()
}

pub fn margin_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let methods = generate_box_style_methods(
        margin_box_style_prefixes(),
        box_style_suffixes(),
        input.method_visibility,
    );
    let output = quote! {
        #(#methods)*
    };

    output.into()
}

pub fn padding_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let methods = generate_box_style_methods(
        padding_box_style_prefixes(),
        box_style_suffixes(),
        input.method_visibility,
    );
    let output = quote! {
        #(#methods)*
    };

    output.into()
}

pub fn position_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let visibility = input.method_visibility;
    let methods = generate_box_style_methods(
        position_box_style_prefixes(),
        box_style_suffixes(),
        visibility.clone(),
    );
    let output = quote! {
        /// Sets the position of the element to `relative`.
        /// [Docs](https://tailwindcss.com/docs/position)
        #visibility fn relative(mut self) -> Self {
            self.style().position = Some(gpui::Position::Relative);
            self
        }

        /// Sets the position of the element to `absolute`.
        /// [Docs](https://tailwindcss.com/docs/position)
        #visibility fn absolute(mut self) -> Self {
            self.style().position = Some(gpui::Position::Absolute);
            self
        }

        #(#methods)*
    };

    output.into()
}

pub fn overflow_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let visibility = input.method_visibility;
    let output = quote! {
        /// Sets the behavior of content that overflows the container to be hidden.
        /// [Docs](https://tailwindcss.com/docs/overflow#hiding-content-that-overflows)
        #visibility fn overflow_hidden(mut self) -> Self {
            self.style().overflow.x = Some(gpui::Overflow::Hidden);
            self.style().overflow.y = Some(gpui::Overflow::Hidden);
            self
        }

        /// Sets the behavior of content that overflows the container on the X axis to be hidden.
        /// [Docs](https://tailwindcss.com/docs/overflow#hiding-content-that-overflows)
        #visibility fn overflow_x_hidden(mut self) -> Self {
            self.style().overflow.x = Some(gpui::Overflow::Hidden);
            self
        }

        /// Sets the behavior of content that overflows the container on the Y axis to be hidden.
        /// [Docs](https://tailwindcss.com/docs/overflow#hiding-content-that-overflows)
        #visibility fn overflow_y_hidden(mut self) -> Self {
            self.style().overflow.y = Some(gpui::Overflow::Hidden);
            self
        }
    };

    output.into()
}

pub fn cursor_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let visibility = input.method_visibility;
    let output = quote! {
        /// Set the cursor style when hovering over this element
        #visibility fn cursor(mut self, cursor: CursorStyle) -> Self {
            self.style().mouse_cursor = Some(cursor);
            self
        }

        /// Sets the cursor style when hovering an element to `default`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_default(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::Arrow);
            self
        }

        /// Sets the cursor style when hovering an element to `pointer`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_pointer(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::PointingHand);
            self
        }

        /// Sets cursor style when hovering over an element to `text`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_text(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::IBeam);
            self
        }

        /// Sets cursor style when hovering over an element to `move`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_move(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ClosedHand);
            self
        }

        /// Sets cursor style when hovering over an element to `not-allowed`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_not_allowed(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::OperationNotAllowed);
            self
        }

        /// Sets cursor style when hovering over an element to `context-menu`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_context_menu(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ContextualMenu);
            self
        }

        /// Sets cursor style when hovering over an element to `crosshair`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_crosshair(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::Crosshair);
            self
        }

        /// Sets cursor style when hovering over an element to `vertical-text`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_vertical_text(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::IBeamCursorForVerticalLayout);
            self
        }

        /// Sets cursor style when hovering over an element to `alias`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_alias(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::DragLink);
            self
        }

        /// Sets cursor style when hovering over an element to `copy`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_copy(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::DragCopy);
            self
        }

        /// Sets cursor style when hovering over an element to `no-drop`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_no_drop(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::OperationNotAllowed);
            self
        }

        /// Sets cursor style when hovering over an element to `grab`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_grab(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::OpenHand);
            self
        }

        /// Sets cursor style when hovering over an element to `grabbing`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_grabbing(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ClosedHand);
            self
        }

        /// Sets cursor style when hovering over an element to `ew-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_ew_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeLeftRight);
            self
        }

        /// Sets cursor style when hovering over an element to `ns-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_ns_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeUpDown);
            self
        }

        /// Sets cursor style when hovering over an element to `nesw-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_nesw_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeUpRightDownLeft);
            self
        }

        /// Sets cursor style when hovering over an element to `nwse-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_nwse_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeUpLeftDownRight);
            self
        }

        /// Sets cursor style when hovering over an element to `col-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_col_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeColumn);
            self
        }

        /// Sets cursor style when hovering over an element to `row-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_row_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeRow);
            self
        }

        /// Sets cursor style when hovering over an element to `n-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_n_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeUp);
            self
        }

        /// Sets cursor style when hovering over an element to `e-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_e_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeRight);
            self
        }

        /// Sets cursor style when hovering over an element to `s-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_s_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeDown);
            self
        }

        /// Sets cursor style when hovering over an element to `w-resize`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_w_resize(mut self) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::ResizeLeft);
            self
        }

        /// Sets cursor style when hovering over an element to `none`.
        /// [Docs](https://tailwindcss.com/docs/cursor)
        #visibility fn cursor_none(mut self, cursor: CursorStyle) -> Self {
            self.style().mouse_cursor = Some(gpui::CursorStyle::None);
            self
        }
    };

    output.into()
}

pub fn border_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let visibility = input.method_visibility;

    let mut methods = Vec::new();

    for border_style_prefix in border_prefixes() {
        methods.push(generate_custom_value_setter(
            visibility.clone(),
            border_style_prefix.prefix,
            quote! { AbsoluteLength },
            &border_style_prefix.fields,
            border_style_prefix.doc_string_prefix,
        ));

        for border_style_suffix in border_suffixes() {
            methods.push(generate_predefined_setter(
                visibility.clone(),
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

    let output = quote! {
        /// Sets the border color of the element.
        #visibility fn border_color<C>(mut self, border_color: C) -> Self
        where
            C: Into<gpui::Hsla>,
            Self: Sized,
        {
            self.style().border_color = Some(border_color.into());
            self
        }

        #(#methods)*
    };

    output.into()
}

pub fn box_shadow_style_methods(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StyleableMacroInput);
    let visibility = input.method_visibility;
    let output = quote! {
        /// Sets the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow(mut self, shadows: smallvec::SmallVec<[gpui::BoxShadow; 2]>) -> Self {
            self.style().box_shadow = Some(shadows);
            self
        }

        /// Clears the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow_none(mut self) -> Self {
            self.style().box_shadow = Some(Default::default());
            self
        }

        /// Sets the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow_sm(mut self) -> Self {
            use gpui::{BoxShadow, hsla, point, px};
            use smallvec::smallvec;

            self.style().box_shadow = Some(smallvec![BoxShadow {
                color: hsla(0., 0., 0., 0.05),
                offset: point(px(0.), px(1.)),
                blur_radius: px(2.),
                spread_radius: px(0.),
            }]);
            self
        }

        /// Sets the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow_md(mut self) -> Self {
            use gpui::{BoxShadow, hsla, point, px};
            use smallvec::smallvec;

            self.style().box_shadow = Some(smallvec![
                BoxShadow {
                    color: hsla(0.5, 0., 0., 0.1),
                    offset: point(px(0.), px(4.)),
                    blur_radius: px(6.),
                    spread_radius: px(-1.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(2.)),
                    blur_radius: px(4.),
                    spread_radius: px(-2.),
                }
            ]);
            self
        }

        /// Sets the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow_lg(mut self) -> Self {
            use gpui::{BoxShadow, hsla, point, px};
            use smallvec::smallvec;

            self.style().box_shadow = Some(smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(10.)),
                    blur_radius: px(15.),
                    spread_radius: px(-3.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(4.)),
                    blur_radius: px(6.),
                    spread_radius: px(-4.),
                }
            ]);
            self
        }

        /// Sets the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow_xl(mut self) -> Self {
            use gpui::{BoxShadow, hsla, point, px};
            use smallvec::smallvec;

            self.style().box_shadow = Some(smallvec![
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(20.)),
                    blur_radius: px(25.),
                    spread_radius: px(-5.),
                },
                BoxShadow {
                    color: hsla(0., 0., 0., 0.1),
                    offset: point(px(0.), px(8.)),
                    blur_radius: px(10.),
                    spread_radius: px(-6.),
                }
            ]);
            self
        }

        /// Sets the box shadow of the element.
        /// [Docs](https://tailwindcss.com/docs/box-shadow)
        #visibility fn shadow_2xl(mut self) -> Self {
            use gpui::{BoxShadow, hsla, point, px};
            use smallvec::smallvec;

            self.style().box_shadow = Some(smallvec![BoxShadow {
                color: hsla(0., 0., 0., 0.25),
                offset: point(px(0.), px(25.)),
                blur_radius: px(50.),
                spread_radius: px(-12.),
            }]);
            self
        }
    };

    output.into()
}

struct BoxStylePrefix {
    prefix: &'static str,
    auto_allowed: bool,
    fields: Vec<TokenStream2>,
    doc_string_prefix: &'static str,
}

struct BoxStyleSuffix {
    suffix: &'static str,
    length_tokens: TokenStream2,
    doc_string_suffix: &'static str,
}

struct CornerStylePrefix {
    prefix: &'static str,
    fields: Vec<TokenStream2>,
    doc_string_prefix: &'static str,
}

struct CornerStyleSuffix {
    suffix: &'static str,
    radius_tokens: TokenStream2,
    doc_string_suffix: &'static str,
}

struct BorderStylePrefix {
    prefix: &'static str,
    fields: Vec<TokenStream2>,
    doc_string_prefix: &'static str,
}

struct BorderStyleSuffix {
    suffix: &'static str,
    width_tokens: TokenStream2,
    doc_string_suffix: &'static str,
}

fn generate_box_style_methods(
    prefixes: Vec<BoxStylePrefix>,
    suffixes: Vec<BoxStyleSuffix>,
    visibility: Visibility,
) -> Vec<TokenStream2> {
    let mut methods = Vec::new();

    for box_style_prefix in prefixes {
        methods.push(generate_custom_value_setter(
            visibility.clone(),
            box_style_prefix.prefix,
            if box_style_prefix.auto_allowed {
                quote! { Length }
            } else {
                quote! { DefiniteLength }
            },
            &box_style_prefix.fields,
            box_style_prefix.doc_string_prefix,
        ));

        for box_style_suffix in &suffixes {
            if box_style_suffix.suffix != "auto" || box_style_prefix.auto_allowed {
                methods.push(generate_predefined_setter(
                    visibility.clone(),
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
                    visibility.clone(),
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

    methods
}

fn generate_methods() -> Vec<TokenStream2> {
    let visibility = Visibility::Inherited;
    let mut methods =
        generate_box_style_methods(box_prefixes(), box_style_suffixes(), visibility.clone());

    for corner_style_prefix in corner_prefixes() {
        methods.push(generate_custom_value_setter(
            visibility.clone(),
            corner_style_prefix.prefix,
            quote! { AbsoluteLength },
            &corner_style_prefix.fields,
            corner_style_prefix.doc_string_prefix,
        ));

        for corner_style_suffix in corner_suffixes() {
            methods.push(generate_predefined_setter(
                visibility.clone(),
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

    methods
}

fn generate_predefined_setter(
    visibility: Visibility,
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
        #visibility fn #method_name(mut self) -> Self {
            let style = self.style();
            #(#field_assignments)*
            self
        }
    };

    method
}

fn generate_custom_value_setter(
    visibility: Visibility,
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
        #visibility fn #method_name(mut self, length: impl std::clone::Clone + Into<gpui::#length_type>) -> Self {
            let style = self.style();
            #(#field_assignments)*
            self
        }
    };

    method
}

fn margin_box_style_prefixes() -> Vec<BoxStylePrefix> {
    vec![
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
    ]
}

fn padding_box_style_prefixes() -> Vec<BoxStylePrefix> {
    vec![
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
    ]
}

fn position_box_style_prefixes() -> Vec<BoxStylePrefix> {
    vec![
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
    ]
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

fn box_style_suffixes() -> Vec<BoxStyleSuffix> {
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

fn corner_suffixes() -> Vec<CornerStyleSuffix> {
    vec![
        CornerStyleSuffix {
            suffix: "none",
            radius_tokens: quote! { px(0.) },
            doc_string_suffix: "0px",
        },
        CornerStyleSuffix {
            suffix: "xs",
            radius_tokens: quote! { rems(0.125) },
            doc_string_suffix: "2px (0.125rem)",
        },
        CornerStyleSuffix {
            suffix: "sm",
            radius_tokens: quote! { rems(0.25) },
            doc_string_suffix: "4px (0.25rem)",
        },
        CornerStyleSuffix {
            suffix: "md",
            radius_tokens: quote! { rems(0.375) },
            doc_string_suffix: "6px (0.375rem)",
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
            doc_string_prefix: "Sets the border width of the element. [Docs](https://tailwindcss.com/docs/border-width)",
        },
        BorderStylePrefix {
            prefix: "border_t",
            fields: vec![quote! { border_widths.top }],
            doc_string_prefix: "Sets the border width of the top side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)",
        },
        BorderStylePrefix {
            prefix: "border_b",
            fields: vec![quote! { border_widths.bottom }],
            doc_string_prefix: "Sets the border width of the bottom side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)",
        },
        BorderStylePrefix {
            prefix: "border_r",
            fields: vec![quote! { border_widths.right }],
            doc_string_prefix: "Sets the border width of the right side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)",
        },
        BorderStylePrefix {
            prefix: "border_l",
            fields: vec![quote! { border_widths.left }],
            doc_string_prefix: "Sets the border width of the left side of the element. [Docs](https://tailwindcss.com/docs/border-width#individual-sides)",
        },
        BorderStylePrefix {
            prefix: "border_x",
            fields: vec![
                quote! { border_widths.left },
                quote! { border_widths.right },
            ],
            doc_string_prefix: "Sets the border width of the vertical sides of the element. [Docs](https://tailwindcss.com/docs/border-width#horizontal-and-vertical-sides)",
        },
        BorderStylePrefix {
            prefix: "border_y",
            fields: vec![
                quote! { border_widths.top },
                quote! { border_widths.bottom },
            ],
            doc_string_prefix: "Sets the border width of the horizontal sides of the element. [Docs](https://tailwindcss.com/docs/border-width#horizontal-and-vertical-sides)",
        },
    ]
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
