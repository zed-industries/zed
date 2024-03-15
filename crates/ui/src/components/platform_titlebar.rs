// allowing due to multiple platform conditional code
#![allow(unused_imports)]

use gpui::{
    div,
    prelude::FluentBuilder,
    px, AnyElement, Div, Element, ElementId, Fill, InteractiveElement, Interactivity, IntoElement,
    ParentElement, Pixels, RenderOnce, Rgba, Stateful, StatefulInteractiveElement, StyleRefinement,
    Styled,
    WindowAppearance::{Dark, Light, VibrantDark, VibrantLight},
    WindowContext,
};
use smallvec::SmallVec;

use crate::h_flex;

pub enum PlatformStyle {
    Linux,
    Windows,
    MacOs,
}

impl PlatformStyle {
    pub fn platform() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else if cfg!(macos) {
            Self::MacOs
        } else {
            Self::Linux
        }
    }

    pub fn windows(&self) -> bool {
        matches!(self, Self::Windows)
    }

    pub fn macos(&self) -> bool {
        matches!(self, Self::MacOs)
    }
}

#[derive(IntoElement)]
pub struct PlatformTitlebar {
    platform: PlatformStyle,
    titlebar_bg: Option<Fill>,
    content: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Styled for PlatformTitlebar {
    fn style(&mut self) -> &mut StyleRefinement {
        self.content.style()
    }
}

impl PlatformTitlebar {
    /// Change the platform style used
    pub fn with_platform_style(self, style: PlatformStyle) -> Self {
        Self {
            platform: style,
            ..self
        }
    }

    fn titlebar_top_padding(&self, cx: &WindowContext) -> Pixels {
        if self.platform.windows() && cx.is_maximized() {
            // todo(windows): get padding from win32 api, need HWND from window context somehow
            // should be GetSystemMetricsForDpi(SM_CXPADDEDBORDER, dpi) * 2
            px(8.0)
        } else {
            px(0.0)
        }
    }

    fn windows_caption_button_width(_cx: &WindowContext) -> Pixels {
        // todo(windows): get padding from win32 api, need HWND from window context somehow
        // should be GetSystemMetricsForDpi(SM_CXSIZE, dpi)
        px(36.0)
    }

    fn render_window_controls_right(&self, cx: &mut WindowContext) -> impl Element {
        if self.platform.windows() {
            let btn_height = cx.titlebar_height() - self.titlebar_top_padding(cx);
            let close_btn_hover_color = Rgba {
                r: 232.0 / 255.0,
                g: 17.0 / 255.0,
                b: 32.0 / 255.0,
                a: 1.0,
            };

            let btn_hover_color = match cx.appearance() {
                Light | VibrantLight => Rgba {
                    r: 0.1,
                    g: 0.1,
                    b: 0.1,
                    a: 0.2,
                },
                Dark | VibrantDark => Rgba {
                    r: 0.9,
                    g: 0.9,
                    b: 0.9,
                    a: 0.1,
                },
            };

            fn windows_caption_btn(
                id: &'static str,
                icon_text: &'static str,
                hover_color: Rgba,
                cx: &WindowContext,
            ) -> Stateful<Div> {
                let mut active_color = hover_color;
                active_color.a *= 0.2;
                h_flex()
                    .id(id)
                    .h_full()
                    .justify_center()
                    .content_center()
                    .items_center()
                    .w(PlatformTitlebar::windows_caption_button_width(cx))
                    .hover(|style| style.bg(hover_color))
                    .active(|style| style.bg(active_color))
                    .child(icon_text)
            }

            div()
                .id("caption-buttons-windows")
                .flex()
                .flex_row()
                .justify_center()
                .content_stretch()
                .max_h(btn_height)
                .min_h(btn_height)
                .font("Segoe Fluent Icons")
                .text_size(gpui::Pixels(10.0))
                .children(vec![
                    windows_caption_btn("minimize", "\u{e921}", btn_hover_color, cx), // minimize icon
                    windows_caption_btn(
                        "maximize",
                        if cx.is_maximized() {
                            "\u{e923}" // restore icon
                        } else {
                            "\u{e922}" // maximize icon
                        },
                        btn_hover_color,
                        cx,
                    ),
                    windows_caption_btn("close", "\u{e8bb}", close_btn_hover_color, cx), // close icon
                ])
        } else {
            div().id("caption-buttons-windows")
        }
    }

    /// Sets the background color of titlebar.
    pub fn titlebar_bg<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
        Self: Sized,
    {
        self.titlebar_bg = Some(fill.into());
        self
    }
}

pub fn platform_titlebar(id: impl Into<ElementId>) -> PlatformTitlebar {
    let id = id.into();
    PlatformTitlebar {
        platform: PlatformStyle::platform(),
        titlebar_bg: None,
        content: div().id(id.clone()),
        children: SmallVec::new(),
    }
}

impl RenderOnce for PlatformTitlebar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let titlebar_height = cx.titlebar_height();
        let titlebar_top_padding = self.titlebar_top_padding(cx);
        let window_controls_right = self.render_window_controls_right(cx);
        let macos = self.platform.macos();
        h_flex()
            .id("titlebar")
            .w_full()
            .pt(titlebar_top_padding)
            .max_h(titlebar_height)
            .min_h(titlebar_height)
            .map(|mut this| {
                this.style().background = self.titlebar_bg;

                if macos {
                    if !cx.is_fullscreen() {
                        // Use pixels here instead of a rem-based size because the macOS traffic
                        // lights are a static size, and don't scale with the rest of the UI.
                        return this.pl(px(80.));
                    }
                }

                this
            })
            .content_stretch()
            .child(
                self.content
                    .flex()
                    .flex_row()
                    .w_full()
                    .id("titlebar-content")
                    .children(self.children),
            )
            .child(window_controls_right)
    }
}

impl InteractiveElement for PlatformTitlebar {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.content.interactivity()
    }
}
impl StatefulInteractiveElement for PlatformTitlebar {}

impl ParentElement for PlatformTitlebar {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
