use anyhow::Result;
use buffer_diff::BufferDiff;
use editor::{Editor, EditorEvent, MultiBuffer, multibuffer_context_lines};
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, SharedString, Task, Window,
};
use language::{Buffer, Capability, OffsetRangeExt};
use multi_buffer::PathKey;
use project::Project;
use std::{
    any::{Any, TypeId},
    path::{Path, PathBuf},
    sync::Arc,
};
use theme;
use ui::{Color, Icon, IconName, Label, LabelCommon as _};
use util::paths::PathStyle;
use util::rel_path::RelPath;
use workspace::{
    Item, ItemHandle as _, ItemNavHistory, ToolbarItemLocation, Workspace,
    item::{BreadcrumbText, ItemEvent, SaveOptions, TabContentParams},
    searchable::SearchableItemHandle,
};

pub struct MultiDiffView {
    editor: Entity<Editor>,
    file_count: usize,
}

struct Entry {
    index: usize,
    new_path: PathBuf,
    new_buffer: Entity<Buffer>,
    diff: Entity<BufferDiff>,
}

async fn load_entries(
    diff_pairs: Vec<[String; 2]>,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<(Vec<Entry>, Option<PathBuf>)> {
    let mut entries = Vec::with_capacity(diff_pairs.len());
    let mut all_paths = Vec::with_capacity(diff_pairs.len());

    for (ix, pair) in diff_pairs.into_iter().enumerate() {
        let old_path = PathBuf::from(&pair[0]);
        let new_path = PathBuf::from(&pair[1]);

        let old_buffer = project
            .update(cx, |project, cx| project.open_local_buffer(&old_path, cx))
            .await?;
        let new_buffer = project
            .update(cx, |project, cx| project.open_local_buffer(&new_path, cx))
            .await?;

        let diff = build_buffer_diff(&old_buffer, &new_buffer, cx).await?;

        all_paths.push(new_path.clone());
        entries.push(Entry {
            index: ix,
            new_path,
            new_buffer: new_buffer.clone(),
            diff,
        });
    }

    let common_root = common_prefix(&all_paths);
    Ok((entries, common_root))
}

fn register_entry(
    multibuffer: &Entity<MultiBuffer>,
    entry: Entry,
    common_root: &Option<PathBuf>,
    context_lines: u32,
    cx: &mut Context<Workspace>,
) {
    let snapshot = entry.new_buffer.read(cx).snapshot();
    let diff_snapshot = entry.diff.read(cx).snapshot(cx);

    let ranges: Vec<std::ops::Range<language::Point>> = diff_snapshot
        .hunks(&snapshot)
        .map(|hunk| hunk.buffer_range.to_point(&snapshot))
        .collect();

    let display_rel = common_root
        .as_ref()
        .and_then(|root| entry.new_path.strip_prefix(root).ok())
        .map(|rel| {
            RelPath::new(rel, PathStyle::local())
                .map(|r| r.into_owned().into())
                .unwrap_or_else(|_| {
                    RelPath::new(Path::new("untitled"), PathStyle::Posix)
                        .unwrap()
                        .into_owned()
                        .into()
                })
        })
        .unwrap_or_else(|| {
            entry
                .new_path
                .file_name()
                .and_then(|n| n.to_str())
                .and_then(|s| RelPath::new(Path::new(s), PathStyle::Posix).ok())
                .map(|r| r.into_owned().into())
                .unwrap_or_else(|| {
                    RelPath::new(Path::new("untitled"), PathStyle::Posix)
                        .unwrap()
                        .into_owned()
                        .into()
                })
        });

    let path_key = PathKey::with_sort_prefix(entry.index as u64, display_rel);

    multibuffer.update(cx, |multibuffer, cx| {
        multibuffer.set_excerpts_for_path(
            path_key,
            entry.new_buffer.clone(),
            ranges,
            context_lines,
            cx,
        );
        multibuffer.add_diff(entry.diff.clone(), cx);
    });
}

fn common_prefix(paths: &[PathBuf]) -> Option<PathBuf> {
    let mut iter = paths.iter();
    let mut prefix = iter.next()?.clone();

    for path in iter {
        while !path.starts_with(&prefix) {
            if !prefix.pop() {
                return Some(PathBuf::new());
            }
        }
    }

    Some(prefix)
}

async fn build_buffer_diff(
    old_buffer: &Entity<Buffer>,
    new_buffer: &Entity<Buffer>,
    cx: &mut AsyncApp,
) -> Result<Entity<BufferDiff>> {
    let old_buffer_snapshot = old_buffer.read_with(cx, |buffer, _| buffer.snapshot());
    let new_buffer_snapshot = new_buffer.read_with(cx, |buffer, _| buffer.snapshot());

    let diff = cx.new(|cx| BufferDiff::new(&new_buffer_snapshot.text, cx));

    let update = diff
        .update(cx, |diff, cx| {
            diff.update_diff(
                new_buffer_snapshot.text.clone(),
                Some(old_buffer_snapshot.text().into()),
                Some(true),
                new_buffer_snapshot.language().cloned(),
                cx,
            )
        })
        .await;

    diff.update(cx, |diff, cx| {
        diff.set_snapshot(update, &new_buffer_snapshot.text, cx)
    })
    .await;

    Ok(diff)
}

impl MultiDiffView {
    pub fn open(
        diff_pairs: Vec<[String; 2]>,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let project = workspace.project().clone();
        let workspace = workspace.weak_handle();
        let context_lines = multibuffer_context_lines(cx);

        window.spawn(cx, async move |cx| {
            let (entries, common_root) = load_entries(diff_pairs, &project, cx).await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let multibuffer = cx.new(|cx| {
                    let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
                    multibuffer.set_all_diff_hunks_expanded(cx);
                    multibuffer
                });

                let file_count = entries.len();
                for entry in entries {
                    register_entry(&multibuffer, entry, &common_root, context_lines, cx);
                }

                let diff_view = cx.new(|cx| {
                    Self::new(multibuffer.clone(), project.clone(), file_count, window, cx)
                });

                let pane = workspace.active_pane();
                pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(diff_view.clone()), true, true, None, window, cx);
                });

                // Hide the left dock (file explorer) for a cleaner diff view
                workspace.left_dock().update(cx, |dock, cx| {
                    dock.set_open(false, window, cx);
                });

                diff_view
            })
        })
    }

    fn new(
        multibuffer: Entity<MultiBuffer>,
        project: Entity<Project>,
        file_count: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let editor = cx.new(|cx| {
            let mut editor =
                Editor::for_multibuffer(multibuffer, Some(project.clone()), window, cx);
            editor.start_temporary_diff_override();
            editor.disable_diagnostics(cx);
            editor.set_expand_all_diff_hunks(cx);
            editor.set_render_diff_hunk_controls(
                Arc::new(|_, _, _, _, _, _, _, _| gpui::Empty.into_any_element()),
                cx,
            );
            editor
        });

        Self { editor, file_count }
    }

    fn title(&self) -> SharedString {
        let suffix = if self.file_count == 1 {
            "1 file".to_string()
        } else {
            format!("{} files", self.file_count)
        };
        format!("Diff ({suffix})").into()
    }
}

