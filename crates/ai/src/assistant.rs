use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    OpenAIRequest, OpenAIResponseStreamEvent, RequestMessage, Role,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use collections::HashMap;
use editor::{Editor, ExcerptId, ExcerptRange, MultiBuffer};
use fs::Fs;
use futures::{io::BufReader, AsyncBufReadExt, AsyncReadExt, Stream, StreamExt};
use gpui::{
    actions, elements::*, executor::Background, Action, AppContext, AsyncAppContext, Entity,
    ModelContext, ModelHandle, Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle,
    WindowContext,
};
use isahc::{http::StatusCode, Request, RequestExt};
use language::{language_settings::SoftWrap, Buffer, LanguageRegistry};
use settings::SettingsStore;
use std::{io, sync::Arc};
use util::{post_inc, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    item::Item,
    pane, Pane, Workspace,
};

actions!(assistant, [NewContext, Assist, QuoteSelection, ToggleFocus]);

pub fn init(cx: &mut AppContext) {
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
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    _subscriptions: Vec<Subscription>,
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
                        languages: workspace.app_state().languages.clone(),
                        fs: workspace.app_state().fs.clone(),
                        width: None,
                        height: None,
                        _subscriptions: Default::default(),
                    };

                    let mut old_dock_position = this.position(cx);
                    let mut old_openai_api_key = settings::get::<AssistantSettings>(cx)
                        .openai_api_key
                        .clone();
                    this._subscriptions = vec![
                        cx.observe(&this.pane, |_, _, cx| cx.notify()),
                        cx.subscribe(&this.pane, Self::handle_pane_event),
                        cx.observe_global::<SettingsStore, _>(move |this, cx| {
                            let new_dock_position = this.position(cx);
                            if new_dock_position != old_dock_position {
                                old_dock_position = new_dock_position;
                                cx.emit(AssistantPanelEvent::DockPositionChanged);
                            }

                            let new_openai_api_key = settings::get::<AssistantSettings>(cx)
                                .openai_api_key
                                .clone();
                            if old_openai_api_key != new_openai_api_key {
                                old_openai_api_key = new_openai_api_key;
                                cx.notify();
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
        let editor = cx.add_view(|cx| AssistantEditor::new(self.languages.clone(), cx));
        self.pane.update(cx, |pane, cx| {
            pane.add_item(Box::new(editor), true, focus, None, cx)
        });
    }
}

impl Entity for AssistantPanel {
    type Event = AssistantPanelEvent;
}

impl View for AssistantPanel {
    fn ui_name() -> &'static str {
        "AssistantPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        ChildView::new(&self.pane, cx).into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.pane);
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
        if active && self.pane.read(cx).items_len() == 0 {
            self.add_context(cx);
        }
    }

    fn icon_path(&self) -> &'static str {
        "icons/speech_bubble_12.svg"
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
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, AssistantPanelEvent::Focus)
    }
}

struct Assistant {
    buffer: ModelHandle<MultiBuffer>,
    messages: Vec<Message>,
    messages_by_id: HashMap<ExcerptId, Message>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    languages: Arc<LanguageRegistry>,
}

impl Entity for Assistant {
    type Event = ();
}

impl Assistant {
    fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ModelContext<Self>) -> Self {
        let mut this = Self {
            buffer: cx.add_model(|_| MultiBuffer::new(0)),
            messages: Default::default(),
            messages_by_id: Default::default(),
            completion_count: Default::default(),
            pending_completions: Default::default(),
            languages: language_registry,
        };
        this.push_message(Role::User, cx);
        this
    }

    fn assist(&mut self, cx: &mut ModelContext<Self>) {
        let messages = self
            .messages
            .iter()
            .map(|message| RequestMessage {
                role: message.role,
                content: message.content.read(cx).text(),
            })
            .collect();
        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".into(),
            messages,
            stream: true,
        };

        if let Some(api_key) = settings::get::<AssistantSettings>(cx)
            .openai_api_key
            .clone()
        {
            let stream = stream_completion(api_key, cx.background().clone(), request);
            let response = self.push_message(Role::Assistant, cx);
            self.push_message(Role::User, cx);
            let task = cx.spawn(|this, mut cx| {
                async move {
                    let mut messages = stream.await?;

                    while let Some(message) = messages.next().await {
                        let mut message = message?;
                        if let Some(choice) = message.choices.pop() {
                            response.content.update(&mut cx, |content, cx| {
                                let text: Arc<str> = choice.delta.content?.into();
                                content.edit([(content.len()..content.len(), text)], None, cx);
                                Some(())
                            });
                        }
                    }

                    this.update(&mut cx, |this, _| {
                        this.pending_completions
                            .retain(|completion| completion.id != this.completion_count);
                    });

                    anyhow::Ok(())
                }
                .log_err()
            });

            self.pending_completions.push(PendingCompletion {
                id: post_inc(&mut self.completion_count),
                _task: task,
            });
        }
    }

    fn cancel_last_assist(&mut self) -> bool {
        self.pending_completions.pop().is_some()
    }

    fn push_message(&mut self, role: Role, cx: &mut ModelContext<Self>) -> Message {
        let content = cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, "", cx);
            let markdown = self.languages.language_for_name("Markdown");
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
            buffer.set_language_registry(self.languages.clone());
            buffer
        });
        let excerpt_id = self.buffer.update(cx, |buffer, cx| {
            buffer
                .push_excerpts(
                    content.clone(),
                    vec![ExcerptRange {
                        context: 0..0,
                        primary: None,
                    }],
                    cx,
                )
                .pop()
                .unwrap()
        });

        let message = Message {
            role,
            content: content.clone(),
            sent_at: Local::now(),
        };
        self.messages.push(message.clone());
        self.messages_by_id.insert(excerpt_id, message.clone());
        message
    }
}

