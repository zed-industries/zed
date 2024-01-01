use crate::{channel_view::ChannelView, is_channels_feature_enabled, ChatPanelSettings};
use anyhow::Result;
use call::ActiveCall;
use channel::{ChannelChat, ChannelChatEvent, ChannelMessageId, ChannelStore};
use client::Client;
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    actions, div, list, prelude::*, px, serde_json, AnyElement, AppContext, AsyncWindowContext,
    ClickEvent, ElementId, EventEmitter, FocusableView, ListOffset, ListScrollEvent, ListState,
    Model, Render, Subscription, Task, View, ViewContext, VisualContext, WeakView,
};
use language::LanguageRegistry;
use menu::Confirm;
use message_editor::MessageEditor;
use project::Fs;
use rich_text::RichText;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsStore};
use std::sync::Arc;
use theme::ActiveTheme as _;
use time::{OffsetDateTime, UtcOffset};
use ui::{prelude::*, Avatar, Button, Icon, IconButton, Label, TabBar, Tooltip};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

mod message_editor;

const MESSAGE_LOADING_THRESHOLD: usize = 50;
const CHAT_PANEL_KEY: &'static str = "ChatPanel";

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<ChatPanel>(cx);
        });
    })
    .detach();
}

pub struct ChatPanel {
    client: Arc<Client>,
    channel_store: Model<ChannelStore>,
    languages: Arc<LanguageRegistry>,
    message_list: ListState,
    active_chat: Option<(Model<ChannelChat>, Subscription)>,
    input_editor: View<MessageEditor>,
    local_timezone: UtcOffset,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    active: bool,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
    workspace: WeakView<Workspace>,
    is_scrolled_to_bottom: bool,
    markdown_data: HashMap<ChannelMessageId, RichText>,
}

#[derive(Serialize, Deserialize)]
struct SerializedChatPanel {
    width: Option<Pixels>,
}

#[derive(Debug)]
pub enum Event {
    DockPositionChanged,
    Focus,
    Dismissed,
}

actions!(chat_panel, [ToggleFocus]);

