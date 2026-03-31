pub mod platforms;
mod system_window_tabs;

use feature_flags::{AgentV2FeatureFlag, FeatureFlagAppExt};
use gpui::{
    Action, AnyElement, App, Context, Decorations, Entity, Hsla, InteractiveElement, IntoElement,
    MouseButton, ParentElement, StatefulInteractiveElement, Styled, WeakEntity, Window,
    WindowButtonLayout, WindowControlArea, div, px,
};
use project::DisableAiSettings;
use settings::Settings;
use smallvec::SmallVec;
use std::mem;
use ui::{
    prelude::*,
    utils::{TRAFFIC_LIGHT_PADDING, platform_title_bar_height},
};
use workspace::{MultiWorkspace, SidebarRenderState, SidebarSide};

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
    button_layout: Option<WindowButtonLayout>,
    multi_workspace: Option<WeakEntity<MultiWorkspace>>,
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
            button_layout: None,
            multi_workspace: None,
        }
    }

    pub fn with_multi_workspace(mut self, multi_workspace: WeakEntity<MultiWorkspace>) -> Self {
        self.multi_workspace = Some(multi_workspace);
        self
    }

    pub fn set_multi_workspace(&mut self, multi_workspace: WeakEntity<MultiWorkspace>) {
        self.multi_workspace = Some(multi_workspace);
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

    pub fn set_button_layout(&mut self, button_layout: Option<WindowButtonLayout>) {
        self.button_layout = button_layout;
    }

    fn effective_button_layout(
        &self,
        decorations: &Decorations,
        cx: &App,
    ) -> Option<WindowButtonLayout> {
        if self.platform_style == PlatformStyle::Linux
            && matches!(decorations, Decorations::Client { .. })
        {
            self.button_layout.or_else(|| cx.button_layout())
        } else {
            None
        }
    }

    pub fn init(cx: &mut App) {
        SystemWindowTabs::init(cx);
    }

    fn sidebar_render_state(&self, cx: &App) -> SidebarRenderState {
        self.multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).sidebar_render_state(cx))
            .unwrap_or_default()
    }

    pub fn is_multi_workspace_enabled(cx: &App) -> bool {
        cx.has_flag::<AgentV2FeatureFlag>() && !DisableAiSettings::get_global(cx).disable_ai
    }
}

/// Renders the platform-appropriate left-side window controls (e.g. Ubuntu/GNOME close button).
///
/// Only relevant on Linux with client-side decorations when the window manager
/// places controls on the left.
pub fn render_left_window_controls(
    button_layout: Option<WindowButtonLayout>,
    close_action: Box<dyn Action>,
    window: &Window,
) -> Option<AnyElement> {
    if PlatformStyle::platform() != PlatformStyle::Linux {
        return None;
    }
    if !matches!(window.window_decorations(), Decorations::Client { .. }) {
        return None;
    }
    let button_layout = button_layout?;
    if button_layout.left[0].is_none() {
        return None;
    }
    Some(
        platform_linux::LinuxWindowControls::new(
            "left-window-controls",
            button_layout.left,
            close_action,
        )
        .into_any_element(),
    )
}

/// Renders the platform-appropriate right-side window controls (close, minimize, maximize).
///
/// Returns `None` on Mac or when the platform doesn't need custom controls
/// (e.g. Linux with server-side decorations).
pub fn render_right_window_controls(
    button_layout: Option<WindowButtonLayout>,
    close_action: Box<dyn Action>,
    window: &Window,
) -> Option<AnyElement> {
    let decorations = window.window_decorations();
    let height = platform_title_bar_height(window);

    match PlatformStyle::platform() {
        PlatformStyle::Linux => {
            if !matches!(decorations, Decorations::Client { .. }) {
                return None;
            }
            let button_layout = button_layout?;
            if button_layout.right[0].is_none() {
                return None;
            }
            Some(
                platform_linux::LinuxWindowControls::new(
                    "right-window-controls",
                    button_layout.right,
                    close_action,
                )
                .into_any_element(),
            )
        }
        PlatformStyle::Windows => {
            Some(platform_windows::WindowsWindowControls::new(height).into_any_element())
        }
        PlatformStyle::Mac => None,
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

        let button_layout = self.effective_button_layout(&decorations, cx);
        let sidebar = self.sidebar_render_state(cx);

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
                let show_left_controls = !(sidebar.open && sidebar.side == SidebarSide::Left);

                if window.is_fullscreen() {
                    this.pl_2()
                } else if self.platform_style == PlatformStyle::Mac && show_left_controls {
                    this.pl(px(TRAFFIC_LIGHT_PADDING))
                } else if let Some(controls) = show_left_controls
                    .then(|| {
                        render_left_window_controls(
                            button_layout,
                            close_action.as_ref().boxed_clone(),
                            window,
                        )
                    })
                    .flatten()
                {
                    this.child(controls)
                } else {
                    this.pl_2()
                }
            })
            .map(|el| match decorations {
                Decorations::Server => el,
                Decorations::Client { tiling, .. } => el
                    .when(
                        !(tiling.top || tiling.right)
                            && !(sidebar.open && sidebar.side == SidebarSide::Right),
                        |el| el.rounded_tr(theme::CLIENT_SIDE_DECORATION_ROUNDING),
                    )
                    .when(
                        !(tiling.top || tiling.left)
                            && !(sidebar.open && sidebar.side == SidebarSide::Left),
                        |el| el.rounded_tl(theme::CLIENT_SIDE_DECORATION_ROUNDING),
                    )
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
                let show_right_controls = !(sidebar.open && sidebar.side == SidebarSide::Right);

                let title_bar = title_bar.children(
                    show_right_controls
                        .then(|| {
                            render_right_window_controls(
                                button_layout,
                                close_action.as_ref().boxed_clone(),
                                window,
                            )
                        })
                        .flatten(),
                );

                if self.platform_style == PlatformStyle::Linux
                    && matches!(decorations, Decorations::Client { .. })
                {
                    title_bar.when(supported_controls.window_menu, |titlebar| {
                        titlebar.on_mouse_down(MouseButton::Right, move |ev, window, _| {
                            window.show_window_menu(ev.position)
                        })
                    })
                } else {
                    title_bar
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
