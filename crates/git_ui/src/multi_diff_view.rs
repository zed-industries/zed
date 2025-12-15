use anyhow::Result;
use buffer_diff::{BufferDiff, BufferDiffSnapshot};
use editor::{Editor, EditorEvent, MultiBuffer, multibuffer_context_lines};
use gpui::{
    AnyElement, App, AppContext as _, AsyncApp, Context, Entity, EventEmitter, FocusHandle,
    Focusable, IntoElement, Render, SharedString, Task, Window,
};
use language::{Anchor, Buffer, BufferId, Capability, OffsetRangeExt, Point};
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
    buffer_labels: collections::HashMap<BufferId, String>,
    file_count: usize,
}

pub fn open(
    diff_pairs: Vec<[String; 2]>,
    diff_labels: Vec<String>,
    workspace: &Workspace,
    window: &mut Window,
    cx: &mut App,
) -> Task<Result<Entity<MultiDiffView>>> {
    MultiDiffView::open(diff_pairs, diff_labels, workspace, window, cx)
}

struct Entry {
    index: usize,
    new_buffer: Entity<Buffer>,
    diff: Entity<BufferDiff>,
    display_rel: Arc<RelPath>,
    display_label: String,
}

async fn load_entries(
    diff_pairs: Vec<[String; 2]>,
    diff_labels: Vec<String>,
    project: &Entity<Project>,
    cx: &mut AsyncApp,
) -> Result<(Vec<Entry>, Option<PathBuf>)> {
    let mut entries = Vec::with_capacity(diff_pairs.len());
    let mut all_paths = Vec::with_capacity(diff_pairs.len());

    for (ix, (pair, label)) in diff_pairs
        .into_iter()
        .zip(diff_labels.into_iter())
        .enumerate()
    {
        let old_path = PathBuf::from(&pair[0]);
        let new_path = PathBuf::from(&pair[1]);

        let old_buffer = project
            .update(cx, |project, cx| project.open_local_buffer(&old_path, cx))?
            .await?;
        let new_buffer = project
            .update(cx, |project, cx| project.open_local_buffer(&new_path, cx))?
            .await?;

        let diff = build_buffer_diff(&old_buffer, &new_buffer, cx).await?;

        let display_rel = label_to_rel_path(&label).unwrap_or_else(|| {
            RelPath::new(Path::new("untitled"), PathStyle::Posix)
                .unwrap()
                .into_owned()
                .into()
        });

        entries.push(Entry {
            index: ix,
            new_buffer: new_buffer.clone(),
            diff,
            display_label: label,
            display_rel,
        });
        all_paths.push(new_path);
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
) -> Option<(BufferId, String)> {
    let (path_for_key, buffer_label) = {
        let buffer = entry.new_buffer.read(cx);
        let buffer_path = buffer.file().map(|file| file.path().clone());

        let label = buffer_path
            .as_ref()
            .map(|path| path.as_unix_str().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| {
                common_root.as_ref().and_then(|root| {
                    buffer.file().and_then(|file| {
                        let full = file.full_path(cx);
                        full.strip_prefix(root)
                            .ok()
                            .and_then(|rel| rel.to_str().map(|s| s.replace('\\', "/")))
                    })
                })
            })
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| entry.display_label.clone());

        (
            buffer_path
                .filter(|p| !p.as_unix_str().is_empty())
                .unwrap_or_else(|| entry.display_rel.clone()),
            label,
        )
    };

    let new_snapshot = entry.new_buffer.read(cx).snapshot();

    let ranges: Vec<std::ops::Range<Point>> = {
        let diff_read = entry.diff.read(cx);
        diff_read
            .hunks_intersecting_range(
                Anchor::min_max_range_for_buffer(diff_read.buffer_id),
                &new_snapshot,
                cx,
            )
            .map(|hunk| hunk.buffer_range.to_point(&new_snapshot))
            .collect()
    };

    let path_key = PathKey::with_sort_prefix(entry.index as u64, path_for_key);

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
    let buffer_id = entry.new_buffer.read(cx).remote_id();
    Some((buffer_id, buffer_label))
}

