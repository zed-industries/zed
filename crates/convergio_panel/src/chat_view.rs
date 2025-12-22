//! Convergio Chat View - Custom chat UI for Convergio agents
//!
//! This component displays chat messages from convergio.db and handles
//! user input for sending messages to Convergio agents.

use crate::convergio_db::{ChatMessage, ConvergioDb, MessageType};
use chrono::{DateTime, Local, Utc};
use collections::HashMap;
use editor::Editor;
use gpui::{
    actions, div, prelude::*, px, rems, AbsoluteLength, App, BorderStyle, Context,
    DefiniteLength, EdgesRefinement, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, KeyContext, Length, ParentElement, Render, ScrollHandle,
    SharedString, Styled, StyleRefinement, Subscription, Task, TextStyleRefinement, WeakEntity,
    Window,
};
use language::LanguageRegistry;
use markdown::{HeadingLevelStyles, Markdown, MarkdownElement, MarkdownStyle};
use std::sync::Arc;
use std::time::Duration;
use theme::ThemeSettings;
use settings::Settings;
use ui::{prelude::*, Icon, IconName, Label, LabelSize};
use workspace::{
    item::{Item, ItemEvent},
    Workspace,
};

actions!(convergio_chat, [Send, Refresh]);

/// Custom chat view that reads from convergio.db
pub struct ConvergioChatView {
    focus_handle: FocusHandle,
    agent_name: SharedString,
    agent_display_name: SharedString,
    session_id: Option<String>,
    messages: Vec<ChatMessage>,
    message_markdowns: HashMap<i64, Entity<Markdown>>,
    input_editor: Entity<Editor>,
    scroll_handle: ScrollHandle,
    db: Option<Arc<ConvergioDb>>,
    language_registry: Option<Arc<LanguageRegistry>>,
    #[allow(dead_code)]
    workspace: WeakEntity<Workspace>,
    is_loading: bool,
    is_streaming: bool,
    last_message_count: i64,
    _input_subscription: Subscription,
    _poll_task: Option<Task<()>>,
}

impl ConvergioChatView {
    /// Create a new chat view for a Convergio agent
    pub fn new(
        agent_name: SharedString,
        agent_display_name: SharedString,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = cx.focus_handle();

        // Create input editor
        let input_editor = cx.new(|cx| {
            let mut editor = Editor::auto_height(1, 4, window, cx);
            editor.set_placeholder_text("Type a message...", window, cx);
            editor
        });

        // Subscribe to editor events for send on enter
        let subscription = cx.subscribe(&input_editor, |_this, _, event: &editor::EditorEvent, cx| {
            if let editor::EditorEvent::BufferEdited { .. } = event {
                // Could add typing indicator here
                cx.notify();
            }
        });

        // Try to open database
        let db = match ConvergioDb::open() {
            Ok(db) => Some(Arc::new(db)),
            Err(e) => {
                log::error!("Failed to open convergio database: {}", e);
                None
            }
        };

        // Get language registry for markdown syntax highlighting
        let language_registry = workspace.upgrade().map(|ws| {
            ws.read(cx).project().read(cx).languages().clone()
        });

        let mut view = Self {
            focus_handle,
            agent_name: agent_name.clone(),
            agent_display_name,
            session_id: None,
            messages: Vec::new(),
            message_markdowns: HashMap::default(),
            input_editor,
            scroll_handle: ScrollHandle::new(),
            db,
            language_registry,
            workspace,
            is_loading: true,
            is_streaming: false,
            last_message_count: 0,
            _input_subscription: subscription,
            _poll_task: None,
        };

        // Load initial messages
        view.load_latest_session(cx);

        // Start polling for updates
        view.start_polling(cx);

        view
    }

