use anyhow::Result;
use editor::{Editor, EditorEvent, EditorMode, MultiBuffer};
use gpui::{
    AsyncWindowContext, DivInspectorState, Entity, InspectorElementId, IntoElement, WeakEntity,
    Window,
};
use language::Buffer;
use language::language_settings::SoftWrap;
use project::{Project, ProjectPath};
use std::path::Path;
use ui::prelude::*;
use ui::{Label, LabelSize, v_flex};

/// Path used for unsaved buffer that contains style json. To support the json lanugage server, this
/// matches the name used in the generated schemas.
const ZED_INSPECTOR_STYLE_PATH: &str = "/zed-inspector-style.json";

pub(crate) struct DivInspector {
    project: Entity<Project>,
    last_id: Option<InspectorElementId>,
    style_buffer: Option<Entity<Buffer>>,
    style_editor: Option<Entity<Editor>>,
    last_error: Option<SharedString>,
}

impl DivInspector {
    pub fn new(
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DivInspector {
        // Open the buffer once, so it can then be used for each editor.
        cx.spawn_in(window, {
            let project = project.clone();
            async move |this, cx| Self::open_style_buffer(project, this, cx).await
        })
        .detach();

        DivInspector {
            project,
            last_id: None,
            style_buffer: None,
            style_editor: None,
            last_error: None,
        }
    }

    async fn open_style_buffer(
        project: Entity<Project>,
        this: WeakEntity<DivInspector>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let worktree = project
            .update(cx, |project, cx| {
                project.create_worktree(ZED_INSPECTOR_STYLE_PATH, false, cx)
            })?
            .await?;

        let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
            worktree_id: worktree.id(),
            path: Path::new("").into(),
        })?;

        let style_buffer = project
            .update(cx, |project, cx| project.open_path(project_path, cx))?
            .await?
            .1;

        project.update(cx, |project, cx| {
            project.register_buffer_with_language_servers(&style_buffer, cx)
        })?;

        this.update_in(cx, |this, window, cx| {
            this.style_buffer = Some(style_buffer);
            // TODO: Avoid clone somehow
            if let Some(id) = this.last_id.clone() {
                window.update_inspector_state(&id, |state, window| {
                    if let Some(state) = state.as_ref() {
                        this.update_inspected_element(&id, state, window, cx);
                    }
                });
            }
        })?;

        Ok(())
    }

    pub fn update_inspected_element(
        &mut self,
        id: &InspectorElementId,
        state: &DivInspectorState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.last_id.as_ref() == Some(id) {
            return;
        }
        let Some(style_buffer) = self.style_buffer.clone() else {
            return;
        };
        self.last_id = Some(id.clone());

        let base_style_json = match serde_json::to_string_pretty(&state.base_style) {
            Ok(base_style_json) => base_style_json,
            Err(err) => {
                self.style_editor = None;
                self.last_error =
                    Some(format!("Failed to convert base_style to JSON: {err}").into());
                return;
            }
        };
        self.last_error = None;

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

        cx.subscribe_in(&style_editor, window, {
            let id = id.clone();
            move |this, editor, event: &EditorEvent, window, cx| match event {
                EditorEvent::BufferEdited => {
                    let base_style_json = editor.read(cx).text(cx);
                    match serde_json_lenient::from_str(&base_style_json) {
                        Ok(new_base_style) => {
                            window.update_inspector_state::<DivInspectorState, _>(
                                &id,
                                |state, _window| {
                                    if let Some(state) = state.as_mut() {
                                        *state.base_style = new_base_style;
                                    }
                                },
                            );
                            this.last_error = None;
                        }
                        Err(err) => this.last_error = Some(err.to_string().into()),
                    }
                }
                _ => {}
            }
        })
        .detach();

        self.style_editor = Some(style_editor);
        cx.notify();
    }
}

impl Render for DivInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if let Some(style_editor) = self.style_editor.as_ref() {
            v_flex()
                .size_full()
                .gap_2()
                .child(Label::new("Style").size(LabelSize::Large))
                .child(div().h_128().child(style_editor.clone()))
                .when_some(self.last_error.as_ref(), |this, last_error| {
                    this.child(
                        div()
                            .w_full()
                            .border_1()
                            .border_color(Color::Error.color(cx))
                            .child(Label::new(last_error)),
                    )
                })
                .into_any_element()
        } else {
            Label::new("Loading...").into_any_element()
        }
    }
}
