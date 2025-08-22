use channel::{ChannelMembership, ChannelStore};
use client::{
    ChannelId, User, UserId, UserStore,
    proto::{self, ChannelRole, ChannelVisibility},
};
use fuzzy::{StringMatchCandidate, match_strings};
use gpui::{
    App, ClipboardItem, Context, DismissEvent, Entity, EventEmitter, Focusable, ParentElement,
    Render, Styled, Subscription, Task, WeakEntity, Window, actions, anchored, deferred, div,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{Avatar, CheckboxWithLabel, ContextMenu, ListItem, ListItemSpacing, prelude::*};
use util::TryFutureExt;
use workspace::{ModalView, notifications::DetachAndPromptErr};

actions!(
    channel_modal,
    [
        /// Selects the next control in the channel modal.
        SelectNextControl,
        /// Toggles between invite members and manage members mode.
        ToggleMode,
        /// Toggles admin status for the selected member.
        ToggleMemberAdmin,
        /// Removes the selected member from the channel.
        RemoveMember
    ]
);

pub struct ChannelModal {
    picker: Entity<Picker<ChannelModalDelegate>>,
    channel_store: Entity<ChannelStore>,
    channel_id: ChannelId,
}

impl ChannelModal {
    pub fn new(
        user_store: Entity<UserStore>,
        channel_store: Entity<ChannelStore>,
        channel_id: ChannelId,
        mode: Mode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe(&channel_store, |_, _, cx| cx.notify()).detach();
        let channel_modal = cx.entity().downgrade();
        let picker = cx.new(|cx| {
            Picker::uniform_list(
                ChannelModalDelegate {
                    channel_modal,
                    matching_users: Vec::new(),
                    matching_member_indices: Vec::new(),
                    selected_index: 0,
                    user_store: user_store.clone(),
                    channel_store: channel_store.clone(),
                    channel_id,
                    match_candidates: Vec::new(),
                    context_menu: None,
                    members: Vec::new(),
                    has_all_members: false,
                    mode,
                },
                window,
                cx,
            )
            .modal(false)
        });

        Self {
            picker,
            channel_store,
            channel_id,
        }
    }

    fn toggle_mode(&mut self, _: &ToggleMode, window: &mut Window, cx: &mut Context<Self>) {
        let mode = match self.picker.read(cx).delegate.mode {
            Mode::ManageMembers => Mode::InviteMembers,
            Mode::InviteMembers => Mode::ManageMembers,
        };
        self.set_mode(mode, window, cx);
    }

    fn set_mode(&mut self, mode: Mode, window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            let delegate = &mut picker.delegate;
            delegate.mode = mode;
            delegate.selected_index = 0;
            picker.set_query("", window, cx);
            picker.update_matches(picker.query(cx), window, cx);
            cx.notify()
        });
        cx.notify()
    }

    fn set_channel_visibility(
        &mut self,
        selection: &ToggleState,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.channel_store.update(cx, |channel_store, cx| {
            channel_store
                .set_channel_visibility(
                    self.channel_id,
                    match selection {
                        ToggleState::Unselected => ChannelVisibility::Members,
                        ToggleState::Selected => ChannelVisibility::Public,
                        ToggleState::Indeterminate => return,
                    },
                    cx,
                )
                .detach_and_log_err(cx)
        });
    }

    fn dismiss(&mut self, _: &menu::Cancel, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ChannelModal {}
impl ModalView for ChannelModal {}

impl Focusable for ChannelModal {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ChannelModal {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let channel_store = self.channel_store.read(cx);
        let Some(channel) = channel_store.channel_for_id(self.channel_id) else {
            return div();
        };
        let channel_name = channel.name.clone();
        let channel_id = channel.id;
        let visibility = channel.visibility;
        let mode = self.picker.read(cx).delegate.mode;

        v_flex()
            .key_context("ChannelModal")
            .on_action(cx.listener(Self::toggle_mode))
            .on_action(cx.listener(Self::dismiss))
            .elevation_3(cx)
            .w(rems(34.))
            .child(
                v_flex()
                    .px_2()
                    .py_1()
                    .gap_2()
                    .child(
                        h_flex()
                            .w_px()
                            .flex_1()
                            .gap_1()
                            .child(Icon::new(IconName::Hash).size(IconSize::Medium))
                            .child(Label::new(channel_name)),
                    )
                    .child(
                        h_flex()
                            .w_full()
                            .h(rems_from_px(22.))
                            .justify_between()
                            .line_height(rems(1.25))
                            .child(CheckboxWithLabel::new(
                                "is-public",
                                Label::new("Public").size(LabelSize::Small),
                                if visibility == ChannelVisibility::Public {
                                    ui::ToggleState::Selected
                                } else {
                                    ui::ToggleState::Unselected
                                },
                                cx.listener(Self::set_channel_visibility),
                            ))
                            .children(
                                Some(
                                    Button::new("copy-link", "Copy Link")
                                        .label_size(LabelSize::Small)
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            if let Some(channel) = this
                                                .channel_store
                                                .read(cx)
                                                .channel_for_id(channel_id)
                                            {
                                                let item =
                                                    ClipboardItem::new_string(channel.link(cx));
                                                cx.write_to_clipboard(item);
                                            }
                                        })),
                                )
                                .filter(|_| visibility == ChannelVisibility::Public),
                            ),
                    )
                    .child(
                        h_flex()
                            .child(
                                div()
                                    .id("manage-members")
                                    .px_2()
                                    .py_1()
                                    .cursor_pointer()
                                    .border_b_2()
                                    .when(mode == Mode::ManageMembers, |this| {
                                        this.border_color(cx.theme().colors().border)
                                    })
                                    .child(Label::new("Manage Members"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.set_mode(Mode::ManageMembers, window, cx);
                                    })),
                            )
                            .child(
                                div()
                                    .id("invite-members")
                                    .px_2()
                                    .py_1()
                                    .cursor_pointer()
                                    .border_b_2()
                                    .when(mode == Mode::InviteMembers, |this| {
                                        this.border_color(cx.theme().colors().border)
                                    })
                                    .child(Label::new("Invite Members"))
                                    .on_click(cx.listener(|this, _, window, cx| {
                                        this.set_mode(Mode::InviteMembers, window, cx);
                                    })),
                            ),
                    ),
            )
            .child(self.picker.clone())
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    ManageMembers,
    InviteMembers,
}

