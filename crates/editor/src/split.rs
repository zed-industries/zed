use std::ops::Range;

use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use collections::HashMap;
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use language::{Buffer, BufferSnapshot, Capability};
use multi_buffer::{Anchor, ExcerptRange, MultiBuffer, MultiBufferSnapshot, PathKey};
use project::Project;
use rope::Point;
use text::{BufferId, OffsetRangeExt as _};
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
    primary_editor: Entity<Editor>,
    secondary: Option<SecondaryEditor>,
    panes: PaneGroup,
    workspace: WeakEntity<Workspace>,
    _subscriptions: Vec<Subscription>,
}

struct SecondaryEditor {
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
        buffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        workspace: Entity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let primary_editor =
            cx.new(|cx| Editor::for_multibuffer(buffer, Some(project.clone()), window, cx));
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
        let language_registry = project.read(cx).languages().clone();

        let primary_multibuffer = self.primary_editor.read(cx).buffer().read(cx);

        let base_text_buffers_by_main_buffer_id: HashMap<
            BufferId,
            (Entity<Buffer>, BufferDiffSnapshot),
        > = primary_multibuffer
            .all_buffer_ids_iter()
            .filter_map(|main_buffer_id| {
                let diff = primary_multibuffer.diff_for(main_buffer_id)?;
                let base_text_buffer = cx.new(|cx| {
                    let base_text = diff.read(cx).base_text();
                    let mut buffer = Buffer::local_normalized(
                        base_text.as_rope().clone(),
                        base_text.line_ending(),
                        cx,
                    );
                    buffer.set_language(base_text.language().cloned(), cx);
                    buffer.set_language_registry(language_registry.clone());
                    buffer
                });
                Some((
                    main_buffer_id,
                    (base_text_buffer, diff.read(cx).snapshot(cx)),
                ))
            })
            .collect();
        let snapshot = primary_multibuffer.snapshot(cx);
        let mut excerpt_ranges_by_base_buffer: HashMap<
            Entity<Buffer>,
            (PathKey, Vec<ExcerptRange<Point>>),
        > = HashMap::default();
        for (path_key, excerpt_id) in primary_multibuffer.excerpts_with_paths() {
            let main_buffer = snapshot.buffer_for_excerpt(*excerpt_id).unwrap();
            let excerpt_range = snapshot.excerpt_range_for_excerpt(*excerpt_id).unwrap();
            let (base_text_buffer, diff) = base_text_buffers_by_main_buffer_id
                .get(&main_buffer.remote_id())
                .unwrap();
            let point_to_base_text_point = |point: Point| {
                let row = diff.row_to_base_text_row(point.row, &main_buffer);
                let column = diff.base_text().line_len(row);
                Point::new(row, column)
            };
            let primary = excerpt_range.primary.to_point(&main_buffer);
            let context = excerpt_range.context.to_point(&main_buffer);
            let translated_range = ExcerptRange {
                primary: point_to_base_text_point(primary.start)
                    ..point_to_base_text_point(primary.end),
                context: point_to_base_text_point(context.start)
                    ..point_to_base_text_point(context.end),
            };
            excerpt_ranges_by_base_buffer
                .entry(base_text_buffer.clone())
                .or_insert((path_key.clone(), Vec::new()))
                .1
                .push(translated_range);
        }

        let secondary_multibuffer = cx.new(|cx| {
            let mut multibuffer = MultiBuffer::new(Capability::ReadOnly);
            for (base_text_buffer, (path_key, ranges)) in excerpt_ranges_by_base_buffer {
                let base_text_buffer_snapshot = base_text_buffer.read(cx).snapshot();
                multibuffer.update_path_excerpts(
                    path_key,
                    base_text_buffer,
                    &base_text_buffer_snapshot,
                    ranges,
                    cx,
                );
            }
            multibuffer
        });
        let secondary_editor =
            cx.new(|cx| Editor::for_multibuffer(secondary_multibuffer, Some(project), window, cx));

        // FIXME
        // - have to subscribe to the diffs to update the base text buffers (and handle language changed I think?)
        // - implement SplittableEditor::set_excerpts_for_path

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
            pane: secondary_pane.clone(),
            has_latest_selection: false,
            _subscriptions: subscriptions,
        };
        for (path_key, diff, original_range, original_buffer) in whatever {
            secondary.sync_path_excerpts_for_buffer(
                path_key,
                diff,
                original_range,
                original_buffer,
                cx,
            );
        }
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
            primary.buffer().update(cx, |buffer, _| {
                buffer.set_filter_mode(None);
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
        cx: &mut Context<Self>,
    ) -> (Vec<Range<Anchor>>, bool) {
        let (anchors, added_a_new_excerpt) =
            self.primary_editor
                .read(cx)
                .buffer()
                .update(cx, |multibuffer, cx| {
                    multibuffer.set_excerpts_for_path(path, buffer, ranges, context_line_count, cx)
                });
        if let Some(secondary) = &mut self.secondary {
            secondary.sync_path_excerpts_for_buffer(cx);
        }
        (anchors, added_a_new_excerpt)
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
    fn sync_path_excerpts_for_buffer(
        &mut self,
        path_key: PathKey,
        main_buffer: &BufferSnapshot,
        primary_multibuffer: &MultiBuffer,
        cx: &mut App,
    ) {
        let diff = primary_multibuffer
            .diff_for(main_buffer.remote_id())
            .unwrap();
        let diff = diff.read(cx).snapshot(cx);
        // option 1: hold onto the base text buffers in splittable editor so that we can check whether they exist yet
        // option 2: have the multibuffer continue to be fully responsible for holding the base text buffers; then need to be able to get a buffer out of the multibuffer based on a pathkey
        let base_text_buffer = self.editor.update(cx, |editor, cx| {
            editor
                .buffer()
                .update(cx, |buffer, cx| buffer.buffer_for_path_key)
        });
        let new = primary_multibuffer
            .excerpts_for_buffer(main_buffer.remote_id(), cx)
            .into_iter()
            .map(|(excerpt_id, excerpt_range)| {
                let point_to_base_text_point = |point: Point| {
                    let row = diff.row_to_base_text_row(point.row, &main_buffer);
                    let column = diff.base_text().line_len(row);
                    Point::new(row, column)
                };
                let primary = excerpt_range.primary.to_point(&main_buffer);
                let context = excerpt_range.context.to_point(&main_buffer);
                ExcerptRange {
                    primary: point_to_base_text_point(primary.start)
                        ..point_to_base_text_point(primary.end),
                    context: point_to_base_text_point(context.start)
                        ..point_to_base_text_point(context.end),
                }
            })
            .collect();

        self.editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |buffer, cx| {
                buffer.update_path_excerpts(
                    path_key,
                    base_text_buffer,
                    base_text_buffer_snapshot,
                    new,
                    cx,
                )
            })
        });
    }
}
