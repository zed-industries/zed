use channel::{ChannelMembership, ChannelStore};
use client::{
    proto::{self, ChannelRole, ChannelVisibility},
    ChannelId, User, UserId, UserStore,
};
use fuzzy::{match_strings, StringMatchCandidate};
use gpui::{
    actions, anchored, deferred, div, AppContext, ClipboardItem, DismissEvent, EventEmitter,
    FocusableView, Model, ParentElement, Render, Styled, Subscription, Task, View, ViewContext,
    VisualContext, WeakView,
};
use picker::{Picker, PickerDelegate};
use std::sync::Arc;
use ui::{prelude::*, Avatar, CheckboxWithLabel, ContextMenu, ListItem, ListItemSpacing};
use util::TryFutureExt;
use workspace::{notifications::DetachAndPromptErr, ModalView};

actions!(
    channel_modal,
    [
        SelectNextControl,
        ToggleMode,
        ToggleMemberAdmin,
        RemoveMember
    ]
);

pub struct ChannelModal {
    picker: View<Picker<ChannelModalDelegate>>,
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
}

impl ChannelModal {
    pub fn new(
        user_store: Model<UserStore>,
        channel_store: Model<ChannelStore>,
        channel_id: ChannelId,
        mode: Mode,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        cx.observe(&channel_store, |_, _, cx| cx.notify()).detach();
        let channel_modal = cx.view().downgrade();
        let picker = cx.new_view(|cx| {
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

    fn toggle_mode(&mut self, _: &ToggleMode, cx: &mut ViewContext<Self>) {
        let mode = match self.picker.read(cx).delegate.mode {
            Mode::ManageMembers => Mode::InviteMembers,
            Mode::InviteMembers => Mode::ManageMembers,
        };
        self.set_mode(mode, cx);
    }

    fn set_mode(&mut self, mode: Mode, cx: &mut ViewContext<Self>) {
        self.picker.update(cx, |picker, cx| {
            let delegate = &mut picker.delegate;
            delegate.mode = mode;
            delegate.selected_index = 0;
            picker.set_query("", cx);
            picker.update_matches(picker.query(cx), cx);
            cx.notify()
        });
        cx.notify()
    }

    fn set_channel_visibility(&mut self, selection: &Selection, cx: &mut ViewContext<Self>) {
        self.channel_store.update(cx, |channel_store, cx| {
            channel_store
                .set_channel_visibility(
                    self.channel_id,
                    match selection {
                        Selection::Unselected => ChannelVisibility::Members,
                        Selection::Selected => ChannelVisibility::Public,
                        Selection::Indeterminate => return,
                    },
                    cx,
                )
                .detach_and_log_err(cx)
        });
    }

    fn dismiss(&mut self, _: &menu::Cancel, cx: &mut ViewContext<Self>) {
        cx.emit(DismissEvent);
    }
}

impl EventEmitter<DismissEvent> for ChannelModal {}
impl ModalView for ChannelModal {}

impl FocusableView for ChannelModal {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for ChannelModal {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
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
                                    ui::Selection::Selected
                                } else {
                                    ui::Selection::Unselected
                                },
                                cx.listener(Self::set_channel_visibility),
                            ))
                            .children(
                                Some(
                                    Button::new("copy-link", "Copy Link")
                                        .label_size(LabelSize::Small)
                                        .on_click(cx.listener(move |this, _, cx| {
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
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.set_mode(Mode::ManageMembers, cx);
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
                                    .on_click(cx.listener(|this, _, cx| {
                                        this.set_mode(Mode::InviteMembers, cx);
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
    channel_modal: WeakView<ChannelModal>,
    matching_users: Vec<Arc<User>>,
    matching_member_indices: Vec<usize>,
    user_store: Model<UserStore>,
    channel_store: Model<ChannelStore>,
    channel_id: ChannelId,
    selected_index: usize,
    mode: Mode,
    match_candidates: Vec<StringMatchCandidate>,
    members: Vec<ChannelMembership>,
    has_all_members: bool,
    context_menu: Option<(View<ContextMenu>, Subscription)>,
}

impl PickerDelegate for ChannelModalDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _cx: &mut WindowContext) -> Arc<str> {
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

    fn set_selected_index(&mut self, ix: usize, _: &mut ViewContext<Picker<Self>>) {
        self.selected_index = ix;
    }

    fn update_matches(&mut self, query: String, cx: &mut ViewContext<Picker<Self>>) -> Task<()> {
        match self.mode {
            Mode::ManageMembers => {
                if self.has_all_members {
                    self.match_candidates.clear();
                    self.match_candidates
                        .extend(self.members.iter().enumerate().map(|(id, member)| {
                            StringMatchCandidate {
                                id,
                                string: member.user.github_login.clone(),
                                char_bag: member.user.github_login.chars().collect(),
                            }
                        }));

                    let matches = cx.background_executor().block(match_strings(
                        &self.match_candidates,
                        &query,
                        true,
                        usize::MAX,
                        &Default::default(),
                        cx.background_executor().clone(),
                    ));

                    cx.spawn(|picker, mut cx| async move {
                        picker
                            .update(&mut cx, |picker, cx| {
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
                    cx.spawn(|picker, mut cx| async move {
                        async {
                            let members = search_members.await?;
                            picker.update(&mut cx, |picker, cx| {
                                picker.delegate.has_all_members =
                                    query == "" && members.len() < 100;
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
                cx.spawn(|picker, mut cx| async move {
                    async {
                        let users = search_users.await?;
                        picker.update(&mut cx, |picker, cx| {
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

    fn confirm(&mut self, _: bool, cx: &mut ViewContext<Picker<Self>>) {
        if let Some(selected_user) = self.user_at_index(self.selected_index) {
            if Some(selected_user.id) == self.user_store.read(cx).current_user().map(|user| user.id)
            {
                return;
            }
            match self.mode {
                Mode::ManageMembers => self.show_context_menu(self.selected_index, cx),
                Mode::InviteMembers => match self.member_status(selected_user.id, cx) {
                    Some(proto::channel_member::Kind::Invitee) => {
                        self.remove_member(selected_user.id, cx);
                    }
                    Some(proto::channel_member::Kind::Member) => {}
                    None => self.invite_member(selected_user, cx),
                },
            }
        }
    }

    fn dismissed(&mut self, cx: &mut ViewContext<Picker<Self>>) {
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
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let user = self.user_at_index(ix)?;
        let membership = self.member_at_index(ix);
        let request_status = self.member_status(user.id, cx);
        let is_me = self.user_store.read(cx).current_user().map(|user| user.id) == Some(user.id);

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .selected(selected)
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
                                                .anchor(gpui::AnchorCorner::TopRight)
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
    fn member_status(
        &self,
        user_id: UserId,
        cx: &AppContext,
    ) -> Option<proto::channel_member::Kind> {
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
        cx: &mut ViewContext<Picker<Self>>,
    ) -> Option<()> {
        let update = self.channel_store.update(cx, |store, cx| {
            store.set_member_role(self.channel_id, user_id, new_role, cx)
        });
        cx.spawn(|picker, mut cx| async move {
            update.await?;
            picker.update(&mut cx, |picker, cx| {
                let this = &mut picker.delegate;
                if let Some(member) = this.members.iter_mut().find(|m| m.user.id == user_id) {
                    member.role = new_role;
                }
                cx.focus_self();
                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to update role", cx, |_, _| None);
        Some(())
    }

    fn remove_member(&mut self, user_id: UserId, cx: &mut ViewContext<Picker<Self>>) -> Option<()> {
        let update = self.channel_store.update(cx, |store, cx| {
            store.remove_member(self.channel_id, user_id, cx)
        });
        cx.spawn(|picker, mut cx| async move {
            update.await?;
            picker.update(&mut cx, |picker, cx| {
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

                picker.focus(cx);
                cx.notify();
            })
        })
        .detach_and_prompt_err("Failed to remove member", cx, |_, _| None);
        Some(())
    }

    fn invite_member(&mut self, user: Arc<User>, cx: &mut ViewContext<Picker<Self>>) {
        let invite_member = self.channel_store.update(cx, |store, cx| {
            store.invite_member(self.channel_id, user.id, ChannelRole::Member, cx)
        });

        cx.spawn(|this, mut cx| async move {
            invite_member.await?;

            this.update(&mut cx, |this, cx| {
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
        .detach_and_prompt_err("Failed to invite member", cx, |_, _| None);
    }

    fn show_context_menu(&mut self, ix: usize, cx: &mut ViewContext<Picker<Self>>) {
        let Some(membership) = self.member_at_index(ix) else {
            return;
        };
        let user_id = membership.user.id;
        let picker = cx.view().clone();
        let context_menu = ContextMenu::build(cx, |mut menu, _cx| {
            let role = membership.role;

            if role == ChannelRole::Admin || role == ChannelRole::Member {
                let picker = picker.clone();
                menu = menu.entry("Demote to Guest", None, move |cx| {
                    picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .set_user_role(user_id, ChannelRole::Guest, cx);
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

                menu = menu.entry(label, None, move |cx| {
                    picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .set_user_role(user_id, ChannelRole::Member, cx);
                    })
                });
            }

            if role == ChannelRole::Member || role == ChannelRole::Guest {
                let picker = picker.clone();
                menu = menu.entry("Promote to Admin", None, move |cx| {
                    picker.update(cx, |picker, cx| {
                        picker
                            .delegate
                            .set_user_role(user_id, ChannelRole::Admin, cx);
                    })
                });
            };

            menu = menu.separator();
            menu = menu.entry("Remove from Channel", None, {
                let picker = picker.clone();
                move |cx| {
                    picker.update(cx, |picker, cx| {
                        picker.delegate.remove_member(user_id, cx);
                    })
                }
            });
            menu
        });
        cx.focus_view(&context_menu);
        let subscription = cx.subscribe(&context_menu, |picker, _, _: &DismissEvent, cx| {
            picker.delegate.context_menu = None;
            picker.focus(cx);
            cx.notify();
        });
        self.context_menu = Some((context_menu, subscription));
    }
}
