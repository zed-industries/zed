use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    OpenAIRequest, OpenAIResponseStreamEvent, RequestMessage, Role,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle, ToDisplayPoint},
    scroll::autoscroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, ToOffset as _,
};
use fs::Fs;
use futures::{io::BufReader, AsyncBufReadExt, AsyncReadExt, Stream, StreamExt};
use gpui::{
    actions,
    elements::*,
    executor::Background,
    geometry::vector::{vec2f, Vector2F},
    platform::{CursorStyle, MouseButton},
    Action, AppContext, AsyncAppContext, ClipboardItem, Entity, ModelContext, ModelHandle,
    Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use isahc::{http::StatusCode, Request, RequestExt};
use language::{language_settings::SoftWrap, Buffer, LanguageRegistry, ToOffset as _};
use serde::Deserialize;
use settings::SettingsStore;
use std::{
    borrow::Cow, cell::RefCell, cmp, fmt::Write, io, iter, ops::Range, rc::Rc, sync::Arc,
    time::Duration,
};
use util::{channel::ReleaseChannel, post_inc, truncate_and_trailoff, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    item::Item,
    pane, Pane, Workspace,
};

const OPENAI_API_URL: &'static str = "https://api.openai.com/v1";

actions!(
    assistant,
    [NewContext, Assist, QuoteSelection, ToggleFocus, ResetKey]
);

pub fn init(cx: &mut AppContext) {
    if *util::channel::RELEASE_CHANNEL == ReleaseChannel::Stable {
        cx.update_default_global::<collections::CommandPaletteFilter, _, _>(move |filter, _cx| {
            filter.filtered_namespaces.insert("assistant");
        });
    }

    settings::register::<AssistantSettings>(cx);
    cx.add_action(
        |workspace: &mut Workspace, _: &NewContext, cx: &mut ViewContext<Workspace>| {
            if let Some(this) = workspace.panel::<AssistantPanel>(cx) {
                this.update(cx, |this, cx| this.add_context(cx))
            }

            workspace.focus_panel::<AssistantPanel>(cx);
        },
    );
    cx.add_action(AssistantEditor::assist);
    cx.capture_action(AssistantEditor::cancel_last_assist);
    cx.add_action(AssistantEditor::quote_selection);
    cx.capture_action(AssistantEditor::copy);
    cx.add_action(AssistantPanel::save_api_key);
    cx.add_action(AssistantPanel::reset_api_key);
    cx.add_action(
        |workspace: &mut Workspace, _: &ToggleFocus, cx: &mut ViewContext<Workspace>| {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        },
    );
}

pub enum AssistantPanelEvent {
    ZoomIn,
    ZoomOut,
    Focus,
    Close,
    DockPositionChanged,
}

pub struct AssistantPanel {
    width: Option<f32>,
    height: Option<f32>,
    pane: ViewHandle<Pane>,
    api_key: Rc<RefCell<Option<String>>>,
    api_key_editor: Option<ViewHandle<Editor>>,
    has_read_credentials: bool,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    subscriptions: Vec<Subscription>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            // TODO: deserialize state.
            workspace.update(&mut cx, |workspace, cx| {
                cx.add_view::<Self, _>(|cx| {
                    let weak_self = cx.weak_handle();
                    let pane = cx.add_view(|cx| {
                        let mut pane = Pane::new(
                            workspace.weak_handle(),
                            workspace.project().clone(),
                            workspace.app_state().background_actions,
                            Default::default(),
                            cx,
                        );
                        pane.set_can_split(false, cx);
                        pane.set_can_navigate(false, cx);
                        pane.on_can_drop(move |_, _| false);
                        pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                            let weak_self = weak_self.clone();
                            Flex::row()
                                .with_child(Pane::render_tab_bar_button(
                                    0,
                                    "icons/plus_12.svg",
                                    false,
                                    Some(("New Context".into(), Some(Box::new(NewContext)))),
                                    cx,
                                    move |_, cx| {
                                        let weak_self = weak_self.clone();
                                        cx.window_context().defer(move |cx| {
                                            if let Some(this) = weak_self.upgrade(cx) {
                                                this.update(cx, |this, cx| this.add_context(cx));
                                            }
                                        })
                                    },
                                    None,
                                ))
                                .with_child(Pane::render_tab_bar_button(
                                    1,
                                    if pane.is_zoomed() {
                                        "icons/minimize_8.svg"
                                    } else {
                                        "icons/maximize_8.svg"
                                    },
                                    pane.is_zoomed(),
                                    Some((
                                        "Toggle Zoom".into(),
                                        Some(Box::new(workspace::ToggleZoom)),
                                    )),
                                    cx,
                                    move |pane, cx| pane.toggle_zoom(&Default::default(), cx),
                                    None,
                                ))
                                .into_any()
                        });
                        let buffer_search_bar = cx.add_view(search::BufferSearchBar::new);
                        pane.toolbar()
                            .update(cx, |toolbar, cx| toolbar.add_item(buffer_search_bar, cx));
                        pane
                    });

                    let mut this = Self {
                        pane,
                        api_key: Rc::new(RefCell::new(None)),
                        api_key_editor: None,
                        has_read_credentials: false,
                        languages: workspace.app_state().languages.clone(),
                        fs: workspace.app_state().fs.clone(),
                        width: None,
                        height: None,
                        subscriptions: Default::default(),
                    };

                    let mut old_dock_position = this.position(cx);
                    this.subscriptions = vec![
                        cx.observe(&this.pane, |_, _, cx| cx.notify()),
                        cx.subscribe(&this.pane, Self::handle_pane_event),
                        cx.observe_global::<SettingsStore, _>(move |this, cx| {
                            let new_dock_position = this.position(cx);
                            if new_dock_position != old_dock_position {
                                old_dock_position = new_dock_position;
                                cx.emit(AssistantPanelEvent::DockPositionChanged);
                            }
                        }),
                    ];

                    this
                })
            })
        })
    }

    fn handle_pane_event(
        &mut self,
        _pane: ViewHandle<Pane>,
        event: &pane::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            pane::Event::ZoomIn => cx.emit(AssistantPanelEvent::ZoomIn),
            pane::Event::ZoomOut => cx.emit(AssistantPanelEvent::ZoomOut),
            pane::Event::Focus => cx.emit(AssistantPanelEvent::Focus),
            pane::Event::Remove => cx.emit(AssistantPanelEvent::Close),
            _ => {}
        }
    }

    fn add_context(&mut self, cx: &mut ViewContext<Self>) {
        let focus = self.has_focus(cx);
        let editor = cx
            .add_view(|cx| AssistantEditor::new(self.api_key.clone(), self.languages.clone(), cx));
        self.subscriptions
            .push(cx.subscribe(&editor, Self::handle_assistant_editor_event));
        self.pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(editor), true, focus, None, cx)
        });
    }

    fn handle_assistant_editor_event(
        &mut self,
        _: ViewHandle<AssistantEditor>,
        event: &AssistantEditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            AssistantEditorEvent::TabContentChanged => self.pane.update(cx, |_, cx| cx.notify()),
        }
    }

    fn save_api_key(&mut self, _: &menu::Confirm, cx: &mut ViewContext<Self>) {
        if let Some(api_key) = self
            .api_key_editor
            .as_ref()
            .map(|editor| editor.read(cx).text(cx))
        {
            if !api_key.is_empty() {
                cx.platform()
                    .write_credentials(OPENAI_API_URL, "Bearer", api_key.as_bytes())
                    .log_err();
                *self.api_key.borrow_mut() = Some(api_key);
                self.api_key_editor.take();
                cx.focus_self();
                cx.notify();
            }
        } else {
            cx.propagate_action();
        }
    }

    fn reset_api_key(&mut self, _: &ResetKey, cx: &mut ViewContext<Self>) {
        cx.platform().delete_credentials(OPENAI_API_URL).log_err();
        self.api_key.take();
        self.api_key_editor = Some(build_api_key_editor(cx));
        cx.focus_self();
        cx.notify();
    }
}

