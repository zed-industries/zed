use ::settings::Settings as _;
use editor::{Editor, EditorEvent};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _, PresenceFlag, register_feature_flag};
use gpui::{
    AppContext, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, Task, actions,
};
use language::Buffer;
use project::{Project, ProjectPath};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::table_data_engine::{DisplayToDataMapping, TableDataEngine};
use ui::{
    AbsoluteLength, ResizableColumnsState, SharedString, TableInteractionState,
    TableResizeBehavior, prelude::*,
};
use workspace::item::{ItemBufferKind, ProjectItem};
use workspace::{AutoPreview, Item, Pane, SplitDirection, Workspace, WorkspaceSettings};
use zed_actions::preview::{OpenSource, Toggle, TogglePlacement};

use crate::{parser::EditorState, settings::CsvPreviewSettings, types::TableLikeContent};

mod parser;
mod renderer;
mod settings;
mod table_data_engine;
mod types;

actions!(csv, [OpenPreview, OpenPreviewToTheSide]);

pub struct TabularDataPreviewFeatureFlag;

impl FeatureFlag for TabularDataPreviewFeatureFlag {
    const NAME: &'static str = "tabular-data-preview";
    type Value = PresenceFlag;
}
register_feature_flag!(TabularDataPreviewFeatureFlag);

pub struct CsvPreviewView {
    _workspace_subscription: Option<gpui::Subscription>,
    pub(crate) engine: TableDataEngine,

    pub(crate) focus_handle: FocusHandle,
    active_editor_state: EditorState,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) is_parsing: bool,
    /// Background task computing the display-to-data mapping after a filter/sort change.
    /// Stored here so that a new change cancels the previous in-flight computation.
    pub(crate) filter_sort_task: Option<Task<()>>,
    pub(crate) settings: CsvPreviewSettings,
    /// Performance metrics for debugging and monitoring CSV operations.
    pub(crate) performance_metrics: PerformanceMetrics,
    pub(crate) list_state: gpui::ListState,
    /// Time when the last parsing operation ended, used for smart debouncing
    pub(crate) last_parse_end_time: Option<std::time::Instant>,
}

pub fn init(cx: &mut App) {
    workspace::register_project_item::<CsvPreviewView>(cx);
    workspace::register_auto_preview_provider(CsvPreviewView::auto_preview_provider(), cx);
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        CsvPreviewView::register(workspace);
    })
    .detach()
}

impl CsvPreviewView {
    pub(crate) fn sync_column_widths(&self, cx: &mut Context<Self>) {
        // plus 1 for the row identifier column
        let cols = self.engine.contents.headers.cols() + 1;
        let line_number_width = self.calculate_row_identifier_column_width();

        let mut widths: Vec<AbsoluteLength> = vec![AbsoluteLength::Pixels(px(150.)); cols];
        widths[0] = AbsoluteLength::Pixels(px(line_number_width));

        let mut resize_behaviors = vec![TableResizeBehavior::Resizable; cols];
        resize_behaviors[0] = TableResizeBehavior::None;

        self.column_widths.widths.update(cx, |state, _cx| {
            if state.cols() != cols {
                *state = ResizableColumnsState::new(cols, widths, resize_behaviors);
            } else {
                state.set_column_configuration(
                    0,
                    AbsoluteLength::Pixels(px(line_number_width)),
                    TableResizeBehavior::None,
                );
            }
        });
    }

