mod assistant_settings;
mod attachments;
mod completion_provider;
mod saved_conversation;
mod saved_conversations;
mod tools;
pub mod ui;

use crate::saved_conversation::SavedConversationMetadata;
use crate::ui::UserOrAssistant;
use ::ui::{div, prelude::*, Color, Tooltip, ViewContext};
use anyhow::{Context, Result};
use assistant_tooling::{
    AttachmentRegistry, ProjectContext, ToolFunctionCall, ToolRegistry, UserAttachment,
};
use attachments::ActiveEditorAttachmentTool;
use client::{proto, Client, UserStore};
use collections::HashMap;
use completion_provider::*;
use editor::Editor;
use feature_flags::FeatureFlagAppExt as _;
use file_icons::FileIcons;
use fs::Fs;
use futures::{future::join_all, StreamExt};
use gpui::{
    list, AnyElement, AppContext, AsyncWindowContext, ClickEvent, EventEmitter, FocusHandle,
    FocusableView, ListAlignment, ListState, Model, ReadGlobal, Render, Task, UpdateGlobal, View,
    WeakView,
};
use language::{language_settings::SoftWrap, LanguageRegistry};
use markdown::{Markdown, MarkdownStyle};
use open_ai::{FunctionContent, ToolCall, ToolCallContent};
use saved_conversation::{SavedAssistantMessagePart, SavedChatMessage, SavedConversation};
use saved_conversations::SavedConversations;
use semantic_index::{CloudEmbeddingProvider, ProjectIndex, ProjectIndexDebugView, SemanticIndex};
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::sync::Arc;
use tools::{AnnotationTool, CreateBufferTool, ProjectIndexTool};
use ui::{ActiveFileButton, Composer, ProjectIndexButton};
use util::paths::CONVERSATIONS_DIR;
use util::{maybe, paths::EMBEDDINGS_DIR, ResultExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

pub use assistant_settings::AssistantSettings;

const MAX_COMPLETION_CALLS_PER_SUBMISSION: usize = 5;

#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
pub struct Submit(SubmitMode);

/// There are multiple different ways to submit a model request, represented by this enum.
#[derive(Eq, PartialEq, Copy, Clone, Deserialize)]
pub enum SubmitMode {
    /// Only include the conversation.
    Simple,
    /// Send the current file as context.
    CurrentFile,
    /// Search the codebase and send relevant excerpts.
    Codebase,
}

gpui::actions!(assistant2, [Cancel, ToggleFocus, DebugProjectIndex,]);
gpui::impl_actions!(assistant2, [Submit]);

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    AssistantSettings::register(cx);

    cx.spawn(|mut cx| {
        let client = client.clone();
        async move {
            let embedding_provider = CloudEmbeddingProvider::new(client.clone());
            let semantic_index = SemanticIndex::new(
                EMBEDDINGS_DIR.join("semantic-index-db.0.mdb"),
                Arc::new(embedding_provider),
                &mut cx,
            )
            .await?;
            cx.update(|cx| cx.set_global(semantic_index))
        }
    })
    .detach();

    cx.set_global(CompletionProvider::new(CloudCompletionProvider::new(
        client,
    )));

    cx.observe_new_views(
        |workspace: &mut Workspace, _cx: &mut ViewContext<Workspace>| {
            workspace.register_action(|workspace, _: &ToggleFocus, cx| {
                workspace.toggle_panel_focus::<AssistantPanel>(cx);
            });
            workspace.register_action(|workspace, _: &DebugProjectIndex, cx| {
                if let Some(panel) = workspace.panel::<AssistantPanel>(cx) {
                    let index = panel.read(cx).chat.read(cx).project_index.clone();
                    let view = cx.new_view(|cx| ProjectIndexDebugView::new(index, cx));
                    workspace.add_item_to_center(Box::new(view), cx);
                }
            });
        },
    )
    .detach();
}

pub fn enabled(cx: &AppContext) -> bool {
    cx.is_staff()
}

pub struct AssistantPanel {
    chat: View<AssistantChat>,
    width: Option<Pixels>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let (app_state, project) = workspace.update(&mut cx, |workspace, _| {
                (workspace.app_state().clone(), workspace.project().clone())
            })?;

