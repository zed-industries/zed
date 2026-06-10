//! Modal implementation for the which-key display.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, FontWeight, Keystroke,
    Modifiers, ScrollHandle, Subscription, WeakEntity, Window,
};
use settings::Settings;
use std::collections::{HashMap, HashSet};
use theme_settings::ThemeSettings;
use ui::{
    Divider, DividerColor, DynamicSpacing, LabelSize, WithScrollbar, prelude::*,
    text_for_keystrokes,
};
use workspace::{ModalView, Workspace};

use crate::FILTERED_KEYSTROKES;

pub struct WhichKeyModal {
    _workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    bindings: Vec<(SharedString, SharedString)>,
    pending_keys: SharedString,
    manual: bool,
    showing_continuations: bool,
    _pending_input_subscription: Subscription,
    _keystroke_subscription: Option<Subscription>,
    _focus_out_subscription: Subscription,
}

impl WhichKeyModal {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut this = Self::new_inner(workspace, window, cx);
        this.update_pending_keys(window, cx);
        this
    }

    pub fn new_manual(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Keep focus where it currently is
        let focus_handle = window.focused(cx).unwrap_or(cx.focus_handle());
        let handle = cx.weak_entity();
        let mut this = Self {
            _workspace: workspace,
            focus_handle: focus_handle.clone(),
            scroll_handle: ScrollHandle::new(),
            bindings: Vec::new(),
            pending_keys: SharedString::new_static(""),
            manual: true,
            showing_continuations: false,
            _pending_input_subscription: cx.observe_pending_input(
                window,
                |this: &mut Self, window, cx| {
                    if window.pending_input_keystrokes().is_some() {
                        this.showing_continuations = true;
                        this.update_pending_keys(window, cx);
                    } else if this.showing_continuations {
                        cx.emit(DismissEvent);
                    }
                },
            ),
            _keystroke_subscription: None,
            _focus_out_subscription: window.on_focus_out(&focus_handle, cx, move |_, _, cx| {
                handle.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
            }),
        };
        this.update_all_bindings(window, cx);
        // Defer the keystroke observer so it doesn't fire for the keystroke
        // that triggered ToggleWhichKey.
        cx.defer_in(window, |this, _window, cx| {
            this._keystroke_subscription = Some(cx.observe_keystrokes(
                |this, _event, _window, cx| {
                    if !this.showing_continuations {
                        this.dismiss(cx);
                    }
                },
            ));
        });
        this
    }

    pub fn is_manual(&self) -> bool {
        self.manual
    }

    fn new_inner(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Keep focus where it currently is
        let focus_handle = window.focused(cx).unwrap_or(cx.focus_handle());
        let handle = cx.weak_entity();
        Self {
            _workspace: workspace,
            focus_handle: focus_handle.clone(),
            scroll_handle: ScrollHandle::new(),
            bindings: Vec::new(),
            pending_keys: SharedString::new_static(""),
            manual: false,
            showing_continuations: false,
            _keystroke_subscription: None,
            _pending_input_subscription: cx.observe_pending_input(
                window,
                |this: &mut Self, window, cx| {
                    this.update_pending_keys(window, cx);
                },
            ),
            _focus_out_subscription: window.on_focus_out(&focus_handle, cx, move |_, _, cx| {
                handle.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
            }),
        }
    }

    pub fn dismiss(&self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }

    fn update_pending_keys(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pending_keys) = window.pending_input_keystrokes() else {
            cx.emit(DismissEvent);
            return;
        };
        let bindings =
            Self::collect_filtered_bindings(window, pending_keys, Some(&self.focus_handle), true);
        let binding_data = bindings
            .into_iter()
            .map(|(keystrokes, action_name)| {
                let remaining = keystrokes[pending_keys.len()..].to_vec();
                (remaining, action_name)
            })
            .collect();
        let title = text_for_keystrokes(&pending_keys, cx).into();
        self.sort_and_finalize_bindings(binding_data, title, cx);
    }

    fn update_all_bindings(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let binding_data =
            Self::collect_filtered_bindings(window, &[], Some(&self.focus_handle), false);
        self.sort_and_finalize_bindings(binding_data, "Keybindings".into(), cx);
    }

    fn collect_filtered_bindings(
        window: &Window,
        input: &[Keystroke],
        focus_handle: Option<&gpui::FocusHandle>,
        dedup_by_action: bool,
    ) -> Vec<(Vec<Keystroke>, SharedString)> {
        let mut seen = HashSet::new();
        let bindings = if let Some(focus) = focus_handle {
            window.possible_bindings_for_input_in(input, focus)
        } else {
            window.possible_bindings_for_input(input)
        };
        bindings
            .iter()
            // Map to keystrokes
            .map(|binding| {
                (
                    binding
                        .keystrokes()
                        .iter()
                        .map(|k| k.inner().to_owned())
                        .collect::<Vec<_>>(),
                    binding.action(),
                )
            })
            // Check if this binding matches any filtered keystroke pattern
            .filter(|(keystrokes, _action)| {
                !FILTERED_KEYSTROKES.iter().any(|filtered| {
                    keystrokes.len() >= filtered.len()
                        && keystrokes[..filtered.len()] == filtered[..]
                })
            })
            // Map to remaining keystrokes and action name
            .map(|(keystrokes, action)| {
                let action_name: SharedString =
                    command_palette::humanize_action_name(action.name()).into();
                (keystrokes, action_name)
            })
            .filter(|(_, action_name)| !dedup_by_action || seen.insert(action_name.clone()))
            .collect()
    }

    fn sort_and_finalize_bindings(
        &mut self,
        binding_data: Vec<(Vec<Keystroke>, SharedString)>,
        title: SharedString,
        cx: &mut Context<Self>,
    ) {
        let binding_data = group_bindings(binding_data);

        let mut entries: Vec<_> = binding_data
            .into_iter()
            .map(|(keystrokes, action)| {
                let text: SharedString = text_for_keystrokes(&keystrokes, cx).into();
                (keystrokes.len(), text, action)
            })
            .collect();

        // Sort bindings from shortest to longest, with groups last
        // Using stable sort to preserve relative order of equal elements
        entries.sort_by(|(len_a, text_a, action_a), (len_b, text_b, action_b)| {
            // Groups (actions starting with "+") should go last
            let is_group_a = action_a.starts_with('+');
            let is_group_b = action_b.starts_with('+');

            // First, separate groups from non-groups
            let group_cmp = is_group_a.cmp(&is_group_b);
            if group_cmp != std::cmp::Ordering::Equal {
                return group_cmp;
            }

            // Then sort by keystroke count
            let keystroke_cmp = len_a.cmp(len_b);
            if keystroke_cmp != std::cmp::Ordering::Equal {
                return keystroke_cmp;
            }

            // Finally sort by text length, then lexicographically for full stability
            let text_len_cmp = text_a.len().cmp(&text_b.len());
            if text_len_cmp != std::cmp::Ordering::Equal {
                return text_len_cmp;
            }
            let text_cmp = text_a.cmp(text_b);
            if text_cmp != std::cmp::Ordering::Equal {
                return text_cmp;
            }
            action_a.cmp(action_b)
        });
        entries.dedup();

        self.pending_keys = title;
        let mut seen = HashSet::new();
        self.bindings = entries
            .into_iter()
            .map(|(_len, text, action)| (text, action))
            .filter(|entry| seen.insert(entry.clone()))
            .collect();
    }
}

