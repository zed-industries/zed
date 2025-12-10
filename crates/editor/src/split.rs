use std::ops::Range;

use buffer_diff::BufferDiff;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use language::{Buffer, Capability};
use multi_buffer::{Anchor, ExcerptId, ExcerptRange, ExpandExcerptDirection, MultiBuffer, PathKey};
use project::Project;
use rope::Point;
use text::OffsetRangeExt as _;
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
            Editor::for_multibuffer(
                primary_multibuffer.clone(),
                Some(project.clone()),
                window,
                cx,
            )
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
        let subscriptions =
            vec![
                cx.subscribe(&primary_editor, |this, _, event: &EditorEvent, cx| {
                    if let EditorEvent::SelectionsChanged { .. } = event
                        && let Some(secondary) = &mut this.secondary
                    {
                        secondary.has_latest_selection = false;
                    }
                    cx.emit(event.clone())
                }),
            ];

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

        let subscriptions =
            vec![
                cx.subscribe(&secondary_editor, |this, _, event: &EditorEvent, cx| {
                    if let EditorEvent::SelectionsChanged { .. } = event
                        && let Some(secondary) = &mut this.secondary
                    {
                        secondary.has_latest_selection = true;
                    }
                    cx.emit(event.clone())
                }),
            ];
        let mut secondary = SecondaryEditor {
            editor: secondary_editor,
            multibuffer: secondary_multibuffer,
            pane: secondary_pane.clone(),
            has_latest_selection: false,
            _subscriptions: subscriptions,
        };
        self.primary_editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |primary_multibuffer, cx| {
                primary_multibuffer.set_show_deleted_hunks(false, cx);
                let paths = primary_multibuffer.paths().collect::<Vec<_>>();
                for path in paths {
                    let Some(excerpt_id) = primary_multibuffer.excerpts_for_path(&path).next()
                    else {
                        continue;
                    };
                    let snapshot = primary_multibuffer.snapshot(cx);
                    let buffer = snapshot.buffer_for_excerpt(excerpt_id).unwrap();
                    let diff = primary_multibuffer.diff_for(buffer.remote_id()).unwrap();
                    secondary.sync_path_excerpts(path, primary_multibuffer, diff, cx);
                }
            })
        });
        self.secondary = Some(secondary);

        let primary_pane = self.panes.first_pane();
        self.panes
            .split(&primary_pane, &secondary_pane, SplitDirection::Left)
            .unwrap();
        cx.notify();
    }

    fn unsplit(&mut self, _: &UnsplitDiff, _: &mut Window, cx: &mut Context<Self>) {
        let Some(secondary) = self.secondary.take() else {
            return;
        };
        self.panes.remove(&secondary.pane).unwrap();
        self.primary_editor.update(cx, |primary, cx| {
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
        ranges: impl IntoIterator<Item = Range<Point>>,
        context_line_count: u32,
        diff: Entity<BufferDiff>,
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        self.primary_editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |primary_multibuffer, cx| {
                let (anchors, added_a_new_excerpt) = primary_multibuffer.set_excerpts_for_path(
                    path.clone(),
                    buffer,
                    ranges,
                    context_line_count,
                    cx,
                );
                primary_multibuffer.add_diff(diff.clone(), cx);
                if let Some(secondary) = &mut self.secondary {
                    secondary.sync_path_excerpts(path, primary_multibuffer, diff, cx);
                }
                (anchors, added_a_new_excerpt)
            })
        })
    }

    /// Expands excerpts in both sides.
    ///
    /// While the left multibuffer does have separate excerpts with separate
    /// IDs, this is an implementation detail. We do not expose the left excerpt
    /// IDs in the public API of [`SplittableEditor`].
    pub fn expand_excerpts(
        &mut self,
        excerpt_ids: impl Iterator<Item = ExcerptId> + Clone,
        lines: u32,
        direction: ExpandExcerptDirection,
        cx: &mut Context<Self>,
    ) {
        self.primary_multibuffer.update(cx, |multibuffer, cx| {
            multibuffer.expand_excerpts(excerpt_ids.clone(), lines, direction, cx);
        });
        let paths: Vec<(ExcerptId, PathKey)> = excerpt_ids
            .flat_map(|excerpt_id| {
                let path = self
                    .primary_multibuffer
                    .read(cx)
                    .path_for_excerpt(excerpt_id)
                    .cloned()?;
                Some((excerpt_id, path))
            })
            .collect();

        if let Some(secondary) = &mut self.secondary {
            self.primary_editor.update(cx, |editor, cx| {
                editor.buffer().update(cx, |multibuffer, cx| {
                    let snapshot = multibuffer.snapshot(cx);
                    for (excerpt_id, path) in paths {
                        let buffer = snapshot.buffer_for_excerpt(excerpt_id).unwrap();
                        let diff = multibuffer.diff_for(buffer.remote_id()).unwrap();
                        secondary.sync_path_excerpts(path, multibuffer, diff, cx);
                    }
                })
            })
        }
    }

    pub fn remove_excerpts_for_path(&mut self, path: PathKey, cx: &mut Context<Self>) {
        self.primary_multibuffer.update(cx, |buffer, cx| {
            buffer.remove_excerpts_for_path(path.clone(), cx)
        });
        if let Some(secondary) = &self.secondary {
            secondary
                .multibuffer
                .update(cx, |buffer, cx| buffer.remove_excerpts_for_path(path, cx))
        }
    }
}

