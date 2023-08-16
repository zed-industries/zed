use crate::{
    assistant_settings::{AssistantDockPosition, AssistantSettings},
    MessageId, MessageMetadata, MessageStatus, OpenAIRequest, OpenAIResponseStreamEvent,
    RequestMessage, Role, SavedConversation, SavedConversationMetadata, SavedMessage,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Local};
use collections::{HashMap, HashSet};
use editor::{
    display_map::{BlockDisposition, BlockId, BlockProperties, BlockStyle, ToDisplayPoint},
    scroll::autoscroll::{Autoscroll, AutoscrollStrategy},
    Anchor, Editor, ToOffset,
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
use search::BufferSearchBar;
use serde::Deserialize;
use settings::SettingsStore;
use std::{
    cell::RefCell,
    cmp, env,
    fmt::Write,
    io, iter,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    sync::Arc,
    time::Duration,
};
use theme::AssistantStyle;
use util::{paths::CONVERSATIONS_DIR, post_inc, ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    searchable::Direction,
    Save, ToggleZoom, Toolbar, Workspace,
};

const OPENAI_API_URL: &'static str = "https://api.openai.com/v1";

actions!(
    assistant,
    [
        NewConversation,
        Assist,
        Split,
        CycleMessageRole,
        QuoteSelection,
        ToggleFocus,
        ResetKey,
    ]
);

pub fn init(cx: &mut AppContext) {
    settings::register::<AssistantSettings>(cx);
    cx.add_action(
        |this: &mut AssistantPanel,
         _: &workspace::NewFile,
         cx: &mut ViewContext<AssistantPanel>| {
            this.new_conversation(cx);
        },
    );
    cx.add_action(ConversationEditor::assist);
    cx.capture_action(ConversationEditor::cancel_last_assist);
    cx.capture_action(ConversationEditor::save);
    cx.add_action(ConversationEditor::quote_selection);
    cx.capture_action(ConversationEditor::copy);
    cx.add_action(ConversationEditor::split);
    cx.capture_action(ConversationEditor::cycle_message_role);
    cx.add_action(AssistantPanel::save_api_key);
    cx.add_action(AssistantPanel::reset_api_key);
    cx.add_action(AssistantPanel::toggle_zoom);
    cx.add_action(AssistantPanel::deploy);
    cx.add_action(AssistantPanel::select_next_match);
    cx.add_action(AssistantPanel::select_prev_match);
    cx.add_action(AssistantPanel::handle_editor_cancel);
    cx.add_action(
        |workspace: &mut Workspace, _: &ToggleFocus, cx: &mut ViewContext<Workspace>| {
            workspace.toggle_panel_focus::<AssistantPanel>(cx);
        },
    );
}

#[derive(Debug)]
pub enum AssistantPanelEvent {
    ZoomIn,
    ZoomOut,
    Focus,
    Close,
    DockPositionChanged,
}

pub struct AssistantPanel {
    workspace: WeakViewHandle<Workspace>,
    width: Option<f32>,
    height: Option<f32>,
    active_editor_index: Option<usize>,
    prev_active_editor_index: Option<usize>,
    editors: Vec<ViewHandle<ConversationEditor>>,
    saved_conversations: Vec<SavedConversationMetadata>,
    saved_conversations_list_state: UniformListState,
    zoomed: bool,
    has_focus: bool,
    toolbar: ViewHandle<Toolbar>,
    api_key: Rc<RefCell<Option<String>>>,
    api_key_editor: Option<ViewHandle<Editor>>,
    has_read_credentials: bool,
    languages: Arc<LanguageRegistry>,
    fs: Arc<dyn Fs>,
    subscriptions: Vec<Subscription>,
    _watch_saved_conversations: Task<Result<()>>,
}

impl AssistantPanel {
    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let fs = workspace.read_with(&cx, |workspace, _| workspace.app_state().fs.clone())?;
            let saved_conversations = SavedConversationMetadata::list(fs.clone())
                .await
                .log_err()
                .unwrap_or_default();

            // TODO: deserialize state.
            let workspace_handle = workspace.clone();
            workspace.update(&mut cx, |workspace, cx| {
                cx.add_view::<Self, _>(|cx| {
                    const CONVERSATION_WATCH_DURATION: Duration = Duration::from_millis(100);
                    let _watch_saved_conversations = cx.spawn(move |this, mut cx| async move {
                        let mut events = fs
                            .watch(&CONVERSATIONS_DIR, CONVERSATION_WATCH_DURATION)
                            .await;
                        while events.next().await.is_some() {
                            let saved_conversations = SavedConversationMetadata::list(fs.clone())
                                .await
                                .log_err()
                                .unwrap_or_default();
                            this.update(&mut cx, |this, cx| {
                                this.saved_conversations = saved_conversations;
                                cx.notify();
                            })
                            .ok();
                        }

                        anyhow::Ok(())
                    });

                    let toolbar = cx.add_view(|cx| {
                        let mut toolbar = Toolbar::new();
                        toolbar.set_can_navigate(false, cx);
                        toolbar.add_item(cx.add_view(|cx| BufferSearchBar::new(cx)), cx);
                        toolbar
                    });
                    let mut this = Self {
                        workspace: workspace_handle,
                        active_editor_index: Default::default(),
                        prev_active_editor_index: Default::default(),
                        editors: Default::default(),
                        saved_conversations,
                        saved_conversations_list_state: Default::default(),
                        zoomed: false,
                        has_focus: false,
                        toolbar,
                        api_key: Rc::new(RefCell::new(None)),
                        api_key_editor: None,
                        has_read_credentials: false,
                        languages: workspace.app_state().languages.clone(),
                        fs: workspace.app_state().fs.clone(),
                        width: None,
                        height: None,
                        subscriptions: Default::default(),
                        _watch_saved_conversations,
                    };

                    let mut old_dock_position = this.position(cx);
                    this.subscriptions =
                        vec![cx.observe_global::<SettingsStore, _>(move |this, cx| {
                            let new_dock_position = this.position(cx);
                            if new_dock_position != old_dock_position {
                                old_dock_position = new_dock_position;
                                cx.emit(AssistantPanelEvent::DockPositionChanged);
                            }
                        })];

                    this
                })
            })
        })
    }

    fn new_conversation(&mut self, cx: &mut ViewContext<Self>) -> ViewHandle<ConversationEditor> {
        let editor = cx.add_view(|cx| {
            ConversationEditor::new(
                self.api_key.clone(),
                self.languages.clone(),
                self.fs.clone(),
                cx,
            )
        });
        self.add_conversation(editor.clone(), cx);
        editor
    }

    fn add_conversation(
        &mut self,
        editor: ViewHandle<ConversationEditor>,
        cx: &mut ViewContext<Self>,
    ) {
        self.subscriptions
            .push(cx.subscribe(&editor, Self::handle_conversation_editor_event));

        let conversation = editor.read(cx).conversation.clone();
        self.subscriptions
            .push(cx.observe(&conversation, |_, _, cx| cx.notify()));

        let index = self.editors.len();
        self.editors.push(editor);
        self.set_active_editor_index(Some(index), cx);
    }

    fn set_active_editor_index(&mut self, index: Option<usize>, cx: &mut ViewContext<Self>) {
        self.prev_active_editor_index = self.active_editor_index;
        self.active_editor_index = index;
        if let Some(editor) = self.active_editor() {
            let editor = editor.read(cx).editor.clone();
            self.toolbar.update(cx, |toolbar, cx| {
                toolbar.set_active_item(Some(&editor), cx);
            });
            if self.has_focus(cx) {
                cx.focus(&editor);
            }
        } else {
            self.toolbar.update(cx, |toolbar, cx| {
                toolbar.set_active_item(None, cx);
            });
        }

        cx.notify();
    }

    fn handle_conversation_editor_event(
        &mut self,
        _: ViewHandle<ConversationEditor>,
        event: &ConversationEditorEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ConversationEditorEvent::TabContentChanged => cx.notify(),
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

    fn toggle_zoom(&mut self, _: &workspace::ToggleZoom, cx: &mut ViewContext<Self>) {
        if self.zoomed {
            cx.emit(AssistantPanelEvent::ZoomOut)
        } else {
            cx.emit(AssistantPanelEvent::ZoomIn)
        }
    }

    fn deploy(&mut self, action: &search::buffer_search::Deploy, cx: &mut ViewContext<Self>) {
        let mut propagate_action = true;
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |search_bar, cx| {
                if search_bar.show(cx) {
                    search_bar.search_suggested(cx);
                    if action.focus {
                        search_bar.select_query(cx);
                        cx.focus_self();
                    }
                    propagate_action = false
                }
            });
        }
        if propagate_action {
            cx.propagate_action();
        }
    }

    fn handle_editor_cancel(&mut self, _: &editor::Cancel, cx: &mut ViewContext<Self>) {
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            if !search_bar.read(cx).is_dismissed() {
                search_bar.update(cx, |search_bar, cx| {
                    search_bar.dismiss(&Default::default(), cx)
                });
                return;
            }
        }
        cx.propagate_action();
    }

    fn select_next_match(&mut self, _: &search::SelectNextMatch, cx: &mut ViewContext<Self>) {
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_match(Direction::Next, 1, cx));
        }
    }

    fn select_prev_match(&mut self, _: &search::SelectPrevMatch, cx: &mut ViewContext<Self>) {
        if let Some(search_bar) = self.toolbar.read(cx).item_of_type::<BufferSearchBar>() {
            search_bar.update(cx, |bar, cx| bar.select_match(Direction::Prev, 1, cx));
        }
    }

    fn active_editor(&self) -> Option<&ViewHandle<ConversationEditor>> {
        self.editors.get(self.active_editor_index?)
    }

    fn render_hamburger_button(cx: &mut ViewContext<Self>) -> impl Element<Self> {
        enum History {}
        let theme = theme::current(cx);
        let tooltip_style = theme::current(cx).tooltip.clone();
        MouseEventHandler::new::<History, _>(0, cx, |state, _| {
            let style = theme.assistant.hamburger_button.style_for(state);
            Svg::for_style(style.icon.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this: &mut Self, cx| {
            if this.active_editor().is_some() {
                this.set_active_editor_index(None, cx);
            } else {
                this.set_active_editor_index(this.prev_active_editor_index, cx);
            }
        })
        .with_tooltip::<History>(1, "History", None, tooltip_style, cx)
    }

    fn render_editor_tools(&self, cx: &mut ViewContext<Self>) -> Vec<AnyElement<Self>> {
        if self.active_editor().is_some() {
            vec![
                Self::render_split_button(cx).into_any(),
                Self::render_quote_button(cx).into_any(),
                Self::render_assist_button(cx).into_any(),
            ]
        } else {
            Default::default()
        }
    }

    fn render_split_button(cx: &mut ViewContext<Self>) -> impl Element<Self> {
        let theme = theme::current(cx);
        let tooltip_style = theme::current(cx).tooltip.clone();
        MouseEventHandler::new::<Split, _>(0, cx, |state, _| {
            let style = theme.assistant.split_button.style_for(state);
            Svg::for_style(style.icon.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this: &mut Self, cx| {
            if let Some(active_editor) = this.active_editor() {
                active_editor.update(cx, |editor, cx| editor.split(&Default::default(), cx));
            }
        })
        .with_tooltip::<Split>(
            1,
            "Split Message",
            Some(Box::new(Split)),
            tooltip_style,
            cx,
        )
    }

    fn render_assist_button(cx: &mut ViewContext<Self>) -> impl Element<Self> {
        let theme = theme::current(cx);
        let tooltip_style = theme::current(cx).tooltip.clone();
        MouseEventHandler::new::<Assist, _>(0, cx, |state, _| {
            let style = theme.assistant.assist_button.style_for(state);
            Svg::for_style(style.icon.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this: &mut Self, cx| {
            if let Some(active_editor) = this.active_editor() {
                active_editor.update(cx, |editor, cx| editor.assist(&Default::default(), cx));
            }
        })
        .with_tooltip::<Assist>(1, "Assist", Some(Box::new(Assist)), tooltip_style, cx)
    }

    fn render_quote_button(cx: &mut ViewContext<Self>) -> impl Element<Self> {
        let theme = theme::current(cx);
        let tooltip_style = theme::current(cx).tooltip.clone();
        MouseEventHandler::new::<QuoteSelection, _>(0, cx, |state, _| {
            let style = theme.assistant.quote_button.style_for(state);
            Svg::for_style(style.icon.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this: &mut Self, cx| {
            if let Some(workspace) = this.workspace.upgrade(cx) {
                cx.window_context().defer(move |cx| {
                    workspace.update(cx, |workspace, cx| {
                        ConversationEditor::quote_selection(workspace, &Default::default(), cx)
                    });
                });
            }
        })
        .with_tooltip::<QuoteSelection>(
            1,
            "Quote Selection",
            Some(Box::new(QuoteSelection)),
            tooltip_style,
            cx,
        )
    }

    fn render_plus_button(cx: &mut ViewContext<Self>) -> impl Element<Self> {
        let theme = theme::current(cx);
        let tooltip_style = theme::current(cx).tooltip.clone();
        MouseEventHandler::new::<NewConversation, _>(0, cx, |state, _| {
            let style = theme.assistant.plus_button.style_for(state);
            Svg::for_style(style.icon.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this: &mut Self, cx| {
            this.new_conversation(cx);
        })
        .with_tooltip::<NewConversation>(
            1,
            "New Conversation",
            Some(Box::new(NewConversation)),
            tooltip_style,
            cx,
        )
    }

    fn render_zoom_button(&self, cx: &mut ViewContext<Self>) -> impl Element<Self> {
        enum ToggleZoomButton {}

        let theme = theme::current(cx);
        let tooltip_style = theme::current(cx).tooltip.clone();
        let style = if self.zoomed {
            &theme.assistant.zoom_out_button
        } else {
            &theme.assistant.zoom_in_button
        };

        MouseEventHandler::new::<ToggleZoomButton, _>(0, cx, |state, _| {
            let style = style.style_for(state);
            Svg::for_style(style.icon.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this, cx| {
            this.toggle_zoom(&ToggleZoom, cx);
        })
        .with_tooltip::<ToggleZoom>(
            0,
            if self.zoomed { "Zoom Out" } else { "Zoom In" },
            Some(Box::new(ToggleZoom)),
            tooltip_style,
            cx,
        )
    }

    fn render_saved_conversation(
        &mut self,
        index: usize,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        let conversation = &self.saved_conversations[index];
        let path = conversation.path.clone();
        MouseEventHandler::new::<SavedConversationMetadata, _>(index, cx, move |state, cx| {
            let style = &theme::current(cx).assistant.saved_conversation;
            Flex::row()
                .with_child(
                    Label::new(
                        conversation.mtime.format("%F %I:%M%p").to_string(),
                        style.saved_at.text.clone(),
                    )
                    .aligned()
                    .contained()
                    .with_style(style.saved_at.container),
                )
                .with_child(
                    Label::new(conversation.title.clone(), style.title.text.clone())
                        .aligned()
                        .contained()
                        .with_style(style.title.container),
                )
                .contained()
                .with_style(*style.container.style_for(state))
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            this.open_conversation(path.clone(), cx)
                .detach_and_log_err(cx)
        })
    }

    fn open_conversation(&mut self, path: PathBuf, cx: &mut ViewContext<Self>) -> Task<Result<()>> {
        if let Some(ix) = self.editor_index_for_path(&path, cx) {
            self.set_active_editor_index(Some(ix), cx);
            return Task::ready(Ok(()));
        }

        let fs = self.fs.clone();
        let api_key = self.api_key.clone();
        let languages = self.languages.clone();
        cx.spawn(|this, mut cx| async move {
            let saved_conversation = fs.load(&path).await?;
            let saved_conversation = serde_json::from_str(&saved_conversation)?;
            let conversation = cx.add_model(|cx| {
                Conversation::deserialize(saved_conversation, path.clone(), api_key, languages, cx)
            });
            this.update(&mut cx, |this, cx| {
                // If, by the time we've loaded the conversation, the user has already opened
                // the same conversation, we don't want to open it again.
                if let Some(ix) = this.editor_index_for_path(&path, cx) {
                    this.set_active_editor_index(Some(ix), cx);
                } else {
                    let editor = cx
                        .add_view(|cx| ConversationEditor::for_conversation(conversation, fs, cx));
                    this.add_conversation(editor, cx);
                }
            })?;
            Ok(())
        })
    }

    fn editor_index_for_path(&self, path: &Path, cx: &AppContext) -> Option<usize> {
        self.editors
            .iter()
            .position(|editor| editor.read(cx).conversation.read(cx).path.as_deref() == Some(path))
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
        let theme = &theme::current(cx);
        let style = &theme.assistant;
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
            let title = self.active_editor().map(|editor| {
                Label::new(editor.read(cx).title(cx), style.title.text.clone())
                    .contained()
                    .with_style(style.title.container)
                    .aligned()
                    .left()
                    .flex(1., false)
            });
            let mut header = Flex::row()
                .with_child(Self::render_hamburger_button(cx).aligned())
                .with_children(title);
            if self.has_focus {
                header.add_children(
                    self.render_editor_tools(cx)
                        .into_iter()
                        .map(|tool| tool.aligned().flex_float()),
                );
                header.add_child(Self::render_plus_button(cx).aligned().flex_float());
                header.add_child(self.render_zoom_button(cx).aligned());
            }

            Flex::column()
                .with_child(
                    header
                        .contained()
                        .with_style(theme.workspace.tab_bar.container)
                        .expanded()
                        .constrained()
                        .with_height(theme.workspace.tab_bar.height),
                )
                .with_children(if self.toolbar.read(cx).hidden() {
                    None
                } else {
                    Some(ChildView::new(&self.toolbar, cx).expanded())
                })
                .with_child(if let Some(editor) = self.active_editor() {
                    ChildView::new(editor, cx).flex(1., true).into_any()
                } else {
                    UniformList::new(
                        self.saved_conversations_list_state.clone(),
                        self.saved_conversations.len(),
                        cx,
                        |this, range, items, cx| {
                            for ix in range {
                                items.push(this.render_saved_conversation(ix, cx).into_any());
                            }
                        },
                    )
                    .flex(1., true)
                    .into_any()
                })
                .into_any()
        }
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = true;
        self.toolbar
            .update(cx, |toolbar, cx| toolbar.focus_changed(true, cx));
        cx.notify();
        if cx.is_self_focused() {
            if let Some(editor) = self.active_editor() {
                cx.focus(editor);
            } else if let Some(api_key_editor) = self.api_key_editor.as_ref() {
                cx.focus(api_key_editor);
            }
        }
    }

    fn focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = false;
        self.toolbar
            .update(cx, |toolbar, cx| toolbar.focus_changed(false, cx));
        cx.notify();
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

    fn is_zoomed(&self, _: &WindowContext) -> bool {
        self.zoomed
    }

    fn set_zoomed(&mut self, zoomed: bool, cx: &mut ViewContext<Self>) {
        self.zoomed = zoomed;
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        if active {
            if self.api_key.borrow().is_none() && !self.has_read_credentials {
                self.has_read_credentials = true;
                let api_key = if let Ok(api_key) = env::var("OPENAI_API_KEY") {
                    Some(api_key)
                } else if let Some((_, api_key)) = cx
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

            if self.editors.is_empty() {
                self.new_conversation(cx);
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

    fn has_focus(&self, _: &WindowContext) -> bool {
        self.has_focus
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, AssistantPanelEvent::Focus)
    }
}

enum ConversationEvent {
    MessagesEdited,
    SummaryChanged,
    StreamedCompletion,
}

#[derive(Default)]
struct Summary {
    text: String,
    done: bool,
}

struct Conversation {
    buffer: ModelHandle<Buffer>,
    message_anchors: Vec<MessageAnchor>,
    messages_metadata: HashMap<MessageId, MessageMetadata>,
    next_message_id: MessageId,
    summary: Option<Summary>,
    pending_summary: Task<Option<()>>,
    completion_count: usize,
    pending_completions: Vec<PendingCompletion>,
    model: String,
    token_count: Option<usize>,
    max_token_count: usize,
    pending_token_count: Task<Option<()>>,
    api_key: Rc<RefCell<Option<String>>>,
    pending_save: Task<Result<()>>,
    path: Option<PathBuf>,
    _subscriptions: Vec<Subscription>,
}

impl Entity for Conversation {
    type Event = ConversationEvent;
}

impl Conversation {
    fn new(
        api_key: Rc<RefCell<Option<String>>>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let model = "gpt-3.5-turbo-0613";
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
            message_anchors: Default::default(),
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
            pending_save: Task::ready(Ok(())),
            path: None,
            api_key,
            buffer,
        };
        let message = MessageAnchor {
            id: MessageId(post_inc(&mut this.next_message_id.0)),
            start: language::Anchor::MIN,
        };
        this.message_anchors.push(message.clone());
        this.messages_metadata.insert(
            message.id,
            MessageMetadata {
                role: Role::User,
                sent_at: Local::now(),
                status: MessageStatus::Done,
            },
        );

        this.count_remaining_tokens(cx);
        this
    }

    fn serialize(&self, cx: &AppContext) -> SavedConversation {
        SavedConversation {
            zed: "conversation".into(),
            version: SavedConversation::VERSION.into(),
            text: self.buffer.read(cx).text(),
            message_metadata: self.messages_metadata.clone(),
            messages: self
                .messages(cx)
                .map(|message| SavedMessage {
                    id: message.id,
                    start: message.offset_range.start,
                })
                .collect(),
            summary: self
                .summary
                .as_ref()
                .map(|summary| summary.text.clone())
                .unwrap_or_default(),
            model: self.model.clone(),
        }
    }

    fn deserialize(
        saved_conversation: SavedConversation,
        path: PathBuf,
        api_key: Rc<RefCell<Option<String>>>,
        language_registry: Arc<LanguageRegistry>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let model = saved_conversation.model;
        let markdown = language_registry.language_for_name("Markdown");
        let mut message_anchors = Vec::new();
        let mut next_message_id = MessageId(0);
        let buffer = cx.add_model(|cx| {
            let mut buffer = Buffer::new(0, saved_conversation.text, cx);
            for message in saved_conversation.messages {
                message_anchors.push(MessageAnchor {
                    id: message.id,
                    start: buffer.anchor_before(message.start),
                });
                next_message_id = cmp::max(next_message_id, MessageId(message.id.0 + 1));
            }
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
            message_anchors,
            messages_metadata: saved_conversation.message_metadata,
            next_message_id,
            summary: Some(Summary {
                text: saved_conversation.summary,
                done: true,
            }),
            pending_summary: Task::ready(None),
            completion_count: Default::default(),
            pending_completions: Default::default(),
            token_count: None,
            max_token_count: tiktoken_rs::model::get_context_size(&model),
            pending_token_count: Task::ready(None),
            model,
            _subscriptions: vec![cx.subscribe(&buffer, Self::handle_buffer_event)],
            pending_save: Task::ready(Ok(())),
            path: Some(path),
            api_key,
            buffer,
        };
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
                cx.emit(ConversationEvent::MessagesEdited);
            }
            _ => {}
        }
    }

    fn count_remaining_tokens(&mut self, cx: &mut ModelContext<Self>) {
        let messages = self
            .messages(cx)
            .into_iter()
            .filter_map(|message| {
                Some(tiktoken_rs::ChatCompletionRequestMessage {
                    role: match message.role {
                        Role::User => "user".into(),
                        Role::Assistant => "assistant".into(),
                        Role::System => "system".into(),
                    },
                    content: self
                        .buffer
                        .read(cx)
                        .text_for_range(message.offset_range)
                        .collect(),
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
                    .ok_or_else(|| anyhow!("conversation was dropped"))?
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

    fn assist(
        &mut self,
        selected_messages: HashSet<MessageId>,
        cx: &mut ModelContext<Self>,
    ) -> Vec<MessageAnchor> {
        let mut user_messages = Vec::new();
        let mut tasks = Vec::new();

        let last_message_id = self.message_anchors.iter().rev().find_map(|message| {
            message
                .start
                .is_valid(self.buffer.read(cx))
                .then_some(message.id)
        });

        for selected_message_id in selected_messages {
            let selected_message_role =
                if let Some(metadata) = self.messages_metadata.get(&selected_message_id) {
                    metadata.role
                } else {
                    continue;
                };

            if selected_message_role == Role::Assistant {
                if let Some(user_message) = self.insert_message_after(
                    selected_message_id,
                    Role::User,
                    MessageStatus::Done,
                    cx,
                ) {
                    user_messages.push(user_message);
                } else {
                    continue;
                }
            } else {
                let request = OpenAIRequest {
                    model: self.model.clone(),
                    messages: self
                        .messages(cx)
                        .filter(|message| matches!(message.status, MessageStatus::Done))
                        .flat_map(|message| {
                            let mut system_message = None;
                            if message.id == selected_message_id {
                                system_message = Some(RequestMessage {
                                    role: Role::System,
                                    content: concat!(
                                        "Treat the following messages as additional knowledge you have learned about, ",
                                        "but act as if they were not part of this conversation. That is, treat them ",
                                        "as if the user didn't see them and couldn't possibly inquire about them."
                                    ).into()
                                });
                            }

                            Some(message.to_open_ai_message(self.buffer.read(cx))).into_iter().chain(system_message)
                        })
                        .chain(Some(RequestMessage {
                            role: Role::System,
                            content: format!(
                                "Direct your reply to message with id {}. Do not include a [Message X] header.",
                                selected_message_id.0
                            ),
                        }))
                        .collect(),
                    stream: true,
                };

                let Some(api_key) = self.api_key.borrow().clone() else { continue };
                let stream = stream_completion(api_key, cx.background().clone(), request);
                let assistant_message = self
                    .insert_message_after(
                        selected_message_id,
                        Role::Assistant,
                        MessageStatus::Pending,
                        cx,
                    )
                    .unwrap();

                // Queue up the user's next reply
                if Some(selected_message_id) == last_message_id {
                    let user_message = self
                        .insert_message_after(
                            assistant_message.id,
                            Role::User,
                            MessageStatus::Done,
                            cx,
                        )
                        .unwrap();
                    user_messages.push(user_message);
                }

                tasks.push(cx.spawn_weak({
                    |this, mut cx| async move {
                        let assistant_message_id = assistant_message.id;
                        let stream_completion = async {
                            let mut messages = stream.await?;

                            while let Some(message) = messages.next().await {
                                let mut message = message?;
                                if let Some(choice) = message.choices.pop() {
                                    this.upgrade(&cx)
                                        .ok_or_else(|| anyhow!("conversation was dropped"))?
                                        .update(&mut cx, |this, cx| {
                                            let text: Arc<str> = choice.delta.content?.into();
                                            let message_ix = this.message_anchors.iter().position(
                                                |message| message.id == assistant_message_id,
                                            )?;
                                            this.buffer.update(cx, |buffer, cx| {
                                                let offset = this.message_anchors[message_ix + 1..]
                                                    .iter()
                                                    .find(|message| message.start.is_valid(buffer))
                                                    .map_or(buffer.len(), |message| {
                                                        message
                                                            .start
                                                            .to_offset(buffer)
                                                            .saturating_sub(1)
                                                    });
                                                buffer.edit([(offset..offset, text)], None, cx);
                                            });
                                            cx.emit(ConversationEvent::StreamedCompletion);

                                            Some(())
                                        });
                                }
                                smol::future::yield_now().await;
                            }

                            this.upgrade(&cx)
                                .ok_or_else(|| anyhow!("conversation was dropped"))?
                                .update(&mut cx, |this, cx| {
                                    this.pending_completions.retain(|completion| {
                                        completion.id != this.completion_count
                                    });
                                    this.summarize(cx);
                                });

                            anyhow::Ok(())
                        };

                        let result = stream_completion.await;
                        if let Some(this) = this.upgrade(&cx) {
                            this.update(&mut cx, |this, cx| {
                                if let Some(metadata) =
                                    this.messages_metadata.get_mut(&assistant_message.id)
                                {
                                    match result {
                                        Ok(_) => {
                                            metadata.status = MessageStatus::Done;
                                        }
                                        Err(error) => {
                                            metadata.status = MessageStatus::Error(
                                                error.to_string().trim().into(),
                                            );
                                        }
                                    }
                                    cx.notify();
                                }
                            });
                        }
                    }
                }));
            }
        }

        if !tasks.is_empty() {
            self.pending_completions.push(PendingCompletion {
                id: post_inc(&mut self.completion_count),
                _tasks: tasks,
            });
        }

        user_messages
    }

    fn cancel_last_assist(&mut self) -> bool {
        self.pending_completions.pop().is_some()
    }

    fn cycle_message_roles(&mut self, ids: HashSet<MessageId>, cx: &mut ModelContext<Self>) {
        for id in ids {
            if let Some(metadata) = self.messages_metadata.get_mut(&id) {
                metadata.role.cycle();
                cx.emit(ConversationEvent::MessagesEdited);
                cx.notify();
            }
        }
    }

    fn insert_message_after(
        &mut self,
        message_id: MessageId,
        role: Role,
        status: MessageStatus,
        cx: &mut ModelContext<Self>,
    ) -> Option<MessageAnchor> {
        if let Some(prev_message_ix) = self
            .message_anchors
            .iter()
            .position(|message| message.id == message_id)
        {
            // Find the next valid message after the one we were given.
            let mut next_message_ix = prev_message_ix + 1;
            while let Some(next_message) = self.message_anchors.get(next_message_ix) {
                if next_message.start.is_valid(self.buffer.read(cx)) {
                    break;
                }
                next_message_ix += 1;
            }

            let start = self.buffer.update(cx, |buffer, cx| {
                let offset = self
                    .message_anchors
                    .get(next_message_ix)
                    .map_or(buffer.len(), |message| message.start.to_offset(buffer) - 1);
                buffer.edit([(offset..offset, "\n")], None, cx);
                buffer.anchor_before(offset + 1)
            });
            let message = MessageAnchor {
                id: MessageId(post_inc(&mut self.next_message_id.0)),
                start,
            };
            self.message_anchors
                .insert(next_message_ix, message.clone());
            self.messages_metadata.insert(
                message.id,
                MessageMetadata {
                    role,
                    sent_at: Local::now(),
                    status,
                },
            );
            cx.emit(ConversationEvent::MessagesEdited);
            Some(message)
        } else {
            None
        }
    }

    fn split_message(
        &mut self,
        range: Range<usize>,
        cx: &mut ModelContext<Self>,
    ) -> (Option<MessageAnchor>, Option<MessageAnchor>) {
        let start_message = self.message_for_offset(range.start, cx);
        let end_message = self.message_for_offset(range.end, cx);
        if let Some((start_message, end_message)) = start_message.zip(end_message) {
            // Prevent splitting when range spans multiple messages.
            if start_message.id != end_message.id {
                return (None, None);
            }

            let message = start_message;
            let role = message.role;
            let mut edited_buffer = false;

            let mut suffix_start = None;
            if range.start > message.offset_range.start && range.end < message.offset_range.end - 1
            {
                if self.buffer.read(cx).chars_at(range.end).next() == Some('\n') {
                    suffix_start = Some(range.end + 1);
                } else if self.buffer.read(cx).reversed_chars_at(range.end).next() == Some('\n') {
                    suffix_start = Some(range.end);
                }
            }

            let suffix = if let Some(suffix_start) = suffix_start {
                MessageAnchor {
                    id: MessageId(post_inc(&mut self.next_message_id.0)),
                    start: self.buffer.read(cx).anchor_before(suffix_start),
                }
            } else {
                self.buffer.update(cx, |buffer, cx| {
                    buffer.edit([(range.end..range.end, "\n")], None, cx);
                });
                edited_buffer = true;
                MessageAnchor {
                    id: MessageId(post_inc(&mut self.next_message_id.0)),
                    start: self.buffer.read(cx).anchor_before(range.end + 1),
                }
            };

            self.message_anchors
                .insert(message.index_range.end + 1, suffix.clone());
            self.messages_metadata.insert(
                suffix.id,
                MessageMetadata {
                    role,
                    sent_at: Local::now(),
                    status: MessageStatus::Done,
                },
            );

            let new_messages =
                if range.start == range.end || range.start == message.offset_range.start {
                    (None, Some(suffix))
                } else {
                    let mut prefix_end = None;
                    if range.start > message.offset_range.start
                        && range.end < message.offset_range.end - 1
                    {
                        if self.buffer.read(cx).chars_at(range.start).next() == Some('\n') {
                            prefix_end = Some(range.start + 1);
                        } else if self.buffer.read(cx).reversed_chars_at(range.start).next()
                            == Some('\n')
                        {
                            prefix_end = Some(range.start);
                        }
                    }

                    let selection = if let Some(prefix_end) = prefix_end {
                        cx.emit(ConversationEvent::MessagesEdited);
                        MessageAnchor {
                            id: MessageId(post_inc(&mut self.next_message_id.0)),
                            start: self.buffer.read(cx).anchor_before(prefix_end),
                        }
                    } else {
                        self.buffer.update(cx, |buffer, cx| {
                            buffer.edit([(range.start..range.start, "\n")], None, cx)
                        });
                        edited_buffer = true;
                        MessageAnchor {
                            id: MessageId(post_inc(&mut self.next_message_id.0)),
                            start: self.buffer.read(cx).anchor_before(range.end + 1),
                        }
                    };

                    self.message_anchors
                        .insert(message.index_range.end + 1, selection.clone());
                    self.messages_metadata.insert(
                        selection.id,
                        MessageMetadata {
                            role,
                            sent_at: Local::now(),
                            status: MessageStatus::Done,
                        },
                    );
                    (Some(selection), Some(suffix))
                };

            if !edited_buffer {
                cx.emit(ConversationEvent::MessagesEdited);
            }
            new_messages
        } else {
            (None, None)
        }
    }

    fn summarize(&mut self, cx: &mut ModelContext<Self>) {
        if self.message_anchors.len() >= 2 && self.summary.is_none() {
            let api_key = self.api_key.borrow().clone();
            if let Some(api_key) = api_key {
                let messages = self
                    .messages(cx)
                    .take(2)
                    .map(|message| message.to_open_ai_message(self.buffer.read(cx)))
                    .chain(Some(RequestMessage {
                        role: Role::User,
                        content:
                            "Summarize the conversation into a short title without punctuation"
                                .into(),
                    }));
                let request = OpenAIRequest {
                    model: self.model.clone(),
                    messages: messages.collect(),
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
                                    this.summary
                                        .get_or_insert(Default::default())
                                        .text
                                        .push_str(&text);
                                    cx.emit(ConversationEvent::SummaryChanged);
                                });
                            }
                        }

                        this.update(&mut cx, |this, cx| {
                            if let Some(summary) = this.summary.as_mut() {
                                summary.done = true;
                                cx.emit(ConversationEvent::SummaryChanged);
                            }
                        });

                        anyhow::Ok(())
                    }
                    .log_err()
                });
            }
        }
    }

    fn message_for_offset(&self, offset: usize, cx: &AppContext) -> Option<Message> {
        self.messages_for_offsets([offset], cx).pop()
    }

    fn messages_for_offsets(
        &self,
        offsets: impl IntoIterator<Item = usize>,
        cx: &AppContext,
    ) -> Vec<Message> {
        let mut result = Vec::new();

        let mut messages = self.messages(cx).peekable();
        let mut offsets = offsets.into_iter().peekable();
        let mut current_message = messages.next();
        while let Some(offset) = offsets.next() {
            // Locate the message that contains the offset.
            while current_message.as_ref().map_or(false, |message| {
                !message.offset_range.contains(&offset) && messages.peek().is_some()
            }) {
                current_message = messages.next();
            }
            let Some(message) = current_message.as_ref() else { break };

            // Skip offsets that are in the same message.
            while offsets.peek().map_or(false, |offset| {
                message.offset_range.contains(offset) || messages.peek().is_none()
            }) {
                offsets.next();
            }

            result.push(message.clone());
        }
        result
    }

    fn messages<'a>(&'a self, cx: &'a AppContext) -> impl 'a + Iterator<Item = Message> {
        let buffer = self.buffer.read(cx);
        let mut message_anchors = self.message_anchors.iter().enumerate().peekable();
        iter::from_fn(move || {
            while let Some((start_ix, message_anchor)) = message_anchors.next() {
                let metadata = self.messages_metadata.get(&message_anchor.id)?;
                let message_start = message_anchor.start.to_offset(buffer);
                let mut message_end = None;
                let mut end_ix = start_ix;
                while let Some((_, next_message)) = message_anchors.peek() {
                    if next_message.start.is_valid(buffer) {
                        message_end = Some(next_message.start);
                        break;
                    } else {
                        end_ix += 1;
                        message_anchors.next();
                    }
                }
                let message_end = message_end
                    .unwrap_or(language::Anchor::MAX)
                    .to_offset(buffer);
                return Some(Message {
                    index_range: start_ix..end_ix,
                    offset_range: message_start..message_end,
                    id: message_anchor.id,
                    anchor: message_anchor.start,
                    role: metadata.role,
                    sent_at: metadata.sent_at,
                    status: metadata.status.clone(),
                });
            }
            None
        })
    }

    fn save(
        &mut self,
        debounce: Option<Duration>,
        fs: Arc<dyn Fs>,
        cx: &mut ModelContext<Conversation>,
    ) {
        self.pending_save = cx.spawn(|this, mut cx| async move {
            if let Some(debounce) = debounce {
                cx.background().timer(debounce).await;
            }

            let (old_path, summary) = this.read_with(&cx, |this, _| {
                let path = this.path.clone();
                let summary = if let Some(summary) = this.summary.as_ref() {
                    if summary.done {
                        Some(summary.text.clone())
                    } else {
                        None
                    }
                } else {
                    None
                };
                (path, summary)
            });

            if let Some(summary) = summary {
                let conversation = this.read_with(&cx, |this, cx| this.serialize(cx));
                let path = if let Some(old_path) = old_path {
                    old_path
                } else {
                    let mut discriminant = 1;
                    let mut new_path;
                    loop {
                        new_path = CONVERSATIONS_DIR.join(&format!(
                            "{} - {}.zed.json",
                            summary.trim(),
                            discriminant
                        ));
                        if fs.is_file(&new_path).await {
                            discriminant += 1;
                        } else {
                            break;
                        }
                    }
                    new_path
                };

                fs.create_dir(CONVERSATIONS_DIR.as_ref()).await?;
                fs.atomic_write(path.clone(), serde_json::to_string(&conversation).unwrap())
                    .await?;
                this.update(&mut cx, |this, _| this.path = Some(path));
            }

            Ok(())
        });
    }
}

struct PendingCompletion {
    id: usize,
    _tasks: Vec<Task<()>>,
}

enum ConversationEditorEvent {
    TabContentChanged,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct ScrollPosition {
    offset_before_cursor: Vector2F,
    cursor: Anchor,
}

struct ConversationEditor {
    conversation: ModelHandle<Conversation>,
    fs: Arc<dyn Fs>,
    editor: ViewHandle<Editor>,
    blocks: HashSet<BlockId>,
    scroll_position: Option<ScrollPosition>,
    _subscriptions: Vec<Subscription>,
}

impl ConversationEditor {
    fn new(
        api_key: Rc<RefCell<Option<String>>>,
        language_registry: Arc<LanguageRegistry>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let conversation = cx.add_model(|cx| Conversation::new(api_key, language_registry, cx));
        Self::for_conversation(conversation, fs, cx)
    }

    fn for_conversation(
        conversation: ModelHandle<Conversation>,
        fs: Arc<dyn Fs>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(conversation.read(cx).buffer.clone(), None, cx);
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_gutter(false, cx);
            editor.set_show_wrap_guides(false, cx);
            editor
        });

        let _subscriptions = vec![
            cx.observe(&conversation, |_, _, cx| cx.notify()),
            cx.subscribe(&conversation, Self::handle_conversation_event),
            cx.subscribe(&editor, Self::handle_editor_event),
        ];

        let mut this = Self {
            conversation,
            editor,
            blocks: Default::default(),
            scroll_position: None,
            fs,
            _subscriptions,
        };
        this.update_message_headers(cx);
        this
    }

    fn assist(&mut self, _: &Assist, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);

        let user_messages = self.conversation.update(cx, |conversation, cx| {
            let selected_messages = conversation
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            conversation.assist(selected_messages, cx)
        });
        let new_selections = user_messages
            .iter()
            .map(|message| {
                let cursor = message
                    .start
                    .to_offset(self.conversation.read(cx).buffer.read(cx));
                cursor..cursor
            })
            .collect::<Vec<_>>();
        if !new_selections.is_empty() {
            self.editor.update(cx, |editor, cx| {
                editor.change_selections(
                    Some(Autoscroll::Strategy(AutoscrollStrategy::Fit)),
                    cx,
                    |selections| selections.select_ranges(new_selections),
                );
            });
            // Avoid scrolling to the new cursor position so the assistant's output is stable.
            cx.defer(|this, _| this.scroll_position = None);
        }
    }

    fn cancel_last_assist(&mut self, _: &editor::Cancel, cx: &mut ViewContext<Self>) {
        if !self
            .conversation
            .update(cx, |conversation, _| conversation.cancel_last_assist())
        {
            cx.propagate_action();
        }
    }

    fn cycle_message_role(&mut self, _: &CycleMessageRole, cx: &mut ViewContext<Self>) {
        let cursors = self.cursors(cx);
        self.conversation.update(cx, |conversation, cx| {
            let messages = conversation
                .messages_for_offsets(cursors, cx)
                .into_iter()
                .map(|message| message.id)
                .collect();
            conversation.cycle_message_roles(messages, cx)
        });
    }

    fn cursors(&self, cx: &AppContext) -> Vec<usize> {
        let selections = self.editor.read(cx).selections.all::<usize>(cx);
        selections
            .into_iter()
            .map(|selection| selection.head())
            .collect()
    }

    fn handle_conversation_event(
        &mut self,
        _: ModelHandle<Conversation>,
        event: &ConversationEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ConversationEvent::MessagesEdited => {
                self.update_message_headers(cx);
                self.conversation.update(cx, |conversation, cx| {
                    conversation.save(Some(Duration::from_millis(500)), self.fs.clone(), cx);
                });
            }
            ConversationEvent::SummaryChanged => {
                cx.emit(ConversationEditorEvent::TabContentChanged);
                self.conversation.update(cx, |conversation, cx| {
                    conversation.save(None, self.fs.clone(), cx);
                });
            }
            ConversationEvent::StreamedCompletion => {
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
                .conversation
                .read(cx)
                .messages(cx)
                .map(|message| BlockProperties {
                    position: buffer.anchor_in_excerpt(excerpt_id, message.anchor),
                    height: 2,
                    style: BlockStyle::Sticky,
                    render: Arc::new({
                        let conversation = self.conversation.clone();
                        // let metadata = message.metadata.clone();
                        // let message = message.clone();
                        move |cx| {
                            enum Sender {}
                            enum ErrorTooltip {}

                            let theme = theme::current(cx);
                            let style = &theme.assistant;
                            let message_id = message.id;
                            let sender = MouseEventHandler::new::<Sender, _>(
                                message_id.0,
                                cx,
                                |state, _| match message.role {
                                    Role::User => {
                                        let style = style.user_sender.style_for(state);
                                        Label::new("You", style.text.clone())
                                            .contained()
                                            .with_style(style.container)
                                    }
                                    Role::Assistant => {
                                        let style = style.assistant_sender.style_for(state);
                                        Label::new("Assistant", style.text.clone())
                                            .contained()
                                            .with_style(style.container)
                                    }
                                    Role::System => {
                                        let style = style.system_sender.style_for(state);
                                        Label::new("System", style.text.clone())
                                            .contained()
                                            .with_style(style.container)
                                    }
                                },
                            )
                            .with_cursor_style(CursorStyle::PointingHand)
                            .on_down(MouseButton::Left, {
                                let conversation = conversation.clone();
                                move |_, _, cx| {
                                    conversation.update(cx, |conversation, cx| {
                                        conversation.cycle_message_roles(
                                            HashSet::from_iter(Some(message_id)),
                                            cx,
                                        )
                                    })
                                }
                            });

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
                                .with_children(
                                    if let MessageStatus::Error(error) = &message.status {
                                        Some(
                                            Svg::new("icons/circle_x_mark_12.svg")
                                                .with_color(style.error_icon.color)
                                                .constrained()
                                                .with_width(style.error_icon.width)
                                                .contained()
                                                .with_style(style.error_icon.container)
                                                .with_tooltip::<ErrorTooltip>(
                                                    message_id.0,
                                                    error.to_string(),
                                                    None,
                                                    theme.tooltip.clone(),
                                                    cx,
                                                )
                                                .aligned(),
                                        )
                                    } else {
                                        None
                                    },
                                )
                                .aligned()
                                .left()
                                .contained()
                                .with_style(style.message_header)
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
        let Some(editor) = workspace.active_item(cx).and_then(|item| item.act_as::<Editor>(cx)) else {
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
                let conversation = panel
                    .active_editor()
                    .cloned()
                    .unwrap_or_else(|| panel.new_conversation(cx));
                conversation.update(cx, |conversation, cx| {
                    conversation
                        .editor
                        .update(cx, |editor, cx| editor.insert(&text, cx))
                });
            });
        }
    }

    fn copy(&mut self, _: &editor::Copy, cx: &mut ViewContext<Self>) {
        let editor = self.editor.read(cx);
        let conversation = self.conversation.read(cx);
        if editor.selections.count() == 1 {
            let selection = editor.selections.newest::<usize>(cx);
            let mut copied_text = String::new();
            let mut spanned_messages = 0;
            for message in conversation.messages(cx) {
                if message.offset_range.start >= selection.range().end {
                    break;
                } else if message.offset_range.end >= selection.range().start {
                    let range = cmp::max(message.offset_range.start, selection.range().start)
                        ..cmp::min(message.offset_range.end, selection.range().end);
                    if !range.is_empty() {
                        spanned_messages += 1;
                        write!(&mut copied_text, "## {}\n\n", message.role).unwrap();
                        for chunk in conversation.buffer.read(cx).text_for_range(range) {
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

    fn split(&mut self, _: &Split, cx: &mut ViewContext<Self>) {
        self.conversation.update(cx, |conversation, cx| {
            let selections = self.editor.read(cx).selections.disjoint_anchors();
            for selection in selections.into_iter() {
                let buffer = self.editor.read(cx).buffer().read(cx).snapshot(cx);
                let range = selection
                    .map(|endpoint| endpoint.to_offset(&buffer))
                    .range();
                conversation.split_message(range, cx);
            }
        });
    }

    fn save(&mut self, _: &Save, cx: &mut ViewContext<Self>) {
        self.conversation.update(cx, |conversation, cx| {
            conversation.save(None, self.fs.clone(), cx)
        });
    }

    fn cycle_model(&mut self, cx: &mut ViewContext<Self>) {
        self.conversation.update(cx, |conversation, cx| {
            let new_model = match conversation.model.as_str() {
                "gpt-4-0613" => "gpt-3.5-turbo-0613",
                _ => "gpt-4-0613",
            };
            conversation.set_model(new_model.into(), cx);
        });
    }

    fn title(&self, cx: &AppContext) -> String {
        self.conversation
            .read(cx)
            .summary
            .as_ref()
            .map(|summary| summary.text.clone())
            .unwrap_or_else(|| "New Conversation".into())
    }

    fn render_current_model(
        &self,
        style: &AssistantStyle,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        enum Model {}

        MouseEventHandler::new::<Model, _>(0, cx, |state, cx| {
            let style = style.model.style_for(state);
            Label::new(self.conversation.read(cx).model.clone(), style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, |_, this, cx| this.cycle_model(cx))
    }

    fn render_remaining_tokens(
        &self,
        style: &AssistantStyle,
        cx: &mut ViewContext<Self>,
    ) -> Option<impl Element<Self>> {
        let remaining_tokens = self.conversation.read(cx).remaining_tokens()?;
        let remaining_tokens_style = if remaining_tokens <= 0 {
            &style.no_remaining_tokens
        } else if remaining_tokens <= 500 {
            &style.low_remaining_tokens
        } else {
            &style.remaining_tokens
        };
        Some(
            Label::new(
                remaining_tokens.to_string(),
                remaining_tokens_style.text.clone(),
            )
            .contained()
            .with_style(remaining_tokens_style.container),
        )
    }
}

impl Entity for ConversationEditor {
    type Event = ConversationEditorEvent;
}

impl View for ConversationEditor {
    fn ui_name() -> &'static str {
        "ConversationEditor"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = &theme::current(cx).assistant;
        Stack::new()
            .with_child(
                ChildView::new(&self.editor, cx)
                    .contained()
                    .with_style(theme.container),
            )
            .with_child(
                Flex::row()
                    .with_child(self.render_current_model(theme, cx))
                    .with_children(self.render_remaining_tokens(theme, cx))
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

#[derive(Clone, Debug)]
struct MessageAnchor {
    id: MessageId,
    start: language::Anchor,
}

#[derive(Clone, Debug)]
pub struct Message {
    offset_range: Range<usize>,
    index_range: Range<usize>,
    id: MessageId,
    anchor: language::Anchor,
    role: Role,
    sent_at: DateTime<Local>,
    status: MessageStatus,
}

impl Message {
    fn to_open_ai_message(&self, buffer: &Buffer) -> RequestMessage {
        let mut content = format!("[Message {}]\n", self.id.0).to_string();
        content.extend(buffer.text_for_range(self.offset_range.clone()));
        RequestMessage {
            role: self.role,
            content: content.trim_end().into(),
        }
    }
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
    use crate::MessageId;
    use gpui::AppContext;

    #[gpui::test]
    fn test_inserting_and_removing_messages(cx: &mut AppContext) {
        let registry = Arc::new(LanguageRegistry::test());
        let conversation = cx.add_model(|cx| Conversation::new(Default::default(), registry, cx));
        let buffer = conversation.read(cx).buffer.clone();

        let message_1 = conversation.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&conversation, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        let message_2 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_1.id, Role::Assistant, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..1),
                (message_2.id, Role::Assistant, 1..1)
            ]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "1"), (1..1, "2")], None, cx)
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..3)
            ]
        );

        let message_3 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..2),
                (message_2.id, Role::Assistant, 2..4),
                (message_3.id, Role::User, 4..4)
            ]
        );

        let message_4 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
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
            messages(&conversation, cx),
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
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Undoing the deletion should also undo the merge.
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(
            messages(&conversation, cx),
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
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_3.id, Role::User, 3..4),
            ]
        );

        // Ensure we can still insert after a merged message.
        let message_5 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..3),
                (message_5.id, Role::System, 3..4),
                (message_3.id, Role::User, 4..5)
            ]
        );
    }

    #[gpui::test]
    fn test_message_splitting(cx: &mut AppContext) {
        let registry = Arc::new(LanguageRegistry::test());
        let conversation = cx.add_model(|cx| Conversation::new(Default::default(), registry, cx));
        let buffer = conversation.read(cx).buffer.clone();

        let message_1 = conversation.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&conversation, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "aaa\nbbb\nccc\nddd\n")], None, cx)
        });

        let (_, message_2) =
            conversation.update(cx, |conversation, cx| conversation.split_message(3..3, cx));
        let message_2 = message_2.unwrap();

        // We recycle newlines in the middle of a split message
        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..16),
            ]
        );

        let (_, message_3) =
            conversation.update(cx, |conversation, cx| conversation.split_message(3..3, cx));
        let message_3 = message_3.unwrap();

        // We don't recycle newlines at the end of a split message
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..17),
            ]
        );

        let (_, message_4) =
            conversation.update(cx, |conversation, cx| conversation.split_message(9..9, cx));
        let message_4 = message_4.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..17),
            ]
        );

        let (_, message_5) =
            conversation.update(cx, |conversation, cx| conversation.split_message(9..9, cx));
        let message_5 = message_5.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\nddd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..10),
                (message_5.id, Role::User, 10..18),
            ]
        );

        let (message_6, message_7) = conversation.update(cx, |conversation, cx| {
            conversation.split_message(14..16, cx)
        });
        let message_6 = message_6.unwrap();
        let message_7 = message_7.unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\n\nbbb\n\nccc\ndd\nd\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_3.id, Role::User, 4..5),
                (message_2.id, Role::User, 5..9),
                (message_4.id, Role::User, 9..10),
                (message_5.id, Role::User, 10..14),
                (message_6.id, Role::User, 14..17),
                (message_7.id, Role::User, 17..19),
            ]
        );
    }

    #[gpui::test]
    fn test_messages_for_offsets(cx: &mut AppContext) {
        let registry = Arc::new(LanguageRegistry::test());
        let conversation = cx.add_model(|cx| Conversation::new(Default::default(), registry, cx));
        let buffer = conversation.read(cx).buffer.clone();

        let message_1 = conversation.read(cx).message_anchors[0].clone();
        assert_eq!(
            messages(&conversation, cx),
            vec![(message_1.id, Role::User, 0..0)]
        );

        buffer.update(cx, |buffer, cx| buffer.edit([(0..0, "aaa")], None, cx));
        let message_2 = conversation
            .update(cx, |conversation, cx| {
                conversation.insert_message_after(message_1.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(4..4, "bbb")], None, cx));

        let message_3 = conversation
            .update(cx, |conversation, cx| {
                conversation.insert_message_after(message_2.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        buffer.update(cx, |buffer, cx| buffer.edit([(8..8, "ccc")], None, cx));

        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..8),
                (message_3.id, Role::User, 8..11)
            ]
        );

        assert_eq!(
            message_ids_for_offsets(&conversation, &[0, 4, 9], cx),
            [message_1.id, message_2.id, message_3.id]
        );
        assert_eq!(
            message_ids_for_offsets(&conversation, &[0, 1, 11], cx),
            [message_1.id, message_3.id]
        );

        let message_4 = conversation
            .update(cx, |conversation, cx| {
                conversation.insert_message_after(message_3.id, Role::User, MessageStatus::Done, cx)
            })
            .unwrap();
        assert_eq!(buffer.read(cx).text(), "aaa\nbbb\nccc\n");
        assert_eq!(
            messages(&conversation, cx),
            vec![
                (message_1.id, Role::User, 0..4),
                (message_2.id, Role::User, 4..8),
                (message_3.id, Role::User, 8..12),
                (message_4.id, Role::User, 12..12)
            ]
        );
        assert_eq!(
            message_ids_for_offsets(&conversation, &[0, 4, 8, 12], cx),
            [message_1.id, message_2.id, message_3.id, message_4.id]
        );

        fn message_ids_for_offsets(
            conversation: &ModelHandle<Conversation>,
            offsets: &[usize],
            cx: &AppContext,
        ) -> Vec<MessageId> {
            conversation
                .read(cx)
                .messages_for_offsets(offsets.iter().copied(), cx)
                .into_iter()
                .map(|message| message.id)
                .collect()
        }
    }

    #[gpui::test]
    fn test_serialization(cx: &mut AppContext) {
        let registry = Arc::new(LanguageRegistry::test());
        let conversation =
            cx.add_model(|cx| Conversation::new(Default::default(), registry.clone(), cx));
        let buffer = conversation.read(cx).buffer.clone();
        let message_0 = conversation.read(cx).message_anchors[0].id;
        let message_1 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_0, Role::Assistant, MessageStatus::Done, cx)
                .unwrap()
        });
        let message_2 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_1.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(0..0, "a"), (1..1, "b\nc")], None, cx);
            buffer.finalize_last_transaction();
        });
        let _message_3 = conversation.update(cx, |conversation, cx| {
            conversation
                .insert_message_after(message_2.id, Role::System, MessageStatus::Done, cx)
                .unwrap()
        });
        buffer.update(cx, |buffer, cx| buffer.undo(cx));
        assert_eq!(buffer.read(cx).text(), "a\nb\nc\n");
        assert_eq!(
            messages(&conversation, cx),
            [
                (message_0, Role::User, 0..2),
                (message_1.id, Role::Assistant, 2..6),
                (message_2.id, Role::System, 6..6),
            ]
        );

        let deserialized_conversation = cx.add_model(|cx| {
            Conversation::deserialize(
                conversation.read(cx).serialize(cx),
                Default::default(),
                Default::default(),
                registry.clone(),
                cx,
            )
        });
        let deserialized_buffer = deserialized_conversation.read(cx).buffer.clone();
        assert_eq!(deserialized_buffer.read(cx).text(), "a\nb\nc\n");
        assert_eq!(
            messages(&deserialized_conversation, cx),
            [
                (message_0, Role::User, 0..2),
                (message_1.id, Role::Assistant, 2..6),
                (message_2.id, Role::System, 6..6),
            ]
        );
    }

    fn messages(
        conversation: &ModelHandle<Conversation>,
        cx: &AppContext,
    ) -> Vec<(MessageId, Role, Range<usize>)> {
        conversation
            .read(cx)
            .messages(cx)
            .map(|message| (message.id, message.role, message.offset_range))
            .collect()
    }
}
