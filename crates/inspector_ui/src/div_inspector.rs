use anyhow::Result;
use editor::{Bias, CompletionProvider, Editor, EditorEvent, EditorMode, ExcerptId, MultiBuffer};
use fuzzy::StringMatch;
use gpui::{
    AsyncWindowContext, DivInspectorState, Entity, InspectorElementId, IntoElement,
    StyleRefinement, Task, Window, inspector_reflection::FunctionReflection, styled_reflection,
};

use language::language_settings::SoftWrap;
use language::{Anchor, Buffer, BufferSnapshot, CodeLabel, Point, ToOffset as _, ToPoint as _};
use project::lsp_store::CompletionDocumentation;
use project::{Completion, CompletionSource, Project, ProjectPath};
use std::cell::RefCell;
use std::fmt::Write as _;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::sync::LazyLock;
use ui::{Label, LabelSize, Tooltip, prelude::*, styled_ext_reflection, v_flex};

/// Path used for unsaved buffer that contains style json. To support the json language server, this
/// matches the name used in the generated schemas.
const ZED_INSPECTOR_STYLE_JSON: &str = "/zed-inspector-style.json";

const ZED_INSPECTOR_STYLE_RUST: &str = "/zed-inspector-style.rs";

pub(crate) struct DivInspector {
    project: Entity<Project>,
    inspector_id: Option<InspectorElementId>,
    inspector_state: Option<DivInspectorState>,
    state: State,
    initial_style: StyleRefinement,
    json_style_overrides: StyleRefinement,
    json_style_error: Option<SharedString>,
    rust_completion: Option<String>,
    rust_completion_position: Option<Anchor>,
}

enum State {
    Loading,
    BuffersLoaded {
        rust_style_buffer: Entity<Buffer>,
        json_style_buffer: Entity<Buffer>,
    },
    Ready {
        rust_style_buffer: Entity<Buffer>,
        rust_style_editor: Entity<Editor>,
        json_style_buffer: Entity<Buffer>,
        json_style_editor: Entity<Editor>,
    },
    LoadError {
        message: SharedString,
    },
}

