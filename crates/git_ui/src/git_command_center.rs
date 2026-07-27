//! A single hierarchical palette for git operations.
//!
//! Rows are either verbs, which run and dismiss, or categories, which open
//! another page without leaving the palette. Navigation state lives in
//! [`picker::tree::Stack`]; everything git-specific — which nodes exist, how
//! they're ranked, how they render — lives here.

use std::rc::Rc;
use std::sync::Arc;

use fuzzy_nucleo::StringMatchCandidate;
use git::repository::Branch;
use gpui::{
    AnyElement, App, Context, DismissEvent, Entity, EventEmitter, FocusHandle, Focusable,
    InteractiveElement, IntoElement, KeyContext, MouseButton, ParentElement, Render, SharedString,
    Styled, Subscription, Task, WeakEntity, Window, actions, rems,
};
use picker::tree::{Activate, Children, Node, Stack};
use picker::{Picker, PickerDelegate};
use project::git_store::{Repository, RepositoryEvent};
use ui::{
    CommonAnimationExt, Divider, HighlightedLabel, KeyBinding, ListItem, ListItemSpacing,
    ListSubHeader, Tooltip, prelude::*,
};
use util::ResultExt;
use workspace::{ModalView, Workspace};

use crate::branch_picker::normalize_branch_name;
use crate::git_panel::show_error_toast;
use crate::project_diff::DeployBranchDiff;

actions!(
    git_command_center,
    [
        /// Returns to the previous page of the git command center.
        NavigateBack,
        /// Opens the selected row's page in the git command center.
        NavigateForward,
    ]
);

/// Fuzzy matches are capped to keep a keystroke bounded on a repository with a
/// pathological number of refs. Matching this many already fills far more than
/// the user will scroll.
const MAX_MATCHES: usize = 10_000;

/// How many branches the "Recent Branches" page shows. Recency stops being
/// meaningful well before this.
const RECENT_BRANCH_COUNT: usize = 20;

struct Match {
    /// Index into the current page's nodes.
    node_index: usize,
    positions: Vec<usize>,
}

pub struct GitCommandCenter {
    picker: Entity<Picker<GitCommandCenterDelegate>>,
    _subscriptions: Vec<Subscription>,
}

impl GitCommandCenter {
    fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let previous_focus_handle = window.focused(cx).unwrap_or_else(|| cx.focus_handle());
        let delegate = GitCommandCenterDelegate::new(
            workspace,
            repository.clone(),
            previous_focus_handle,
            cx,
        );

        let picker = cx.new(|cx| {
            Picker::uniform_list(delegate, window, cx)
                .initial_width(rems(36.))
                .show_scrollbar(true)
                .embedded()
        });

        // The search field is a full editor in auto-height mode, which gives
        // auto-grow, IME, undo/redo, selections and multiline paste. Plain
        // `enter` is unbound in that mode, so it still reaches `menu::Confirm`
        // while `shift-enter` inserts a newline.
        picker.update(cx, |picker, cx| {
            picker.set_multiline(Some(4), window, cx);
            picker.delegate.focus_handle = picker.focus_handle(cx);
        });

        let mut subscriptions = vec![
            cx.subscribe(&picker, |_, _, _: &DismissEvent, cx| {
                cx.emit(DismissEvent);
            }),
            // The breadcrumbs and the `empty_query` key context are both derived
            // from delegate state, which notifies the picker rather than this
            // wrapper. Without this they would render one navigation behind.
            cx.observe(&picker, |_, _, cx| cx.notify()),
        ];

        // The root page is derived from repository state, so rebuild it when
        // that state moves underneath us. Deeper pages keep the data they
        // captured on descent; they re-resolve when reopened.
        if let Some(repository) = &repository {
            subscriptions.push(cx.subscribe_in(
                repository,
                window,
                |this, _, event, window, cx| {
                    if matches!(
                        event,
                        RepositoryEvent::BranchListChanged
                            | RepositoryEvent::HeadChanged
                            | RepositoryEvent::StashEntriesChanged
                    ) {
                        this.picker.update(cx, |picker, cx| {
                            picker.delegate.rebuild_root(cx);
                            if picker.delegate.stack.at_root() {
                                picker.refresh(window, cx);
                            }
                        });
                    }
                },
            ));
        }

        Self {
            picker,
            _subscriptions: subscriptions,
        }
    }

    fn navigate_back(&mut self, window: &mut Window, cx: &mut Context<Self>) -> bool {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .show_current_page(|stack| stack.pop(), window, cx)
        })
    }

    fn navigate_to_depth(&mut self, depth: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.picker.update(cx, |picker, cx| {
            picker
                .delegate
                .show_current_page(|stack| stack.truncate(depth), window, cx)
        });
    }

    fn render_breadcrumbs(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let delegate = &self.picker.read(cx).delegate;
        let titles: Vec<SharedString> = delegate.stack.titles().cloned().collect();
        let depth = delegate.stack.depth();
        let loading = delegate.pending_children.is_some();
        let last_index = titles.len().saturating_sub(1);

        h_flex()
            .w_full()
            .px_2()
            .pt_1p5()
            .pb_1()
            .gap_0p5()
            .flex_wrap()
            .children(titles.into_iter().enumerate().map(|(index, title)| {
                let is_last = index == last_index;
                h_flex()
                    .gap_0p5()
                    .when(index > 0, |this| {
                        this.child(
                            Icon::new(IconName::ChevronRight)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .child(if is_last {
                        Label::new(title)
                            .size(LabelSize::Small)
                            .single_line()
                            .into_any_element()
                    } else {
                        Button::new(("breadcrumb", index), title)
                            .label_size(LabelSize::Small)
                            .color(Color::Muted)
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.navigate_to_depth(index, window, cx);
                            }))
                            .into_any_element()
                    })
                    .into_any_element()
            }))
            .when(loading, |this| {
                this.child(div().flex_1()).child(
                    Icon::new(IconName::ArrowCircle)
                        .size(IconSize::XSmall)
                        .color(Color::Accent)
                        .with_rotate_animation(2),
                )
            })
            .when(depth > 0, |this| {
                let focus_handle = self.picker.focus_handle(cx);
                this.child(div().flex_1()).child(
                    IconButton::new("git-command-center-back", IconName::ArrowLeft)
                        .icon_size(IconSize::XSmall)
                        .tooltip(move |_, cx| {
                            Tooltip::for_action_in("Go Back", &NavigateBack, &focus_handle, cx)
                        })
                        .on_click(cx.listener(|this, _, window, cx| {
                            this.navigate_back(window, cx);
                        })),
                )
            })
    }
}