            cx.new_view(|cx| {
                let project_index = SemanticIndex::update_global(cx, |semantic_index, cx| {
                    semantic_index.project_index(project.clone(), cx)
                });

                // Used in tools to render file icons
                cx.observe_global::<FileIcons>(|_, cx| {
                    cx.notify();
                })
                .detach();

                let mut tool_registry = ToolRegistry::new();
                tool_registry
                    .register(ProjectIndexTool::new(project_index.clone()))
                    .unwrap();
                tool_registry
                    .register(CreateBufferTool::new(workspace.clone(), project.clone()))
                    .unwrap();
                tool_registry
                    .register(AnnotationTool::new(workspace.clone(), project.clone()))
                    .unwrap();

                let mut attachment_registry = AttachmentRegistry::new();
                attachment_registry
                    .register(ActiveEditorAttachmentTool::new(workspace.clone(), cx));

                Self::new(
                    project.read(cx).fs().clone(),
                    app_state.languages.clone(),
                    Arc::new(tool_registry),
                    Arc::new(attachment_registry),
                    app_state.user_store.clone(),
                    project_index,
                    workspace,
                    cx,
                )
            })
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
        attachment_registry: Arc<AttachmentRegistry>,
        user_store: Model<UserStore>,
        project_index: Model<ProjectIndex>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let chat = cx.new_view(|cx| {
            AssistantChat::new(
                fs,
                language_registry,
                tool_registry.clone(),
                attachment_registry,
                user_store,
                project_index,
                workspace,
                cx,
            )
        });

        Self { width: None, chat }
    }
}

impl Render for AssistantPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .size_full()
            .v_flex()
            .bg(cx.theme().colors().panel_background)
            .child(self.chat.clone())
    }
}

impl Panel for AssistantPanel {
    fn persistent_name() -> &'static str {
        "AssistantPanelv2"
    }

    fn position(&self, _cx: &WindowContext) -> workspace::dock::DockPosition {
        // todo!("Add a setting / use assistant settings")
        DockPosition::Right
    }

    fn position_is_valid(&self, position: workspace::dock::DockPosition) -> bool {
        matches!(position, DockPosition::Right)
    }

    fn set_position(&mut self, _: workspace::dock::DockPosition, _: &mut ViewContext<Self>) {
        // Do nothing until we have a setting for this
    }

    fn size(&self, _cx: &WindowContext) -> Pixels {
        self.width.unwrap_or(px(400.))
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        cx.notify();
    }

    fn icon(&self, _cx: &WindowContext) -> Option<::ui::IconName> {
        Some(IconName::ZedAssistant)
    }

    fn icon_tooltip(&self, _: &WindowContext) -> Option<&'static str> {
        Some("Assistant Panel ✨")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for AssistantPanel {}

impl FocusableView for AssistantPanel {
    fn focus_handle(&self, cx: &AppContext) -> FocusHandle {
        self.chat.read(cx).composer_editor.read(cx).focus_handle(cx)
    }
}

pub struct AssistantChat {
    model: String,
    messages: Vec<ChatMessage>,
    list_state: ListState,
    fs: Arc<dyn Fs>,
    language_registry: Arc<LanguageRegistry>,
    composer_editor: View<Editor>,
    saved_conversations: View<SavedConversations>,
    saved_conversations_open: bool,
    project_index_button: View<ProjectIndexButton>,
    active_file_button: Option<View<ActiveFileButton>>,
    user_store: Model<UserStore>,
    next_message_id: MessageId,
    collapsed_messages: HashMap<MessageId, bool>,
    editing_message: Option<EditingMessage>,
    pending_completion: Option<Task<()>>,
    tool_registry: Arc<ToolRegistry>,
    attachment_registry: Arc<AttachmentRegistry>,
    project_index: Model<ProjectIndex>,
    markdown_style: MarkdownStyle,
}

struct EditingMessage {
    id: MessageId,
    body: View<Editor>,
}