pub struct ChannelModalDelegate {
    channel_modal: WeakEntity<ChannelModal>,
    matching_users: Vec<Arc<User>>,
    matching_member_indices: Vec<usize>,
    user_store: Entity<UserStore>,
    channel_store: Entity<ChannelStore>,
    channel_id: ChannelId,
    selected_index: usize,
    mode: Mode,
    match_candidates: Vec<StringMatchCandidate>,
    members: Vec<ChannelMembership>,
    has_all_members: bool,
    context_menu: Option<(Entity<ContextMenu>, Subscription)>,
}

impl PickerDelegate for ChannelModalDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Search collaborator by username...".into()
    }

    fn match_count(&self) -> usize {
        match self.mode {
            Mode::ManageMembers => self.matching_member_indices.len(),
            Mode::InviteMembers => self.matching_users.len(),
        }
    }

    fn selected_index(&self) -> usize {
        self.selected_index
    }

    fn set_selected_index(
        &mut self,
        ix: usize,
        _window: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        match self.mode {
            Mode::ManageMembers => {
                if self.has_all_members {
                    self.match_candidates.clear();
                    self.match_candidates
                        .extend(self.members.iter().enumerate().map(|(id, member)| {
                            StringMatchCandidate::new(id, &member.user.github_login)
                        }));

                    let matches = cx.background_executor().block(match_strings(
                        &self.match_candidates,
                        &query,
                        true,
                        true,
                        usize::MAX,
                        &Default::default(),
                        cx.background_executor().clone(),
                    ));

                    cx.spawn_in(window, async move |picker, cx| {
                        picker
                            .update(cx, |picker, cx| {
                                let delegate = &mut picker.delegate;
                                delegate.matching_member_indices.clear();
                                delegate
                                    .matching_member_indices
                                    .extend(matches.into_iter().map(|m| m.candidate_id));
                                cx.notify();
                            })
                            .ok();
                    })
                } else {
                    let search_members = self.channel_store.update(cx, |store, cx| {
                        store.fuzzy_search_members(self.channel_id, query.clone(), 100, cx)
                    });
                    cx.spawn_in(window, async move |picker, cx| {
                        async {
                            let members = search_members.await?;
                            picker.update(cx, |picker, cx| {
                                picker.delegate.has_all_members =
                                    query.is_empty() && members.len() < 100;
                                picker.delegate.matching_member_indices =
                                    (0..members.len()).collect();
                                picker.delegate.members = members;
                                cx.notify();
                            })?;
                            anyhow::Ok(())
                        }
                        .log_err()
                        .await;
                    })
                }
            }
            Mode::InviteMembers => {
                let search_users = self
                    .user_store
                    .update(cx, |store, cx| store.fuzzy_search_users(query, cx));
                cx.spawn_in(window, async move |picker, cx| {
                    async {
                        let users = search_users.await?;
                        picker.update(cx, |picker, cx| {
                            picker.delegate.matching_users = users;
                            cx.notify();
                        })?;
                        anyhow::Ok(())
                    }
                    .log_err()
                    .await;
                })
            }
        }
    }

    fn confirm(&mut self, _: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        if let Some(selected_user) = self.user_at_index(self.selected_index) {
            if Some(selected_user.id) == self.user_store.read(cx).current_user().map(|user| user.id)
            {
                return;
            }
            match self.mode {
                Mode::ManageMembers => self.show_context_menu(self.selected_index, window, cx),
                Mode::InviteMembers => match self.member_status(selected_user.id, cx) {
                    Some(proto::channel_member::Kind::Invitee) => {
                        self.remove_member(selected_user.id, window, cx);
                    }
                    Some(proto::channel_member::Kind::Member) => {}
                    None => self.invite_member(selected_user, window, cx),
                },
            }
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        if self.context_menu.is_none() {
            self.channel_modal
                .update(cx, |_, cx| {
                    cx.emit(DismissEvent);
                })
                .ok();
        }
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let user = self.user_at_index(ix)?;
        let membership = self.member_at_index(ix);
        let request_status = self.member_status(user.id, cx);
        let is_me = self.user_store.read(cx).current_user().map(|user| user.id) == Some(user.id);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .start_slot(Avatar::new(user.avatar_uri.clone()))
                .child(Label::new(user.github_login.clone()))
                .end_slot(h_flex().gap_2().map(|slot| {
                    match self.mode {
                        Mode::ManageMembers => slot
                            .children(
                                if request_status == Some(proto::channel_member::Kind::Invitee) {
                                    Some(Label::new("Invited"))
                                } else {
                                    None
                                },
                            )
                            .children(match membership.map(|m| m.role) {
                                Some(ChannelRole::Admin) => Some(Label::new("Admin")),
                                Some(ChannelRole::Guest) => Some(Label::new("Guest")),
                                _ => None,
                            })
                            .when(!is_me, |el| {
                                el.child(IconButton::new("ellipsis", IconName::Ellipsis))
                            })
                            .when(is_me, |el| el.child(Label::new("You").color(Color::Muted)))
                            .children(
                                if let (Some((menu, _)), true) = (&self.context_menu, selected) {
                                    Some(
                                        deferred(
                                            anchored()
                                                .anchor(gpui::Corner::TopRight)
                                                .child(menu.clone()),
                                        )
                                        .with_priority(1),
                                    )
                                } else {
                                    None
                                },
                            ),
                        Mode::InviteMembers => match request_status {
                            Some(proto::channel_member::Kind::Invitee) => {
                                slot.children(Some(Label::new("Invited")))
                            }
                            Some(proto::channel_member::Kind::Member) => {
                                slot.children(Some(Label::new("Member")))
                            }
                            _ => slot,
                        },
                    }
                })),
        )
    }
}

