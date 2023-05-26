use crate::{OpenAIRequest, OpenAIResponseStreamEvent, RequestMessage, Role};
use anyhow::{anyhow, Result};
use editor::{Editor, MultiBuffer};
use futures::{io::BufReader, AsyncBufReadExt, AsyncReadExt, Stream, StreamExt};
use gpui::{
    actions, elements::*, executor::Background, Action, AppContext, Entity, ModelHandle,
    Subscription, Task, View, ViewContext, ViewHandle, WeakViewHandle, WindowContext,
};
use isahc::{http::StatusCode, Request, RequestExt};
use language::{language_settings::SoftWrap, Anchor, Buffer};
use std::{io, sync::Arc};
use util::{post_inc, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    item::Item,
    pane, Pane, Workspace,
};

actions!(assistant, [NewContext, Assist, CancelLastAssist]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(Assistant::assist);
    cx.capture_action(Assistant::cancel_last_assist);
}

pub enum AssistantPanelEvent {
    ZoomIn,
    ZoomOut,
    Focus,
    Close,
}

pub struct AssistantPanel {
    width: Option<f32>,
    pane: ViewHandle<Pane>,
    workspace: WeakViewHandle<Workspace>,
    _subscriptions: Vec<Subscription>,
}

impl AssistantPanel {
    pub fn new(workspace: &Workspace, cx: &mut ViewContext<Self>) -> Self {
        let pane = cx.add_view(|cx| {
            let mut pane = Pane::new(
                workspace.weak_handle(),
                workspace.app_state().background_actions,
                Default::default(),
                cx,
            );
            pane.set_can_split(false, cx);
            pane.set_can_navigate(false, cx);
            pane.on_can_drop(move |_, _| false);
            pane.set_render_tab_bar_buttons(cx, move |pane, cx| {
                Flex::row()
                    .with_child(Pane::render_tab_bar_button(
                        0,
                        "icons/plus_12.svg",
                        Some(("New Context".into(), Some(Box::new(NewContext)))),
                        cx,
                        move |_, _| todo!(),
                        None,
                    ))
                    .with_child(Pane::render_tab_bar_button(
                        1,
                        if pane.is_zoomed() {
                            "icons/minimize_8.svg"
                        } else {
                            "icons/maximize_8.svg"
                        },
                        Some(("Toggle Zoom".into(), Some(Box::new(workspace::ToggleZoom)))),
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
        let subscriptions = vec![
            cx.observe(&pane, |_, _, cx| cx.notify()),
            cx.subscribe(&pane, Self::handle_pane_event),
        ];

        Self {
            pane,
            workspace: workspace.weak_handle(),
            width: None,
            _subscriptions: subscriptions,
        }
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
}

impl Panel for AssistantPanel {
    fn position(&self, _: &WindowContext) -> DockPosition {
        DockPosition::Right
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Right)
    }

    fn set_position(&mut self, _: DockPosition, _: &mut ViewContext<Self>) {
        // TODO!
    }

    fn size(&self, _: &WindowContext) -> f32 {
        self.width.unwrap_or(480.)
    }

    fn set_size(&mut self, size: f32, cx: &mut ViewContext<Self>) {
        self.width = Some(size);
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
            cx.defer(|this, cx| {
                if let Some(workspace) = this.workspace.upgrade(cx) {
                    workspace.update(cx, |workspace, cx| {
                        let focus = this.pane.read(cx).has_focus();
                        let editor = Box::new(cx.add_view(|cx| Assistant::new(cx)));
                        Pane::add_item(workspace, &this.pane, editor, true, focus, None, cx);
                    })
                }
            });
        }
    }

    fn icon_path(&self) -> &'static str {
        "icons/speech_bubble_12.svg"
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn Action>>) {
        ("Assistant Panel".into(), None)
    }

    fn should_change_position_on_event(_: &Self::Event) -> bool {
        // TODO!
        false
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
    messages: Vec<Message>,
    editor: ViewHandle<Editor>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
}

struct PendingCompletion {
    id: usize,
    _task: Task<Option<()>>,
}

impl Assistant {
    fn new(cx: &mut ViewContext<Self>) -> Self {
        let messages = vec![Message {
            role: Role::User,
            content: cx.add_model(|cx| Buffer::new(0, "", cx)),
        }];

        let multibuffer = cx.add_model(|cx| {
            let mut multibuffer = MultiBuffer::new(0);
            for message in &messages {
                multibuffer.push_excerpts_with_context_lines(
                    message.content.clone(),
                    vec![Anchor::MIN..Anchor::MAX],
                    0,
                    cx,
                );
            }
            multibuffer
        });
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_multibuffer(multibuffer, None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_gutter(false, cx);
            editor
        });

        Self {
            messages,
            editor,
            completion_count: 0,
            pending_completions: Vec::new(),
        }
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
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

        if let Some(api_key) = std::env::var("OPENAI_API_KEY").log_err() {
            let stream = stream_completion(api_key, cx.background_executor().clone(), request);
            let response_buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
            self.messages.push(Message {
                role: Role::Assistant,
                content: response_buffer.clone(),
            });
            let next_request_buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
            self.messages.push(Message {
                role: Role::User,
                content: next_request_buffer.clone(),
            });
            self.editor.update(cx, |editor, cx| {
                editor.buffer().update(cx, |multibuffer, cx| {
                    multibuffer.push_excerpts_with_context_lines(
                        response_buffer.clone(),
                        vec![Anchor::MIN..Anchor::MAX],
                        0,
                        cx,
                    );
                    multibuffer.push_excerpts_with_context_lines(
                        next_request_buffer,
                        vec![Anchor::MIN..Anchor::MAX],
                        0,
                        cx,
                    );
                });
            });
            let task = cx.spawn(|this, mut cx| {
                async move {
                    let mut messages = stream.await?;

                    while let Some(message) = messages.next().await {
                        let mut message = message?;
                        if let Some(choice) = message.choices.pop() {
                            response_buffer.update(&mut cx, |content, cx| {
                                let text: Arc<str> = choice.delta.content?.into();
                                content.edit([(content.len()..content.len(), text)], None, cx);
                                Some(())
                            });
                        }
                    }

                    this.update(&mut cx, |this, _| {
                        this.pending_completions
                            .retain(|completion| completion.id != this.completion_count);
                    })
                    .ok();

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

    fn cancel_last_assist(&mut self, _: &editor::Cancel, cx: &mut ViewContext<Self>) {
        if self.pending_completions.pop().is_none() {
            cx.propagate_action();
        }
    }
}

impl Entity for Assistant {
    type Event = ();
}

impl View for Assistant {
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
}

impl Item for Assistant {
    fn tab_content<V: View>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &gpui::AppContext,
    ) -> AnyElement<V> {
        Label::new("New Context", style.label.clone()).into_any()
    }
}

struct Message {
    role: Role,
    content: ModelHandle<Buffer>,
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
