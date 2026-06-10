use command_palette::humanize_action_name;
use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, FontWeight, KeyBinding,
    KeyContext, Keystroke, Subscription, WeakEntity, Window,
};
use std::collections::HashSet;
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{prelude::*, text_for_keystrokes, DynamicSpacing, LabelSize};
use workspace::{ModalView, Workspace};

pub struct DescribeKeyModal {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    state: DescribeKeyState,
    intercept_subscription: Option<Subscription>,
    _focus_out_subscription: Subscription,
}

enum DescribeKeyState {
    WaitingForKey,
    CollectingChord(CollectingChordState),
    ShowingResults(DescribeKeyResults),
}

struct CollectingChordState {
    keystrokes: Vec<Keystroke>,
    context_stack: Vec<KeyContext>,
    direct_binding: Option<BindingInfo>,
    pending_bindings: Vec<(SharedString, SharedString)>,
}

struct DescribeKeyResults {
    keystroke_label: SharedString,
    active_binding: Option<BindingInfo>,
    shadowed_bindings: Vec<BindingInfo>,
    pending_bindings: Vec<(SharedString, SharedString)>,
}

struct BindingInfo {
    action_name: SharedString,
    context_predicate: Option<SharedString>,
    documentation: Option<&'static str>,
}

impl BindingInfo {
    fn from_binding(binding: &KeyBinding, cx: &App) -> Self {
        let action_name = humanize_action_name(binding.action().name()).into();
        let context_predicate = binding.predicate().map(|p| SharedString::from(p.to_string()));
        let documentation = cx
            .action_documentation()
            .get(binding.action().name())
            .copied();

        Self {
            action_name,
            context_predicate,
            documentation,
        }
    }
}

impl DescribeKeyModal {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let focus_handle = window.focused(cx).unwrap_or(cx.focus_handle());

        let handle = cx.weak_entity();
        let focus_out_subscription = window.on_focus_out(&focus_handle, cx, move |_, _, cx| {
            handle.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
        });