impl ModalView for GitCommandCenter {}
impl EventEmitter<DismissEvent> for GitCommandCenter {}

impl Focusable for GitCommandCenter {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.picker.focus_handle(cx)
    }
}

impl Render for GitCommandCenter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let query_is_empty = self.picker.read(cx).delegate.last_query.is_empty();

        v_flex()
            .occlude()
            .w(rems(36.))
            .elevation_3(cx)
            .overflow_hidden()
            .key_context({
                let mut key_context = KeyContext::new_with_defaults();
                key_context.add("Pane");
                key_context.add("GitCommandCenter");
                // Backspace and Left only navigate when there is nothing to
                // delete or move over, so the search field keeps its normal
                // editing behavior while the user is typing.
                if query_is_empty {
                    key_context.add("empty_query");
                }
                key_context
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            })
            .on_action(cx.listener(|this, _: &menu::Cancel, window, cx| {
                if !this.navigate_back(window, cx) {
                    cx.emit(DismissEvent);
                }
            }))
            .on_action(cx.listener(|this, _: &NavigateBack, window, cx| {
                this.navigate_back(window, cx);
            }))
            .on_action(cx.listener(|this, _: &NavigateForward, window, cx| {
                this.picker.update(cx, |picker, cx| {
                    picker.delegate.descend_into_selected(window, cx);
                });
            }))
            .child(self.render_breadcrumbs(cx))
            .child(Divider::horizontal())
            .child(self.picker.clone())
    }
}

pub struct GitCommandCenterDelegate {
    workspace: WeakEntity<Workspace>,
    repository: Option<Entity<Repository>>,
    previous_focus_handle: FocusHandle,
    focus_handle: FocusHandle,
    stack: Stack,
    matches: Vec<Match>,
    selected_index: usize,
    last_query: String,
    /// The row to land on once matches are next rebuilt. Set by navigation so
    /// returning to a page restores where the user was, rather than snapping to
    /// the top like a fresh search would.
    restore_selection: Option<usize>,
    /// Set while a deferred page is resolving. Dropping it cancels the load,
    /// which is what should happen if the user navigates away first.
    pending_children: Option<Task<()>>,
}

impl GitCommandCenterDelegate {
    fn new(
        workspace: WeakEntity<Workspace>,
        repository: Option<Entity<Repository>>,
        previous_focus_handle: FocusHandle,
        cx: &mut App,
    ) -> Self {
        let root = root_nodes(&workspace, repository.as_ref(), cx);
        Self {
            workspace,
            repository,
            previous_focus_handle,
            focus_handle: cx.focus_handle(),
            stack: Stack::new("Git", root),
            matches: Vec::new(),
            selected_index: 0,
            last_query: String::new(),
            restore_selection: None,
            pending_children: None,
        }
    }

    fn rebuild_root(&mut self, cx: &mut App) {
        let root = root_nodes(&self.workspace, self.repository.as_ref(), cx);
        self.stack.set_root_nodes(root);
    }

    fn selected_node(&self) -> Option<&Node> {
        let node_index = self.matches.get(self.selected_index)?.node_index;
        self.stack.nodes().get(node_index)
    }

    /// Moves the selection onto a row that can actually be selected, so a
    /// section heading is never left highlighted.
    fn snap_selection(&mut self) {
        let nodes = self.stack.nodes();
        let selectable = |index: usize| {
            self.matches
                .get(index)
                .and_then(|m| nodes.get(m.node_index))
                .is_some_and(|node| node.is_selectable())
        };
        if selectable(self.selected_index) {
            return;
        }
        let next = (self.selected_index..self.matches.len()).find(|ix| selectable(*ix));
        let previous = || (0..self.selected_index).rev().find(|ix| selectable(*ix));
        self.selected_index = next.or_else(previous).unwrap_or(0);
    }

    fn descend_into_selected(&mut self, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        let Some(node) = self.selected_node() else {
            return;
        };
        let Some(children) = node.child_page().cloned() else {
            return;
        };
        let title = node.label.clone();
        self.open_page(title, children, window, cx);
    }

    fn open_page(
        &mut self,
        title: SharedString,
        children: Children,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        match children {
            Children::Ready(nodes) => self.push_page(title, nodes.to_vec(), window, cx),
            Children::Deferred(resolve) => {
                let task = resolve(cx);
                self.pending_children = Some(cx.spawn_in(window, async move |picker, cx| {
                    let nodes = task.await;
                    picker
                        .update_in(cx, |picker, window, cx| {
                            picker.delegate.pending_children = None;
                            match nodes {
                                Ok(nodes) => {
                                    picker.delegate.push_page(title, nodes, window, cx);
                                }
                                Err(error) => {
                                    picker.delegate.report("load git data", error, cx);
                                }
                            }
                        })
                        .log_err();
                }));
                cx.notify();
            }
        }
    }

