pub mod platforms;
mod system_window_tabs;

use gpui::{
    Action, AnyElement, App, Context, Decorations, Entity, Hsla, InteractiveElement, IntoElement,
    MouseButton, ParentElement, StatefulInteractiveElement, Styled, WeakEntity, Window,
    WindowButtonLayout, WindowControlArea, div, px, rgb,
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
    fullscreen_macos_controls_reveal: f32,
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
            fullscreen_macos_controls_reveal: 0.0,
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
        !DisableAiSettings::get_global(cx).disable_ai
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

#[derive(IntoElement)]
struct MacFullscreenWindowControls {
    opacity: f32,
    close_action: Box<dyn Action>,
}

impl MacFullscreenWindowControls {
    fn new(opacity: f32, close_action: Box<dyn Action>) -> Self {
        Self {
            opacity,
            close_action,
        }
    }
}

impl RenderOnce for MacFullscreenWindowControls {
    fn render(self, window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let supported_controls = window.window_controls();
        h_flex()
            .id("fullscreen-macos-window-controls")
            .h_full()
            .w(px(TRAFFIC_LIGHT_PADDING))
            .flex_none()
            .items_center()
            .pl(px(9.0))
            .gap(px(8.0))
            .opacity(self.opacity)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(MacFullscreenWindowControl::new(
                "fullscreen-macos-close",
                MacFullscreenWindowControlKind::Close,
                rgb(0xff5f57).into(),
                Some(self.close_action),
            ))
            .children(supported_controls.minimize.then(|| {
                MacFullscreenWindowControl::new(
                    "fullscreen-macos-minimize",
                    MacFullscreenWindowControlKind::Minimize,
                    rgb(0xffbd2e).into(),
                    None,
                )
            }))
            .children(supported_controls.maximize.then(|| {
                MacFullscreenWindowControl::new(
                    "fullscreen-macos-zoom",
                    MacFullscreenWindowControlKind::Zoom,
                    rgb(0x28c840).into(),
                    None,
                )
            }))
    }
}

#[derive(IntoElement)]
struct MacFullscreenWindowControl {
    id: &'static str,
    kind: MacFullscreenWindowControlKind,
    color: Hsla,
    close_action: Option<Box<dyn Action>>,
}

impl MacFullscreenWindowControl {
    fn new(
        id: &'static str,
        kind: MacFullscreenWindowControlKind,
        color: Hsla,
        close_action: Option<Box<dyn Action>>,
    ) -> Self {
        Self {
            id,
            kind,
            color,
            close_action,
        }
    }
}

#[derive(Clone, Copy)]
enum MacFullscreenWindowControlKind {
    Close,
    Minimize,
    Zoom,
}

impl RenderOnce for MacFullscreenWindowControl {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .id(self.id)
            .size(px(12.0))
            .rounded_full()
            .bg(self.color)
            .border_1()
            .border_color(gpui::black().opacity(0.12))
            .hover(|this| this.opacity(0.85))
            .active(|this| this.opacity(0.7))
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .on_click(move |_, window, cx| {
                cx.stop_propagation();
                match self.kind {
                    MacFullscreenWindowControlKind::Close => {
                        if let Some(close_action) = self.close_action.as_ref() {
                            window.dispatch_action(close_action.boxed_clone(), cx);
                        }
                    }
                    MacFullscreenWindowControlKind::Minimize => window.minimize_window(),
                    MacFullscreenWindowControlKind::Zoom => window.zoom_window(),
                }
            })
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

        if self.platform_style == PlatformStyle::Mac && window.is_fullscreen() {
            let mouse_position = window.mouse_position();
            let should_show_controls = mouse_position.x <= px(96.0) && mouse_position.y <= height;
            let target = if should_show_controls { 1.0 } else { 0.0 };
            let delta = target - self.fullscreen_macos_controls_reveal;
            if delta.abs() <= 0.02 {
                self.fullscreen_macos_controls_reveal = target;
            } else {
                self.fullscreen_macos_controls_reveal =
                    (self.fullscreen_macos_controls_reveal + delta.signum() * 0.18).clamp(0.0, 1.0);
                window.request_animation_frame();
            }
        } else {
            self.fullscreen_macos_controls_reveal = 0.0;
        }

        let fullscreen_macos_controls_reveal = self.fullscreen_macos_controls_reveal;

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
                .on_mouse_move(cx.listener(move |this, _ev, window, cx| {
                    if this.should_move {
                        this.should_move = false;
                        window.start_window_move();
                    }
                    if this.platform_style == PlatformStyle::Mac && window.is_fullscreen() {
                        cx.notify();
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

                if window.is_fullscreen() && self.platform_style == PlatformStyle::Mac {
                    this.child(MacFullscreenWindowControls::new(
                        fullscreen_macos_controls_reveal,
                        close_action.as_ref().boxed_clone(),
                    ))
                } else if window.is_fullscreen() {
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