impl DivInspector {
    pub fn new(
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> DivInspector {
        // Open the buffers once, so they can then be used for each editor.
        cx.spawn_in(window, {
            let project = project.clone();
            async move |this, cx| {
                let json_style_buffer =
                    Self::open_buffer(ZED_INSPECTOR_STYLE_JSON, &project, cx).await;

                let rust_style_buffer =
                    Self::open_buffer(ZED_INSPECTOR_STYLE_RUST, &project, cx).await;

                match json_style_buffer.and_then(|json_style_buffer| {
                    rust_style_buffer
                        .map(|rust_style_buffer| (json_style_buffer, rust_style_buffer))
                }) {
                    Ok((json_style_buffer, rust_style_buffer)) => {
                        this.update_in(cx, |this, window, cx| {
                            this.state = State::BuffersLoaded {
                                json_style_buffer: json_style_buffer,
                                rust_style_buffer: rust_style_buffer,
                            };

                            // Initialize editors immediately instead of waiting for
                            // `update_inspected_element`. This avoids continuing to show
                            // "Loading..." until the user moves the mouse to a different element.
                            if let Some(id) = this.inspector_id.take() {
                                let inspector_state =
                                    window.with_inspector_state(Some(&id), cx, |state, _window| {
                                        state.clone()
                                    });
                                if let Some(inspector_state) = inspector_state {
                                    this.update_inspected_element(&id, inspector_state, window, cx);
                                    cx.notify();
                                }
                            }
                        })
                        .ok();
                    }
                    Err(err) => {
                        this.update(cx, |this, _cx| {
                            this.state = State::LoadError {
                                message: format!(
                                    "Failed to create buffers for style editing: {err}"
                                )
                                .into(),
                            };
                        })
                        .ok();
                    }
                }
            }
        })
        .detach();

        DivInspector {
            project,
            inspector_id: None,
            inspector_state: None,
            state: State::Loading,
            initial_style: StyleRefinement::default(),
            json_style_overrides: StyleRefinement::default(),
            rust_completion: None,
            rust_completion_position: None,
            json_style_error: None,
        }
    }

    pub fn update_inspected_element(
        &mut self,
        id: &InspectorElementId,
        inspector_state: DivInspectorState,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let style = (*inspector_state.base_style).clone();
        self.inspector_state = Some(inspector_state);

        if self.inspector_id.as_ref() == Some(id) {
            return;
        }
        self.inspector_id = Some(id.clone());

        self.initial_style = style.clone();
        self.json_style_overrides = StyleRefinement::default();

        let (rust_style_buffer, json_style_buffer) = match &self.state {
            State::BuffersLoaded {
                rust_style_buffer,
                json_style_buffer,
            }
            | State::Ready {
                rust_style_buffer,
                json_style_buffer,
                ..
            } => (rust_style_buffer.clone(), json_style_buffer.clone()),
            State::Loading | State::LoadError { .. } => return,
        };

        let style_json = match serde_json::to_string_pretty(&style) {
            Ok(style_json) => style_json,
            Err(err) => {
                self.state = State::BuffersLoaded {
                    rust_style_buffer: rust_style_buffer.clone(),
                    json_style_buffer: json_style_buffer.clone(),
                };
                self.json_style_error =
                    Some(format!("Failed to convert style to JSON: {err}").into());
                return;
            }
        };
        self.json_style_error = None;

        json_style_buffer.update(cx, |json_style_buffer, cx| {
            json_style_buffer.set_text(style_json, cx)
        });

        let json_style_editor = self.create_editor(json_style_buffer.clone(), window, cx);

        cx.subscribe_in(&json_style_editor, window, {
            let id = id.clone();
            let rust_style_buffer = rust_style_buffer.clone();
            move |this, editor, event: &EditorEvent, window, cx| match event {
                EditorEvent::BufferEdited => {
                    let style_json = editor.read(cx).text(cx);
                    match serde_json_lenient::from_str::<StyleRefinement>(&style_json) {
                        Ok(new_style) => {
                            // `json_style_overrides` is the parts of the json style that do not
                            // match (initial_style + rust_style). This allows for user edits to
                            // the json style to stick around after switching to edit the rust
                            // style.
                            let rust_style =
                                this.style_from_rust_buffer(rust_style_buffer.read(cx));
                            let mut initial_plus_rust = this.initial_style.clone();
                            initial_plus_rust.refine(&rust_style);
                            this.json_style_overrides = new_style.subtract(&initial_plus_rust);

                            window.with_inspector_state::<DivInspectorState, _>(
                                Some(&id),
                                cx,
                                |inspector_state, _window| {
                                    if let Some(inspector_state) = inspector_state.as_mut() {
                                        *inspector_state.base_style = new_style;
                                    }
                                },
                            );
                            window.refresh();
                            this.json_style_error = None;
                        }
                        Err(err) => this.json_style_error = Some(err.to_string().into()),
                    }
                }
                _ => {}
            }
        })
        .detach();

        rust_style_buffer.update(cx, |rust_style_buffer, cx| {
            rust_style_buffer.set_text(guess_rust_code_from_style(&style), cx)
        });

        let rust_style_editor = self.create_editor(rust_style_buffer.clone(), window, cx);

        let div_inspector = cx.entity();
        rust_style_editor.update(cx, |rust_style_editor, _cx| {
            rust_style_editor.set_completion_provider(Some(Rc::new(RustStyleCompletionProvider {
                div_inspector,
            })));
        });

        cx.subscribe_in(&rust_style_editor, window, {
            let json_style_editor = json_style_editor.clone();
            let rust_style_buffer = rust_style_buffer.clone();
            move |this, _editor, event: &EditorEvent, window, cx| match event {
                EditorEvent::BufferEdited => {
                    this.update_json_style_from_rust(
                        &json_style_editor,
                        &rust_style_buffer,
                        window,
                        cx,
                    );
                }
                _ => {}
            }
        })
        .detach();

        self.state = State::Ready {
            rust_style_buffer,
            rust_style_editor,
            json_style_buffer,
            json_style_editor,
        };
    }

    fn handle_rust_completion_selection_change(
        &mut self,
        rust_completion: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.rust_completion = rust_completion;
        if let State::Ready {
            rust_style_buffer,
            json_style_editor,
            ..
        } = &self.state
        {
            self.update_json_style_from_rust(
                &json_style_editor.clone(),
                &rust_style_buffer.clone(),
                window,
                cx,
            );
        }
    }

    fn update_json_style_from_rust(
        &mut self,
        json_style_editor: &Entity<Editor>,
        rust_style_buffer: &Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let rust_style = self.style_from_rust_buffer(rust_style_buffer.read(cx));

        // Preserve parts of the json style which do not come from the initial style or rust style.
        // This way user edits to the json style are preserved when they are not overridden by the
        // rust style.
        //
        // This results in a behavior where user changes to the json style that do overlap with the
        // rust style will get set to the rust style when the user switches back to the rust style
        // editor. It would be possible to update the rust style when the json style changes, but
        // this is undesireable as the user may be working on the actual code in the rust style.
        let mut new_style = self.initial_style.clone();
        new_style.refine(&self.json_style_overrides);
        let new_style = new_style.refined(rust_style);

        match serde_json::to_string_pretty(&new_style) {
            Ok(json) => {
                json_style_editor.update(cx, |json_style_editor, cx| {
                    json_style_editor.set_text(json, window, cx);
                });
            }
            Err(err) => {
                self.json_style_error = Some(err.to_string().into());
            }
        }
    }

    fn style_from_rust_buffer(&self, rust_style_buffer: &Buffer) -> StyleRefinement {
        let method_names = if let Some((completion, position)) = self
            .rust_completion
            .as_ref()
            .zip(self.rust_completion_position.as_ref())
        {
            let snapshot = rust_style_buffer.snapshot();
            let Range { start, end } = completion_replace_range(&snapshot, position)
                .unwrap_or(position.clone()..position.clone());
            let before_text = snapshot
                .text_for_range(0..start.to_offset(&snapshot))
                .collect::<String>();
            let after_text = snapshot
                .text_for_range(
                    end.to_offset(&snapshot)..snapshot.clip_offset(usize::MAX, Bias::Left),
                )
                .collect::<String>();
            let mut method_names = before_text
                .split(is_not_identifier_char)
                .map(|name| name.to_string())
                .collect::<Vec<_>>();
            method_names.push(completion.clone());
            method_names.extend(
                after_text
                    .split(is_not_identifier_char)
                    .map(|name| name.to_string()),
            );
            method_names
        } else {
            rust_style_buffer
                .text()
                .split(is_not_identifier_char)
                .map(|name| name.to_string())
                .collect::<Vec<_>>()
        };
        style_from_method_names(method_names)
    }

    async fn open_buffer(
        path: impl AsRef<Path>,
        project: &Entity<Project>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Entity<Buffer>> {
        let worktree = project
            .update(cx, |project, cx| project.create_worktree(path, false, cx))?
            .await?;

        let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
            worktree_id: worktree.id(),
            path: Path::new("").into(),
        })?;

        let buffer = project
            .update(cx, |project, cx| project.open_path(project_path, cx))?
            .await?
            .1;

        Ok(buffer)
    }

