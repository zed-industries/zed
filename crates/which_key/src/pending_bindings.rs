use std::{collections::HashMap, rc::Rc};

use gpui::{
    App, AvailableSpace, KeybindingKeystroke, Pixels, RenderOnce, ScrollHandle, Window, size,
};
use ui::{
    Divider, DividerColor, KeyBinding, LabelSize, WithScrollbar, prelude::*,
    text_for_keybinding_keystrokes,
};

use crate::PendingBinding;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct PendingBindingRow {
    pub(crate) keystrokes: Rc<[KeybindingKeystroke]>,
    pub(crate) action_name: SharedString,
    pub(crate) is_group: bool,
}

pub(crate) fn prepare_pending_bindings(
    bindings: Vec<PendingBinding>,
    cx: &App,
) -> Vec<PendingBindingRow> {
    let mut rows = group_bindings(bindings)
        .into_iter()
        .map(|row| {
            let formatted_text = text_for_keybinding_keystrokes(&row.keystrokes, cx);
            (row, formatted_text)
        })
        .collect::<Vec<_>>();

    rows.sort_by(|(row_a, text_a), (row_b, text_b)| {
        row_a
            .is_group
            .cmp(&row_b.is_group)
            .then_with(|| row_a.keystrokes.len().cmp(&row_b.keystrokes.len()))
            .then_with(|| text_a.len().cmp(&text_b.len()))
            .then_with(|| text_a.cmp(text_b))
    });
    rows.dedup_by(|(row_a, _), (row_b, _)| {
        row_a.keystrokes == row_b.keystrokes && row_a.action_name == row_b.action_name
    });

    rows.into_iter().map(|(row, _)| row).collect()
}

fn group_bindings(bindings: Vec<PendingBinding>) -> Vec<PendingBindingRow> {
    let mut groups: HashMap<Option<KeybindingKeystroke>, Vec<PendingBinding>> = HashMap::new();
    // Group bindings by their first keystroke
    for binding in bindings {
        groups
            .entry(binding.remaining_keystrokes.first().cloned())
            .or_default()
            .push(binding);
    }

    let mut result = Vec::new();
    for (first_keystroke, mut bindings) in groups {
        // Preserve which-key's adjacent-only deduplication and candidate order.
        bindings.dedup_by(|binding_a, binding_b| {
            binding_a.remaining_keystrokes == binding_b.remaining_keystrokes
        });

        if let Some(first_keystroke) = first_keystroke
            && bindings.len() > 1
        {
            // Collapse bindings sharing the next keystroke into a single row.
            result.push(PendingBindingRow {
                keystrokes: Rc::from([first_keystroke]),
                action_name: format!("+{} keybinds", bindings.len()).into(),
                is_group: true,
            });
        } else {
            // Keep individual bindings as-is when there is nothing to collapse.
            result.extend(bindings.into_iter().map(|binding| PendingBindingRow {
                keystrokes: binding.remaining_keystrokes.into(),
                action_name: binding.action_name,
                is_group: false,
            }));
        }
    }

    result
}

#[derive(IntoElement)]
pub(crate) struct PendingBindings {
    id: &'static str,
    pending_keystrokes: Rc<[KeybindingKeystroke]>,
    bindings: Rc<[PendingBindingRow]>,
    scroll_handle: ScrollHandle,
    max_content_height: Pixels,
}

impl PendingBindings {
    pub(crate) fn new(
        id: &'static str,
        pending_keystrokes: Rc<[KeybindingKeystroke]>,
        bindings: Rc<[PendingBindingRow]>,
        scroll_handle: ScrollHandle,
        max_content_height: Pixels,
    ) -> Self {
        Self {
            id,
            pending_keystrokes,
            bindings,
            scroll_handle,
            max_content_height,
        }
    }

    fn keybinding(keystrokes: Rc<[KeybindingKeystroke]>, cx: &App) -> KeyBinding {
        KeyBinding::from_keystrokes(keystrokes, KeyBinding::is_vim_mode(cx)).color(Color::Accent)
    }
}

