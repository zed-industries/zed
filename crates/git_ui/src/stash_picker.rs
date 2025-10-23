use fuzzy::StringMatchCandidate;

use chrono;
use git::stash::StashEntry;
use gpui::{
    Action, AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render,
    SharedString, Styled, Subscription, Task, WeakEntity, Window, actions, rems, svg,
};
use picker::{Picker, PickerDelegate};
use project::git_store::{Repository, RepositoryEvent};
use std::sync::Arc;
use time::{OffsetDateTime, UtcOffset};
use time_format;
use ui::{
    ButtonLike, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing, Tooltip, prelude::*,
};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

use crate::commit_view::CommitView;
use crate::stash_picker;

actions!(
    stash_picker,
    [
        /// Drop the selected stash entry.
        DropStashItem,
        /// Show the diff view of the selected stash entry.
        ShowStashItem,
    ]
);

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::ViewStash,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let repository = workspace.project().read(cx).active_repository(cx);
    let weak_workspace = workspace.weak_handle();
    workspace.toggle_modal(window, cx, |window, cx| {
        StashList::new(repository, weak_workspace, rems(34.), window, cx)
    })
}

pub struct StashList {
    width: Rems,
    pub picker: Entity<Picker<StashListDelegate>>,
    picker_focus_handle: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl StashList {
    fn new(
        repository: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut _subscriptions = Vec::new();
        let stash_request = repository
            .clone()
            .map(|repository| repository.read_with(cx, |repo, _| repo.cached_stash()));

        if let Some(repo) = repository.clone() {
            _subscriptions.push(
                cx.subscribe_in(&repo, window, |this, _, event, window, cx| {
                    if matches!(event, RepositoryEvent::StashEntriesChanged) {
                        let stash_entries = this.picker.read_with(cx, |picker, cx| {
                            picker
                                .delegate
                                .repo
                                .clone()
                                .map(|repo| repo.read(cx).cached_stash().entries.to_vec())
                        });
                        this.picker.update(cx, |this, cx| {
                            this.delegate.all_stash_entries = stash_entries;
                            this.refresh(window, cx);
                        });
                    }
                }),
            )
        }

        cx.spawn_in(window, async move |this, cx| {
            let stash_entries = stash_request
                .map(|git_stash| git_stash.entries.to_vec())
                .unwrap_or_default();

            this.update_in(cx, |this, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.all_stash_entries = Some(stash_entries);
                    picker.refresh(window, cx);
                })
            })?;

            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        let delegate = StashListDelegate::new(repository, workspace, window, cx);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));
        let picker_focus_handle = picker.focus_handle(cx);
        picker.update(cx, |picker, _| {
            picker.delegate.focus_handle = picker_focus_handle.clone();
        });

        _subscriptions.push(cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        }));

        Self {
            picker,
            picker_focus_handle,
            width,
            _subscriptions,
        }
    }

    fn handle_drop_stash(
        &mut self,
        _: &DropStashItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .drop_stash_at(picker.delegate.selected_index(), window, cx);
        });
        cx.notify();
    }

    fn handle_show_stash(
        &mut self,
        _: &ShowStashItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .show_stash_at(picker.delegate.selected_index(), window, cx);
        });
        cx.notify();
    }

    fn handle_modifiers_changed(
        &mut self,
        ev: &ModifiersChangedEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.picker
            .update(cx, |picker, _| picker.delegate.modifiers = ev.modifiers)
    }
}

impl ModalView for StashList {}
impl EventEmitter<DismissEvent> for StashList {}
impl Focusable for StashList {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.picker_focus_handle.clone()
    }
}

impl Render for StashList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("StashList")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .on_action(cx.listener(Self::handle_drop_stash))
            .on_action(cx.listener(Self::handle_show_stash))
            .child(self.picker.clone())
    }
}

#[derive(Debug, Clone)]
struct StashEntryMatch {
    entry: StashEntry,
    positions: Vec<usize>,
    formatted_timestamp: String,
}

pub struct StashListDelegate {
    matches: Vec<StashEntryMatch>,
    all_stash_entries: Option<Vec<StashEntry>>,
    repo: Option<Entity<Repository>>,
    workspace: WeakEntity<Workspace>,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
    focus_handle: FocusHandle,
    timezone: UtcOffset,
}