fn build_api_key_editor(cx: &mut ViewContext<AssistantPanel>) -> ViewHandle<Editor> {
    cx.add_view(|cx| {
        let mut editor = Editor::single_line(
            Some(Arc::new(|theme| theme.assistant.api_key_editor.clone())),
            cx,
        );
        editor.set_placeholder_text("sk-000000000000000000000000000000000000000000000000", cx);
        editor
    })
}

impl Entity for AssistantPanel {
    type Event = AssistantPanelEvent;
}

impl View for AssistantPanel {
    fn ui_name() -> &'static str {
        "AssistantPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let style = &theme::current(cx).assistant;
        if let Some(api_key_editor) = self.api_key_editor.as_ref() {
            Flex::column()
                .with_child(
                    Text::new(
                        "Paste your OpenAI API key and press Enter to use the assistant",
                        style.api_key_prompt.text.clone(),
                    )
                    .aligned(),
                )
                .with_child(
                    ChildView::new(api_key_editor, cx)
                        .contained()
                        .with_style(style.api_key_editor.container)
                        .aligned(),
                )
                .contained()
                .with_style(style.api_key_prompt.container)
                .aligned()
                .into_any()
        } else {
            ChildView::new(&self.pane, cx).into_any()
        }
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            if let Some(api_key_editor) = self.api_key_editor.as_ref() {
                cx.focus(api_key_editor);
            } else {
                cx.focus(&self.pane);
            }
        }
    }
}

impl Panel for AssistantPanel {
    fn position(&self, cx: &WindowContext) -> DockPosition {
        match settings::get::<AssistantSettings>(cx).dock {
            AssistantDockPosition::Left => DockPosition::Left,
            AssistantDockPosition::Bottom => DockPosition::Bottom,
            AssistantDockPosition::Right => DockPosition::Right,
        }
    }

    fn position_is_valid(&self, _: DockPosition) -> bool {
        true
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<AssistantSettings>(self.fs.clone(), cx, move |settings| {
            let dock = match position {
                DockPosition::Left => AssistantDockPosition::Left,
                DockPosition::Bottom => AssistantDockPosition::Bottom,
                DockPosition::Right => AssistantDockPosition::Right,
            };
            settings.dock = Some(dock);
        });
    }

