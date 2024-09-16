use gpui::{div, prelude::*, px, rems, AnyElement, Div, SharedString, WindowContext};
use itertools::Itertools;
use smallvec::SmallVec;

pub struct Story {}

impl Story {
    pub fn container(cx: &WindowContext) -> gpui::Stateful<Div> {
        let color = cx.default_style().color;

        div()
            .id("story_container")
            .overflow_y_scroll()
            .w_full()
            .min_h_full()
            .flex()
            .flex_col()
            .text_color(color.foreground)
            .bg(color.background)
    }

    pub fn title(cx: &WindowContext, title: impl Into<SharedString>) -> impl Element {
        let color = cx.default_style().color;

        div()
            .text_xs()
            .text_color(color.foreground)
            .child(title.into())
    }

    pub fn title_for<T>(cx: &WindowContext) -> impl Element {
        Self::title(cx, std::any::type_name::<T>())
    }

    pub fn section(cx: &WindowContext) -> Div {
        let color = cx.default_style().color;

        div().p_4().m_4().border_1().border_color(color.separator)
    }

    pub fn section_title(cx: &WindowContext) -> Div {
        let color = cx.default_style().color;

        div().text_lg().text_color(color.foreground)
    }

    pub fn group(cx: &WindowContext) -> Div {
        let color = cx.default_style().color;
        div().my_2().bg(color.container)
    }

    pub fn code_block(cx: &WindowContext, code: impl Into<SharedString>) -> Div {
        let color = cx.default_style().color;

        div()
            .size_full()
            .p_2()
            .max_w(rems(36.))
            .bg(color.container)
            .rounded_md()
            .text_sm()
            .text_color(color.foreground)
            .overflow_hidden()
            .child(code.into())
    }

    pub fn divider(cx: &WindowContext) -> Div {
        let color = cx.default_style().color;

        div().my_2().h(px(1.)).bg(color.separator)
    }

    pub fn description(cx: &WindowContext, description: impl Into<SharedString>) -> impl Element {
        let color = cx.default_style().color;

        div()
            .text_sm()
            .text_color(color.foreground)
            .min_w_96()
            .child(description.into())
    }

    pub fn label(cx: &WindowContext, label: impl Into<SharedString>) -> impl Element {
        let color = cx.default_style().color;

        div()
            .text_xs()
            .text_color(color.foreground)
            .child(label.into())
    }

    /// Note: Not `ui::v_flex` as the `story` crate doesn't depend on the `ui` crate.
    pub fn v_flex() -> Div {
        div().flex().flex_col().gap_1()
    }
}

#[derive(IntoElement)]
pub struct StoryItem {
    label: SharedString,
    item: AnyElement,
    description: Option<SharedString>,
    usage: Option<SharedString>,
}

impl StoryItem {
    pub fn new(label: impl Into<SharedString>, item: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            item: item.into_any_element(),
            description: None,
            usage: None,
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn usage(mut self, code: impl Into<SharedString>) -> Self {
        self.usage = Some(code.into());
        self
    }
}

impl RenderOnce for StoryItem {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let color = cx.default_style().color;

        div()
            .my_2()
            .flex()
            .gap_4()
            .w_full()
            .child(
                Story::v_flex()
                    .px_2()
                    .w_1_2()
                    .min_h_px()
                    .child(Story::label(cx, self.label))
                    .child(
                        div()
                            .rounded_md()
                            .bg(color.background)
                            .border_1()
                            .border_color(color.border)
                            .py_1()
                            .px_2()
                            .overflow_hidden()
                            .child(self.item),
                    )
                    .when_some(self.description, |this, description| {
                        this.child(Story::description(cx, description))
                    }),
            )
            .child(
                Story::v_flex()
                    .px_2()
                    .flex_none()
                    .w_1_2()
                    .min_h_px()
                    .when_some(self.usage, |this, usage| {
                        this.child(Story::label(cx, "Example Usage"))
                            .child(Story::code_block(cx, usage))
                    }),
            )
    }
}

#[derive(IntoElement)]
pub struct StorySection {
    description: Option<SharedString>,
    children: SmallVec<[AnyElement; 2]>,
}

impl Default for StorySection {
    fn default() -> Self {
        Self::new()
    }
}

impl StorySection {
    pub fn new() -> Self {
        Self {
            description: None,
            children: SmallVec::new(),
        }
    }

    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl RenderOnce for StorySection {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let children: SmallVec<[AnyElement; 2]> = SmallVec::from_iter(Itertools::intersperse_with(
            self.children.into_iter(),
            || Story::divider(cx).into_any_element(),
        ));

        Story::section(cx)
            // Section title
            .py_2()
            // Section description
            .when_some(self.description.clone(), |section, description| {
                section.child(Story::description(cx, description))
            })
            .child(div().flex().flex_col().gap_2().children(children))
            .child(Story::divider(cx))
    }
}

impl ParentElement for StorySection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