impl StashListDelegate {
    fn new(
        repo: Option<Entity<Repository>>,
        workspace: WeakEntity<Workspace>,
        _window: &mut Window,
        cx: &mut Context<StashList>,
    ) -> Self {
        let timezone =
            UtcOffset::from_whole_seconds(chrono::Local::now().offset().local_minus_utc())
                .unwrap_or(UtcOffset::UTC);

        Self {
            matches: vec![],
            repo,
            workspace,
            all_stash_entries: None,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
            focus_handle: cx.focus_handle(),
            timezone,
        }
    }

    fn format_message(ix: usize, message: &String) -> String {
        format!("#{}: {}", ix, message)
    }

    fn format_timestamp(timestamp: i64, timezone: UtcOffset) -> String {
        let timestamp =
            OffsetDateTime::from_unix_timestamp(timestamp).unwrap_or(OffsetDateTime::now_utc());
        time_format::format_localized_timestamp(
            timestamp,
            OffsetDateTime::now_utc(),
            timezone,
            time_format::TimestampFormat::EnhancedAbsolute,
        )
    }

    fn drop_stash_at(&self, ix: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry_match) = self.matches.get(ix) else {
            return;
        };
        let stash_index = entry_match.entry.index;
        let Some(repo) = self.repo.clone() else {
            return;
        };

        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, cx| repo.stash_drop(Some(stash_index), cx))?
                .await??;
            Ok(())
        })
        .detach_and_prompt_err("Failed to drop stash", window, cx, |e, _, _| {
            Some(e.to_string())
        });
    }

    fn show_stash_at(&self, ix: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry_match) = self.matches.get(ix) else {
            return;
        };
        let stash_sha = entry_match.entry.oid.to_string();
        let stash_index = entry_match.entry.index;
        let Some(repo) = self.repo.clone() else {
            return;
        };
        CommitView::open(
            stash_sha,
            repo.downgrade(),
            self.workspace.clone(),
            Some(stash_index),
            window,
            cx,
        );
    }

    fn pop_stash(&self, stash_index: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(repo) = self.repo.clone() else {
            return;
        };

        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, cx| repo.stash_pop(Some(stash_index), cx))?
                .await?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to pop stash", window, cx, |e, _, _| {
            Some(e.to_string())
        });
        cx.emit(DismissEvent);
    }

    fn apply_stash(&self, stash_index: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(repo) = self.repo.clone() else {
            return;
        };

        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, cx| repo.stash_apply(Some(stash_index), cx))?
                .await?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to apply stash", window, cx, |e, _, _| {
            Some(e.to_string())
        });
        cx.emit(DismissEvent);
    }
}