impl AssistantChat {
    #[allow(clippy::too_many_arguments)]
    fn new(
        fs: Arc<dyn Fs>,
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
        attachment_registry: Arc<AttachmentRegistry>,
        user_store: Model<UserStore>,
        project_index: Model<ProjectIndex>,
        workspace: WeakView<Workspace>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let model = CompletionProvider::global(cx).default_model();
        let view = cx.view().downgrade();
        let list_state = ListState::new(
            0,
            ListAlignment::Bottom,
            px(1024.),
            move |ix, cx: &mut WindowContext| {
                view.update(cx, |this, cx| this.render_message(ix, cx))
                    .unwrap()
            },
        );

        let project_index_button = cx.new_view(|cx| {
            ProjectIndexButton::new(project_index.clone(), tool_registry.clone(), cx)
        });

        let active_file_button = match workspace.upgrade() {
            Some(workspace) => {
                Some(cx.new_view(
                    |cx| ActiveFileButton::new(attachment_registry.clone(), workspace, cx), //
                ))
            }
            _ => None,
        };

        let saved_conversations = cx.new_view(|cx| SavedConversations::new(cx));
        cx.spawn({
            let fs = fs.clone();
            let saved_conversations = saved_conversations.downgrade();
            |_assistant_chat, mut cx| async move {
                let saved_conversation_metadata = SavedConversationMetadata::list(fs).await?;

                cx.update(|cx| {
                    saved_conversations.update(cx, |this, cx| {
                        this.init(saved_conversation_metadata, cx);
                    })
                })??;

                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);

        Self {
            model,
            messages: Vec::new(),
            composer_editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                editor.set_placeholder_text("Send a message…", cx);
                editor
            }),
            saved_conversations,
            saved_conversations_open: false,
            list_state,
            user_store,
            fs,
            language_registry,
            project_index_button,
            active_file_button,
            project_index,
            next_message_id: MessageId(0),
            editing_message: None,
            collapsed_messages: HashMap::default(),
            pending_completion: None,
            attachment_registry,
            tool_registry,
            markdown_style: MarkdownStyle {
                code_block: gpui::TextStyleRefinement {
                    font_family: Some("Zed Mono".into()),
                    color: Some(cx.theme().colors().editor_foreground),
                    background_color: Some(cx.theme().colors().editor_background),
                    ..Default::default()
                },
                inline_code: gpui::TextStyleRefinement {
                    font_family: Some("Zed Mono".into()),
                    // @nate: Could we add inline-code specific styles to the theme?
                    color: Some(cx.theme().colors().editor_foreground),
                    background_color: Some(cx.theme().colors().editor_background),
                    ..Default::default()
                },
                rule_color: Color::Muted.color(cx),
                block_quote_border_color: Color::Muted.color(cx),
                block_quote: gpui::TextStyleRefinement {
                    color: Some(Color::Muted.color(cx)),
                    ..Default::default()
                },
                link: gpui::TextStyleRefinement {
                    color: Some(Color::Accent.color(cx)),
                    underline: Some(gpui::UnderlineStyle {
                        thickness: px(1.),
                        color: Some(Color::Accent.color(cx)),
                        wavy: false,
                    }),
                    ..Default::default()
                },
                syntax: cx.theme().syntax().clone(),
                selection_background_color: {
                    let mut selection = cx.theme().players().local().selection;
                    selection.fade_out(0.7);
                    selection
                },
            },
        }
    }

    fn message_for_id(&self, id: MessageId) -> Option<&ChatMessage> {
        self.messages.iter().find(|message| match message {
            ChatMessage::User(message) => message.id == id,
            ChatMessage::Assistant(message) => message.id == id,
        })
    }

    fn toggle_saved_conversations(&mut self) {
        self.saved_conversations_open = !self.saved_conversations_open;
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        // If we're currently editing a message, cancel the edit.
        if self.editing_message.take().is_some() {
            cx.notify();
            return;
        }

        if self.pending_completion.take().is_some() {
            if let Some(ChatMessage::Assistant(grouping)) = self.messages.last() {
                if grouping.messages.is_empty() {
                    self.pop_message(cx);
                }
            }
            return;
        }

        cx.propagate();
    }

