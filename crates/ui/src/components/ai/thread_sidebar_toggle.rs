use gpui::{AnyView, ClickEvent};
use ui_macros::RegisterComponent;

use crate::prelude::*;
use crate::{IconButton, IconName, Tooltip};

#[derive(IntoElement, RegisterComponent)]
pub struct ThreadSidebarToggle {
    sidebar_selected: bool,
    thread_selected: bool,
    flipped: bool,
    sidebar_tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    thread_tooltip: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyView + 'static>>,
    on_sidebar_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_thread_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl ThreadSidebarToggle {
    pub fn new() -> Self {
        Self {
            sidebar_selected: false,
            thread_selected: false,
            flipped: false,
            sidebar_tooltip: None,
            thread_tooltip: None,
            on_sidebar_click: None,
            on_thread_click: None,
        }
    }

    pub fn sidebar_selected(mut self, selected: bool) -> Self {
        self.sidebar_selected = selected;
        self
    }

    pub fn thread_selected(mut self, selected: bool) -> Self {
        self.thread_selected = selected;
        self
    }

    pub fn flipped(mut self, flipped: bool) -> Self {
        self.flipped = flipped;
        self
    }

    pub fn sidebar_tooltip(
        mut self,
        tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.sidebar_tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn thread_tooltip(
        mut self,
        tooltip: impl Fn(&mut Window, &mut App) -> AnyView + 'static,
    ) -> Self {
        self.thread_tooltip = Some(Box::new(tooltip));
        self
    }

    pub fn on_sidebar_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_sidebar_click = Some(Box::new(handler));
        self
    }

    pub fn on_thread_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_thread_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for ThreadSidebarToggle {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let sidebar_icon = match (self.sidebar_selected, self.flipped) {
            (true, false) => IconName::ThreadsSidebarLeftOpen,
            (false, false) => IconName::ThreadsSidebarLeftClosed,
            (true, true) => IconName::ThreadsSidebarRightOpen,
            (false, true) => IconName::ThreadsSidebarRightClosed,
        };

        h_flex()
            .min_w_0()
            .rounded_sm()
            .gap_px()
            .border_1()
            .border_color(cx.theme().colors().border)
            .when(self.flipped, |this| this.flex_row_reverse())
            .child(
                IconButton::new("sidebar-toggle", sidebar_icon)
                    .icon_size(IconSize::Small)
                    .toggle_state(self.sidebar_selected)
                    .when_some(self.sidebar_tooltip, |this, tooltip| this.tooltip(tooltip))
                    .when_some(self.on_sidebar_click, |this, handler| {
                        this.on_click(handler)
                    }),
            )
            .child(div().h_4().w_px().bg(cx.theme().colors().border))
            .child(
                IconButton::new("thread-toggle", IconName::Thread)
                    .icon_size(IconSize::Small)
                    .toggle_state(self.thread_selected)
                    .when_some(self.thread_tooltip, |this, tooltip| this.tooltip(tooltip))
                    .when_some(self.on_thread_click, |this, handler| this.on_click(handler)),
            )
    }
}

impl Component for ThreadSidebarToggle {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || div().p_2().bg(cx.theme().colors().status_bar_background);

        let examples = vec![
            single_example(
                "Both Unselected",
                container()
                    .child(ThreadSidebarToggle::new())
                    .into_any_element(),
            ),
            single_example(
                "Sidebar Selected",
                container()
                    .child(ThreadSidebarToggle::new().sidebar_selected(true))
                    .into_any_element(),
            ),
            single_example(
                "Thread Selected",
                container()
                    .child(ThreadSidebarToggle::new().thread_selected(true))
                    .into_any_element(),
            ),
            single_example(
                "Both Selected",
                container()
                    .child(
                        ThreadSidebarToggle::new()
                            .sidebar_selected(true)
                            .thread_selected(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Flipped",
                container()
                    .child(
                        ThreadSidebarToggle::new()
                            .sidebar_selected(true)
                            .thread_selected(true)
                            .flipped(true),
                    )
                    .into_any_element(),
            ),
            single_example(
                "With Tooltips",
                container()
                    .child(
                        ThreadSidebarToggle::new()
                            .sidebar_tooltip(Tooltip::text("Toggle Sidebar"))
                            .thread_tooltip(Tooltip::text("Toggle Thread")),
                    )
                    .into_any_element(),
            ),
        ];

        Some(example_group(examples).into_any_element())
    }
}