impl PickerDelegate for StashListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select a stash…".into()
    }

    fn match_count(&self) -> usize {
        self.matches.len()
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
        let Some(all_stash_entries) = self.all_stash_entries.clone() else {
            return Task::ready(());
        };

        let timezone = self.timezone;

        cx.spawn_in(window, async move |picker, cx| {
            let matches: Vec<StashEntryMatch> = if query.is_empty() {
                all_stash_entries
                    .into_iter()
                    .map(|entry| {
                        let formatted_timestamp = Self::format_timestamp(entry.timestamp, timezone);

                        StashEntryMatch {
                            entry,
                            positions: Vec::new(),
                            formatted_timestamp,
                        }
                    })
                    .collect()
            } else {
                let candidates = all_stash_entries
                    .iter()
                    .enumerate()
                    .map(|(ix, entry)| {
                        StringMatchCandidate::new(
                            ix,
                            &Self::format_message(entry.index, &entry.message),
                        )
                    })
                    .collect::<Vec<StringMatchCandidate>>();
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    false,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|candidate| {
                    let entry = all_stash_entries[candidate.candidate_id].clone();
                    let formatted_timestamp = Self::format_timestamp(entry.timestamp, timezone);

                    StashEntryMatch {
                        entry,
                        positions: candidate.positions,
                        formatted_timestamp,
                    }
                })
                .collect()
            };

            picker
                .update(cx, |picker, _| {
                    let delegate = &mut picker.delegate;
                    delegate.matches = matches;
                    if delegate.matches.is_empty() {
                        delegate.selected_index = 0;
                    } else {
                        delegate.selected_index =
                            core::cmp::min(delegate.selected_index, delegate.matches.len() - 1);
                    }
                    delegate.last_query = query;
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(entry_match) = self.matches.get(self.selected_index()) else {
            return;
        };
        let stash_index = entry_match.entry.index;
        if secondary {
            self.pop_stash(stash_index, window, cx);
        } else {
            self.apply_stash(stash_index, window, cx);
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_match = &self.matches[ix];

        let stash_message =
            Self::format_message(entry_match.entry.index, &entry_match.entry.message);
        let positions = entry_match.positions.clone();
        let stash_label = HighlightedLabel::new(stash_message, positions)
            .truncate()
            .into_any_element();

        let branch_name = entry_match.entry.branch.clone().unwrap_or_default();
        let branch_label = h_flex()
            .gap_1p5()
            .w_full()
            .child(
                h_flex()
                    .gap_0p5()
                    .child(
                        Icon::new(IconName::GitBranch)
                            .color(Color::Muted)
                            .size(IconSize::Small),
                    )
                    .child(
                        Label::new(branch_name)
                            .truncate()
                            .color(Color::Muted)
                            .size(LabelSize::Small),
                    ),
            )
            .child(
                Label::new("•")
                    .alpha(0.5)
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            )
            .child(
                Label::new(entry_match.formatted_timestamp.clone())
                    .color(Color::Muted)
                    .size(LabelSize::Small),
            );

        let show_button = div()
            .group("show-button-hover")
            .child(
                ButtonLike::new("show-button")
                    .child(
                        svg()
                            .size(IconSize::Medium.rems())
                            .flex_none()
                            .path(IconName::Eye.path())
                            .text_color(Color::Default.color(cx))
                            .group_hover("show-button-hover", |this| {
                                this.text_color(Color::Accent.color(cx))
                            })
                            .hover(|this| this.text_color(Color::Accent.color(cx))),
                    )
                    .tooltip(Tooltip::for_action_title("Show Stash", &ShowStashItem))
                    .on_click(cx.listener(move |picker, _, window, cx| {
                        cx.stop_propagation();
                        picker.delegate.show_stash_at(ix, window, cx);
                    })),
            )
            .into_any_element();

        Some(
            ListItem::new(SharedString::from(format!("stash-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .end_slot(show_button)
                .child(
                    v_flex()
                        .w_full()
                        .overflow_hidden()
                        .child(stash_label)
                        .child(branch_label.into_element()),
                )
                .tooltip(Tooltip::text(format!(
                    "stash@{{{}}}",
                    entry_match.entry.index
                ))),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No stashes found".into())
    }

    fn render_footer(&self, _: &mut Window, cx: &mut Context<Picker<Self>>) -> Option<AnyElement> {
        let focus_handle = self.focus_handle.clone();

        Some(
            h_flex()
                .w_full()
                .p_1p5()
                .gap_0p5()
                .justify_end()
                .border_t_1()
                .border_color(cx.theme().colors().border_variant)
                .child(
                    Button::new("apply-stash", "Apply")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::Confirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::Confirm.boxed_clone(), cx)
                        }),
                )
                .child(
                    Button::new("pop-stash", "Pop")
                        .key_binding(
                            KeyBinding::for_action_in(&menu::SecondaryConfirm, &focus_handle, cx)
                                .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(menu::SecondaryConfirm.boxed_clone(), cx)
                        }),
                )
                .child(
                    Button::new("drop-stash", "Drop")
                        .key_binding(
                            KeyBinding::for_action_in(
                                &stash_picker::DropStashItem,
                                &focus_handle,
                                cx,
                            )
                            .map(|kb| kb.size(rems_from_px(12.))),
                        )
                        .on_click(|_, window, cx| {
                            window.dispatch_action(stash_picker::DropStashItem.boxed_clone(), cx)
                        }),
                )
                .into_any(),
        )
    }
}
