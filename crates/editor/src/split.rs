use std::{ops::Range, sync::Arc};

use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    Action, AppContext as _, Entity, EventEmitter, Focusable, NoAction, Subscription, WeakEntity,
};
use language::{Buffer, Capability, LanguageRegistry};
use multi_buffer::{Anchor, ExcerptRange, MultiBuffer, PathKey};
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

        // FIXME
        // - have to subscribe to the diffs to update the base text buffers (and handle language changed I think?)

        let secondary_editor = cx.new(|cx| {
            let multibuffer = cx.new(|cx| {
                let mut multibuffer = MultiBuffer::new(Capability::ReadOnly);
                multibuffer.set_all_diff_hunks_expanded(cx);
                multibuffer
            });
            Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx)
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
            pane: secondary_pane.clone(),
            has_latest_selection: false,
            _subscriptions: subscriptions,
        };
        self.primary_editor.update(cx, |editor, cx| {
            editor.buffer().update(cx, |primary_multibuffer, cx| {
                primary_multibuffer.set_show_deleted_hunks(false, cx);
                let paths = primary_multibuffer.paths().collect::<Vec<_>>();
                for path in paths {
                    secondary.sync_path_excerpts(
                        path,
                        primary_multibuffer,
                        project.read(cx).languages().clone(),
                        cx,
                    );
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

    // FIXME need add_diff management in here too

    pub fn set_excerpts_for_path(
        &mut self,
        path: PathKey,
        buffer: Entity<Buffer>,
        ranges: impl IntoIterator<Item = Range<Point>>,
        context_line_count: u32,
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
                if let Some(secondary) = &mut self.secondary
                    && let Some(languages) = self
                        .workspace
                        .update(cx, |workspace, cx| {
                            workspace.project().read(cx).languages().clone()
                        })
                        .ok()
                {
                    secondary.sync_path_excerpts(path, primary_multibuffer, languages, cx);
                }
                (anchors, added_a_new_excerpt)
            })
        })
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
        languages: Arc<LanguageRegistry>,
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
        let diff = primary_multibuffer
            .diff_for(main_buffer.remote_id())
            .unwrap();
        let diff = diff.read(cx).snapshot(cx);
        let base_text_buffer = self
            .editor
            .update(cx, |editor, cx| {
                editor.buffer().update(cx, |secondary_multibuffer, cx| {
                    let excerpt_id = secondary_multibuffer.excerpts_for_path(&path_key).next()?;
                    let secondary_buffer_snapshot = secondary_multibuffer.snapshot(cx);
                    let buffer = secondary_buffer_snapshot
                        .buffer_for_excerpt(excerpt_id)
                        .unwrap();
                    Some(secondary_multibuffer.buffer(buffer.remote_id()).unwrap())
                })
            })
            .unwrap_or_else(|| {
                cx.new(|cx| {
                    let base_text = diff.base_text();
                    let mut buffer = Buffer::local_normalized(
                        base_text.as_rope().clone(),
                        base_text.line_ending(),
                        cx,
                    );
                    buffer.set_language(base_text.language().cloned(), cx);
                    buffer.set_language_registry(languages);
                    buffer
                })
            });
        let base_text_buffer_snapshot = base_text_buffer.read(cx).snapshot();
        let new = primary_multibuffer
            .excerpts_for_buffer(main_buffer.remote_id(), cx)
            .into_iter()
            .map(|(_, excerpt_range)| {
                let point_to_base_text_point = |point: Point| {
                    let row = diff.row_to_base_text_row(point.row, main_buffer);
                    let column = diff.base_text().line_len(row);
                    Point::new(row, column)
                };
                let primary = excerpt_range.primary.to_point(main_buffer);
                let context = excerpt_range.context.to_point(main_buffer);
                ExcerptRange {
                    primary: point_to_base_text_point(primary.start)
                        ..point_to_base_text_point(primary.end),
                    context: point_to_base_text_point(context.start)
                        ..point_to_base_text_point(context.end),
                }
            })
            .collect();

        let diff = primary_multibuffer
            .diff_for(main_buffer.remote_id())
            .unwrap();
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
                buffer.add_inverted_diff(
                    base_text_buffer_snapshot.remote_id(),
                    diff,
                    main_buffer,
                    cx,
                );
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
    use settings::SettingsStore;
    use ui::VisualContext as _;
    use workspace::Workspace;

    use crate::SplittableEditor;

    #[gpui::test]
    async fn test_basic_excerpts(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let store = SettingsStore::test(cx);
            cx.set_global(store);
            theme::init(theme::LoadThemes::JustBase, cx);
            crate::init(cx);
        });
        let base_text = indoc! {"
            hello
        "};
        let buffer_text = indoc! {"
            HELLO!
        "};
        let buffer = cx.new(|cx| Buffer::local(buffer_text, cx));
        let diff = cx.new(|cx| BufferDiff::new_with_base_text(base_text, &buffer, cx));
        let project = Project::test(FakeFs::new(cx.executor()), [], cx).await;
        let (workspace, cx) =
            cx.add_window_view(|window, cx| Workspace::test_new(project.clone(), window, cx));
        let multibuffer = cx.new(|_| MultiBuffer::new(Capability::ReadWrite));
        let editor = cx.new_window_entity(|window, cx| {
            SplittableEditor::new_unsplit(multibuffer, project, workspace, window, cx)
        });
    }
}
