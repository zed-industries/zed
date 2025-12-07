use editor::Editor;
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, Task, actions};

use ui::{SharedString, TableInteractionState, prelude::*};
use workspace::{Item, Workspace};

use crate::{nasty_code_duplication::ColumnWidths, parsed_csv::ParsedCsv, parser::EditorState};

mod nasty_code_duplication;
mod parsed_csv;
mod parser;
mod renderer;

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

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum OrderingDirection {
    Asc,
    Desc,
}

#[derive(Clone, Copy)]
pub struct Ordering {
    /// 0-based column index
    pub col_idx: usize,
    /// Direction of ordering
    pub direction: OrderingDirection,
}

pub struct CsvPreviewView {
    pub(crate) focus_handle: FocusHandle,
    pub(crate) active_editor: Option<EditorState>,
    pub(crate) contents: ParsedCsv,
    pub(crate) table_interaction_state: Entity<TableInteractionState>,
    pub(crate) column_widths: ColumnWidths,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    pub(crate) ordering: Option<Ordering>,
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
        let table_interaction_state = cx.new(|cx| TableInteractionState::new(cx));
        let contents = ParsedCsv::default();

        cx.new(|cx| {
            let mut view = Self {
                focus_handle: cx.focus_handle(),
                active_editor: None,
                contents,
                table_interaction_state,
                column_widths: ColumnWidths::new(cx),
                parsing_task: None,
                ordering: None,
            };

            view.set_editor(editor.clone(), cx);
            view
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

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::FileDoc))
    }

    fn tab_content_text(&self, _detail: usize, cx: &App) -> SharedString {
        self.active_editor
            .as_ref()
            .and_then(|state| {
                state
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
            })
            .unwrap_or_else(|| SharedString::from("CSV Preview"))
    }
}