struct PendingCompletion {
    id: usize,
    _task: Task<Option<()>>,
}

struct AssistantEditor {
    assistant: ModelHandle<Assistant>,
    editor: ViewHandle<Editor>,
}

impl AssistantEditor {
    fn new(language_registry: Arc<LanguageRegistry>, cx: &mut ViewContext<Self>) -> Self {
        let assistant = cx.add_model(|cx| Assistant::new(language_registry, cx));
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_multibuffer(assistant.read(cx).buffer.clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_gutter(false, cx);
            editor.set_render_excerpt_header(
                {
                    let assistant = assistant.clone();
                    move |_editor, params: editor::RenderExcerptHeaderParams, cx| {
                        let style = &theme::current(cx).assistant;
                        if let Some(message) = assistant.read(cx).messages_by_id.get(&params.id) {
                            let sender = match message.role {
                                Role::User => Label::new("You", style.user_sender.text.clone())
                                    .contained()
                                    .with_style(style.user_sender.container),
                                Role::Assistant => {
                                    Label::new("Assistant", style.assistant_sender.text.clone())
                                        .contained()
                                        .with_style(style.assistant_sender.container)
                                }
                                Role::System => {
                                    Label::new("System", style.assistant_sender.text.clone())
                                        .contained()
                                        .with_style(style.assistant_sender.container)
                                }
                            };

                            Flex::row()
                                .with_child(sender.aligned())
                                .with_child(
                                    Label::new(
                                        message.sent_at.format("%I:%M%P").to_string(),
                                        style.sent_at.text.clone(),
                                    )
                                    .contained()
                                    .with_style(style.sent_at.container)
                                    .aligned(),
                                )
                                .aligned()
                                .left()
                                .contained()
                                .with_style(style.header)
                                .into_any()
                        } else {
                            Empty::new().into_any()
                        }
                    }
                },
                cx,
            );
            editor
        });
        Self { assistant, editor }
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        self.assistant
            .update(cx, |assistant, cx| assistant.assist(cx));
    }

    fn cancel_last_assist(&mut self, _: &editor::Cancel, cx: &mut ViewContext<Self>) {
        if !self
            .assistant
            .update(cx, |assistant, _| assistant.cancel_last_assist())
        {
            cx.propagate_action();
        }
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
}

impl Entity for AssistantEditor {
    type Event = ();
}

impl View for AssistantEditor {
    fn ui_name() -> &'static str {
        "ContextEditor"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &theme::current(cx).assistant;

        ChildView::new(&self.editor, cx)
            .contained()
            .with_style(theme.container)
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
        _: &gpui::AppContext,
    ) -> AnyElement<V> {
        Label::new("New Context", style.label.clone()).into_any()
    }
}

#[derive(Clone)]
struct Message {
    role: Role,
    content: ModelHandle<Buffer>,
    sent_at: DateTime<Local>,
}

async fn stream_completion(
    api_key: String,
    executor: Arc<Background>,
    mut request: OpenAIRequest,
) -> Result<impl Stream<Item = Result<OpenAIResponseStreamEvent>>> {
    request.stream = true;

    let (tx, rx) = futures::channel::mpsc::unbounded::<Result<OpenAIResponseStreamEvent>>();

    let json_data = serde_json::to_string(&request)?;
    let mut response = Request::post("https://api.openai.com/v1/chat/completions")
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
                        tx.unbounded_send(event).log_err();
                    }
                }

                anyhow::Ok(())
            })
            .detach();

        Ok(rx)
    } else {
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Err(anyhow!(
            "Failed to connect to OpenAI API: {} {}",
            response.status(),
            body,
        ))
    }
}
