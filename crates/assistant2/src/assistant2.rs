mod assistant_settings;
mod completion_provider;
pub mod tools;

use anyhow::{Context, Result};
use assistant_tooling::{ToolFunctionCall, ToolRegistry};
use client::{proto, Client};
use completion_provider::*;
use editor::Editor;
use feature_flags::FeatureFlagAppExt as _;
use futures::{future::join_all, StreamExt};
use gpui::{
    list, prelude::*, AnyElement, AppContext, AsyncWindowContext, EventEmitter, FocusHandle,
    FocusableView, Global, ListAlignment, ListState, Render, Task, View, WeakView,
};
use language::{language_settings::SoftWrap, LanguageRegistry};
use open_ai::{FunctionContent, ToolCall, ToolCallContent};
use rich_text::RichText;
use semantic_index::{CloudEmbeddingProvider, SemanticIndex};
use serde::Deserialize;
use settings::Settings;
use std::sync::Arc;
use theme::ThemeSettings;
use tools::ProjectIndexTool;
use ui::{popover_menu, prelude::*, ButtonLike, Color, ContextMenu, Tooltip};
use util::{paths::EMBEDDINGS_DIR, ResultExt};
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

gpui::actions!(assistant2, [Cancel, ToggleFocus]);
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

                let tool_registry = Arc::new(tool_registry);

                Self::new(app_state.languages.clone(), tool_registry, cx)
            })
        })
    }

    pub fn new(
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let chat = cx.new_view(|cx| {
            AssistantChat::new(language_registry.clone(), tool_registry.clone(), cx)
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

    fn icon(&self, _cx: &WindowContext) -> Option<ui::IconName> {
        Some(IconName::Ai)
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
        self.chat
            .read(cx)
            .messages
            .iter()
            .rev()
            .find_map(|msg| msg.focus_handle(cx))
            .expect("no user message in chat")
    }
}

struct AssistantChat {
    model: String,
    messages: Vec<ChatMessage>,
    list_state: ListState,
    language_registry: Arc<LanguageRegistry>,
    next_message_id: MessageId,
    pending_completion: Option<Task<()>>,
    tool_registry: Arc<ToolRegistry>,
}

impl AssistantChat {
    fn new(
        language_registry: Arc<LanguageRegistry>,
        tool_registry: Arc<ToolRegistry>,
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

        let mut this = Self {
            model,
            messages: Vec::new(),
            list_state,
            language_registry,
            next_message_id: MessageId(0),
            pending_completion: None,
            tool_registry,
        };
        this.push_new_user_message(true, cx);
        this
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
        if self.pending_completion.take().is_none() {
            cx.propagate();
            return;
        }

        if let Some(ChatMessage::Assistant(message)) = self.messages.last() {
            if message.body.text.is_empty() {
                self.pop_message(cx);
            } else {
                self.push_new_user_message(false, cx);
            }
        }
    }

    fn submit(&mut self, Submit(mode): &Submit, cx: &mut ViewContext<Self>) {
        let Some(focused_message_id) = self.focused_message_id(cx) else {
            log::error!("unexpected state: no user message editor is focused.");
            return;
        };

        self.truncate_messages(focused_message_id, cx);

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

            this.update(&mut cx, |this, cx| {
                let focus = this
                    .user_message(focused_message_id)
                    .body
                    .focus_handle(cx)
                    .contains_focused(cx);
                this.push_new_user_message(focus, cx);
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
                let completion = this.update(cx, |this, cx| {
                    this.push_new_assistant_message(cx);

                    let definitions = if call_count < limit
                        && matches!(mode, SubmitMode::Codebase | SubmitMode::Simple)
                    {
                        this.tool_registry.definitions()
                    } else {
                        &[]
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

    fn user_message(&mut self, message_id: MessageId) -> &mut UserMessage {
        self.messages
            .iter_mut()
            .find_map(|message| match message {
                ChatMessage::User(user_message) if user_message.id == message_id => {
                    Some(user_message)
                }
                _ => None,
            })
            .expect("User message not found")
    }

    fn push_new_user_message(&mut self, focus: bool, cx: &mut ViewContext<Self>) {
        let id = self.next_message_id.post_inc();
        let body = cx.new_view(|cx| {
            let mut editor = Editor::auto_height(80, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            if focus {
                cx.focus_self();
            }
            editor
        });
        let message = ChatMessage::User(UserMessage { id, body });
        self.push_message(message, cx);
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
            ChatMessage::User(UserMessage { body, .. }) => div()
                .when(!is_last, |element| element.mb_2())
                .child(div().p_2().child(Label::new("You").color(Color::Default)))
                .child(
                    div()
                        .on_action(cx.listener(Self::submit))
                        .p_2()
                        .text_color(cx.theme().colors().editor_foreground)
                        .font(ThemeSettings::get_global(cx).buffer_font.clone())
                        .bg(cx.theme().colors().editor_background)
                        .child(body.clone()),
                )
                .into_any(),
            ChatMessage::Assistant(AssistantMessage {
                id,
                body,
                error,
                tool_calls,
                ..
            }) => {
                let assistant_body = if body.text.is_empty() && !tool_calls.is_empty() {
                    div()
                } else {
                    div().p_2().child(body.element(ElementId::from(id.0), cx))
                };

                div()
                    .when(!is_last, |element| element.mb_2())
                    .child(
                        div()
                            .p_2()
                            .child(Label::new("Assistant").color(Color::Modified)),
                    )
                    .child(assistant_body)
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

    fn render_model_dropdown(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let this = cx.view().downgrade();
        div().h_flex().justify_end().child(
            div().w_32().child(
                popover_menu("user-menu")
                    .menu(move |cx| {
                        ContextMenu::build(cx, |mut menu, cx| {
                            for model in CompletionProvider::get(cx).available_models() {
                                menu = menu.custom_entry(
                                    {
                                        let model = model.clone();
                                        move |_| Label::new(model.clone()).into_any_element()
                                    },
                                    {
                                        let this = this.clone();
                                        move |cx| {
                                            _ = this.update(cx, |this, cx| {
                                                this.model = model.clone();
                                                cx.notify();
                                            });
                                        }
                                    },
                                );
                            }
                            menu
                        })
                        .into()
                    })
                    .trigger(
                        ButtonLike::new("active-model")
                            .child(
                                h_flex()
                                    .w_full()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .overflow_x_hidden()
                                            .flex_grow()
                                            .whitespace_nowrap()
                                            .child(Label::new(self.model.clone())),
                                    )
                                    .child(div().child(
                                        Icon::new(IconName::ChevronDown).color(Color::Muted),
                                    )),
                            )
                            .style(ButtonStyle::Subtle)
                            .tooltip(move |cx| Tooltip::text("Change Model", cx)),
                    )
                    .anchor(gpui::AnchorCorner::TopRight),
            ),
        )
    }
}

impl Render for AssistantChat {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .relative()
            .flex_1()
            .v_flex()
            .key_context("AssistantChat")
            .on_action(cx.listener(Self::cancel))
            .text_color(Color::Default.color(cx))
            .child(self.render_model_dropdown(cx))
            .child(list(self.list_state.clone()).flex_1())
            .child(
                h_flex()
                    .mt_2()
                    .gap_2()
                    .children(self.tool_registry.status_views().iter().cloned()),
            )
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct MessageId(usize);

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
