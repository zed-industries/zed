use editor::{Editor, EditorEvent};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _, PresenceFlag, register_feature_flag};
use gpui::{
    AppContext, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, Task, actions,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::table_data_engine::{DisplayToDataMapping, TableDataEngine};
use ui::{
    AbsoluteLength, ResizableColumnsState, SharedString, TableInteractionState,
    TableResizeBehavior, prelude::*,
};
use workspace::{Item, Pane, Workspace};

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
    /// Cached row height, refreshed from the actual text line height on every render.
    /// Used to size not-yet-rendered rows for the scrollbar without a full `.measure_all()`
    /// pass, so it tracks the real row height instead of a hardcoded guess.
    pub(crate) row_height: Pixels,
    /// Time when the last parsing operation ended, used for smart debouncing
    pub(crate) last_parse_end_time: Option<std::time::Instant>,
}

pub fn init(cx: &mut App) {
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

    pub fn register(workspace: &mut Workspace) {
        workspace.register_action_renderer(|div, _, _, cx| {
            div.when(cx.has_flag::<TabularDataPreviewFeatureFlag>(), |div| {
                div.on_action(cx.listener(|workspace, _: &OpenPreview, window, cx| {
                    if let Some(editor) = Self::resolve_active_item_as_csv_editor(workspace, cx) {
                        let pane = workspace.active_pane().clone();
                        Self::open_preview_in_pane(editor, pane, window, cx);
                    }
                }))
                .on_action(cx.listener(
                    |workspace, _: &OpenPreviewToTheSide, window, cx| {
                        if let Some(editor) = Self::resolve_active_item_as_csv_editor(workspace, cx)
                        {
                            let pane = workspace.active_pane().clone();
                            Self::open_preview_to_the_side_of_pane(
                                workspace, editor, pane, window, cx,
                            );
                        }
                    },
                ))
            })
        });
    }

    pub fn open_preview_in_pane(
        editor: Entity<Editor>,
        pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        Self::activate_or_add_preview(editor, pane, true, window, cx);
    }

    pub fn open_preview_to_the_side_of_pane(
        workspace: &mut Workspace,
        editor: Entity<Editor>,
        origin_pane: Entity<Pane>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let target_pane = workspace.adjacent_pane_of(&origin_pane, window, cx);
        Self::activate_or_add_preview(editor, target_pane, false, window, cx);
    }

    fn activate_or_add_preview(
        editor: Entity<Editor>,
        pane: Entity<Pane>,
        focus: bool,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) {
        let existing_view_idx = Self::find_existing_preview_item_idx(pane.read(cx), &editor, cx);
        if let Some(existing_view_idx) = existing_view_idx {
            pane.update(cx, |pane, cx| {
                pane.activate_item(existing_view_idx, focus, focus, window, cx);
            });
        } else {
            let csv_preview = Self::new(&editor, window, cx);
            pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(csv_preview), focus, focus, None, window, cx);
            });
        }
        cx.notify();
    }

    fn find_existing_preview_item_idx(
        pane: &Pane,
        editor: &Entity<Editor>,
        cx: &App,
    ) -> Option<usize> {
        pane.items_of_type::<CsvPreviewView>()
            .find(|view| &view.read(cx).active_editor_state.editor == editor)
            .and_then(|view| pane.index_for_item(&view))
    }

    fn new(editor: &Entity<Editor>, window: &Window, cx: &mut Context<Workspace>) -> Entity<Self> {
        let contents = TableLikeContent::default();
        let table_interaction_state = cx.new(|cx| {
            TableInteractionState::new(cx).with_custom_scrollbar(ui::Scrollbars::for_settings::<
                editor::EditorSettingsScrollbarProxy,
            >())
        });

        cx.new(|cx| {
            let subscription = cx.subscribe(
                editor,
                |this: &mut CsvPreviewView, _editor, event: &EditorEvent, cx| {
                    match event {
                        EditorEvent::Edited { .. } | EditorEvent::DirtyChanged => {
                            this.parse_csv_from_active_editor(true, cx);
                        }
                        _ => {}
                    };
                },
            );

            let row_height = window.pixel_snap(window.line_height());
            let mut view = CsvPreviewView {
                focus_handle: cx.focus_handle(),
                active_editor_state: EditorState {
                    editor: editor.clone(),
                    _subscription: subscription,
                },
                table_interaction_state,
                column_widths: ColumnWidths::new(cx, 1),
                parsing_task: None,
                is_parsing: false,
                filter_sort_task: None,
                performance_metrics: PerformanceMetrics::default(),
                list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.))
                    .with_uniform_item_height(row_height),
                row_height,
                settings: CsvPreviewSettings::default(),
                last_parse_end_time: None,
                engine: TableDataEngine::default(),
            };

            view.parse_csv_from_active_editor(false, cx);
            view
        })
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
                // Uses the row height measured on the last render. Cheaper than a full
                // `.measure_all()` pass; exact row heights are re-measured on scrolling.
                view.list_state
                    .reset_with_uniform_height(visible_rows, view.row_height);
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

    pub fn is_csv_file(editor: &Entity<Editor>, cx: &App) -> bool {
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
