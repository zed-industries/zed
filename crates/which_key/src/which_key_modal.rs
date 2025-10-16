//! Modal implementation for the which-key display.

use gpui::{
    App, AvailableSpace, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, FontWeight,
    Keystroke, Subscription, WeakEntity, Window, size,
};
use settings::Settings;
use std::collections::HashMap;
use theme::ThemeSettings;
use ui::{DynamicSpacing, prelude::*, text_for_keystrokes};
use workspace::{ModalView, Workspace};

use crate::FILTERED_KEYSTROKES;

pub struct WhichKeyModal {
    workspace: WeakEntity<Workspace>,
    focus_handle: FocusHandle,
    bindings: Vec<(SharedString, SharedString)>,
    pending_keys: SharedString,
    _pending_input_subscription: Subscription,
    _focus_out_subscription: Subscription,
}

impl WhichKeyModal {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        // Keep focus where it currently is
        let focus_handle = window.focused(cx).unwrap_or(cx.focus_handle());

        let handle = cx.weak_entity();
        let mut this = Self {
            workspace: workspace.clone(),
            focus_handle: focus_handle.clone(),
            bindings: Vec::new(),
            pending_keys: SharedString::new_static(""),
            _pending_input_subscription: cx.observe_pending_input(
                window,
                |this: &mut Self, window, cx| {
                    this.update_pending_keys(window, cx);
                },
            ),
            _focus_out_subscription: window.on_focus_out(&focus_handle, cx, move |_, _, cx| {
                handle.update(cx, |_, cx| cx.emit(DismissEvent)).ok();
            }),
        };
        this.update_pending_keys(window, cx);
        this
    }

    pub fn dismiss(&self, cx: &mut Context<Self>) {
        cx.emit(DismissEvent)
    }

    fn update_pending_keys(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(pending_keys) = window.pending_input_keystrokes() else {
            cx.emit(DismissEvent);
            return;
        };
        let bindings = window.possible_bindings_for_input(pending_keys);

        let mut binding_data = bindings
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
                !FILTERED_KEYSTROKES.iter().any(|filtered| {
                    keystrokes.len() >= filtered.len()
                        && keystrokes[..filtered.len()] == filtered[..]
                })
            })
            .map(|(keystrokes, action)| {
                // Map to remaining keystrokes and action name
                let remaining_keystrokes = keystrokes[pending_keys.len()..].to_vec();
                let action_name: SharedString =
                    command_palette::humanize_action_name(action.name()).into();
                (remaining_keystrokes, action_name)
            })
            .collect();

        binding_data = group_bindings(binding_data);

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
        binding_data.dedup();
        self.pending_keys = text_for_keystrokes(&pending_keys, cx).into();
        self.bindings = binding_data
            .into_iter()
            .map(|(keystrokes, action)| (text_for_keystrokes(&keystrokes, cx).into(), action))
            .collect();
    }
}

impl Render for WhichKeyModal {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let ui_font_size = ThemeSettings::get_global(cx).ui_font_size(cx);
        let status_bar_height = DynamicSpacing::Base08.px(cx) * 2.0 + ui_font_size;

        let is_zoomed = self
            .workspace
            .read_with(cx, |workspace, _cx| workspace.zoomed_item().is_some())
            .unwrap_or(false);