impl EventEmitter<EditorEvent> for MultiDiffView {}

impl Focusable for MultiDiffView {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.editor.focus_handle(cx)
    }
}

impl Item for MultiDiffView {
    type Event = EditorEvent;

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Diff).color(Color::Muted))
    }

    fn tab_content(&self, params: TabContentParams, _window: &Window, _cx: &App) -> AnyElement {
        Label::new(self.title())
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn tab_tooltip_text(&self, _cx: &App) -> Option<ui::SharedString> {
        Some(self.title())
    }

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        self.title()
    }

    fn to_item_events(event: &EditorEvent, f: impl FnMut(ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        Some("Diff View Opened")
    }

    fn deactivated(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.editor
            .update(cx, |editor, cx| editor.deactivated(window, cx));
    }

    fn act_as_type<'a>(
        &'a self,
        type_id: TypeId,
        self_handle: &'a Entity<Self>,
        _: &'a App,
    ) -> Option<gpui::AnyEntity> {
        if type_id == TypeId::of::<Self>() {
            Some(self_handle.clone().into())
        } else if type_id == TypeId::of::<Editor>() {
            Some(self.editor.clone().into())
        } else {
            None
        }
    }

    fn as_searchable(&self, _: &Entity<Self>, _: &App) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(self.editor.clone()))
    }

    fn set_nav_history(
        &mut self,
        nav_history: ItemNavHistory,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, _| {
            editor.set_nav_history(Some(nav_history));
        });
    }

    fn navigate(
        &mut self,
        data: Arc<dyn Any + Send>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        self.editor
            .update(cx, |editor, cx| editor.navigate(data, window, cx))
    }

    fn breadcrumb_location(&self, _: &App) -> ToolbarItemLocation {
        ToolbarItemLocation::PrimaryLeft
    }

    fn breadcrumbs(&self, theme: &theme::Theme, cx: &App) -> Option<Vec<BreadcrumbText>> {
        self.editor.breadcrumbs(theme, cx)
    }

    fn added_to_workspace(
        &mut self,
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.editor.update(cx, |editor, cx| {
            editor.added_to_workspace(workspace, window, cx)
        });
    }

    fn can_save(&self, cx: &App) -> bool {
        self.editor.read(cx).can_save(cx)
    }

    fn save(
        &mut self,
        options: SaveOptions,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::Task<Result<()>> {
        self.editor
            .update(cx, |editor, cx| editor.save(options, project, window, cx))
    }
}

impl Render for MultiDiffView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.editor.clone()
    }
}
