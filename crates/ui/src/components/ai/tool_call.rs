use crate::prelude::*;
use gpui::{AnyElement, IntoElement, ParentElement, SharedString};

#[derive(IntoElement, RegisterComponent)]
pub struct ToolCall {
    icon: IconName,
    title: SharedString,
    actions_slot: Option<AnyElement>,
    content: Option<AnyElement>,
    use_card_layout: bool,
}

impl ToolCall {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            icon: IconName::ToolSearch,
            title: title.into(),
            actions_slot: None,
            use_card_layout: false,
            content: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = icon;
        self
    }

    pub fn actions_slot(mut self, action: impl IntoElement) -> Self {
        self.actions_slot = Some(action.into_any_element());
        self
    }

    pub fn content(mut self, content: impl IntoElement) -> Self {
        self.content = Some(content.into_any_element());
        self
    }

    pub fn use_card_layout(mut self, use_card_layout: bool) -> Self {
        self.use_card_layout = use_card_layout;
        self
    }
}

impl RenderOnce for ToolCall {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .when(self.use_card_layout, |this| {
                this.border_1()
                    .border_color(cx.theme().colors().border)
                    .rounded_md()
                    .overflow_hidden()
            })
            .child(
                h_flex()
                    .gap_1()
                    .justify_between()
                    .when(self.use_card_layout, |this| {
                        this.px_2()
                            .py_1()
                            .bg(cx.theme().colors().element_background.opacity(0.2))
                    })
                    .child(
                        h_flex()
                            .hover(|s| s.bg(cx.theme().colors().element_hover.opacity(0.5)))
                            .gap_1p5()
                            .rounded_xs()
                            .child(
                                Icon::new(self.icon)
                                    .size(IconSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                Label::new(self.title)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .when_some(self.actions_slot, |this, action| this.child(action)),
            )
            .when_some(self.content, |this, content| {
                this.child(
                    div()
                        .when(self.use_card_layout, |this| {
                            this.p_2()
                                .border_t_1()
                                .border_color(cx.theme().colors().border)
                                .bg(cx.theme().colors().editor_background)
                        })
                        .child(content),
                )
            })
    }
}

impl Component for ToolCall {
    fn scope() -> ComponentScope {
        ComponentScope::Agent
    }

    fn preview(_window: &mut Window, cx: &mut App) -> Option<AnyElement> {
        let container = || {
            v_flex()
                .p_2()
                .w_128()
                .border_1()
                .border_color(cx.theme().colors().border_variant)
                .bg(cx.theme().colors().panel_background)
        };

        let muted_icon_button = |id: &'static str, icon: IconName| {
            IconButton::new(id, icon)
                .icon_size(IconSize::Small)
                .icon_color(Color::Muted)
        };

        let examples = vec![
            single_example(
                "Non-card (header only)",
                container()
                    .child(
                        ToolCall::new("Search repository")
                            .icon(IconName::ToolSearch)
                            .actions_slot(muted_icon_button(
                                "toolcall-noncard-expand",
                                IconName::ChevronDown,
                            )),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Non-card + content",
                container()
                    .child(
                        ToolCall::new("Edit file: src/main.rs")
                            .icon(IconName::File)
                            .content(
                                Label::new("Tool output here — markdown, list, etc.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            ),
                    )
                    .into_any_element(),
            ),
            single_example(
                "Card layout + actions",
                container()
                    .child(
                        ToolCall::new("Run Command")
                            .icon(IconName::ToolTerminal)
                            .use_card_layout(true)
                            .actions_slot(muted_icon_button(
                                "toolcall-card-expand",
                                IconName::ChevronDown,
                            ))
                            .content(
                                Label::new("git status")
                                    .size(LabelSize::Small)
                                    .buffer_font(cx),
                            ),
                    )
                    .into_any_element(),
            ),
        ];

        Some(example_group(examples).vertical().into_any_element())
    }
}
