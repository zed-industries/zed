use gpui::{transparent_black, AnyElement, Fill, Interactivity, Rgba, Stateful, WindowAppearance};
use smallvec::SmallVec;

use crate::prelude::*;

pub enum PlatformStyle {
    Linux,
    Windows,
    MacOs,
}

pub fn titlebar_height(cx: &mut WindowContext) -> Pixels {
    (1.75 * cx.rem_size()).max(px(32.))
}

impl PlatformStyle {
    pub fn platform() -> Self {
        if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "macos") {
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
    background: Fill,
    content: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,
}

impl PlatformTitlebar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            platform: PlatformStyle::platform(),
            background: transparent_black().into(),
            content: div().id(id.into()),
            children: SmallVec::new(),
        }
    }

    /// Sets the platform style.
    pub fn platform_style(mut self, style: PlatformStyle) -> Self {
        self.platform = style;
        self
    }

    /// Sets the background color of the titlebar.
    pub fn background<F>(mut self, fill: F) -> Self
    where
        F: Into<Fill>,
        Self: Sized,
    {
        self.background = fill.into();
        self
    }

    fn top_padding(&self, cx: &WindowContext) -> Pixels {
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
        if !self.platform.windows() {
            return div().id("caption-buttons-windows");
        }

        let button_height = titlebar_height(cx) - self.top_padding(cx);
        let close_button_hover_color = Rgba {
            r: 232.0 / 255.0,
            g: 17.0 / 255.0,
            b: 32.0 / 255.0,
            a: 1.0,
        };

        let button_hover_color = match cx.appearance() {
            WindowAppearance::Light | WindowAppearance::VibrantLight => Rgba {
                r: 0.1,
                g: 0.1,
                b: 0.1,
                a: 0.2,
            },
            WindowAppearance::Dark | WindowAppearance::VibrantDark => Rgba {
                r: 0.9,
                g: 0.9,
                b: 0.9,
                a: 0.1,
            },
        };

        fn windows_caption_button(
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

        const MINIMIZE_ICON: &str = "\u{e921}";
        const RESTORE_ICON: &str = "\u{e923}";
        const MAXIMIZE_ICON: &str = "\u{e922}";
        const CLOSE_ICON: &str = "\u{e8bb}";

        div()
            .id("caption-buttons-windows")
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(button_height)
            .min_h(button_height)
            .font("Segoe Fluent Icons")
            .text_size(px(10.0))
            .children(vec![
                windows_caption_button("minimize", MINIMIZE_ICON, button_hover_color, cx),
                windows_caption_button(
                    "maximize",
                    if cx.is_maximized() {
                        RESTORE_ICON
                    } else {
                        MAXIMIZE_ICON
                    },
                    button_hover_color,
                    cx,
                ),
                windows_caption_button("close", CLOSE_ICON, close_button_hover_color, cx),
            ])
    }
}

impl RenderOnce for PlatformTitlebar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let titlebar_height = titlebar_height(cx);
        let titlebar_top_padding = self.top_padding(cx);
        let window_controls_right = self.render_window_controls_right(cx);

        h_flex()
            .id("titlebar")
            .w_full()
            .pt(titlebar_top_padding)
            .h(titlebar_height)
            .map(|this| {
                if cx.is_fullscreen() {
                    this.pl_2()
                } else if self.platform.macos() {
                    // Use pixels here instead of a rem-based size because the macOS traffic
                    // lights are a static size, and don't scale with the rest of the UI.
                    this.pl(px(80.))
                } else {
                    this.pl_2()
                }
            })
            .bg(self.background)
            .content_stretch()
            .child(
                self.content
                    .id("titlebar-content")
                    .flex()
                    .flex_row()
                    .justify_between()
                    .w_full()
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