#[cfg(test)]
impl SplittableEditor {
    fn check_invariants(&self, cx: &App) {
        let Some(secondary) = &self.secondary else {
            return;
        };

        let primary_excerpts = self.primary_multibuffer.read(cx).excerpt_ids();
        let secondary_excerpts = secondary.multibuffer.read(cx).excerpt_ids();
        assert_eq!(primary_excerpts.len(), secondary_excerpts.len());

        // self.primary_multibuffer.read(cx).check_invariants(cx);
        // secondary.multibuffer.read(cx).check_invariants(cx);
        // Assertions:...
        //
        // left.display_lines().filter(is_unmodified) == right.display_lines().filter(is_unmodified)
        //
        // left excerpts and right excerpts bijectivity
        //
        //

        // let primary_buffer_text = self
        //     .primary_multibuffer
        //     .read(cx)
        //     .text_summary_for_range(Anchor::min()..Anchor::max());
        // let secondary_buffer_text = secondary
        //     .multibuffer
        //     .read(cx)
        //     .text_summary_for_range(Anchor::min()..Anchor::max());
        // let primary_buffer_base_text = self
        //     .primary_multibuffer
        //     .read(cx)
        //     .base_text_summary_for_range(Anchor::min()..Anchor::max());
        // let secondary_buffer_base_text = secondary
        //     .multibuffer
        //     .read(cx)
        //     .base_text_summary_for_range(Anchor::min()..Anchor::max());
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

        let paths = self
            .primary_multibuffer
            .read(cx)
            .paths()
            .collect::<Vec<_>>();
        let excerpt_ids = self.primary_multibuffer.read(cx).excerpt_ids();

        for _ in 0..mutation_count {
            if rng.random_bool(0.05) {
                log::info!("Clearing multi-buffer");
                self.primary_multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer.clear(cx);
                });
                continue;
            } else if rng.random_bool(0.1) && !excerpt_ids.is_empty() {
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
                let existing_buffers = self.primary_multibuffer.read(cx).all_buffers();
                let buffer = if rng.random() || existing_buffers.is_empty() {
                    let len = rng.random_range(0..500);
                    let text = RandomCharIter::new(&mut *rng).take(len).collect::<String>();
                    let buffer = cx.new(|cx| Buffer::local(text, cx));
                    log::info!(
                        "Creating new buffer {} with text: {:?}",
                        buffer.read(cx).remote_id(),
                        buffer.read(cx).text()
                    );
                    buffer
                } else {
                    existing_buffers.iter().choose(rng).unwrap().clone()
                };

                let buffer_snapshot = buffer.read(cx).snapshot();
                let diff = cx.new(|cx| BufferDiff::new_unchanged(&buffer_snapshot, cx));
                // Create some initial diff hunks.
                buffer.update(cx, |buffer, cx| {
                    buffer.randomly_edit(rng, 2, cx);
                });
                let buffer_snapshot = buffer.read(cx).text_snapshot();
                let ranges = diff.update(cx, |diff, cx| {
                    diff.recalculate_diff_sync(&buffer_snapshot, cx);
                    diff.snapshot(cx)
                        .hunks(&buffer_snapshot)
                        .map(|hunk| hunk.range.clone())
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
                    self.remove_excerpts_for_path(path, cx);
                }
            }
        }
    }

    fn randomly_mutate(
        &mut self,
        rng: &mut impl rand::Rng,
        mutation_count: usize,
        cx: &mut Context<Self>,
    ) {
        use rand::prelude::*;

        if rng.random_bool(0.7) {
            let buffers = self.primary_editor.read(cx).buffer().read(cx).all_buffers();
            let buffer = buffers.iter().choose(rng);

            if let Some(buffer) = buffer {
                buffer.update(cx, |buffer, cx| {
                    if rng.random() {
                        buffer.randomly_edit(rng, mutation_count, cx);
                    } else {
                        buffer.randomly_undo_redo(rng, cx);
                    }
                });
            } else {
                self.primary_multibuffer.update(cx, |multibuffer, cx| {
                    multibuffer.randomly_edit(rng, mutation_count, cx);
                });
            }
        } else if rng.random() {
            self.randomly_edit_excerpts(rng, mutation_count, cx);
        } else {
            for buffer in self.primary_multibuffer.read(cx).all_buffers() {
                let diff = self
                    .primary_multibuffer
                    .read(cx)
                    .diff_for(buffer.read(cx).remote_id())
                    .unwrap();
                let buffer_snapshot = buffer.read(cx).text_snapshot();
                diff.update(cx, |diff, cx| {
                    diff.recalculate_diff_sync(&buffer_snapshot, cx);
                });
                // TODO(split-diff) might be a good idea to try to separate the diff recalculation from the excerpt recalculation
                let diff_snapshot = diff.read(cx).snapshot(cx);
                let ranges = diff_snapshot
                    .hunks(&buffer_snapshot)
                    .map(|hunk| hunk.range.clone());
                let path = PathKey::for_buffer(&buffer, cx);
                self.set_excerpts_for_path(path, buffer, ranges, 2, diff, cx);
            }
        }

        self.check_invariants(cx);
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
        let excerpt_id = primary_multibuffer
            .excerpts_for_path(&path_key)
            .next()
            .unwrap();
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
                    let start_row =
                        diff_snapshot.row_to_base_text_row(range.start.row, main_buffer);
                    let end_row = diff_snapshot.row_to_base_text_row(range.end.row, main_buffer);
                    let end_column = diff_snapshot.base_text().line_len(end_row);
                    Point::new(start_row, 0)..Point::new(end_row, end_column)
                };
                let primary = excerpt_range.primary.to_point(main_buffer);
                let context = excerpt_range.context.to_point(main_buffer);
                ExcerptRange {
                    primary: point_range_to_base_text_point_range(dbg!(primary)),
                    context: point_range_to_base_text_point_range(dbg!(context)),
                }
            })
            .collect();

        let main_buffer = primary_multibuffer.buffer(main_buffer.remote_id()).unwrap();

        self.editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.update_path_excerpts(
                    path_key,
                    base_text_buffer,
                    &base_text_buffer_snapshot,
                    new,
                    cx,
                );
                buffer.add_inverted_diff(diff, main_buffer, cx);
            })
        });
    }
}