    fn create_editor(
        &self,
        buffer: Entity<Buffer>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Entity<Editor> {
        cx.new(|cx| {
            let multi_buffer = cx.new(|cx| MultiBuffer::singleton(buffer, cx));
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
        })
    }
}

impl Render for DivInspector {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .size_full()
            .gap_2()
            .when_some(self.inspector_state.as_ref(), |this, inspector_state| {
                this.child(
                    v_flex()
                        .child(Label::new("Layout").size(LabelSize::Large))
                        .child(render_layout_state(inspector_state, cx)),
                )
            })
            .map(|this| match &self.state {
                State::Loading | State::BuffersLoaded { .. } => {
                    this.child(Label::new("Loading..."))
                }
                State::LoadError { message } => this.child(
                    div()
                        .w_full()
                        .border_1()
                        .border_color(Color::Error.color(cx))
                        .child(Label::new(message)),
                ),
                State::Ready {
                    rust_style_editor,
                    json_style_editor,
                    ..
                } => this
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Label::new("Rust Style").size(LabelSize::Large))
                            .child(div().h_64().child(rust_style_editor.clone())),
                    )
                    .child(
                        v_flex()
                            .gap_2()
                            .child(Label::new("JSON Style").size(LabelSize::Large))
                            .child(div().h_128().child(json_style_editor.clone()))
                            .when_some(self.json_style_error.as_ref(), |this, last_error| {
                                this.child(
                                    div()
                                        .w_full()
                                        .border_1()
                                        .border_color(Color::Error.color(cx))
                                        .child(Label::new(last_error)),
                                )
                            }),
                    ),
            })
            .into_any_element()
    }
}

fn render_layout_state(inspector_state: &DivInspectorState, cx: &App) -> Div {
    v_flex()
        .child(
            div()
                .text_ui(cx)
                .child(format!("Bounds: {}", inspector_state.bounds)),
        )
        .child(
            div()
                .id("content-size")
                .text_ui(cx)
                .tooltip(Tooltip::text("Size of the element's children"))
                .child(
                    if inspector_state.content_size != inspector_state.bounds.size {
                        format!("Content size: {}", inspector_state.content_size)
                    } else {
                        "".to_string()
                    },
                ),
        )
}

static STYLE_METHODS: LazyLock<Vec<(Box<StyleRefinement>, FunctionReflection<StyleRefinement>)>> =
    LazyLock::new(|| {
        // Include StyledExt methods first so that those methods take precedence.
        styled_ext_reflection::methods::<StyleRefinement>()
            .into_iter()
            .chain(styled_reflection::methods::<StyleRefinement>())
            .map(|method| (Box::new(method.invoke(StyleRefinement::default())), method))
            .collect()
    });