impl Render for WhichKeyModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let has_rows = !self.bindings.is_empty();
        let viewport_size = window.viewport_size();

        let max_panel_width = px((f32::from(viewport_size.width) * 0.5).min(480.0));
        let max_content_height = px(f32::from(viewport_size.height) * 0.4);

        // Push above status bar when visible
        let status_height = self
            ._workspace
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

        // Title section
        let title_section = {
            let mut column = v_flex().gap(px(0.)).child(
                div()
                    .child(
                        Label::new(self.pending_keys.clone())
                            .size(LabelSize::Default)
                            .weight(FontWeight::MEDIUM)
                            .color(Color::Accent),
                    )
                    .mb(px(2.)),
            );

            if has_rows {
                column = column.child(
                    div()
                        .child(Divider::horizontal().color(DividerColor::BorderFaded))
                        .mb(px(2.)),
                );
            }

            column
        };

        let content = h_flex()
            .items_start()
            .id("which-key-content")
            .gap(px(8.))
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .h_full()
            .max_h(max_content_height)
            .child(
                // Keystrokes column
                v_flex()
                    .gap(px(4.))
                    .flex_shrink_0()
                    .children(self.bindings.iter().map(|(keystrokes, _)| {
                        div()
                            .child(
                                Label::new(keystrokes.clone())
                                    .size(LabelSize::Default)
                                    .color(Color::Accent),
                            )
                            .text_align(gpui::TextAlign::Right)
                    })),
            )
            .child(
                // Actions column
                v_flex()
                    .gap(px(4.))
                    .flex_1()
                    .min_w_0()
                    .children(self.bindings.iter().map(|(_, action_name)| {
                        let is_group = action_name.starts_with('+');
                        let label_color = if is_group {
                            Color::Success
                        } else {
                            Color::Default
                        };

                        div().child(
                            Label::new(action_name.clone())
                                .size(LabelSize::Default)
                                .color(label_color)
                                .single_line()
                                .truncate(),
                        )
                    })),
            );

        div()
            .id("which-key-buffer-panel-scroll")
            .occlude()
            .absolute()
            .bottom(bottom_offset)
            .right(px(16.))
            .min_w(px(220.))
            .max_w(max_panel_width)
            .elevation_3(cx)
            .px(px(12.))
            .child(v_flex().child(title_section).when(has_rows, |el| {
                el.child(
                    div()
                        .max_h(max_content_height)
                        .child(content)
                        .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                )
            }))
    }
}