        let mut this = Self {
            workspace,
            focus_handle,
            state: DescribeKeyState::WaitingForKey,
            intercept_subscription: None,
            _focus_out_subscription: focus_out_subscription,
        };
        this.start_intercepting(cx);
        this
    }

    fn start_intercepting(&mut self, cx: &mut Context<Self>) {
        let listener = cx.listener(
            |this, event: &gpui::KeystrokeEvent, window, cx| {
                let keystroke = &event.keystroke;

                match &this.state {
                    DescribeKeyState::WaitingForKey => {
                        if keystroke.key == "escape" && keystroke.modifiers == gpui::Modifiers::default() {
                            cx.emit(DismissEvent);
                            return;
                        }

                        cx.stop_propagation();

                        let context_stack = event.context_stack.clone();
                        let continuations =
                            Self::collect_continuations(std::slice::from_ref(keystroke), &this.focus_handle, window, cx);

                        if continuations.is_empty() {
                            this.intercept_subscription.take();
                            let results = Self::resolve_bindings(
                                std::slice::from_ref(keystroke),
                                &context_stack,
                                &this.focus_handle,
                                window,
                                cx,
                            );
                            this.state = DescribeKeyState::ShowingResults(results);
                            this.start_dismiss_interception(cx);
                        } else {
                            let direct_binding = Self::find_direct_binding(
                                std::slice::from_ref(keystroke),
                                &context_stack,
                                cx,
                            );
                            this.state =
                                DescribeKeyState::CollectingChord(CollectingChordState {
                                    keystrokes: vec![keystroke.clone()],
                                    context_stack,
                                    direct_binding,
                                    pending_bindings: continuations,
                                });
                        }
                        cx.notify();
                    }
                    DescribeKeyState::CollectingChord(chord) => {
                        cx.stop_propagation();

                        let mut keystrokes = chord.keystrokes.clone();
                        let context_stack = chord.context_stack.clone();

                        if keystroke.key == "escape" && keystroke.modifiers == gpui::Modifiers::default() {
                            this.intercept_subscription.take();
                            let results = Self::resolve_bindings(
                                &keystrokes,
                                &context_stack,
                                &this.focus_handle,
                                window,
                                cx,
                            );
                            this.state = DescribeKeyState::ShowingResults(results);
                            this.start_dismiss_interception(cx);
                            cx.notify();
                            return;
                        }

                        keystrokes.push(keystroke.clone());

                        let continuations =
                            Self::collect_continuations(&keystrokes, &this.focus_handle, window, cx);

                        if continuations.is_empty() {
                            this.intercept_subscription.take();
                            let results = Self::resolve_bindings(
                                &keystrokes,
                                &context_stack,
                                &this.focus_handle,
                                window,
                                cx,
                            );
                            this.state = DescribeKeyState::ShowingResults(results);
                            this.start_dismiss_interception(cx);
                        } else {
                            let direct_binding =
                                Self::find_direct_binding(&keystrokes, &context_stack, cx);
                            this.state =
                                DescribeKeyState::CollectingChord(CollectingChordState {
                                    keystrokes,
                                    context_stack,
                                    direct_binding,
                                    pending_bindings: continuations,
                                });
                        }
                        cx.notify();
                    }
                    DescribeKeyState::ShowingResults(_) => {}
                }
            },
        );
        self.intercept_subscription = Some(cx.intercept_keystrokes(listener));
    }

    fn start_dismiss_interception(&mut self, cx: &mut Context<Self>) {
        let listener = cx.listener(|_this, _event: &gpui::KeystrokeEvent, _window, cx| {
            cx.stop_propagation();
            cx.emit(DismissEvent);
        });
        self.intercept_subscription = Some(cx.intercept_keystrokes(listener));
    }

    fn collect_continuations(
        keystrokes: &[Keystroke],
        focus_handle: &gpui::FocusHandle,
        window: &mut Window,
        cx: &App,
    ) -> Vec<(SharedString, SharedString)> {
        let mut seen = HashSet::new();
        window
            .possible_bindings_for_input_in(keystrokes, focus_handle)
            .iter()
            .filter(|b| b.keystrokes().len() > keystrokes.len())
            .map(|binding| {
                let remaining_keystrokes: Vec<Keystroke> = binding
                    .keystrokes()
                    .iter()
                    .skip(keystrokes.len())
                    .map(|k| k.inner().to_owned())
                    .collect();
                let keystrokes_label: SharedString =
                    format!("... {}", text_for_keystrokes(&remaining_keystrokes, cx)).into();
                let action_name: SharedString =
                    humanize_action_name(binding.action().name()).into();
                (keystrokes_label, action_name)
            })
            .filter(|entry| seen.insert(entry.clone()))
            .collect()
    }

    fn find_direct_binding(
        keystrokes: &[Keystroke],
        context_stack: &[KeyContext],
        cx: &App,
    ) -> Option<BindingInfo> {
        cx.all_bindings_for_input(keystrokes)
            .iter()
            .find(|binding| {
                binding
                    .predicate()
                    .map_or(true, |predicate| predicate.depth_of(context_stack).is_some())
            })
            .map(|binding| BindingInfo::from_binding(binding, cx))
    }

    fn resolve_bindings(
        keystrokes: &[Keystroke],
        context_stack: &[KeyContext],
        focus_handle: &gpui::FocusHandle,
        window: &mut Window,
        cx: &mut App,
    ) -> DescribeKeyResults {
        let all_bindings = cx.all_bindings_for_input(keystrokes);

        let mut active_binding = None;
        let mut shadowed_bindings = Vec::new();

        let mut seen_actions = HashSet::new();
        for binding in &all_bindings {
            let matches_context = binding
                .predicate()
                .map_or(true, |predicate| predicate.depth_of(context_stack).is_some());

            if !matches_context {
                continue;
            }

            let action_key = (
                SharedString::from(binding.action().name()),
                binding.predicate().map(|p| p.to_string()),
            );
            if !seen_actions.insert(action_key) {
                continue;
            }

            if active_binding.is_none() {
                active_binding = Some(BindingInfo::from_binding(binding, cx));
            } else {
                shadowed_bindings.push(BindingInfo::from_binding(binding, cx));
            }
        }

        let pending_bindings =
            Self::collect_continuations(keystrokes, focus_handle, window, cx);

        let keystroke_label: SharedString = text_for_keystrokes(keystrokes, cx).into();

        DescribeKeyResults {
            keystroke_label,
            active_binding,
            shadowed_bindings,
            pending_bindings,
        }
    }

    fn render_waiting(&self, _cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .gap(px(4.))
            .child(
                Label::new("Press any key to describe...")
                    .size(LabelSize::Default)
                    .weight(FontWeight::MEDIUM)
                    .color(Color::Accent),
            )
            .child(
                Label::new("Escape to cancel")
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            )
    }

    fn render_collecting(
        &self,
        chord_state: &CollectingChordState,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let keys_label = text_for_keystrokes(&chord_state.keystrokes, cx);

        let mut content = v_flex().gap(px(8.));

        content = content.child(
            Label::new(format!("{} ...", keys_label))
                .size(LabelSize::Large)
                .weight(FontWeight::BOLD)
                .color(Color::Accent),
        );

        if let Some(direct) = &chord_state.direct_binding {
            content = content.child(
                h_flex()
                    .gap(px(8.))
                    .child(
                        Label::new("Alone:")
                            .size(LabelSize::XSmall)
                            .weight(FontWeight::MEDIUM)
                            .color(Color::Muted),
                    )
                    .child(
                        Label::new(direct.action_name.clone())
                            .size(LabelSize::Small)
                            .color(Color::Default),
                    ),
            );
        }

        content = content.child(
            Label::new("Waiting for next key...")
                .size(LabelSize::XSmall)
                .color(Color::Muted),
        );

        if !chord_state.pending_bindings.is_empty() {
            content = content
                .child(
                    Label::new("Possible continuations:")
                        .size(LabelSize::XSmall)
                        .weight(FontWeight::MEDIUM)
                        .color(Color::Muted),
                )
                .child(
                    v_flex().gap(px(2.)).children(
                        chord_state.pending_bindings.iter().map(|(keys, action)| {
                            h_flex()
                                .gap(px(8.))
                                .child(
                                    Label::new(keys.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Accent),
                                )
                                .child(
                                    Label::new(action.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Default),
                                )
                        }),
                    ),
                );
        }

        content
    }

    fn render_results(
        &self,
        results: &DescribeKeyResults,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let mut content = v_flex().gap(px(8.));

        content = content.child(
            Label::new(results.keystroke_label.clone())
                .size(LabelSize::Large)
                .weight(FontWeight::BOLD)
                .color(Color::Accent),
        );

        if let Some(active) = &results.active_binding {
            content = content.child(Self::render_binding_info(active, false));
        } else if results.pending_bindings.is_empty() {
            content = content.child(
                Label::new("No binding found")
                    .size(LabelSize::Default)
                    .color(Color::Muted),
            );
        }

        if !results.shadowed_bindings.is_empty() {
            content = content
                .child(
                    Label::new("Shadowed bindings:")
                        .size(LabelSize::XSmall)
                        .weight(FontWeight::MEDIUM)
                        .color(Color::Muted),
                )
                .children(
                    results
                        .shadowed_bindings
                        .iter()
                        .map(|info| Self::render_binding_info(info, true)),
                );
        }

        if !results.pending_bindings.is_empty() {
            content = content
                .child(
                    Label::new("Multi-key continuations:")
                        .size(LabelSize::XSmall)
                        .weight(FontWeight::MEDIUM)
                        .color(Color::Muted),
                )
                .child(
                    v_flex().gap(px(2.)).children(
                        results.pending_bindings.iter().map(|(keys, action)| {
                            h_flex()
                                .gap(px(8.))
                                .child(
                                    Label::new(keys.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Accent),
                                )
                                .child(
                                    Label::new(action.clone())
                                        .size(LabelSize::Small)
                                        .color(Color::Default),
                                )
                        }),
                    ),
                );
        }

        content
    }

    fn render_binding_info(info: &BindingInfo, dimmed: bool) -> impl IntoElement {
        let name_color = if dimmed {
            Color::Muted
        } else {
            Color::Default
        };

        let mut row = v_flex().gap(px(2.)).child(
            Label::new(info.action_name.clone())
                .size(LabelSize::Default)
                .weight(FontWeight::MEDIUM)
                .color(name_color),
        );

        if let Some(context) = &info.context_predicate {
            row = row.child(
                Label::new(format!("context: {}", context))
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            );
        }

        if let Some(docs) = info.documentation {
            row = row.child(
                Label::new(docs.to_string())
                    .size(LabelSize::XSmall)
                    .color(Color::Muted),
            );
        }

        row
    }
}

