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
use ui::{Label, LabelSize, Tooltip, prelude::*, v_flex};

/// Path used for unsaved buffer that contains style json. To support the json lanugage server, this
/// matches the name used in the generated schemas.
const ZED_INSPECTOR_STYLE_PATH: &str = "/zed-inspector-style.json";

pub(crate) struct DivInspector {
    project: Entity<Project>,
    inspector_id: Option<InspectorElementId>,
    state: Option<DivInspectorState>,
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
            inspector_id: None,
            state: None,
            style_buffer: None,
            style_editor: None,
            last_error: None,
        }
    }

    // TODO: Buffer will leak memory over time due to building up history. Instead of using
    // `project` to create it, should just directly build the buffer and File.
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
            if let Some(id) = this.inspector_id.clone() {
                let state =
                    window.with_inspector_state(Some(&id), cx, |state, _window| state.clone());
                if let Some(state) = state {
                    this.update_inspected_element(&id, state, window, cx);
                    cx.notify();
                }
            }
        })?;

        Ok(())
    }

    pub fn update_inspected_element(
        &mut self,
        id: &InspectorElementId,
        state: DivInspectorState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let base_style_json = serde_json::to_string_pretty(&state.base_style);
        self.state = Some(state);

        if self.inspector_id.as_ref() == Some(id) {
            return;
        } else {
            self.inspector_id = Some(id.clone());
        }
        let Some(style_buffer) = self.style_buffer.clone() else {
            return;
        };

        let base_style_json = match base_style_json {
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
                            window.with_inspector_state::<DivInspectorState, _>(
                                Some(&id),
                                cx,
                                |state, _window| {
                                    if let Some(state) = state.as_mut() {
                                        *state.base_style = new_base_style;
                                    }
                                },
                            );
                            window.refresh();
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
    }
}

impl Render for DivInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .when_some(self.state.as_ref(), |this, state| {
                this.child(
                    v_flex()
                        .child(Label::new("Layout").size(LabelSize::Large))
                        .child(render_layout_state(state, cx)),
                )
            })
            .when_some(self.style_editor.as_ref(), |this, style_editor| {
                this.child(
                    v_flex()
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
                        }),
                )
            })
            .when_none(&self.style_editor, |this| {
                this.child(Label::new("Loading..."))
            })
            .into_any_element()
    }
}

fn render_layout_state(state: &DivInspectorState, cx: &App) -> Div {
    v_flex()
        .child(div().text_ui(cx).child(format!("Bounds: {}", state.bounds)))
        .child(
            div()
                .id("content-size")
                .text_ui(cx)
                .tooltip(Tooltip::text("Size of the element's children"))
                .child(if state.content_size != state.bounds.size {
                    format!("Content size: {}", state.content_size)
                } else {
                    "".to_string()
                }),
        )
}