impl EventEmitter<DismissEvent> for WhichKeyModal {}

impl Focusable for WhichKeyModal {
    fn focus_handle(&self, _cx: &App) -> gpui::FocusHandle {
        self.focus_handle.clone()
    }
}

impl ModalView for WhichKeyModal {
    fn render_bare(&self) -> bool {
        true
    }
}

fn group_bindings(
    binding_data: Vec<(Vec<Keystroke>, SharedString)>,
) -> Vec<(Vec<Keystroke>, SharedString)> {
    type GroupKey = Option<(Modifiers, String)>;
    let mut groups: HashMap<GroupKey, Vec<(Vec<Keystroke>, SharedString)>> = HashMap::new();

    // Group bindings by the visible identity of their first keystroke
    for (remaining_keystrokes, action_name) in binding_data {
        let first_key = remaining_keystrokes
            .first()
            .map(|k| (k.modifiers, k.key.clone()));
        groups
            .entry(first_key)
            .or_default()
            .push((remaining_keystrokes, action_name));
    }

    let mut result = Vec::new();

    for (first_key, mut group_bindings) in groups {
        // Remove duplicates within each group (HashMap order is arbitrary,
        // so dedup_by_key which only removes adjacent duplicates is insufficient)
        let mut seen_keystrokes = HashSet::new();
        group_bindings.retain(|(keystrokes, _)| seen_keystrokes.insert(keystrokes.clone()));

        if first_key.is_some()
            && group_bindings.len() > 1
        {
            // Separate direct (single-key) bindings from chord (multi-key) bindings
            let (direct, chords): (Vec<_>, Vec<_>) = group_bindings
                .into_iter()
                .partition(|(keystrokes, _)| keystrokes.len() <= 1);

            // Direct bindings are shown individually (dedup by action name for key_char variants)
            let mut seen_actions = HashSet::new();
            result.extend(
                direct
                    .into_iter()
                    .filter(|(_, action)| seen_actions.insert(action.clone())),
            );

            // Chord bindings are collapsed into a group
            if chords.len() > 1 {
                let first_keystroke = vec![chords[0].0[0].clone()];
                let count = chords.len();
                result.push((first_keystroke, format!("+{} keybinds", count).into()));
            } else {
                result.extend(chords);
            }
        } else {
            // Not a group or empty keystrokes - add all bindings as-is
            result.append(&mut group_bindings);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Action, Entity, Keystroke, TestAppContext, VisualTestContext, actions};
    use project::Project;
    use settings::KeymapFile;
    use workspace::{AppState, MultiWorkspace, Workspace};

    actions!(
        test_which_key,
        [
            TestActionA,
            TestActionB,
            TestChordAction,
            TestChordAction2,
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

    fn open_which_key_manual(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<WhichKeyModal> {
        workspace.update_in(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            workspace.toggle_modal(window, cx, |window, cx| {
                WhichKeyModal::new_manual(workspace_handle, window, cx)
            });
        });
        cx.run_until_parked();
        workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_modal::<WhichKeyModal>(cx)
            })
            .expect("WhichKeyModal should be open")
    }

    fn open_which_key_pending(
        workspace: &Entity<Workspace>,
        cx: &mut VisualTestContext,
    ) -> Entity<WhichKeyModal> {
        workspace.update_in(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            workspace.toggle_modal(window, cx, |window, cx| {
                WhichKeyModal::new(workspace_handle, window, cx)
            });
        });
        cx.run_until_parked();
        workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_modal::<WhichKeyModal>(cx)
            })
            .expect("WhichKeyModal should be open")
    }

    #[gpui::test]
    async fn test_manual_modal_shows_keybindings_title(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-a": "test_which_key::TestActionA" } }]"#,
                cx,
            ));
        });

        let modal = open_which_key_manual(&workspace, cx);

        modal.read_with(cx, |modal, _| {
            assert_eq!(modal.pending_keys.as_ref(), "Keybindings");
        });
    }

    #[gpui::test]
    async fn test_manual_modal_lists_bindings(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": {
                    "ctrl-a": "test_which_key::TestActionA",
                    "ctrl-b": "test_which_key::TestActionB"
                } }]"#,
                cx,
            ));
        });

        let modal = open_which_key_manual(&workspace, cx);

        modal.read_with(cx, |modal, _| {
            let action_names: Vec<&str> =
                modal.bindings.iter().map(|(_, a)| a.as_ref()).collect();
            assert!(
                action_names.contains(&"test which key: test action a"),
                "expected TestActionA in bindings, got: {:?}",
                action_names
            );
            assert!(
                action_names.contains(&"test which key: test action b"),
                "expected TestActionB in bindings, got: {:?}",
                action_names
            );
        });
    }

    #[gpui::test]
    async fn test_toggle_which_key_action_opens_modal(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        workspace.update_in(cx, |_, window, cx| {
            window.dispatch_action(
                crate::ToggleWhichKey.boxed_clone(),
                cx,
            );
        });
        cx.run_until_parked();

        let modal = workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_modal::<WhichKeyModal>(cx)
            })
            .expect("ToggleWhichKey should open the modal");

        modal.read_with(cx, |modal, _| {
            assert_eq!(modal.pending_keys.as_ref(), "Keybindings");
        });
    }

    #[gpui::test]
    async fn test_pending_key_modal_shows_continuations(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d": "test_which_key::TestChordAction" } }]"#,
                cx,
            ));
        });

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        let modal = open_which_key_pending(&workspace, cx);

        modal.read_with(cx, |modal, _| {
            assert!(
                !modal.bindings.is_empty(),
                "pending key modal should list continuations"
            );
            let action_names: Vec<&str> =
                modal.bindings.iter().map(|(_, a)| a.as_ref()).collect();
            assert!(
                action_names.contains(&"test which key: test chord action"),
                "expected TestChordAction continuation, got: {:?}",
                action_names
            );
        });
    }

    #[gpui::test]
    async fn test_escape_dismisses_manual_modal(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let _modal = open_which_key_manual(&workspace, cx);

        cx.simulate_keystrokes("escape");
        cx.run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            assert!(
                workspace.active_modal::<WhichKeyModal>(cx).is_none(),
                "modal should be dismissed after escape"
            );
        });
    }

    #[gpui::test]
    async fn test_toggle_which_key_closes_if_already_open(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        let _modal = open_which_key_manual(&workspace, cx);

        workspace.update_in(cx, |_, window, cx| {
            window.dispatch_action(crate::ToggleWhichKey.boxed_clone(), cx);
        });
        cx.run_until_parked();

        workspace.read_with(cx, |workspace, cx| {
            assert!(
                workspace.active_modal::<WhichKeyModal>(cx).is_none(),
                "second toggle should dismiss the modal"
            );
        });
    }

    #[test]
    fn test_group_bindings_collapses_shared_prefix() {
        let key_a = Keystroke::parse("a").unwrap();
        let key_b = Keystroke::parse("b").unwrap();
        let key_c = Keystroke::parse("c").unwrap();

        let bindings = vec![
            (vec![key_a.clone(), key_b], "Action One".into()),
            (vec![key_a.clone(), key_c], "Action Two".into()),
        ];

        let grouped = group_bindings(bindings);

        assert_eq!(grouped.len(), 1, "two bindings sharing first key should collapse into one group");
        let (keystrokes, action) = &grouped[0];
        assert_eq!(keystrokes.len(), 1);
        assert_eq!(keystrokes[0], key_a);
        assert_eq!(action.as_ref(), "+2 keybinds");
    }

    #[test]
    fn test_group_bindings_preserves_direct_binding_with_chord_prefix() {
        let key_h = Keystroke::parse("ctrl-h").unwrap();
        let key_b = Keystroke::parse("b").unwrap();
        let key_k = Keystroke::parse("k").unwrap();

        let bindings = vec![
            (vec![key_h.clone()], "Deploy Replace".into()),
            (vec![key_h.clone(), key_b], "Toggle Which Key".into()),
            (vec![key_h.clone(), key_k], "Describe Key".into()),
        ];

        let grouped = group_bindings(bindings);

        let direct: Vec<_> = grouped
            .iter()
            .filter(|(_, a)| a.as_ref() == "Deploy Replace")
            .collect();
        assert_eq!(
            direct.len(),
            1,
            "direct binding should appear individually, got: {:?}",
            grouped.iter().map(|(_, a)| a.as_ref()).collect::<Vec<_>>()
        );

        let groups: Vec<_> = grouped
            .iter()
            .filter(|(_, a)| a.starts_with('+'))
            .collect();
        assert_eq!(
            groups.len(),
            1,
            "chord bindings should be collapsed into one group"
        );
        assert_eq!(groups[0].1.as_ref(), "+2 keybinds");
    }

    #[test]
    fn test_group_bindings_keeps_unique_prefixes() {
        let key_a = Keystroke::parse("a").unwrap();
        let key_b = Keystroke::parse("b").unwrap();

        let bindings = vec![
            (vec![key_a], "Action A".into()),
            (vec![key_b], "Action B".into()),
        ];

        let grouped = group_bindings(bindings);

        assert_eq!(grouped.len(), 2, "bindings with different first keys stay separate");
    }

    #[test]
    fn test_group_bindings_handles_empty_keystrokes() {
        let bindings: Vec<(Vec<Keystroke>, SharedString)> = vec![
            (vec![], "Direct Action".into()),
        ];

        let grouped = group_bindings(bindings);

        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].1.as_ref(), "Direct Action");
    }

    #[gpui::test]
    async fn test_duplicate_bindings_are_deduped(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d": "test_which_key::TestChordAction" } }]"#,
                cx,
            ));
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": { "ctrl-k ctrl-d": "test_which_key::TestChordAction" } }]"#,
                cx,
            ));
        });

        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        let modal = open_which_key_pending(&workspace, cx);

        modal.read_with(cx, |modal, _| {
            let matching: Vec<_> = modal
                .bindings
                .iter()
                .filter(|(_, a)| a.as_ref() == "test which key: test chord action")
                .collect();
            assert_eq!(
                matching.len(),
                1,
                "duplicate bindings should be deduped, got: {:?}",
                modal.bindings
            );
        });
    }

    #[gpui::test]
    async fn test_manual_modal_shows_continuations_on_chord_prefix(cx: &mut TestAppContext) {
        let app_state = init_test(cx);
        let project = Project::test(app_state.fs.clone(), [], cx).await;
        let (multi_workspace, cx) =
            cx.add_window_view(|window, cx| MultiWorkspace::test_new(project.clone(), window, cx));
        let workspace = multi_workspace.read_with(cx, |mw, _| mw.workspace().clone());

        cx.update(|_window, cx| {
            cx.bind_keys(KeymapFile::load_panic_on_failure(
                r#"[{ "bindings": {
                    "ctrl-k ctrl-d": "test_which_key::TestChordAction",
                    "ctrl-k ctrl-e": "test_which_key::TestChordAction2"
                } }]"#,
                cx,
            ));
        });

        let modal = open_which_key_manual(&workspace, cx);

        // Initially shows all bindings with "Keybindings" title
        modal.read_with(cx, |modal, _| {
            assert_eq!(modal.pending_keys.as_ref(), "Keybindings");
            assert!(!modal.showing_continuations);
        });

        // Press ctrl-k to enter pending input
        cx.simulate_keystrokes("ctrl-k");
        cx.run_until_parked();

        // Modal should still be open, now showing continuations
        let modal = workspace
            .read_with(cx, |workspace, cx| {
                workspace.active_modal::<WhichKeyModal>(cx)
            })
            .expect("modal should still be open after chord prefix");

        modal.read_with(cx, |modal, _| {
            assert!(modal.showing_continuations);
            let action_names: Vec<&str> =
                modal.bindings.iter().map(|(_, a)| a.as_ref()).collect();
            assert!(
                action_names.contains(&"test which key: test chord action"),
                "expected continuations for ctrl-k, got: {:?}",
                action_names
            );
        });
    }

    #[test]
    fn test_group_bindings_dedupes_with_different_key_char() {
        let key_a = Keystroke::parse("k").unwrap();
        let mut key_a_with_char = key_a.clone();
        key_a_with_char.key_char = Some("k".to_string());

        let bindings = vec![
            (vec![key_a], "zed: describe key".into()),
            (vec![key_a_with_char], "zed: describe key".into()),
        ];

        let grouped = group_bindings(bindings);

        assert_eq!(
            grouped.len(),
            1,
            "bindings differing only in key_char should dedup, got: {:?}",
            grouped.iter().map(|(_, a)| a.as_ref()).collect::<Vec<_>>()
        );
    }
}