fn label_to_rel_path(label: &str) -> Option<Arc<RelPath>> {
    RelPath::new(Path::new(label), PathStyle::Posix)
        .ok()
        .map(|rel| rel.into_owned().into())
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
    let old_snapshot = old_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
    let new_snapshot = new_buffer.read_with(cx, |buffer, _| buffer.snapshot())?;

    let diff_snapshot = cx
        .update(|cx| {
            BufferDiffSnapshot::new_with_base_buffer(
                new_snapshot.text.clone(),
                Some(old_snapshot.text().into()),
                old_snapshot,
                cx,
            )
        })?
        .await;

    cx.new(|cx| {
        let mut diff = BufferDiff::new(&new_snapshot.text, cx);
        diff.set_snapshot(diff_snapshot, &new_snapshot.text, cx);
        diff
    })
}

impl MultiDiffView {
    pub fn open(
        diff_pairs: Vec<[String; 2]>,
        diff_labels: Vec<String>,
        workspace: &Workspace,
        window: &mut Window,
        cx: &mut App,
    ) -> Task<Result<Entity<Self>>> {
        let project = workspace.project().clone();
        let workspace = workspace.weak_handle();
        let context_lines = multibuffer_context_lines(cx);

        window.spawn(cx, async move |cx| {
            let (entries, common_root) =
                load_entries(diff_pairs, diff_labels, &project, cx).await?;

            workspace.update_in(cx, |workspace, window, cx| {
                let multibuffer = cx.new(|cx| {
                    let mut multibuffer = MultiBuffer::new(Capability::ReadWrite);
                    multibuffer.set_all_diff_hunks_expanded(cx);
                    multibuffer
                });

                let file_count = entries.len();
                let mut buffer_labels = collections::HashMap::default();
                for entry in entries {
                    if let Some((buffer_id, label)) =
                        register_entry(&multibuffer, entry, &common_root, context_lines, cx)
                    {
                        buffer_labels.insert(buffer_id, label);
                    }
                }

                let diff_view = cx.new(|cx| {
                    Self::new(
                        multibuffer.clone(),
                        buffer_labels,
                        project.clone(),
                        file_count,
                        window,
                        cx,
                    )
                });

                let pane = workspace.active_pane();
                pane.update(cx, |pane, cx| {
                    pane.add_item(Box::new(diff_view.clone()), true, true, None, window, cx);
                });

                diff_view
            })
        })
    }

    fn new(
        multibuffer: Entity<MultiBuffer>,
        buffer_labels: collections::HashMap<BufferId, String>,
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
        {
            let labels = buffer_labels.clone();
            editor.update(cx, |editor, _cx| {
                for (buffer_id, label) in labels {
                    editor.set_path_override(buffer_id, label);
                }
            });
        }

        {
            cx.subscribe(
                &editor,
                move |this: &mut MultiDiffView, _, event, cx| match event {
                    EditorEvent::SelectionsChanged { .. }
                    | EditorEvent::Focused
                    | EditorEvent::FocusedIn => {
                        this.update_header(cx);
                    }
                    _ => {}
                },
            )
            .detach();
        }

        Self {
            editor,
            buffer_labels,
            file_count,
        }
    }

    fn update_header(&self, cx: &mut Context<Self>) {
        let labels = &self.buffer_labels;
        self.editor.update(cx, |editor, _cx| {
            let anchor = editor.selections.newest_anchor().head();
            if let Some(buffer_id) = anchor.text_anchor.buffer_id {
                if let Some(label) = labels.get(&buffer_id) {
                    editor.set_breadcrumb_header(label.clone());
                }
            }
        });
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
        data: Box<dyn Any>,
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
