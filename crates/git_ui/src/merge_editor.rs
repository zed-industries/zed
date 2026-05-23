//! 3-way merge editor.
//!
//! Renders a conflicted file alongside the `ours` and `theirs` index stages
//! (and optionally the common ancestor `base`) so the user can resolve the
//! conflict with full side-by-side context. Side panes show per-stage diffs
//! against the common ancestor when one is available.
//!
//! The Result pane keeps Zed's existing inline conflict resolution buttons
//! (registered globally via `git_ui::init`'s `observe_new(Editor)` hook) and
//! gains the "Use Base" button alongside the existing Use Ours / Use Theirs /
//! Use Both buttons whenever the conflict has a base section.

use anyhow::{Context as _, Result, anyhow};
use buffer_diff::BufferDiff;
use editor::{Editor, EditorEvent, MultiBuffer};
use git::repository::UnmergedStages;
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, Subscription, Task, WeakEntity, Window,
};
use language::{Buffer, Capability};
use project::{ConflictRegion, ConflictSetUpdate, Project, ProjectItem as _, ProjectPath};
use std::any::{Any, TypeId};
use std::sync::Arc;
use ui::prelude::*;
use ui::{IconButtonShape, Tooltip};
use util::paths::PathExt as _;
use workspace::{
    Item, ItemHandle as _, Workspace,
    item::{ItemEvent, SaveOptions, TabContentParams},
};

/// Which side of a conflict a button accepts.
#[derive(Clone, Copy)]
enum AcceptSide {
    Ours,
    Theirs,
}

impl AcceptSide {
    fn range(self, conflict: &project::ConflictRegion) -> Option<std::ops::Range<language::Anchor>> {
        match self {
            AcceptSide::Ours => Some(conflict.ours.clone()),
            AcceptSide::Theirs => Some(conflict.theirs.clone()),
        }
    }
}

pub struct MergeEditor {
    result_buffer: Entity<Buffer>,
    result_editor: Entity<Editor>,
    ours_editor: Entity<Editor>,
    theirs_editor: Entity<Editor>,
    /// Base/common-ancestor editor. Present even when hidden so toggling is
    /// cheap; when `base_visible` is false it's simply not rendered.
    base_editor: Option<Entity<Editor>>,
    base_visible: bool,
    ours_branch_label: SharedString,
    theirs_branch_label: SharedString,
    /// Which inner editor most recently received focus. Drives
    /// [`Focusable::focus_handle`] and [`Item::act_as_type`] so that workspace
    /// systems (focus tracking, vim, inline assist, etc.) see the editor the
    /// user is actually interacting with, not just the Result pane.
    last_focused_pane: FocusedPane,
    /// Per-conflict accept-button block IDs that we own in the Ours and
    /// Theirs side panes, so we can remove and re-insert them when the
    /// underlying conflict set changes.
    ours_side_blocks: Vec<editor::display_map::CustomBlockId>,
    theirs_side_blocks: Vec<editor::display_map::CustomBlockId>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Copy, Clone, Default)]
enum FocusedPane {
    Ours,
    #[default]
    Result,
    Theirs,
    Base,
}

