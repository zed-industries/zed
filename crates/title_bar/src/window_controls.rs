use gpui::{Action, Hsla, svg};
use ui::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum WindowControlType {
    Minimize,
    Restore,
    Maximize,
    Close,
}

impl WindowControlType {
    /// Returns the icon name for the window control type.
    ///
    /// Will take a [PlatformStyle] in the future to return a different
    /// icon name based on the platform.
    pub fn icon(&self) -> IconName {
        match self {
            WindowControlType::Minimize => IconName::GenericMinimize,
            WindowControlType::Restore => IconName::GenericRestore,
            WindowControlType::Maximize => IconName::GenericMaximize,
            WindowControlType::Close => IconName::GenericClose,
        }
    }
}

#[allow(unused)]
pub struct WindowControlStyle {
    background: Hsla,
    background_hover: Hsla,
    icon: Hsla,
    icon_hover: Hsla,
}

impl WindowControlStyle {
    pub fn default(cx: &mut App) -> Self {
        let colors = cx.theme().colors();

        Self {
            background: colors.ghost_element_background,
            background_hover: colors.ghost_element_hover,
            icon: colors.icon,
            icon_hover: colors.icon_muted,
        }
    }

    #[allow(unused)]
    /// Sets the background color of the control.
    pub fn background(mut self, color: impl Into<Hsla>) -> Self {
        self.background = color.into();
        self
    }

    #[allow(unused)]
    /// Sets the background color of the control when hovered.
    pub fn background_hover(mut self, color: impl Into<Hsla>) -> Self {
        self.background_hover = color.into();
        self
    }

    #[allow(unused)]
    /// Sets the color of the icon.
    pub fn icon(mut self, color: impl Into<Hsla>) -> Self {
        self.icon = color.into();
        self
    }

    #[allow(unused)]
    /// Sets the color of the icon when hovered.
    pub fn icon_hover(mut self, color: impl Into<Hsla>) -> Self {
        self.icon_hover = color.into();
        self
    }
}

#[derive(IntoElement)]
pub struct WindowControl {
    id: ElementId,
    icon: WindowControlType,
    style: WindowControlStyle,
    close_action: Option<Box<dyn Action>>,
}

impl WindowControl {
    pub fn new(id: impl Into<ElementId>, icon: WindowControlType, cx: &mut App) -> Self {
        let style = WindowControlStyle::default(cx);

        Self {
            id: id.into(),
            icon,
            style,
            close_action: None,
        }
    }

    pub fn new_close(
        id: impl Into<ElementId>,
        icon: WindowControlType,
        close_action: Box<dyn Action>,
        cx: &mut App,
    ) -> Self {
        let style = WindowControlStyle::default(cx);

        Self {
            id: id.into(),
            icon,
            style,
            close_action: Some(close_action.boxed_clone()),
        }
    }

    #[allow(unused)]
    pub fn custom_style(
        id: impl Into<ElementId>,
        icon: WindowControlType,
        style: WindowControlStyle,
    ) -> Self {
        Self {
            id: id.into(),
            icon,
            style,
            close_action: None,
        }
    }
}

impl RenderOnce for WindowControl {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let icon = svg()
            .size_4()
            .flex_none()
            .path(self.icon.icon().path())
            .text_color(self.style.icon)
            .group_hover("", |this| this.text_color(self.style.icon_hover));

        h_flex()
            .id(self.id)
            .group("")
            .cursor_pointer()
            .justify_center()
            .content_center()
            .rounded_2xl()
            .w_5()
            .h_5()
            .hover(|this| this.bg(self.style.background_hover))
            .active(|this| this.bg(self.style.background_hover))
            .child(icon)
            .on_mouse_move(|_, _, cx| cx.stop_propagation())
            .on_click(move |_, window, cx| {
                cx.stop_propagation();
                match self.icon {
                    WindowControlType::Minimize => window.minimize_window(),
                    WindowControlType::Restore => window.zoom_window(),
                    WindowControlType::Maximize => window.zoom_window(),
                    WindowControlType::Close => window.dispatch_action(
                        self.close_action
                            .as_ref()
                            .expect("Use WindowControl::new_close() for close control.")
                            .boxed_clone(),
                        cx,
                    ),
                }
            })
    }
}