impl ChatPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let channel_store = ChannelStore::global(cx);
        let languages = workspace.app_state().languages.clone();

        let input_editor = cx.build_view(|cx| {
            MessageEditor::new(
                languages.clone(),
                channel_store.clone(),
                cx.build_view(|cx| Editor::auto_height(4, cx)),
                cx,
            )
        });

        let workspace_handle = workspace.weak_handle();

        cx.build_view(|cx: &mut ViewContext<Self>| {
            let view = cx.view().downgrade();
            let message_list =
                ListState::new(0, gpui::ListAlignment::Bottom, px(1000.), move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view, cx| {
                            view.render_message(ix, cx).into_any_element()
                        })
                    } else {
                        div().into_any()
                    }
                });

            message_list.set_scroll_handler(cx.listener(|this, event: &ListScrollEvent, cx| {
                if event.visible_range.start < MESSAGE_LOADING_THRESHOLD {
                    this.load_more_messages(cx);
                }
                this.is_scrolled_to_bottom = event.visible_range.end == event.count;
            }));

            let mut this = Self {
                fs,
                client,
                channel_store,
                languages,
                message_list,
                active_chat: Default::default(),
                pending_serialization: Task::ready(None),
                input_editor,
                local_timezone: cx.local_timezone(),
                subscriptions: Vec::new(),
                workspace: workspace_handle,
                is_scrolled_to_bottom: true,
                active: false,
                width: None,
                markdown_data: Default::default(),
            };

            let mut old_dock_position = this.position(cx);
            this.subscriptions.push(cx.observe_global::<SettingsStore>(
                move |this: &mut Self, cx| {
                    let new_dock_position = this.position(cx);
                    if new_dock_position != old_dock_position {
                        old_dock_position = new_dock_position;
                        cx.emit(Event::DockPositionChanged);
                    }
                    cx.notify();
                },
            ));

            this
        })
    }

    pub fn is_scrolled_to_bottom(&self) -> bool {
        self.is_scrolled_to_bottom
    }

    pub fn active_chat(&self) -> Option<Model<ChannelChat>> {
        self.active_chat.as_ref().map(|(chat, _)| chat.clone())
    }

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background_executor()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(CHAT_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedChatPanel>(&panel)?)
            } else {
                None
            };

            workspace.update(&mut cx, |workspace, cx| {
                let panel = Self::new(workspace, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width;
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_executor().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        CHAT_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedChatPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn set_active_chat(&mut self, chat: Model<ChannelChat>, cx: &mut ViewContext<Self>) {
        if self.active_chat.as_ref().map(|e| &e.0) != Some(&chat) {
            let channel_id = chat.read(cx).channel_id;
            {
                self.markdown_data.clear();
                let chat = chat.read(cx);
                self.message_list.reset(chat.message_count());

                let channel_name = chat.channel(cx).map(|channel| channel.name.clone());
                self.input_editor.update(cx, |editor, cx| {
                    editor.set_channel(channel_id, channel_name, cx);
                });
            };
            let subscription = cx.subscribe(&chat, Self::channel_did_change);
            self.active_chat = Some((chat, subscription));
            self.acknowledge_last_message(cx);
            cx.notify();
        }
    }

    fn channel_did_change(
        &mut self,
        _: Model<ChannelChat>,
        event: &ChannelChatEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelChatEvent::MessagesUpdated {
                old_range,
                new_count,
            } => {
                self.message_list.splice(old_range.clone(), *new_count);
                if self.active {
                    self.acknowledge_last_message(cx);
                }
            }
            ChannelChatEvent::NewMessage {
                channel_id,
                message_id,
            } => {
                if !self.active {
                    self.channel_store.update(cx, |store, cx| {
                        store.new_message(*channel_id, *message_id, cx)
                    })
                }
            }
        }
        cx.notify();
    }

    fn acknowledge_last_message(&mut self, cx: &mut ViewContext<Self>) {
        if self.active && self.is_scrolled_to_bottom {
            if let Some((chat, _)) = &self.active_chat {
                chat.update(cx, |chat, cx| {
                    chat.acknowledge_last_message(cx);
                });
            }
        }
    }

    fn render_channel(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        v_stack()
            .full()
            .on_action(cx.listener(Self::send))
            .child(
                h_stack().z_index(1).child(
                    TabBar::new("chat_header")
                        .child(
                            h_stack()
                                .w_full()
                                .h(rems(ui::Tab::HEIGHT_IN_REMS))
                                .px_2()
                                .child(Label::new(
                                    self.active_chat
                                        .as_ref()
                                        .and_then(|c| {
                                            Some(format!("#{}", c.0.read(cx).channel(cx)?.name))
                                        })
                                        .unwrap_or_default(),
                                )),
                        )
                        .end_child(
                            IconButton::new("notes", Icon::File)
                                .on_click(cx.listener(Self::open_notes))
                                .tooltip(|cx| Tooltip::text("Open notes", cx)),
                        )
                        .end_child(
                            IconButton::new("call", Icon::AudioOn)
                                .on_click(cx.listener(Self::join_call))
                                .tooltip(|cx| Tooltip::text("Join call", cx)),
                        ),
                ),
            )
            .child(div().flex_grow().px_2().py_1().map(|this| {
                if self.active_chat.is_some() {
                    this.child(list(self.message_list.clone()).full())
                } else {
                    this
                }
            }))
            .child(
                div()
                    .z_index(1)
                    .p_2()
                    .bg(cx.theme().colors().background)
                    .child(self.input_editor.clone()),
            )
            .into_any()
    }

    fn render_message(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_chat = &self.active_chat.as_ref().unwrap().0;
        let (message, is_continuation_from_previous, is_continuation_to_next, is_admin) =
            active_chat.update(cx, |active_chat, cx| {
                let is_admin = self
                    .channel_store
                    .read(cx)
                    .is_channel_admin(active_chat.channel_id);

                let last_message = active_chat.message(ix.saturating_sub(1));
                let this_message = active_chat.message(ix).clone();
                let next_message =
                    active_chat.message(ix.saturating_add(1).min(active_chat.message_count() - 1));

                let is_continuation_from_previous = last_message.id != this_message.id
                    && last_message.sender.id == this_message.sender.id;
                let is_continuation_to_next = this_message.id != next_message.id
                    && this_message.sender.id == next_message.sender.id;

                if let ChannelMessageId::Saved(id) = this_message.id {
                    if this_message
                        .mentions
                        .iter()
                        .any(|(_, user_id)| Some(*user_id) == self.client.user_id())
                    {
                        active_chat.acknowledge_message(id);
                    }
                }

                (
                    this_message,
                    is_continuation_from_previous,
                    is_continuation_to_next,
                    is_admin,
                )
            });

        let _is_pending = message.is_pending();
        let text = self.markdown_data.entry(message.id).or_insert_with(|| {
            Self::render_markdown_with_mentions(&self.languages, self.client.id(), &message)
        });

        let now = OffsetDateTime::now_utc();

        let belongs_to_user = Some(message.sender.id) == self.client.user_id();
        let message_id_to_remove = if let (ChannelMessageId::Saved(id), true) =
            (message.id, belongs_to_user || is_admin)
        {
            Some(id)
        } else {
            None
        };

        let element_id: ElementId = match message.id {
            ChannelMessageId::Saved(id) => ("saved-message", id).into(),
            ChannelMessageId::Pending(id) => ("pending-message", id).into(),
        };

        v_stack()
            .w_full()
            .id(element_id)
            .relative()
            .overflow_hidden()
            .group("")
            .when(!is_continuation_from_previous, |this| {
                this.child(
                    h_stack()
                        .gap_2()
                        .child(Avatar::new(message.sender.avatar_uri.clone()))
                        .child(Label::new(message.sender.github_login.clone()))
                        .child(
                            Label::new(format_timestamp(
                                message.timestamp,
                                now,
                                self.local_timezone,
                            ))
                            .color(Color::Muted),
                        ),
                )
            })
            .when(!is_continuation_to_next, |this|
                // HACK: This should really be a margin, but margins seem to get collapsed.
                this.pb_2())
            .child(text.element("body".into(), cx))
            .child(
                div()
                    .absolute()
                    .top_1()
                    .right_2()
                    .w_8()
                    .visible_on_hover("")
                    .children(message_id_to_remove.map(|message_id| {
                        IconButton::new(("remove", message_id), Icon::XCircle).on_click(
                            cx.listener(move |this, _, cx| {
                                this.remove_message(message_id, cx);
                            }),
                        )
                    })),
            )
    }

    fn render_markdown_with_mentions(
        language_registry: &Arc<LanguageRegistry>,
        current_user_id: u64,
        message: &channel::ChannelMessage,
    ) -> RichText {
        let mentions = message
            .mentions
            .iter()
            .map(|(range, user_id)| rich_text::Mention {
                range: range.clone(),
                is_self_mention: *user_id == current_user_id,
            })
            .collect::<Vec<_>>();

        rich_text::render_markdown(message.body.clone(), &mentions, language_registry, None)
    }

    fn render_sign_in_prompt(&self, cx: &mut ViewContext<Self>) -> AnyElement {
        Button::new("sign-in", "Sign in to use chat")
            .on_click(cx.listener(move |this, _, cx| {
                let client = this.client.clone();
                cx.spawn(|this, mut cx| async move {
                    if client
                        .authenticate_and_connect(true, &cx)
                        .log_err()
                        .await
                        .is_some()
                    {
                        this.update(&mut cx, |_, cx| {
                            cx.focus_self();
                        })
                        .ok();
                    }
                })
                .detach();
            }))
            .into_any_element()
    }

    fn send(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            let message = self
                .input_editor
                .update(cx, |editor, cx| editor.take_message(cx));

            if let Some(task) = chat
                .update(cx, |chat, cx| chat.send_message(message, cx))
                .log_err()
            {
                task.detach();
            }
        }
    }

    fn remove_message(&mut self, id: u64, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            chat.update(cx, |chat, cx| chat.remove_message(id, cx).detach())
        }
    }

    fn load_more_messages(&mut self, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            chat.update(cx, |channel, cx| {
                if let Some(task) = channel.load_more_messages(cx) {
                    task.detach();
                }
            })
        }
    }

    pub fn select_channel(
        &mut self,
        selected_channel_id: u64,
        scroll_to_message_id: Option<u64>,
        cx: &mut ViewContext<ChatPanel>,
    ) -> Task<Result<()>> {
        let open_chat = self
            .active_chat
            .as_ref()
            .and_then(|(chat, _)| {
                (chat.read(cx).channel_id == selected_channel_id)
                    .then(|| Task::ready(anyhow::Ok(chat.clone())))
            })
            .unwrap_or_else(|| {
                self.channel_store.update(cx, |store, cx| {
                    store.open_channel_chat(selected_channel_id, cx)
                })
            });

        cx.spawn(|this, mut cx| async move {
            let chat = open_chat.await?;
            this.update(&mut cx, |this, cx| {
                this.set_active_chat(chat.clone(), cx);
            })?;

            if let Some(message_id) = scroll_to_message_id {
                if let Some(item_ix) =
                    ChannelChat::load_history_since_message(chat.clone(), message_id, (*cx).clone())
                        .await
                {
                    this.update(&mut cx, |this, cx| {
                        if this.active_chat.as_ref().map_or(false, |(c, _)| *c == chat) {
                            this.message_list.scroll_to(ListOffset {
                                item_ix,
                                offset_in_item: px(0.0),
                            });
                            cx.notify();
                        }
                    })?;
                }
            }

            Ok(())
        })
    }

    fn open_notes(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = &self.active_chat {
            let channel_id = chat.read(cx).channel_id;
            if let Some(workspace) = self.workspace.upgrade() {
                ChannelView::open(channel_id, workspace, cx).detach();
            }
        }
    }

    fn join_call(&mut self, _: &ClickEvent, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = &self.active_chat {
            let channel_id = chat.read(cx).channel_id;
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.join_channel(channel_id, cx))
                .detach_and_log_err(cx);
        }
    }
}