impl MergeEditor {
    pub fn open(
        project_path: ProjectPath,
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        window.spawn(cx, async move |cx| {
            // Reuse an existing merge editor for this path if one is already
            // open in the workspace; clicking the same conflicted file in the
            // git panel shouldn't proliferate tabs. The check is done inside
            // the spawned future to avoid nesting workspace.update calls when
            // the caller is itself running inside one.
            let existing = workspace.update_in(cx, |workspace, window, cx| {
                let existing = workspace
                    .items_of_type::<MergeEditor>(cx)
                    .find(|item| {
                        item.read(cx)
                            .result_buffer
                            .read(cx)
                            .project_path(cx)
                            .as_ref()
                            == Some(&project_path)
                    });
                if let Some(existing) = existing {
                    workspace.activate_item(&existing, true, true, window, cx);
                    Some(existing)
                } else {
                    None
                }
            })?;
            if let Some(existing) = existing {
                return Ok(existing);
            }

            let project = workspace.update(cx, |workspace, _| workspace.project().clone())?;

            let (repository, repo_path) = project.read_with(cx, |project, cx| {
                project
                    .git_store()
                    .read(cx)
                    .repository_and_path_for_project_path(&project_path, cx)
                    .context("path is not in a git repository")
            })?;

            let result_buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await?;

            let stages = repository
                .update(cx, |repo, cx| repo.load_unmerged_stages(repo_path.clone(), cx))
                .await?;

            // No stages at all = file isn't actually unmerged in the index, or
            // every stage is binary/non-UTF8. A 3-pane view with three empty
            // editors would be misleading; better to surface the issue.
            if stages == UnmergedStages::default() {
                return Err(anyhow!(
                    "Cannot open 3-way merge editor: no text stages in git index \
                     (file may be binary, a submodule, or not actually conflicted)"
                ));
            }

            // If the working tree was written with 2-way markers (user's
            // `merge.conflictStyle` is `merge`), fetch the diff3-style merge
            // output now; we'll apply it to the buffer AFTER the result
            // editor is fully constructed, to avoid a race where the
            // `ConflictAddon`'s subscription on the conflict set fires before
            // its own initial-state sync has populated `block_ids` and trips
            // the addon's `debug_panic!` in conflict_view.rs. Skipped if the
            // buffer is already dirty (preserves in-progress edits) or if
            // the base stage isn't available.
            let needs_diff3_rewrite = result_buffer.read_with(cx, |buffer, _| {
                if buffer.is_dirty() {
                    return false;
                }
                let snapshot = buffer.snapshot();
                let conflicts = project::ConflictSet::parse(&snapshot);
                !conflicts.conflicts.is_empty()
                    && conflicts.conflicts.iter().all(|c| c.base.is_none())
            });
            let pending_diff3_rewrite = if needs_diff3_rewrite && stages.base.is_some() {
                repository
                    .update(cx, |repo, cx| repo.merge_file_diff3(repo_path.clone(), cx))
                    .await
                    .ok()
            } else {
                None
            };

            let (ours_branch_label, theirs_branch_label) = repository.read_with(cx, |repo, _| {
                let branch = repo
                    .branch
                    .as_ref()
                    .map(|b| b.name().to_string())
                    .unwrap_or_else(|| "HEAD".to_string());
                (
                    SharedString::from(branch),
                    SharedString::new_static("Incoming"),
                )
            });

            // Build the transient stage buffers and base↔ours / base↔theirs
            // diffs up front (off the workspace update), so the editor
            // construction itself stays synchronous.
            let StageBuffers {
                base_buffer,
                ours_buffer,
                theirs_buffer,
                ours_diff,
                theirs_diff,
            } = build_stage_buffers(&result_buffer, &stages, cx).await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let project = workspace.project().clone();
                let result_buffer_for_rewrite = result_buffer.clone();
                let merge_editor = cx.new(|cx| {
                    MergeEditor::new(
                        result_buffer,
                        base_buffer,
                        ours_buffer,
                        theirs_buffer,
                        ours_diff,
                        theirs_diff,
                        ours_branch_label,
                        theirs_branch_label,
                        project,
                        window,
                        cx,
                    )
                });

                // Now that the result editor + `ConflictAddon` are fully
                // initialized (the addon's `block_ids` matches the conflict
                // set's current snapshot), apply the diff3 rewrite. The
                // subsequent `ConflictSetUpdate` event arrives at a
                // consistent addon state and can be processed without
                // tripping the `block_ids` invariant in conflict_view.rs.
                if let Some(diff3_text) = pending_diff3_rewrite {
                    result_buffer_for_rewrite.update(cx, |buffer, cx| {
                        buffer.set_text(diff3_text, cx);
                    });
                }

                let pane = workspace.active_pane().clone();
                pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(merge_editor.clone()), true, true, None, window, cx);
                });