impl Render for DescribeKeyModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let viewport_size = window.viewport_size();
        let max_panel_width = px((f32::from(viewport_size.width) * 0.5).min(480.0));

        let status_height = self
            .workspace
            .upgrade()
            .and_then(|workspace| {
                workspace.read_with(cx, |workspace, cx| {
                    if workspace.status_bar_visible(cx) {
                        Some(
                            DynamicSpacing::Base04.px(cx) * 2.0
                                + ThemeSettings::get_global(cx).ui_font_size(cx),
                        )
                    } else {
                        None
                    }
                })
            })
            .unwrap_or(px(0.));

        let margin_bottom = px(16.);
        let bottom_offset = margin_bottom + status_height;

        let body = match &self.state {
            DescribeKeyState::WaitingForKey => self.render_waiting(cx).into_any_element(),
            DescribeKeyState::CollectingChord(chord_state) => {
                self.render_collecting(chord_state, cx).into_any_element()
            }
            DescribeKeyState::ShowingResults(results) => {
                self.render_results(results, cx).into_any_element()
            }
        };

        div()
            .id("describe-key-modal")
            .occlude()
            .absolute()
            .bottom(bottom_offset)
            .right(px(16.))
            .min_w(px(220.))
            .max_w(max_panel_width)
            .elevation_3(cx)
            .px(px(12.))
            .py(px(8.))
            .child(body)
    }
}

