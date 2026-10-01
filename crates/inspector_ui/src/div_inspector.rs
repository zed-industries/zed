use anyhow::{Result, anyhow};
use editor::{
    Bias, CompletionProvider, Editor, EditorEvent, EditorMode, MinimapVisibility, MultiBuffer,
};
use fuzzy::StringMatch;
use gpui::{
    AsyncWindowContext, DivInspectorState, Entity, InspectorElementId, IntoElement,
    StyleRefinement, Subscription, Task, WeakEntity, Window,
    inspector_reflection::FunctionReflection, styled_reflection,
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
    _initialization_task: Task<()>,
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
        _subscriptions: [Subscription; 2],
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
        let initialization_task = cx.spawn_in(window, {
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
                let rust_style_buffer = rust_language_result.map(|rust_language| {
                    cx.new(|cx| Buffer::local("", cx).with_language_async(rust_language, cx))
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
                                let inspector_state = window
                                    .with_inspector_state(Some(&id), cx, |state, _window| {
                                        state.clone()
                                    })
                                    .flatten();
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
        });

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
            _initialization_task: initialization_task,
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
            let div_inspector = cx.weak_entity();
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

        let json_subscription = cx.subscribe_in(&json_style_editor, window, {
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
        });

        let rust_subscription = cx.subscribe(&rust_style_editor, {
            let json_style_buffer = json_style_buffer.clone();
            let rust_style_buffer = rust_style_buffer.clone();
            move |this, _editor, event: &EditorEvent, cx| {
                if let EditorEvent::BufferEdited = event {
                    this.update_json_style_from_rust(&json_style_buffer, &rust_style_buffer, cx);
                }
            }
        });

        self.unconvertible_style = style.subtract(&rust_style);
        self.json_style_overrides = StyleRefinement::default();
        self.state = State::Ready {
            rust_style_buffer,
            rust_style_editor,
            json_style_buffer,
            json_style_editor,
            _subscriptions: [json_subscription, rust_subscription],
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
            let mut method_names = split_str_with_ranges(&before_text, &is_not_identifier_char)
                .into_iter()
                .map(|(range, name)| (Some(range), name.to_string()))
                .collect::<Vec<_>>();
            method_names.push((None, completion.clone()));
            method_names.extend(
                split_str_with_ranges(&after_text, &is_not_identifier_char)
                    .into_iter()
                    .map(|(range, name)| (Some(range), name.to_string())),
            );
            method_names
        } else {
            split_str_with_ranges(&snapshot.text(), &is_not_identifier_char)
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
            .map(|(ix, range)| {
                DiagnosticEntry::new(
                    range,
                    Diagnostic {
                        message: "unrecognized".into(),
                        severity: DiagnosticSeverity::WARNING,
                        is_primary: true,
                        group_id: ix,
                        ..Default::default()
                    },
                )
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
            .update(cx, |project, cx| project.create_worktree(path, false, cx))
            .await?;

        let project_path = worktree.read_with(cx, |worktree, _cx| ProjectPath {
            worktree_id: worktree.id(),
            path: RelPath::empty_arc(),
        });

        let buffer = project
            .update(cx, |project, cx| project.open_path(project_path, cx))
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
            editor.set_show_bookmarks(false, cx);
            editor.set_show_breakpoints(false, cx);
            editor.set_show_git_diff_gutter(false, cx);
            editor.set_show_runnables(false, cx);
            editor.disable_mouse_wheel_zoom();
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
                .child(format!(
                    "Bounds: ⌜{} - {}⌟",
                    inspector_state.bounds.origin,
                    inspector_state.bounds.bottom_right()
                ))
                .child(format!("Size: {}", inspector_state.bounds.size)),
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
            let _ = write!(code, "\n        .{}()", method.name);
        }
    }
    code.push_str("\n}");

    (code, style)
}

fn is_not_identifier_char(c: char) -> bool {
    !c.is_alphanumeric() && c != '_'
}

struct RustStyleCompletionProvider {
    div_inspector: WeakEntity<DivInspector>,
}

impl CompletionProvider for RustStyleCompletionProvider {
    fn completions(
        &self,
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

        if self
            .div_inspector
            .update(cx, |div_inspector, _cx| {
                div_inspector.rust_completion_replace_range = Some(replace_range.clone());
            })
            .is_err()
        {
            return Task::ready(Ok(Vec::new()));
        }

        Task::ready(Ok(vec![CompletionResponse {
            completions: STYLE_METHODS
                .iter()
                .map(|(_, method)| Completion {
                    replace_range: replace_range.clone(),
                    new_text: format!(".{}()", method.name),
                    label: CodeLabel::plain(method.name.to_string(), None),
                    match_start: None,
                    snippet_deduplication_key: None,
                    icon_path: None,
                    icon_color: None,
                    documentation: method.documentation.map(|documentation| {
                        CompletionDocumentation::MultiLineMarkdown(documentation.into())
                    }),
                    source: CompletionSource::Custom,
                    insert_text_mode: None,
                    confirm: None,
                    group: None,
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
        cx: &mut Context<Editor>,
    ) -> bool {
        completion_replace_range(&buffer.read(cx).snapshot(), &position).is_some()
    }

    fn selection_changed(&self, mat: Option<&StringMatch>, _window: &mut Window, cx: &mut App) {
        let div_inspector = self.div_inspector.clone();
        let rust_completion = mat.as_ref().map(|mat| mat.string.clone());
        cx.defer(move |cx| {
            div_inspector
                .update(cx, |div_inspector, cx| {
                    div_inspector.handle_rust_completion_selection_change(rust_completion, cx);
                })
                .ok();
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

#[cfg(test)]
mod tests {
    use super::*;
    use fs::FakeFs;
    use futures::{FutureExt as _, channel::oneshot};
    use gpui::{
        AnyWeakEntity, Focusable as _, Modifiers, TestAppContext, VisualTestContext, point,
    };
    use language::{LanguageConfig, LanguageName, LanguageQueries, LoadedLanguage};
    use settings::SettingsStore;
    use snippet_provider::SnippetProvider;
    use std::{
        cell::RefCell,
        collections::BTreeSet,
        path::PathBuf,
        sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        },
    };

    #[gpui::test]
    async fn test_drop_during_initialization(cx: &mut TestAppContext) {
        let (fs, watcher_baseline) = test_fs(cx);
        let project = Project::test(fs.clone(), [], cx).await;
        let weak_project = project.downgrade();
        let weak_lsp_store = project.read_with(cx, |project, _| project.lsp_store().downgrade());
        let (finish_loading, loading_gate) = oneshot::channel::<()>();
        let loading_gate = loading_gate.shared();
        let loading_started = Arc::new(AtomicBool::new(false));
        project.read_with(cx, |project, _| {
            project.languages().register_language(
                LanguageName::from("Rust"),
                None,
                Arc::default(),
                false,
                None,
                Arc::new({
                    let loading_started = loading_started.clone();
                    move || {
                        let loading_gate = loading_gate.clone();
                        let loading_started = loading_started.clone();
                        Box::pin(async move {
                            loading_started.store(true, Ordering::SeqCst);
                            loading_gate.await?;
                            Ok(LoadedLanguage {
                                config: LanguageConfig {
                                    name: LanguageName::from("Rust"),
                                    ..LanguageConfig::default()
                                },
                                queries: LanguageQueries::default(),
                                context_provider: None,
                                toolchain_provider: None,
                                manifest_name: None,
                            })
                        })
                    }
                }),
            );
        });

        let cx = cx.add_empty_window();
        let div_inspector =
            cx.new_window_entity(|window, cx| DivInspector::new(project, window, cx));
        let weak_inspector = div_inspector.downgrade();
        cx.run_until_parked();

        assert!(loading_started.load(Ordering::SeqCst));
        assert_inspector_watchers(&fs, &watcher_baseline);
        let weak_json_buffer = div_inspector.read_with(cx, |div_inspector, cx| {
            assert!(matches!(div_inspector.state, State::Loading));
            div_inspector
                .project
                .read(cx)
                .buffer_store()
                .read(cx)
                .buffers()
                .next()
                .expect("initialization must load the JSON buffer before Rust")
                .downgrade()
        });

        cx.update(|_, _| drop(div_inspector));
        flush_teardown(cx);

        assert!(weak_inspector.upgrade().is_none());
        assert!(weak_project.upgrade().is_none());
        assert!(weak_lsp_store.upgrade().is_none());
        assert!(weak_json_buffer.upgrade().is_none());
        assert_eq!(watched_paths(&fs), watcher_baseline);

        finish_loading
            .send(())
            .expect("language loader must still be waiting");
        cx.run_until_parked();
        assert!(weak_inspector.upgrade().is_none());
        assert!(weak_project.upgrade().is_none());
        assert!(weak_lsp_store.upgrade().is_none());
        assert_eq!(watched_paths(&fs), watcher_baseline);
    }

    #[gpui::test]
    async fn test_close_focused_inspector_and_reopen_same_element(cx: &mut TestAppContext) {
        let (fs, watcher_baseline) = test_fs(cx);
        let pending_project = Rc::new(RefCell::new(None::<Entity<Project>>));
        let inspectors = Rc::new(RefCell::new(Vec::new()));
        cx.update({
            let pending_project = pending_project.clone();
            let inspectors = inspectors.clone();
            move |cx| {
                cx.set_inspector_renderer(Box::new(|inspector, window, cx| {
                    v_flex()
                        .size_full()
                        .children(inspector.render_inspector_states(window, cx))
                        .into_any_element()
                }));
                cx.register_inspector_element(move |window, cx| {
                    let project = pending_project.borrow_mut().take().expect("fresh project");
                    let inspector = cx.new(|cx| DivInspector::new(project, window, cx));
                    inspectors.borrow_mut().push(inspector.downgrade());
                    move |id, state: &DivInspectorState, window: &mut Window, cx: &mut App| {
                        inspector.update(cx, |inspector, cx| {
                            inspector.update_inspected_element(&id, state.clone(), window, cx);
                            inspector.render(window, cx).into_any_element()
                        })
                    }
                });
            }
        });
        let first_window = cx.add_window(|_, _| InspectorTarget);
        let second_window = cx.add_window(|_, _| InspectorTarget);
        let mut second = VisualTestContext::from_window(second_window.into(), cx);
        let mut selected_id = None;
        let mut resources = Vec::new();
        let mut surviving_resources = Vec::new();
        let mut surviving_editor = None;
        let mut active_baseline = watcher_baseline.clone();
        let windows = [second_window, first_window, first_window];
        for (generation, window) in windows.into_iter().enumerate() {
            let cx = &mut VisualTestContext::from_window(window.into(), cx);
            let project = Project::test(fs.clone(), [], cx).await;
            project.read_with(cx, |project, _| {
                project.languages().register_test_language(LanguageConfig {
                    name: LanguageName::from("Rust"),
                    ..LanguageConfig::default()
                });
            });
            *pending_project.borrow_mut() = Some(project);
            cx.update(|window, cx| {
                window.toggle_inspector(cx);
                window.draw(cx).clear(cx);
            });
            let position = point(px(5.), px(5.));
            cx.simulate_mouse_move(position, None, Modifiers::default());
            cx.simulate_click(position, Modifiers::default());
            cx.update(|window, cx| window.draw(cx).clear(cx));
            cx.run_until_parked();
            assert_eq!(inspectors.borrow().len(), generation + 1);
            let weak_inspector = inspectors
                .borrow()
                .last()
                .expect("rendered inspector")
                .clone();
            let (weak_entities, rust_editor, focus_handle) = weak_inspector
                .read_with(cx, |inspector, cx| {
                    let id = inspector.inspector_id.clone().expect("picked element");
                    if generation > 0
                        && let Some(previous_id) = selected_id.replace(id.clone())
                    {
                        assert_eq!(id, previous_id);
                    }
                    let State::Ready {
                        rust_style_buffer,
                        rust_style_editor,
                        json_style_buffer,
                        json_style_editor,
                        ..
                    } = &inspector.state
                    else {
                        panic!("picked element must have initialized editors");
                    };
                    let editor = if generation == 2 {
                        json_style_editor
                    } else {
                        rust_style_editor
                    };
                    (
                        [
                            AnyWeakEntity::from(weak_inspector.clone()),
                            AnyWeakEntity::from(inspector.project.downgrade()),
                            AnyWeakEntity::from(inspector.project.read(cx).lsp_store().downgrade()),
                            AnyWeakEntity::from(rust_style_editor.downgrade()),
                            AnyWeakEntity::from(json_style_editor.downgrade()),
                            AnyWeakEntity::from(rust_style_buffer.downgrade()),
                            AnyWeakEntity::from(json_style_buffer.downgrade()),
                        ],
                        rust_style_editor.downgrade(),
                        editor.focus_handle(cx),
                    )
                })
                .expect("live inspector");
            cx.update(|window, cx| {
                window.focus(&focus_handle, cx);
                window.draw(cx).clear(cx);
                assert!(focus_handle.is_focused(window));
            });
            assert_inspector_watchers(&fs, &active_baseline);
            resources.extend(weak_entities);
            if generation == 0 {
                surviving_editor = Some(rust_editor);
                surviving_resources = resources.clone();
                active_baseline = watched_paths(&fs);
                continue;
            }
            cx.update(|window, cx| {
                if generation == 1 {
                    window.toggle_inspector(cx);
                    window.draw(cx).clear(cx);
                } else {
                    window.remove_window();
                }
            });
            flush_teardown(&mut second);
            second.update(|window, cx| window.draw(cx).clear(cx));
            let editor = surviving_editor.as_ref().expect("second window editor");
            let text = |cx: &TestAppContext| {
                editor
                    .read_with(cx, |editor, cx| editor.text(cx))
                    .expect("live editor")
            };
            let before = text(&second);
            second.simulate_input(" ");
            assert_eq!(text(&second).len(), before.len() + 1);
            resources.retain(AnyWeakEntity::is_upgradable);
            assert_eq!(resources, surviving_resources);
            assert_eq!(watched_paths(&fs), active_baseline);
        }
        second.update(|window, cx| {
            window.toggle_inspector(cx);
            window.draw(cx).clear(cx);
        });
        flush_teardown(&mut second);
        assert!(resources.iter().all(|entity| !entity.is_upgradable()));
        assert_eq!(watched_paths(&fs), watcher_baseline);
    }

    fn test_fs(cx: &mut TestAppContext) -> (Arc<FakeFs>, Vec<PathBuf>) {
        cx.update(|cx| {
            let settings = SettingsStore::test(cx);
            cx.set_global(settings);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            editor::init(cx);
        });
        let fs = FakeFs::new(cx.executor());
        cx.update(|cx| {
            drop(SnippetProvider::new(fs.clone(), BTreeSet::new(), cx));
        });
        cx.run_until_parked();
        let watcher_baseline = watched_paths(&fs);
        assert!(!watcher_baseline.is_empty());
        (fs, watcher_baseline)
    }

    fn watched_paths(fs: &FakeFs) -> Vec<PathBuf> {
        let mut paths = fs.watched_paths();
        paths.sort();
        paths
    }

    fn assert_inspector_watchers(fs: &FakeFs, baseline: &[PathBuf]) {
        let mut expected = baseline.to_vec();
        expected.extend([
            paths::tasks_file().clone(),
            paths::debug_scenarios_file().clone(),
            PathBuf::from(ZED_INSPECTOR_STYLE_JSON),
        ]);
        expected.sort();
        assert_eq!(watched_paths(fs), expected);
    }

    fn flush_teardown(cx: &mut VisualTestContext) {
        cx.run_until_parked();
        cx.update(|_, _| {});
        cx.run_until_parked();
    }

    struct InspectorTarget;

    impl Render for InspectorTarget {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div().id("inspected-element").size(px(100.))
        }
    }
}
