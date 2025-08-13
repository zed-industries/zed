use crate::{ChatPanelButton, ChatPanelSettings, collab_panel};
use anyhow::Result;
use call::{ActiveCall, room};
use channel::{ChannelChat, ChannelChatEvent, ChannelMessage, ChannelMessageId, ChannelStore};
use client::{ChannelId, Client};
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, actions};
use gpui::{
    Action, App, AsyncWindowContext, ClipboardItem, Context, CursorStyle, DismissEvent, ElementId,
    Entity, EventEmitter, FocusHandle, Focusable, FontWeight, HighlightStyle, ListOffset,
    ListScrollEvent, ListState, Render, Stateful, Subscription, Task, WeakEntity, Window, actions,
    div, list, prelude::*, px,
};
use language::LanguageRegistry;
use menu::Confirm;
use message_editor::MessageEditor;
use project::Fs;
use rich_text::{Highlight, RichText};
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{sync::Arc, time::Duration};
use time::{OffsetDateTime, UtcOffset};
use ui::{
    Avatar, Button, ContextMenu, IconButton, IconName, KeyBinding, Label, PopoverMenu, Tab, TabBar,
    Tooltip, prelude::*,
};
use util::{ResultExt, TryFutureExt};
use workspace::{
    Workspace,
    dock::{DockPosition, Panel, PanelEvent},
};

mod message_editor;

const MESSAGE_LOADING_THRESHOLD: usize = 50;
const CHAT_PANEL_KEY: &str = "ChatPanel";

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, window, cx| {
            workspace.toggle_panel_focus::<ChatPanel>(window, cx);
        });
    })
    .detach();
}

pub struct ChatPanel {
    client: Arc<Client>,
    channel_store: Entity<ChannelStore>,
    languages: Arc<LanguageRegistry>,
    message_list: ListState,
    active_chat: Option<(Entity<ChannelChat>, Subscription)>,
    message_editor: Entity<MessageEditor>,
    local_timezone: UtcOffset,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    active: bool,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
    is_scrolled_to_bottom: bool,
    markdown_data: HashMap<ChannelMessageId, RichText>,
    focus_handle: FocusHandle,
    open_context_menu: Option<(u64, Subscription)>,
    highlighted_message: Option<(u64, Task<()>)>,
    last_acknowledged_message_id: Option<u64>,
}

#[derive(Serialize, Deserialize)]
struct SerializedChatPanel {
    width: Option<Pixels>,
}

actions!(
    chat_panel,
    [
        /// Toggles focus on the chat panel.
        ToggleFocus
    ]
);

impl ChatPanel {
    pub fn new(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let channel_store = ChannelStore::global(cx);
        let user_store = workspace.app_state().user_store.clone();
        let languages = workspace.app_state().languages.clone();

        let input_editor = cx.new(|cx| {
            MessageEditor::new(
                languages.clone(),
                user_store.clone(),
                None,
                cx.new(|cx| Editor::auto_height(1, 4, window, cx)),
                window,
                cx,
            )
        });

        cx.new(|cx| {
            let message_list = ListState::new(0, gpui::ListAlignment::Bottom, px(1000.));

            message_list.set_scroll_handler(cx.listener(
                |this: &mut Self, event: &ListScrollEvent, _, cx| {
                    if event.visible_range.start < MESSAGE_LOADING_THRESHOLD {
                        this.load_more_messages(cx);
                    }
                    this.is_scrolled_to_bottom = !event.is_scrolled;
                },
            ));

            let local_offset = chrono::Local::now().offset().local_minus_utc();
            let mut this = Self {
                fs,
                client,
                channel_store,
                languages,
                message_list,
                active_chat: Default::default(),
                pending_serialization: Task::ready(None),
                message_editor: input_editor,
                local_timezone: UtcOffset::from_whole_seconds(local_offset).unwrap(),
                subscriptions: Vec::new(),
                is_scrolled_to_bottom: true,
                active: false,
                width: None,
                markdown_data: Default::default(),
                focus_handle: cx.focus_handle(),
                open_context_menu: None,
                highlighted_message: None,
                last_acknowledged_message_id: None,
            };

            if let Some(channel_id) = ActiveCall::global(cx)
                .read(cx)
                .room()
                .and_then(|room| room.read(cx).channel_id())
            {
                this.select_channel(channel_id, None, cx)
                    .detach_and_log_err(cx);
            }

            this.subscriptions.push(cx.subscribe(
                &ActiveCall::global(cx),
                move |this: &mut Self, call, event: &room::Event, cx| match event {
                    room::Event::RoomJoined { channel_id } => {
                        if let Some(channel_id) = channel_id {
                            this.select_channel(*channel_id, None, cx)
                                .detach_and_log_err(cx);

                            if call
                                .read(cx)
                                .room()
                                .is_some_and(|room| room.read(cx).contains_guests())
                            {
                                cx.emit(PanelEvent::Activate)
                            }
                        }
                    }
                    room::Event::RoomLeft { channel_id } => {
                        if channel_id == &this.channel_id(cx) {
                            cx.emit(PanelEvent::Close)
                        }
                    }
                    _ => {}
                },
            ));

            this
        })
    }