    /// Load the latest session for this agent from the database
    fn load_latest_session(&mut self, cx: &mut Context<Self>) {
        let Some(db) = self.db.clone() else {
            self.is_loading = false;
            cx.notify();
            return;
        };

        let agent_name = self.agent_name.clone();
        self.is_loading = true;
        cx.notify();

        cx.spawn(async move |this, cx| {
            // Query for latest session
            let result = cx.background_executor().spawn(async move {
                db.latest_session_for_agent(&agent_name)
            }).await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(Some(session_meta)) => {
                        this.session_id = Some(session_meta.session.id.clone());
                        this.load_messages_for_session(&session_meta.session.id, cx);
                    }
                    Ok(None) => {
                        log::info!("No existing session found for agent {}", this.agent_name);
                        this.is_loading = false;
                        cx.notify();
                    }
                    Err(e) => {
                        log::error!("Failed to load session: {}", e);
                        this.is_loading = false;
                        cx.notify();
                    }
                }
            }).ok();
        }).detach();
    }

    /// Load messages for a specific session
    fn load_messages_for_session(&mut self, session_id: &str, cx: &mut Context<Self>) {
        let Some(db) = self.db.clone() else {
            return;
        };

        let session_id = session_id.to_string();
        cx.spawn(async move |this, cx| {
            let result = cx.background_executor().spawn({
                let session_id = session_id.clone();
                async move {
                    db.messages_for_session(&session_id)
                }
            }).await;

            this.update(cx, |this, cx| {
                match result {
                    Ok(messages) => {
                        this.last_message_count = messages.len() as i64;
                        // Clear markdown cache for messages no longer present
                        this.message_markdowns.retain(|id, _| messages.iter().any(|m| m.id == *id));
                        this.messages = messages;
                        this.is_loading = false;
                        this.scroll_to_bottom(cx);
                        cx.notify();
                    }
                    Err(e) => {
                        log::error!("Failed to load messages: {}", e);
                        this.is_loading = false;
                        cx.notify();
                    }
                }
            }).ok();
        }).detach();
    }

    /// Start polling for new messages
    fn start_polling(&mut self, cx: &mut Context<Self>) {
        let task = cx.spawn(async move |this, cx| {
            loop {
                // Poll every 2 seconds
                cx.background_executor().timer(Duration::from_secs(2)).await;

                let should_refresh = this.update(cx, |this, _| {
                    this.session_id.is_some() && this.db.is_some()
                }).unwrap_or(false);

                if should_refresh {
                    let _ = this.update(cx, |this, cx| {
                        this.check_for_updates(cx);
                    });
                }
            }
        });

        self._poll_task = Some(task);
    }

    /// Check if there are new messages
    fn check_for_updates(&mut self, cx: &mut Context<Self>) {
        let Some(db) = self.db.clone() else {
            return;
        };

        let Some(session_id) = self.session_id.clone() else {
            return;
        };

        let last_count = self.last_message_count;

        cx.spawn(async move |this, cx| {
            let result = cx.background_executor().spawn({
                let session_id = session_id.clone();
                async move {
                    db.message_count(&session_id)
                }
            }).await;

            if let Ok(count) = result {
                if count > last_count {
                    // New messages available, reload
                    let _ = this.update(cx, |this, cx| {
                        this.load_messages_for_session(&session_id, cx);
                    });
                }
            }
        }).detach();
    }

    /// Refresh messages manually
    fn refresh(&mut self, _: &Refresh, _window: &mut Window, cx: &mut Context<Self>) {
        if let Some(session_id) = self.session_id.clone() {
            self.load_messages_for_session(&session_id, cx);
        } else {
            self.load_latest_session(cx);
        }
    }

    /// Send a message to the agent via database
    fn send(&mut self, _: &Send, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.input_editor.read(cx).text(cx);
        if content.trim().is_empty() {
            return;
        }

        // Clear input
        self.input_editor.update(cx, |editor, cx| {
            editor.clear(window, cx);
        });

        let Some(db) = self.db.clone() else {
            log::error!("Database not available for sending message");
            return;
        };

        let agent_name = self.agent_name.clone();
        let session_id = self.session_id.clone();

        // Insert message into database
        cx.spawn(async move |this, cx| {
            // Get or create session
            let session_id = match session_id {
                Some(id) => id,
                None => {
                    match db.get_or_create_session(&agent_name) {
                        Ok(id) => id,
                        Err(e) => {
                            log::error!("Failed to create session: {}", e);
                            return;
                        }
                    }
                }
            };

            // Insert the user message
            match db.insert_user_message(&session_id, &content) {
                Ok(msg_id) => {
                    log::info!("Inserted user message {} to session {}", msg_id, session_id);
                    // Update session ID and reload messages
                    let _ = this.update(cx, |this, cx| {
                        this.session_id = Some(session_id.clone());
                        this.is_streaming = true;
                        this.load_messages_for_session(&session_id, cx);
                    });
                }
                Err(e) => {
                    log::error!("Failed to insert message: {}", e);
                }
            }
        }).detach();

        cx.notify();
    }

    fn scroll_to_bottom(&mut self, _cx: &mut Context<Self>) {
        self.scroll_handle.scroll_to_bottom();
    }

    fn dispatch_context(&self, _window: &Window, _cx: &Context<Self>) -> KeyContext {
        let mut context = KeyContext::new_with_defaults();
        context.add("ConvergioChatView");
        context
    }

    /// Get or create a markdown entity for a message
    fn get_or_create_markdown(&mut self, message: &ChatMessage, cx: &mut Context<Self>) -> Entity<Markdown> {
        if let Some(md) = self.message_markdowns.get(&message.id) {
            return md.clone();
        }

        let content: SharedString = message.content.clone().into();
        let lang_registry = self.language_registry.clone();
        let markdown = cx.new(|cx| {
            Markdown::new(content, lang_registry, None, cx)
        });

        self.message_markdowns.insert(message.id, markdown.clone());
        markdown
    }

    /// Render a single chat message
    fn render_message(&mut self, message: &ChatMessage, window: &Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_user = message.message_type == MessageType::User;
        let is_system = message.message_type == MessageType::System;
        let is_tool = message.message_type == MessageType::Tool;

        let bg_color = if is_user {
            cx.theme().colors().element_selected
        } else if is_system || is_tool {
            cx.theme().colors().surface_background
        } else {
            cx.theme().colors().editor_background
        };

        let time_str = format_time(message.created_at);

        // Get or create markdown for this message
        let markdown = self.get_or_create_markdown(message, cx);
        let markdown_style = default_markdown_style(window, cx);

        let message_bubble = div()
            .max_w(rems(40.))
            .flex()
            .flex_col()
            .gap_1()
            .child(
                // Header with sender and time
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Label::new(
                            message.sender_name.clone().unwrap_or_else(|| {
                                if is_user { "You".to_string() } else { "Assistant".to_string() }
                            })
                        )
                        .size(LabelSize::XSmall)
                        .color(Color::Muted)
                    )
                    .child(
                        Label::new(time_str)
                            .size(LabelSize::XSmall)
                            .color(Color::Disabled)
                    )
            )
            .child(
                // Message content with markdown rendering
                div()
                    .px_3()
                    .py_2()
                    .rounded_lg()
                    .bg(bg_color)
                    .child(
                        MarkdownElement::new(markdown, markdown_style)
                    )
            )
            // Show token usage if available
            .when(message.input_tokens > 0 || message.output_tokens > 0, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            Label::new(format!(
                                "{}↓ {}↑",
                                message.input_tokens,
                                message.output_tokens
                            ))
                            .size(LabelSize::XSmall)
                            .color(Color::Disabled)
                        )
                        .when(message.cost_usd > 0.0, |this| {
                            this.child(
                                Label::new(format!("${:.4}", message.cost_usd))
                                    .size(LabelSize::XSmall)
                                    .color(Color::Disabled)
                            )
                        })
                )
            });

        // User messages aligned to the right, assistant to the left
        if is_user {
            div()
                .w_full()
                .flex()
                .justify_end()
                .px_3()
                .py_2()
                .child(message_bubble)
        } else {
            div()
                .w_full()
                .flex()
                .justify_start()
                .px_3()
                .py_2()
                .child(message_bubble)
        }
    }

    /// Render the input area
    fn render_input(&self, cx: &Context<Self>) -> impl IntoElement {
        div()
            .w_full()
            .flex()
            .flex_col()
            .border_t_1()
            .border_color(cx.theme().colors().border)
            .bg(cx.theme().colors().panel_background)
            .p_2()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_end()
                    .gap_2()
                    .child(
                        div()
                            .flex_1()
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .bg(cx.theme().colors().editor_background)
                            .child(self.input_editor.clone())
                    )
                    .child(
                        div()
                            .id("send-button")
                            .px_3()
                            .py_1()
                            .rounded_md()
                            .bg(cx.theme().colors().element_selected)
                            .cursor_pointer()
                            .hover(|this| this.bg(cx.theme().colors().element_hover))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.send(&Send, window, cx);
                            }))
                            .child(
                                Icon::new(IconName::Send)
                                    .size(IconSize::Small)
                                    .color(Color::Accent)
                            )
                    )
            )
            // Show streaming indicator when waiting for response
            .when(self.is_streaming, |this| {
                this.child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            Icon::new(IconName::ArrowCircle)
                                .size(IconSize::XSmall)
                                .color(Color::Muted)
                        )
                        .child(
                            Label::new("Waiting for response...")
                                .size(LabelSize::XSmall)
                                .color(Color::Muted)
                        )
                )
            })
    }

    /// Render empty state when no messages
    fn render_empty_state(&self, _cx: &Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Icon::new(IconName::Chat)
                    .size(IconSize::XLarge)
                    .color(Color::Muted)
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        Label::new(format!("Start a conversation with {}", self.agent_display_name))
                            .size(LabelSize::Large)
                            .color(Color::Default)
                    )
                    .child(
                        Label::new("Messages will appear here and sync with the CLI")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
    }

    /// Render loading state
    fn render_loading(&self, _cx: &Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Icon::new(IconName::ArrowCircle)
                            .size(IconSize::Medium)
                            .color(Color::Muted)
                    )
                    .child(
                        Label::new("Loading conversation...")
                            .size(LabelSize::Default)
                            .color(Color::Muted)
                    )
            )
    }

    /// Render database unavailable state
    fn render_db_unavailable(&self, _cx: &Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .child(
                Icon::new(IconName::Warning)
                    .size(IconSize::XLarge)
                    .color(Color::Warning)
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        Label::new("Convergio database not available")
                            .size(LabelSize::Large)
                            .color(Color::Warning)
                    )
                    .child(
                        Label::new("Make sure Convergio CLI is installed and has been run at least once")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                    )
            )
    }
}

