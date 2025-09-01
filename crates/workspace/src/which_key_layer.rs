use crate::Workspace;
use crate::{BottomDockLayout, WorkspaceSettings};
use gpui::{
    AvailableSpace, FontWeight, KeyBinding, Keystroke, Task, WeakEntity, humanize_action_name,
};
use settings::Settings;
use std::time::Duration;
use theme::ThemeSettings;
use ui::{DynamicSpacing, prelude::*, text_for_keystrokes};
use util::ResultExt;
use which_key::{WhichKeyLocation, WhichKeySettings};

// Hard-coded list of keystrokes to filter out from which-key display
static FILTERED_KEYSTROKES: &[&str] = &[
    // // Modifiers on normal vim commands
    // "g h",
    // "g j",
    // "g k",
    // "g l",
    // "g $",
    // "g ^",
    // // Duplicate keys with "ctrl" held, e.g. "ctrl-w ctrl-a" is duplicate of "ctrl-w a"
    // "ctrl-w ctrl-a",
    // "ctrl-w ctrl-c",
    // "ctrl-w ctrl-h",
    // "ctrl-w ctrl-j",
    // "ctrl-w ctrl-k",
    // "ctrl-w ctrl-l",
    // "ctrl-w ctrl-n",
    // "ctrl-w ctrl-o",
    // "ctrl-w ctrl-p",
    // "ctrl-w ctrl-q",
    // "ctrl-w ctrl-s",
    // "ctrl-w ctrl-v",
    // "ctrl-w ctrl-w",
    // "ctrl-w ctrl-]",
    // "ctrl-w ctrl-shift-w",
    // "ctrl-w ctrl-g t",
    // "ctrl-w ctrl-g shift-t",
];

