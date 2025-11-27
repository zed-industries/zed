//! Modal implementation for the which-key display.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, AvailableSpace, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, FontWeight,
    Keystroke, ScrollHandle, Subscription, WeakEntity, Window,
};
use settings::Settings;
use std::collections::HashMap;
use theme::ThemeSettings;
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
            _workspace: workspace.clone(),
            focus_handle: focus_handle.clone(),
            scroll_handle: ScrollHandle::new(),
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
        let margin_bottom = px(16.);
        let margin_right = px(16.);
        let panel_padding_x = px(12.);
        let panel_padding_y = px(8.);
        let key_action_gap = px(8.);
        let row_gap = px(2.);
        let row_padding_y = px(2.);
        let title_gap = px(2.);
        let divider_gap = px(2.);

        let row_count = self.bindings.len();
        let has_rows = row_count > 0;
        let viewport_size = window.viewport_size();

        // Push above status bar when visible
        // Estimate the status bar height dynamically using spacing constants
        // and theme font size since the API doesn’t provide it directly
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
        let bottom_offset = margin_bottom + status_height;

        // Title section
        let title_text: SharedString = self.pending_keys.clone();
        let build_title_section = || -> AnyElement {
            let mut column = v_flex().gap(px(0.)).child(
                div()
                    .child(
                        Label::new(title_text.clone())
                            .size(LabelSize::Small)
                            .weight(FontWeight::MEDIUM)
                            .color(Color::Muted),
                    )
                    .mb(title_gap),
            );

            if has_rows {
                column = column.child(
                    div()
                        .child(Divider::horizontal().color(DividerColor::BorderFaded))
                        .mb(divider_gap),
                );
            }

            column.into_any_element()
        };

        // Compute consistent key column width
        let key_column_width = self
            .bindings
            .iter()
            .map(|(keystrokes, _)| {
                Label::new(keystrokes.clone())
                    .size(LabelSize::Default)
                    .into_any_element()
                    .layout_as_root(AvailableSpace::min_size(), window, cx)
                    .width
            })
            .max()
            .unwrap_or(px(0.))
            .max(px(28.));

        // Measure rows (unconstrained) to derive natural width
        let mut max_row_width = px(0.);
        for (keystrokes, action_name) in &self.bindings {
            let row_size = measure_binding_row(
                keystrokes.clone(),
                action_name.clone(),
                key_column_width,
                key_action_gap,
                row_padding_y,
                None,
                window,
                cx,
            );

            max_row_width = max_row_width.max(row_size.width);
        }

        // Width: tight to content with clamps
        // Review the hard coded min/max value
        let min_panel_width = px(220.);
        let max_panel_width = px((f32::from(viewport_size.width) * 0.5).min(480.0));
        let mut panel_width = max_row_width + (panel_padding_x * 2.0);
        panel_width = panel_width.clamp(min_panel_width, max_panel_width);

        let available_content_width = panel_width - (panel_padding_x * 2.0);

        // Remeasure title/rows with width constraint to get accurate heights/wrapping
        let title_section_height = build_title_section()
            .layout_as_root(
                gpui::Size {
                    width: AvailableSpace::Definite(available_content_width),
                    height: AvailableSpace::MinContent,
                },
                window,
                cx,
            )
            .height;

        let mut row_height = px(0.);
        for (keystrokes, action_name) in &self.bindings {
            let row_size = measure_binding_row(
                keystrokes.clone(),
                action_name.clone(),
                key_column_width,
                key_action_gap,
                row_padding_y,
                Some(available_content_width),
                window,
                cx,
            );

            row_height = row_height.max(row_size.height);
        }

        if row_height == px(0.) {
            row_height = measure_binding_row(
                SharedString::new_static(""),
                SharedString::new_static(""),
                key_column_width,
                key_action_gap,
                row_padding_y,
                Some(available_content_width),
                window,
                cx,
            )
            .height;
        }

        let content_height = if row_count > 0 {
            (row_height * row_count) + (row_gap * row_count.saturating_sub(1))
        } else {
            px(0.)
        };

        let panel_vert_padding = panel_padding_y * 2.0;
        let desired_height = panel_vert_padding + title_section_height + content_height;
        let available_height_above_margin =
            (f32::from(viewport_size.height) - f32::from(bottom_offset)).max(0.0);
        let max_panel_height = px((f32::from(viewport_size.height) * 0.6).floor())
            .min(px(available_height_above_margin));
        let min_panel_height = panel_vert_padding + title_section_height;
        let panel_height = if max_panel_height < min_panel_height {
            max_panel_height
        } else {
            desired_height.clamp(min_panel_height, max_panel_height)
        };

        let list_container_height =
            (panel_height - panel_vert_padding - title_section_height).max(px(0.));

        let rows = v_flex()
            .id("which-key-rows")
            .gap(row_gap)
            .overflow_y_scroll()
            .track_scroll(&self.scroll_handle)
            .children(self.bindings.iter().map(|(keystrokes, action_name)| {
                binding_row(
                    keystrokes.clone(),
                    action_name.clone(),
                    key_column_width,
                    key_action_gap,
                    row_padding_y,
                )
                .into_any_element()
            }));

        div()
            .id("which-key-buffer-panel-scroll")
            .occlude()
            .absolute()
            .bottom(bottom_offset)
            .right(margin_right)
            .w(panel_width)
            .h(panel_height)
            .elevation_3(cx)
            .pl(panel_padding_x)
            .pr(panel_padding_x)
            .pt(panel_padding_y)
            .pb(panel_padding_y)
            .child(
                v_flex()
                    .gap(px(0.))
                    .child(build_title_section())
                    .when(has_rows, |content| {
                        content.child(
                            div()
                                .h(list_container_height)
                                .child(rows.h(list_container_height))
                                .vertical_scrollbar_for(&self.scroll_handle, window, cx),
                        )
                    }),
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

fn binding_row(
    keystrokes: SharedString,
    action_name: SharedString,
    key_column_width: Pixels,
    key_action_gap: Pixels,
    row_padding_y: Pixels,
) -> impl IntoElement {
    let is_group = action_name.starts_with('+');
    let label_color = if is_group {
        Color::Success
    } else {
        Color::Default
    };

    h_flex()
        .w_full()
        .min_w_0()
        .items_center()
        .gap(key_action_gap)
        .py(row_padding_y)
        .child(
            div()
                .flex_shrink_0()
                .w(key_column_width)
                .child(
                    Label::new(keystrokes)
                        .size(LabelSize::Default)
                        .color(Color::Accent),
                )
                .text_align(gpui::TextAlign::Right),
        )
        .child(
            div().flex_1().min_w_0().child(
                Label::new(action_name)
                    .size(LabelSize::Default)
                    .color(label_color)
                    .single_line()
                    .truncate(),
            ),
        )
}

fn measure_binding_row(
    keystrokes: SharedString,
    action_name: SharedString,
    key_column_width: Pixels,
    key_action_gap: Pixels,
    row_padding_y: Pixels,
    available_width: Option<Pixels>,
    window: &mut Window,
    cx: &mut Context<WhichKeyModal>,
) -> gpui::Size<Pixels> {
    binding_row(
        keystrokes,
        action_name,
        key_column_width,
        key_action_gap,
        row_padding_y,
    )
    .into_any_element()
    .layout_as_root(
        gpui::Size {
            width: available_width
                .map(AvailableSpace::Definite)
                .unwrap_or(AvailableSpace::MinContent),
            height: AvailableSpace::MinContent,
        },
        window,
        cx,
    )
}