                merge_editor
            })
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn new(
        result_buffer: Entity<Buffer>,
        base_buffer: Option<Entity<Buffer>>,
        ours_buffer: Entity<Buffer>,
        theirs_buffer: Entity<Buffer>,
        ours_diff: Option<Entity<BufferDiff>>,
        theirs_diff: Option<Entity<BufferDiff>>,
        ours_branch_label: SharedString,
        theirs_branch_label: SharedString,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let result_editor = cx.new(|cx| {
            let multibuffer = cx.new(|cx| MultiBuffer::singleton(result_buffer.clone(), cx));
            let mut editor = Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx);
            // Suppress the regular working-tree-vs-HEAD diff overlay. The
            // merge editor's Result pane should show conflict highlights only,
            // not the ambient git diff that paints `import sys` and the
            // conflict marker lines red.
            editor.start_temporary_diff_override();
            // Override the default conflict marker colors with the same
            // row-background palette the side panes use for their
            // additions/deletions hunks, so the Result pane reads as a
            // continuation of those rather than its own visual language.
            // Base gets a subdued accent tint (no editor-level base color
            // exists; the side panes don't render base directly).
            let theme_colors = cx.theme().colors();
            editor.register_addon(crate::conflict_view::ConflictHighlightPalette {
                ours: theme_colors.editor_diff_hunk_added_background,
                theirs: theme_colors.editor_diff_hunk_deleted_background,
                base: theme_colors.text_accent.opacity(0.18),
            });
            editor
        });

        let ours_editor = build_stage_editor(ours_buffer, ours_diff, project.clone(), window, cx);
        let theirs_editor =
            build_stage_editor(theirs_buffer, theirs_diff, project.clone(), window, cx);
        let base_editor =
            base_buffer.map(|buffer| build_stage_editor(buffer, None, project.clone(), window, cx));

        let mut subscriptions = Vec::new();
        // Re-emit the inner editor's events as our own so the workspace's Item
        // machinery sees dirty/saved/title-changed transitions on this tab.
        // GPUI events don't bubble through `Render`; the workspace subscribes
        // to events on the Item entity itself.
        subscriptions.push(cx.subscribe(&result_editor, |_, _, event: &EditorEvent, cx| {
            cx.emit(event.clone());
        }));
        // Track focus across all panes so `focus_handle` and `act_as_type`
        // can return the editor the user is actually interacting with.
        // Without this, clicking into Ours/Theirs/Base leaves vim and the
        // workspace's `contains_focused` check pointing at Result.
        for (pane, editor) in [
            (FocusedPane::Result, &result_editor),
            (FocusedPane::Ours, &ours_editor),
            (FocusedPane::Theirs, &theirs_editor),
        ]
        .into_iter()
        .chain(base_editor.as_ref().map(|e| (FocusedPane::Base, e)))
        {
            let focus_handle = editor.read(cx).focus_handle(cx);
            subscriptions.push(cx.on_focus_in(&focus_handle, window, move |this, _, cx| {
                this.last_focused_pane = pane;
                cx.notify();
            }));
        }
        // Subscribe to the conflict set so per-conflict accept buttons in
        // the side panes are kept in sync as conflicts are added, removed,
        // or resolved. The set is opened by `ConflictAddon` (registered on
        // the result editor via `git_ui::init`); we go through the project's
        // git store to get a handle to the same one.
        let git_store = project.read(cx).git_store().clone();
        let conflict_set = git_store.update(cx, |git_store, cx| {
            git_store.open_conflict_set(result_buffer.clone(), cx)
        });
        subscriptions.push(cx.subscribe(
            &conflict_set,
            |this, _, _: &ConflictSetUpdate, cx| {
                this.refresh_side_pane_buttons(cx);
            },
        ));

        Self {
            result_buffer,
            result_editor,
            ours_editor,
            theirs_editor,
            base_editor,
            base_visible: false,
            ours_branch_label,
            theirs_branch_label,
            last_focused_pane: FocusedPane::default(),
            ours_side_blocks: Vec::new(),
            theirs_side_blocks: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    /// The inner editor the user is currently interacting with, or
    /// `result_editor` when no pane has been focused yet (the initial state).
    /// Falls back to `result_editor` if the recorded pane is `Base` but no
    /// base editor exists.
    fn focused_inner_editor(&self) -> &Entity<Editor> {
        match self.last_focused_pane {
            FocusedPane::Ours => &self.ours_editor,
            FocusedPane::Theirs => &self.theirs_editor,
            FocusedPane::Base => self.base_editor.as_ref().unwrap_or(&self.result_editor),
            FocusedPane::Result => &self.result_editor,
        }
    }

    /// Whether the working-tree buffer currently has any parsed conflicts.
    /// Drives the enabled state of the per-pane "Accept all" buttons.
    fn has_conflicts(&self, cx: &App) -> bool {
        !self.conflict_regions(cx).is_empty()
    }

    /// Returns every parsed conflict region in the result buffer, in document
    /// order. Pulled from the `ConflictAddon` that `git_ui::init` registers on
    /// every editor; if it isn't there yet (e.g., the first paint), the result
    /// is empty.
    fn conflict_regions(&self, cx: &App) -> Vec<project::ConflictRegion> {
        let editor = self.result_editor.read(cx);
        let Some(addon) = editor.addon::<crate::conflict_view::ConflictAddon>() else {
            return Vec::new();
        };
        let buffer_id = self.result_buffer.read(cx).remote_id();
        let Some(conflict_set) = addon.conflict_set(buffer_id) else {
            return Vec::new();
        };
        conflict_set.read(cx).snapshot.conflicts.to_vec()
    }

    /// Accepts the given side for every conflict in the result buffer.
    fn accept_all(
        &mut self,
        side: AcceptSide,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let conflicts = self.conflict_regions(cx);
        for conflict in conflicts {
            let Some(range) = side.range(&conflict) else {
                continue;
            };
            crate::conflict_view::resolve_conflict(
                self.result_editor.downgrade(),
                conflict,
                vec![range],
                window,
                cx,
            )
            .detach();
        }
    }

    /// Refreshes the per-conflict "Use this" buttons in the Ours and Theirs
    /// side panes, mirroring the inline buttons the result editor already
    /// shows above each conflict. Each button is anchored to the first text
    /// match of that conflict's side content within the side pane's buffer —
    /// a cheap text-search rather than a full 3-way alignment, which suffices
    /// when conflict payloads are reasonably distinctive (the common case).
    fn refresh_side_pane_buttons(&mut self, cx: &mut Context<Self>) {
        let conflicts = self.conflict_regions(cx);
        let result_snapshot = self.result_buffer.read(cx).snapshot();
        let result_handle = self.result_editor.downgrade();

        for (editor, side, stored_blocks) in [
            (
                self.ours_editor.clone(),
                AcceptSide::Ours,
                &mut self.ours_side_blocks,
            ),
            (
                self.theirs_editor.clone(),
                AcceptSide::Theirs,
                &mut self.theirs_side_blocks,
            ),
        ] {
            // Tear down the previous batch before recomputing positions
            // against the (possibly changed) conflict set.
            editor.update(cx, |editor, cx| {
                let stale = std::mem::take(stored_blocks);
                if !stale.is_empty() {
                    editor.remove_blocks(stale.into_iter().collect(), None, cx);
                }
            });

            let new_blocks = build_side_pane_button_blocks(
                &editor,
                &result_snapshot,
                &conflicts,
                side,
                result_handle.clone(),
                cx,
            );
            *stored_blocks = new_blocks;
        }
    }

    /// Returns the underlying single buffer behind a side-pane editor.
    /// Side-pane editors wrap their stage buffer in a `MultiBuffer::singleton`
    /// so `as_singleton()` always succeeds.
    #[cfg(test)]
    fn stage_buffer(editor: &Entity<Editor>, cx: &App) -> Entity<Buffer> {
        editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .expect("side-pane multibuffer is a singleton")
    }

    fn toggle_base(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        if self.base_editor.is_some() {
            self.base_visible = !self.base_visible;
            cx.notify();
        }
    }

    fn render_header(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let base_available = self.base_editor.is_some();
        let base_visible = self.base_visible;
        h_flex()
            .px_2()
            .py_1()
            .gap_2()
            .bg(cx.theme().colors().editor_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(SharedString::new_static("3-way merge"))
                    .size(ui::LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(div().flex_grow())
            .child(
                IconButton::new(
                    "toggle-merge-base",
                    if base_visible {
                        IconName::EyeOff
                    } else {
                        IconName::Eye
                    },
                )
                .shape(IconButtonShape::Square)
                .icon_size(ui::IconSize::Small)
                .disabled(!base_available)
                .tooltip(move |_window, cx| {
                    let text = if !base_available {
                        "No common ancestor available (file added on both sides)"
                    } else if base_visible {
                        "Hide common ancestor"
                    } else {
                        "Show common ancestor"
                    };
                    Tooltip::simple(text, cx)
                })
                .on_click(cx.listener(|this, _, window, cx| this.toggle_base(window, cx))),
            )
    }

    fn render_pane_header(
        &self,
        label: SharedString,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        h_flex()
            .px_2()
            .py_1()
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(label)
                    .size(ui::LabelSize::Small)
                    .color(Color::Muted),
            )
    }

    /// Side-pane header with an "Accept all" button that resolves every
    /// conflict in the result buffer with this side. The button is disabled
    /// when there are no remaining conflicts to apply.
    fn render_side_pane_header(
        &self,
        label: SharedString,
        side: AcceptSide,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let has_conflicts = self.has_conflicts(cx);
        let (button_id, button_label, tooltip): (&'static str, &'static str, &'static str) =
            match side {
                AcceptSide::Ours => (
                    "accept-all-ours",
                    "Accept all",
                    "Accept this side for every conflict in the file",
                ),
                AcceptSide::Theirs => (
                    "accept-all-theirs",
                    "Accept all",
                    "Accept this side for every conflict in the file",
                ),
            };
        h_flex()
            .px_2()
            .py_1()
            .gap_2()
            .bg(cx.theme().colors().tab_bar_background)
            .border_b_1()
            .border_color(cx.theme().colors().border)
            .child(
                Label::new(label)
                    .size(ui::LabelSize::Small)
                    .color(Color::Muted),
            )
            .child(div().flex_grow())
            .child(
                Button::new(button_id, button_label)
                    .label_size(LabelSize::Small)
                    .disabled(!has_conflicts)
                    .tooltip(move |_window, cx| Tooltip::simple(tooltip, cx))
                    .on_click(cx.listener(move |this, _, window, cx| {
                        this.accept_all(side, window, cx)
                    })),
            )
    }
}

/// Inserts a "Use this" accept button block above each conflict's region in
/// the given side-pane editor and returns the resulting block IDs. Each
/// button locates its anchor by searching the side pane's text for the
/// conflict's payload (the text between the markers in the working tree);
/// if the payload isn't found (e.g. it's empty or duplicated elsewhere with
/// no unique prefix), that conflict simply doesn't get a side-pane button —
/// the inline buttons in the Result pane still work as the fallback.
fn build_side_pane_button_blocks(
    editor: &Entity<Editor>,
    result_snapshot: &language::BufferSnapshot,
    conflicts: &[ConflictRegion],
    side: AcceptSide,
    result_handle: WeakEntity<Editor>,
    cx: &mut Context<MergeEditor>,
) -> Vec<editor::display_map::CustomBlockId> {
    use editor::display_map::{BlockPlacement, BlockProperties, BlockStyle};
    use language::OffsetRangeExt as _;

    let pane_buffer = editor
        .read(cx)
        .buffer()
        .read(cx)
        .as_singleton()
        .expect("side-pane multibuffer is a singleton");
    let pane_snapshot = pane_buffer.read(cx).snapshot();
    let pane_text = pane_snapshot.text();

    let label: SharedString = match side {
        AcceptSide::Ours => "Use this".into(),
        AcceptSide::Theirs => "Use this".into(),
    };

    let mut block_properties = Vec::new();
    for conflict in conflicts {
        let Some(range) = side.range(conflict) else {
            continue;
        };
        let offset_range = range.to_offset(result_snapshot);
        if offset_range.is_empty() {
            continue;
        }
        let conflict_text: String = result_snapshot
            .text_for_range(offset_range.clone())
            .collect();
        let Some(byte_pos) = pane_text.find(&conflict_text) else {
            continue;
        };
        let buffer_anchor = pane_snapshot.anchor_before(byte_pos);
        let multibuffer_snapshot = editor.read(cx).buffer().read(cx).snapshot(cx);
        let Some(anchor) = multibuffer_snapshot.anchor_in_excerpt(buffer_anchor) else {
            continue;
        };
        let conflict_for_click = conflict.clone();
        let result_handle = result_handle.clone();
        let label = label.clone();
        block_properties.push(BlockProperties {
            placement: BlockPlacement::Above(anchor),
            height: Some(1),
            style: BlockStyle::Sticky,
            render: std::sync::Arc::new(move |cx| {
                let conflict = conflict_for_click.clone();
                let result_handle = result_handle.clone();
                let label = label.clone();
                let resolve_range = side.range(&conflict).expect("side range present");
                h_flex()
                    .id(cx.block_id)
                    .h(cx.line_height)
                    .ml(cx.margins.gutter.width)
                    .bg(cx.theme().colors().editor_background)
                    .child(
                        Button::new(
                            gpui::ElementId::Name(
                                format!("merge-side-accept-{:?}", cx.block_id).into(),
                            ),
                            label,
                        )
                            .label_size(LabelSize::Small)
                            .on_click(move |_, window, cx| {
                                crate::conflict_view::resolve_conflict(
                                    result_handle.clone(),
                                    conflict.clone(),
                                    vec![resolve_range.clone()],
                                    window,
                                    cx,
                                )
                                .detach();
                            }),
                    )
                    .into_any()
            }),
            priority: 0,
        });
    }

    if block_properties.is_empty() {
        return Vec::new();
    }
    editor.update(cx, |editor, cx| editor.insert_blocks(block_properties, None, cx))
}

/// Builds an editor for one of the read-only side panes. If a `BufferDiff` is
/// provided (i.e. we have a common ancestor to diff against), it's attached so
/// the pane shows red/green hunks for what that stage changed vs. base.
fn build_stage_editor(
    stage_buffer: Entity<Buffer>,
    diff: Option<Entity<BufferDiff>>,
    project: Entity<Project>,
    window: &mut Window,
    cx: &mut Context<MergeEditor>,
) -> Entity<Editor> {
    cx.new(|cx| {
        let multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::singleton(stage_buffer, cx);
            if let Some(diff) = diff {
                multibuffer.add_diff(diff, cx);
            }
            multibuffer
        });
        let mut editor = Editor::for_multibuffer(multibuffer, Some(project), window, cx);
        editor.set_read_only(true);
        // Show all hunks expanded since the user is here to review every change.
        editor.set_expand_all_diff_hunks(cx);
        editor
    })
}