        // Get dock widths and bottom dock height for dynamic padding
        // If workspace is zoomed, ignore panel padding and render at bottom of buffer
        let (left_margin, right_margin) = if let Ok(margins) =
            self.workspace.read_with(cx, |workspace, cx| {
                if is_zoomed {
                    return (Pixels::ZERO, Pixels::ZERO);
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

                (left_width, right_width)
            }) {
            margins
        } else {
            (Pixels::ZERO, Pixels::ZERO)
        };

        let column_gap = DynamicSpacing::Base32.px(cx); // Gap between columns
        let row_gap = DynamicSpacing::Base04.px(cx); // Gap between rows
        let content_gap = DynamicSpacing::Base08.px(cx); // Gap between current pending keystroke and grid of keys+actions
        let margin = DynamicSpacing::Base08.px(cx); // Margin around the panel
        let padding = DynamicSpacing::Base16.px(cx); // Padding inside the panel

        // Calculate column width based on UI font size (as maximum)
        let max_column_width = ui_font_size * 125.0;

        // Calculate actual column width based on largest binding element
        let largest_binding = self
            .bindings
            .iter()
            .max_by_key(|(remaining_keystrokes, action_name)| {
                remaining_keystrokes.len() + action_name.len()
            })
            .map(|(remaining_keystrokes, action_name)| {
                create_aligned_binding_element(
                    remaining_keystrokes.clone(),
                    action_name.clone(),
                    None,
                )
                .into_any_element()
                .layout_as_root(AvailableSpace::min_size(), window, cx)
            })
            .unwrap_or(size(Pixels::ZERO, Pixels::ZERO));

        // Final width of the columns
        let column_width = largest_binding.width.min(max_column_width);

        // Calculate available width (window width minus padding)
        let window_width = largest_binding.width.min(max_column_width);
        let available_width =
            window_width - (left_margin + right_margin + (margin * 2.0) + (padding * 2.0));

        // Calculate number of columns that can fit
        let columns = ((available_width + column_gap) / (column_width + column_gap))
            .floor()
            .max(1.0) as usize;

        // Calculate rows per column
        let total_items = self.bindings.len();
        let rows_per_column = (total_items + columns - 1).div_ceil(columns);

        // Create columns
        let mut column_elements = Vec::new();
        for col in 0..columns {
            let start_idx = col * rows_per_column;
            let end_idx = ((col + 1) * rows_per_column).min(total_items);

            if start_idx >= total_items {
                break;
            }

            let column_items = &self.bindings[start_idx..end_idx];

            // Find the longest_keystroke text width for this column
            let column_longest_keystroke_width = column_items
                .iter()
                .max_by_key(|(remaining_keystrokes, _)| {
                    (
                        remaining_keystrokes.len(),
                        remaining_keystrokes.ends_with(|c| c == 'm' || c == 'w'),
                    )
                })
                .map(|(remaining_keystrokes, _)| {
                    Label::new(remaining_keystrokes.clone())
                        .into_any_element()
                        .layout_as_root(AvailableSpace::min_size(), window, cx)
                        .width
                        + px(10.)
                });

            let column = v_flex().gap(row_gap).children(column_items.iter().map(
                |(remaining_keystrokes, action_name)| {
                    create_aligned_binding_element(
                        remaining_keystrokes.clone(),
                        action_name.clone(),
                        column_longest_keystroke_width,
                    )
                },
            ));

            column_elements.push(column);
        }

        // Calculate real size of 1 row
        let row_height = largest_binding.height;

        // Calculate height
        let base_height = (padding * 2) /* Container padding */
            + (row_height) /* Pending keys */
            + content_gap; /* Pending keys gap */
        let total_height = base_height
            + (rows_per_column * row_height) /* Rows */
            + ((rows_per_column - 1) * row_gap); /* Rows gap */

        // Calculate minimum height (to show ~2.5 rows, using 2.15 as the last row spills over in the margin)
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

        let panel_bottom_y = status_bar_height + margin;

        // Adjust height to avoid covering cursor
        let adjusted_height = if let Some(cursor_pos) = cursor_position {
            let cursor_padding = (ThemeSettings::get_global(cx).buffer_font_size(cx)
                * ThemeSettings::get_global(cx).line_height())
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
            .h(adjusted_height)
            .child(
                v_flex()
                    .gap(content_gap)
                    .child(Label::new(self.pending_keys.clone()).weight(FontWeight::BOLD))
                    .child(
                        h_flex()
                            .gap(column_gap)
                            .items_start()
                            .children(column_elements),
                    ),
            )
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
    let mut groups: HashMap<Option<Keystroke>, Vec<(Vec<Keystroke>, SharedString)>> =
        HashMap::new();

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
            result.push((first_keystroke, format!("+{} keybinds", count).into()));
        } else {
            // Not a group or empty keystrokes - add all bindings as-is
            result.append(&mut group_bindings);
        }
    }

    result
}

fn create_aligned_binding_element(
    keystrokes: SharedString,
    action_name: SharedString,
    keystroke_width: Option<Pixels>,
) -> impl IntoElement {
    let keystroke = div()
        .when_some(keystroke_width, |div, width| div.w(width))
        .child(Label::new(keystrokes).color({
            if action_name.starts_with('+') {
                Color::Success
            } else {
                Color::Accent
            }
        }))
        .text_align(gpui::TextAlign::Right);

    h_flex().items_center().gap_1p5().children([
        keystroke.into_any_element(),
        Label::new(action_name).truncate().into_any_element(),
    ])
}