    fn submit(&mut self, Submit(mode): &Submit, cx: &mut ViewContext<Self>) {
        if self.composer_editor.focus_handle(cx).is_focused(cx) {
            // Don't allow multiple concurrent completions.
            if self.pending_completion.is_some() {
                cx.propagate();
                return;
            }

            let message = self.composer_editor.update(cx, |composer_editor, cx| {
                let text = composer_editor.text(cx);
                let id = self.next_message_id.post_inc();
                let body = cx.new_view(|cx| {
                    Markdown::new(
                        text,
                        self.markdown_style.clone(),
                        Some(self.language_registry.clone()),
                        cx,
                    )
                });
                composer_editor.clear(cx);

                ChatMessage::User(UserMessage {
                    id,
                    body,
                    attachments: Vec::new(),
                })
            });
            self.push_message(message, cx);
        } else if let Some(editing_message) = self.editing_message.as_ref() {
            let focus_handle = editing_message.body.focus_handle(cx);
            if focus_handle.contains_focused(cx) {
                if let Some(ChatMessage::User(user_message)) =
                    self.message_for_id(editing_message.id)
                {
                    user_message.body.update(cx, |body, cx| {
                        body.reset(editing_message.body.read(cx).text(cx), cx);
                    });
                }

                self.truncate_messages(editing_message.id, cx);

                self.pending_completion.take();
                self.composer_editor.focus_handle(cx).focus(cx);
                self.editing_message.take();
            } else {
                log::error!("unexpected state: no user message editor is focused.");
                return;
            }
        } else {
            log::error!("unexpected state: no user message editor is focused.");
            return;
        }

        let mode = *mode;
        self.pending_completion = Some(cx.spawn(move |this, mut cx| async move {
            let attachments_task = this.update(&mut cx, |this, cx| {
                let attachment_registry = this.attachment_registry.clone();
                attachment_registry.call_all_attachment_tools(cx)
            });

            let attachments = maybe!(async {
                let attachments_task = attachments_task?;
                let attachments = attachments_task.await?;

                anyhow::Ok(attachments)
            })
            .await
            .log_err()
            .unwrap_or_default();

            // Set the attachments to the _last_ user message
            this.update(&mut cx, |this, _cx| {
                if let Some(ChatMessage::User(message)) = this.messages.last_mut() {
                    message.attachments = attachments;
                }
            })
            .log_err();

            Self::request_completion(
                this.clone(),
                mode,
                MAX_COMPLETION_CALLS_PER_SUBMISSION,
                &mut cx,
            )
            .await
            .log_err();

            this.update(&mut cx, |this, _cx| {
                this.pending_completion = None;
            })
            .context("Failed to push new user message")
            .log_err();
        }));
    }

