use std::sync::Arc;

use call::ActiveCall;
use client::{Contact, User, UserStore};
use editor::{Cancel, Editor};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    elements::*, impl_internal_actions, keymap, AppContext, ClipboardItem, CursorStyle, Entity,
    ModelHandle, MouseButton, MutableAppContext, RenderContext, Subscription, View, ViewContext,
    ViewHandle,
};
use menu::{Confirm, SelectNext, SelectPrev};
use settings::Settings;
use theme::IconButton;

impl_internal_actions!(contacts_panel, [ToggleExpanded, Call]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ContactsPopover::clear_filter);
    cx.add_action(ContactsPopover::select_next);
    cx.add_action(ContactsPopover::select_prev);
    cx.add_action(ContactsPopover::confirm);
    cx.add_action(ContactsPopover::toggle_expanded);
    cx.add_action(ContactsPopover::call);
}

#[derive(Clone, PartialEq)]
struct ToggleExpanded(Section);

#[derive(Clone, PartialEq)]
struct Call {
    recipient_user_id: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Section {
    Requests,
    Online,
    Offline,
}

#[derive(Clone)]
enum ContactEntry {
    Header(Section),
    IncomingRequest(Arc<User>),
    OutgoingRequest(Arc<User>),
    Contact(Arc<Contact>),
}

impl PartialEq for ContactEntry {
    fn eq(&self, other: &Self) -> bool {
        match self {
            ContactEntry::Header(section_1) => {
                if let ContactEntry::Header(section_2) = other {
                    return section_1 == section_2;
                }
            }
            ContactEntry::IncomingRequest(user_1) => {
                if let ContactEntry::IncomingRequest(user_2) = other {
                    return user_1.id == user_2.id;
                }
            }
            ContactEntry::OutgoingRequest(user_1) => {
                if let ContactEntry::OutgoingRequest(user_2) = other {
                    return user_1.id == user_2.id;
                }
            }
            ContactEntry::Contact(contact_1) => {
                if let ContactEntry::Contact(contact_2) = other {
                    return contact_1.user.id == contact_2.user.id;
                }
            }
        }
        false
    }
}

pub enum Event {
    Dismissed,
}

pub struct ContactsPopover {
    room_subscription: Option<Subscription>,
    entries: Vec<ContactEntry>,
    match_candidates: Vec<StringMatchCandidate>,
    list_state: ListState,
    user_store: ModelHandle<UserStore>,
    filter_editor: ViewHandle<Editor>,
    collapsed_sections: Vec<Section>,
    selection: Option<usize>,
    _subscriptions: Vec<Subscription>,
}

impl ContactsPopover {
    pub fn new(user_store: ModelHandle<UserStore>, cx: &mut ViewContext<Self>) -> Self {
        let filter_editor = cx.add_view(|cx| {
            let mut editor = Editor::single_line(
                Some(|theme| theme.contacts_panel.user_query_editor.clone()),
                cx,
            );
            editor.set_placeholder_text("Filter contacts", cx);
            editor
        });

        cx.subscribe(&filter_editor, |this, _, event, cx| {
            if let editor::Event::BufferEdited = event {
                let query = this.filter_editor.read(cx).text(cx);
                if !query.is_empty() {
                    this.selection.take();
                }
                this.update_entries(cx);
                if !query.is_empty() {
                    this.selection = this
                        .entries
                        .iter()
                        .position(|entry| !matches!(entry, ContactEntry::Header(_)));
                }
            }
        })
        .detach();

        let list_state = ListState::new(0, Orientation::Top, 1000., cx, move |this, ix, cx| {
            let theme = cx.global::<Settings>().theme.clone();
            let is_selected = this.selection == Some(ix);

            match &this.entries[ix] {
                ContactEntry::Header(section) => {
                    let is_collapsed = this.collapsed_sections.contains(section);
                    Self::render_header(
                        *section,
                        &theme.contacts_panel,
                        is_selected,
                        is_collapsed,
                        cx,
                    )
                }
                ContactEntry::IncomingRequest(user) => Self::render_contact_request(
                    user.clone(),
                    this.user_store.clone(),
                    &theme.contacts_panel,
                    true,
                    is_selected,
                    cx,
                ),
                ContactEntry::OutgoingRequest(user) => Self::render_contact_request(
                    user.clone(),
                    this.user_store.clone(),
                    &theme.contacts_panel,
                    false,
                    is_selected,
                    cx,
                ),
                ContactEntry::Contact(contact) => {
                    Self::render_contact(contact, &theme.contacts_panel, is_selected, cx)
                }
            }
        });

        let active_call = ActiveCall::global(cx);
        let mut subscriptions = Vec::new();
        subscriptions.push(cx.observe(&user_store, |this, _, cx| this.update_entries(cx)));
        subscriptions.push(cx.observe(&active_call, |this, _, cx| this.active_call_changed(cx)));

        let mut this = Self {
            room_subscription: None,
            list_state,
            selection: None,
            collapsed_sections: Default::default(),
            entries: Default::default(),
            match_candidates: Default::default(),
            filter_editor,
            _subscriptions: subscriptions,
            user_store,
        };
        this.update_entries(cx);
        this.active_call_changed(cx);
        this
    }

