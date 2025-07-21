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

        let binding_data: Vec<_> = bindings
            .iter()
            .map(|binding| {
                let remaining_keystrokes = binding.keystrokes()[pending_keys.len()..].to_vec();
                let action_name = humanize_action_name(binding.action().name());
                (remaining_keystrokes, action_name)
            })
            .collect();

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
