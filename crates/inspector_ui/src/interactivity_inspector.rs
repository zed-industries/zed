use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use editor::{Editor, EditorEvent, EditorMode, MultiBuffer};
use futures::{FutureExt as _, future::Shared};
use gpui::{
    App, AsyncWindowContext, Entity, InspectorElementId, InteractivityInspectorState, IntoElement,
    Task, Window,
};
use language::Buffer;
use language::language_settings::SoftWrap;
use project::{Project, ProjectPath};
use ui::prelude::*;
use ui::{Label, LabelSize, v_flex};
use util::ResultExt as _;
use workspace::Workspace;

// todo! rename back to DivInspector
pub(crate) struct InteractivityInspector {
    id: Option<InspectorElementId>,
    project: Entity<Project>,
    style_buffer: Option<Entity<Buffer>>,
    style_editor: Option<Entity<Editor>>,
}

// todo! Remove unwraps
impl InteractivityInspector {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> InteractivityInspector {
        let workspace = window.root::<Workspace>().flatten().unwrap();
        let project = workspace.read(cx).project().clone();

        let worktree_id = project
            .read(cx)
            .worktrees(cx)
            .filter(|worktree| {
                let worktree = worktree.read(cx);
                !worktree.is_single_file() && worktree.is_local()
            })
            .next()
            .unwrap()
            .read(cx)
            .id();
        let project_path = ProjectPath {
            worktree_id,
            path: Path::new("zed-inspector-style.json").into(),
        };

        // Load the buffer once, so it can then be used for each editor.
        cx.spawn_in(window, {
            let project = project.clone();
            async move |this, cx| {
                // todo! Make a new project instead of needing the current window to be a workspace.

                let style_buffer = project
                    .update(cx, |project, cx| project.open_path(project_path, cx))
                    .unwrap()
                    .await
                    .unwrap()
                    .1;

                project
                    .update(cx, |project, cx| {
                        project.register_buffer_with_language_servers(&style_buffer, cx)
                    })
                    .ok();

                this.update_in(cx, |this, window, cx| {
                    this.style_buffer = Some(style_buffer);
                    // TODO: Avoid clone somehow
                    if let Some(id) = this.id.clone() {
                        window.update_inspector_state(&id, |state, window| {
                            if let Some(state) = state.as_ref() {
                                this.update_inspected_element(&id, state, window, cx)
                            }
                        });
                    }
                })
                .ok();
            }
        })
        .detach();

        InteractivityInspector {
            id: None,
            project,
            style_buffer: None,
            style_editor: None,
        }
    }

    pub fn update_inspected_element(
        &mut self,
        id: &InspectorElementId,
        state: &InteractivityInspectorState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.id.as_ref() == Some(id) {
            return;
        }
        let Some(style_buffer) = self.style_buffer.clone() else {
            return;
        };

        let base_style_json = serde_json::to_string_pretty(&state.base_style).unwrap();
        style_buffer.update(cx, |style_buffer, cx| {
            style_buffer.set_text(base_style_json, cx)
        });

        let style_editor = cx.new(|cx| {
            let multi_buffer = cx.new(|cx| MultiBuffer::singleton(style_buffer, cx));
            let mut editor = Editor::new(
                EditorMode::full(),
                multi_buffer,
                Some(self.project.clone()),
                window,
                cx,
            );
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor.set_show_line_numbers(false, cx);
            editor.set_show_code_actions(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.set_show_edit_predictions(Some(false), window, cx);
            editor
        });

        window
            .subscribe(&style_editor, cx, {
                let id = id.clone();
                move |editor, event: &EditorEvent, window, cx| {
                    match event {
                        EditorEvent::BufferEdited => {
                            let base_style_json = editor.read(cx).text(cx);
                            // todo! error handling
                            let Some(new_base_style) =
                                serde_json_lenient::from_str(&base_style_json).log_err()
                            else {
                                return;
                            };
                            window.update_inspector_state::<InteractivityInspectorState, _>(
                                &id,
                                |state, _window| {
                                    if let Some(state) = state.as_mut() {
                                        *state.base_style = new_base_style;
                                    }
                                },
                            )
                        }
                        _ => {}
                    }
                }
            })
            .detach();

        self.id = Some(id.clone());
        self.style_editor = Some(style_editor);
        cx.notify();
    }
}

impl Render for InteractivityInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(style_editor) = self.style_editor.as_ref() {
            v_flex()
                .size_full()
                .bg(cx.theme().colors().panel_background)
                .gap_2()
                .child(Label::new("Style").size(LabelSize::Large))
                .child(div().h_128().child(style_editor.clone()))
                .into_any_element()
        } else {
            Label::new("Loading...").into_any_element()
        }
    }
}
