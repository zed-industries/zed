use std::ops::Range;

use buffer_diff::BufferDiff;
use collections::HashMap;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use language::{Buffer, Capability};
use multi_buffer::{Anchor, ExcerptId, ExcerptRange, ExpandExcerptDirection, MultiBuffer, PathKey};
use project::Project;
use rope::Point;
use text::{Bias, OffsetRangeExt as _};
use ui::{
    App, Context, InteractiveElement as _, IntoElement as _, ParentElement as _, Render,
    Styled as _, Window, div,
};
use workspace::{
    ActivePaneDecorator, Item, ItemHandle, Pane, PaneGroup, SplitDirection, Workspace,
};

use crate::{Editor, EditorEvent};

struct SplitDiffFeatureFlag;

impl FeatureFlag for SplitDiffFeatureFlag {
    const NAME: &'static str = "split-diff";

    fn enabled_for_staff() -> bool {
        true
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
struct SplitDiff;

#[derive(Clone, Copy, PartialEq, Eq, Action, Default)]
#[action(namespace = editor)]
struct UnsplitDiff;

pub struct SplittableEditor {
    primary_multibuffer: Entity<MultiBuffer>,
    primary_editor: Entity<Editor>,
    secondary: Option<SecondaryEditor>,
    panes: PaneGroup,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

struct SecondaryEditor {
    multibuffer: Entity<MultiBuffer>,
    editor: Entity<Editor>,
    pane: Entity<Pane>,
    has_latest_selection: bool,
    primary_to_secondary: HashMap<ExcerptId, ExcerptId>,
    secondary_to_primary: HashMap<ExcerptId, ExcerptId>,
    _subscriptions: Vec<Subscription>,
}

impl SplittableEditor {
    pub fn primary_editor(&self) -> &Entity<Editor> {
        &self.primary_editor
    }

    pub fn last_selected_editor(&self) -> &Entity<Editor> {
        if let Some(secondary) = &self.secondary
            && secondary.has_latest_selection
        {
            &secondary.editor
        } else {
            &self.primary_editor
        }
    }

    pub fn new_unsplit(
        primary_multibuffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let primary_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                primary_multibuffer.clone(),
                Some(project.clone()),
                window,
                cx,
            );
            editor.set_expand_all_diff_hunks(cx);
            editor
        });
        let pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.downgrade(),
                project,
                Default::default(),
                None,
                NoAction.boxed_clone(),
                true,
                window,
                cx,
            );
            pane.set_should_display_tab_bar(|_, _| false);
            pane.add_item(primary_editor.boxed_clone(), true, true, None, window, cx);
            pane
        });
        let panes = PaneGroup::new(pane);
        // TODO(split-diff) we might want to tag editor events with whether they came from primary/secondary
        let subscriptions = vec![cx.subscribe(
            &primary_editor,
            |this, _, event: &EditorEvent, cx| match event {
                EditorEvent::ExpandExcerptsRequested {
                    excerpt_ids,
                    lines,
                    direction,
                } => {
                    this.expand_excerpts(excerpt_ids.iter().copied(), *lines, *direction, cx);
                }
                EditorEvent::SelectionsChanged { .. } => {
                    if let Some(secondary) = &mut this.secondary {
                        secondary.has_latest_selection = false;
                    }
                    cx.emit(event.clone());
                }
                _ => cx.emit(event.clone()),
            },
        )];

        window.defer(cx, {
            let workspace = workspace.downgrade();
            let primary_editor = primary_editor.downgrade();
            move |window, cx| {
                workspace
                    .update(cx, |workspace, cx| {
                        primary_editor.update(cx, |editor, cx| {
                            editor.added_to_workspace(workspace, window, cx);
                        })
                    })
                    .ok();
            }
        });
        Self {
            primary_editor,
            primary_multibuffer,
            secondary: None,
            panes,
            workspace: workspace.downgrade(),
            _subscriptions: subscriptions,
        }
    }

    fn split(&mut self, _: &SplitDiff, window: &mut Window, cx: &mut Context<Self>) {
        if !cx.has_flag::<SplitDiffFeatureFlag>() {
            return;
        }
        if self.secondary.is_some() {
            return;
        }
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };
        let project = workspace.read(cx).project().clone();

        let secondary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadOnly);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let secondary_editor = cx.new(|cx| {
            let mut editor = Editor::for_multibuffer(
                secondary_multibuffer.clone(),
                Some(project.clone()),
                window,
                cx,
            );
            editor.number_deleted_lines = true;
            editor.set_delegate_expand_excerpts(true);
            editor
        });
        let secondary_pane = cx.new(|cx| {
            let mut pane = Pane::new(
                workspace.downgrade(),
                workspace.read(cx).project().clone(),
                Default::default(),
                None,
                NoAction.boxed_clone(),
                true,
                window,
                cx,
            );
            pane.set_should_display_tab_bar(|_, _| false);
            pane.add_item(
                ItemHandle::boxed_clone(&secondary_editor),
                false,
                false,
                None,
                window,
                cx,
            );
            pane
        });

        let subscriptions = vec![cx.subscribe(
            &secondary_editor,
            |this, _, event: &EditorEvent, cx| match event {
                EditorEvent::ExpandExcerptsRequested {
                    excerpt_ids,
                    lines,
                    direction,
                } => {
                    if let Some(secondary) = &this.secondary {
                        let primary_ids: Vec<_> = excerpt_ids
                            .iter()
                            .filter_map(|id| secondary.secondary_to_primary.get(id).copied())
                            .collect();
                        this.expand_excerpts(primary_ids.into_iter(), *lines, *direction, cx);
                    }
                }
                EditorEvent::SelectionsChanged { .. } => {
                    if let Some(secondary) = &mut this.secondary {
                        secondary.has_latest_selection = true;
                    }
                    cx.emit(event.clone());
                }
                _ => cx.emit(event.clone()),
            },
        )];
        let mut secondary = SecondaryEditor {
            editor: secondary_editor,
            multibuffer: secondary_multibuffer,
            pane: secondary_pane.clone(),
            has_latest_selection: false,
            primary_to_secondary: HashMap::default(),
            secondary_to_primary: HashMap::default(),
            _subscriptions: subscriptions,
        };
        self.primary_editor.update(cx, |editor, cx| {
            editor.set_delegate_expand_excerpts(true);
            editor.buffer().update(cx, |primary_multibuffer, cx| {
                primary_multibuffer.set_show_deleted_hunks(false, cx);
                let paths = primary_multibuffer.paths().cloned().collect::<Vec<_>>();
                for path in paths {
                    let Some(excerpt_id) = primary_multibuffer.excerpts_for_path(&path).next()
                    else {
                        continue;
                    };
                    let snapshot = primary_multibuffer.snapshot(cx);
                    let buffer = snapshot.buffer_for_excerpt(excerpt_id).unwrap();
                    let diff = primary_multibuffer.diff_for(buffer.remote_id()).unwrap();
                    secondary.sync_path_excerpts(path.clone(), primary_multibuffer, diff, cx);
                }
            })
        });
        self.secondary = Some(secondary);

        let primary_pane = self.panes.first_pane();
        self.panes
            .split(&primary_pane, &secondary_pane, SplitDirection::Left, cx)
            .unwrap();
        cx.notify();
    }

    fn unsplit(&mut self, _: &UnsplitDiff, _: &mut Window, cx: &mut Context<Self>) {
        let Some(secondary) = self.secondary.take() else {
            return;
        };
        self.panes.remove(&secondary.pane, cx).unwrap();
        self.primary_editor.update(cx, |primary, cx| {
            primary.set_delegate_expand_excerpts(false);
            primary.buffer().update(cx, |buffer, cx| {
                buffer.set_show_deleted_hunks(true, cx);
            });
        });
        cx.notify();
    }

    pub fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.workspace = workspace.weak_handle();
        self.primary_editor.update(cx, |primary_editor, cx| {
            primary_editor.added_to_workspace(workspace, window, cx);
        });
        if let Some(secondary) = &self.secondary {
            secondary.editor.update(cx, |secondary_editor, cx| {
                secondary_editor.added_to_workspace(workspace, window, cx);
            });
        }
    }

    pub fn set_excerpts_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>> + Clone,
        context_line_count: u32,
        diff: Entity<BufferDiff>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        self.primary_multibuffer
            .update(cx, |primary_multibuffer, cx| {
                let (anchors, added_a_new_excerpt) = primary_multibuffer.set_excerpts_for_path(
                    path.clone(),
                    buffer.clone(),
                    ranges,
                    context_line_count,
                    cx,
                );
                if !anchors.is_empty()
                    && primary_multibuffer
                        .diff_for(buffer.read(cx).remote_id())
                        .is_none_or(|old_diff| old_diff.entity_id() != diff.entity_id())
                {
                    primary_multibuffer.add_diff(diff.clone(), cx);
                }
                if let Some(secondary) = &mut self.secondary {
                    secondary.sync_path_excerpts(path, primary_multibuffer, diff, cx);
                }
                (anchors, added_a_new_excerpt)
            })
    }

    fn expand_excerpts(
        &mut self,
        excerpt_ids: impl Iterator<Item = ExcerptId> + Clone,
        lines: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        let mut corresponding_paths = HashMap::default();
        self.primary_multibuffer.update(cx, |multibuffer, cx| {
            let snapshot = multibuffer.snapshot(cx);
            if self.secondary.is_some() {
                corresponding_paths = excerpt_ids
                    .clone()
                    .map(|excerpt_id| {
                        let path = multibuffer.path_for_excerpt(excerpt_id).unwrap();
                        let buffer = snapshot.buffer_for_excerpt(excerpt_id).unwrap();
                        let diff = multibuffer.diff_for(buffer.remote_id()).unwrap();
                        (path, diff)
                    })
                    .collect::<HashMap<_, _>>();
            }
            multibuffer.expand_excerpts(excerpt_ids.clone(), lines, direction, cx);
        });

        if let Some(secondary) = &mut self.secondary {
            self.primary_multibuffer.update(cx, |multibuffer, cx| {
                for (path, diff) in corresponding_paths {
                    secondary.sync_path_excerpts(path, multibuffer, diff, cx);
                }
            })
        }
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        self.primary_multibuffer.update(cx, |buffer, cx| {
            buffer.remove_excerpts_for_path(path.clone(), cx)
        });
        if let Some(secondary) = &mut self.secondary {
            secondary.remove_mappings_for_path(&path, cx);
            secondary
                .multibuffer
                .update(cx, |buffer, cx| buffer.remove_excerpts_for_path(path, cx))
        }
    }
}