fn guess_rust_code_from_style(goal_style: &StyleRefinement) -> String {
    let mut subset_methods = Vec::new();
    for (style, method) in STYLE_METHODS.iter() {
        if goal_style.is_superset_of(style) {
            subset_methods.push(method);
        }
    }

    let mut result = "fn build() -> Div {\n    div()".to_string();
    let mut style = StyleRefinement::default();
    for method in subset_methods {
        let before_change = style.clone();
        style = method.invoke(style);
        if before_change != style {
            let _ = write!(result, "\n        .{}()", &method.name);
        }
    }
    result.push_str("\n}");
    result
}

fn style_from_method_names(method_names: impl IntoIterator<Item = String>) -> StyleRefinement {
    let mut style = StyleRefinement::default();
    for name in method_names {
        if let Some((_, method)) = STYLE_METHODS.iter().find(|(_, m)| m.name == name) {
            style = method.invoke(style);
        }
    }
    style
}

fn is_not_identifier_char(c: char) -> bool {
    !c.is_alphanumeric() && c != '_'
}

struct RustStyleCompletionProvider {
    div_inspector: Entity<DivInspector>,
}

impl CompletionProvider for RustStyleCompletionProvider {
    fn completions(
        &self,
        _excerpt_id: ExcerptId,
        buffer: &Entity<Buffer>,
        position: Anchor,
        _: editor::CompletionContext,
        _window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> Task<Result<Option<Vec<project::Completion>>>> {
        self.div_inspector.update(cx, |div_inspector, _cx| {
            div_inspector.rust_completion_position = Some(position.clone());
        });

        let Some(replace_range) = completion_replace_range(&buffer.read(cx).snapshot(), &position)
        else {
            return Task::ready(Ok(Some(Vec::new())));
        };

        Task::ready(Ok(Some(
            STYLE_METHODS
                .iter()
                .map(|(_, method)| Completion {
                    replace_range: replace_range.clone(),
                    new_text: format!(".{}()", method.name),
                    label: CodeLabel::plain(method.name.to_string(), None),
                    icon_path: None,
                    documentation: method.documentation.map(|documentation| {
                        CompletionDocumentation::MultiLineMarkdown(documentation.into())
                    }),
                    source: CompletionSource::Custom,
                    insert_text_mode: None,
                    confirm: None,
                })
                .collect(),
        )))
    }

    fn resolve_completions(
        &self,
        _buffer: Entity<Buffer>,
        _completion_indices: Vec<usize>,
        _completions: Rc<RefCell<Box<[Completion]>>>,
        _cx: &mut Context<Editor>,
    ) -> Task<Result<bool>> {
        Task::ready(Ok(true))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<language::Buffer>,
        position: language::Anchor,
        _: &str,
        _: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        completion_replace_range(&buffer.read(cx).snapshot(), &position).is_some()
    }

    fn selection_changed(&self, mat: Option<&StringMatch>, window: &mut Window, cx: &mut App) {
        let div_inspector = self.div_inspector.clone();
        let rust_completion = mat.as_ref().map(|mat| mat.string.clone());
        window.defer(cx, move |window, cx| {
            div_inspector.update(cx, |div_inspector, cx| {
                div_inspector.handle_rust_completion_selection_change(rust_completion, window, cx);
            });
        });
    }

    fn sort_completions(&self) -> bool {
        false
    }
}

fn completion_replace_range(snapshot: &BufferSnapshot, anchor: &Anchor) -> Option<Range<Anchor>> {
    let point = anchor.to_point(&snapshot);
    let offset = point.to_offset(&snapshot);
    let line_start = Point::new(point.row, 0).to_offset(&snapshot);
    let line_end = Point::new(point.row, snapshot.line_len(point.row)).to_offset(&snapshot);
    let mut lines = snapshot.text_for_range(line_start..line_end).lines();
    let line = lines.next()?;

    let start_in_line = &line[..offset - line_start]
        .rfind(|c| is_not_identifier_char(c) && c != '.')
        .map(|ix| ix + 1)
        .unwrap_or(0);
    let end_in_line = &line[offset - line_start..]
        .rfind(|c| is_not_identifier_char(c) && c != '(' && c != ')')
        .unwrap_or(line_end - line_start);

    if end_in_line > start_in_line {
        let replace_start = snapshot.anchor_before(line_start + start_in_line);
        let replace_end = snapshot.anchor_before(line_start + end_in_line);
        Some(replace_start..replace_end)
    } else {
        None
    }
}