    /// Opens (or reveals) a preview for the active CSV editor.
    /// Returns false when the active item is not a CSV editor.
    fn open_preview_for_active_editor(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        let Some(editor) = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))
            .filter(|editor| Self::is_csv_file(editor, cx))
        else {
            return false;
        };
        let csv_preview = Self::new(&editor, cx);
        workspace.active_pane().update(cx, |pane, cx| {
            let existing = pane
                .items_of_type::<CsvPreviewView>()
                .find(|view| view.read(cx).active_editor_state.editor == editor);
            if let Some(idx) = existing.and_then(|e| pane.index_for_item(&e)) {
                pane.activate_item(idx, true, true, window, cx);
            } else {
                pane.add_item(Box::new(csv_preview), true, true, None, window, cx);
            }
        });
        cx.notify();
        true
    }

    /// Activates (or opens) a text editor for the active CSV preview.
    /// Returns false when the active item is not a CSV preview.
    fn open_source_for_active_preview(
        workspace: &mut Workspace,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> bool {
        let Some(preview) = workspace
            .active_item(cx)
            .and_then(|item| item.downcast::<CsvPreviewView>())
        else {
            return false;
        };
        let editor = preview.read(cx).active_editor_state.editor.clone();
        if !workspace.activate_item(&editor, true, true, window, cx) {
            workspace.active_pane().update(cx, |pane, cx| {
                pane.add_item(Box::new(editor.clone()), true, true, None, window, cx);
            });
        }
        true
    }

    pub fn register(workspace: &mut Workspace) {
        workspace.register_action_renderer(|div, _, _, cx| {
            div.when(cx.has_flag::<TabularDataPreviewFeatureFlag>(), |div| {
                div.on_action(cx.listener(|workspace, _: &OpenPreview, window, cx| {
                    Self::open_preview_for_active_editor(workspace, window, cx);
                }))
                .on_action(
                    cx.listener(|workspace, _: &OpenPreviewToTheSide, window, cx| {
                        if let Some(editor) = workspace
                            .active_item(cx)
                            .and_then(|item| item.act_as::<Editor>(cx))
                            .filter(|editor| Self::is_csv_file(editor, cx))
                        {
                            let csv_preview = Self::new(&editor, cx);
                            let pane = workspace
                                .find_pane_in_direction(SplitDirection::Right, cx)
                                .unwrap_or_else(|| {
                                    workspace.split_pane(
                                        workspace.active_pane().clone(),
                                        SplitDirection::Right,
                                        window,
                                        cx,
                                    )
                                });
                            pane.update(cx, |pane, cx| {
                                let existing =
                                    pane.items_of_type::<CsvPreviewView>().find(|view| {
                                        view.read(cx).active_editor_state.editor == editor
                                    });
                                if let Some(idx) = existing.and_then(|e| pane.index_for_item(&e)) {
                                    pane.activate_item(idx, true, true, window, cx);
                                } else {
                                    pane.add_item(
                                        Box::new(csv_preview),
                                        false,
                                        false,
                                        None,
                                        window,
                                        cx,
                                    );
                                }
                            });
                            cx.notify();
                        }
                    }),
                )
                .on_action(cx.listener(|workspace, _: &OpenSource, window, cx| {
                    if !Self::open_source_for_active_preview(workspace, window, cx) {
                        cx.propagate();
                    }
                }))
                .on_action(cx.listener(
                    |workspace, action: &Toggle, window, cx| {
                        let handled = match action.placement {
                            TogglePlacement::InPlace => {
                                Self::open_source_for_active_preview(workspace, window, cx)
                                    || Self::open_preview_for_active_editor(workspace, window, cx)
                            }
                            TogglePlacement::ToTheSide => {
                                workspace::show_side_preview_for_active_item(workspace, window, cx)
                            }
                        };
                        if !handled {
                            cx.propagate();
                        }
                    },
                ))
            })
        });
    }

    fn new(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| Self::build(editor.clone(), cx))
    }

    fn new_following(
        editor: &Entity<Editor>,
        window: &Window,
        cx: &mut Context<Workspace>,
    ) -> Entity<Self> {
        let workspace = cx.entity();
        cx.new(|cx| {
            let mut this = Self::build(editor.clone(), cx);
            this._workspace_subscription = Some(cx.subscribe_in(
                &workspace,
                window,
                |this: &mut Self, workspace, event: &workspace::Event, _window, cx| {
                    if let workspace::Event::ActiveItemChanged = event
                        && let Some(editor) = workspace
                            .read(cx)
                            .active_item(cx)
                            .and_then(|item| item.downcast::<Editor>())
                        && Self::is_csv_file(&editor, cx)
                        && this.active_editor_state.editor != editor
                    {
                        this.set_editor(editor, cx);
                    }
                },
            ));
            this
        })
    }

    fn is_following(&self) -> bool {
        self._workspace_subscription.is_some()
    }

    fn set_editor(&mut self, editor: Entity<Editor>, cx: &mut Context<Self>) {
        let subscription = Self::subscribe_to_editor(&editor, cx);
        self.active_editor_state = EditorState {
            editor,
            _subscription: subscription,
        };
        self.parse_csv_from_active_editor(false, cx);
        cx.notify();
    }

    fn subscribe_to_editor(editor: &Entity<Editor>, cx: &mut Context<Self>) -> gpui::Subscription {
        cx.subscribe(
            editor,
            |this: &mut CsvPreviewView, _editor, event: &EditorEvent, cx| {
                match event {
                    EditorEvent::Edited { .. } | EditorEvent::DirtyChanged => {
                        this.parse_csv_from_active_editor(true, cx);
                    }
                    _ => {}
                };
            },
        )
    }

    pub(crate) fn auto_preview_provider() -> workspace::AutoPreviewProvider {
        workspace::AutoPreviewProvider {
            applies_to: |item, cx| {
                cx.has_flag::<TabularDataPreviewFeatureFlag>()
                    && item
                        .downcast::<Editor>()
                        .is_some_and(|editor| Self::is_csv_file(&editor, cx))
            },
            has_open_sources: |workspace, cx| {
                workspace
                    .items_of_type::<Editor>(cx)
                    .any(|editor| Self::is_csv_file(&editor, cx))
            },
            is_follow_view: |item, cx| {
                item.downcast::<CsvPreviewView>()
                    .is_some_and(|view| view.read(cx).is_following())
            },
            is_preview_view: |item, cx| {
                item.downcast::<CsvPreviewView>()
                    .is_some_and(|view| !view.read(cx).is_following())
            },
            build_follow_view: |workspace, window, cx| {
                let editor = workspace
                    .active_item(cx)
                    .and_then(|item| item.downcast::<Editor>())
                    .filter(|editor| Self::is_csv_file(editor, cx))?;
                Some(Box::new(Self::new_following(&editor, window, cx)))
            },
            build_preview_view: |_, item, _, cx| {
                let editor = item.downcast::<Editor>()?;
                Some(Box::new(Self::new(&editor, cx)))
            },
            source_view: |_, item, _, cx| {
                let preview = item.downcast::<CsvPreviewView>()?;
                Some(Box::new(
                    preview.read(cx).active_editor_state.editor.clone(),
                ))
            },
        }
    }

    fn build(editor: Entity<Editor>, cx: &mut Context<Self>) -> Self {
        let contents = TableLikeContent::default();
        let table_interaction_state = cx.new(|cx| {
            TableInteractionState::new(cx).with_custom_scrollbar(ui::Scrollbars::for_settings::<
                editor::EditorSettingsScrollbarProxy,
            >())
        });

        let subscription = Self::subscribe_to_editor(&editor, cx);

        let mut view = CsvPreviewView {
            _workspace_subscription: None,
            focus_handle: cx.focus_handle(),
            active_editor_state: EditorState {
                editor,
                _subscription: subscription,
            },
            table_interaction_state,
            column_widths: ColumnWidths::new(cx, 1),
            parsing_task: None,
            is_parsing: false,
            filter_sort_task: None,
            performance_metrics: PerformanceMetrics::default(),
            list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.))
                .with_uniform_item_height(px(24.)),
            settings: CsvPreviewSettings::default(),
            last_parse_end_time: None,
            engine: TableDataEngine::default(),
        };

        view.parse_csv_from_active_editor(false, cx);
        view
    }

    pub(crate) fn editor_state(&self) -> &EditorState {
        &self.active_editor_state
    }
    pub(crate) fn apply_sort(&mut self, cx: &mut Context<Self>) {
        self.apply_filter_sort(cx);
    }

    pub fn clear_filters(&mut self, col: types::AnyColumn, cx: &mut Context<Self>) {
        self.engine.clear_filters_for_col(col);
        self.apply_filter_sort(cx);
    }

    pub fn toggle_filter(
        &mut self,
        col: types::AnyColumn,
        value: Option<SharedString>,
        cx: &mut Context<Self>,
    ) {
        if let Err(err) = self.engine.toggle_filter(col, value) {
            log::error!("Failed to toggle filter: {err}");
            return;
        }
        self.apply_filter_sort(cx);
    }

    /// Spawns a background task to recompute the display-to-data mapping after a filter or sort
    /// change. Storing the task cancels any previous in-flight computation automatically.
    pub(crate) fn apply_filter_sort(&mut self, cx: &mut Context<Self>) {
        let contents = self.engine.contents.clone();
        let filter_stack = self.engine.filter_stack.clone();
        let sorting = self.engine.applied_sorting;

        self.filter_sort_task = Some(cx.spawn(async move |this, cx| {
            let mapping = cx
                .background_spawn(async move {
                    DisplayToDataMapping::compute(&contents, &filter_stack, sorting)
                })
                .await;

            this.update(cx, |view, cx| {
                view.engine.set_d2d_mapping(mapping);
                let visible_rows = view.engine.d2d_mapping().visible_row_count();
                // Approximation of single csv table row height. Will be re-measured on scrolling.
                // This cheap solution allow to render scrollbar with fraction of a cost compared to `.measure_all()` call
                let approximate_height = px(24.);
                view.list_state
                    .reset_with_uniform_height(visible_rows, approximate_height);
                cx.notify();
            })
            .ok();
        }));
    }

    pub fn resolve_active_item_as_csv_editor(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<Editor>> {
        let editor = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))?;
        Self::is_csv_file(&editor, cx).then_some(editor)
    }

    fn is_csv_path(path: impl AsRef<std::path::Path>) -> bool {
        path.as_ref()
            .extension()
            .is_some_and(|extension| extension.eq_ignore_ascii_case("csv"))
    }

    fn source_buffer(&self, cx: &App) -> Option<Entity<Buffer>> {
        self.active_editor_state
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
    }

    fn is_csv_file(editor: &Entity<Editor>, cx: &App) -> bool {
        editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|buffer| {
                buffer
                    .read(cx)
                    .file()
                    .and_then(|file| file.path().extension())
                    .map(|ext| ext.eq_ignore_ascii_case("csv"))
            })
            .unwrap_or(false)
    }
}

