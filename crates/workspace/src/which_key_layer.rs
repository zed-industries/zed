use std::time::Duration;

use gpui::{
    AvailableSpace, FontWeight, KeyBinding, Keystroke, Task, WeakEntity, humanize_action_name,
};
use settings::Settings;
use theme::ThemeSettings;
use ui::{DynamicSpacing, prelude::*, text_for_keystrokes};
use util::ResultExt;
use which_key::WhichKeySettings;

use crate::Workspace;

pub struct WhichKeyLayer {
    timer: Option<Task<()>>,
    show: bool,
    pending_keys: Option<Vec<Keystroke>>,
    bindings: Option<Vec<KeyBinding>>,
    workspace: WeakEntity<Workspace>,
}

impl WhichKeyLayer {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        cx.observe_pending_input(window, |this: &mut Self, window, cx| {
            this.update_pending_keys(window, cx);
            cx.notify();
        })
        .detach();

        Self {
            timer: None,
            pending_keys: None,
            bindings: None,
            show: false,
            workspace,
        }
    }

    fn update_pending_keys(&mut self, window: &mut Window, cx: &Context<Self>) {
        self.pending_keys = window.pending_input_keystrokes().map(|x| x.to_vec());

        if let Some(pending_keys) = &self.pending_keys {
            self.bindings = Some(window.possible_bindings_for_input(pending_keys));
        } else {
            self.show = false;
            self.bindings = None;
            if self.timer.is_some() {
                self.timer = None;
            }
            return;
        }

        let which_key_settings = WhichKeySettings::get_global(cx);
        let delay_ms = which_key_settings.delay_ms;

        self.timer = Some(cx.spawn(async move |handle, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(delay_ms))
                .await;
            handle
                .update(cx, |which_key_layer, cx| {
                    which_key_layer.show = true;
                    cx.notify();
                })
                .log_err();
        }));
    }
}

