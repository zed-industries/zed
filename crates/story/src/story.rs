use gpui::{
    AnyElement, App, DefaultColor, DefaultColors, Div, SharedString, Window, div, prelude::*, px,
    rems,
};
use itertools::Itertools;
use smallvec::SmallVec;

pub struct Story {}

impl Story {
    pub fn container() -> gpui::Stateful<Div> {
        let colors = DefaultColors::light();

        div()
            .id("story_container")
            .overflow_y_scroll()
            .w_full()
            .min_h_full()
            .flex()
            .flex_col()
            .text_color(DefaultColor::Text.hsla(&colors))
            .bg(DefaultColor::Background.hsla(&colors))
    }

    pub fn title(title: impl Into<SharedString>) -> impl Element {
        let colors = DefaultColors::light();

        div()
            .text_xs()
            .text_color(DefaultColor::Text.hsla(&colors))
            .child(title.into())
    }

    pub fn title_for<T>() -> impl Element {
        Self::title(std::any::type_name::<T>())
    }

    pub fn section() -> Div {
        let colors = DefaultColors::light();

        div()
            .p_4()
            .m_4()
            .border_1()
            .border_color(DefaultColor::Separator.hsla(&colors))
    }

    pub fn section_title() -> Div {
        let colors = DefaultColors::light();

        div().text_lg().text_color(DefaultColor::Text.hsla(&colors))
    }

    pub fn group() -> Div {
        let colors = DefaultColors::light();
        div().my_2().bg(DefaultColor::Container.hsla(&colors))
    }

    pub fn code_block(code: impl Into<SharedString>) -> Div {
        let colors = DefaultColors::light();

        div()
            .size_full()
            .p_2()
            .max_w(rems(36.))
            .bg(DefaultColor::Container.hsla(&colors))
            .rounded_sm()
            .text_sm()
            .text_color(DefaultColor::Text.hsla(&colors))
            .overflow_hidden()
            .child(code.into())
    }

    pub fn divider() -> Div {
        let colors = DefaultColors::light();

        div()
            .my_2()
            .h(px(1.))
            .bg(DefaultColor::Separator.hsla(&colors))
    }

    pub fn description(description: impl Into<SharedString>) -> impl Element {
        let colors = DefaultColors::light();

        div()
            .text_sm()
            .text_color(DefaultColor::Text.hsla(&colors))
            .min_w_96()
            .child(description.into())
    }

    pub fn label(label: impl Into<SharedString>) -> impl Element {
        let colors = DefaultColors::light();

        div()
            .text_xs()
            .text_color(DefaultColor::Text.hsla(&colors))
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let colors = DefaultColors::light();

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
                    .child(Story::label(self.label))
                    .child(
                        div()
                            .rounded_sm()
                            .bg(DefaultColor::Background.hsla(&colors))
                            .border_1()
                            .border_color(DefaultColor::Border.hsla(&colors))
                            .py_1()
                            .px_2()
                            .overflow_hidden()
                            .child(self.item),
                    )
                    .when_some(self.description, |this, description| {
                        this.child(Story::description(description))
                    }),
            )
            .child(
                Story::v_flex()
                    .px_2()
                    .flex_none()
                    .w_1_2()
                    .min_h_px()
                    .when_some(self.usage, |this, usage| {
                        this.child(Story::label("Example Usage"))
                            .child(Story::code_block(usage))
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
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let children: SmallVec<[AnyElement; 2]> = SmallVec::from_iter(Itertools::intersperse_with(
            self.children.into_iter(),
            || Story::divider().into_any_element(),
        ));

        Story::section()
            // Section title
            .py_2()
            // Section description
            .when_some(self.description.clone(), |section, description| {
                section.child(Story::description(description))
            })
            .child(div().flex().flex_col().gap_2().children(children))
            .child(Story::divider())
    }
}

impl ParentElement for StorySection {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}