impl EventEmitter<Event> for ChatPanel {}

impl Render for ChatPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl Element {
        div()
            .full()
            .child(if self.client.user_id().is_some() {
                self.render_channel(cx)
            } else {
                self.render_sign_in_prompt(cx)
            })
            .min_w(px(150.))
    }
}

impl FocusableView for ChatPanel {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.input_editor.read(cx).focus_handle(cx)
    }
}

impl Panel for ChatPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        ChatPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<ChatPanelSettings>(self.fs.clone(), cx, move |settings| {
            settings.dock = Some(position)
        });
    }

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| ChatPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        self.active = active;
        if active {
            self.acknowledge_last_message(cx);
            if !is_channels_feature_enabled(cx) {
                cx.emit(Event::Dismissed);
            }
        }
    }

    fn persistent_name() -> &'static str {
        "ChatPanel"
    }

    fn icon(&self, _cx: &WindowContext) -> Option<ui::Icon> {
        Some(ui::Icon::MessageBubbles)
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for ChatPanel {}

fn format_timestamp(
    mut timestamp: OffsetDateTime,
    mut now: OffsetDateTime,
    local_timezone: UtcOffset,
) -> String {
    timestamp = timestamp.to_offset(local_timezone);
    now = now.to_offset(local_timezone);

    let today = now.date();
    let date = timestamp.date();
    let mut hour = timestamp.hour();
    let mut part = "am";
    if hour > 12 {
        hour -= 12;
        part = "pm";
    }
    if date == today {
        format!("{:02}:{:02}{}", hour, timestamp.minute(), part)
    } else if date.next_day() == Some(today) {
        format!("yesterday at {:02}:{:02}{}", hour, timestamp.minute(), part)
    } else {
        format!("{:02}/{}/{}", date.month() as u32, date.day(), date.year())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::HighlightStyle;
    use pretty_assertions::assert_eq;
    use rich_text::Highlight;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_render_markdown_with_mentions() {
        let language_registry = Arc::new(LanguageRegistry::test());
        let (body, ranges) = marked_text_ranges("*hi*, «@abc», let's **call** «@fgh»", false);
        let message = channel::ChannelMessage {
            id: ChannelMessageId::Saved(0),
            body,
            timestamp: OffsetDateTime::now_utc(),
            sender: Arc::new(client::User {
                github_login: "fgh".into(),
                avatar_uri: "avatar_fgh".into(),
                id: 103,
            }),
            nonce: 5,
            mentions: vec![(ranges[0].clone(), 101), (ranges[1].clone(), 102)],
        };

        let message = ChatPanel::render_markdown_with_mentions(&language_registry, 102, &message);

        // Note that the "'" was replaced with ’ due to smart punctuation.
        let (body, ranges) = marked_text_ranges("«hi», «@abc», let’s «call» «@fgh»", false);
        assert_eq!(message.text, body);
        assert_eq!(
            message.highlights,
            vec![
                (
                    ranges[0].clone(),
                    HighlightStyle {
                        font_style: Some(gpui::FontStyle::Italic),
                        ..Default::default()
                    }
                    .into()
                ),
                (ranges[1].clone(), Highlight::Mention),
                (
                    ranges[2].clone(),
                    HighlightStyle {
                        font_weight: Some(gpui::FontWeight::BOLD),
                        ..Default::default()
                    }
                    .into()
                ),
                (ranges[3].clone(), Highlight::SelfMention)
            ]
        );
    }
}