    fn active_call_changed(&mut self, cx: &mut ViewContext<Self>) {
        if let Some(room) = ActiveCall::global(cx).read(cx).room().cloned() {
            self.room_subscription = Some(cx.observe(&room, |_, _, cx| cx.notify()));
        } else {
            self.room_subscription = None;
        }
        cx.notify();
    }

    fn clear_filter(&mut self, _: &Cancel, cx: &mut ViewContext<Self>) {
        let did_clear = self.filter_editor.update(cx, |editor, cx| {
            if editor.buffer().read(cx).len(cx) > 0 {
                editor.set_text("", cx);
                true
            } else {
                false
            }
        });
        if !did_clear {
            cx.emit(Event::Dismissed);
        }
    }

    fn select_next(&mut self, _: &SelectNext, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selection {
            if self.entries.len() > ix + 1 {
                self.selection = Some(ix + 1);
            }
        } else if !self.entries.is_empty() {
            self.selection = Some(0);
        }
        cx.notify();
        self.list_state.reset(self.entries.len());
    }

    fn select_prev(&mut self, _: &SelectPrev, cx: &mut ViewContext<Self>) {
        if let Some(ix) = self.selection {
            if ix > 0 {
                self.selection = Some(ix - 1);
            } else {
                self.selection = None;
            }
        }
        cx.notify();
        self.list_state.reset(self.entries.len());
    }

    fn confirm(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some(selection) = self.selection {
            if let Some(entry) = self.entries.get(selection) {
                match entry {
                    ContactEntry::Header(section) => {
                        let section = *section;
                        self.toggle_expanded(&ToggleExpanded(section), cx);
                    }
                    _ => {}
                }
            }
        }
    }

    fn toggle_expanded(&mut self, action: &ToggleExpanded, cx: &mut ViewContext<Self>) {
        let section = action.0;
        if let Some(ix) = self.collapsed_sections.iter().position(|s| *s == section) {
            self.collapsed_sections.remove(ix);
        } else {
            self.collapsed_sections.push(section);
        }
        self.update_entries(cx);
    }

