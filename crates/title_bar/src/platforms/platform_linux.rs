use gpui::{prelude::*, Action, Rgba, WindowAppearance};

use ui::prelude::*;

use crate::window_controls::WindowControlType;

#[derive(IntoElement)]
pub struct LinuxWindowControls {
    button_height: Pixels,
    close_window_action: Box<dyn Action>,
}

impl LinuxWindowControls {
    pub fn new(button_height: Pixels, close_window_action: Box<dyn Action>) -> Self {
        Self {
            button_height,
            close_window_action,
        }
    }
}

impl RenderOnce for LinuxWindowControls {
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
            .id("linux-window-controls")
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(TitlebarButton::new(
                "minimize",
                WindowControlType::Minimize,
                button_hover_color,
                self.close_window_action.boxed_clone(),
            ))
            .child(TitlebarButton::new(
                "maximize-or-restore",
                if cx.is_maximized() {
                    WindowControlType::Restore
                } else {
                    WindowControlType::Maximize
                },
                button_hover_color,
                self.close_window_action.boxed_clone(),
            ))
            .child(TitlebarButton::new(
                "close",
                WindowControlType::Close,
                close_button_hover_color,
                self.close_window_action,
            ))
    }
}

#[derive(IntoElement)]
struct TitlebarButton {
    id: ElementId,
    icon: WindowControlType,
    hover_background_color: Rgba,
    close_window_action: Box<dyn Action>,
}

impl TitlebarButton {
    pub fn new(
        id: impl Into<ElementId>,
        icon: WindowControlType,
        hover_background_color: Rgba,
        close_window_action: Box<dyn Action>,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            hover_background_color,
            close_window_action,
        }
    }
}

impl RenderOnce for TitlebarButton {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let width = px(36.);

        h_flex()
            .id(self.id)
            .justify_center()
            .content_center()
            .w(width)
            .h_full()
            .hover(|style| style.bg(self.hover_background_color))
            .active(|style| {
                let mut active_color = self.hover_background_color;
                active_color.a *= 0.2;

                style.bg(active_color)
            })
            .child(Icon::new(self.icon.icon()))
            .on_mouse_move(|_, cx| cx.stop_propagation())
            .on_click(move |_, cx| {
                cx.stop_propagation();
                match self.icon {
                    WindowControlType::Minimize => cx.minimize_window(),
                    WindowControlType::Restore => cx.zoom_window(),
                    WindowControlType::Maximize => cx.zoom_window(),
                    WindowControlType::Close => {
                        cx.dispatch_action(self.close_window_action.boxed_clone())
                    }
                }
            })
    }
}