impl Focusable for ConvergioChatView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<ItemEvent> for ConvergioChatView {}

impl Item for ConvergioChatView {
    type Event = ItemEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.agent_display_name.clone()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<ui::Icon> {
        Some(Icon::new(IconName::Chat))
    }

    fn to_item_events(_event: &Self::Event, _f: impl FnMut(ItemEvent)) {}
}

impl Render for ConvergioChatView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Check if database is available
        if self.db.is_none() {
            return div()
                .size_full()
                .child(self.render_db_unavailable(cx))
                .into_any_element();
        }

        div()
            .id("convergio-chat-view")
            .size_full()
            .flex()
            .flex_col()
            .bg(cx.theme().colors().panel_background)
            .track_focus(&self.focus_handle(cx))
            .key_context(self.dispatch_context(window, cx))
            .on_action(cx.listener(Self::send))
            .on_action(cx.listener(Self::refresh))
            // Header
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_3()
                    .py_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                Icon::new(IconName::ConvergioAli)
                                    .size(IconSize::Medium)
                                    .color(Color::Accent)
                            )
                            .child(
                                Label::new(self.agent_display_name.clone())
                                    .size(LabelSize::Default)
                                    .weight(gpui::FontWeight::MEDIUM)
                            )
                    )
                    .child(
                        div()
                            .id("refresh-btn")
                            .px_2()
                            .py_1()
                            .rounded_md()
                            .cursor_pointer()
                            .hover(|this| this.bg(cx.theme().colors().element_hover))
                            .on_click(cx.listener(|this, _, window, cx| {
                                this.refresh(&Refresh, window, cx);
                            }))
                            .child(
                                Icon::new(IconName::RotateCw)
                                    .size(IconSize::Small)
                                    .color(Color::Muted)
                            )
                    )
            )
            // Messages area with scrolling
            .child(
                div()
                    .id("messages-scroll-area")
                    .flex_1()
                    .overflow_scroll()
                    .track_scroll(&self.scroll_handle)
                    .child(
                        if self.is_loading {
                            self.render_loading(cx).into_any_element()
                        } else if self.messages.is_empty() {
                            self.render_empty_state(cx).into_any_element()
                        } else {
                            // Clone messages to avoid borrow issues with mutable self
                            let messages: Vec<_> = self.messages.clone();
                            let rendered: Vec<_> = messages.iter()
                                .map(|msg| self.render_message(msg, window, cx).into_any_element())
                                .collect();
                            div()
                                .flex()
                                .flex_col()
                                .py_2()
                                .children(rendered)
                                .into_any_element()
                        }
                    )
            )
            // Input area
            .child(self.render_input(cx))
            .into_any_element()
    }
}