impl Render for WhichKeyLayer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let which_key_settings = WhichKeySettings::get_global(cx);
        if !which_key_settings.enabled || !self.show {
            return div();
        }
        let (Some(pending_keys), Some(bindings)) = (&self.pending_keys, &self.bindings) else {
            return div();
        };
        if bindings.is_empty() {
            return div();
        }

        // Get dock widths and bottom dock height for dynamic padding
        let (left_margin, right_margin, bottom_margin) = if let Ok(margins) =
            self.workspace.read_with(cx, |workspace, cx| {
                let left_width = workspace
                    .left_dock()
                    .read(cx)
                    .active_panel_size(window, cx)
                    .unwrap_or_default();
                let right_width = workspace
                    .right_dock()
                    .read(cx)
                    .active_panel_size(window, cx)
                    .unwrap_or_default();
                let bottom_height = workspace
                    .bottom_dock()
                    .read(cx)
                    .active_panel_size(window, cx)
                    .unwrap_or_default();

                // Status bar height
                let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
                let status_bar_height = DynamicSpacing::Base08.px(cx) * 2.0 + ui_font_size;

                (left_width, right_width, bottom_height + status_bar_height)
            }) {
            margins
        } else {
            (Pixels::ZERO, Pixels::ZERO, Pixels::ZERO)
        };

        let margin = DynamicSpacing::Base12.px(cx);
        let padding = DynamicSpacing::Base16.px(cx);

        let mut binding_data: Vec<_> = bindings
            .iter()
            .map(|binding| {
                let remaining_keystrokes = binding.keystrokes()[pending_keys.len()..].to_vec();
                let action_name = humanize_action_name(binding.action().name());
                (remaining_keystrokes, action_name)
            })
            .collect();

        // Group bindings if enabled
        if which_key_settings.group {
            binding_data = group_bindings(binding_data);
        }

        // Sort bindings from shortest to longest, with groups last
        // Using stable sort to preserve relative order of equal elements
        binding_data.sort_by(|(keystrokes_a, action_a), (keystrokes_b, action_b)| {
            // Groups (actions starting with "+") should go last
            let is_group_a = action_a.starts_with('+');
            let is_group_b = action_b.starts_with('+');

            // First, separate groups from non-groups
            let group_cmp = is_group_a.cmp(&is_group_b);
            if group_cmp != std::cmp::Ordering::Equal {
                return group_cmp;
            }

            // Then sort by keystroke count
            let keystroke_cmp = keystrokes_a.len().cmp(&keystrokes_b.len());
            if keystroke_cmp != std::cmp::Ordering::Equal {
                return keystroke_cmp;
            }

            // Finally sort by text length, then lexicographically for full stability
            let text_a = text_for_keystrokes(keystrokes_a, cx);
            let text_b = text_for_keystrokes(keystrokes_b, cx);
            let text_len_cmp = text_a.len().cmp(&text_b.len());
            if text_len_cmp != std::cmp::Ordering::Equal {
                return text_len_cmp;
            }
            text_a.cmp(&text_b)
        });

        // Find the longest text width
        let longest_text = binding_data
            .iter()
            .map(|(remaining_keystrokes, action_name)| {
                create_binding_element(remaining_keystrokes, action_name, cx)
                    .into_any_element()
                    .layout_as_root(AvailableSpace::min_size(), window, cx)
                    .width
            })
            .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
            .unwrap_or(Pixels::ZERO);

        // Calculate available width (window width minus padding)
        let window_width = window.viewport_size().width;
        let available_width =
            window_width - (left_margin + right_margin + (margin * 2.0) + (padding * 2.0));

        // Calculate number of columns that can fit
        let gap_width = DynamicSpacing::Base20.px(cx);
        let columns = ((available_width + gap_width) / (longest_text + gap_width))
            .floor()
            .max(1.0) as usize;

        // Create rows for grid
        let chunks = binding_data.chunks(columns);
        let mut largest_chunk = 0;
        let mut rows = Vec::new();
        for chunk in chunks {
            let mut row = h_flex().gap(gap_width).children(chunk.iter().map(
                |(remaining_keystrokes, action_name)| {
                    div()
                        .child(create_binding_element(
                            remaining_keystrokes,
                            action_name,
                            cx,
                        ))
                        .min_w(longest_text)
                },
            ));

            if chunk.len() > largest_chunk {
                largest_chunk = chunk.len();
            }
            // Ensure all rows have equal number of children
            for _ in 0..(largest_chunk - chunk.len()) {
                row = row.child(div().min_w(longest_text));
            }

            rows.push(row);
        }

        div()
            .occlude()
            .absolute()
            .bottom(bottom_margin + margin)
            .left(left_margin + margin)
            .right(right_margin + margin)
            .elevation_3(cx)
            .p(padding)
            .child(
                v_flex()
                    .gap_3()
                    .child(
                        Label::new(text_for_keystrokes(pending_keys, cx)).weight(FontWeight::BOLD),
                    )
                    .child(v_flex().gap_2().children(rows)),
            )
    }
}

fn group_bindings(binding_data: Vec<(Vec<Keystroke>, String)>) -> Vec<(Vec<Keystroke>, String)> {
    use std::collections::HashMap;

    let mut groups: HashMap<Option<Keystroke>, Vec<(Vec<Keystroke>, String)>> = HashMap::new();

    // Group bindings by their first keystroke
    for (remaining_keystrokes, action_name) in binding_data {
        let first_key = remaining_keystrokes.first().cloned();
        groups
            .entry(first_key)
            .or_default()
            .push((remaining_keystrokes, action_name));
    }

    let mut result = Vec::new();

    for (first_key, mut group_bindings) in groups {
        if group_bindings.len() > 1 && first_key.is_some() {
            // This is a group - create a single entry with just the first keystroke
            let first_keystroke = vec![first_key.unwrap()];
            let count = group_bindings.len();
            result.push((first_keystroke, format!("+{} keybinds", count)));
        } else {
            // Not a group or empty keystrokes - add all bindings as-is
            result.append(&mut group_bindings);
        }
    }

    result
}

fn create_binding_element(
    remaining_keystrokes: &[Keystroke],
    action_name: &str,
    cx: &Context<WhichKeyLayer>,
) -> impl IntoElement {
    h_flex().children([
        Label::new(text_for_keystrokes(remaining_keystrokes, cx)).weight(FontWeight::BOLD),
        Label::new(format!(": {}", action_name)),
    ])
}
