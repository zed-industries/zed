use chrono::Local;
use gpui::*;
use serde::Deserialize;
use ui::{prelude::*, Tooltip};

gpui::actions!(
    assistant_conversations,
    [
        PinConversation,
        DeleteConversation,
        RevealFile,
        OpenConversation,
        ReturnToLastConversation
    ]
);

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

#[derive(Eq, PartialEq, Clone, Deserialize)]
pub struct ConversationPreviewData {
    pub id: SharedString,
    pub title: SharedString,
    pub last_message: SharedString,
    pub last_message_time: chrono::DateTime<chrono::Local>,
}

pub struct ConversationsState {
    conversation_previews: Vec<ConversationPreviewData>,
    pinned_conversations: Vec<SharedString>,
    // TODO: For a "Return to Last Conversation action"
    last_conversation: Option<SharedString>,
    view_style: ConversationViewStyle,
}

impl ConversationsState {
    pub fn new() -> Self {
        Self {
            conversation_previews: Vec::new(),
            pinned_conversations: Vec::new(),
            last_conversation: None,
            view_style: ConversationViewStyle::Details,
        }
    }
}

pub struct ConversationsView {
    // todo, get this from assistant2
    handle_new_conversation: Option<Box<dyn Fn()>>,
    state: Model<ConversationsState>,
}

impl ConversationsView {
    pub fn new(cx: &mut WindowContext) -> Self {
        let state = cx.new_model(|_| ConversationsState::new());

        Self {
            state,
            handle_new_conversation: None,
        }
    }

    pub fn pin_conversation(&self, conversation_id: SharedString) {
        todo!()
    }

    pub fn delete_conversation(&self, conversation_id: SharedString) {
        todo!()
    }

    pub fn reveal_file(&self, file_id: SharedString) {
        todo!()
    }

    pub fn open_conversation(&self, conversation_id: SharedString) {
        todo!()
    }

    pub fn set_view_style(&self, cx: &mut ViewContext<Self>, view_style: ConversationViewStyle) {
        self.state.update(cx, |s, _| {
            s.view_style = view_style;
        });
    }

    pub fn set_conversation_previews(&self, conversations: Vec<ConversationPreviewData>) {
        todo!("get conversations from index and make conversation previews")
    }
}

impl Render for ConversationsView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let header_height = Spacing::Small.rems(cx) * 2.0 + ButtonSize::Default.rems();

        let view_style = self.state.read(cx).view_style;
        let conversations = &self.state.read(cx).conversation_previews;
        let pinned_conversation_ids = self.state.read(cx).pinned_conversations.clone();

        let mut pinned_conversations = vec![];
        let mut not_pinned_conversations = vec![];

        for conversation in conversations {
            if pinned_conversation_ids.contains(&conversation.id) {
                pinned_conversations.push(conversation);
            } else {
                not_pinned_conversations.push(conversation);
            }
        }

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
                    .child(
                        v_flex()
                            .flex_1()
                            .child(Headline::new("Pinned").size(HeadlineSize::XSmall))
                            .children(
                                pinned_conversations
                                    .into_iter()
                                    .map(|conversation| ConversationPreview::new(conversation)),
                            ),
                    )
                    .child(
                        v_flex()
                            .flex_1()
                            .child(Headline::new("All Conversations").size(HeadlineSize::XSmall))
                            .children(
                                not_pinned_conversations
                                    .into_iter()
                                    .map(|conversation| ConversationPreview::new(conversation)),
                            ),
                    ),
            )
    }
}

#[derive(IntoElement)]
struct ConversationPreview {
    conversation: ConversationPreviewData,
}

impl ConversationPreview {
    pub fn new(conversation: &ConversationPreviewData) -> Self {
        let conversation = conversation.clone();

        Self { conversation }
    }
}

impl RenderOnce for ConversationPreview {
    fn render(self, cx: &mut WindowContext) -> impl IntoElement {
        let line_height = rems(1.15);
        let max_preview_height = line_height.clone() * 3.0;

        let preview_string: SharedString = format!(
            "{}  {}",
            render_relative_date(self.conversation.last_message_time),
            self.conversation.last_message.clone()
        )
        .into();

        v_flex()
            .id(self.conversation.id.clone())
            .flex_none()
            .w_full()
            .line_height(line_height)
            .p(Spacing::Large.rems(cx))
            // .on_click(todo!())
            .child(Headline::new(self.conversation.title.clone()).size(HeadlineSize::XSmall))
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