pub struct WhichKeyLayer {
    timer: Option<Task<()>>,
    show: bool,
    pending_keys: Option<Vec<Keystroke>>,
    bindings: Option<Vec<KeyBinding>>,
    workspace: WeakEntity<Workspace>,
    filtered_keystrokes: Vec<Vec<Keystroke>>,
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
            filtered_keystrokes: FILTERED_KEYSTROKES
                .iter()
                .filter_map(|s| {
                    let keystrokes: Result<Vec<_>, _> = s
                        .split(' ')
                        .map(|keystroke_str| Keystroke::parse(keystroke_str))
                        .collect();
                    keystrokes.ok()
                })
                .collect(),
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
            // All return paths must return the same concrete type, so we convert `div()` into an `AnyElement`.
            return div().into_any_element();
        }
        let (Some(pending_keys), Some(bindings)) = (&self.pending_keys, &self.bindings) else {
            // All return paths must return the same concrete type, so we convert `div()` into an `AnyElement`.
            return div().into_any_element();
        };
        if bindings.is_empty() {
            // All return paths must return the same concrete type, so we convert `div()` into an `AnyElement`.
            return div().into_any_element();
        }

        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let status_bar_height = DynamicSpacing::Base08.px(cx) * 2.0 + ui_font_size;

        let is_zoomed = self
            .workspace
            .read_with(cx, |workspace, _cx| workspace.zoomed.is_some())
            .unwrap_or(false);

        // Get dock widths and bottom dock height for dynamic padding
        // If workspace is zoomed, ignore panel padding and render at bottom of buffer
        let (left_margin, right_margin, bottom_margin) = if let Ok(margins) =
            self.workspace.read_with(cx, |workspace, cx| {
                if is_zoomed {
                    return (Pixels::ZERO, Pixels::ZERO, Pixels::ZERO);
                }

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

                (left_width, right_width, bottom_height)
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
                // Map to keystrokes
                (
                    binding
                        .keystrokes()
                        .iter()
                        .map(|k| k.inner().to_owned())
                        .collect::<Vec<_>>(),
                    binding.action(),
                )
            })
            .filter(|(keystrokes, _action)| {
                // Check if this binding matches any filtered keystroke pattern
                !self.filtered_keystrokes.iter().any(|filtered| {
                    keystrokes.len() >= filtered.len()
                        && keystrokes[..filtered.len()] == filtered[..]
                })
            })
            .map(|(keystrokes, action)| {
                // Map to remaining keystrokes and action name
                let remaining_keystrokes = keystrokes[pending_keys.len()..].to_vec();
                let action_name = humanize_action_name(action.name());
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
        // Remove duplicates
        binding_data.dedup();

        // Check if we should show in left panel
        if which_key_settings.location == WhichKeyLocation::LeftPanel
            && left_margin >= ui_font_size * 20.0
            && !is_zoomed
        {
            return self
                .render_in_left_panel(
                    pending_keys,
                    &binding_data,
                    left_margin,
                    bottom_margin,
                    status_bar_height,
                    window,
                    cx,
                )
                .into_any_element();
        }

        // Calculate column width based on UI font size (as maximum)
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let max_column_width = ui_font_size * 125.0;

        // Calculate actual column width based on largest binding element
        let actual_column_width = binding_data
            .iter()
            .map(|(remaining_keystrokes, action_name)| {
                create_aligned_binding_element(remaining_keystrokes, action_name, None, cx)
                    .into_any_element()
                    .layout_as_root(AvailableSpace::min_size(), window, cx)
                    .width
            })
            .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap())
            .unwrap_or(Pixels::ZERO);

        // Final width of the columns
        let column_width = actual_column_width.min(max_column_width);

        // Calculate available width (window width minus padding)
        let window_width = window.viewport_size().width;
        let available_width =
            window_width - (left_margin + right_margin + (margin * 2.0) + (padding * 2.0));

        let column_gap = DynamicSpacing::Base32.px(cx); // Gap between columns
        let row_gap = DynamicSpacing::Base04.px(cx); // Gap between rows
        let content_gap = px(10.0); // Gap between current pending keystroke and grid of keys+actions

        // Calculate number of columns that can fit
        let columns = ((available_width + column_gap) / (column_width + column_gap))
            .floor()
            .max(1.0) as usize;

        // Calculate rows per column
        let total_items = binding_data.len();
        let rows_per_column = (total_items + columns - 1) / columns; // Ceiling division

        // Create columns
        let mut column_elements = Vec::new();
        for col in 0..columns {
            let start_idx = col * rows_per_column;
            let end_idx = ((col + 1) * rows_per_column).min(total_items);

            if start_idx >= total_items {
                break;
            }

            let column_items = &binding_data[start_idx..end_idx];

            // Find the longest keystroke text width for this column
            let column_longest_keystroke_width = column_items
                .iter()
                .map(|(remaining_keystrokes, _)| {
                    Label::new(text_for_keystrokes(remaining_keystrokes, cx))
                        .into_any_element()
                        .layout_as_root(AvailableSpace::min_size(), window, cx)
                        .width
                })
                .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

            let column = v_flex().gap(row_gap).children(column_items.iter().map(
                |(remaining_keystrokes, action_name)| {
                    create_aligned_binding_element(
                        remaining_keystrokes,
                        action_name,
                        column_longest_keystroke_width,
                        cx,
                    )
                },
            ));

            column_elements.push(column);
        }

        // Calculate real size of 1 row
        let row_height = Label::new("")
            .into_any_element()
            .layout_as_root(AvailableSpace::min_size(), window, cx)
            .height;

        // Calculate height
        let base_height = (padding * 2) /* Container padding */
            + (row_height) /* Pending keys */
            + content_gap; /* Pending keys gap */
        let total_height = base_height
            + (rows_per_column * row_height) /* Rows */
            + ((rows_per_column - 1) * row_gap); /* Rows gap */
        // Calculate minimum height (to show 2.5 rows, using 2.15 as the last row spills over in the margin)
        let minimum_rows = (rows_per_column as f32).min(2.15);
        let minimum_height = base_height
            + (minimum_rows * row_height) /* Rows */
            + ((minimum_rows - 1.0) * row_gap); /* Rows gap */

        let cursor_position = self
            .workspace
            .read_with(cx, |workspace, cx| {
                workspace
                    .active_item(cx)
                    .and_then(|item| item.pixel_position_of_cursor(cx))
            })
            .unwrap_or(None);

        let panel_bottom_y = bottom_margin + status_bar_height + margin;

        // Adjust height to avoid covering cursor
        let adjusted_height = if let Some(cursor_pos) = cursor_position {
            let cursor_padding = ((ThemeSettings::get_global(cx).buffer_font_size(cx)
                * ThemeSettings::get_global(cx).line_height())
                / 2.0)
                + margin;
            let window_height = window.viewport_size().height;
            // Calculate available space from cursor to bottom of panel
            let available_space = window_height - panel_bottom_y - cursor_pos.y - cursor_padding;
            if available_space > px(0.0) {
                total_height.min(available_space).max(minimum_height)
            } else {
                total_height
            }
        } else {
            total_height
        };

        div()
            .id("which-key-buffer-panel-scroll")
            .occlude()
            .absolute()
            .bottom(panel_bottom_y)
            .left(left_margin + margin)
            .right(right_margin + margin)
            .elevation_3(cx)
            .p(padding)
            .overflow_y_scroll()
            // .max_h_128()
            .h(adjusted_height)
            .child(
                v_flex()
                    .gap(content_gap)
                    .child(
                        Label::new(text_for_keystrokes(pending_keys, cx)).weight(FontWeight::BOLD),
                    )
                    .child(
                        h_flex()
                            .gap(column_gap)
                            .items_start()
                            .children(column_elements),
                    ),
            )
            .into_any_element() // All return paths must return the same concrete type.
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
        // Remove duplicates within each group
        group_bindings.dedup_by_key(|(keystrokes, _)| keystrokes.clone());

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

fn create_aligned_binding_element(
    remaining_keystrokes: &[Keystroke],
    action_name: &str,
    keystroke_width: Option<Pixels>,
    cx: &Context<WhichKeyLayer>,
) -> impl IntoElement {
    let keystroke = div()
        .when_some(keystroke_width, |div, width| div.w(width))
        .child(
            Label::new(text_for_keystrokes(remaining_keystrokes, cx)).color({
                if action_name.starts_with('+') {
                    Color::Success
                } else {
                    Color::Accent
                }
            }),
        )
        .text_align(gpui::TextAlign::Right);

    h_flex().items_center().gap_1p5().children([
        keystroke.into_any_element(),
        Label::new(action_name.to_string())
            .truncate()
            .into_any_element(),
    ])
}

impl WhichKeyLayer {
    fn render_in_left_panel(
        &self,
        pending_keys: &[Keystroke],
        binding_data: &[(Vec<Keystroke>, String)],
        left_margin: Pixels,
        bottom_margin: Pixels,
        status_bar_height: Pixels,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let workspace_settings = WorkspaceSettings::get_global(cx);
        let margin = DynamicSpacing::Base12.px(&cx);
        let padding = DynamicSpacing::Base16.px(&cx);
        let bottom_offset = if workspace_settings.bottom_dock_layout == BottomDockLayout::Contained
        {
            margin + status_bar_height
        } else {
            margin + status_bar_height + bottom_margin
        };

        // For left panel, we use a single column layout
        let available_width = left_margin - (padding * 2.0);

        // Find the longest keystroke text width for alignment
        let longest_keystroke_width = binding_data
            .iter()
            .map(|(remaining_keystrokes, _)| {
                Label::new(text_for_keystrokes(remaining_keystrokes, cx))
                    .weight(FontWeight::BOLD)
                    .into_any_element()
                    .layout_as_root(AvailableSpace::min_size(), window, cx)
                    .width
            })
            .max_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

        let items = binding_data
            .iter()
            .map(|(remaining_keystrokes, action_name)| {
                create_aligned_binding_element(
                    remaining_keystrokes,
                    action_name,
                    longest_keystroke_width,
                    cx,
                )
            });

        div()
            .id("which-key-left-panel-scroll")
            .occlude()
            .absolute()
            .top(margin)
            .left(margin)
            .bottom(bottom_offset)
            .w(available_width)
            .elevation_3(cx)
            .p(padding)
            .overflow_y_scroll()
            .child(
                v_flex()
                    .max_h_full()
                    .gap_2p5()
                    .child(
                        Label::new(text_for_keystrokes(pending_keys, cx)).weight(FontWeight::BOLD),
                    )
                    .child(div().flex_grow().child(v_flex().gap_1().children(items))),
            )
    }
}