/// Format a UTC datetime for display in local time
fn format_time(dt: DateTime<Utc>) -> String {
    let local: DateTime<Local> = dt.into();
    local.format("%H:%M").to_string()
}

/// Create default markdown style for chat messages
fn default_markdown_style(window: &Window, cx: &App) -> MarkdownStyle {
    let theme_settings = ThemeSettings::get_global(cx);
    let colors = cx.theme().colors();

    let ui_font_size = theme_settings.ui_font_size(cx);
    let line_height = ui_font_size * 1.5;

    let mut text_style = window.text_style();
    text_style.refine(&TextStyleRefinement {
        font_family: Some(theme_settings.ui_font.family.clone()),
        font_fallbacks: theme_settings.ui_font.fallbacks.clone(),
        font_features: Some(theme_settings.ui_font.features.clone()),
        font_size: Some(ui_font_size.into()),
        line_height: Some(line_height.into()),
        color: Some(colors.text),
        ..Default::default()
    });

    MarkdownStyle {
        base_text_style: text_style,
        syntax: cx.theme().syntax().clone(),
        selection_background_color: colors.element_selection_background,
        code_block_overflow_x_scroll: true,
        heading_level_styles: Some(HeadingLevelStyles {
            h1: Some(TextStyleRefinement {
                font_size: Some(rems(1.15).into()),
                ..Default::default()
            }),
            h2: Some(TextStyleRefinement {
                font_size: Some(rems(1.1).into()),
                ..Default::default()
            }),
            h3: Some(TextStyleRefinement {
                font_size: Some(rems(1.05).into()),
                ..Default::default()
            }),
            h4: Some(TextStyleRefinement {
                font_size: Some(rems(1.).into()),
                ..Default::default()
            }),
            h5: Some(TextStyleRefinement {
                font_size: Some(rems(0.95).into()),
                ..Default::default()
            }),
            h6: Some(TextStyleRefinement {
                font_size: Some(rems(0.875).into()),
                ..Default::default()
            }),
        }),
        code_block: StyleRefinement {
            padding: EdgesRefinement {
                top: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                left: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                right: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
                bottom: Some(DefiniteLength::Absolute(AbsoluteLength::Pixels(px(8.)))),
            },
            margin: EdgesRefinement {
                top: Some(Length::Definite(px(8.).into())),
                left: Some(Length::Definite(px(0.).into())),
                right: Some(Length::Definite(px(0.).into())),
                bottom: Some(Length::Definite(px(8.).into())),
            },
            border_style: Some(BorderStyle::Solid),
            border_widths: EdgesRefinement {
                top: Some(AbsoluteLength::Pixels(px(1.))),
                left: Some(AbsoluteLength::Pixels(px(1.))),
                right: Some(AbsoluteLength::Pixels(px(1.))),
                bottom: Some(AbsoluteLength::Pixels(px(1.))),
            },
            border_color: Some(colors.border_variant),
            background: Some(colors.editor_background.into()),
            text: TextStyleRefinement {
                font_family: Some(theme_settings.buffer_font.family.clone()),
                font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
                font_features: Some(theme_settings.buffer_font.features.clone()),
                font_size: Some(theme_settings.buffer_font_size(cx).into()),
                ..Default::default()
            },
            ..Default::default()
        },
        inline_code: TextStyleRefinement {
            font_family: Some(theme_settings.buffer_font.family.clone()),
            font_fallbacks: theme_settings.buffer_font.fallbacks.clone(),
            background_color: Some(colors.surface_background),
            ..Default::default()
        },
        block_quote: TextStyleRefinement {
            color: Some(colors.text_muted),
            ..Default::default()
        },
        link: TextStyleRefinement {
            color: Some(colors.link_text_hover),
            underline: Some(gpui::UnderlineStyle {
                thickness: px(1.),
                color: Some(colors.link_text_hover),
                wavy: false,
            }),
            ..Default::default()
        },
        rule_color: colors.border,
        block_quote_border_color: colors.border_variant,
        ..Default::default()
    }
}

pub fn init(_cx: &mut App) {
    // Register actions
}
