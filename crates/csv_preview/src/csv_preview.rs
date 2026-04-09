use editor::{Editor, EditorEvent};
use feature_flags::{FeatureFlag, FeatureFlagAppExt as _};
use gpui::{
    AppContext, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, Task, actions,
};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use crate::table_data_engine::TableDataEngine;
use ui::{
    AbsoluteLength, ResizableColumnsState, SharedString, TableInteractionState,
    TableResizeBehavior, prelude::*,
};
use workspace::{Item, SplitDirection, Workspace};

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
}

pub struct CsvPreviewView {
    pub(crate) engine: TableDataEngine,

    pub(crate) focus_handle: FocusHandle,
    active_editor_state: EditorState,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) settings: CsvPreviewSettings,
    /// Performance metrics for debugging and monitoring CSV operations.
    pub(crate) performance_metrics: PerformanceMetrics,
    pub(crate) list_state: gpui::ListState,
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
                    if let Some(editor) = workspace
                        .active_item(cx)
                        .and_then(|item| item.act_as::<Editor>(cx))
                        .filter(|editor| Self::is_csv_file(editor, cx))
                    {
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
                    }
                }))
                .on_action(cx.listener(
                    |workspace, _: &OpenPreviewToTheSide, window, cx| {
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
                    },
                ))
            })
        });
    }

    fn new(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
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

            let mut view = CsvPreviewView {
                focus_handle: cx.focus_handle(),
                active_editor_state: EditorState {
                    editor: editor.clone(),
                    _subscription: subscription,
                },
                table_interaction_state,
                column_widths: ColumnWidths::new(cx, 1),
                parsing_task: None,
                performance_metrics: PerformanceMetrics::default(),
                list_state: gpui::ListState::new(contents.rows.len(), ListAlignment::Top, px(1.)),
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
    pub(crate) fn apply_sort(&mut self) {
        self.performance_metrics.record("Sort", || {
            self.engine.apply_sort();
        });
    }

    /// Update ordered indices when ordering or content changes
    pub(crate) fn apply_filter_sort(&mut self) {
        self.performance_metrics.record("Filter&sort", || {
            self.engine.calculate_d2d_mapping();
        });

        // Update list state with filtered row count
        let visible_rows = self.engine.d2d_mapping().visible_row_count();
        self.list_state = gpui::ListState::new(visible_rows, ListAlignment::Top, px(100.));
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
                format!("{name}: {took:.2}ms {ago}s ago")
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