    fn size(&self, cx: &WindowContext) -> f32 {
        let settings = settings::get::<AssistantSettings>(cx);
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => {
                self.width.unwrap_or_else(|| settings.default_width)
            }
            DockPosition::Bottom => self.height.unwrap_or_else(|| settings.default_height),
        }
    }

    fn set_size(&mut self, size: f32, cx: &mut ViewContext<Self>) {
        match self.position(cx) {
            DockPosition::Left | DockPosition::Right => self.width = Some(size),
            DockPosition::Bottom => self.height = Some(size),
        }
        cx.notify();
    }

    fn should_zoom_in_on_event(event: &AssistantPanelEvent) -> bool {
        matches!(event, AssistantPanelEvent::ZoomIn)
    }

    fn should_zoom_out_on_event(event: &AssistantPanelEvent) -> bool {
        matches!(event, AssistantPanelEvent::ZoomOut)
    }

    fn is_zoomed(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).is_zoomed()
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.pane.update(cx, |pane, cx| pane.set_zoomed(zoomed, cx));
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            if self.api_key.borrow().is_none() && !self.has_read_credentials {
                self.has_read_credentials = true;
                let api_key = if let Some((_, api_key)) = cx
                    .platform()
                    .read_credentials(OPENAI_API_URL)
                    .log_err()
                    .flatten()
                {
                    String::from_utf8(api_key).log_err()
                } else {
                    None
                };
                if let Some(api_key) = api_key {
                    *self.api_key.borrow_mut() = Some(api_key);
                } else if self.api_key_editor.is_none() {
                    self.api_key_editor = Some(build_api_key_editor(cx));
                    cx.notify();
                }
            }

            if self.pane.read(cx).items_len() == 0 {
                self.add_context(cx);
            }
        }
    }

    fn icon_path(&self) -> &'static str {
        "icons/robot_14.svg"
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>) {
        ("Assistant Panel".into(), Some(Box::new(ToggleFocus)))
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, AssistantPanelEvent::DockPositionChanged)
    }

    fn should_activate_on_event(_: &Self::Event) -> bool {
        false
    }

    fn should_close_on_event(event: &AssistantPanelEvent) -> bool {
        matches!(event, AssistantPanelEvent::Close)
    }

    fn has_focus(&self, cx: &WindowContext) -> bool {
        self.pane.read(cx).has_focus()
            || self
                .api_key_editor
                .as_ref()
                .map_or(false, |editor| editor.is_focused(cx))
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, AssistantPanelEvent::Focus)
    }
}

enum AssistantEvent {
    MessagesEdited,
    SummaryChanged,
    StreamedCompletion,
}

struct Assistant {
    buffer: ModelHandle<Buffer>,
    messages: Vec<Message>,
    messages_metadata: HashMap<MessageId, MessageMetadata>,
    next_message_id: MessageId,
    summary: Option<String>,
    pending_summary: Task<Option<()>>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    model: String,
    token_count: Option<usize>,
    max_token_count: usize,
    pending_token_count: Task<Option<()>>,
    api_key: Rc<RefCell<Option<String>>>,
    _subscriptions: Vec<Subscription>,
}

impl Entity for Assistant {
    type Event = AssistantEvent;
}

