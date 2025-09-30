use anyhow::{Result, anyhow};
use editor::{
    Bias, CompletionProvider, Editor, EditorEvent, EditorMode, ExcerptId, MinimapVisibility,
    MultiBuffer,
};
use fuzzy::StringMatch;
use gpui::{
    AsyncWindowContext, DivInspectorState, Entity, InspectorElementId, IntoElement,
    StyleRefinement, Task, Window, inspector_reflection::FunctionReflection, styled_reflection,
};
use language::language_settings::SoftWrap;
use language::{
    Anchor, Buffer, BufferSnapshot, CodeLabel, Diagnostic, DiagnosticEntry, DiagnosticSet,
    DiagnosticSeverity, LanguageServerId, Point, ToOffset as _, ToPoint as _,
};
use project::lsp_store::CompletionDocumentation;
use project::{
    Completion, CompletionDisplayOptions, CompletionResponse, CompletionSource, Project,
    ProjectPath,
};
use std::fmt::Write as _;
use std::ops::Range;
use std::path::Path;
use std::rc::Rc;
use std::sync::LazyLock;
use ui::{Label, LabelSize, Tooltip, prelude::*, styled_ext_reflection, v_flex};
use util::rel_path::RelPath;
use util::split_str_with_ranges;

/// Path used for unsaved buffer that contains style json. To support the json language server, this
/// matches the name used in the generated schemas.
const ZED_INSPECTOR_STYLE_JSON: &str = util_macros::path!("/zed-inspector-style.json");