impl Focusable for CsvPreviewView {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl EventEmitter<()> for CsvPreviewView {}

impl Item for CsvPreviewView {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.editor_state()
            .editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .and_then(|b| {
                let file = b.read(cx).file()?;
                let local_file = file.as_local()?;
                local_file
                    .abs_path(cx)
                    .file_name()
                    .map(|name| format!("Preview {}", name.to_string_lossy()).into())
            })
            .unwrap_or_else(|| SharedString::from("CSV Preview"))
    }

    fn buffer_kind(&self, _cx: &App) -> ItemBufferKind {
        ItemBufferKind::Singleton
    }

    fn is_dirty(&self, cx: &App) -> bool {
        self.source_buffer(cx)
            .is_some_and(|buffer| buffer.read(cx).is_dirty())
    }

    fn for_each_project_item(
        &self,
        cx: &App,
        f: &mut dyn FnMut(gpui::EntityId, &dyn project::ProjectItem),
    ) {
        if let Some(buffer) = self.source_buffer(cx) {
            f(buffer.entity_id(), buffer.read(cx))
        }
    }
}

/// A [`project::ProjectItem`] that claims CSV files when the `auto_preview` setting
/// is set to `in_place`, so that opening such files shows their rendered preview instead of an editor.
pub struct CsvPreviewItem {
    buffer: Entity<Buffer>,
}