impl Assistant {
    fn new(
        api_key: Rc<RefCell<Option<String>>>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let model = "gpt-3.5-turbo";
        let markdown = language_registry.language_for_name("Markdown");
        let buffer = cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            buffer.set_language_registry(language_registry);
            cx.spawn_weak(|buffer, mut cx| async move {
                let markdown = markdown.await?;
                let buffer = buffer
                    .upgrade(&cx)
                    .ok_or_else(|| anyhow!("buffer was dropped"))?;
                buffer.update(&mut cx, |buffer, cx| {
                    buffer.set_language(Some(markdown), cx)
                });
                anyhow::Ok(())
            })
            .detach_and_log_err(cx);
            buffer
        });

        let mut this = Self {
            messages: Default::default(),
            messages_metadata: Default::default(),
            next_message_id: Default::default(),
            summary: None,
            pending_summary: Task::ready(None),
            completion_count: Default::default(),
            pending_completions: Default::default(),
            token_count: None,
            max_token_count: tiktoken_rs::model::get_context_size(model),
            pending_token_count: Task::ready(None),
            model: model.into(),
            _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
            api_key,
            buffer,
        };
        let message = Message {
            id: MessageId(post_inc(&mut this.next_message_id.0)),
            start: language::Anchor::MIN,
        };
        this.messages.push(message.clone());
        this.messages_metadata.insert(
            message.id,
            MessageMetadata {
                role: Role::User,
                sent_at: Local::now(),
                error: None,
            },
        );

        this.count_remaining_tokens(cx);
        this
    }

    fn handle_buffer_event(
        &mut self,
        _: ModelHandle<Buffer>,
        event: &language::Event,
        cx: &mut ModelContext<Self>,
    ) {
        match event {
            language::Event::Edited => {
                self.count_remaining_tokens(cx);
                cx.emit(AssistantEvent::MessagesEdited);
            }
            _ => {}
        }
    }

    fn count_remaining_tokens(&mut self, cx: &mut ModelContext<Self>) {
        let messages = self
            .open_ai_request_messages(cx)
            .into_iter()
            .filter_map(|message| {
                Some(tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: message.content,
                    name: None,
                })
            })
            .collect::<Vec<_>>();
        let model = self.model.clone();
        self.pending_token_count = cx.spawn_weak(|this, mut cx| {
            async move {
                cx.background().timer(Duration::from_millis(200)).await;
                let token_count = cx
                    .background()
                    .spawn(async move { tiktoken_rs::num_tokens_from_messages(&model, &messages) })
                    .await?;

                this.upgrade(&cx)
                    .ok_or_else(|| anyhow!("assistant was dropped"))?
                    .update(&mut cx, |this, cx| {
                        this.max_token_count = tiktoken_rs::model::get_context_size(&this.model);
                        this.token_count = Some(token_count);
                        cx.notify()
                    });
                anyhow::Ok(())
            }
            .log_err()
        });
    }

    fn remaining_tokens(&self) -> Option<isize> {
        Some(self.max_token_count as isize - self.token_count? as isize)
    }

    fn set_model(&mut self, model: String, cx: &mut ModelContext<Self>) {
        self.model = model;
        self.count_remaining_tokens(cx);
        cx.notify();
    }

    fn assist(&mut self, cx: &mut ModelContext<Self>) -> Option<(Message, Message)> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: self.open_ai_request_messages(cx),
            stream: true,
        };

        let api_key = self.api_key.borrow().clone()?;
        let stream = stream_completion(api_key, cx.background().clone(), request);
        let assistant_message =
            self.insert_message_after(self.messages.last()?.id, Role::Assistant, cx)?;
        let user_message = self.insert_message_after(assistant_message.id, Role::User, cx)?;
        let task = cx.spawn_weak({
            |this, mut cx| async move {
                let assistant_message_id = assistant_message.id;
                let stream_completion = async {
                    let mut messages = stream.await?;

                    while let Some(message) = messages.next().await {
                        let mut message = message?;
                        if let Some(choice) = message.choices.pop() {
                            this.upgrade(&cx)
                                .ok_or_else(|| anyhow!("assistant was dropped"))?
                                .update(&mut cx, |this, cx| {
                                    let text: Arc<str> = choice.delta.content?.into();
                                    let message_ix = this
                                        .messages
                                        .iter()
                                        .position(|message| message.id == assistant_message_id)?;
                                    this.buffer.update(cx, |buffer, cx| {
                                        let offset = if message_ix + 1 == this.messages.len() {
                                            buffer.len()
                                        } else {
                                            this.messages[message_ix + 1]
                                                .start
                                                .to_offset(buffer)
                                                .saturating_sub(1)
                                        };
                                        buffer.edit([(offset..offset, text)], None, cx);
                                    });
                                    cx.emit(AssistantEvent::StreamedCompletion);

                                    Some(())
                                });
                        }
                    }

                    this.upgrade(&cx)
                        .ok_or_else(|| anyhow!("assistant was dropped"))?
                        .update(&mut cx, |this, cx| {
                            this.pending_completions
                                .retain(|completion| completion.id != this.completion_count);
                            this.summarize(cx);
                        });

                    anyhow::Ok(())
                };

                let result = stream_completion.await;
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        if let Err(error) = result {
                            if let Some(metadata) =
                                this.messages_metadata.get_mut(&assistant_message.id)
                            {
                                metadata.error = Some(error.to_string().trim().into());
                                cx.notify();
                            }
                        }
                    });
                }
            }
        });

        self.pending_completions.push(PendingCompletion {
            id: post_inc(&mut self.completion_count),
            _task: task,
        });
        Some((assistant_message, user_message))
    }

    fn cancel_last_assist(&mut self) -> bool {
        self.pending_completions.pop().is_some()
    }

    fn cycle_message_role(&mut self, id: MessageId, cx: &mut ModelContext<Self>) {
        if let Some(metadata) = self.messages_metadata.get_mut(&id) {
            metadata.role.cycle();
            cx.emit(AssistantEvent::MessagesEdited);
            cx.notify();
        }
    }

    fn insert_message_after(
        &mut self,
        message_id: MessageId,
        role: Role,
        cx: &mut ModelContext<Self>,
    ) -> Option<Message> {
        if let Some(prev_message_ix) = self
            .messages
            .iter()
            .position(|message| message.id == message_id)
        {
            let start = self.buffer.update(cx, |buffer, cx| {
                let offset = self.messages[prev_message_ix + 1..]
                    .iter()
                    .find(|message| message.start.is_valid(buffer))
                    .map_or(buffer.len(), |message| message.start.to_offset(buffer) - 1);
                buffer.edit([(offset..offset, "\n")], None, cx);
                buffer.anchor_before(offset + 1)
            });
            let message = Message {
                id: MessageId(post_inc(&mut self.next_message_id.0)),
                start,
            };
            self.messages.insert(prev_message_ix + 1, message.clone());
            self.messages_metadata.insert(
                message.id,
                MessageMetadata {
                    role,
                    sent_at: Local::now(),
                    error: None,
                },
            );
            cx.emit(AssistantEvent::MessagesEdited);
            Some(message)
        } else {
            None
        }
    }

    fn summarize(&mut self, cx: &mut ModelContext<Self>) {
        if self.messages.len() >= 2 && self.summary.is_none() {
            let api_key = self.api_key.borrow().clone();
            if let Some(api_key) = api_key {
                let mut messages = self.open_ai_request_messages(cx);
                messages.truncate(2);
                messages.push(RequestMessage {
                    role: Role::User,
                    content: "Summarize the conversation into a short title without punctuation"
                        .into(),
                });
                let request = OpenAIRequest {
                    model: self.model.clone(),
                    messages,
                    stream: true,
                };

                let stream = stream_completion(api_key, cx.background().clone(), request);
                self.pending_summary = cx.spawn(|this, mut cx| {
                    async move {
                        let mut messages = stream.await?;

                        while let Some(message) = messages.next().await {
                            let mut message = message?;
                            if let Some(choice) = message.choices.pop() {
                                let text = choice.delta.content.unwrap_or_default();
                                this.update(&mut cx, |this, cx| {
                                    this.summary.get_or_insert(String::new()).push_str(&text);
                                    cx.emit(AssistantEvent::SummaryChanged);
                                });
                            }
                        }

                        anyhow::Ok(())
                    }
                    .log_err()
                });
            }
        }
    }

    fn open_ai_request_messages(&self, cx: &AppContext) -> Vec<RequestMessage> {
        let buffer = self.buffer.read(cx);
        self.messages(cx)
            .map(|(_message, metadata, range)| RequestMessage {
                role: metadata.role,
                content: buffer.text_for_range(range).collect(),
            })
            .collect()
    }

    fn message_id_for_offset(&self, offset: usize, cx: &AppContext) -> Option<MessageId> {
        Some(
            self.messages(cx)
                .find(|(_, _, range)| range.contains(&offset))
                .map(|(message, _, _)| message)
                .or(self.messages.last())?
                .id,
        )
    }

    fn messages<'a>(
        &'a self,
        cx: &'a AppContext,
    ) -> impl 'a + Iterator<Item = (&Message, &MessageMetadata, Range<usize>)> {
        let buffer = self.buffer.read(cx);
        let mut messages = self.messages.iter().peekable();
        iter::from_fn(move || {
            while let Some(message) = messages.next() {
                let metadata = self.messages_metadata.get(&message.id)?;
                let message_start = message.start.to_offset(buffer);
                let mut message_end = None;
                while let Some(next_message) = messages.peek() {
                    if next_message.start.is_valid(buffer) {
                        message_end = Some(next_message.start);
                        break;
                    } else {
                        messages.next();
                    }
                }
                let message_end = message_end
                    .unwrap_or(language::Anchor::MAX)
                    .to_offset(buffer);
                return Some((message, metadata, message_start..message_end));
            }
            None
        })
    }
}

