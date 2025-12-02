//! Modal implementation for the which-key display.

use gpui::prelude::FluentBuilder;
use gpui::{
    App, Context, DismissEvent, EventEmitter, FocusHandle, Focusable, FontWeight,
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
            _workspace: workspace,
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
                    }))
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

                        div()
                            .child(
                                Label::new(action_name.clone())
                                    .size(LabelSize::Default)
                                    .color(label_color)
                                    .single_line()
                                    .truncate(),
                            )
                    }))
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
            .child(
                v_flex()
                    .child(title_section)
                    .when(has_rows, |el| {
                        el.child(
                            div()
                                .max_h(max_content_height)
                                .child(content)
                                .vertical_scrollbar_for(&self.scroll_handle, window, cx)
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