    fn push_page(
        &mut self,
        title: SharedString,
        nodes: Vec<Node>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) {
        self.stack
            .push(title, nodes, self.last_query.clone(), self.selected_index);
        self.show_current_page(|_| Some(String::new()), window, cx);
    }

    /// Applies a stack move and re-syncs the search field and matches to the
    /// page that is now current.
    ///
    /// `update_matches` has to be called explicitly rather than left to
    /// `set_query`: when the restored query is identical to what's already in
    /// the field the editor emits no edit event, and the matches would keep
    /// pointing into the page we just left.
    fn show_current_page(
        &mut self,
        move_stack: impl FnOnce(&mut Stack) -> Option<String>,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> bool {
        let Some(query) = move_stack(&mut self.stack) else {
            return false;
        };
        self.pending_children = None;
        self.restore_selection = Some(self.stack.current().selected);
        cx.defer_in(window, move |picker, window, cx| {
            picker.set_query(&query, window, cx);
            picker.update_matches(query, window, cx);
            cx.notify();
        });
        true
    }

    fn report(&self, action: &'static str, error: anyhow::Error, cx: &mut App) {
        if let Some(workspace) = self.workspace.upgrade() {
            show_error_toast(workspace, action, error, cx);
        } else {
            log::error!("failed to {action}: {error}");
        }
    }
}

impl PickerDelegate for GitCommandCenterDelegate {
    type ListItem = AnyElement;

    fn name() -> &'static str {
        "git command center"
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
        cx: &mut Context<Picker<Self>>,
    ) {
        self.selected_index = ix;
        self.stack.current_mut().selected = ix;
        cx.notify();
    }

    fn can_select(
        &self,
        ix: usize,
        _window: &mut Window,
        _cx: &mut Context<Picker<Self>>,
    ) -> bool {
        self.matches
            .get(ix)
            .and_then(|m| self.stack.nodes().get(m.node_index))
            .is_some_and(|node| node.is_selectable())
    }

    fn separators_after_indices(&self) -> Vec<usize> {
        let nodes = self.stack.nodes();
        self.matches
            .iter()
            .enumerate()
            .skip(1)
            .filter(|(_, m)| nodes.get(m.node_index).is_some_and(Node::is_section))
            .map(|(ix, _)| ix - 1)
            .collect()
    }

    fn placeholder_text(&self, _window: &mut Window, _cx: &mut App) -> Arc<str> {
        if self.stack.at_root() {
            "Search git actions, branches, stashes…".into()
        } else {
            format!("Search {}…", self.stack.current().title).into()
        }
    }

    fn no_matches_text(&self, _window: &mut Window, _cx: &mut App) -> Option<SharedString> {
        if self.pending_children.is_some() {
            Some("Loading…".into())
        } else {
            Some("No matches".into())
        }
    }

