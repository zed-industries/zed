use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, actions};

use ui::{
    DefiniteLength, SharedString, Table, TableColumnWidths, TableInteractionState,
    TableResizeBehavior, prelude::*,
};
use workspace::{Item, Workspace};

use crate::{nasty_code_duplication::ColumnWidths, parsed_csv::ParsedCsv};

mod nasty_code_duplication;
mod parsed_csv;

actions!(csv, [OpenPreview]);

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, window, cx| {
        let Some(window) = window else {
            return;
        };
        CsvPreviewView::register(workspace, window, cx);
    })
    .detach()
}

pub struct CsvPreviewView {
    focus_handle: FocusHandle,
    _editor: Entity<Editor>,
    contents: ParsedCsv,
    table_interaction_state: Entity<TableInteractionState>,
    column_widths: ColumnWidths,
}

impl CsvPreviewView {
    pub fn register(
        workspace: &mut Workspace,
        _window: &mut Window,
        _cx: &mut Context<'_, Workspace>,
    ) {
        workspace.register_action(|workspace, _: &OpenPreview, window, cx| {
            if let Some(editor) = workspace
                .active_item(cx)
                .and_then(|item| item.act_as::<Editor>(cx))
                .filter(|editor| Self::is_csv_file(editor, cx))
            {
                let csv_preview = Self::from_editor(&editor, cx);
                workspace.add_item_to_active_pane(Box::new(csv_preview), None, true, window, cx);
            }
        });
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
                    .map(|ext| ext == "csv")
            })
            .unwrap_or(false)
    }

    fn from_editor(editor: &Entity<Editor>, cx: &mut Context<Workspace>) -> Entity<Self> {
        let raw_text = editor
            .read(cx)
            .buffer()
            .read(cx)
            .as_singleton()
            .map(|b| b.read(cx).text())
            .unwrap_or_else(|| "".to_string());

        let table_interaction_state = cx.new(|cx| TableInteractionState::new(cx));
        let contents = ParsedCsv::from_str(raw_text);

        cx.new(|cx| Self {
            focus_handle: cx.focus_handle(),
            _editor: editor.clone(),
            contents,
            table_interaction_state,
            column_widths: ColumnWidths::new(cx),
        })
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

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> SharedString {
        "CSV Preview".into()
    }
}

impl Render for CsvPreviewView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme();

        v_flex()
            .w_full()
            .h_full()
            .p_4()
            .bg(theme.colors().editor_background)
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .mb_4()
                    .child("CSV Preview"),
            )
            .child({
                if self.contents.headers.is_empty() {
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_32()
                        .text_ui(cx)
                        .text_color(cx.theme().colors().text_muted)
                        .child("No CSV content to display")
                        .into_any_element()
                } else {
                    let column_count = self.contents.headers.len();

                    self.render_table_with_cols(column_count, cx)
                }
            })
    }
}

impl CsvPreviewView {
    fn create_table<const COLS: usize>(
        &self,
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let widths = [DefiniteLength::Fraction(1. / COLS as f32); COLS];
        let resize_behaviors = [TableResizeBehavior::Resizable; COLS];

        self.create_table_inner(
            self.contents.rows.len(),
            widths,
            resize_behaviors,
            current_widths,
            cx,
        )
    }

    fn create_table_inner<const COLS: usize>(
        &self,
        row_count: usize,
        widths: [DefiniteLength; COLS],
        resize_behaviors: [TableResizeBehavior; COLS],
        current_widths: &Entity<TableColumnWidths<COLS>>,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        // Create headers array
        let mut headers = Vec::with_capacity(COLS);
        for i in 0..COLS {
            headers.push(
                self.contents
                    .headers
                    .get(i)
                    .cloned()
                    .unwrap_or_else(|| format!("Col {}", i + 1).into()),
            );
        }
        let headers_array: [SharedString; COLS] = headers.try_into().unwrap();

        Table::new()
            .interactable(&self.table_interaction_state)
            .striped()
            .column_widths(widths)
            .resizable_columns(resize_behaviors, current_widths, cx)
            .header(headers_array)
            .uniform_list(
                "csv-table",
                row_count,
                cx.processor(move |this, range: std::ops::Range<usize>, _window, _cx| {
                    range
                        .filter_map(|row_index| {
                            let row = this.contents.rows.get(row_index)?;

                            let mut elements = Vec::with_capacity(COLS);
                            for col in 0..COLS {
                                let cell_content: SharedString =
                                    row.get(col).cloned().unwrap_or_else(|| "".into());
                                elements.push(div().child(cell_content).into_any_element());
                            }

                            let elements_array: [gpui::AnyElement; COLS] =
                                elements.try_into().ok()?;
                            Some(elements_array)
                        })
                        .collect()
                }),
            )
            .into_any_element()
    }
}
