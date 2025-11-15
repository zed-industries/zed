use gpui::{AnyElement, ScrollHandle, Stateful};
use smallvec::SmallVec;

use crate::Tab;
use crate::prelude::*;

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum TabBarLayout {
    #[default]
    Horizontal,
    Vertical,
}

#[derive(IntoElement, RegisterComponent)]
pub struct TabBar {
    id: ElementId,
    start_children: SmallVec<[AnyElement; 2]>,
    children: SmallVec<[AnyElement; 2]>,
    end_children: SmallVec<[AnyElement; 2]>,
    scroll_handle: Option<ScrollHandle>,
    layout: TabBarLayout,
}

impl TabBar {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            start_children: SmallVec::new(),
            children: SmallVec::new(),
            end_children: SmallVec::new(),
            scroll_handle: None,
            layout: TabBarLayout::Horizontal,
        }
    }

    pub fn track_scroll(mut self, scroll_handle: ScrollHandle) -> Self {
        self.scroll_handle = Some(scroll_handle);
        self
    }

    pub fn start_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.start_children
    }

    pub fn start_child(mut self, start_child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut()
            .push(start_child.into_element().into_any());
        self
    }

    pub fn start_children(
        mut self,
        start_children: impl IntoIterator<Item = impl IntoElement>,
    ) -> Self
    where
        Self: Sized,
    {
        self.start_children_mut().extend(
            start_children
                .into_iter()
                .map(|child| child.into_any_element()),
        );
        self
    }

    pub fn end_children_mut(&mut self) -> &mut SmallVec<[AnyElement; 2]> {
        &mut self.end_children
    }

    pub fn end_child(mut self, end_child: impl IntoElement) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut()
            .push(end_child.into_element().into_any());
        self
    }

    pub fn end_children(mut self, end_children: impl IntoIterator<Item = impl IntoElement>) -> Self
    where
        Self: Sized,
    {
        self.end_children_mut().extend(
            end_children
                .into_iter()
                .map(|child| child.into_any_element()),
        );
        self
    }

    pub fn layout(mut self, layout: TabBarLayout) -> Self {
        self.layout = layout;
        self
    }

    fn render_horizontal(self, cx: &mut App) -> Stateful<Div> {
        let TabBar {
            id,
            start_children,
            children,
            end_children,
            scroll_handle,
            ..
        } = self;

        div()
            .id(id)
            .group("tab_bar")
            .flex()
            .flex_none()
            .w_full()
            .h(Tab::container_height(cx))
            .bg(cx.theme().colors().tab_bar_background)
            .when(!start_children.is_empty(), |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .px(DynamicSpacing::Base06.rems(cx))
                        .border_b_1()
                        .border_r_1()
                        .border_color(cx.theme().colors().border)
                        .children(start_children),
                )
            })
            .child(
                div()
                    .relative()
                    .flex_1()
                    .h_full()
                    .overflow_x_hidden()
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .border_b_1()
                            .border_color(cx.theme().colors().border),
                    )
                    .child(
                        h_flex()
                            .id("tabs")
                            .flex_grow()
                            .overflow_x_scroll()
                            .when_some(scroll_handle, |cx, scroll_handle| {
                                cx.track_scroll(&scroll_handle)
                            })
                            .children(children),
                    ),
            )
            .when(!end_children.is_empty(), |this| {
                this.child(
                    h_flex()
                        .flex_none()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .px(DynamicSpacing::Base06.rems(cx))
                        .border_b_1()
                        .border_l_1()
                        .border_color(cx.theme().colors().border)
                        .children(end_children),
                )
            })
    }

    fn render_vertical(self, cx: &mut App) -> Stateful<Div> {
        let TabBar {
            id,
            start_children,
            children,
            end_children,
            scroll_handle,
            ..
        } = self;

        v_flex()
            .id(id)
            .group("tab_bar")
            .flex_1()
            .w_full()
            .h_full()
            .bg(cx.theme().colors().tab_bar_background)
            .when(!start_children.is_empty(), |this| {
                this.child(
                    v_flex()
                        .flex_none()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .px(DynamicSpacing::Base06.rems(cx))
                        .py(DynamicSpacing::Base04.rems(cx))
                        .border_b_1()
                        .border_color(cx.theme().colors().border)
                        .children(start_children),
                )
            })
            .child(
                v_flex()
                    .relative()
                    .flex_1()
                    .w_full()
                    .overflow_y_hidden()
                    .child(
                        v_flex()
                            .id("tabs")
                            .flex_grow()
                            .overflow_y_scroll()
                            .when_some(scroll_handle, |cx, scroll_handle| {
                                cx.track_scroll(&scroll_handle)
                            })
                            .children(children),
                    ),
            )
            .when(!end_children.is_empty(), |this| {
                this.child(
                    v_flex()
                        .flex_none()
                        .gap(DynamicSpacing::Base04.rems(cx))
                        .px(DynamicSpacing::Base06.rems(cx))
                        .py(DynamicSpacing::Base04.rems(cx))
                        .border_t_1()
                        .border_color(cx.theme().colors().border)
                        .children(end_children),
                )
            })
    }
}

impl ParentElement for TabBar {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl RenderOnce for TabBar {
    fn render(self, _: &mut Window, cx: &mut App) -> impl IntoElement {
        match self.layout {
            TabBarLayout::Horizontal => self.render_horizontal(cx),
            TabBarLayout::Vertical => self.render_vertical(cx),
        }
    }
}

impl Component for TabBar {
    fn scope() -> ComponentScope {
        ComponentScope::Navigation
    }

    fn name() -> &'static str {
        "TabBar"
    }

    fn description() -> Option<&'static str> {
        Some("A horizontal bar containing tabs for navigation between different views or sections.")
    }

    fn preview(_window: &mut Window, _cx: &mut App) -> Option<AnyElement> {
        Some(
            v_flex()
                .gap_6()
                .children(vec![
                    example_group_with_title(
                        "Basic Usage",
                        vec![
                            single_example(
                                "Empty TabBar",
                                TabBar::new("empty_tab_bar").into_any_element(),
                            ),
                            single_example(
                                "With Tabs",
                                TabBar::new("tab_bar_with_tabs")
                                    .child(Tab::new("tab1"))
                                    .child(Tab::new("tab2"))
                                    .child(Tab::new("tab3"))
                                    .into_any_element(),
                            ),
                        ],
                    ),
                    example_group_with_title(
                        "With Start and End Children",
                        vec![single_example(
                            "Full TabBar",
                            TabBar::new("full_tab_bar")
                                .start_child(Button::new("start_button", "Start"))
                                .child(Tab::new("tab1"))
                                .child(Tab::new("tab2"))
                                .child(Tab::new("tab3"))
                                .end_child(Button::new("end_button", "End"))
                                .into_any_element(),
                        )],
                    ),
                ])
                .into_any_element(),
        )
    }
}