struct PendingCompletion {
    id: usize,
    _task: Task<()>,
}

enum AssistantEditorEvent {
    TabContentChanged,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: Vector2F,
    cursor: Anchor,
}

struct AssistantEditor {
    assistant: ModelHandle<Assistant>,
    editor: ViewHandle<Editor>,
    blocks: HashSet<BlockId>,
    scroll_position: Option<ScrollPosition>,
    _subscriptions: Vec<Subscription>,
}

impl AssistantEditor {
    fn new(
        api_key: Rc<RefCell<Option<String>>>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let assistant = cx.add_model(|cx| Assistant::new(api_key, language_registry, cx));
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(assistant.read(cx).buffer.clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_gutter(false, cx);
            editor
        });

        let _subscriptions = vec![
            cx.observe(&assistant, |_, _, cx| cx.notify()),
            cx.subscribe(&assistant, Self::handle_assistant_event),
            cx.subscribe(&editor, Self::handle_editor_event),
        ];

        let mut this = Self {
            assistant,
            editor,
            blocks: Default::default(),
            scroll_position: None,
            _subscriptions,
        };
        this.update_message_headers(cx);
        this
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        let user_message = self.assistant.update(cx, |assistant, cx| {
            let editor = self.editor.read(cx);
            let newest_selection = editor
                .selections
                .newest_anchor()
                .head()
                .to_offset(&editor.buffer().read(cx).snapshot(cx));
            let message_id = assistant.message_id_for_offset(newest_selection, cx)?;
            let metadata = assistant.messages_metadata.get(&message_id)?;
            let user_message = if metadata.role == Role::User {
                let (_, user_message) = assistant.assist(cx)?;
                user_message
            } else {
                let user_message = assistant.insert_message_after(message_id, Role::User, cx)?;
                user_message
            };
            Some(user_message)
        });

