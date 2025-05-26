use gpui::{Context, Entity, Window, Timer};
use language_model::{LanguageModel, Role};
use std::sync::Arc;
use ui::{prelude::*, IconName, Label, LabelSize, Button, ButtonStyle, Icon};
use ui_input::SingleLineInput;

/// Interactive chat interface for AI conversations
pub struct ChatInterface {
    messages: Vec<ChatMessage>,
    input: Entity<SingleLineInput>,
    selected_model: Option<Arc<dyn LanguageModel>>,
    is_generating: bool,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub role: Role,
    pub content: String,
    pub timestamp: std::time::SystemTime,
}

impl ChatInterface {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| SingleLineInput::new(window, cx, "Type your message..."));
        
        Self {
            messages: Vec::new(),
            input,
            selected_model: None,
            is_generating: false,
        }
    }

    pub fn set_model(&mut self, model: Arc<dyn LanguageModel>, cx: &mut Context<Self>) {
        self.selected_model = Some(model);
        cx.notify();
    }

    pub fn send_message(&mut self, content: String, window: &mut Window, cx: &mut Context<Self>) {
        if content.trim().is_empty() || self.is_generating {
            return;
        }

        // Add user message
        let user_message = ChatMessage {
            role: Role::User,
            content: content.clone(),
            timestamp: std::time::SystemTime::now(),
        };
        self.messages.push(user_message);

        // Clear input
        self.input.update(cx, |input, cx| {
            input.editor.update(cx, |editor, cx| {
                editor.set_text("", window, cx);
            });
        });

        // Generate AI response if model is available
        if let Some(model) = &self.selected_model {
            self.is_generating = true;
            self.generate_response(model.clone(), cx);
        } else {
            // Add system message about no model
            let system_message = ChatMessage {
                role: Role::Assistant,
                content: "No AI model selected. Please select a model to continue the conversation.".to_string(),
                timestamp: std::time::SystemTime::now(),
            };
            self.messages.push(system_message);
        }

        cx.notify();
    }

    fn generate_response(&mut self, _model: Arc<dyn LanguageModel>, cx: &mut Context<Self>) {
        // For now, just add a placeholder response without making an actual request
        // TODO: Implement actual streaming response
        cx.spawn(async move |this, cx| {
            // Simulate some delay
            Timer::after(std::time::Duration::from_millis(1000)).await;
            
            this.update(cx, |this, cx| {
                let response_message = ChatMessage {
                    role: Role::Assistant,
                    content: "This is a placeholder response. Streaming chat implementation coming soon!".to_string(),
                    timestamp: std::time::SystemTime::now(),
                };
                this.messages.push(response_message);
                this.is_generating = false;
                cx.notify();
            }).ok();
        }).detach();
    }

    pub fn clear_chat(&mut self, cx: &mut Context<Self>) {
        self.messages.clear();
        cx.notify();
    }

    fn format_timestamp(&self, timestamp: std::time::SystemTime) -> String {
        match timestamp.duration_since(std::time::UNIX_EPOCH) {
            Ok(duration) => {
                let secs = duration.as_secs();
                let hours = (secs / 3600) % 24;
                let minutes = (secs / 60) % 60;
                format!("{:02}:{:02}", hours, minutes)
            }
            Err(_) => "??:??".to_string(),
        }
    }

    fn render_message(&self, message: &ChatMessage) -> impl IntoElement {
        let is_user = matches!(message.role, Role::User);
        let timestamp = self.format_timestamp(message.timestamp);
        
        div()
            .flex()
            .gap_3()
            .p_3()
            .when(is_user, |this| this.justify_end())
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .max_w_96()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .when(!is_user, |this| {
                                this.child(
                                    Icon::new(IconName::ZedAssistant)
                                        .size(ui::IconSize::Small)
                                        .color(Color::Accent)
                                )
                            })
                            .child(
                                Label::new(match message.role {
                                    Role::User => "You",
                                    Role::Assistant => "AI",
                                    Role::System => "System",
                                })
                                .size(LabelSize::Small)
                                .color(if is_user { Color::Default } else { Color::Accent })
                            )
                            .child(
                                Label::new(timestamp)
                                    .size(LabelSize::Small)
                                    .color(Color::Muted)
                            )
                    )
                    .child(
                        div()
                            .p_3()
                            .bg(if is_user {
                                // Use a fixed color instead of accessing theme through cx
                                gpui::rgb(0x2d2d2d)
                            } else {
                                gpui::rgb(0x1e1e1e)
                            })
                            .rounded_lg()
                            .when(is_user, |this| {
                                this.border_1()
                                    .border_color(gpui::rgb(0x3e3e3e))
                            })
                            .child(
                                Label::new(message.content.clone())
                                    .size(LabelSize::Default)
                            )
                    )
            )
    }

    fn render_input_area(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .gap_2()
            .p_4()
            .bg(cx.theme().colors().panel_background)
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .flex_1()
                    .child(self.input.clone())
            )
            .child(
                Button::new("send", "Send")
                    .style(ButtonStyle::Filled)
                    .icon(Some(IconName::Send))
                    .disabled(self.is_generating)
                    .on_click(cx.listener(|this, _, window, cx| {
                        let content = this.input.read(cx).editor.read(cx).text(cx).to_string();
                        this.send_message(content, window, cx);
                    }))
            )
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .justify_between()
            .items_center()
            .p_4()
            .bg(cx.theme().colors().panel_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::MessageBubbles)
                            .size(ui::IconSize::XLarge)
                    )
                    .child(
                        Label::new("AI Chat")
                            .size(LabelSize::Large)
                    )
                    .when_some(self.selected_model.as_ref(), |this, model| {
                        this.child(
                            Label::new(format!("• {}", model.name().0))
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                        )
                    })
            )
            .child(
                div()
                    .flex()
                    .gap_2()
                    .child(
                        Button::new("select_model", "Select Model")
                            .style(ButtonStyle::Subtle)
                            .icon(Some(IconName::Settings))
                            .on_click(cx.listener(|_this, _, _window, _cx| {
                                log::info!("Select model clicked");
                            }))
                    )
                    .child(
                        Button::new("clear", "Clear")
                            .style(ButtonStyle::Subtle)
                            .icon(Some(IconName::Trash))
                            .disabled(self.messages.is_empty())
                            .on_click(cx.listener(|this, _, _window, cx| {
                                this.clear_chat(cx);
                            }))
                    )
            )
    }

    fn render_messages(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex_1()
            .overflow_y_hidden()
            .child(
                if self.messages.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .size_full()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_2()
                                .child(
                                    Icon::new(IconName::MessageBubbles)
                                        .size(ui::IconSize::XLarge)
                                        .color(Color::Muted)
                                )
                                .child(
                                    Label::new("Start a conversation")
                                        .size(LabelSize::Large)
                                        .color(Color::Muted)
                                )
                                .child(
                                    Label::new("Type a message below to begin chatting with AI")
                                        .size(LabelSize::Small)
                                        .color(Color::Muted)
                                )
                        )
                } else {
                    let message_elements: Vec<_> = self.messages.iter().map(|message| {
                        self.render_message(message)
                    }).collect();

                    div()
                        .flex()
                        .flex_col()
                        .children(message_elements)
                        .when(self.is_generating, |this| {
                            this.child(
                                div()
                                    .flex()
                                    .gap_3()
                                    .p_3()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(
                                                Icon::new(IconName::ZedAssistant)
                                                    .size(ui::IconSize::Small)
                                                    .color(Color::Accent)
                                            )
                                            .child(
                                                Label::new("AI is typing...")
                                                    .size(LabelSize::Small)
                                                    .color(Color::Muted)
                                            )
                                    )
                            )
                        })
                }
            )
    }
}

impl Render for ChatInterface {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(cx.theme().colors().background)
            .child(self.render_header(cx))
            .child(self.render_messages(cx))
            .child(self.render_input_area(cx))
    }
} 