#[cfg(test)]
impl SplittableEditor {
    fn check_invariants(&self, quiesced: bool, cx: &App) {
        use buffer_diff::DiffHunkStatusKind;
        use collections::HashSet;
        use multi_buffer::MultiBufferOffset;
        use multi_buffer::MultiBufferRow;
        use multi_buffer::MultiBufferSnapshot;

        fn format_diff(snapshot: &MultiBufferSnapshot) -> String {
            let text = snapshot.text();
            let row_infos = snapshot.row_infos(MultiBufferRow(0)).collect::<Vec<_>>();
            let boundary_rows = snapshot
                .excerpt_boundaries_in_range(MultiBufferOffset(0)..)
                .map(|b| b.row)
                .collect::<HashSet<_>>();

            text.split('\n')
                .enumerate()
                .zip(row_infos)
                .map(|((ix, line), info)| {
                    let marker = match info.diff_status.map(|status| status.kind) {
                        Some(DiffHunkStatusKind::Added) => "+ ",
                        Some(DiffHunkStatusKind::Deleted) => "- ",
                        Some(DiffHunkStatusKind::Modified) => unreachable!(),
                        None => {
                            if !line.is_empty() {
                                "  "
                            } else {
                                ""
                            }
                        }
                    };
                    let boundary_row = if boundary_rows.contains(&MultiBufferRow(ix as u32)) {
                        "  ----------\n"
                    } else {
                        ""
                    };
                    let expand = info
                        .expand_info
                        .map(|expand_info| match expand_info.direction {
                            ExpandExcerptDirection::Up => " [↑]",
                            ExpandExcerptDirection::Down => " [↓]",
                            ExpandExcerptDirection::UpAndDown => " [↕]",
                        })
                        .unwrap_or_default();

                    format!("{boundary_row}{marker}{line}{expand}")
                })
                .collect::<Vec<_>>()
                .join("\n")
        }

        let Some(secondary) = &self.secondary else {
            return;
        };

        log::info!(
            "primary:\n\n{}",
            format_diff(&self.primary_multibuffer.read(cx).snapshot(cx))
        );

        log::info!(
            "secondary:\n\n{}",
            format_diff(&secondary.multibuffer.read(cx).snapshot(cx))
        );

        let primary_excerpts = self.primary_multibuffer.read(cx).excerpt_ids();
        let secondary_excerpts = secondary.multibuffer.read(cx).excerpt_ids();
        assert_eq!(primary_excerpts.len(), secondary_excerpts.len());

        assert_eq!(
            secondary.primary_to_secondary.len(),
            primary_excerpts.len(),
            "primary_to_secondary mapping count should match excerpt count"
        );
        assert_eq!(
            secondary.secondary_to_primary.len(),
            secondary_excerpts.len(),
            "secondary_to_primary mapping count should match excerpt count"
        );

        for primary_id in &primary_excerpts {
            assert!(
                secondary.primary_to_secondary.contains_key(primary_id),
                "primary excerpt {:?} should have a mapping to secondary",
                primary_id
            );
        }
        for secondary_id in &secondary_excerpts {
            assert!(
                secondary.secondary_to_primary.contains_key(secondary_id),
                "secondary excerpt {:?} should have a mapping to primary",
                secondary_id
            );
        }

        for (primary_id, secondary_id) in &secondary.primary_to_secondary {
            assert_eq!(
                secondary.secondary_to_primary.get(secondary_id),
                Some(primary_id),
                "mappings should be bijective"
            );
        }

        if quiesced {
            let primary_snapshot = self.primary_multibuffer.read(cx).snapshot(cx);
            let secondary_snapshot = secondary.multibuffer.read(cx).snapshot(cx);
            let primary_diff_hunks = primary_snapshot
                .diff_hunks()
                .map(|hunk| hunk.diff_base_byte_range)
                .collect::<Vec<_>>();
            let secondary_diff_hunks = secondary_snapshot
                .diff_hunks()
                .map(|hunk| hunk.diff_base_byte_range)
                .collect::<Vec<_>>();
            pretty_assertions::assert_eq!(primary_diff_hunks, secondary_diff_hunks);

            // Filtering out empty lines is a bit of a hack, to work around a case where
            // the base text has a trailing newline but the current text doesn't, or vice versa.
            // In this case, we get the additional newline on one side, but that line is not
            // marked as added/deleted by rowinfos.
            let primary_unmodified_rows = primary_snapshot
                .text()
                .split("\n")
                .zip(primary_snapshot.row_infos(MultiBufferRow(0)))
                .filter(|(line, row_info)| !line.is_empty() && row_info.diff_status.is_none())
                .map(|(line, _)| line.to_owned())
                .collect::<Vec<_>>();
            let secondary_unmodified_rows = secondary_snapshot
                .text()
                .split("\n")
                .zip(secondary_snapshot.row_infos(MultiBufferRow(0)))
                .filter(|(line, row_info)| !line.is_empty() && row_info.diff_status.is_none())
                .map(|(line, _)| line.to_owned())
                .collect::<Vec<_>>();
            pretty_assertions::assert_eq!(primary_unmodified_rows, secondary_unmodified_rows);
        }
    }