impl RenderOnce for PendingBindings {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let has_bindings = !self.bindings.is_empty();
        // Measure the actual key components so the fixed header and scrolling rows
        // share a column width.
        let key_column_width = std::iter::once(self.pending_keystrokes.clone())
            .chain(
                self.bindings
                    .iter()
                    .map(|binding| binding.keystrokes.clone()),
            )
            .map(|keystrokes| {
                Self::keybinding(keystrokes, cx)
                    .into_any_element()
                    .layout_as_root(
                        size(AvailableSpace::MaxContent, AvailableSpace::MaxContent),
                        window,
                        cx,
                    )
                    .width
            })
            .fold(px(0.), Pixels::max);
        let content = h_flex()
            .items_start()
            .gap_2()
            .px_2()
            .py_1()
            .child(
                v_flex()
                    .w(key_column_width)
                    .gap_1()
                    .items_end()
                    .flex_shrink_0()
                    .children(self.bindings.iter().map(|binding| {
                        h_flex()
                            .h_6()
                            .flex_none()
                            .child(Self::keybinding(binding.keystrokes.clone(), cx))
                    })),
            )
            .child(
                v_flex()
                    .gap_1()
                    .flex_1()
                    .min_w_0()
                    .children(self.bindings.iter().map(|binding| {
                        h_flex().h_6().flex_none().w_full().min_w_0().child(
                            Label::new(binding.action_name.clone())
                                .size(LabelSize::Small)
                                .color(if binding.is_group {
                                    Color::Success
                                } else {
                                    Color::Default
                                })
                                .single_line()
                                .truncate(),
                        )
                    })),
            );

        v_flex()
            // Title section
            .child(
                h_flex()
                    .gap_2()
                    .px_2()
                    .py_1()
                    .child(
                        h_flex()
                            .w(key_column_width)
                            .flex_shrink_0()
                            .justify_end()
                            .child(Self::keybinding(self.pending_keystrokes, cx)),
                    )
                    .child(
                        Label::new("is waiting for more keys…")
                            .size(LabelSize::Small)
                            .color(Color::Muted)
                            .single_line()
                            .truncate(),
                    ),
            )
            .when(has_bindings, |element| {
                element.child(Divider::horizontal().color(DividerColor::BorderFaded))
            })
            .when(has_bindings, |element| {
                element.child(
                    div()
                        .max_h(self.max_content_height)
                        .child(
                            div()
                                .id(self.id)
                                .overflow_y_scroll()
                                .track_scroll(&self.scroll_handle)
                                .max_h(self.max_content_height)
                                .child(content),
                        )
                        .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                )
            })
    }
}

#[cfg(test)]
mod tests {
    use gpui::{Action as _, KeyBinding, KeyContext, Keymap, Keystroke, TestAppContext};
    use settings::KeybindSource;

    use super::*;

    fn parse_keystrokes(keystrokes: &str) -> Vec<KeybindingKeystroke> {
        keystrokes
            .split_whitespace()
            .map(|keystroke| {
                KeybindingKeystroke::from_keystroke(
                    Keystroke::parse(keystroke).expect("valid test keystroke"),
                )
            })
            .collect()
    }

    fn binding(keystrokes: &str, action_name: &str) -> PendingBinding {
        PendingBinding {
            remaining_keystrokes: parse_keystrokes(keystrokes),
            action_name: action_name.into(),
        }
    }

    fn binding_after_first_keystroke(chord: &str, action_name: &str) -> PendingBinding {
        let (_, remaining_keystrokes) = chord.split_once(' ').expect("multi-keystroke binding");
        binding(
            remaining_keystrokes,
            &command_palette::humanize_action_name(action_name),
        )
    }

    fn unparsed_keystrokes(binding: &PendingBindingRow) -> Vec<String> {
        binding
            .keystrokes
            .iter()
            .map(|keystroke| keystroke.inner().unparse())
            .collect()
    }

