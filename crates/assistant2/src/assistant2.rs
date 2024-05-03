mod assistant_settings;
mod completion_provider;
mod tools;
pub mod ui;

use ::ui::{div, prelude::*, Color, ViewContext};
use anyhow::{Context, Result};
use assistant_tooling::{ToolFunctionCall, ToolRegistry};
use client::{proto, Client, UserStore};
use collections::HashMap;
use completion_provider::*;
use editor::Editor;
use feature_flags::FeatureFlagAppExt as _;
use futures::{future::join_all, StreamExt};
use gpui::{
    list, AnyElement, AppContext, AsyncWindowContext, ClickEvent, EventEmitter, FocusHandle,
    FocusableView, ListAlignment, ListState, Model, Render, Task, View, WeakView,
};
use language::{language_settings::SoftWrap, LanguageRegistry};
use open_ai::{FunctionContent, ToolCall, ToolCallContent};
use rich_text::RichText;
use semantic_index::{CloudEmbeddingProvider, ProjectIndex, SemanticIndex};
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use ui::{Composer, ProjectIndexButton};
use util::{paths::EMBEDDINGS_DIR, ResultExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

pub use assistant_settings::AssistantSettings;

use crate::tools::{CreateBufferTool, ProjectIndexTool};
use crate::ui::UserOrAssistant;

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

gpui::actions!(assistant2, [Cancel, ToggleFocus, DebugProjectIndex]);
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

            let user_store = app_state.user_store.clone();

            cx.new_view(|cx| {
                // todo!("this will panic if the semantic index failed to load or has not loaded yet")
                let project_index = cx.update_global(|semantic_index: &mut SemanticIndex, cx| {
                    semantic_index.project_index(project.clone(), cx)
                });

                let mut tool_registry = ToolRegistry::new();
                tool_registry
                    .register(
                        ProjectIndexTool::new(project_index.clone(), app_state.fs.clone()),
                        cx,
                    )
                    .context("failed to register ProjectIndexTool")
                    .log_err();
                tool_registry
                    .register(
                        CreateBufferTool::new(workspace.clone(), project.clone()),
                        cx,
                    )
                    .context("failed to register CreateBufferTool")
                    .log_err();

                let tool_registry = Arc::new(tool_registry);

                Self::new(
                    app_state.languages.clone(),
                    tool_registry,
                    user_store,
                    Some(project_index),
                    cx,
                )
            })
        })
    }

    pub fn new(
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
        user_store: Model<UserStore>,
        project_index: Option<Model<ProjectIndex>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let chat = cx.new_view(|cx| {
            AssistantChat::new(
                language_registry.clone(),
                tool_registry.clone(),
                user_store,
                project_index,
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
            .p_2()
            .bg(cx.theme().colors().background)
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
    language_registry: Arc<LanguageRegistry>,
    composer_editor: View<Editor>,
    project_index_button: Option<View<ProjectIndexButton>>,
    user_store: Model<UserStore>,
    next_message_id: MessageId,
    collapsed_messages: HashMap<MessageId, bool>,
    editing_message: Option<EditingMessage>,
    pending_completion: Option<Task<()>>,
    tool_registry: Arc<ToolRegistry>,
    project_index: Option<Model<ProjectIndex>>,
}

struct EditingMessage {
    id: MessageId,
    old_body: Arc<str>,
    body: View<Editor>,
}

impl AssistantChat {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
        user_store: Model<UserStore>,
        project_index: Option<Model<ProjectIndex>>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let model = CompletionProvider::get(cx).default_model();
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

        let project_index_button = project_index.clone().map(|project_index| {
            cx.new_view(|cx| ProjectIndexButton::new(project_index, tool_registry.clone(), cx))
        });

        Self {
            model,
            messages: Vec::new(),
            composer_editor: cx.new_view(|cx| {
                let mut editor = Editor::auto_height(80, cx);
                editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                editor.set_placeholder_text("Send a message…", cx);
                editor
            }),
            list_state,
            user_store,
            language_registry,
            project_index_button,
            project_index,
            next_message_id: MessageId(0),
            editing_message: None,
            collapsed_messages: HashMap::default(),
            pending_completion: None,
            tool_registry,
        }
    }

    fn editing_message_id(&self) -> Option<MessageId> {
        self.editing_message.as_ref().map(|message| message.id)
    }

    fn focused_message_id(&self, cx: &WindowContext) -> Option<MessageId> {
        self.messages.iter().find_map(|message| match message {
            ChatMessage::User(message) => message
                .body
                .focus_handle(cx)
                .contains_focused(cx)
                .then_some(message.id),
            ChatMessage::Assistant(_) => None,
        })
    }

    fn cancel(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        // If we're currently editing a message, cancel the edit.
        if let Some(editing_message) = self.editing_message.take() {
            editing_message
                .body
                .update(cx, |body, cx| body.set_text(editing_message.old_body, cx));
            return;
        }

        if self.pending_completion.take().is_some() {
            if let Some(ChatMessage::Assistant(message)) = self.messages.last() {
                if message.body.text.is_empty() {
                    self.pop_message(cx);
                }
            }
            return;
        }

        cx.propagate();
    }

    fn submit(&mut self, Submit(mode): &Submit, cx: &mut ViewContext<Self>) {
        if let Some(focused_message_id) = self.focused_message_id(cx) {
            self.truncate_messages(focused_message_id, cx);
            self.pending_completion.take();
            self.composer_editor.focus_handle(cx).focus(cx);
            if self.editing_message_id() == Some(focused_message_id) {
                self.editing_message.take();
            }
        } else if self.composer_editor.focus_handle(cx).is_focused(cx) {
            // Don't allow multiple concurrent completions.
            if self.pending_completion.is_some() {
                cx.propagate();
                return;
            }

            let message = self.composer_editor.update(cx, |composer_editor, cx| {
                let text = composer_editor.text(cx);
                let id = self.next_message_id.post_inc();
                let body = cx.new_view(|cx| {
                    let mut editor = Editor::auto_height(80, cx);
                    editor.set_text(text, cx);
                    editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
                    editor
                });
                composer_editor.clear(cx);
                ChatMessage::User(UserMessage { id, body })
            });
            self.push_message(message, cx);
        } else {
            log::error!("unexpected state: no user message editor is focused.");
            return;
        }

        let mode = *mode;
        self.pending_completion = Some(cx.spawn(move |this, mut cx| async move {
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

    fn debug_project_index(&mut self, _: &DebugProjectIndex, cx: &mut ViewContext<Self>) {
        if let Some(index) = &self.project_index {
            index.update(cx, |project_index, cx| {
                project_index.debug(cx).detach_and_log_err(cx)
            });
        }
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
                let completion = this.update(cx, |this, cx| {
                    this.push_new_assistant_message(cx);

                    let definitions = if call_count < limit
                        && matches!(mode, SubmitMode::Codebase | SubmitMode::Simple)
                    {
                        this.tool_registry.definitions()
                    } else {
                        Vec::new()
                    };
                    call_count += 1;

                    let messages = this.completion_messages(cx);

                    CompletionProvider::get(cx).complete(
                        this.model.clone(),
                        messages,
                        Vec::new(),
                        1.0,
                        definitions,
                    )
                });

                let mut stream = completion?.await?;
                let mut body = String::new();
                while let Some(delta) = stream.next().await {
                    let delta = delta?;
                    this.update(cx, |this, cx| {
                        if let Some(ChatMessage::Assistant(AssistantMessage {
                            body: message_body,
                            tool_calls: message_tool_calls,
                            ..
                        })) = this.messages.last_mut()
                        {
                            if let Some(content) = &delta.content {
                                body.push_str(content);
                            }

                            for tool_call in delta.tool_calls {
                                let index = tool_call.index as usize;
                                if index >= message_tool_calls.len() {
                                    message_tool_calls.resize_with(index + 1, Default::default);
                                }
                                let call = &mut message_tool_calls[index];

                                if let Some(id) = &tool_call.id {
                                    call.id.push_str(id);
                                }

                                match tool_call.variant {
                                    Some(proto::tool_call_delta::Variant::Function(tool_call)) => {
                                        if let Some(name) = &tool_call.name {
                                            call.name.push_str(name);
                                        }
                                        if let Some(arguments) = &tool_call.arguments {
                                            call.arguments.push_str(arguments);
                                        }
                                    }
                                    None => {}
                                }
                            }

                            *message_body =
                                RichText::new(body.clone(), &[], &this.language_registry);
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
                    tool_calls,
                    ..
                })) = this.messages.last_mut()
                {
                    if let Err(error) = complete {
                        message_error.replace(SharedString::from(error.to_string()));
                        cx.notify();
                    } else {
                        for tool_call in tool_calls.iter() {
                            tool_tasks.push(this.tool_registry.call(tool_call, cx));
                        }
                    }
                }
            })?;

            if tool_tasks.is_empty() {
                return Ok(());
            }

            let tools = join_all(tool_tasks.into_iter()).await;
            // If the WindowContext went away for any tool's view we don't include it
            // especially since the below call would fail for the same reason.
            let tools = tools.into_iter().filter_map(|tool| tool.ok()).collect();

            this.update(cx, |this, cx| {
                if let Some(ChatMessage::Assistant(AssistantMessage { tool_calls, .. })) =
                    this.messages.last_mut()
                {
                    *tool_calls = tools;
                    cx.notify();
                }
            })?;
        }
    }

    fn push_new_assistant_message(&mut self, cx: &mut ViewContext<Self>) {
        let message = ChatMessage::Assistant(AssistantMessage {
            id: self.next_message_id.post_inc(),
            body: RichText::default(),
            tool_calls: Vec::new(),
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
                .neg_mx_1()
                .rounded_md()
                .border()
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
        let is_last = ix == self.messages.len() - 1;

        match &self.messages[ix] {
            ChatMessage::User(UserMessage { id, body }) => div()
                .id(SharedString::from(format!("message-{}-container", id.0)))
                .when(!is_last, |element| element.mb_2())
                .map(|element| {
                    if self.editing_message_id() == Some(*id) {
                        element.child(Composer::new(
                            body.clone(),
                            self.project_index_button.clone(),
                            crate::ui::ModelSelector::new(
                                cx.view().downgrade(),
                                self.model.clone(),
                            )
                            .into_any_element(),
                        ))
                    } else {
                        element
                            .on_click(cx.listener({
                                let id = *id;
                                let body = body.clone();
                                move |assistant_chat, event: &ClickEvent, cx| {
                                    if event.up.click_count == 2 {
                                        assistant_chat.editing_message = Some(EditingMessage {
                                            id,
                                            body: body.clone(),
                                            old_body: body.read(cx).text(cx).into(),
                                        });
                                        body.focus_handle(cx).focus(cx);
                                    }
                                }
                            }))
                            .child(crate::ui::ChatMessage::new(
                                *id,
                                UserOrAssistant::User(self.user_store.read(cx).current_user()),
                                Some(
                                    RichText::new(
                                        body.read(cx).text(cx),
                                        &[],
                                        &self.language_registry,
                                    )
                                    .element(ElementId::from(id.0), cx),
                                ),
                                self.is_message_collapsed(id),
                                Box::new(cx.listener({
                                    let id = *id;
                                    move |assistant_chat, _event, _cx| {
                                        assistant_chat.toggle_message_collapsed(id)
                                    }
                                })),
                            ))
                    }
                })
                .into_any(),
            ChatMessage::Assistant(AssistantMessage {
                id,
                body,
                error,
                tool_calls,
                ..
            }) => {
                let assistant_body = if body.text.is_empty() {
                    None
                } else {
                    Some(
                        div()
                            .p_2()
                            .child(body.element(ElementId::from(id.0), cx))
                            .into_any_element(),
                    )
                };

                div()
                    .when(!is_last, |element| element.mb_2())
                    .child(crate::ui::ChatMessage::new(
                        *id,
                        UserOrAssistant::Assistant,
                        assistant_body,
                        self.is_message_collapsed(id),
                        Box::new(cx.listener({
                            let id = *id;
                            move |assistant_chat, _event, _cx| {
                                assistant_chat.toggle_message_collapsed(id)
                            }
                        })),
                    ))
                    // TODO: Should the errors and tool calls get passed into `ChatMessage`?
                    .child(self.render_error(error.clone(), ix, cx))
                    .children(tool_calls.iter().map(|tool_call| {
                        let result = &tool_call.result;
                        let name = tool_call.name.clone();
                        match result {
                            Some(result) => {
                                div().p_2().child(result.into_any_element(&name)).into_any()
                            }
                            None => div()
                                .p_2()
                                .child(Label::new(name).color(Color::Modified))
                                .child("Running...")
                                .into_any(),
                        }
                    }))
                    .into_any()
            }
        }
    }

    fn completion_messages(&self, cx: &mut WindowContext) -> Vec<CompletionMessage> {
        let mut completion_messages = Vec::new();

        for message in &self.messages {
            match message {
                ChatMessage::User(UserMessage { body, .. }) => {
                    // When we re-introduce contexts like active file, we'll inject them here instead of relying on the model to request them
                    // contexts.iter().for_each(|context| {
                    //     completion_messages.extend(context.completion_messages(cx))
                    // });

                    // Show user's message last so that the assistant is grounded in the user's request
                    completion_messages.push(CompletionMessage::User {
                        content: body.read(cx).text(cx),
                    });
                }
                ChatMessage::Assistant(AssistantMessage {
                    body, tool_calls, ..
                }) => {
                    // In no case do we want to send an empty message. This shouldn't happen, but we might as well
                    // not break the Chat API if it does.
                    if body.text.is_empty() && tool_calls.is_empty() {
                        continue;
                    }

                    let tool_calls_from_assistant = tool_calls
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
                        content: Some(body.text.to_string()),
                        tool_calls: tool_calls_from_assistant,
                    });

                    for tool_call in tool_calls {
                        // todo!(): we should not be sending when the tool is still running / has no result
                        // For now I'm going to have to assume we send an empty string because otherwise
                        // the Chat API will break -- there is a required message for every tool call by ID
                        let content = match &tool_call.result {
                            Some(result) => result.format(&tool_call.name),
                            None => "".to_string(),
                        };

                        completion_messages.push(CompletionMessage::Tool {
                            content,
                            tool_call_id: tool_call.id.clone(),
                        });
                    }
                }
            }
        }

        completion_messages
    }
}

impl Render for AssistantChat {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .relative()
            .flex_1()
            .v_flex()
            .key_context("AssistantChat")
            .on_action(cx.listener(Self::submit))
            .on_action(cx.listener(Self::cancel))
            .on_action(cx.listener(Self::debug_project_index))
            .text_color(Color::Default.color(cx))
            .child(list(self.list_state.clone()).flex_1())
            .child(Composer::new(
                self.composer_editor.clone(),
                self.project_index_button.clone(),
                crate::ui::ModelSelector::new(cx.view().downgrade(), self.model.clone())
                    .into_any_element(),
            ))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
            ChatMessage::User(UserMessage { body, .. }) => Some(body.focus_handle(cx)),
            ChatMessage::Assistant(_) => None,
        }
    }
}

struct UserMessage {
    id: MessageId,
    body: View<Editor>,
}

struct AssistantMessage {
    id: MessageId,
    body: RichText,
    tool_calls: Vec<ToolFunctionCall>,
    error: Option<SharedString>,
}
