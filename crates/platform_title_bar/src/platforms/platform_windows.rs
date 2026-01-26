use gpui::{Hsla, Rgba, WindowControlArea, prelude::*};

use ui::prelude::*;

#[derive(IntoElement)]
pub struct WindowsWindowControls {
    button_height: Pixels,
}

impl WindowsWindowControls {
    pub fn new(button_height: Pixels) -> Self {
        Self { button_height }
    }

    #[cfg(not(target_os = "windows"))]
    fn get_font() -> &'static str {
        "Segoe Fluent Icons"
    }

    #[cfg(target_os = "windows")]
    fn get_font() -> &'static str {
        use windows::Wdk::System::SystemServices::RtlGetVersion;

        let mut version = unsafe { std::mem::zeroed() };
        let status = unsafe { RtlGetVersion(&mut version) };

        if status.is_ok() && version.dwBuildNumber >= 22000 {
            "Segoe Fluent Icons"
        } else {
            "Segoe MDL2 Assets"
        }
    }
}

impl RenderOnce for WindowsWindowControls {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        div()
            .id("windows-window-controls")
            .font_family(Self::get_font())
            .flex()
            .flex_row()
            .justify_center()
            .content_stretch()
            .max_h(self.button_height)
            .min_h(self.button_height)
            .child(WindowsCaptionButton::Minimize)
            .map(|this| {
                this.child(if window.is_maximized() {
                    WindowsCaptionButton::Restore
                } else {
                    WindowsCaptionButton::Maximize
                })
            })
            .child(WindowsCaptionButton::Close)
    }
}

#[derive(IntoElement)]
enum WindowsCaptionButton {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl WindowsCaptionButton {
    #[inline]
    fn id(&self) -> &'static str {
        match self {
            Self::Minimize => "minimize",
            Self::Restore => "restore",
            Self::Maximize => "maximize",
            Self::Close => "close",
        }
    }

    #[inline]
    fn icon(&self) -> &'static str {
        match self {
            Self::Minimize => "\u{e921}",
            Self::Restore => "\u{e923}",
            Self::Maximize => "\u{e922}",
            Self::Close => "\u{e8bb}",
        }
    }

    #[inline]
    fn control_area(&self) -> WindowControlArea {
        match self {
            Self::Close => WindowControlArea::Close,
            Self::Maximize | Self::Restore => WindowControlArea::Max,
            Self::Minimize => WindowControlArea::Min,
        }
    }
}

impl RenderOnce for WindowsCaptionButton {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        let (hover_bg, hover_fg, active_bg, active_fg) = match self {
            Self::Close => {
                let color: Hsla = Rgba {
                    r: 232.0 / 255.0,
                    g: 17.0 / 255.0,
                    b: 32.0 / 255.0,
                    a: 1.0,
                }
                .into();

                (
                    color,
                    gpui::white(),
                    color.opacity(0.8),
                    gpui::white().opacity(0.8),
                )
            }
            _ => (
                cx.theme().colors().ghost_element_hover,
                cx.theme().colors().text,
                cx.theme().colors().ghost_element_active,
                cx.theme().colors().text,
            ),
        };

        h_flex()
            .id(self.id())
            .justify_center()
            .content_center()
            .occlude()
            .w(px(36.))
            .h_full()
            .text_size(px(10.0))
            .hover(|style| style.bg(hover_bg).text_color(hover_fg))
            .active(|style| style.bg(active_bg).text_color(active_fg))
            .window_control_area(self.control_area())
            .child(self.icon())
    }
}