    #[gpui::test]
    fn test_pending_keybinding_uses_active_vim_style(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        cx.update(|window, cx| {
            // Toggle Vim notation on and back off to catch stale formatting after a mode change.
            for vim_mode in [false, true, false] {
                ui::KeyBinding::set_vim_mode(cx, vim_mode);
                for sequence in ["g", "h", "ctrl-w", "shift-h"] {
                    let keystrokes: Rc<[KeybindingKeystroke]> =
                        binding(sequence, "Action").remaining_keystrokes.into();
                    let expected = text_for_keybinding_keystrokes(&keystrokes, cx);
                    let keybinding = PendingBindings::keybinding(keystrokes, cx);
                    assert_eq!(
                        keybinding
                            .keyboard_shortcut_text(window, cx)
                            .expect("keybinding label")
                            .as_ref(),
                        expected,
                        "sequence {sequence}, Vim mode {vim_mode}",
                    );
                }
            }
        });
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_preserves_keybinding_keystrokes(cx: &mut App) {
        #[cfg(target_os = "windows")]
        let keystroke = KeybindingKeystroke::new(
            Keystroke::parse("ctrl-$").expect("valid keystroke"),
            gpui::Modifiers::control_shift(),
            "4".to_owned(),
        );
        #[cfg(not(target_os = "windows"))]
        let keystroke = KeybindingKeystroke::from_keystroke(
            Keystroke::parse("ctrl-x").expect("valid keystroke"),
        );

        for grouped in [false, true] {
            let bindings = if grouped {
                ["h", "j"]
                    .into_iter()
                    .map(|key| PendingBinding {
                        remaining_keystrokes: vec![
                            keystroke.clone(),
                            KeybindingKeystroke::from_keystroke(
                                Keystroke::parse(key).expect("valid keystroke"),
                            ),
                        ],
                        action_name: key.into(),
                    })
                    .collect()
            } else {
                vec![PendingBinding {
                    remaining_keystrokes: vec![keystroke.clone()],
                    action_name: "Action".into(),
                }]
            };
            let rows = prepare_pending_bindings(bindings, cx);
            assert_eq!(rows.len(), 1);
            let row = rows.first().expect("prepared binding");
            assert_eq!(row.keystrokes.as_ref(), std::slice::from_ref(&keystroke));
            assert_eq!(row.is_group, grouped);
            #[cfg(target_os = "windows")]
            assert_eq!(
                text_for_keybinding_keystrokes(&row.keystrokes, cx),
                "Ctrl-Shift-4"
            );
        }
    }

    #[test]
    fn test_group_bindings_only_deduplicates_adjacent_sequences() {
        for (sequences, expected_label) in [
            (["g h", "g h", "g l"], "+2 keybinds"),
            (["g h", "g l", "g h"], "+3 keybinds"),
        ] {
            let bindings = group_bindings(
                sequences
                    .into_iter()
                    .map(|sequence| binding(sequence, "zed: open settings"))
                    .collect(),
            );

            assert_eq!(bindings.len(), 1);
            let group = bindings.first().expect("group");
            assert_eq!(group.action_name.as_ref(), expected_label);
            assert!(group.is_group);
        }
    }

    #[gpui::test]
    fn test_duplicate_selection_preserves_candidate_order(cx: &mut App) {
        let contexts = [
            KeyContext::parse("Workspace").expect("valid context"),
            KeyContext::parse("Editor").expect("valid context"),
        ];
        let input = [
            Keystroke::parse("cmd-k").expect("valid keystroke"),
            Keystroke::parse("cmd-s").expect("valid keystroke"),
        ];
        for bindings in [
            vec![
                KeyBinding::new("cmd-k cmd-s", zed_actions::OpenKeymap, Some("Workspace"))
                    .with_meta(KeybindSource::User.meta()),
                KeyBinding::new("cmd-k cmd-s", zed_actions::OpenSettings, Some("Workspace"))
                    .with_meta(KeybindSource::User.meta()),
            ],
            vec![
                KeyBinding::new("cmd-k cmd-s", zed_actions::OpenSettings, Some("Editor"))
                    .with_meta(KeybindSource::Default.meta()),
                KeyBinding::new("cmd-k cmd-s", zed_actions::OpenKeymap, Some("Workspace"))
                    .with_meta(KeybindSource::User.meta()),
            ],
            vec![
                KeyBinding::new("cmd-k cmd-s", zed_actions::OpenKeymap, Some("Workspace"))
                    .with_meta(KeybindSource::Default.meta()),
                KeyBinding::load(
                    "cmd-k cmd-s",
                    Box::new(zed_actions::OpenSettings),
                    Some(
                        gpui::KeyBindingContextPredicate::parse("Workspace")
                            .expect("valid context")
                            .into(),
                    ),
                    true,
                    None,
                    cx.keyboard_mapper().as_ref(),
                )
                .expect("user binding with key equivalents")
                .with_meta(KeybindSource::User.meta()),
            ],
        ] {
            let keymap = Keymap::new(bindings);
            let (matches, _) = keymap.bindings_for_input(&input, &contexts);
            let first_match = matches.first().expect("matching binding");
            assert_eq!(
                first_match.action().name(),
                zed_actions::OpenSettings.name()
            );

            let candidates = keymap.possible_next_bindings_for_input(&input[..1], &contexts);
            assert_eq!(
                candidates
                    .iter()
                    .map(|binding| binding.action().name())
                    .collect::<Vec<_>>(),
                vec![
                    zed_actions::OpenSettings.name(),
                    zed_actions::OpenKeymap.name()
                ],
            );
            let pending = candidates
                .into_iter()
                .map(|binding| PendingBinding {
                    remaining_keystrokes: binding.keystrokes().iter().skip(1).cloned().collect(),
                    action_name: command_palette::humanize_action_name(binding.action().name())
                        .into(),
                })
                .collect();
            let rows = prepare_pending_bindings(pending, cx);
            assert_eq!(rows.len(), 1);
            let row = rows.first().expect("remaining binding");
            assert_eq!(
                row.action_name.as_ref(),
                command_palette::humanize_action_name(first_match.action().name())
            );
        }
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_sorts_shorter_chords_first(cx: &mut App) {
        let bindings = prepare_pending_bindings(
            vec![
                binding_after_first_keystroke("cmd-k z a", "zed::OpenSettings"),
                binding_after_first_keystroke("cmd-k cmd-shift-t", "theme::ToggleMode"),
            ],
            cx,
        );

        assert_eq!(
            bindings
                .iter()
                .map(|row| row.keystrokes.to_vec())
                .collect::<Vec<_>>(),
            vec![parse_keystrokes("cmd-shift-t"), parse_keystrokes("z a")],
        );
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_sorts_by_key_label_length(cx: &mut App) {
        ui::KeyBinding::set_vim_mode(cx, false);
        // These default macOS chords both follow cmd-k. Key-label length takes
        // precedence over alphabetical key order: "Z" comes before "Command-Left".
        let rows = prepare_pending_bindings(
            vec![
                binding_after_first_keystroke("cmd-k cmd-left", "workspace::ActivatePaneLeft"),
                binding_after_first_keystroke("cmd-k z", "editor::ToggleSoftWrap"),
            ],
            cx,
        );

        assert_eq!(
            rows.iter()
                .map(|row| row.keystrokes.to_vec())
                .collect::<Vec<_>>(),
            vec![parse_keystrokes("z"), parse_keystrokes("cmd-left")],
        );
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_sorts_equal_length_key_labels(cx: &mut App) {
        ui::KeyBinding::set_vim_mode(cx, false);
        // After cmd-k, language selection (m) precedes encoding selection (n),
        // even though their action names sort in the opposite order.
        let rows = prepare_pending_bindings(
            vec![
                binding_after_first_keystroke("cmd-k n", "encoding_selector::Toggle"),
                binding_after_first_keystroke("cmd-k m", "language_selector::Toggle"),
            ],
            cx,
        );

        assert_eq!(
            rows.iter().map(unparsed_keystrokes).collect::<Vec<_>>(),
            vec![vec!["m"], vec!["n"]],
        );
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_preserves_distinct_actions_with_identical_key_labels(
        cx: &mut App,
    ) {
        ui::KeyBinding::set_vim_mode(cx, false);
        for reverse in [false, true] {
            // Custom bindings are needed to exercise a display collision: é and É
            // are distinct keys, but non-Vim formatting capitalizes both to "É".
            let mut bindings = vec![
                binding_after_first_keystroke("cmd-k é", "zed::OpenSettings"),
                binding_after_first_keystroke("cmd-k É", "zed::OpenKeymap"),
            ];
            let first = bindings.first().expect("first binding");
            let second = bindings.last().expect("second binding");
            assert_ne!(first.remaining_keystrokes, second.remaining_keystrokes);
            assert_eq!(
                text_for_keybinding_keystrokes(&first.remaining_keystrokes, cx),
                text_for_keybinding_keystrokes(&second.remaining_keystrokes, cx),
            );
            if reverse {
                bindings.reverse();
            }

            let rows = prepare_pending_bindings(bindings, cx);
            let mut action_names = rows
                .iter()
                .map(|row| row.action_name.as_ref())
                .collect::<Vec<_>>();
            action_names.sort();
            assert_eq!(action_names, vec!["zed: open keymap", "zed: open settings"]);
            assert!(rows.iter().all(|row| !row.is_group));
        }
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_preserves_distinct_keys_with_identical_display_rows(
        cx: &mut App,
    ) {
        ui::KeyBinding::set_vim_mode(cx, false);
        let rows = prepare_pending_bindings(
            vec![
                binding_after_first_keystroke("cmd-k é", "zed::OpenSettings"),
                binding_after_first_keystroke("cmd-k É", "zed::OpenSettings"),
            ],
            cx,
        );

        let mut keys = rows.iter().map(unparsed_keystrokes).collect::<Vec<_>>();
        keys.sort();
        assert_eq!(keys, vec![vec!["É"], vec!["é"]]);
        for row in rows {
            assert_eq!(text_for_keybinding_keystrokes(&row.keystrokes, cx), "É");
            assert_eq!(row.action_name.as_ref(), "zed: open settings");
            assert!(!row.is_group);
        }
    }

    #[gpui::test]
    fn test_prepare_pending_bindings_groups_complete_chord_with_continuation(cx: &mut App) {
        // A custom keymap can keep Open Keymap on cmd-k cmd-s and add a longer chord.
        let rows = prepare_pending_bindings(
            vec![
                binding_after_first_keystroke("cmd-k cmd-s", "zed::OpenKeymap"),
                binding_after_first_keystroke("cmd-k cmd-s cmd-,", "zed::OpenSettings"),
                binding_after_first_keystroke("cmd-k cmd-t", "theme_selector::Toggle"),
            ],
            cx,
        );

        assert_eq!(
            rows.iter()
                .map(|row| (
                    row.keystrokes.to_vec(),
                    row.action_name.as_ref(),
                    row.is_group,
                ))
                .collect::<Vec<_>>(),
            vec![
                (parse_keystrokes("cmd-t"), "theme selector: toggle", false),
                (parse_keystrokes("cmd-s"), "+2 keybinds", true),
            ],
        );
    }
}