struct StageBuffers {
    base_buffer: Option<Entity<Buffer>>,
    ours_buffer: Entity<Buffer>,
    theirs_buffer: Entity<Buffer>,
    /// `BufferDiff` of Ours against Base, attached to the Ours pane. `None`
    /// when there's no base section to compare against.
    ours_diff: Option<Entity<BufferDiff>>,
    /// `BufferDiff` of Theirs against Base, attached to the Theirs pane.
    theirs_diff: Option<Entity<BufferDiff>>,
}

async fn build_stage_buffers(
    result_buffer: &Entity<Buffer>,
    stages: &UnmergedStages,
    cx: &mut AsyncApp,
) -> Result<StageBuffers> {
    let (language, language_registry) = result_buffer.read_with(cx, |buffer, _| {
        (buffer.language().cloned(), buffer.language_registry())
    });

    // Transient in-memory buffers (no `ProjectPath`) so language servers don't
    // see phantom `didOpen`s for files that don't exist on disk.
    let make_buffer = |text: String, cx: &mut AsyncApp| -> Entity<Buffer> {
        let language = language.clone();
        let language_registry = language_registry.clone();
        cx.new(|cx| {
            let mut buffer = Buffer::local(text, cx);
            buffer.set_language(language, cx);
            if let Some(registry) = language_registry {
                buffer.set_language_registry(registry);
            }
            buffer.set_capability(Capability::ReadOnly, cx);
            buffer
        })
    };

    let base_buffer = stages.base.clone().map(|text| make_buffer(text, cx));
    let ours_buffer = make_buffer(stages.ours.clone().unwrap_or_default(), cx);
    let theirs_buffer = make_buffer(stages.theirs.clone().unwrap_or_default(), cx);

    let base_text: Option<Arc<str>> = stages.base.as_deref().map(Arc::from);

    let ours_diff = match (&base_text, stages.ours.as_ref()) {
        (Some(base), Some(_)) => Some(build_diff(&ours_buffer, base.clone(), cx).await?),
        _ => None,
    };
    let theirs_diff = match (&base_text, stages.theirs.as_ref()) {
        (Some(base), Some(_)) => Some(build_diff(&theirs_buffer, base.clone(), cx).await?),
        _ => None,
    };

    Ok(StageBuffers {
        base_buffer,
        ours_buffer,
        theirs_buffer,
        ours_diff,
        theirs_diff,
    })
}

