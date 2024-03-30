use gpui::{prelude::*, Rgba, WindowAppearance};

use crate::prelude::*;

#[derive(IntoElement)]
pub struct WindowsWindowControls {
    button_height: Pixels,
}

impl WindowsWindowControls {
    pub fn new(button_height: Pixels) -> Self {
        Self { button_height }
    }
}

impl RenderOnce for WindowsWindowControls {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
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

        div()
            .id("windows-window-controls")
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(WindowsCaptionButton::new(
                "minimize",
                WindowsCaptionButtonIcon::Minimize,
                button_hover_color,
            ))
            .child(WindowsCaptionButton::new(
                "maximize-or-restore",
                if cx.is_maximized() {
                    WindowsCaptionButtonIcon::Restore
                } else {
                    WindowsCaptionButtonIcon::Maximize
                },
                button_hover_color,
            ))
            .child(WindowsCaptionButton::new(
                "close",
                WindowsCaptionButtonIcon::Close,
                close_button_hover_color,
            ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum WindowsCaptionButtonIcon {
    Minimize,
    Restore,
    Maximize,
    Close,
}

#[derive(IntoElement)]
struct WindowsCaptionButton {
    id: ElementId,
    icon: WindowsCaptionButtonIcon,
    hover_background_color: Rgba,
}

impl WindowsCaptionButton {
    pub fn new(
        id: impl Into<ElementId>,
        icon: WindowsCaptionButtonIcon,
        hover_background_color: Rgba,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            hover_background_color,
        }
    }
}

impl RenderOnce for WindowsCaptionButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        // todo(windows) report this width to the Windows platform API
        // NOTE: this is intentionally hard coded. An option to use the 'native' size
        //       could be added when the width is reported to the Windows platform API
        //       as this could change between future Windows versions.
        let width = px(36.);

        h_flex()
            .id(self.id)
            .justify_center()
            .content_center()
            .w(width)
            .h_full()
            .font("Segoe Fluent Icons")
            .text_size(px(10.0))
            .hover(|style| style.bg(self.hover_background_color))
            .active(|style| {
                let mut active_color = self.hover_background_color;
                active_color.a *= 0.2;

                style.bg(active_color)
            })
            .child(match self.icon {
                WindowsCaptionButtonIcon::Minimize => "\u{e921}",
                WindowsCaptionButtonIcon::Restore => "\u{e923}",
                WindowsCaptionButtonIcon::Maximize => "\u{e922}",
                WindowsCaptionButtonIcon::Close => "\u{e8bb}",
            })
    }
}