    async fn request_completion(
        this: WeakView<Self>,
        mode: SubmitMode,
        limit: usize,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let mut call_count = 0;
        loop {
            let complete = async {
                let (tool_definitions, model_name, messages) = this.update(cx, |this, cx| {
                    this.push_new_assistant_message(cx);

                    let definitions = if call_count < limit
                        && matches!(mode, SubmitMode::Codebase | SubmitMode::Simple)
                    {
                        this.tool_registry.definitions()
                    } else {
                        Vec::new()
                    };
                    call_count += 1;

                    (
                        definitions,
                        this.model.clone(),
                        this.completion_messages(cx),
                    )
                })?;

                let messages = messages.await?;

                let completion = cx.update(|cx| {
                    CompletionProvider::global(cx).complete(
                        model_name,
                        messages,
                        Vec::new(),
                        1.0,
                        tool_definitions,
                    )
                });

                let mut stream = completion?.await?;
                while let Some(delta) = stream.next().await {
                    let delta = delta?;
                    this.update(cx, |this, cx| {
                        if let Some(ChatMessage::Assistant(AssistantMessage { messages, .. })) =
                            this.messages.last_mut()
                        {
                            if messages.is_empty() {
                                messages.push(AssistantMessagePart {
                                    body: cx.new_view(|cx| {
                                        Markdown::new(
                                            "".into(),
                                            this.markdown_style.clone(),
                                            Some(this.language_registry.clone()),
                                            cx,
                                        )
                                    }),
                                    tool_calls: Vec::new(),
                                })
                            }

                            let message = messages.last_mut().unwrap();

                            if let Some(content) = &delta.content {
                                message
                                    .body
                                    .update(cx, |message, cx| message.append(&content, cx));
                            }

                            for tool_call_delta in delta.tool_calls {
                                let index = tool_call_delta.index as usize;
                                if index >= message.tool_calls.len() {
                                    message.tool_calls.resize_with(index + 1, Default::default);
                                }
                                let tool_call = &mut message.tool_calls[index];

                                if let Some(id) = &tool_call_delta.id {
                                    tool_call.id.push_str(id);
                                }

                                match tool_call_delta.variant {
                                    Some(proto::tool_call_delta::Variant::Function(
                                        tool_call_delta,
                                    )) => {
                                        this.tool_registry.update_tool_call(
                                            tool_call,
                                            tool_call_delta.name.as_deref(),
                                            tool_call_delta.arguments.as_deref(),
                                            cx,
                                        );
                                    }
                                    None => {}
                                }
                            }

                            cx.notify();
                        } else {
                            unreachable!()
                        }
                    })?;
                }

                anyhow::Ok(())
            }
            .await;

            let mut tool_tasks = Vec::new();
            this.update(cx, |this, cx| {
                if let Some(ChatMessage::Assistant(AssistantMessage {
                    error: message_error,
                    messages,
                    ..
                })) = this.messages.last_mut()
                {
                    if let Err(error) = complete {
                        message_error.replace(SharedString::from(error.to_string()));
                        cx.notify();
                    } else {
                        if let Some(current_message) = messages.last_mut() {
                            for tool_call in current_message.tool_calls.iter_mut() {
                                tool_tasks
                                    .extend(this.tool_registry.execute_tool_call(tool_call, cx));
                            }
                        }
                    }
                }
            })?;

            // This ends recursion on calling for responses after tools
            if tool_tasks.is_empty() {
                return Ok(());
            }

            join_all(tool_tasks.into_iter()).await;
        }
    }

    fn push_new_assistant_message(&mut self, cx: &mut ViewContext<Self>) {
        // If the last message is a grouped assistant message, add to the grouped message
        if let Some(ChatMessage::Assistant(AssistantMessage { messages, .. })) =
            self.messages.last_mut()
        {
            messages.push(AssistantMessagePart {
                body: cx.new_view(|cx| {
                    Markdown::new(
                        "".into(),
                        self.markdown_style.clone(),
                        Some(self.language_registry.clone()),
                        cx,
                    )
                }),
                tool_calls: Vec::new(),
            });
            return;
        }

        let message = ChatMessage::Assistant(AssistantMessage {
            id: self.next_message_id.post_inc(),
            messages: vec![AssistantMessagePart {
                body: cx.new_view(|cx| {
                    Markdown::new(
                        "".into(),
                        self.markdown_style.clone(),
                        Some(self.language_registry.clone()),
                        cx,
                    )
                }),
                tool_calls: Vec::new(),
            }],
            error: None,
        });
        self.push_message(message, cx);
    }

    fn push_message(&mut self, message: ChatMessage, cx: &mut ViewContext<Self>) {
        let old_len = self.messages.len();
        let focus_handle = Some(message.focus_handle(cx));
        self.messages.push(message);
        self.list_state
            .splice_focusable(old_len..old_len, focus_handle);
        cx.notify();
    }

    fn pop_message(&mut self, cx: &mut ViewContext<Self>) {
        if self.messages.is_empty() {
            return;
        }

        self.messages.pop();
        self.list_state
            .splice(self.messages.len()..self.messages.len() + 1, 0);
        cx.notify();
    }

    fn truncate_messages(&mut self, last_message_id: MessageId, cx: &mut ViewContext<Self>) {
        if let Some(index) = self.messages.iter().position(|message| match message {
            ChatMessage::User(message) => message.id == last_message_id,
            ChatMessage::Assistant(message) => message.id == last_message_id,
        }) {
            self.list_state.splice(index + 1..self.messages.len(), 0);
            self.messages.truncate(index + 1);
            cx.notify();
        }
    }

    fn is_message_collapsed(&self, id: &MessageId) -> bool {
        self.collapsed_messages.get(id).copied().unwrap_or_default()
    }