impl ChannelModalDelegate {
    fn member_status(&self, user_id: UserId, cx: &App) -> Option<proto::channel_member::Kind> {
        self.members
            .iter()
            .find_map(|membership| (membership.user.id == user_id).then_some(membership.kind))
            .or_else(|| {
                self.channel_store
                    .read(cx)
                    .has_pending_channel_invite(self.channel_id, user_id)
                    .then_some(proto::channel_member::Kind::Invitee)
            })
    }

    fn member_at_index(&self, ix: usize) -> Option<&ChannelMembership> {
        self.matching_member_indices
            .get(ix)
            .and_then(|ix| self.members.get(*ix))
    }

    fn user_at_index(&self, ix: usize) -> Option<Arc<User>> {
        match self.mode {
            Mode::ManageMembers => self.matching_member_indices.get(ix).and_then(|ix| {
                let channel_membership = self.members.get(*ix)?;
                Some(channel_membership.user.clone())
            }),
            Mode::InviteMembers => self.matching_users.get(ix).cloned(),
        }
    }

    fn set_user_role(
        &mut self,
        user_id: UserId,
        new_role: ChannelRole,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<()> {
        let update = self.channel_store.update(cx, |store, cx| {
            store.set_member_role(self.channel_id, user_id, new_role, cx)
        });
        cx.spawn_in(window, async move |picker, cx| {
            update.await?;
            picker.update_in(cx, |picker, window, cx| {
                let this = &mut picker.delegate;
                if let Some(member) = this.members.iter_mut().find(|m| m.user.id == user_id) {
                    member.role = new_role;
                }
                cx.focus_self(window);
                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to update role", window, cx, |_, _, _| None);
        Some(())
    }

    fn remove_member(
        &mut self,
        user_id: UserId,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<()> {
        let update = self.channel_store.update(cx, |store, cx| {
            store.remove_member(self.channel_id, user_id, cx)
        });
        cx.spawn_in(window, async move |picker, cx| {
            update.await?;
            picker.update_in(cx, |picker, window, cx| {
                let this = &mut picker.delegate;
                if let Some(ix) = this.members.iter_mut().position(|m| m.user.id == user_id) {
                    this.members.remove(ix);
                    this.matching_member_indices.retain_mut(|member_ix| {
                        if *member_ix == ix {
                            return false;
                        } else if *member_ix > ix {
                            *member_ix -= 1;
                        }
                        true
                    })
                }

                this.selected_index = this
                    .selected_index
                    .min(this.matching_member_indices.len().saturating_sub(1));

                picker.focus(window, cx);
                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to remove member", window, cx, |_, _, _| None);
        Some(())
    }

    fn invite_member(
        &mut self,
        user: Arc<User>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let invite_member = self.channel_store.update(cx, |store, cx| {
            store.invite_member(self.channel_id, user.id, ChannelRole::Member, cx)
        });

        cx.spawn_in(window, async move |this, cx| {
            invite_member.await?;

            this.update(cx, |this, cx| {
                let new_member = ChannelMembership {
                    user,
                    kind: proto::channel_member::Kind::Invitee,
                    role: ChannelRole::Member,
                };
                let members = &mut this.delegate.members;
                match members.binary_search_by_key(&new_member.sort_key(), |k| k.sort_key()) {
                    Ok(ix) | Err(ix) => members.insert(ix, new_member),
                }

                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to invite member", window, cx, |_, _, _| None);
    }

    fn show_context_menu(
        &mut self,
        ix: usize,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        let Some(membership) = self.member_at_index(ix) else {
            return;
        };
        let user_id = membership.user.id;
        let picker = cx.entity();
        let context_menu = ContextMenu::build(window, cx, |mut menu, _window, _cx| {
            let role = membership.role;

            if role == ChannelRole::Admin || role == ChannelRole::Member {
                let picker = picker.clone();
                menu = menu.entry("Demote to Guest", None, move |window, cx| {
                    picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .set_user_role(user_id, ChannelRole::Guest, window, cx);
                    })
                });
            }

            if role == ChannelRole::Admin || role == ChannelRole::Guest {
                let picker = picker.clone();
                let label = if role == ChannelRole::Guest {
                    "Promote to Member"
                } else {
                    "Demote to Member"
                };

                menu = menu.entry(label, None, move |window, cx| {
                    picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .set_user_role(user_id, ChannelRole::Member, window, cx);
                    })
                });
            }

            if role == ChannelRole::Member || role == ChannelRole::Guest {
                let picker = picker.clone();
                menu = menu.entry("Promote to Admin", None, move |window, cx| {
                    picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .set_user_role(user_id, ChannelRole::Admin, window, cx);
                    })
                });
            };

            menu = menu.separator();
            menu = menu.entry("Remove from Channel", None, {
                let picker = picker.clone();
                move |window, cx| {
                    picker.update(cx, |picker, cx| {
                        picker.delegate.remove_member(user_id, window, cx);
                    })
                }
            });
            menu
        });
        window.focus(&context_menu.focus_handle(cx));
        let subscription = cx.subscribe_in(
            &context_menu,
            window,
            |picker, _, _: &DismissEvent, window, cx| {
                picker.delegate.context_menu = None;
                picker.focus(window, cx);
                cx.notify();
            },
        );
        self.context_menu = Some((context_menu, subscription));
    }
}
