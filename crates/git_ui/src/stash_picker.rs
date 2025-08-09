use fuzzy::StringMatchCandidate;

use git::stash::StashEntry;
use gpui::{
    App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable, InteractiveElement,
    IntoElement, Modifiers, ModifiersChangedEvent, ParentElement, Render, SharedString, Styled,
    Subscription, Task, Window, rems,
};
use picker::{Picker, PickerDelegate, PickerEditorPosition};
use project::git_store::Repository;
use std::sync::Arc;
use ui::{HighlightedLabel, ListItem, ListItemSpacing, prelude::*};
use util::ResultExt;
use workspace::notifications::DetachAndPromptErr;
use workspace::{ModalView, Workspace};

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(open);
}

pub fn open(
    workspace: &mut Workspace,
    _: &zed_actions::git::Stash,
    window: &mut Window,
    cx: &mut Context<Workspace>,
) {
    let repository = workspace.project().read(cx).active_repository(cx).clone();
    let style = StashListStyle::Modal;
    workspace.toggle_modal(window, cx, |window, cx| {
        StashList::new(repository, style, rems(34.), window, cx)
    })
}

pub fn popover(
    repository: Option<Entity<Repository>>,
    window: &mut Window,
    cx: &mut App,
) -> Entity<StashList> {
    cx.new(|cx| {
        let list = StashList::new(repository, StashListStyle::Popover, rems(20.), window, cx);
        list.focus_handle(cx).focus(window);
        list
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StashListStyle {
    Modal,
    Popover,
}

pub struct StashList {
    width: Rems,
    pub picker: Entity<Picker<StashListDelegate>>,
    _subscription: Subscription,
}

impl StashList {
    fn new(
        repository: Option<Entity<Repository>>,
        style: StashListStyle,
        width: Rems,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let stash_request = repository
            .clone()
            .map(|repository| repository.read_with(cx, |repo, _| repo.stash_entries.clone()));

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

        let delegate = StashListDelegate::new(repository.clone(), style);
        let picker = cx.new(|cx| Picker::uniform_list(delegate, window, cx));

        let _subscription = cx.subscribe(&picker, |_, _, _, cx| {
            cx.emit(DismissEvent);
        });

        Self {
            picker,
            width,
            _subscription,
        }
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
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for StashList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .key_context("GitStashSelector")
            .w(self.width)
            .on_modifiers_changed(cx.listener(Self::handle_modifiers_changed))
            .child(self.picker.clone())
            .on_mouse_down_out({
                cx.listener(move |this, _, window, cx| {
                    this.picker.update(cx, |this, cx| {
                        this.cancel(&Default::default(), window, cx);
                    })
                })
            })
    }
}

#[derive(Debug, Clone)]
struct StashEntryMatch {
    entry: StashEntry,
    positions: Vec<usize>,
}

pub struct StashListDelegate {
    matches: Vec<StashEntryMatch>,
    all_stash_entries: Option<Vec<StashEntry>>,
    repo: Option<Entity<Repository>>,
    style: StashListStyle,
    selected_index: usize,
    last_query: String,
    modifiers: Modifiers,
}

impl StashListDelegate {
    fn new(repo: Option<Entity<Repository>>, style: StashListStyle) -> Self {
        Self {
            matches: vec![],
            repo,
            style,
            all_stash_entries: None,
            selected_index: 0,
            last_query: Default::default(),
            modifiers: Default::default(),
        }
    }

    fn drop_stash(&self, stash_index: usize, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(repo) = self.repo.clone() else {
            return;
        };

        cx.spawn(async move |_, cx| {
            repo.update(cx, |repo, cx| repo.stash_drop(Some(stash_index), cx))?
                .await?;
            Ok(())
        })
        .detach_and_prompt_err("Failed to apply stash", window, cx, |e, _, _| {
            Some(e.to_string())
        });
        cx.emit(DismissEvent);
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
}

impl PickerDelegate for StashListDelegate {
    type ListItem = ListItem;

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        "Select stash&".into()
    }

    fn editor_position(&self) -> PickerEditorPosition {
        match self.style {
            StashListStyle::Modal => PickerEditorPosition::Start,
            StashListStyle::Popover => PickerEditorPosition::End,
        }
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

        cx.spawn_in(window, async move |picker, cx| {
            let matches: Vec<StashEntryMatch> = if query.is_empty() {
                all_stash_entries
                    .into_iter()
                    .map(|entry| StashEntryMatch {
                        entry,
                        positions: Vec::new(),
                    })
                    .collect()
            } else {
                let candidates = all_stash_entries
                    .iter()
                    .enumerate()
                    .map(|(ix, entry)| StringMatchCandidate::new(ix, &entry.message))
                    .collect::<Vec<StringMatchCandidate>>();
                fuzzy::match_strings(
                    &candidates,
                    &query,
                    true,
                    true,
                    10000,
                    &Default::default(),
                    cx.background_executor().clone(),
                )
                .await
                .into_iter()
                .map(|candidate| StashEntryMatch {
                    entry: all_stash_entries[candidate.candidate_id].clone(),
                    positions: candidate.positions,
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
            // Secondary action: remove stash (remove from stash list)
            self.drop_stash(stash_index, window, cx);
        } else {
            // Primary action: apply stash (keep in stash list)
            self.pop_stash(stash_index, window, cx);
        }
    }

    fn dismissed(&mut self, _: &mut Window, cx: &mut Context<Picker<Self>>) {
        cx.emit(DismissEvent);
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _: &mut Window,
        _: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let entry_match = &self.matches[ix];

        let stash_name = HighlightedLabel::new(
            entry_match.entry.message.clone(),
            entry_match.positions.clone(),
        )
        .truncate()
        .into_any_element();

        let stash_index_label = Label::new(format!("stash@{{{}}}", entry_match.entry.index))
            .size(LabelSize::Small)
            .color(Color::Muted);

        Some(
            ListItem::new(SharedString::from(format!("stash-{ix}")))
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .child(
                    v_flex()
                        .w_full()
                        .overflow_hidden()
                        .child(
                            h_flex()
                                .gap_6()
                                .justify_between()
                                .overflow_x_hidden()
                                .child(stash_name)
                                .child(stash_index_label.into_element()),
                        )
                        .when(self.style == StashListStyle::Modal, |el| {
                            el.child(div().max_w_96())
                        }),
                ),
        )
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        Some("No stashes found".into())
    }
}
