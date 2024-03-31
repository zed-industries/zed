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
        (1.75 * cx.rem_size()).max(px(34.))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_cx: &mut WindowContext) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.)
    }

    #[cfg(not(target_os = "windows"))]
    fn top_padding(_cx: &WindowContext) -> Pixels {
        px(0.)
    }

    #[cfg(target_os = "windows")]
    fn top_padding(cx: &WindowContext) -> Pixels {
        use windows::Win32::UI::{
            HiDpi::GetSystemMetricsForDpi,
            WindowsAndMessaging::{SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI},
        };

        // This top padding is not dependent on the title bar style and is instead a quirk of maximized windows on Windows:
        // https://devblogs.microsoft.com/oldnewthing/20150304-00/?p=44543
        let padding = unsafe { GetSystemMetricsForDpi(SM_CXPADDEDBORDER, USER_DEFAULT_SCREEN_DPI) };
        if cx.is_maximized() {
            px((padding * 2) as f32)
        } else {
            px(0.)
        }
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
        let height = Self::height(cx);
        h_flex()
            .id("titlebar")
            .w_full()
            .pt(Self::top_padding(cx))
            .h(height + Self::top_padding(cx))
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
            .when(
                self.platform_style == PlatformStyle::Windows && !cx.is_fullscreen(),
                |title_bar| title_bar.child(WindowsWindowControls::new(height)),
            )
    }
}