    fn toggle_message_collapsed(&mut self, id: MessageId) {
        let entry = self.collapsed_messages.entry(id).or_insert(false);
        *entry = !*entry;
    }

    fn reset(&mut self) {
        self.messages.clear();
        self.list_state.reset(0);
        self.editing_message.take();
        self.collapsed_messages.clear();
    }

    fn new_conversation(&mut self, cx: &mut ViewContext<Self>) {
        let messages = std::mem::take(&mut self.messages)
            .into_iter()
            .map(|message| self.serialize_message(message, cx))
            .collect::<Vec<_>>();

        self.reset();

        let title = messages
            .first()
            .map(|message| match message {
                SavedChatMessage::User { body, .. } => body.clone(),
                SavedChatMessage::Assistant { messages, .. } => messages
                    .first()
                    .map(|message| message.body.to_string())
                    .unwrap_or_default(),
            })
            .unwrap_or_else(|| "A conversation with the assistant.".to_string());

        let saved_conversation = SavedConversation {
            version: "0.3.0".to_string(),
            title,
            messages,
        };

        let discriminant = 1;

        let path = CONVERSATIONS_DIR.join(&format!(
            "{title} - {discriminant}.zed.{version}.json",
            title = saved_conversation.title,
            version = saved_conversation.version
        ));

        cx.spawn({
            let fs = self.fs.clone();
            |_this, _cx| async move {
                fs.create_dir(CONVERSATIONS_DIR.as_ref()).await?;
                fs.atomic_write(path, serde_json::to_string(&saved_conversation)?)
                    .await?;

                anyhow::Ok(())
            }
        })
        .detach_and_log_err(cx);
    }

