use editor::{Editor, EditorEvent};
use gpui::{AppContext, Entity, EventEmitter, FocusHandle, Focusable, Task, actions};

use ui::{SharedString, prelude::*};
use workspace::{Item, Pane, Workspace};

use crate::parser::EditorState;

pub use crate::table_view::{PerformanceMetrics, TableView};

mod parser;
mod renderer;
mod settings;
mod table_data_engine;
mod table_view;
pub mod types;

actions!(tabular_data, [OpenPreview, OpenPreviewToTheSide]);

/// Editor-backed adapter: watches an [`Editor`], parses its buffer into a [`crate::types::TableLikeContent`],
/// and feeds the result to an embedded [`TableView`] that owns all grid rendering.
pub struct TabularDataPreviewPane {
    /// The reusable tabular viewer this adapter drives.
    pub(crate) table: Entity<TableView>,
    active_editor_state: EditorState,
    pub(crate) parsing_task: Option<Task<anyhow::Result<()>>>,
    /// Time when the last parsing operation ended, used for smart debouncing
    pub(crate) last_parse_end_time: Option<std::time::Instant>,
}

pub fn init(cx: &mut App) {
    cx.observe_new(|workspace: &mut Workspace, _, _| {
        TabularDataPreviewPane::register(workspace);
    })
    .detach()
}

impl TabularDataPreviewPane {
    pub fn register(workspace: &mut Workspace) {
        workspace.register_action_renderer(|div, _, _, cx| {
            div.on_action(cx.listener(|workspace, _: &OpenPreview, window, cx| {
                if let Some(editor) =
                    Self::resolve_active_item_as_tabular_data_editor(workspace, cx)
                {
                    let pane = workspace.active_pane().clone();
                    Self::open_preview_in_pane(editor, pane, window, cx);
                }
            }))
            .on_action(cx.listener(
                |workspace, _: &OpenPreviewToTheSide, window, cx| {
                    if let Some(editor) =
                        Self::resolve_active_item_as_tabular_data_editor(workspace, cx)
                    {
                        let pane = workspace.active_pane().clone();
                        Self::open_preview_to_the_side_of_pane(workspace, editor, pane, window, cx);
                    }
                },
            ))
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
        Self::activate_or_add_preview(editor.clone(), target_pane, false, window, cx);
        editor.focus_handle(cx).focus(window, cx);
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
            let preview_pane = Self::new(&editor, window, cx);
            pane.update(cx, |pane, cx| {
                pane.add_item(Box::new(preview_pane), focus, focus, None, window, cx);
            });
        }
        cx.notify();
    }

    fn find_existing_preview_item_idx(
        pane: &Pane,
        editor: &Entity<Editor>,
        cx: &App,
    ) -> Option<usize> {
        pane.items_of_type::<TabularDataPreviewPane>()
            .find(|view| &view.read(cx).active_editor_state.editor == editor)
            .and_then(|view| pane.index_for_item(&view))
    }

    fn new(editor: &Entity<Editor>, window: &Window, cx: &mut Context<Workspace>) -> Entity<Self> {
        cx.new(|cx| {
            let subscription = cx.subscribe(
                editor,
                |this: &mut TabularDataPreviewPane, _editor, event: &EditorEvent, cx| {
                    match event {
                        EditorEvent::Edited { .. } | EditorEvent::DirtyChanged => {
                            this.parse_from_active_editor(true, cx);
                        }
                        _ => {}
                    };
                },
            );

            let table = cx.new(|cx| TableView::new(window, cx));

            let mut view = TabularDataPreviewPane {
                active_editor_state: EditorState {
                    editor: editor.clone(),
                    _subscription: subscription,
                },
                table,
                parsing_task: None,
                last_parse_end_time: None,
            };

            view.parse_from_active_editor(false, cx);
            view
        })
    }

    pub(crate) fn editor_state(&self) -> &EditorState {
        &self.active_editor_state
    }

    pub fn resolve_active_item_as_tabular_data_editor(
        workspace: &Workspace,
        cx: &mut Context<Workspace>,
    ) -> Option<Entity<Editor>> {
        let editor = workspace
            .active_item(cx)
            .and_then(|item| item.act_as::<Editor>(cx))?;
        Self::is_tabular_data_file(&editor, cx).then_some(editor)
    }

    pub fn is_tabular_data_file(editor: &Entity<Editor>, cx: &App) -> bool {
        parser::TabularFormat::from_editor(editor, cx).is_some()
    }
}

impl Focusable for TabularDataPreviewPane {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        self.table.read(cx).focus_handle(cx)
    }
}

impl Render for TabularDataPreviewPane {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.table.clone())
    }
}

impl EventEmitter<()> for TabularDataPreviewPane {}

impl Item for TabularDataPreviewPane {
    type Event = ();

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(Icon::new(IconName::Table))
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
            .unwrap_or_else(|| SharedString::from("Tabular Data Preview"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use serde_json::json;
    use std::path::Path;
    use util::path;
    use workspace::AppState;

    #[gpui::test]
    async fn test_detects_tabular_files_outside_the_project(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/project"),
            json!({ "inside.csv": "a,b\n1,2\n", "inside.txt": "plain" }),
        )
        .await;
        fs.insert_tree(
            path!("/elsewhere"),
            json!({ "outside.csv": "a,b\n1,2\n", "outside.txt": "plain" }),
        )
        .await;

        let project = Project::test(fs, [Path::new(path!("/project"))], cx).await;

        for (abs_path, expected) in [
            (path!("/project/inside.csv"), true),
            (path!("/project/inside.txt"), false),
            (path!("/elsewhere/outside.csv"), true),
            (path!("/elsewhere/outside.txt"), false),
        ] {
            let buffer = project
                .update(cx, |project, cx| project.open_local_buffer(abs_path, cx))
                .await
                .unwrap();
            let (editor, _) = cx.add_window_view(|window, cx| {
                Editor::for_buffer(buffer, Some(project.clone()), window, cx)
            });
            let is_tabular =
                cx.update(|cx| TabularDataPreviewPane::is_tabular_data_file(&editor, cx));
            assert_eq!(is_tabular, expected, "{abs_path}");
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            AppState::test(cx);
            editor::init(cx);
        });
    }
}