impl project::ProjectItem for CsvPreviewItem {
    fn try_open(
        project: &Entity<Project>,
        path: &ProjectPath,
        cx: &mut App,
    ) -> Option<Task<anyhow::Result<Entity<Self>>>> {
        if !cx.has_flag::<TabularDataPreviewFeatureFlag>()
            || WorkspaceSettings::get_global(cx).auto_preview != AutoPreview::InPlace
            || !project
                .read(cx)
                .absolute_path(path, cx)
                .is_some_and(CsvPreviewView::is_csv_path)
        {
            return None;
        }
        let buffer = project.update(cx, |project, cx| project.open_buffer(path.clone(), cx));
        Some(cx.spawn(async move |cx| {
            let buffer = buffer.await?;
            Ok(cx.new(|_| CsvPreviewItem { buffer }))
        }))
    }

    fn entry_id(&self, cx: &App) -> Option<project::ProjectEntryId> {
        project::ProjectItem::entry_id(self.buffer.read(cx), cx)
    }

    fn project_path(&self, cx: &App) -> Option<ProjectPath> {
        project::ProjectItem::project_path(self.buffer.read(cx), cx)
    }

    fn is_dirty(&self) -> bool {
        // This item is only a carrier between `try_open` and `for_project_item`: the
        // preview reports its dirty state through the buffer it renders.
        false
    }
}