#[cfg(test)]
mod tests {
    use buffer_diff::BufferDiff;
    use db::indoc;
    use fs::FakeFs;
    use gpui::AppContext as _;
    use language::{Buffer, Capability};
    use multi_buffer::MultiBuffer;
    use project::Project;
    use rand::{Rng, rngs::StdRng};
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

    #[gpui::test]
    async fn test_basic_excerpts(mut rng: StdRng, cx: &mut gpui::TestAppContext) {
        init_test(cx);
        let base_text = indoc! {"
            hello
        "};
        let buffer_text = indoc! {"
            HELLO!
        "};
        let buffer = cx.new(|cx| Buffer::local(buffer_text, cx));
        let diff = cx.new(|cx| {
            BufferDiff::new_with_base_text(base_text, &buffer.read(cx).text_snapshot(), cx)
        });
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        let editor = cx.new_window_entity(|window, cx| {
            SplittableEditor::new_unsplit(multibuffer, project, workspace, window, cx)
        });

        let mutation_count = rng.random_range(0..100);
        editor.update(cx, |editor, cx| {
            editor.randomly_mutate(&mut rng, mutation_count, cx);
        })

        // for _ in 0..random() {
        //     editor.update(cx, |editor, cx| {
        //         randomly_mutate(primary_multibuffer);
        //         editor.primary_editor().update(cx, |editor, cx| {
        //             editor.edit(vec![(random()..random(), "...")], cx);
        //         })
        //     });
        // }

        // editor.read(cx).primary_editor().read(cx).display_map.read(cx)
    }
}