    fn randomly_edit_excerpts(
        &mut self,
        rng: &mut impl rand::Rng,
        mutation_count: usize,
        cx: &mut Context<Self>,
    ) {
        use collections::HashSet;
        use rand::prelude::*;
        use std::env;
        use util::RandomCharIter;

        let max_excerpts = env::var("MAX_EXCERPTS")
            .map(|i| i.parse().expect("invalid `MAX_EXCERPTS` variable"))
            .unwrap_or(5);

        for _ in 0..mutation_count {
            let paths = self
                .primary_multibuffer
                .read(cx)
                .paths()
                .cloned()
                .collect::<Vec<_>>();
            let excerpt_ids = self.primary_multibuffer.read(cx).excerpt_ids();

            if rng.random_bool(0.1) && !excerpt_ids.is_empty() {
                let mut excerpts = HashSet::default();
                for _ in 0..rng.random_range(0..excerpt_ids.len()) {
                    excerpts.extend(excerpt_ids.choose(rng).copied());
                }

                let line_count = rng.random_range(0..5);

                log::info!("Expanding excerpts {excerpts:?} by {line_count} lines");

                self.expand_excerpts(
                    excerpts.iter().cloned(),
                    line_count,
                    ExpandExcerptDirection::UpAndDown,
                    cx,
                );
                continue;
            }

            if excerpt_ids.is_empty() || (rng.random() && excerpt_ids.len() < max_excerpts) {
                let len = rng.random_range(100..500);
                let text = RandomCharIter::new(&mut *rng).take(len).collect::<String>();
                let buffer = cx.new(|cx| Buffer::local(text, cx));
                log::info!(
                    "Creating new buffer {} with text: {:?}",
                    buffer.read(cx).remote_id(),
                    buffer.read(cx).text()
                );
                let buffer_snapshot = buffer.read(cx).snapshot();
                let diff = cx.new(|cx| BufferDiff::new_unchanged(&buffer_snapshot, cx));
                // Create some initial diff hunks.
                buffer.update(cx, |buffer, cx| {
                    buffer.randomly_edit(rng, 1, cx);
                });
                let buffer_snapshot = buffer.read(cx).text_snapshot();
                let ranges = diff.update(cx, |diff, cx| {
                    diff.recalculate_diff_sync(&buffer_snapshot, cx);
                    diff.snapshot(cx)
                        .hunks(&buffer_snapshot)
                        .map(|hunk| hunk.buffer_range.to_point(&buffer_snapshot))
                        .collect::<Vec<_>>()
                });
                let path = PathKey::for_buffer(&buffer, cx);
                self.set_excerpts_for_path(path, buffer, ranges, 2, diff, cx);
            } else {
                let remove_count = rng.random_range(1..=paths.len());
                let paths_to_remove = paths
                    .choose_multiple(rng, remove_count)
                    .cloned()
                    .collect::<Vec<_>>();
                for path in paths_to_remove {
                    self.remove_excerpts_for_path(path.clone(), cx);
                }
            }
        }
    }
}