impl EventEmitter<DismissEvent> for DescribeKeyModal {}

impl Focusable for DescribeKeyModal {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for DescribeKeyModal {
    fn render_bare(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Entity, TestAppContext, VisualTestContext, actions};
    use project::Project;
    use settings::KeymapFile;
    use workspace::{AppState, MultiWorkspace, Workspace};

    actions!(
        test_describe_key,
        [
            TestSingleKeyAction,
            TestChordAction,
            TestChordAction2,
            TestThreeKeyAction,
        ]
    );

    fn init_test(cx: &mut TestAppContext) -> std::sync::Arc<AppState> {
        cx.update(|cx| {
            let app_state = AppState::test(cx);
            theme_settings::init(theme::LoadThemes::JustBase, cx);
            workspace::init(app_state.clone(), cx);
            crate::init(cx);
            app_state
        })
    }

    fn open_describe_key(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<DescribeKeyModal> {
        workspace.update_in(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            workspace.toggle_modal(window, cx, |window, cx| {
                DescribeKeyModal::new(workspace_handle, window, cx)
            });
        });
        cx.run_until_parked();
        workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_modal::<DescribeKeyModal>(cx)
            })
            .expect("DescribeKeyModal should be open")
    }

    #[gpui::test]
    async fn test_single_key_binding_shows_results_immediately(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-c": "test_describe_key::TestSingleKeyAction" } }]"#,
                cx,
            ));
        });

        let modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("ctrl-c");
        cx.run_until_parked();

        modal.read_with(cx, |modal, _| {
            match &modal.state {
                DescribeKeyState::ShowingResults(results) => {
                    assert!(
                        results.active_binding.is_some(),
                        "should have an active binding"
                    );
                    assert_eq!(
                        results.active_binding.as_ref().map(|b| b.action_name.as_ref()),
                        Some("test describe key: test single key action"),
                    );
                }
                other => panic!("expected ShowingResults, got {:?}", state_name(other)),
            }
        });
    }

    #[gpui::test]
    async fn test_multi_key_chord_enters_collecting_state(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d": "test_describe_key::TestChordAction" } }]"#,
                cx,
            ));
        });

        let modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        modal.read_with(cx, |modal, _| {
            match &modal.state {
                DescribeKeyState::CollectingChord(chord) => {
                    assert_eq!(chord.keystrokes.len(), 1);
                    assert!(!chord.pending_bindings.is_empty(), "should list continuations");
                }
                other => panic!("expected CollectingChord, got {:?}", state_name(other)),
            }
        });
    }

    #[gpui::test]
    async fn test_multi_key_chord_resolves_after_full_sequence(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d": "test_describe_key::TestChordAction" } }]"#,
                cx,
            ));
        });

        let modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        cx.simulate_keystrokes("ctrl-d");
        cx.run_until_parked();

        modal.read_with(cx, |modal, _| {
            match &modal.state {
                DescribeKeyState::ShowingResults(results) => {
                    assert!(
                        results.active_binding.is_some(),
                        "should have resolved the chord to an action"
                    );
                    assert_eq!(
                        results.active_binding.as_ref().map(|b| b.action_name.as_ref()),
                        Some("test describe key: test chord action"),
                    );
                }
                other => panic!("expected ShowingResults, got {:?}", state_name(other)),
            }
        });
    }

    #[gpui::test]
    async fn test_escape_during_collecting_shows_results(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d": "test_describe_key::TestChordAction" } }]"#,
                cx,
            ));
        });

        let modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        cx.simulate_keystrokes("escape");
        cx.run_until_parked();

        modal.read_with(cx, |modal, _| {
            match &modal.state {
                DescribeKeyState::ShowingResults(results) => {
                    assert!(
                        results.active_binding.is_none(),
                        "ctrl-k alone should have no binding"
                    );
                }
                other => panic!("expected ShowingResults, got {:?}", state_name(other)),
            }
        });
    }

    #[gpui::test]
    async fn test_escape_while_waiting_dismisses_modal(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let _modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("escape");
        cx.run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            assert!(
                workspace.active_modal::<DescribeKeyModal>(cx).is_none(),
                "modal should be dismissed after escape"
            );
        });
    }

    #[gpui::test]
    async fn test_three_key_chord_accumulates(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d ctrl-a": "test_describe_key::TestThreeKeyAction" } }]"#,
                cx,
            ));
        });

        let modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();
        modal.read_with(cx, |modal, _| {
            assert!(matches!(modal.state, DescribeKeyState::CollectingChord(_)));
        });

        cx.simulate_keystrokes("ctrl-d");
        cx.run_until_parked();
        modal.read_with(cx, |modal, _| {
            match &modal.state {
                DescribeKeyState::CollectingChord(chord) => {
                    assert_eq!(chord.keystrokes.len(), 2);
                }
                other => panic!("expected CollectingChord with 2 keys, got {:?}", state_name(other)),
            }
        });

        cx.simulate_keystrokes("ctrl-a");
        cx.run_until_parked();
        modal.read_with(cx, |modal, _| {
            match &modal.state {
                DescribeKeyState::ShowingResults(results) => {
                    assert_eq!(
                        results.active_binding.as_ref().map(|b| b.action_name.as_ref()),
                        Some("test describe key: test three key action"),
                    );
                }
                other => panic!("expected ShowingResults, got {:?}", state_name(other)),
            }
        });
    }

    #[gpui::test]
    async fn test_dismiss_after_results(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-c": "test_describe_key::TestSingleKeyAction" } }]"#,
                cx,
            ));
        });

        let _modal = open_describe_key(&workspace, cx);

        cx.simulate_keystrokes("ctrl-c");
        cx.run_until_parked();

        // Any key should dismiss after showing results
        cx.simulate_keystrokes("a");
        cx.run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            assert!(
                workspace.active_modal::<DescribeKeyModal>(cx).is_none(),
                "modal should be dismissed after pressing a key while showing results"
            );
        });
    }

    fn state_name(state: &DescribeKeyState) -> &'static str {
        match state {
            DescribeKeyState::WaitingForKey => "WaitingForKey",
            DescribeKeyState::CollectingChord(_) => "CollectingChord",
            DescribeKeyState::ShowingResults(_) => "ShowingResults",
        }
    }
}