impl ProjectItem for CsvPreviewView {
    type Item = CsvPreviewItem;

    fn for_project_item(
        project: Entity<Project>,
        _pane: Option<&Pane>,
        item: Entity<Self::Item>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let buffer = item.read(cx).buffer.clone();
        let editor = cx.new(|cx| Editor::for_buffer(buffer, Some(project), window, cx));
        Self::build(editor, cx)
    }
}

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    /// Map of timing metrics with their duration and measurement time.
    pub timings: HashMap<&'static str, (Duration, Instant)>,
    /// List of display indices that were rendered in the current frame.
    pub rendered_indices: Vec<usize>,
}
impl PerformanceMetrics {
    pub fn record<F, R>(&mut self, name: &'static str, mut f: F) -> R
    where
        F: FnMut() -> R,
    {
        let start_time = Instant::now();
        let ret = f();
        let duration = start_time.elapsed();
        self.timings.insert(name, (duration, Instant::now()));
        ret
    }

    /// Displays all metrics sorted A-Z in format: `{name}: {took}ms {ago}s ago`
    pub fn display(&self) -> String {
        let mut metrics = self.timings.iter().collect::<Vec<_>>();
        metrics.sort_by_key(|&(name, _)| *name);
        metrics
            .iter()
            .map(|(name, (duration, time))| {
                let took = duration.as_secs_f32() * 1000.;
                let ago = time.elapsed().as_secs();
                format!("{name}: {took:.3}ms {ago}s ago")
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Get timing for a specific metric
    pub fn get_timing(&self, name: &str) -> Option<Duration> {
        self.timings.get(name).map(|(duration, _)| *duration)
    }
}

/// Holds state of column widths for a table component in CSV preview.
pub(crate) struct ColumnWidths {
    pub widths: Entity<ResizableColumnsState>,
}

impl ColumnWidths {
    pub(crate) fn new(cx: &mut Context<CsvPreviewView>, cols: usize) -> Self {
        Self {
            widths: cx.new(|_cx| {
                ResizableColumnsState::new(
                    cols,
                    vec![AbsoluteLength::Pixels(px(150.)); cols],
                    vec![ui::TableResizeBehavior::Resizable; cols],
                )
            }),
        }
    }
}