        if let Some(user_message) = user_message {
            let cursor = user_message
                .start
                .to_offset(&self.assistant.read(cx).buffer.read(cx));
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Fit)),
                    cx,
                    |selections| selections.select_ranges([cursor..cursor]),
                );
            });
        }
    }

    fn cancel_last_assist(&mut self, _: &editor::Cancel, cx: &mut ViewContext<Self>) {
        if !self
            .assistant
            .update(cx, |assistant, _| assistant.cancel_last_assist())
        {
            cx.propagate_action();
        }
    }

    fn handle_assistant_event(
        &mut self,
        _: ModelHandle<Assistant>,
        event: &AssistantEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            AssistantEvent::MessagesEdited => self.update_message_headers(cx),
            AssistantEvent::SummaryChanged => {
                cx.emit(AssistantEditorEvent::TabContentChanged);
            }
            AssistantEvent::StreamedCompletion => {
                self.editor.update(cx, |editor, cx| {
                    if let Some(scroll_position) = self.scroll_position {
                        let snapshot = editor.snapshot(cx);
                        let cursor_point = scroll_position.cursor.to_display_point(&snapshot);
                        let scroll_top =
                            cursor_point.row() as f32 - scroll_position.offset_before_cursor.y();
                        editor.set_scroll_position(
                            vec2f(scroll_position.offset_before_cursor.x(), scroll_top),
                            cx,
                        );
                    }
                });
            }
        }
    }

    fn handle_editor_event(
        &mut self,
        _: ViewHandle<Editor>,
        event: &editor::Event,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            editor::Event::ScrollPositionChanged { autoscroll, .. } => {
                let cursor_scroll_position = self.cursor_scroll_position(cx);
                if *autoscroll {
                    self.scroll_position = cursor_scroll_position;
                } else if self.scroll_position != cursor_scroll_position {
                    self.scroll_position = None;
                }
            }
            editor::Event::SelectionsChanged { .. } => {
                self.scroll_position = self.cursor_scroll_position(cx);
            }
            _ => {}
        }
    }

    fn cursor_scroll_position(&self, cx: &mut ViewContext<Self>) -> Option<ScrollPosition> {
        self.editor.update(cx, |editor, cx| {
            let snapshot = editor.snapshot(cx);
            let cursor = editor.selections.newest_anchor().head();
            let cursor_row = cursor.to_display_point(&snapshot.display_snapshot).row() as f32;
            let scroll_position = editor
                .scroll_manager
                .anchor()
                .scroll_position(&snapshot.display_snapshot);

            let scroll_bottom = scroll_position.y() + editor.visible_line_count().unwrap_or(0.);
            if (scroll_position.y()..scroll_bottom).contains(&cursor_row) {
                Some(ScrollPosition {
                    cursor,
                    offset_before_cursor: vec2f(
                        scroll_position.x(),
                        cursor_row - scroll_position.y(),
                    ),
                })
            } else {
                None
            }
        })
    }

    fn update_message_headers(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |editor, cx| {
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let excerpt_id = *buffer.as_singleton().unwrap().0;
            let old_blocks = std::mem::take(&mut self.blocks);
            let new_blocks = self
                .assistant
                .read(cx)
                .messages(cx)
                .map(|(message, metadata, _)| BlockProperties {
                    position: buffer.anchor_in_excerpt(excerpt_id, message.start),
                    height: 2,
                    style: BlockStyle::Sticky,
                    render: Arc::new({
                        let assistant = self.assistant.clone();
                        let metadata = metadata.clone();
                        let message = message.clone();
                        move |cx| {
                            enum Sender {}
                            enum ErrorTooltip {}

                            let theme = theme::current(cx);
                            let style = &theme.assistant;
                            let message_id = message.id;
                            let sender = MouseEventHandler::<Sender, _>::new(
                                message_id.0,
                                cx,
                                |state, _| match metadata.role {
                                    Role::User => {
                                        let style = style.user_sender.style_for(state, false);
                                        Label::new("You", style.text.clone())
                                            .contained()
                                            .with_style(style.container)
                                    }
                                    Role::Assistant => {
                                        let style = style.assistant_sender.style_for(state, false);
                                        Label::new("Assistant", style.text.clone())
                                            .contained()
                                            .with_style(style.container)
                                    }
                                    Role::System => {
                                        let style = style.system_sender.style_for(state, false);
                                        Label::new("System", style.text.clone())
                                            .contained()
                                            .with_style(style.container)
                                    }
                                },
                            )
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_down(MouseButton::Left, {
                                let assistant = assistant.clone();
                                move |_, _, cx| {
                                    assistant.update(cx, |assistant, cx| {
                                        assistant.cycle_message_role(message_id, cx)
                                    })
                                }
                            });

                            Flex::row()
                                .with_child(sender.aligned())
                                .with_child(
                                    Label::new(
                                        metadata.sent_at.format("%I:%M%P").to_string(),
                                        style.sent_at.text.clone(),
                                    )
                                    .contained()
                                    .with_style(style.sent_at.container)
                                    .aligned(),
                                )
                                .with_children(metadata.error.clone().map(|error| {
                                    Svg::new("icons/circle_x_mark_12.svg")
                                        .with_color(style.error_icon.color)
                                        .constrained()
                                        .with_width(style.error_icon.width)
                                        .contained()
                                        .with_style(style.error_icon.container)
                                        .with_tooltip::<ErrorTooltip>(
                                            message_id.0,
                                            error,
                                            None,
                                            theme.tooltip.clone(),
                                            cx,
                                        )
                                        .aligned()
                                }))
                                .aligned()
                                .left()
                                .contained()
                                .with_style(style.header)
                                .into_any()
                        }
                    }),
                    disposition: BlockDisposition::Above,
                })
                .collect::<Vec<_>>();

            editor.remove_blocks(old_blocks, None, cx);
            let ids = editor.insert_blocks(new_blocks, None, cx);
            self.blocks = HashSet::from_iter(ids);
        });
    }

    fn quote_selection(
        workspace: &mut Workspace,
        _: &QuoteSelection,
        cx: &mut ViewContext<Workspace>,
    ) {
        let Some(panel) = workspace.panel::<AssistantPanel>(cx) else {
            return;
        };
        let Some(editor) = workspace.active_item(cx).and_then(|item| item.downcast::<Editor>()) else {
            return;
        };

        let text = editor.read_with(cx, |editor, cx| {
            let range = editor.selections.newest::<usize>(cx).range();
            let buffer = editor.buffer().read(cx).snapshot(cx);
            let start_language = buffer.language_at(range.start);
            let end_language = buffer.language_at(range.end);
            let language_name = if start_language == end_language {
                start_language.map(|language| language.name())
            } else {
                None
            };
            let language_name = language_name.as_deref().unwrap_or("").to_lowercase();

            let selected_text = buffer.text_for_range(range).collect::<String>();
            if selected_text.is_empty() {
                None
            } else {
                Some(if language_name == "markdown" {
                    selected_text
                        .lines()
                        .map(|line| format!("> {}", line))
                        .collect::<Vec<_>>()
                        .join("\n")
                } else {
                    format!("```{language_name}\n{selected_text}\n```")
                })
            }
        });

        // Activate the panel
        if !panel.read(cx).has_focus(cx) {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        }

        if let Some(text) = text {
            panel.update(cx, |panel, cx| {
                if let Some(assistant) = panel
                    .pane
                    .read(cx)
                    .active_item()
                    .and_then(|item| item.downcast::<AssistantEditor>())
                    .ok_or_else(|| anyhow!("no active context"))
                    .log_err()
                {
                    assistant.update(cx, |assistant, cx| {
                        assistant
                            .editor
                            .update(cx, |editor, cx| editor.insert(&text, cx))
                    });
                }
            });
        }
    }

    fn copy(&mut self, _: &editor::Copy, cx: &mut ViewContext<Self>) {
        let editor = self.editor.read(cx);
        let assistant = self.assistant.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let mut copied_text = String::new();
            let mut spanned_messages = 0;
            for (_message, metadata, message_range) in assistant.messages(cx) {
                if message_range.start >= selection.range().end {
                    break;
                } else if message_range.end >= selection.range().start {
                    let range = cmp::max(message_range.start, selection.range().start)
                        ..cmp::min(message_range.end, selection.range().end);
                    if !range.is_empty() {
                        spanned_messages += 1;
                        write!(&mut copied_text, "## {}\n\n", metadata.role).unwrap();
                        for chunk in assistant.buffer.read(cx).text_for_range(range) {
                            copied_text.push_str(&chunk);
                        }
                        copied_text.push('\n');
                    }
                }
            }

            if spanned_messages > 1 {
                cx.platform()
                    .write_to_clipboard(ClipboardItem::new(copied_text));
                return;
            }
        }

        cx.propagate_action();
    }

    fn cycle_model(&mut self, cx: &mut ViewContext<Self>) {
        self.assistant.update(cx, |assistant, cx| {
            let new_model = match assistant.model.as_str() {
                "gpt-4" => "gpt-3.5-turbo",
                _ => "gpt-4",
            };
            assistant.set_model(new_model.into(), cx);
        });
    }

    fn title(&self, cx: &AppContext) -> String {
        self.assistant
            .read(cx)
            .summary
            .clone()
            .unwrap_or_else(|| "New Context".into())
    }
}

