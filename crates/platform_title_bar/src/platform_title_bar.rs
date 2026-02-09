mod platforms;
mod system_window_tabs;

use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt};
use gpui::{
    AnyElement, App, Context, Decorations, Entity, Hsla, InteractiveElement, IntoElement,
    MouseButton, ParentElement, StatefulInteractiveElement, Styled, Window, WindowControlArea, div,
    px,
};
use smallvec::SmallVec;
use std::mem;
use ui::{
    prelude::*,
    utils::{TRAFFIC_LIGHT_PADDING, platform_title_bar_height},
};

use crate::{
    platforms::{platform_linux, platform_windows},
    system_window_tabs::SystemWindowTabs,
};

pub use system_window_tabs::{
    DraggedWindowTab, MergeAllWindows, MoveTabToNewWindow, ShowNextWindowTab, ShowPreviousWindowTab,
};

pub struct PlatformTitleBar {
    id: ElementId,
    platform_style: PlatformStyle,
    children: SmallVec<[AnyElement; 2]>,
    should_move: bool,
    system_window_tabs: Entity<SystemWindowTabs>,
    workspace_sidebar_open: bool,
    sidebar_has_notifications: bool,
}

impl PlatformTitleBar {
    pub fn new(id: impl Into<ElementId>, cx: &mut Context<Self>) -> Self {
        let platform_style = PlatformStyle::platform();
        let system_window_tabs = cx.new(|_cx| SystemWindowTabs::new());

        Self {
            id: id.into(),
            platform_style,
            children: SmallVec::new(),
            should_move: false,
            system_window_tabs,
            workspace_sidebar_open: false,
            sidebar_has_notifications: false,
        }
    }

    pub fn title_bar_color(&self, window: &mut Window, cx: &mut Context<Self>) -> Hsla {
        if cfg!(any(target_os = "linux", target_os = "freebsd")) {
            if window.is_window_active() && !self.should_move {
                cx.theme().colors().title_bar_background
            } else {
                cx.theme().colors().title_bar_inactive_background
            }
        } else {
            cx.theme().colors().title_bar_background
        }
    }

    pub fn set_children<T>(&mut self, children: T)
    where
        T: IntoIterator<Item = AnyElement>,
    {
        self.children = children.into_iter().collect();
    }

    pub fn init(cx: &mut App) {
        SystemWindowTabs::init(cx);
    }

    pub fn is_workspace_sidebar_open(&self) -> bool {
        self.workspace_sidebar_open
    }

    pub fn set_workspace_sidebar_open(&mut self, open: bool, cx: &mut Context<Self>) {
        self.workspace_sidebar_open = open;
        cx.notify();
    }

    pub fn sidebar_has_notifications(&self) -> bool {
        self.sidebar_has_notifications
    }

    pub fn set_sidebar_has_notifications(
        &mut self,
        has_notifications: bool,
        cx: &mut Context<Self>,
    ) {
        self.sidebar_has_notifications = has_notifications;
        cx.notify();
    }

    pub fn is_multi_workspace_enabled(cx: &App) -> bool {
        cx.has_flag::<AgentV2FeatureFlag>()
    }
}

impl Render for PlatformTitleBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let supported_controls = window.window_controls();
        let decorations = window.window_decorations();
        let height = platform_title_bar_height(window);
        let titlebar_color = self.title_bar_color(window, cx);
        let close_action = Box::new(workspace::CloseWindow);
        let children = mem::take(&mut self.children);

        let is_multiworkspace_sidebar_open =
            PlatformTitleBar::is_multi_workspace_enabled(cx) && self.is_workspace_sidebar_open();

        let title_bar = h_flex()
            .window_control_area(WindowControlArea::Drag)
            .w_full()
            .h(height)
            .map(|this| {
                this.on_mouse_down_out(cx.listener(move |this, _ev, _window, _cx| {
                    this.should_move = false;
                }))
                .on_mouse_up(
                    gpui::MouseButton::Left,
                    cx.listener(move |this, _ev, _window, _cx| {
                        this.should_move = false;
                    }),
                )
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(move |this, _ev, _window, _cx| {
                        this.should_move = true;
                    }),
                )
                .on_mouse_move(cx.listener(move |this, _ev, window, _| {
                    if this.should_move {
                        this.should_move = false;
                        window.start_window_move();
                    }
                }))
            })
            .map(|this| {
                // Note: On Windows the title bar behavior is handled by the platform implementation.
                this.id(self.id.clone())
                    .when(self.platform_style == PlatformStyle::Mac, |this| {
                        this.on_click(|event, window, _| {
                            if event.click_count() == 2 {
                                window.titlebar_double_click();
                            }
                        })
                    })
                    .when(self.platform_style == PlatformStyle::Linux, |this| {
                        this.on_click(|event, window, _| {
                            if event.click_count() == 2 {
                                window.zoom_window();
                            }
                        })
                    })
            })
            .map(|this| {
                if window.is_fullscreen() {
                    this.pl_2()
                } else if self.platform_style == PlatformStyle::Mac
                    && !is_multiworkspace_sidebar_open
                {
                    this.pl(px(TRAFFIC_LIGHT_PADDING))
                } else {
                    this.pl_2()
                }
            })
            .map(|el| match decorations {
                Decorations::Server => el,
                Decorations::Client { tiling, .. } => el
                    .when(!(tiling.top || tiling.right), |el| {
                        el.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                    })
                    .when(!(tiling.top || tiling.left), |el| {
                        el.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING)
                    })
                    // this border is to avoid a transparent gap in the rounded corners
                    .mt(px(-1.))
                    .mb(px(-1.))
                    .border(px(1.))
                    .border_color(titlebar_color),
            })
            .bg(titlebar_color)
            .content_stretch()
            .child(
                div()
                    .id(self.id.clone())
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .overflow_x_hidden()
                    .w_full()
                    .children(children),
            )
            .when(!window.is_fullscreen(), |title_bar| {
                match self.platform_style {
                    PlatformStyle::Mac => title_bar,
                    PlatformStyle::Linux => {
                        if matches!(decorations, Decorations::Client { .. }) {
                            title_bar
                                .child(platform_linux::LinuxWindowControls::new(close_action))
                                .when(supported_controls.window_menu, |titlebar| {
                                    titlebar
                                        .on_mouse_down(MouseButton::Right, move |ev, window, _| {
                                            window.show_window_menu(ev.position)
                                        })
                                })
                        } else {
                            title_bar
                        }
                    }
                    PlatformStyle::Windows => {
                        title_bar.child(platform_windows::WindowsWindowControls::new(height))
                    }
                }
            });

        v_flex()
            .w_full()
            .child(title_bar)
            .child(self.system_window_tabs.clone().into_any_element())
    }
}

impl ParentElement for PlatformTitleBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