    fn update_matches(
        &mut self,
        query: String,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Task<()> {
        self.last_query = query.clone();
        let nodes = Rc::clone(&self.stack.current().nodes);

        if query.is_empty() {
            self.matches = (0..nodes.len())
                .map(|node_index| Match {
                    node_index,
                    positions: Vec::new(),
                })
                .collect();
            self.selected_index = self.restore_selection.take().unwrap_or(0);
            self.snap_selection();
            cx.notify();
            return Task::ready(());
        }

        // Candidates are plain strings, so matching runs off the main thread
        // even though `Node` itself is not `Send`. A node with keywords gets a
        // second candidate; the best-scoring one per node wins, which keeps
        // highlight positions pointing at the label whenever the label matched.
        let mut candidates = Vec::with_capacity(nodes.len());
        let mut keyword_candidates = Vec::new();
        for (node_index, node) in nodes.iter().enumerate() {
            if node.is_section() {
                continue;
            }
            candidates.push(StringMatchCandidate::from_shared(
                node_index,
                node.label.clone(),
            ));
            if let Some(keywords) = &node.keywords {
                keyword_candidates
                    .push(StringMatchCandidate::from_shared(node_index, keywords.clone()));
            }
        }

        cx.spawn_in(window, async move |picker, cx| {
            let executor = cx.background_executor().clone();
            let label_matches = fuzzy_nucleo::match_strings_async(
                &candidates,
                &query,
                fuzzy_nucleo::Case::Smart,
                fuzzy_nucleo::LengthPenalty::On,
                MAX_MATCHES,
                &Default::default(),
                executor.clone(),
            )
            .await;
            let keyword_matches = if keyword_candidates.is_empty() {
                Vec::new()
            } else {
                fuzzy_nucleo::match_strings_async(
                    &keyword_candidates,
                    &query,
                    fuzzy_nucleo::Case::Smart,
                    fuzzy_nucleo::LengthPenalty::On,
                    MAX_MATCHES,
                    &Default::default(),
                    executor,
                )
                .await
            };

            let mut scored: Vec<(usize, f64, Vec<usize>)> = label_matches
                .into_iter()
                .map(|m| (m.candidate_id, m.score, m.positions))
                .collect();
            for m in keyword_matches {
                if !scored.iter().any(|(id, _, _)| *id == m.candidate_id) {
                    // A keyword-only hit has no positions in the label, so it
                    // renders unhighlighted rather than highlighting nonsense.
                    scored.push((m.candidate_id, m.score, Vec::new()));
                }
            }
            scored.sort_by(|a, b| b.1.total_cmp(&a.1).then(a.0.cmp(&b.0)));

            picker
                .update(cx, |picker, cx| {
                    picker.delegate.matches = scored
                        .into_iter()
                        .map(|(node_index, _, positions)| Match {
                            node_index,
                            positions,
                        })
                        .collect();
                    picker.delegate.selected_index =
                        picker.delegate.restore_selection.take().unwrap_or(0);
                    picker.delegate.snap_selection();
                    cx.notify();
                })
                .log_err();
        })
    }

    fn confirm(&mut self, secondary: bool, window: &mut Window, cx: &mut Context<Picker<Self>>) {
        // Cmd-Enter opens the row's verbs; it works at any depth and needs no
        // extra keybinding, since `menu::SecondaryConfirm` is already bound.
        if secondary {
            self.descend_into_selected(window, cx);
            return;
        }

        let Some(node) = self.selected_node() else {
            return;
        };

        match node.activate.clone() {
            Activate::Section => {}
            Activate::Page(children) => {
                let title = node.label.clone();
                self.open_page(title, children, window, cx);
            }
            Activate::Action(action) => {
                window.focus(&self.previous_focus_handle, cx);
                cx.emit(DismissEvent);
                window.dispatch_action(action, cx);
            }
            Activate::Run(run) => {
                let query = self.last_query.clone();
                cx.emit(DismissEvent);
                run(&query, window, cx);
            }
        }
    }

    fn dismissed(&mut self, _window: &mut Window, cx: &mut Context<Picker<Self>>) {
        self.pending_children = None;
        cx.emit(DismissEvent);
    }

    /// `menu::SelectParent` and `menu::SelectChild` step through the hierarchy.
    /// Both drive [`Self::show_current_page`] and return `None`, because handing
    /// the picker a query back would skip the explicit re-match that a stack
    /// move needs.
    fn select_parent(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        self.show_current_page(|stack| stack.pop(), window, cx);
        None
    }

    fn select_child(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<String> {
        self.descend_into_selected(window, cx);
        None
    }

    fn render_match(
        &self,
        ix: usize,
        selected: bool,
        _window: &mut Window,
        cx: &mut Context<Picker<Self>>,
    ) -> Option<Self::ListItem> {
        let matched = self.matches.get(ix)?;
        let node = self.stack.nodes().get(matched.node_index)?;

        if node.is_section() {
            return Some(
                ListSubHeader::new(node.label.clone())
                    .inset(true)
                    .into_any_element(),
            );
        }

        let disabled = node.disabled_reason.is_some();
        let detail = node
            .disabled_reason
            .clone()
            .or_else(|| node.detail.clone());

        let label = h_flex()
            .w_full()
            .min_w_0()
            .gap_2p5()
            .when_some(node.icon, |this, icon| {
                this.child(
                    Icon::new(icon)
                        .size(IconSize::Small)
                        .color(if disabled {
                            Color::Disabled
                        } else {
                            node.icon_color.unwrap_or(Color::Muted)
                        }),
                )
            })
            .child(
                v_flex()
                    .w_full()
                    .min_w_0()
                    .child(
                        HighlightedLabel::new(node.label.clone(), matched.positions.clone())
                            .when(disabled, |this| this.color(Color::Disabled))
                            .single_line()
                            .truncate(),
                    )
                    .when_some(detail, |this, detail| {
                        this.child(
                            Label::new(detail)
                                .size(LabelSize::Small)
                                .color(Color::Muted)
                                .single_line()
                                .truncate(),
                        )
                    }),
            );

        let trailing = h_flex()
            .gap_1p5()
            .flex_none()
            .when_some(node.trailing.clone(), |this, trailing| {
                this.child(
                    Label::new(trailing)
                        .size(LabelSize::Small)
                        .color(Color::Muted),
                )
            })
            .map(|this| match &node.activate {
                Activate::Action(action) => this.child(
                    KeyBinding::for_action_in(&**action, &self.previous_focus_handle, cx)
                        .size(rems_from_px(12.)),
                ),
                _ => this,
            })
            .when(node.has_child_page(), |this| {
                this.child(
                    Icon::new(IconName::ChevronRight)
                        .size(IconSize::XSmall)
                        .color(Color::Muted),
                )
            });

        let has_child_page = node.has_child_page();
        let keyshortcuts: SharedString = if has_child_page {
            "Right or Cmd-Enter opens".into()
        } else {
            "Enter runs".into()
        };

        Some(
            ListItem::new(ix)
                .inset(true)
                .spacing(ListItemSpacing::Sparse)
                .toggle_state(selected)
                .disabled(disabled)
                // Reported to assistive technology as a listbox option. The
                // picker's editor keeps real keyboard focus, so the selected
                // row is announced as the active descendant.
                .aria_role(gpui::Role::ListBoxOption)
                .aria_label(node.label.clone())
                .aria_keyshortcuts(keyshortcuts)
                .when(selected, |this| this.aria_active_descendant())
                .child(h_flex().w_full().min_w_0().gap_2().child(label).child(trailing))
                // Clicking a row runs it, so a row that also has verbs needs its
                // own hover target to reach them — the mouse equivalent of Right.
                .when(has_child_page && !disabled, |this| {
                    this.end_slot_on_hover(
                        IconButton::new(("open-verbs", ix), IconName::Ellipsis)
                            .icon_size(IconSize::Small)
                            .tooltip(Tooltip::text("Open Actions"))
                            .on_click(cx.listener(move |picker, _, window, cx| {
                                picker.delegate.selected_index = ix;
                                picker.delegate.descend_into_selected(window, cx);
                            })),
                    )
                })
                .into_any_element(),
        )
    }
}

// -- Git node tree -----------------------------------------------------------

fn root_nodes(
    workspace: &WeakEntity<Workspace>,
    repository: Option<&Entity<Repository>>,
    cx: &mut App,
) -> Vec<Node> {
    let Some(repository) = repository else {
        return vec![
            Node::section("Repository"),
            Node::action("init", "Initialize Repository", Box::new(git::Init))
                .icon(IconName::Plus),
            Node::action("clone", "Clone Repository…", Box::new(git::Clone))
                .icon(IconName::CloudDownload),
        ];
    };

    let snapshot = repository.read(cx);
    let head = snapshot.branch.clone();
    let stash_count = snapshot.stash_entries.entries.len();
    let branch_count = snapshot.branch_list.len();
    let branch_error = snapshot.branch_list_error.clone();

    let (ahead, behind) = head
        .as_ref()
        .and_then(|branch| branch.upstream.as_ref())
        .and_then(|upstream| upstream.tracking.status())
        .map(|status| (status.ahead, status.behind))
        .unwrap_or((0, 0));
    let head_name: SharedString = head
        .as_ref()
        .map(|branch| SharedString::from(branch.name().to_string()))
        .unwrap_or_else(|| "detached HEAD".into());

    let mut nodes = vec![
        Node::section("Repository"),
        Node::action("commit", "Commit…", Box::new(git::Commit))
            .icon(IconName::GitCommit)
            .detail(format!("On {head_name}"))
            .keywords("stage message"),
        Node::action("amend", "Amend Last Commit", Box::new(git::Amend)).icon(IconName::Pencil),
        push_node(ahead, head.as_ref()),
        Node::action("push-to", "Push To…", Box::new(git::PushTo)).icon(IconName::ArrowUp),
        Node::action("force-push", "Force Push", Box::new(git::ForcePush))
            .icon(IconName::ArrowUp)
            .detail("Overwrites the remote branch"),
        pull_node(behind),
        Node::action("pull-rebase", "Pull (Rebase)", Box::new(git::PullRebase))
            .icon(IconName::ArrowDown)
            .keywords("rebase"),
        Node::action("fetch", "Fetch", Box::new(git::Fetch)).icon(IconName::ArrowCircle),
        Node::action("fetch-from", "Fetch From…", Box::new(git::FetchFrom))
            .icon(IconName::ArrowCircle),
        Node::action("uncommit", "Undo Last Commit", Box::new(git::Uncommit))
            .icon(IconName::Undo)
            .detail("Keeps the changes in the working tree"),
    ];

    nodes.push(Node::section("Branches"));
    nodes.push(
        Node::page(
            "checkout",
            "Checkout Branch…",
            branch_page(workspace.clone(), repository.clone(), BranchScope::All),
        )
        .icon(IconName::GitBranch)
        .detail(if let Some(error) = &branch_error {
            error.clone()
        } else {
            format!("{branch_count} refs").into()
        })
        .keywords("switch"),
    );
    nodes.push(
        Node::page(
            "recent-branches",
            "Recent Branches",
            branch_page(workspace.clone(), repository.clone(), BranchScope::Recent),
        )
        .icon(IconName::HistoryRerun),
    );
    nodes.push(
        Node::page(
            "local-branches",
            "Local Branches",
            branch_page(workspace.clone(), repository.clone(), BranchScope::Local),
        )
        .icon(IconName::GitBranch),
    );
    nodes.push(
        Node::page(
            "remote-branches",
            "Remote Branches",
            branch_page(workspace.clone(), repository.clone(), BranchScope::Remote),
        )
        .icon(IconName::Server),
    );
    nodes.push(
        Node::page(
            "new-branch",
            "New Branch…",
            new_branch_page(workspace.clone(), repository.clone(), None),
        )
        .icon(IconName::GitBranchPlus)
        .detail(format!("From {head_name}")),
    );
    nodes.push(
        Node::action(
            "rename-branch",
            "Rename Current Branch…",
            Box::new(git::RenameBranch { branch: None }),
        )
        .icon(IconName::Pencil),
    );
    nodes.push(
        Node::action("copy-branch-name", "Copy Branch Name", Box::new(git::CopyBranchName))
            .icon(IconName::Copy),
    );
    nodes.push(
        Node::action("compare-branches", "Compare With Branch…", Box::new(DeployBranchDiff))
            .icon(IconName::Diff)
            .keywords("diff branch"),
    );

    nodes.push(Node::section("History"));
    nodes.push(
        Node::action("graph", "Commit Graph", Box::new(crate::git_graph::Open))
            .icon(IconName::GitGraph),
    );
    nodes.push(
        Node::action(
            "uncommitted",
            "Uncommitted Changes",
            Box::new(zed_actions::git::ViewUncommittedChanges),
        )
        .icon(IconName::Diff),
    );
    nodes.push(
        Node::action(
            "staged",
            "Staged Changes",
            Box::new(zed_actions::git::ViewStagedChanges),
        )
        .icon(IconName::Diff),
    );
    nodes.push(
        Node::action(
            "unstaged",
            "Unstaged Changes",
            Box::new(zed_actions::git::ViewUnstagedChanges),
        )
        .icon(IconName::Diff),
    );
    nodes.push(
        Node::action("file-history", "File History", Box::new(git::FileHistory))
            .icon(IconName::HistoryRerun),
    );
    nodes.push(Node::action("blame", "Blame Current File", Box::new(git::Blame)).icon(IconName::Book));

    nodes.push(Node::section("Stashes"));
    nodes.push(
        Node::action("stash-all", "Stash All Changes", Box::new(git::StashAll))
            .icon(IconName::Bookmark),
    );
    if stash_count == 0 {
        nodes.push(
            Node::run("stash-list", "Stashes", |_, _, _| {})
                .icon(IconName::Bookmark)
                .disabled("No stashes"),
        );
    } else {
        nodes.push(
            Node::page(
                "stash-list",
                "Stashes",
                stash_page(workspace.clone(), repository.clone()),
            )
            .icon(IconName::Bookmark)
            .trailing(stash_count.to_string()),
        );
        nodes.push(
            Node::action("stash-pop", "Pop Latest Stash", Box::new(git::StashPop))
                .icon(IconName::Bookmark),
        );
        nodes.push(
            Node::action("stash-apply", "Apply Latest Stash", Box::new(git::StashApply))
                .icon(IconName::Bookmark),
        );
    }

    nodes.push(Node::section("Remotes & Repositories"));
    nodes.push(
        Node::page(
            "remotes",
            "Remotes",
            remote_page(repository.clone()),
        )
        .icon(IconName::Server),
    );
    nodes.push(
        Node::action(
            "create-pull-request",
            "Create Pull Request",
            Box::new(zed_actions::git::CreatePullRequest),
        )
        .icon(IconName::Github),
    );
    nodes.push(
        Node::action("worktrees", "Worktrees…", Box::new(zed_actions::git::Worktree))
            .icon(IconName::GitWorktree),
    );
    nodes.push(
        Node::action(
            "select-repo",
            "Switch Repository…",
            Box::new(zed_actions::git::SelectRepo),
        )
        .icon(IconName::Folder),
    );

    nodes
}

fn push_node(ahead: u32, head: Option<&Branch>) -> Node {
    let node = Node::action("push", "Push", Box::new(git::Push)).icon(IconName::ArrowUp);
    match head.and_then(|branch| branch.upstream.as_ref()) {
        None => node.detail("No upstream; sets one on push"),
        Some(_) if ahead > 0 => node.trailing(format!("↑{ahead}")),
        Some(_) => node.detail("Nothing to push"),
    }
}

fn pull_node(behind: u32) -> Node {
    let node = Node::action("pull", "Pull", Box::new(git::Pull)).icon(IconName::ArrowDown);
    if behind > 0 {
        node.trailing(format!("↓{behind}"))
    } else {
        node
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BranchScope {
    All,
    Local,
    Remote,
    Recent,
}

impl BranchScope {
    fn includes(self, branch: &Branch) -> bool {
        match self {
            Self::All | Self::Recent => true,
            Self::Local => !branch.is_remote(),
            Self::Remote => branch.is_remote(),
        }
    }
}

/// The branch list is already an `Arc<[Branch]>` kept up to date off the main
/// thread, so this only turns the relevant slice into rows.
///
/// ponytail: rows are materialized for the whole scope when the page opens.
/// That is strictly cheaper than `branch_picker`, which rebuilds a `Vec<Entry>`
/// of cloned `Branch`es on every keystroke; if a repository ever makes even this
/// too slow, the fix is to keep `Arc<[Branch]>` in the page and build rows only
/// for matched indices.
fn branch_page(
    workspace: WeakEntity<Workspace>,
    repository: Entity<Repository>,
    scope: BranchScope,
) -> Children {
    Children::deferred(move |cx| {
        let workspace = workspace.clone();
        let repository = repository.clone();
        let snapshot = repository.read(cx);
        let head_ref = snapshot.branch.as_ref().map(|branch| branch.ref_name.clone());
        // `branch_list` already arrives newest-first, so the recency page is
        // just a prefix of it. Take before cloning so a 100k-ref repository
        // doesn't clone 100k branches to show twenty.
        let limit = if scope == BranchScope::Recent {
            RECENT_BRANCH_COUNT
        } else {
            usize::MAX
        };
        let nodes = snapshot
            .branch_list
            .iter()
            .filter(|branch| scope.includes(branch))
            .take(limit)
            .cloned()
            .map(|branch| branch_node(&workspace, &repository, branch, head_ref.as_ref()))
            .collect();
        Task::ready(Ok(nodes))
    })
}

fn branch_node(
    workspace: &WeakEntity<Workspace>,
    repository: &Entity<Repository>,
    branch: Branch,
    head_ref: Option<&SharedString>,
) -> Node {
    let name: SharedString = branch.name().to_string().into();
    let is_head = head_ref.is_some_and(|head| *head == branch.ref_name);
    let is_remote = branch.is_remote();

    let detail = branch.most_recent_commit.as_ref().map(|commit| {
        SharedString::from(format!(
            "{} · {}",
            commit.sha.chars().take(git::SHORT_SHA_LENGTH).collect::<String>(),
            commit.subject
        ))
    });
    let trailing = branch
        .upstream
        .as_ref()
        .and_then(|upstream| upstream.tracking.status())
        .and_then(|status| match (status.ahead, status.behind) {
            (0, 0) => None,
            (ahead, 0) => Some(format!("↑{ahead}")),
            (0, behind) => Some(format!("↓{behind}")),
            (ahead, behind) => Some(format!("↑{ahead} ↓{behind}")),
        });

    let mut node = Node::run(
        format!("branch:{}", branch.ref_name),
        name.clone(),
        {
            let repository = repository.clone();
            let workspace = workspace.clone();
            move |_query, _window, cx| {
                checkout(&repository, &workspace, &name, cx);
            }
        },
    )
    .icon(if is_head {
        IconName::Check
    } else if is_remote {
        IconName::Server
    } else {
        IconName::GitBranch
    })
    .keywords(branch.ref_name.clone())
    .submenu(branch_verbs(
        workspace.clone(),
        repository.clone(),
        branch,
        is_head,
    ));

    if is_head {
        node = node.icon_color(Color::Accent);
    }
    if let Some(detail) = detail {
        node = node.detail(detail);
    }
    if let Some(trailing) = trailing {
        node = node.trailing(trailing);
    }
    node
}

fn branch_verbs(
    workspace: WeakEntity<Workspace>,
    repository: Entity<Repository>,
    branch: Branch,
    is_head: bool,
) -> Children {
    let name: SharedString = branch.name().to_string().into();
    let is_remote = branch.is_remote();
    let mut verbs = Vec::new();

    verbs.push({
        let node = Node::run("checkout", "Checkout", {
            let repository = repository.clone();
            let workspace = workspace.clone();
            let name = name.clone();
            move |_query, _window, cx| checkout(&repository, &workspace, &name, cx)
        })
        .icon(IconName::Check);
        if is_head {
            node.disabled("Already checked out")
        } else {
            node
        }
    });

    verbs.push(
        Node::page(
            "new-from",
            "New Branch From Here…",
            new_branch_page(
                workspace.clone(),
                repository.clone(),
                Some(name.to_string()),
            ),
        )
        .icon(IconName::GitBranchPlus),
    );

    if !is_remote {
        verbs.push(
            Node::action(
                "rename",
                "Rename…",
                Box::new(git::RenameBranch {
                    branch: Some(name.to_string()),
                }),
            )
            .icon(IconName::Pencil),
        );
    }

    verbs.push({
        let node = Node::run("delete", "Delete", {
            let repository = repository.clone();
            let workspace = workspace.clone();
            let name = name.clone();
            move |_query, _window, cx| {
                delete_branch(&repository, &workspace, &name, is_remote, false, cx)
            }
        })
        .icon(IconName::Trash);
        if is_head {
            node.disabled("Cannot delete the checked-out branch")
        } else {
            node
        }
    });

    verbs.push({
        let node = Node::run("force-delete", "Force Delete", {
            let name = name.clone();
            move |_query, _window, cx| {
                delete_branch(&repository, &workspace, &name, is_remote, true, cx)
            }
        })
        .icon(IconName::Trash)
        .detail("Discards unmerged commits");
        if is_head {
            node.disabled("Cannot delete the checked-out branch")
        } else {
            node
        }
    });

    verbs.push(
        Node::run("copy-name", "Copy Name", move |_query, _window, cx| {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(name.to_string()));
        })
        .icon(IconName::Copy),
    );

    if let Some(commit) = branch.most_recent_commit.as_ref() {
        let sha = commit.sha.clone();
        verbs.push(
            Node::run("copy-sha", "Copy Commit SHA", move |_query, _window, cx| {
                cx.write_to_clipboard(gpui::ClipboardItem::new_string(sha.to_string()));
            })
            .icon(IconName::Hash)
            .detail(
                commit
                    .sha
                    .chars()
                    .take(git::SHORT_SHA_LENGTH)
                    .collect::<String>(),
            ),
        );
    }

    // Push and pull act on HEAD, so they are only honest verbs on the branch
    // that is actually checked out.
    if is_head {
        verbs.push(Node::action("push", "Push", Box::new(git::Push)).icon(IconName::ArrowUp));
        verbs.push(Node::action("pull", "Pull", Box::new(git::Pull)).icon(IconName::ArrowDown));
        verbs.push(
            Node::action("compare", "Compare With Branch…", Box::new(DeployBranchDiff))
                .icon(IconName::Diff),
        );
    }

    Children::ready(verbs)
}

/// A page whose single row turns whatever the user types into a branch name.
/// This is how the palette avoids a modal for text entry: the search field
/// already is the text field.
fn new_branch_page(
    workspace: WeakEntity<Workspace>,
    repository: Entity<Repository>,
    base: Option<String>,
) -> Children {
    Children::deferred(move |_cx| {
        let workspace = workspace.clone();
        let repository = repository.clone();
        let base = base.clone();
        let detail = match &base {
            Some(base) => format!("Type a name, then press Enter. Based on {base}"),
            None => "Type a name, then press Enter. Based on the current branch".to_string(),
        };
        Task::ready(Ok(vec![Node::run(
            "create-branch",
            "Create Branch",
            move |query, _window, cx| {
                let name = normalize_branch_name(query);
                if name.is_empty() {
                    return;
                }
                let workspace = workspace.clone();
                let base = base.clone();
                let receiver = repository
                    .update(cx, |repository, _| repository.create_branch(name, base));
                cx.spawn(async move |cx| {
                    match receiver.await {
                        Ok(Ok(())) => {}
                        Ok(Err(error)) => {
                            report_in(&workspace, "create branch", error, cx).await;
                        }
                        Err(error) => {
                            report_in(&workspace, "create branch", error.into(), cx).await;
                        }
                    }
                })
                .detach();
            },
        )
        .icon(IconName::GitBranchPlus)
        .detail(detail)]))
    })
}

fn stash_page(workspace: WeakEntity<Workspace>, repository: Entity<Repository>) -> Children {
    Children::deferred(move |cx| {
        let workspace = workspace.clone();
        let repository = repository.clone();
        let entries = repository.read(cx).stash_entries.entries.clone();
        let nodes = entries
            .iter()
            .map(|entry| {
                let index = entry.index;
                let verbs = vec![
                    stash_verb(
                        "pop",
                        "Pop",
                        IconName::Bookmark,
                        &workspace,
                        &repository,
                        index,
                        StashVerb::Pop,
                    ),
                    stash_verb(
                        "apply",
                        "Apply",
                        IconName::Copy,
                        &workspace,
                        &repository,
                        index,
                        StashVerb::Apply,
                    ),
                    stash_verb(
                        "drop",
                        "Drop",
                        IconName::Trash,
                        &workspace,
                        &repository,
                        index,
                        StashVerb::Drop,
                    ),
                ];
                let mut node = Node::run(
                    format!("stash:{index}"),
                    format!("stash@{{{index}}}"),
                    {
                        let workspace = workspace.clone();
                        let repository = repository.clone();
                        move |_query, _window, cx| {
                            run_stash_verb(&repository, &workspace, index, StashVerb::Pop, cx)
                        }
                    },
                )
                .icon(IconName::Bookmark)
                .detail(entry.message.clone())
                .submenu(Children::ready(verbs));
                if let Some(branch) = &entry.branch {
                    node = node.keywords(branch.clone());
                }
                node
            })
            .collect();
        Task::ready(Ok(nodes))
    })
}

#[derive(Clone, Copy)]
enum StashVerb {
    Pop,
    Apply,
    Drop,
}

impl StashVerb {
    fn description(self) -> &'static str {
        match self {
            Self::Pop => "pop stash",
            Self::Apply => "apply stash",
            Self::Drop => "drop stash",
        }
    }
}

fn stash_verb(
    id: &'static str,
    label: &'static str,
    icon: IconName,
    workspace: &WeakEntity<Workspace>,
    repository: &Entity<Repository>,
    index: usize,
    verb: StashVerb,
) -> Node {
    let workspace = workspace.clone();
    let repository = repository.clone();
    Node::run(id, label, move |_query, _window, cx| {
        run_stash_verb(&repository, &workspace, index, verb, cx)
    })
    .icon(icon)
}

fn run_stash_verb(
    repository: &Entity<Repository>,
    workspace: &WeakEntity<Workspace>,
    index: usize,
    verb: StashVerb,
    cx: &mut App,
) {
    let workspace = workspace.clone();
    match verb {
        StashVerb::Pop | StashVerb::Apply => {
            let task = repository.update(cx, |repository, cx| match verb {
                StashVerb::Pop => repository.stash_pop(Some(index), cx),
                _ => repository.stash_apply(Some(index), cx),
            });
            cx.spawn(async move |cx| {
                if let Err(error) = task.await {
                    report_in(&workspace, verb.description(), error, cx).await;
                }
            })
            .detach();
        }
        StashVerb::Drop => {
            let receiver = repository.update(cx, |repository, cx| repository.stash_drop(Some(index), cx));
            cx.spawn(async move |cx| match receiver.await {
                Ok(Ok(())) => {}
                Ok(Err(error)) => report_in(&workspace, verb.description(), error, cx).await,
                Err(error) => report_in(&workspace, verb.description(), error.into(), cx).await,
            })
            .detach();
        }
    }
}

fn remote_page(repository: Entity<Repository>) -> Children {
    Children::deferred(move |cx| {
        let receiver = repository.update(cx, |repository, _| repository.remote_urls());
        // Rows are built back on the foreground: `Node` holds `Rc`s, so it can't
        // cross a thread boundary. Only the git invocation is off-thread.
        cx.spawn(async move |_cx| {
            let mut remotes: Vec<(String, String)> = receiver.await??.into_iter().collect();
            remotes.sort_by(|a, b| a.0.cmp(&b.0));
            Ok(remotes.into_iter().map(remote_node).collect())
        })
    })
}

fn remote_node((name, url): (String, String)) -> Node {
    let icon = if url.contains("github") {
        IconName::Github
    } else if url.contains("gitlab") {
        IconName::Gitlab
    } else {
        IconName::Server
    };
    Node::run(format!("remote:{name}"), name, {
        let url = url.clone();
        move |_query, _window, cx| {
            cx.write_to_clipboard(gpui::ClipboardItem::new_string(url.clone()));
        }
    })
    .icon(icon)
    .detail(url)
    .submenu(Children::ready(vec![
        Node::action("fetch-from", "Fetch From…", Box::new(git::FetchFrom))
            .icon(IconName::ArrowCircle),
        Node::action("push-to", "Push To…", Box::new(git::PushTo)).icon(IconName::ArrowUp),
    ]))
}

fn checkout(
    repository: &Entity<Repository>,
    workspace: &WeakEntity<Workspace>,
    name: &str,
    cx: &mut App,
) {
    let workspace = workspace.clone();
    let name = name.to_string();
    let receiver = repository.update(cx, |repository, _| repository.change_branch(name));
    cx.spawn(async move |cx| match receiver.await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => report_in(&workspace, "checkout branch", error, cx).await,
        Err(error) => report_in(&workspace, "checkout branch", error.into(), cx).await,
    })
    .detach();
}

fn delete_branch(
    repository: &Entity<Repository>,
    workspace: &WeakEntity<Workspace>,
    name: &str,
    is_remote: bool,
    force: bool,
    cx: &mut App,
) {
    let workspace = workspace.clone();
    let name = name.to_string();
    let receiver = repository.update(cx, |repository, _| {
        repository.delete_branch(is_remote, name, force)
    });
    cx.spawn(async move |cx| match receiver.await {
        Ok(Ok(())) => {}
        Ok(Err(error)) => report_in(&workspace, "delete branch", error, cx).await,
        Err(error) => report_in(&workspace, "delete branch", error.into(), cx).await,
    })
    .detach();
}

async fn report_in(
    workspace: &WeakEntity<Workspace>,
    action: &'static str,
    error: anyhow::Error,
    cx: &mut gpui::AsyncApp,
) {
    let workspace = workspace.clone();
    cx.update(|cx| {
        if let Some(workspace) = workspace.upgrade() {
            show_error_toast(workspace, action, error, cx);
        } else {
            log::error!("failed to {action}: {error}");
        }
    });
}

pub fn register(workspace: &mut Workspace) {
    workspace.register_action(
        |workspace, _: &zed_actions::git::CommandCenter, window, cx| {
            open(workspace, window, cx);
        },
    );
}

pub fn open(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) {
    let workspace_handle = workspace.weak_handle();
    let repository = workspace.project().read(cx).active_repository(cx);
    workspace.toggle_modal(window, cx, |window, cx| {
        GitCommandCenter::new(workspace_handle, repository, window, cx)
    });
}