    pub fn channel_id(&self, cx: &App) -> Option<ChannelId> {
        self.active_chat
            .as_ref()
            .map(|(chat, _)| chat.read(cx).channel_id)
    }

    pub fn is_scrolled_to_bottom(&self) -> bool {
        self.is_scrolled_to_bottom
    }

    pub fn active_chat(&self) -> Option<Entity<ChannelChat>> {
        self.active_chat.as_ref().map(|(chat, _)| chat.clone())
    }

    pub fn load(
        workspace: WeakEntity<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<Entity<Self>>> {
        cx.spawn(async move |cx| {
            let serialized_panel = if let Some(panel) = cx
                .background_spawn(async move { KEY_VALUE_STORE.read_kvp(CHAT_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedChatPanel>(&panel)?)
            } else {
                None
            };

            workspace.update_in(cx, |workspace, window, cx| {
                let panel = Self::new(workspace, window, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width.map(|r| r.round());
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut Context<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background_spawn(
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

    fn set_active_chat(&mut self, chat: Entity<ChannelChat>, cx: &mut Context<Self>) {
        if self.active_chat.as_ref().map(|e| &e.0) != Some(&chat) {
            self.markdown_data.clear();
            self.message_list.reset(chat.read(cx).message_count());
            self.message_editor.update(cx, |editor, cx| {
                editor.set_channel_chat(chat.clone(), cx);
                editor.clear_reply_to_message_id();
            });
            let subscription = cx.subscribe(&chat, Self::channel_did_change);
            self.active_chat = Some((chat, subscription));
            self.acknowledge_last_message(cx);
            cx.notify();
        }
    }

    fn channel_did_change(
        &mut self,
        _: Entity<ChannelChat>,
        event: &ChannelChatEvent,
        cx: &mut Context<Self>,
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
            ChannelChatEvent::UpdateMessage {
                message_id,
                message_ix,
            } => {
                self.message_list.splice(*message_ix..*message_ix + 1, 1);
                self.markdown_data.remove(message_id);
            }
            ChannelChatEvent::NewMessage {
                channel_id,
                message_id,
            } => {
                if !self.active {
                    self.channel_store.update(cx, |store, cx| {
                        store.update_latest_message_id(*channel_id, *message_id, cx)
                    })
                }
            }
        }
        cx.notify();
    }

    fn acknowledge_last_message(&mut self, cx: &mut Context<Self>) {
        if self.active && self.is_scrolled_to_bottom {
            if let Some((chat, _)) = &self.active_chat {
                if let Some(channel_id) = self.channel_id(cx) {
                    self.last_acknowledged_message_id = self
                        .channel_store
                        .read(cx)
                        .last_acknowledge_message_id(channel_id);
                }

                chat.update(cx, |chat, cx| {
                    chat.acknowledge_last_message(cx);
                });
            }
        }
    }

    fn render_replied_to_message(
        &mut self,
        message_id: Option<ChannelMessageId>,
        reply_to_message: &Option<ChannelMessage>,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let reply_to_message = match reply_to_message {
            None => {
                return div().child(
                    h_flex()
                        .text_ui_xs(cx)
                        .my_0p5()
                        .px_0p5()
                        .gap_x_1()
                        .rounded_sm()
                        .child(Icon::new(IconName::ReplyArrowRight).color(Color::Muted))
                        .when(reply_to_message.is_none(), |el| {
                            el.child(
                                Label::new("Message has been deleted...")
                                    .size(LabelSize::XSmall)
                                    .color(Color::Muted),
                            )
                        }),
                );
            }
            Some(val) => val,
        };

        let user_being_replied_to = reply_to_message.sender.clone();
        let message_being_replied_to = reply_to_message.clone();

        let message_element_id: ElementId = match message_id {
            Some(ChannelMessageId::Saved(id)) => ("reply-to-saved-message-container", id).into(),
            Some(ChannelMessageId::Pending(id)) => {
                ("reply-to-pending-message-container", id).into()
            } // This should never happen
            None => ("composing-reply-container").into(),
        };

        let current_channel_id = self.channel_id(cx);
        let reply_to_message_id = reply_to_message.id;

        div().child(
            h_flex()
                .id(message_element_id)
                .text_ui_xs(cx)
                .my_0p5()
                .px_0p5()
                .gap_x_1()
                .rounded_sm()
                .overflow_hidden()
                .hover(|style| style.bg(cx.theme().colors().element_background))
                .child(Icon::new(IconName::ReplyArrowRight).color(Color::Muted))
                .child(Avatar::new(user_being_replied_to.avatar_uri.clone()).size(rems(0.7)))
                .child(
                    Label::new(format!("@{}", user_being_replied_to.github_login))
                        .size(LabelSize::XSmall)
                        .weight(FontWeight::SEMIBOLD)
                        .color(Color::Muted),
                )
                .child(
                    div().overflow_y_hidden().child(
                        Label::new(message_being_replied_to.body.replace('\n', " "))
                            .size(LabelSize::XSmall)
                            .color(Color::Default),
                    ),
                )
                .cursor(CursorStyle::PointingHand)
                .tooltip(Tooltip::text("Go to message"))
                .on_click(cx.listener(move |chat_panel, _, _, cx| {
                    if let Some(channel_id) = current_channel_id {
                        chat_panel
                            .select_channel(channel_id, reply_to_message_id.into(), cx)
                            .detach_and_log_err(cx)
                    }
                })),
        )
    }

    fn render_message(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let active_chat = &self.active_chat.as_ref().unwrap().0;
        let (message, is_continuation_from_previous, is_admin) =
            active_chat.update(cx, |active_chat, cx| {
                let is_admin = self
                    .channel_store
                    .read(cx)
                    .is_channel_admin(active_chat.channel_id);

                let last_message = active_chat.message(ix.saturating_sub(1));
                let this_message = active_chat.message(ix).clone();

                let duration_since_last_message = this_message.timestamp - last_message.timestamp;
                let is_continuation_from_previous = last_message.sender.id
                    == this_message.sender.id
                    && last_message.id != this_message.id
                    && duration_since_last_message < Duration::from_secs(5 * 60);

                if let ChannelMessageId::Saved(id) = this_message.id {
                    if this_message
                        .mentions
                        .iter()
                        .any(|(_, user_id)| Some(*user_id) == self.client.user_id())
                    {
                        active_chat.acknowledge_message(id);
                    }
                }

                (this_message, is_continuation_from_previous, is_admin)
            });

        let _is_pending = message.is_pending();

        let belongs_to_user = Some(message.sender.id) == self.client.user_id();
        let can_delete_message = belongs_to_user || is_admin;
        let can_edit_message = belongs_to_user;

        let element_id: ElementId = match message.id {
            ChannelMessageId::Saved(id) => ("saved-message", id).into(),
            ChannelMessageId::Pending(id) => ("pending-message", id).into(),
        };

        let mentioning_you = message
            .mentions
            .iter()
            .any(|m| Some(m.1) == self.client.user_id());

        let message_id = match message.id {
            ChannelMessageId::Saved(id) => Some(id),
            ChannelMessageId::Pending(_) => None,
        };

        let reply_to_message = message
            .reply_to_message_id
            .and_then(|id| active_chat.read(cx).find_loaded_message(id))
            .cloned();

        let replied_to_you =
            reply_to_message.as_ref().map(|m| m.sender.id) == self.client.user_id();

        let is_highlighted_message = self
            .highlighted_message
            .as_ref()
            .is_some_and(|(id, _)| Some(id) == message_id.as_ref());
        let background = if is_highlighted_message {
            cx.theme().status().info_background
        } else if mentioning_you || replied_to_you {
            cx.theme().colors().background
        } else {
            cx.theme().colors().panel_background
        };

        let reply_to_message_id = self.message_editor.read(cx).reply_to_message_id();

        v_flex()
            .w_full()
            .relative()
            .group("")
            .when(!is_continuation_from_previous, |this| this.pt_2())
            .child(
                div()
                    .group("")
                    .bg(background)
                    .rounded_sm()
                    .overflow_hidden()
                    .px_1p5()
                    .py_0p5()
                    .when_some(reply_to_message_id, |el, reply_id| {
                        el.when_some(message_id, |el, message_id| {
                            el.when(reply_id == message_id, |el| {
                                el.bg(cx.theme().colors().element_selected)
                            })
                        })
                    })
                    .when(!self.has_open_menu(message_id), |this| {
                        this.hover(|style| style.bg(cx.theme().colors().element_hover))
                    })
                    .when(message.reply_to_message_id.is_some(), |el| {
                        el.child(self.render_replied_to_message(
                            Some(message.id),
                            &reply_to_message,
                            cx,
                        ))
                        .when(is_continuation_from_previous, |this| this.mt_2())
                    })
                    .when(
                        !is_continuation_from_previous || message.reply_to_message_id.is_some(),
                        |this| {
                            this.child(
                                h_flex()
                                    .gap_2()
                                    .text_ui_sm(cx)
                                    .child(
                                        Avatar::new(message.sender.avatar_uri.clone())
                                            .size(rems(1.)),
                                    )
                                    .child(
                                        Label::new(message.sender.github_login.clone())
                                            .size(LabelSize::Small)
                                            .weight(FontWeight::BOLD),
                                    )
                                    .child(
                                        Label::new(time_format::format_localized_timestamp(
                                            message.timestamp,
                                            OffsetDateTime::now_utc(),
                                            self.local_timezone,
                                            time_format::TimestampFormat::EnhancedAbsolute,
                                        ))
                                        .size(LabelSize::Small)
                                        .color(Color::Muted),
                                    ),
                            )
                        },
                    )
                    .when(mentioning_you || replied_to_you, |this| this.my_0p5())
                    .map(|el| {
                        let text = self.markdown_data.entry(message.id).or_insert_with(|| {
                            Self::render_markdown_with_mentions(
                                &self.languages,
                                self.client.id(),
                                &message,
                                self.local_timezone,
                                cx,
                            )
                        });
                        el.child(
                            v_flex()
                                .w_full()
                                .text_ui_sm(cx)
                                .id(element_id)
                                .child(text.element("body".into(), window, cx)),
                        )
                        .when(self.has_open_menu(message_id), |el| {
                            el.bg(cx.theme().colors().element_selected)
                        })
                    }),
            )
            .when(
                self.last_acknowledged_message_id
                    .is_some_and(|l| Some(l) == message_id),
                |this| {
                    this.child(
                        h_flex()
                            .py_2()
                            .gap_1()
                            .items_center()
                            .child(div().w_full().h_0p5().bg(cx.theme().colors().border))
                            .child(
                                div()
                                    .px_1()
                                    .rounded_sm()
                                    .text_ui_xs(cx)
                                    .bg(cx.theme().colors().background)
                                    .child("New messages"),
                            )
                            .child(div().w_full().h_0p5().bg(cx.theme().colors().border)),
                    )
                },
            )
            .child(
                self.render_popover_buttons(message_id, can_delete_message, can_edit_message, cx)
                    .mt_neg_2p5(),
            )
            .into_any_element()
    }

    fn has_open_menu(&self, message_id: Option<u64>) -> bool {
        match self.open_context_menu.as_ref() {
            Some((id, _)) => Some(*id) == message_id,
            None => false,
        }
    }

    fn render_popover_button(&self, cx: &mut Context<Self>, child: Stateful<Div>) -> Div {
        div()
            .w_6()
            .bg(cx.theme().colors().element_background)
            .hover(|style| style.bg(cx.theme().colors().element_hover).rounded_sm())
            .child(child)
    }

    fn render_popover_buttons(
        &self,
        message_id: Option<u64>,
        can_delete_message: bool,
        can_edit_message: bool,
        cx: &mut Context<Self>,
    ) -> Div {
        h_flex()
            .absolute()
            .right_2()
            .overflow_hidden()
            .rounded_sm()
            .border_color(cx.theme().colors().element_selected)
            .border_1()
            .when(!self.has_open_menu(message_id), |el| {
                el.visible_on_hover("")
            })
            .bg(cx.theme().colors().element_background)
            .when_some(message_id, |el, message_id| {
                el.child(
                    self.render_popover_button(
                        cx,
                        div()
                            .id("reply")
                            .child(
                                IconButton::new(("reply", message_id), IconName::ReplyArrowRight)
                                    .on_click(cx.listener(move |this, _, window, cx| {
                                        this.cancel_edit_message(cx);

                                        this.message_editor.update(cx, |editor, cx| {
                                            editor.set_reply_to_message_id(message_id);
                                            window.focus(&editor.focus_handle(cx));
                                        })
                                    })),
                            )
                            .tooltip(Tooltip::text("Reply")),
                    ),
                )
            })
            .when_some(message_id, |el, message_id| {
                el.when(can_edit_message, |el| {
                    el.child(
                        self.render_popover_button(
                            cx,
                            div()
                                .id("edit")
                                .child(
                                    IconButton::new(("edit", message_id), IconName::Pencil)
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            this.message_editor.update(cx, |editor, cx| {
                                                editor.clear_reply_to_message_id();

                                                let message = this
                                                    .active_chat()
                                                    .and_then(|active_chat| {
                                                        active_chat
                                                            .read(cx)
                                                            .find_loaded_message(message_id)
                                                    })
                                                    .cloned();

                                                if let Some(message) = message {
                                                    let buffer = editor
                                                        .editor
                                                        .read(cx)
                                                        .buffer()
                                                        .read(cx)
                                                        .as_singleton()
                                                        .expect("message editor must be singleton");

                                                    buffer.update(cx, |buffer, cx| {
                                                        buffer.set_text(message.body.clone(), cx)
                                                    });

                                                    editor.set_edit_message_id(message_id);
                                                    editor.focus_handle(cx).focus(window);
                                                }
                                            })
                                        })),
                                )
                                .tooltip(Tooltip::text("Edit")),
                        ),
                    )
                })
            })
            .when_some(message_id, |el, message_id| {
                let this = cx.entity().clone();

                el.child(
                    self.render_popover_button(
                        cx,
                        div()
                            .child(
                                PopoverMenu::new(("menu", message_id))
                                    .trigger(IconButton::new(
                                        ("trigger", message_id),
                                        IconName::Ellipsis,
                                    ))
                                    .menu(move |window, cx| {
                                        Some(Self::render_message_menu(
                                            &this,
                                            message_id,
                                            can_delete_message,
                                            window,
                                            cx,
                                        ))
                                    }),
                            )
                            .id("more")
                            .tooltip(Tooltip::text("More")),
                    ),
                )
            })
    }

    fn render_message_menu(
        this: &Entity<Self>,
        message_id: u64,
        can_delete_message: bool,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<ContextMenu> {
        let menu = {
            ContextMenu::build(window, cx, move |menu, window, _| {
                menu.entry(
                    "Copy message text",
                    None,
                    window.handler_for(this, move |this, _, cx| {
                        if let Some(message) = this.active_chat().and_then(|active_chat| {
                            active_chat.read(cx).find_loaded_message(message_id)
                        }) {
                            let text = message.body.clone();
                            cx.write_to_clipboard(ClipboardItem::new_string(text))
                        }
                    }),
                )
                .when(can_delete_message, |menu| {
                    menu.entry(
                        "Delete message",
                        None,
                        window.handler_for(this, move |this, _, cx| {
                            this.remove_message(message_id, cx)
                        }),
                    )
                })
            })
        };
        this.update(cx, |this, cx| {
            let subscription = cx.subscribe_in(
                &menu,
                window,
                |this: &mut Self, _, _: &DismissEvent, _, _| {
                    this.open_context_menu = None;
                },
            );
            this.open_context_menu = Some((message_id, subscription));
        });
        menu
    }

    fn render_markdown_with_mentions(
        language_registry: &Arc<LanguageRegistry>,
        current_user_id: u64,
        message: &channel::ChannelMessage,
        local_timezone: UtcOffset,
        cx: &App,
    ) -> RichText {
        let mentions = message
            .mentions
            .iter()
            .map(|(range, user_id)| rich_text::Mention {
                range: range.clone(),
                is_self_mention: *user_id == current_user_id,
            })
            .collect::<Vec<_>>();

        const MESSAGE_EDITED: &str = " (edited)";

        let mut body = message.body.clone();

        if message.edited_at.is_some() {
            body.push_str(MESSAGE_EDITED);
        }

        let mut rich_text = RichText::new(body, &mentions, language_registry);

        if message.edited_at.is_some() {
            let range = (rich_text.text.len() - MESSAGE_EDITED.len())..rich_text.text.len();
            rich_text.highlights.push((
                range.clone(),
                Highlight::Highlight(HighlightStyle {
                    color: Some(cx.theme().colors().text_muted),
                    ..Default::default()
                }),
            ));

            if let Some(edit_timestamp) = message.edited_at {
                let edit_timestamp_text = time_format::format_localized_timestamp(
                    edit_timestamp,
                    OffsetDateTime::now_utc(),
                    local_timezone,
                    time_format::TimestampFormat::Absolute,
                );

                rich_text.custom_ranges.push(range);
                rich_text.set_tooltip_builder_for_custom_ranges(move |_, _, _, cx| {
                    Some(Tooltip::simple(edit_timestamp_text.clone(), cx))
                })
            }
        }
        rich_text
    }

    fn send(&mut self, _: &Confirm, window: &mut Window, cx: &mut Context<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            let message = self
                .message_editor
                .update(cx, |editor, cx| editor.take_message(window, cx));

            if let Some(id) = self.message_editor.read(cx).edit_message_id() {
                self.message_editor.update(cx, |editor, _| {
                    editor.clear_edit_message_id();
                });

                if let Some(task) = chat
                    .update(cx, |chat, cx| chat.update_message(id, message, cx))
                    .log_err()
                {
                    task.detach();
                }
            } else if let Some(task) = chat
                .update(cx, |chat, cx| chat.send_message(message, cx))
                .log_err()
            {
                task.detach();
            }
        }
    }

    fn remove_message(&mut self, id: u64, cx: &mut Context<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            chat.update(cx, |chat, cx| chat.remove_message(id, cx).detach())
        }
    }

    fn load_more_messages(&mut self, cx: &mut Context<Self>) {
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
        selected_channel_id: ChannelId,
        scroll_to_message_id: Option<u64>,
        cx: &mut Context<ChatPanel>,
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

        cx.spawn(async move |this, cx| {
            let chat = open_chat.await?;
            let highlight_message_id = scroll_to_message_id;
            let scroll_to_message_id = this.update(cx, |this, cx| {
                this.set_active_chat(chat.clone(), cx);

                scroll_to_message_id.or(this.last_acknowledged_message_id)
            })?;

            if let Some(message_id) = scroll_to_message_id {
                if let Some(item_ix) =
                    ChannelChat::load_history_since_message(chat.clone(), message_id, cx.clone())
                        .await
                {
                    this.update(cx, |this, cx| {
                        if let Some(highlight_message_id) = highlight_message_id {
                            let task = cx.spawn(async move |this, cx| {
                                cx.background_executor().timer(Duration::from_secs(2)).await;
                                this.update(cx, |this, cx| {
                                    this.highlighted_message.take();
                                    cx.notify();
                                })
                                .ok();
                            });

                            this.highlighted_message = Some((highlight_message_id, task));
                        }

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

    fn close_reply_preview(&mut self, cx: &mut Context<Self>) {
        self.message_editor
            .update(cx, |editor, _| editor.clear_reply_to_message_id());
    }

    fn cancel_edit_message(&mut self, cx: &mut Context<Self>) {
        self.message_editor.update(cx, |editor, cx| {
            // only clear the editor input if we were editing a message
            if editor.edit_message_id().is_none() {
                return;
            }

            editor.clear_edit_message_id();

            let buffer = editor
                .editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("message editor must be singleton");

            buffer.update(cx, |buffer, cx| buffer.set_text("", cx));
        });
    }
}

impl Render for ChatPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let channel_id = self
            .active_chat
            .as_ref()
            .map(|(c, _)| c.read(cx).channel_id);
        let message_editor = self.message_editor.read(cx);

        let reply_to_message_id = message_editor.reply_to_message_id();
        let edit_message_id = message_editor.edit_message_id();

        v_flex()
            .key_context("ChatPanel")
            .track_focus(&self.focus_handle)
            .size_full()
            .on_action(cx.listener(Self::send))
            .child(
                h_flex().child(
                    TabBar::new("chat_header").child(
                        h_flex()
                            .w_full()
                            .h(Tab::container_height(cx))
                            .px_2()
                            .child(Label::new(
                                self.active_chat
                                    .as_ref()
                                    .and_then(|c| {
                                        Some(format!("#{}", c.0.read(cx).channel(cx)?.name))
                                    })
                                    .unwrap_or("Chat".to_string()),
                            )),
                    ),
                ),
            )
            .child(div().flex_grow().px_2().map(|this| {
                if self.active_chat.is_some() {
                    this.child(
                        list(
                            self.message_list.clone(),
                            cx.processor(Self::render_message),
                        )
                        .size_full(),
                    )
                } else {
                    this.child(
                        div()
                            .size_full()
                            .p_4()
                            .child(
                                Label::new("Select a channel to chat in.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div().pt_1().w_full().items_center().child(
                                    Button::new("toggle-collab", "Open")
                                        .full_width()
                                        .key_binding(KeyBinding::for_action(
                                            &collab_panel::ToggleFocus,
                                            window,
                                            cx,
                                        ))
                                        .on_click(|_, window, cx| {
                                            window.dispatch_action(
                                                collab_panel::ToggleFocus.boxed_clone(),
                                                cx,
                                            )
                                        }),
                                ),
                            ),
                    )
                }
            }))
            .when(!self.is_scrolled_to_bottom, |el| {
                el.child(div().border_t_1().border_color(cx.theme().colors().border))
            })
            .when_some(edit_message_id, |el, _| {
                el.child(
                    h_flex()
                        .px_2()
                        .text_ui_xs(cx)
                        .justify_between()
                        .border_t_1()
                        .border_color(cx.theme().colors().border)
                        .bg(cx.theme().colors().background)
                        .child("Editing message")
                        .child(
                            IconButton::new("cancel-edit-message", IconName::Close)
                                .shape(ui::IconButtonShape::Square)
                                .tooltip(Tooltip::text("Cancel edit message"))
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.cancel_edit_message(cx);
                                })),
                        ),
                )
            })
            .when_some(reply_to_message_id, |el, reply_to_message_id| {
                let reply_message = self
                    .active_chat()
                    .and_then(|active_chat| {
                        active_chat
                            .read(cx)
                            .find_loaded_message(reply_to_message_id)
                    })
                    .cloned();

                el.when_some(reply_message, |el, reply_message| {
                    let user_being_replied_to = reply_message.sender.clone();

                    el.child(
                        h_flex()
                            .when(!self.is_scrolled_to_bottom, |el| {
                                el.border_t_1().border_color(cx.theme().colors().border)
                            })
                            .justify_between()
                            .overflow_hidden()
                            .items_start()
                            .py_1()
                            .px_2()
                            .bg(cx.theme().colors().background)
                            .child(
                                div().flex_shrink().overflow_hidden().child(
                                    h_flex()
                                        .id(("reply-preview", reply_to_message_id))
                                        .child(Label::new("Replying to ").size(LabelSize::Small))
                                        .child(
                                            Label::new(format!(
                                                "@{}",
                                                user_being_replied_to.github_login
                                            ))
                                            .size(LabelSize::Small)
                                            .weight(FontWeight::BOLD),
                                        )
                                        .when_some(channel_id, |this, channel_id| {
                                            this.cursor_pointer().on_click(cx.listener(
                                                move |chat_panel, _, _, cx| {
                                                    chat_panel
                                                        .select_channel(
                                                            channel_id,
                                                            reply_to_message_id.into(),
                                                            cx,
                                                        )
                                                        .detach_and_log_err(cx)
                                                },
                                            ))
                                        }),
                                ),
                            )
                            .child(
                                IconButton::new("close-reply-preview", IconName::Close)
                                    .shape(ui::IconButtonShape::Square)
                                    .tooltip(Tooltip::text("Close reply"))
                                    .on_click(cx.listener(move |this, _, _, cx| {
                                        this.close_reply_preview(cx);
                                    })),
                            ),
                    )
                })
            })
            .children(
                Some(
                    h_flex()
                        .p_2()
                        .on_action(cx.listener(|this, _: &actions::Cancel, _, cx| {
                            this.cancel_edit_message(cx);
                            this.close_reply_preview(cx);
                        }))
                        .map(|el| el.child(self.message_editor.clone())),
                )
                .filter(|_| self.active_chat.is_some()),
            )
            .into_any()
    }
}

impl Focusable for ChatPanel {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        if self.active_chat.is_some() {
            self.message_editor.read(cx).focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl Panel for ChatPanel {
    fn position(&self, _: &Window, cx: &App) -> DockPosition {
        ChatPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, _: &mut Window, cx: &mut Context<Self>) {
        settings::update_settings_file::<ChatPanelSettings>(
            self.fs.clone(),
            cx,
            move |settings, _| settings.dock = Some(position),
        );
    }

    fn size(&self, _: &Window, cx: &App) -> Pixels {
        self.width
            .unwrap_or_else(|| ChatPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, _: &mut Window, cx: &mut Context<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.active = active;
        if active {
            self.acknowledge_last_message(cx);
        }
    }

    fn persistent_name() -> &'static str {
        "ChatPanel"
    }

    fn icon(&self, _window: &Window, cx: &App) -> Option<ui::IconName> {
        self.enabled(cx).then(|| ui::IconName::Chat)
    }

    fn icon_tooltip(&self, _: &Window, _: &App) -> Option<&'static str> {
        Some("Chat Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }

    fn starts_open(&self, _: &Window, cx: &App) -> bool {
        ActiveCall::global(cx)
            .read(cx)
            .room()
            .is_some_and(|room| room.read(cx).contains_guests())
    }

    fn activation_priority(&self) -> u32 {
        7
    }

    fn enabled(&self, cx: &App) -> bool {
        match ChatPanelSettings::get_global(cx).button {
            ChatPanelButton::Never => false,
            ChatPanelButton::Always => true,
            ChatPanelButton::WhenInCall => {
                let is_in_call = ActiveCall::global(cx)
                    .read(cx)
                    .room()
                    .map_or(false, |room| room.read(cx).contains_guests());

                self.active || is_in_call
            }
        }
    }
}

impl EventEmitter<PanelEvent> for ChatPanel {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::HighlightStyle;
    use pretty_assertions::assert_eq;
    use rich_text::Highlight;
    use time::OffsetDateTime;
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_render_markdown_with_mentions(cx: &mut App) {
        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let (body, ranges) = marked_text_ranges("*hi*, «@abc», let's **call** «@fgh»", false);
        let message = channel::ChannelMessage {
            id: ChannelMessageId::Saved(0),
            body,
            timestamp: OffsetDateTime::now_utc(),
            sender: Arc::new(client::User {
                github_login: "fgh".into(),
                avatar_uri: "avatar_fgh".into(),
                id: 103,
                name: None,
            }),
            nonce: 5,
            mentions: vec![(ranges[0].clone(), 101), (ranges[1].clone(), 102)],
            reply_to_message_id: None,
            edited_at: None,
        };

        let message = ChatPanel::render_markdown_with_mentions(
            &language_registry,
            102,
            &message,
            UtcOffset::UTC,
            cx,
        );

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

    #[gpui::test]
    fn test_render_markdown_with_auto_detect_links(cx: &mut App) {
        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let message = channel::ChannelMessage {
            id: ChannelMessageId::Saved(0),
            body: "Here is a link https://zed.dev to zeds website".to_string(),
            timestamp: OffsetDateTime::now_utc(),
            sender: Arc::new(client::User {
                github_login: "fgh".into(),
                avatar_uri: "avatar_fgh".into(),
                id: 103,
                name: None,
            }),
            nonce: 5,
            mentions: Vec::new(),
            reply_to_message_id: None,
            edited_at: None,
        };

        let message = ChatPanel::render_markdown_with_mentions(
            &language_registry,
            102,
            &message,
            UtcOffset::UTC,
            cx,
        );

        // Note that the "'" was replaced with ’ due to smart punctuation.
        let (body, ranges) =
            marked_text_ranges("Here is a link «https://zed.dev» to zeds website", false);
        assert_eq!(message.text, body);
        assert_eq!(1, ranges.len());
        assert_eq!(
            message.highlights,
            vec![(
                ranges[0].clone(),
                HighlightStyle {
                    underline: Some(gpui::UnderlineStyle {
                        thickness: 1.0.into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
                .into()
            ),]
        );
    }

    #[gpui::test]
    fn test_render_markdown_with_auto_detect_links_and_additional_formatting(cx: &mut App) {
        let language_registry = Arc::new(LanguageRegistry::test(cx.background_executor().clone()));
        let message = channel::ChannelMessage {
            id: ChannelMessageId::Saved(0),
            body: "**Here is a link https://zed.dev to zeds website**".to_string(),
            timestamp: OffsetDateTime::now_utc(),
            sender: Arc::new(client::User {
                github_login: "fgh".into(),
                avatar_uri: "avatar_fgh".into(),
                id: 103,
                name: None,
            }),
            nonce: 5,
            mentions: Vec::new(),
            reply_to_message_id: None,
            edited_at: None,
        };

        let message = ChatPanel::render_markdown_with_mentions(
            &language_registry,
            102,
            &message,
            UtcOffset::UTC,
            cx,
        );

        // Note that the "'" was replaced with ’ due to smart punctuation.
        let (body, ranges) = marked_text_ranges(
            "«Here is a link »«https://zed.dev»« to zeds website»",
            false,
        );
        assert_eq!(message.text, body);
        assert_eq!(3, ranges.len());
        assert_eq!(
            message.highlights,
            vec![
                (
                    ranges[0].clone(),
                    HighlightStyle {
                        font_weight: Some(gpui::FontWeight::BOLD),
                        ..Default::default()
                    }
                    .into()
                ),
                (
                    ranges[1].clone(),
                    HighlightStyle {
                        font_weight: Some(gpui::FontWeight::BOLD),
                        underline: Some(gpui::UnderlineStyle {
                            thickness: 1.0.into(),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                    .into()
                ),
                (
                    ranges[2].clone(),
                    HighlightStyle {
                        font_weight: Some(gpui::FontWeight::BOLD),
                        ..Default::default()
                    }
                    .into()
                ),
            ]
        );
    }
}