    fn update_entries(&mut self, cx: &mut ViewContext<Self>) {
        let user_store = self.user_store.read(cx);
        let query = self.filter_editor.read(cx).text(cx);
        let executor = cx.background().clone();

        let prev_selected_entry = self.selection.and_then(|ix| self.entries.get(ix).cloned());
        self.entries.clear();

        let mut request_entries = Vec::new();
        let incoming = user_store.incoming_contact_requests();
        if !incoming.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    incoming
                        .iter()
                        .enumerate()
                        .map(|(ix, user)| StringMatchCandidate {
                            id: ix,
                            string: user.github_login.clone(),
                            char_bag: user.github_login.chars().collect(),
                        }),
                );
            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));
            request_entries.extend(
                matches
                    .iter()
                    .map(|mat| ContactEntry::IncomingRequest(incoming[mat.candidate_id].clone())),
            );
        }

        let outgoing = user_store.outgoing_contact_requests();
        if !outgoing.is_empty() {
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    outgoing
                        .iter()
                        .enumerate()
                        .map(|(ix, user)| StringMatchCandidate {
                            id: ix,
                            string: user.github_login.clone(),
                            char_bag: user.github_login.chars().collect(),
                        }),
                );
            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));
            request_entries.extend(
                matches
                    .iter()
                    .map(|mat| ContactEntry::OutgoingRequest(outgoing[mat.candidate_id].clone())),
            );
        }

        if !request_entries.is_empty() {
            self.entries.push(ContactEntry::Header(Section::Requests));
            if !self.collapsed_sections.contains(&Section::Requests) {
                self.entries.append(&mut request_entries);
            }
        }

        let contacts = user_store.contacts();
        if !contacts.is_empty() {
            // Always put the current user first.
            self.match_candidates.clear();
            self.match_candidates
                .extend(
                    contacts
                        .iter()
                        .enumerate()
                        .map(|(ix, contact)| StringMatchCandidate {
                            id: ix,
                            string: contact.user.github_login.clone(),
                            char_bag: contact.user.github_login.chars().collect(),
                        }),
                );

            let matches = executor.block(match_strings(
                &self.match_candidates,
                &query,
                true,
                usize::MAX,
                &Default::default(),
                executor.clone(),
            ));

            let (online_contacts, offline_contacts) = matches
                .iter()
                .partition::<Vec<_>, _>(|mat| contacts[mat.candidate_id].online);

            for (matches, section) in [
                (online_contacts, Section::Online),
                (offline_contacts, Section::Offline),
            ] {
                if !matches.is_empty() {
                    self.entries.push(ContactEntry::Header(section));
                    if !self.collapsed_sections.contains(&section) {
                        for mat in matches {
                            let contact = &contacts[mat.candidate_id];
                            self.entries.push(ContactEntry::Contact(contact.clone()));
                        }
                    }
                }
            }
        }

        if let Some(prev_selected_entry) = prev_selected_entry {
            self.selection.take();
            for (ix, entry) in self.entries.iter().enumerate() {
                if *entry == prev_selected_entry {
                    self.selection = Some(ix);
                    break;
                }
            }
        }

        self.list_state.reset(self.entries.len());
        cx.notify();
    }

    fn render_active_call(&self, cx: &mut RenderContext<Self>) -> Option<ElementBox> {
        let room = ActiveCall::global(cx).read(cx).room()?;
        let theme = &cx.global::<Settings>().theme.contacts_panel;

        Some(
            Flex::column()
                .with_children(room.read(cx).pending_users().iter().map(|user| {
                    Flex::row()
                        .with_children(user.avatar.clone().map(|avatar| {
                            Image::new(avatar)
                                .with_style(theme.contact_avatar)
                                .aligned()
                                .left()
                                .boxed()
                        }))
                        .with_child(
                            Label::new(
                                user.github_login.clone(),
                                theme.contact_username.text.clone(),
                            )
                            .contained()
                            .with_style(theme.contact_username.container)
                            .aligned()
                            .left()
                            .flex(1., true)
                            .boxed(),
                        )
                        .constrained()
                        .with_height(theme.row_height)
                        .contained()
                        .with_style(theme.contact_row.default)
                        .boxed()
                }))
                .boxed(),
        )
    }

    fn render_header(
        section: Section,
        theme: &theme::ContactsPanel,
        is_selected: bool,
        is_collapsed: bool,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        enum Header {}

        let header_style = theme.header_row.style_for(Default::default(), is_selected);
        let text = match section {
            Section::Requests => "Requests",
            Section::Online => "Online",
            Section::Offline => "Offline",
        };
        let icon_size = theme.section_icon_size;
        MouseEventHandler::<Header>::new(section as usize, cx, |_, _| {
            Flex::row()
                .with_child(
                    Svg::new(if is_collapsed {
                        "icons/chevron_right_8.svg"
                    } else {
                        "icons/chevron_down_8.svg"
                    })
                    .with_color(header_style.text.color)
                    .constrained()
                    .with_max_width(icon_size)
                    .with_max_height(icon_size)
                    .aligned()
                    .constrained()
                    .with_width(icon_size)
                    .boxed(),
                )
                .with_child(
                    Label::new(text.to_string(), header_style.text.clone())
                        .aligned()
                        .left()
                        .contained()
                        .with_margin_left(theme.contact_username.container.margin.left)
                        .flex(1., true)
                        .boxed(),
                )
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(header_style.container)
                .boxed()
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(ToggleExpanded(section))
        })
        .boxed()
    }

    fn render_contact(
        contact: &Contact,
        theme: &theme::ContactsPanel,
        is_selected: bool,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        let user_id = contact.user.id;
        MouseEventHandler::<Contact>::new(contact.user.id as usize, cx, |_, _| {
            Flex::row()
                .with_children(contact.user.avatar.clone().map(|avatar| {
                    Image::new(avatar)
                        .with_style(theme.contact_avatar)
                        .aligned()
                        .left()
                        .boxed()
                }))
                .with_child(
                    Label::new(
                        contact.user.github_login.clone(),
                        theme.contact_username.text.clone(),
                    )
                    .contained()
                    .with_style(theme.contact_username.container)
                    .aligned()
                    .left()
                    .flex(1., true)
                    .boxed(),
                )
                .constrained()
                .with_height(theme.row_height)
                .contained()
                .with_style(*theme.contact_row.style_for(Default::default(), is_selected))
                .boxed()
        })
        .on_click(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(Call {
                recipient_user_id: user_id,
            })
        })
        .boxed()
    }

    fn render_contact_request(
        user: Arc<User>,
        user_store: ModelHandle<UserStore>,
        theme: &theme::ContactsPanel,
        is_incoming: bool,
        is_selected: bool,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        enum Decline {}
        enum Accept {}
        enum Cancel {}

        let mut row = Flex::row()
            .with_children(user.avatar.clone().map(|avatar| {
                Image::new(avatar)
                    .with_style(theme.contact_avatar)
                    .aligned()
                    .left()
                    .boxed()
            }))
            .with_child(
                Label::new(
                    user.github_login.clone(),
                    theme.contact_username.text.clone(),
                )
                .contained()
                .with_style(theme.contact_username.container)
                .aligned()
                .left()
                .flex(1., true)
                .boxed(),
            );

        let user_id = user.id;
        let is_contact_request_pending = user_store.read(cx).is_contact_request_pending(&user);
        let button_spacing = theme.contact_button_spacing;

        if is_incoming {
            row.add_children([
                MouseEventHandler::<Decline>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/x_mark_8.svg")
                        .aligned()
                        // .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    todo!();
                    // cx.dispatch_action(RespondToContactRequest {
                    //     user_id,
                    //     accept: false,
                    // })
                })
                // .flex_float()
                .contained()
                .with_margin_right(button_spacing)
                .boxed(),
                MouseEventHandler::<Accept>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/check_8.svg")
                        .aligned()
                        .flex_float()
                        .boxed()
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    todo!()
                    // cx.dispatch_action(RespondToContactRequest {
                    //     user_id,
                    //     accept: true,
                    // })
                })
                .boxed(),
            ]);
        } else {
            row.add_child(
                MouseEventHandler::<Cancel>::new(user.id as usize, cx, |mouse_state, _| {
                    let button_style = if is_contact_request_pending {
                        &theme.disabled_button
                    } else {
                        theme.contact_button.style_for(mouse_state, false)
                    };
                    render_icon_button(button_style, "icons/x_mark_8.svg")
                        .aligned()
                        .flex_float()
                        .boxed()
                })
                .with_padding(Padding::uniform(2.))
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, cx| {
                    todo!()
                    // cx.dispatch_action(RemoveContact(user_id))
                })
                .flex_float()
                .boxed(),
            );
        }

        row.constrained()
            .with_height(theme.row_height)
            .contained()
            .with_style(*theme.contact_row.style_for(Default::default(), is_selected))
            .boxed()
    }

    fn call(&mut self, action: &Call, cx: &mut ViewContext<Self>) {
        ActiveCall::global(cx)
            .update(cx, |active_call, cx| {
                active_call.invite(action.recipient_user_id, cx)
            })
            .detach_and_log_err(cx);
    }
}

