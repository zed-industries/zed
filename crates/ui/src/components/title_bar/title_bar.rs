use gpui::{windows_title_bar_height, windows_title_bar_top_offset};
use gpui::{AnyElement, Interactivity, Stateful};
use smallvec::SmallVec;

use crate::components::title_bar::windows_window_controls::WindowsWindowControls;
use crate::prelude::*;

#[derive(IntoElement)]
pub struct TitleBar {
    platform_style: PlatformStyle,
    content: Stateful<Div>,
    children: SmallVec<[AnyElement; 2]>,
}

impl TitleBar {
    #[cfg(not(target_os = "windows"))]
    pub fn height(cx: &mut WindowContext) -> Pixels {
        (1.75 * cx.rem_size()).max(px(32.))
    }

    #[cfg(target_os = "windows")]
    pub fn height(cx: &mut WindowContext) -> Pixels {
        windows_title_bar_height(cx)
    }

    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            platform_style: PlatformStyle::platform(),
            content: div().id(id.into()),
            children: SmallVec::new(),
        }
    }

    /// Sets the platform style.
    pub fn platform_style(mut self, style: PlatformStyle) -> Self {
        self.platform_style = style;
        self
    }

    fn top_padding(&self, cx: &WindowContext) -> Pixels {
        if self.platform_style == PlatformStyle::Windows {
            windows_title_bar_top_offset(cx)
        } else {
            px(0.)
        }
    }
}

impl InteractiveElement for TitleBar {
    fn interactivity(&mut self) -> &mut Interactivity {
        self.content.interactivity()
    }
}

impl StatefulInteractiveElement for TitleBar {}

impl ParentElement for TitleBar {
    fn extend(&mut self, elements: impl Iterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TitleBar {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let height = if self.platform_style == PlatformStyle::Windows {
            windows_title_bar_height(cx)
        } else {
            Self::height(cx)
        };

        let top_padding = self.top_padding(cx);

        h_flex()
            .id("titlebar")
            .w_full()
            .pt(top_padding)
            .h(height)
            .map(|this| {
                if cx.is_fullscreen() {
                    this.pl_2()
                } else if self.platform_style == PlatformStyle::Mac {
                    // Use pixels here instead of a rem-based size because the macOS traffic
                    // lights are a static size, and don't scale with the rest of the UI.
                    this.pl(px(80.))
                } else {
                    this.pl_2()
                }
            })
            .bg(cx.theme().colors().title_bar_background)
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
            .when(self.platform_style == PlatformStyle::Windows, |title_bar| {
                title_bar.child(WindowsWindowControls::new(height))
            })
    }
}