impl EventEmitter<EditorEvent> for SplittableEditor {}
impl Focusable for SplittableEditor {
    fn focus_handle(&self, cx: &App) -> gpui::FocusHandle {
        self.primary_editor.read(cx).focus_handle(cx)
    }
}

impl Render for SplittableEditor {
    fn render(
        &mut self,
        window: &mut ui::Window,
        cx: &mut ui::Context<Self>,
    ) -> impl ui::IntoElement {
        let inner = if self.secondary.is_none() {
            self.primary_editor.clone().into_any_element()
        } else if let Some(active) = self.panes.panes().into_iter().next() {
            self.panes
                .render(
                    None,
                    &ActivePaneDecorator::new(active, &self.workspace),
                    window,
                    cx,
                )
                .into_any_element()
        } else {
            div().into_any_element()
        };
        div()
            .id("splittable-editor")
            .on_action(cx.listener(Self::split))
            .on_action(cx.listener(Self::unsplit))
            .size_full()
            .child(inner)
    }
}

impl SecondaryEditor {
    fn sync_path_excerpts(
        &mut self,
        path_key: PathKey,
        primary_multibuffer: &mut MultiBuffer,
        diff: Entity<BufferDiff>,
        cx: &mut App,
    ) {
        let Some(excerpt_id) = primary_multibuffer.excerpts_for_path(&path_key).next() else {
            self.remove_mappings_for_path(&path_key, cx);
            self.multibuffer.update(cx, |multibuffer, cx| {
                multibuffer.remove_excerpts_for_path(path_key, cx);
            });
            return;
        };

        let primary_excerpt_ids: Vec<ExcerptId> =
            primary_multibuffer.excerpts_for_path(&path_key).collect();

        let primary_multibuffer_snapshot = primary_multibuffer.snapshot(cx);
        let main_buffer = primary_multibuffer_snapshot
            .buffer_for_excerpt(excerpt_id)
            .unwrap();
        let base_text_buffer = diff.read(cx).base_text_buffer();
        let diff_snapshot = diff.read(cx).snapshot(cx);
        let base_text_buffer_snapshot = base_text_buffer.read(cx).snapshot();
        let new = primary_multibuffer
            .excerpts_for_buffer(main_buffer.remote_id(), cx)
            .into_iter()
            .map(|(_, excerpt_range)| {
                let point_range_to_base_text_point_range = |range: Range<Point>| {
                    let start_row = diff_snapshot.row_to_base_text_row(
                        range.start.row,
                        Bias::Left,
                        main_buffer,
                    );
                    let end_row =
                        diff_snapshot.row_to_base_text_row(range.end.row, Bias::Right, main_buffer);
                    let end_column = diff_snapshot.base_text().line_len(end_row);
                    Point::new(start_row, 0)..Point::new(end_row, end_column)
                };
                let primary = excerpt_range.primary.to_point(main_buffer);
                let context = excerpt_range.context.to_point(main_buffer);
                ExcerptRange {
                    primary: point_range_to_base_text_point_range(primary),
                    context: point_range_to_base_text_point_range(context),
                }
            })
            .collect();

        let main_buffer = primary_multibuffer.buffer(main_buffer.remote_id()).unwrap();

        self.remove_mappings_for_path(&path_key, cx);

        self.editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                let (ids, _) = buffer.update_path_excerpts(
                    path_key.clone(),
                    base_text_buffer.clone(),
                    &base_text_buffer_snapshot,
                    new,
                    cx,
                );
                if !ids.is_empty()
                    && buffer
                        .diff_for(base_text_buffer.read(cx).remote_id())
                        .is_none_or(|old_diff| old_diff.entity_id() != diff.entity_id())
                {
                    buffer.add_inverted_diff(diff, main_buffer, cx);
                }
            })
        });

        let secondary_excerpt_ids: Vec<ExcerptId> = self
            .multibuffer
            .read(cx)
            .excerpts_for_path(&path_key)
            .collect();

        for (primary_id, secondary_id) in primary_excerpt_ids.into_iter().zip(secondary_excerpt_ids)
        {
            self.primary_to_secondary.insert(primary_id, secondary_id);
            self.secondary_to_primary.insert(secondary_id, primary_id);
        }
    }

    fn remove_mappings_for_path(&mut self, path_key: &PathKey, cx: &App) {
        let secondary_excerpt_ids: Vec<ExcerptId> = self
            .multibuffer
            .read(cx)
            .excerpts_for_path(path_key)
            .collect();

        for secondary_id in secondary_excerpt_ids {
            if let Some(primary_id) = self.secondary_to_primary.remove(&secondary_id) {
                self.primary_to_secondary.remove(&primary_id);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use fs::FakeFs;
    use gpui::AppContext as _;
    use language::Capability;
    use multi_buffer::{MultiBuffer, PathKey};
    use project::Project;
    use rand::rngs::StdRng;
    use settings::SettingsStore;
    use ui::VisualContext as _;
    use workspace::Workspace;

    use crate::SplittableEditor;

    fn init_test(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
    }

    #[ignore]
    #[gpui::test(iterations = 100)]
    async fn test_random_split_editor(mut rng: StdRng, cx: &mut gpui::TestAppContext) {
        use rand::prelude::*;

        init_test(cx);
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let primary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
            multibuffer.set_all_diff_hunks_expanded(cx);
            multibuffer
        });
        let editor = cx.new_window_entity(|window, cx| {
            let mut editor =
                SplittableEditor::new_unsplit(primary_multibuffer, project, workspace, window, cx);
            editor.split(&Default::default(), window, cx);
            editor
        });

        let operations = std::env::var("OPERATIONS")
            .map(|i| i.parse().expect("invalid `OPERATIONS` variable"))
            .unwrap_or(20);
        let rng = &mut rng;
        for _ in 0..operations {
            editor.update(cx, |editor, cx| {
                let buffers = editor
                    .primary_editor
                    .read(cx)
                    .buffer()
                    .read(cx)
                    .all_buffers();

                if buffers.is_empty() {
                    editor.randomly_edit_excerpts(rng, 2, cx);
                    editor.check_invariants(true, cx);
                    return;
                }

                let quiesced = match rng.random_range(0..100) {
                    0..=69 if !buffers.is_empty() => {
                        let buffer = buffers.iter().choose(rng).unwrap();
                        buffer.update(cx, |buffer, cx| {
                            if rng.random() {
                                log::info!("randomly editing single buffer");
                                buffer.randomly_edit(rng, 5, cx);
                            } else {
                                log::info!("randomly undoing/redoing in single buffer");
                                buffer.randomly_undo_redo(rng, cx);
                            }
                        });
                        false
                    }
                    70..=79 => {
                        log::info!("mutating excerpts");
                        editor.randomly_edit_excerpts(rng, 2, cx);
                        false
                    }
                    80..=89 if !buffers.is_empty() => {
                        log::info!("recalculating buffer diff");
                        let buffer = buffers.iter().choose(rng).unwrap();
                        let diff = editor
                            .primary_multibuffer
                            .read(cx)
                            .diff_for(buffer.read(cx).remote_id())
                            .unwrap();
                        let buffer_snapshot = buffer.read(cx).text_snapshot();
                        diff.update(cx, |diff, cx| {
                            diff.recalculate_diff_sync(&buffer_snapshot, cx);
                        });
                        false
                    }
                    _ => {
                        log::info!("quiescing");
                        for buffer in buffers {
                            let buffer_snapshot = buffer.read(cx).text_snapshot();
                            let diff = editor
                                .primary_multibuffer
                                .read(cx)
                                .diff_for(buffer.read(cx).remote_id())
                                .unwrap();
                            diff.update(cx, |diff, cx| {
                                diff.recalculate_diff_sync(&buffer_snapshot, cx);
                            });
                            let diff_snapshot = diff.read(cx).snapshot(cx);
                            let ranges = diff_snapshot
                                .hunks(&buffer_snapshot)
                                .map(|hunk| hunk.range)
                                .collect::<Vec<_>>();
                            let path = PathKey::for_buffer(&buffer, cx);
                            editor.set_excerpts_for_path(path, buffer, ranges, 2, diff, cx);
                        }
                        true
                    }
                };

                editor.check_invariants(quiesced, cx);
            });
        }
    }
}
