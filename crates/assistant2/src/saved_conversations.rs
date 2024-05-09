use chrono::Local;
use gpui::View;
use serde::Deserialize;
use ui::{prelude::*, Tooltip};

use crate::{
    saved_conversation::SavedConversationMetadata,
    saved_conversation_picker::{SavedConversationPicker, SavedConversationPickerDelegate},
};

// temp, will be unified and moved into the time_format crate
fn render_relative_date(date: chrono::DateTime<Local>) -> String {
    let now = Local::now();

    let duration_since = now.signed_duration_since(date);

    if duration_since.num_hours() < 24 {
        return date.format("%H:%M %P").to_string();
    } else if duration_since.num_days() < 7 {
        return date.format("%A").to_string();
    } else {
        return date.format("%Y-%m-%d").to_string();
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum ConversationViewStyle {
    List,
    Details,
}

pub struct SavedConversations {
    view_style: ConversationViewStyle,
    picker: Option<View<SavedConversationPicker>>,
}

impl SavedConversations {
    pub fn new(_cx: &mut WindowContext) -> Self {
        Self {
            view_style: ConversationViewStyle::List,
            picker: None,
        }
    }

    pub fn init(
        &mut self,
        view: WeakView<SavedConversation>,
        saved_conversations: Vec<SavedConversationMetadata>,
        cx: &mut WindowContext,
    ) {
        let delegate =
            SavedConversationPickerDelegate::new(cx.view().downgrade(), saved_conversations);
        self.picker = Some(SavedConversationPicker::new(delegate, cx));
    }
}

impl Render for SavedConversations {
    fn render(&mut self, cx: &mut ui::prelude::ViewContext<Self>) -> impl IntoElement {
        let header_height = Spacing::Small.rems(cx) * 2.0 + ButtonSize::Default.rems();
        let view_style = self.view_style;

        div()
            .relative()
            .flex_1()
            .v_flex()
            .key_context("AssistantConversations")
            .text_color(Color::Default.color(cx))
            .child(
                v_flex()
                    .child(
                        h_flex()
                            .flex_none()
                            .justify_between()
                            .w_full()
                            .h(header_height)
                            .p(Spacing::Small.rems(cx))
                            .border_b_1()
                            .border_color(cx.theme().colors().border)
                            .child(
                                h_flex()
                                    .gap(Spacing::Small.rems(cx))
                                    .child(
                                        IconButton::new("set-view-list", IconName::List)
                                            .icon_color(
                                                if view_style == ConversationViewStyle::List {
                                                    Color::Accent
                                                } else {
                                                    Color::Default
                                                },
                                            )
                                            .selected(view_style == ConversationViewStyle::List)
                                            .tooltip(move |cx| {
                                                Tooltip::text("View conversations as a list", cx)
                                            }),
                                    )
                                    .child(
                                        IconButton::new("set-view-details", IconName::Text)
                                            .icon_color(
                                                if view_style == ConversationViewStyle::Details {
                                                    Color::Accent
                                                } else {
                                                    Color::Default
                                                },
                                            )
                                            .selected(view_style == ConversationViewStyle::Details)
                                            .tooltip(move |cx| {
                                                Tooltip::text(
                                                    "View conversations with summaries",
                                                    cx,
                                                )
                                            }),
                                    ),
                            )
                            .child(
                                IconButton::new("new-conversation", IconName::Plus)
                                    .tooltip(move |cx| Tooltip::text("New Conversation", cx)),
                            ),
                    )
                    .map(|element| {
                        if let Some(picker) = self.picker.as_ref() {
                            element.child(picker.clone())
                        } else {
                            element.child(
                                v_flex()
                                    .flex_1()
                                    .size_full()
                                    .justify_center()
                                    .items_center()
                                    .p(Spacing::Large.rems(cx))
                                    .child(
                                        Label::new("Loading conversations...")
                                            .color(Color::Placeholder),
                                    ),
                            )
                        }
                    }),
            )
    }
}

#[derive(Eq, PartialEq, Clone, Deserialize)]
pub struct ConversationPreviewData {
    pub id: SharedString,
    pub title: SharedString,
    pub last_message: SharedString,
    pub last_message_time: chrono::DateTime<chrono::Local>,
}

impl ConversationPreviewData {
    pub fn new(
        path: impl Into<SharedString>,
        title: impl Into<SharedString>,
        last_message_time: chrono::DateTime<chrono::Local>,
    ) -> Self {
        Self {
            id: path.into().into(),
            title: title.into(),
            last_message: "".into(),
            last_message_time,
        }
    }
}

#[derive(IntoElement)]
struct ConversationPreview {
    conversation_preview: ConversationPreviewData,
}

impl RenderOnce for ConversationPreview {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let line_height = rems(1.15);
        let max_preview_height = line_height.clone() * 3.0;

        let preview_string: SharedString = format!(
            "{}  {}",
            render_relative_date(self.conversation_preview.last_message_time),
            self.conversation_preview.last_message.clone()
        )
        .into();

        v_flex()
            .id(self.conversation_preview.id.clone())
            .flex_none()
            .w_full()
            .line_height(line_height)
            .p(Spacing::Large.rems(cx))
            // .on_click(todo!())
            .child(
                Headline::new(self.conversation_preview.title.clone()).size(HeadlineSize::XSmall),
            )
            .child(
                div()
                    .w_full()
                    .min_h(cx.line_height())
                    .max_h(max_preview_height)
                    .text_color(cx.theme().colors().text_muted)
                    .text_sm()
                    .overflow_hidden()
                    .child(preview_string),
            )
    }
}