/// Builds a `BufferDiff` that compares `buffer`'s text against `base_text`.
/// Highlights in the resulting diff are "what this side changed from base".
async fn build_diff(
    buffer: &Entity<Buffer>,
    base_text: Arc<str>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let diff = cx.new(|cx| BufferDiff::new(&snapshot.text, cx));

    let update = diff
        .update(cx, |diff, cx| {
            diff.update_diff(
                snapshot.text.clone(),
                Some(base_text),
                Some(true),
                snapshot.language().cloned(),
                cx,
            )
        })
        .await;

    diff.update(cx, |diff, cx| diff.set_snapshot(update, &snapshot.text, cx))
        .await;

    Ok(diff)
}

impl EventEmitter<EditorEvent> for MergeEditor {}

impl Focusable for MergeEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.focused_inner_editor().focus_handle(cx)
    }
}

impl Render for MergeEditor {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ours_label = self.ours_branch_label.clone();
        let theirs_label = self.theirs_branch_label.clone();

        let ours_pane = v_flex()
            .flex_1()
            .size_full()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_side_pane_header(
                format!("Ours · {ours_label}").into(),
                AcceptSide::Ours,
                cx,
            ))
            .child(div().flex_1().size_full().child(self.ours_editor.clone()));

        let base_pane = self
            .base_visible
            .then(|| self.base_editor.clone())
            .flatten()
            .map(|editor| {
                v_flex()
                    .flex_1()
                    .size_full()
                    .border_r_1()
                    .border_color(cx.theme().colors().border)
                    .child(self.render_pane_header(SharedString::new_static("Base"), cx))
                    .child(div().flex_1().size_full().child(editor))
            });

        let result_pane = v_flex()
            .flex_1()
            .size_full()
            .border_r_1()
            .border_color(cx.theme().colors().border)
            .child(self.render_pane_header(SharedString::new_static("Result"), cx))
            .child(div().flex_1().size_full().child(self.result_editor.clone()));

        let theirs_pane = v_flex()
            .flex_1()
            .size_full()
            .child(self.render_side_pane_header(
                format!("Theirs · {theirs_label}").into(),
                AcceptSide::Theirs,
                cx,
            ))
            .child(div().flex_1().size_full().child(self.theirs_editor.clone()));

        v_flex()
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(self.render_header(cx))
            .child(
                h_flex()
                    .flex_1()
                    .size_full()
                    .child(ours_pane)
                    .when_some(base_pane, |this, base| this.child(base))
                    .child(result_pane)
                    .child(theirs_pane),
            )
    }
}