pub(crate) struct DivInspector {
    state: State,
    project: Entity<Project>,
    inspector_id: Option<InspectorElementId>,
    inspector_state: Option<DivInspectorState>,
    /// Value of `DivInspectorState.base_style` when initially picked.
    initial_style: StyleRefinement,
    /// Portion of `initial_style` that can't be converted to rust code.
    unconvertible_style: StyleRefinement,
    /// Edits the user has made to the json buffer: `json_editor - (unconvertible_style + rust_editor)`.
    json_style_overrides: StyleRefinement,
    /// Error to display from parsing the json, or if serialization errors somehow occur.
    json_style_error: Option<SharedString>,
    /// Currently selected completion.
    rust_completion: Option<String>,
    /// Range that will be replaced by the completion if selected.
    rust_completion_replace_range: Option<Range<Anchor>>,
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
            let languages = project.read(cx).languages().clone();
            let project = project.clone();
            async move |this, cx| {
                // Open the JSON style buffer in the inspector-specific project, so that it runs the
                // JSON language server.
                let json_style_buffer =
                    Self::create_buffer_in_project(ZED_INSPECTOR_STYLE_JSON, &project, cx).await;

                // Create Rust style buffer without adding it to the project / buffer_store, so that
                // Rust Analyzer doesn't get started for it.
                let rust_language_result = languages.language_for_name("Rust").await;
                let rust_style_buffer = rust_language_result.and_then(|rust_language| {
                    cx.new(|cx| Buffer::local("", cx).with_language(rust_language, cx))
                });

                match json_style_buffer.and_then(|json_style_buffer| {
                    rust_style_buffer
                        .map(|rust_style_buffer| (json_style_buffer, rust_style_buffer))
                }) {
                    Ok((json_style_buffer, rust_style_buffer)) => {
                        this.update_in(cx, |this, window, cx| {
                            this.state = State::BuffersLoaded {
                                json_style_buffer,
                                rust_style_buffer,
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
            state: State::Loading,
            project,
            inspector_id: None,
            inspector_state: None,
            initial_style: StyleRefinement::default(),
            unconvertible_style: StyleRefinement::default(),
            json_style_overrides: StyleRefinement::default(),
            rust_completion: None,
            rust_completion_replace_range: None,
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

        let json_style_editor = self.create_editor(json_style_buffer.clone(), window, cx);
        let rust_style_editor = self.create_editor(rust_style_buffer.clone(), window, cx);

        rust_style_editor.update(cx, {
            let div_inspector = cx.entity();
            |rust_style_editor, _cx| {
                rust_style_editor.set_completion_provider(Some(Rc::new(
                    RustStyleCompletionProvider { div_inspector },
                )));
            }
        });

        let rust_style = match self.reset_style_editors(&rust_style_buffer, &json_style_buffer, cx)
        {
            Ok(rust_style) => {
                self.json_style_error = None;
                rust_style
            }
            Err(err) => {
                self.json_style_error = Some(format!("{err}").into());
                return;
            }
        };

        cx.subscribe_in(&json_style_editor, window, {
            let id = id.clone();
            let rust_style_buffer = rust_style_buffer.clone();
            move |this, editor, event: &EditorEvent, window, cx| {
                if event == &EditorEvent::BufferEdited {
                    let style_json = editor.read(cx).text(cx);
                    match serde_json_lenient::from_str_lenient::<StyleRefinement>(&style_json) {
                        Ok(new_style) => {
                            let (rust_style, _) = this.style_from_rust_buffer_snapshot(
                                &rust_style_buffer.read(cx).snapshot(),
                            );

                            let mut unconvertible_plus_rust = this.unconvertible_style.clone();
                            unconvertible_plus_rust.refine(&rust_style);

                            // The serialization of `DefiniteLength::Fraction` does not perfectly
                            // roundtrip because with f32, `(x / 100.0 * 100.0) == x` is not always
                            // true (such as for `p_1_3`). This can cause these values to
                            // erroneously appear in `json_style_overrides` since they are not
                            // perfectly equal. Roundtripping before `subtract` fixes this.
                            unconvertible_plus_rust =
                                serde_json::to_string(&unconvertible_plus_rust)
                                    .ok()
                                    .and_then(|json| {
                                        serde_json_lenient::from_str_lenient(&json).ok()
                                    })
                                    .unwrap_or(unconvertible_plus_rust);

                            this.json_style_overrides =
                                new_style.subtract(&unconvertible_plus_rust);

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
            }
        })
        .detach();

        cx.subscribe(&rust_style_editor, {
            let json_style_buffer = json_style_buffer.clone();
            let rust_style_buffer = rust_style_buffer.clone();
            move |this, _editor, event: &EditorEvent, cx| {
                if let EditorEvent::BufferEdited = event {
                    this.update_json_style_from_rust(&json_style_buffer, &rust_style_buffer, cx);
                }
            }
        })
        .detach();

        self.unconvertible_style = style.subtract(&rust_style);
        self.json_style_overrides = StyleRefinement::default();
        self.state = State::Ready {
            rust_style_buffer,
            rust_style_editor,
            json_style_buffer,
            json_style_editor,
        };
    }

    fn reset_style(&mut self, cx: &mut App) {
        if let State::Ready {
            rust_style_buffer,
            json_style_buffer,
            ..
        } = &self.state
        {
            if let Err(err) =
                self.reset_style_editors(&rust_style_buffer.clone(), &json_style_buffer.clone(), cx)
            {
                self.json_style_error = Some(format!("{err}").into());
            } else {
                self.json_style_error = None;
            }
        }
    }

    fn reset_style_editors(
        &self,
        rust_style_buffer: &Entity<Buffer>,
        json_style_buffer: &Entity<Buffer>,
        cx: &mut App,
    ) -> Result<StyleRefinement> {
        let json_text = match serde_json::to_string_pretty(&self.initial_style) {
            Ok(json_text) => json_text,
            Err(err) => {
                return Err(anyhow!("Failed to convert style to JSON: {err}"));
            }
        };

        let (rust_code, rust_style) = guess_rust_code_from_style(&self.initial_style);
        rust_style_buffer.update(cx, |rust_style_buffer, cx| {
            rust_style_buffer.set_text(rust_code, cx);
            let snapshot = rust_style_buffer.snapshot();
            let (_, unrecognized_ranges) = self.style_from_rust_buffer_snapshot(&snapshot);
            Self::set_rust_buffer_diagnostics(
                unrecognized_ranges,
                rust_style_buffer,
                &snapshot,
                cx,
            );
        });
        json_style_buffer.update(cx, |json_style_buffer, cx| {
            json_style_buffer.set_text(json_text, cx);
        });

        Ok(rust_style)
    }

    fn handle_rust_completion_selection_change(
        &mut self,
        rust_completion: Option<String>,
        cx: &mut Context<Self>,
    ) {
        self.rust_completion = rust_completion;
        if let State::Ready {
            rust_style_buffer,
            json_style_buffer,
            ..
        } = &self.state
        {
            self.update_json_style_from_rust(
                &json_style_buffer.clone(),
                &rust_style_buffer.clone(),
                cx,
            );
        }
    }

    fn update_json_style_from_rust(
        &mut self,
        json_style_buffer: &Entity<Buffer>,
        rust_style_buffer: &Entity<Buffer>,
        cx: &mut Context<Self>,
    ) {
        let rust_style = rust_style_buffer.update(cx, |rust_style_buffer, cx| {
            let snapshot = rust_style_buffer.snapshot();
            let (rust_style, unrecognized_ranges) = self.style_from_rust_buffer_snapshot(&snapshot);
            Self::set_rust_buffer_diagnostics(
                unrecognized_ranges,
                rust_style_buffer,
                &snapshot,
                cx,
            );
            rust_style
        });

        // Preserve parts of the json style which do not come from the unconvertible style or rust
        // style. This way user edits to the json style are preserved when they are not overridden
        // by the rust style.
        //
        // This results in a behavior where user changes to the json style that do overlap with the
        // rust style will get set to the rust style when the user edits the rust style. It would be
        // possible to update the rust style when the json style changes, but this is undesirable
        // as the user may be working on the actual code in the rust style.
        let mut new_style = self.unconvertible_style.clone();
        new_style.refine(&self.json_style_overrides);
        let new_style = new_style.refined(rust_style);

        match serde_json::to_string_pretty(&new_style) {
            Ok(json) => {
                json_style_buffer.update(cx, |json_style_buffer, cx| {
                    json_style_buffer.set_text(json, cx);
                });
            }
            Err(err) => {
                self.json_style_error = Some(err.to_string().into());
            }
        }
    }

    fn style_from_rust_buffer_snapshot(
        &self,
        snapshot: &BufferSnapshot,
    ) -> (StyleRefinement, Vec<Range<Anchor>>) {
        let method_names = if let Some((completion, completion_range)) = self
            .rust_completion
            .as_ref()
            .zip(self.rust_completion_replace_range.as_ref())
        {
            let before_text = snapshot
                .text_for_range(0..completion_range.start.to_offset(snapshot))
                .collect::<String>();
            let after_text = snapshot
                .text_for_range(
                    completion_range.end.to_offset(snapshot)
                        ..snapshot.clip_offset(usize::MAX, Bias::Left),
                )
                .collect::<String>();
            let mut method_names = split_str_with_ranges(&before_text, is_not_identifier_char)
                .into_iter()
                .map(|(range, name)| (Some(range), name.to_string()))
                .collect::<Vec<_>>();
            method_names.push((None, completion.clone()));
            method_names.extend(
                split_str_with_ranges(&after_text, is_not_identifier_char)
                    .into_iter()
                    .map(|(range, name)| (Some(range), name.to_string())),
            );
            method_names
        } else {
            split_str_with_ranges(&snapshot.text(), is_not_identifier_char)
                .into_iter()
                .map(|(range, name)| (Some(range), name.to_string()))
                .collect::<Vec<_>>()
        };

        let mut style = StyleRefinement::default();
        let mut unrecognized_ranges = Vec::new();
        for (range, name) in method_names {
            if let Some((_, method)) = STYLE_METHODS.iter().find(|(_, m)| m.name == name) {
                style = method.invoke(style);
            } else if let Some(range) = range {
                unrecognized_ranges
                    .push(snapshot.anchor_before(range.start)..snapshot.anchor_before(range.end));
            }
        }

        (style, unrecognized_ranges)
    }

    fn set_rust_buffer_diagnostics(
        unrecognized_ranges: Vec<Range<Anchor>>,
        rust_style_buffer: &mut Buffer,
        snapshot: &BufferSnapshot,
        cx: &mut Context<Buffer>,
    ) {
        let diagnostic_entries = unrecognized_ranges
            .into_iter()
            .enumerate()
            .map(|(ix, range)| DiagnosticEntry {
                range,
                diagnostic: Diagnostic {
                    message: "unrecognized".to_string(),
                    severity: DiagnosticSeverity::WARNING,
                    is_primary: true,
                    group_id: ix,
                    ..Default::default()
                },
            });
        let diagnostics = DiagnosticSet::from_sorted_entries(diagnostic_entries, snapshot);
        rust_style_buffer.update_diagnostics(LanguageServerId(0), diagnostics, cx);
    }

    async fn create_buffer_in_project(
        path: impl AsRef<Path>,
        project: &Entity<Project>,
        cx: &mut AsyncWindowContext,
    ) -> Result<Entity<Buffer>> {
        let worktree = project
            .update(cx, |project, cx| project.create_worktree(path, false, cx))?
            .await?;

        let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
            worktree_id: worktree.id(),
            path: RelPath::empty().into(),
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
            editor.set_minimap_visibility(MinimapVisibility::Disabled, window, cx);
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
                            .child(
                                h_flex()
                                    .justify_between()
                                    .child(Label::new("Rust Style").size(LabelSize::Large))
                                    .child(
                                        IconButton::new("reset-style", IconName::Eraser)
                                            .tooltip(Tooltip::text("Reset style"))
                                            .on_click(cx.listener(|this, _, _window, cx| {
                                                this.reset_style(cx);
                                            })),
                                    ),
                            )
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

fn guess_rust_code_from_style(goal_style: &StyleRefinement) -> (String, StyleRefinement) {
    let mut subset_methods = Vec::new();
    for (style, method) in STYLE_METHODS.iter() {
        if goal_style.is_superset_of(style) {
            subset_methods.push(method);
        }
    }

    let mut code = "fn build() -> Div {\n    div()".to_string();
    let mut style = StyleRefinement::default();
    for method in subset_methods {
        let before_change = style.clone();
        style = method.invoke(style);
        if before_change != style {
            let _ = write!(code, "\n        .{}()", &method.name);
        }
    }
    code.push_str("\n}");

    (code, style)
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
    ) -> Task<Result<Vec<CompletionResponse>>> {
        let Some(replace_range) = completion_replace_range(&buffer.read(cx).snapshot(), &position)
        else {
            return Task::ready(Ok(Vec::new()));
        };

        self.div_inspector.update(cx, |div_inspector, _cx| {
            div_inspector.rust_completion_replace_range = Some(replace_range.clone());
        });

        Task::ready(Ok(vec![CompletionResponse {
            completions: STYLE_METHODS
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
            display_options: CompletionDisplayOptions::default(),
            is_incomplete: false,
        }]))
    }

    fn is_completion_trigger(
        &self,
        buffer: &Entity<language::Buffer>,
        position: language::Anchor,
        _text: &str,
        _trigger_in_words: bool,
        _menu_is_open: bool,
        cx: &mut Context<Editor>,
    ) -> bool {
        completion_replace_range(&buffer.read(cx).snapshot(), &position).is_some()
    }

    fn selection_changed(&self, mat: Option<&StringMatch>, _window: &mut Window, cx: &mut App) {
        let div_inspector = self.div_inspector.clone();
        let rust_completion = mat.as_ref().map(|mat| mat.string.clone());
        cx.defer(move |cx| {
            div_inspector.update(cx, |div_inspector, cx| {
                div_inspector.handle_rust_completion_selection_change(rust_completion, cx);
            });
        });
    }

    fn sort_completions(&self) -> bool {
        false
    }
}

fn completion_replace_range(snapshot: &BufferSnapshot, anchor: &Anchor) -> Option<Range<Anchor>> {
    let point = anchor.to_point(snapshot);
    let offset = point.to_offset(snapshot);
    let line_start = Point::new(point.row, 0).to_offset(snapshot);
    let line_end = Point::new(point.row, snapshot.line_len(point.row)).to_offset(snapshot);
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
        let replace_end = snapshot.anchor_after(line_start + end_in_line);
        Some(replace_start..replace_end)
    } else {
        None
    }
}