impl Entity for AssistantEditor {
    type Event = AssistantEditorEvent;
}

impl View for AssistantEditor {
    fn ui_name() -> &'static str {
        "AssistantEditor"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        enum Model {}
        let theme = &theme::current(cx).assistant;
        let assistant = &self.assistant.read(cx);
        let model = assistant.model.clone();
        let remaining_tokens = assistant.remaining_tokens().map(|remaining_tokens| {
            let remaining_tokens_style = if remaining_tokens <= 0 {
                &theme.no_remaining_tokens
            } else {
                &theme.remaining_tokens
            };
            Label::new(
                remaining_tokens.to_string(),
                remaining_tokens_style.text.clone(),
            )
            .contained()
            .with_style(remaining_tokens_style.container)
        });

        Stack::new()
            .with_child(
                ChildView::new(&self.editor, cx)
                    .contained()
                    .with_style(theme.container),
            )
            .with_child(
                Flex::row()
                    .with_child(
                        MouseEventHandler::<Model, _>::new(0, cx, |state, _| {
                            let style = theme.model.style_for(state, false);
                            Label::new(model, style.text.clone())
                                .contained()
                                .with_style(style.container)
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, |_, this, cx| this.cycle_model(cx)),
                    )
                    .with_children(remaining_tokens)
                    .contained()
                    .with_style(theme.model_info_container)
                    .aligned()
                    .top()
                    .right(),
            )
            .into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.editor);
        }
    }
}