    fn render_error(
        &self,
        error: Option<SharedString>,
        _ix: usize,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement {
        let theme = cx.theme();

        if let Some(error) = error {
            div()
                .py_1()
                .px_2()
                .mx_neg_1()
                .rounded_md()
                .border_1()
                .border_color(theme.status().error_border)
                // .bg(theme.status().error_background)
                .text_color(theme.status().error)
                .child(error.clone())
                .into_any_element()
        } else {
            div().into_any_element()
        }
    }

    fn render_message(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement {
        let is_first = ix == 0;
        let is_last = ix == self.messages.len().saturating_sub(1);

        let padding = Spacing::Large.rems(cx);

        // Whenever there's a run of assistant messages, group as one Assistant UI element

        match &self.messages[ix] {
            ChatMessage::User(UserMessage {
                id,
                body,
                attachments,
            }) => div()
                .id(SharedString::from(format!("message-{}-container", id.0)))
                .when(is_first, |this| this.pt(padding))
                .map(|element| {
                    if let Some(editing_message) = self.editing_message.as_ref() {
                        if editing_message.id == *id {
                            return element.child(Composer::new(
                                editing_message.body.clone(),
                                self.project_index_button.clone(),
                                self.active_file_button.clone(),
                                crate::ui::ModelSelector::new(
                                    cx.view().downgrade(),
                                    self.model.clone(),
                                )
                                .into_any_element(),
                            ));
                        }
                    }

                    element
                        .on_click(cx.listener({
                            let id = *id;
                            let body = body.clone();
                            move |assistant_chat, event: &ClickEvent, cx| {
                                if event.up.click_count == 2 {
                                    let body = cx.new_view(|cx| {
                                        let mut editor = Editor::auto_height(80, cx);
                                        let source = Arc::from(body.read(cx).source());
                                        editor.set_text(source, cx);
                                        editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                                        editor
                                    });
                                    assistant_chat.editing_message = Some(EditingMessage {
                                        id,
                                        body: body.clone(),
                                    });
                                    body.focus_handle(cx).focus(cx);
                                }
                            }
                        }))
                        .child(
                            crate::ui::ChatMessage::new(
                                *id,
                                UserOrAssistant::User(self.user_store.read(cx).current_user()),
                                // todo!(): clean up the vec usage
                                vec![
                                    body.clone().into_any_element(),
                                    h_flex()
                                        .gap_2()
                                        .children(
                                            attachments
                                                .iter()
                                                .map(|attachment| attachment.view.clone()),
                                        )
                                        .into_any_element(),
                                ],
                                self.is_message_collapsed(id),
                                Box::new(cx.listener({
                                    let id = *id;
                                    move |assistant_chat, _event, _cx| {
                                        assistant_chat.toggle_message_collapsed(id)
                                    }
                                })),
                            )
                            // TODO: Wire up selections.
                            .selected(is_last),
                        )
                })
                .into_any(),
            ChatMessage::Assistant(AssistantMessage {
                id,
                messages,
                error,
                ..
            }) => {
                let mut message_elements = Vec::new();

                for message in messages {
                    if !message.body.read(cx).source().is_empty() {
                        message_elements.push(div().child(message.body.clone()).into_any())
                    }

                    let tools = message
                        .tool_calls
                        .iter()
                        .filter_map(|tool_call| self.tool_registry.render_tool_call(tool_call, cx))
                        .collect::<Vec<AnyElement>>();

                    if !tools.is_empty() {
                        message_elements.push(div().children(tools).into_any())
                    }
                }

                if message_elements.is_empty() {
                    message_elements.push(::ui::Label::new("Researching...").into_any_element())
                }

                div()
                    .when(is_first, |this| this.pt(padding))
                    .child(
                        crate::ui::ChatMessage::new(
                            *id,
                            UserOrAssistant::Assistant,
                            message_elements,
                            self.is_message_collapsed(id),
                            Box::new(cx.listener({
                                let id = *id;
                                move |assistant_chat, _event, _cx| {
                                    assistant_chat.toggle_message_collapsed(id)
                                }
                            })),
                        )
                        // TODO: Wire up selections.
                        .selected(is_last),
                    )
                    .child(self.render_error(error.clone(), ix, cx))
                    .into_any()
            }
        }
    }

    fn completion_messages(&self, cx: &mut WindowContext) -> Task<Result<Vec<CompletionMessage>>> {
        let project_index = self.project_index.read(cx);
        let project = project_index.project();
        let fs = project_index.fs();

        let mut project_context = ProjectContext::new(project, fs);
        let mut completion_messages = Vec::new();

        for message in &self.messages {
            match message {
                ChatMessage::User(UserMessage {
                    body, attachments, ..
                }) => {
                    for attachment in attachments {
                        if let Some(content) = attachment.generate(&mut project_context, cx) {
                            completion_messages.push(CompletionMessage::System { content });
                        }
                    }

                    // Show user's message last so that the assistant is grounded in the user's request
                    completion_messages.push(CompletionMessage::User {
                        content: body.read(cx).source().to_string(),
                    });
                }
                ChatMessage::Assistant(AssistantMessage { messages, .. }) => {
                    for message in messages {
                        let body = message.body.clone();

                        if body.read(cx).source().is_empty() && message.tool_calls.is_empty() {
                            continue;
                        }

                        let tool_calls_from_assistant = message
                            .tool_calls
                            .iter()
                            .map(|tool_call| ToolCall {
                                content: ToolCallContent::Function {
                                    function: FunctionContent {
                                        name: tool_call.name.clone(),
                                        arguments: tool_call.arguments.clone(),
                                    },
                                },
                                id: tool_call.id.clone(),
                            })
                            .collect();

                        completion_messages.push(CompletionMessage::Assistant {
                            content: Some(body.read(cx).source().to_string()),
                            tool_calls: tool_calls_from_assistant,
                        });

                        for tool_call in &message.tool_calls {
                            // Every tool call _must_ have a result by ID, otherwise OpenAI will error.
                            let content = self.tool_registry.content_for_tool_call(
                                tool_call,
                                &mut project_context,
                                cx,
                            );
                            completion_messages.push(CompletionMessage::Tool {
                                content,
                                tool_call_id: tool_call.id.clone(),
                            });
                        }
                    }
                }
            }
        }

        let system_message = project_context.generate_system_message(cx);

        cx.background_executor().spawn(async move {
            let content = system_message.await?;
            completion_messages.insert(0, CompletionMessage::System { content });
            Ok(completion_messages)
        })
    }

    fn serialize_message(
        &self,
        message: ChatMessage,
        cx: &mut ViewContext<AssistantChat>,
    ) -> SavedChatMessage {
        match message {
            ChatMessage::User(message) => SavedChatMessage::User {
                id: message.id,
                body: message.body.read(cx).source().into(),
                attachments: message
                    .attachments
                    .iter()
                    .map(|attachment| {
                        self.attachment_registry
                            .serialize_user_attachment(attachment)
                    })
                    .collect(),
            },
            ChatMessage::Assistant(message) => SavedChatMessage::Assistant {
                id: message.id,
                error: message.error,
                messages: message
                    .messages
                    .iter()
                    .map(|message| SavedAssistantMessagePart {
                        body: message.body.read(cx).source().to_string().into(),
                        tool_calls: message
                            .tool_calls
                            .iter()
                            .filter_map(|tool_call| {
                                self.tool_registry
                                    .serialize_tool_call(tool_call, cx)
                                    .log_err()
                            })
                            .collect(),
                    })
                    .collect(),
            },
        }
    }
}

impl Render for AssistantChat {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let header_height = Spacing::Small.rems(cx) * 2.0 + ButtonSize::Default.rems();

        div()
            .relative()
            .flex_1()
            .v_flex()
            .key_context("AssistantChat")
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::cancel))
            .text_color(Color::Default.color(cx))
            .child(list(self.list_state.clone()).flex_1().pt(header_height))
            .child(
                h_flex()
                    .absolute()
                    .top_0()
                    .justify_between()
                    .w_full()
                    .h(header_height)
                    .p(Spacing::Small.rems(cx))
                    .child(
                        IconButton::new(
                            "toggle-saved-conversations",
                            if self.saved_conversations_open {
                                IconName::ChevronRight
                            } else {
                                IconName::ChevronLeft
                            },
                        )
                        .on_click(cx.listener(|this, _event, _cx| {
                            this.toggle_saved_conversations();
                        }))
                        .tooltip(move |cx| Tooltip::text("Switch Conversations", cx)),
                    )
                    .child(
                        h_flex()
                            .gap(Spacing::Large.rems(cx))
                            .child(
                                IconButton::new("new-conversation", IconName::Plus)
                                    .on_click(cx.listener(move |this, _event, cx| {
                                        this.new_conversation(cx);
                                    }))
                                    .tooltip(move |cx| Tooltip::text("New Conversation", cx)),
                            )
                            .child(
                                IconButton::new("assistant-menu", IconName::Menu)
                                    .disabled(true)
                                    .tooltip(move |cx| {
                                        Tooltip::text(
                                            "Coming soon – Assistant settings & controls",
                                            cx,
                                        )
                                    }),
                            ),
                    ),
            )
            .when(self.saved_conversations_open, |element| {
                element.child(
                    h_flex()
                        .absolute()
                        .top(header_height)
                        .w_full()
                        .child(self.saved_conversations.clone()),
                )
            })
            .child(Composer::new(
                self.composer_editor.clone(),
                self.project_index_button.clone(),
                self.active_file_button.clone(),
                crate::ui::ModelSelector::new(cx.view().downgrade(), self.model.clone())
                    .into_any_element(),
            ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct MessageId(usize);

impl MessageId {
    fn post_inc(&mut self) -> Self {
        let id = *self;
        self.0 += 1;
        id
    }
}

enum ChatMessage {
    User(UserMessage),
    Assistant(AssistantMessage),
}

impl ChatMessage {
    fn focus_handle(&self, cx: &AppContext) -> Option<FocusHandle> {
        match self {
            ChatMessage::User(message) => Some(message.body.focus_handle(cx)),
            ChatMessage::Assistant(_) => None,
        }
    }
}

struct UserMessage {
    pub id: MessageId,
    pub body: View<Markdown>,
    pub attachments: Vec<UserAttachment>,
}

struct AssistantMessagePart {
    pub body: View<Markdown>,
    pub tool_calls: Vec<ToolFunctionCall>,
}

struct AssistantMessage {
    pub id: MessageId,
    pub messages: Vec<AssistantMessagePart>,
    pub error: Option<SharedString>,
}