impl Item for MergeEditor {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::GitBranch).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, cx: &App) -> AnyElement {
        Label::new(self.tab_content_text(params.detail.unwrap_or_default(), cx))
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        let name = self
            .result_buffer
            .read(cx)
            .file()
            .and_then(|file| {
                file.full_path(cx)
                    .file_name()
                    .map(|n| n.to_string_lossy().into_owned())
            })
            .unwrap_or_else(|| "untitled".into());
        format!("Merge: {name}").into()
    }

    fn tab_tooltip_text(&self, cx: &App) -> Option<ui::SharedString> {
        let path = self
            .result_buffer
            .read(cx)
            .file()
            .map(|file| file.full_path(cx).compact().to_string_lossy().into_owned())
            .unwrap_or_else(|| "untitled".into());
        Some(format!("Merge: {path}").into())
    }

    fn to_item_events(event: &EditorEvent, f: &mut dyn FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Merge Editor Opened")
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        cx: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else {
            // Route to whichever pane the user most recently focused so vim,
            // inline assist, completion provider, and agent diff act on the
            // editor the user is actually looking at — not just the Result
            // pane.
            self.focused_inner_editor().act_as_type(type_id, cx)
        }
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        self.result_editor.for_each_project_item(cx, f)
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.result_editor.deactivated(window, cx);
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // The inline conflict resolver's `Use Ours` / `Use Theirs` / etc.
        // buttons require the editor's `workspace` handle to be populated —
        // `conflict_view::resolve_conflict` short-circuits when it's `None`.
        // Without this delegation, the result editor's workspace handle
        // never gets set and the buttons silently do nothing.
        self.result_editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx);
        });
    }

    fn navigate(
        &mut self,
        data: std::sync::Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.result_editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn can_save(&self, cx: &App) -> bool {
        self.result_editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Task<Result<()>> {
        self.result_editor.save(options, project, window, cx)
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.result_editor.read(cx).is_dirty(cx)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use git::{
        repository::{UnmergedStages, repo_path},
        status::{UnmergedStatus, UnmergedStatusCode},
    };
    use gpui::{BackgroundExecutor, TestAppContext};
    use project::Project;
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;
    use workspace::MultiWorkspace;

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
        });
    }

    #[gpui::test]
    async fn test_open_merge_editor(_: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "conflict.txt": "<<<<<<< HEAD\nour line\n=======\ntheir line\n>>>>>>> branch\n",
            }),
        )
        .await;

        fs.set_unmerged_paths_for_repo(
            path!("/project/.git").as_ref(),
            &[(
                repo_path("conflict.txt"),
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                },
            )],
        );

        fs.set_unmerged_stages_for_repo(
            path!("/project/.git").as_ref(),
            &[(
                repo_path("conflict.txt"),
                UnmergedStages {
                    base: Some("base line\n".into()),
                    ours: Some("our line\n".into()),
                    theirs: Some("their line\n".into()),
                },
            )],
        );

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            MultiWorkspace::test_new(project.clone(), window, cx)
        });
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.executor().run_until_parked();

        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let project_path = ProjectPath {
            worktree_id,
            path: util::rel_path::rel_path("conflict.txt").into(),
        };

        let merge_editor = workspace
            .update_in(cx, |workspace, window, cx| {
                MergeEditor::open(
                    project_path,
                    workspace.weak_handle(),
                    window,
                    cx,
                )
            })
            .await
            .unwrap();

        merge_editor.read_with(cx, |merge_editor, cx| {
            assert_eq!(
                MergeEditor::stage_buffer(&merge_editor.ours_editor, cx)
                    .read(cx)
                    .snapshot()
                    .text(),
                "our line\n",
            );
            assert_eq!(
                MergeEditor::stage_buffer(&merge_editor.theirs_editor, cx)
                    .read(cx)
                    .snapshot()
                    .text(),
                "their line\n",
            );
            let base_editor = merge_editor
                .base_editor
                .as_ref()
                .expect("base editor should be created when stage 1 is present");
            assert_eq!(
                MergeEditor::stage_buffer(base_editor, cx)
                    .read(cx)
                    .snapshot()
                    .text(),
                "base line\n",
            );
            assert!(!merge_editor.base_visible, "Base hidden by default");
            assert_eq!(
                merge_editor.tab_content_text(0, cx).as_ref(),
                "Merge: conflict.txt"
            );
        });

        // Focus routing: clicking into a side pane must redirect `focus_handle`
        // and `act_as_type::<Editor>()` so vim / inline-assist / contains_focused
        // operate on the editor the user is actually interacting with.
        merge_editor.update_in(cx, |this, window, cx| {
            let ours_focus = this.ours_editor.read(cx).focus_handle(cx);
            window.focus(&ours_focus, cx);
        });
        cx.executor().run_until_parked();

        merge_editor.read_with(cx, |merge_editor, cx| {
            let focused = merge_editor.focus_handle(cx);
            assert_eq!(
                focused,
                merge_editor.ours_editor.read(cx).focus_handle(cx),
                "focus_handle should redirect to the most recently focused inner editor"
            );
            assert_eq!(
                merge_editor.focused_inner_editor().entity_id(),
                merge_editor.ours_editor.entity_id(),
                "focused_inner_editor should track on_focus_in subscriptions"
            );
        });
    }

    #[gpui::test]
    async fn test_refuses_to_open_when_no_stages(
        _: BackgroundExecutor,
        cx: &mut TestAppContext,
    ) {
        init_test(cx);

        let fs = FakeFs::new(cx.background_executor.clone());
        fs.insert_tree(
            path!("/project"),
            json!({
                ".git": {},
                "binary.bin": "ignored",
            }),
        )
        .await;

        // Mark as unmerged but provide no stage contents — emulates a binary
        // conflict where `load_unmerged_stages` returns all-`None`.
        fs.set_unmerged_paths_for_repo(
            path!("/project/.git").as_ref(),
            &[(
                repo_path("binary.bin"),
                UnmergedStatus {
                    first_head: UnmergedStatusCode::Updated,
                    second_head: UnmergedStatusCode::Updated,
                },
            )],
        );

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let (multi_workspace, cx) = cx.add_window_view(|window, cx| {
            MultiWorkspace::test_new(project.clone(), window, cx)
        });
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());
        cx.executor().run_until_parked();

        let worktree_id = project.read_with(cx, |project, cx| {
            project.worktrees(cx).next().unwrap().read(cx).id()
        });
        let project_path = ProjectPath {
            worktree_id,
            path: util::rel_path::rel_path("binary.bin").into(),
        };

        let err = workspace
            .update_in(cx, |workspace, window, cx| {
                MergeEditor::open(project_path, workspace.weak_handle(), window, cx)
            })
            .await
            .expect_err("opening a stage-less unmerged path should error");
        assert!(
            err.to_string().contains("no text stages"),
            "unexpected error: {err}"
        );
    }
}