impl Item for AssistantEditor {
    fn tab_content<V: View>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        cx: &gpui::AppContext,
    ) -> AnyElement<V> {
        let title = truncate_and_trailoff(&self.title(cx), editor::MAX_TAB_TITLE_LEN);
        Label::new(title, style.label.clone()).into_any()
    }

    fn tab_tooltip_text(&self, cx: &AppContext) -> Option<Cow<str>> {
        Some(self.title(cx).into())
    }

    fn as_searchable(
        &self,
        _: &ViewHandle<Self>,
    ) -> Option<Box<dyn workspace::searchable::SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash)]
struct MessageId(usize);

#[derive(Clone, Debug)]
struct Message {
    id: MessageId,
    start: language::Anchor,
}

#[derive(Clone, Debug)]
struct MessageMetadata {
    role: Role,
    sent_at: DateTime<Local>,
    error: Option<String>,
}

async fn stream_completion(
    api_key: String,
    executor: Arc<Background>,
    mut request: OpenAIRequest,
) -> Result<impl Stream<Item = Result<OpenAIResponseStreamEvent>>> {
    request.stream = true;

    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAIResponseStreamEvent>>();

    let json_data = serde_json::to_string(&request)?;
    let mut response = Request::post(format!("{OPENAI_API_URL}/chat/completions"))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .body(json_data)?
        .send_async()
        .await?;

    let status = response.status();
    if status == StatusCode::OK {
        executor
            .spawn(async move {
                let mut lines = BufReader::new(response.body_mut()).lines();

                fn parse_line(
                    line: Result<String, io::Error>,
                ) -> Result<Option<OpenAIResponseStreamEvent>> {
                    if let Some(data) = line?.strip_prefix("data: ") {
                        let event = serde_json::from_str(&data)?;
                        Ok(Some(event))
                    } else {
                        Ok(None)
                    }
                }

                while let Some(line) = lines.next().await {
                    if let Some(event) = parse_line(line).transpose() {
                        let done = event.as_ref().map_or(false, |event| {
                            event
                                .choices
                                .last()
                                .map_or(false, |choice| choice.finish_reason.is_some())
                        });
                        if tx.unbounded_send(event).is_err() {
                            break;
                        }

                        if done {
                            break;
                        }
                    }
                }

                anyhow::Ok(())
            })
            .detach();

        Ok(rx)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        #[derive(Deserialize)]
        struct OpenAIResponse {
            error: OpenAIError,
        }

        #[derive(Deserialize)]
        struct OpenAIError {
            message: String,
        }

        match serde_json::from_str::<OpenAIResponse>(&body) {
            Ok(response) if !response.error.message.is_empty() => Err(anyhow!(
                "Failed to connect to OpenAI API: {}",
                response.error.message,
            )),

            _ => Err(anyhow!(
                "Failed to connect to OpenAI API: {} {}",
                response.status(),
                body,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::AppContext;

    #[gpui::test]
    fn test_inserting_and_removing_messages(cx: &mut AppContext) {
        let registry = Arc::new(LanguageRegistry::test());
        let assistant = cx.add_model(|cx| Assistant::new(Default::default(), registry, cx));
        let buffer = assistant.read(cx).buffer.clone();

        let message_1 = assistant.read(cx).messages[0].clone();
        assert_eq!(
            messages(&assistant, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        let message_2 = assistant.update(cx, |assistant, cx| {
            assistant
                .insert_message_after(message_1.id, Role::Assistant, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..1),
                (message_2.id, Role::Assistant, 1..1)
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1"), (1..1, "2")], None, cx)
        });
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..3)
            ]
        );

        let message_3 = assistant.update(cx, |assistant, cx| {
            assistant
                .insert_message_after(message_2.id, Role::User, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_3.id, Role::User, 4..4)
            ]
        );

        let message_4 = assistant.update(cx, |assistant, cx| {
            assistant
                .insert_message_after(message_2.id, Role::User, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..5),
                (message_3.id, Role::User, 5..5),
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(4..4, "C"), (5..5, "D")], None, cx)
        });
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..6),
                (message_3.id, Role::User, 6..7),
            ]
        );

        // Deleting across message boundaries merges the messages.
        buffer.update(cx, |buffer, cx| buffer.edit([(1..4, "")], None, cx));
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Undoing the deletion should also undo the merge.
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_4.id, Role::User, 4..6),
                (message_3.id, Role::User, 6..7),
            ]
        );

        // Redoing the deletion should also redo the merge.
        buffer.update(cx, |buffer, cx| buffer.redo(cx));
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Ensure we can still insert after a merged message.
        let message_5 = assistant.update(cx, |assistant, cx| {
            assistant
                .insert_message_after(message_1.id, Role::System, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&assistant, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_5.id, Role::System, 3..4),
                (message_3.id, Role::User, 4..5)
            ]
        );
    }

    fn messages(
        assistant: &ModelHandle<Assistant>,
        cx: &AppContext,
    ) -> Vec<(MessageId, Role, Range<usize>)> {
        assistant
            .read(cx)
            .messages(cx)
            .map(|(message, metadata, range)| (message.id, metadata.role, range))
            .collect()
    }
}