impl Entity for ContactsPopover {
    type Event = Event;
}

impl View for ContactsPopover {
    fn ui_name() -> &'static str {
        "ContactsPopover"
    }

    fn keymap_context(&self, _: &AppContext) -> keymap::Context {
        let mut cx = Self::default_keymap_context();
        cx.set.insert("menu".into());
        cx
    }

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        enum AddContact {}
        let theme = cx.global::<Settings>().theme.clone();

        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(
                        ChildView::new(self.filter_editor.clone())
                            .contained()
                            .with_style(theme.contacts_panel.user_query_editor.container)
                            .flex(1., true)
                            .boxed(),
                    )
                    .with_child(
                        MouseEventHandler::<AddContact>::new(0, cx, |_, _| {
                            Svg::new("icons/user_plus_16.svg")
                                .with_color(theme.contacts_panel.add_contact_button.color)
                                .constrained()
                                .with_height(16.)
                                .contained()
                                .with_style(theme.contacts_panel.add_contact_button.container)
                                .aligned()
                                .boxed()
                        })
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, |_, cx| {
                            todo!()
                            // cx.dispatch_action(contact_finder::Toggle)
                        })
                        .boxed(),
                    )
                    .constrained()
                    .with_height(theme.contacts_panel.user_query_editor_height)
                    .boxed(),
            )
            .with_children(self.render_active_call(cx))
            .with_child(List::new(self.list_state.clone()).flex(1., false).boxed())
            .with_children(
                self.user_store
                    .read(cx)
                    .invite_info()
                    .cloned()
                    .and_then(|info| {
                        enum InviteLink {}

                        if info.count > 0 {
                            Some(
                                MouseEventHandler::<InviteLink>::new(0, cx, |state, cx| {
                                    let style = theme
                                        .contacts_panel
                                        .invite_row
                                        .style_for(state, false)
                                        .clone();

                                    let copied = cx.read_from_clipboard().map_or(false, |item| {
                                        item.text().as_str() == info.url.as_ref()
                                    });

                                    Label::new(
                                        format!(
                                            "{} invite link ({} left)",
                                            if copied { "Copied" } else { "Copy" },
                                            info.count
                                        ),
                                        style.label.clone(),
                                    )
                                    .aligned()
                                    .left()
                                    .constrained()
                                    .with_height(theme.contacts_panel.row_height)
                                    .contained()
                                    .with_style(style.container)
                                    .boxed()
                                })
                                .with_cursor_style(CursorStyle::PointingHand)
                                .on_click(MouseButton::Left, move |_, cx| {
                                    cx.write_to_clipboard(ClipboardItem::new(info.url.to_string()));
                                    cx.notify();
                                })
                                .boxed(),
                            )
                        } else {
                            None
                        }
                    }),
            )
            .contained()
            .with_style(theme.workspace.titlebar.contacts_popover.container)
            .constrained()
            .with_width(theme.workspace.titlebar.contacts_popover.width)
            .with_height(theme.workspace.titlebar.contacts_popover.height)
            .boxed()
    }

    fn on_focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.filter_editor.is_focused(cx) {
            cx.focus(&self.filter_editor);
        }
    }

    fn on_focus_out(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if !self.filter_editor.is_focused(cx) {
            cx.emit(Event::Dismissed);
        }
    }
}

fn render_icon_button(style: &IconButton, svg_path: &'static str) -> impl Element {
    Svg::new(svg_path)
        .with_color(style.color)
        .constrained()
        .with_width(style.icon_width)
        .aligned()
        .contained()
        .with_style(style.container)
        .constrained()
        .with_width(style.button_width)
        .with_height(style.button_width)
}
