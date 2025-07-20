use std::time::Duration;

use gpui::{AvailableSpace, KeyBinding, Keystroke, Task, WeakEntity, humanize_action_name};
use settings::Settings;
use theme::ThemeSettings;
use ui::{DynamicSpacing, prelude::*, text_for_keystrokes};
use util::ResultExt;

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

        self.timer = Some(cx.spawn(async move |handle, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(600))
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
        if !self.show {
            return div();
        }
        let Some(pending_keys) = &self.pending_keys else {
            return div();
        };

        let Some(bindings) = &self.bindings else {
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
            (px(0.0), px(0.0), px(0.0))
        };

        let margin = DynamicSpacing::Base08.px(cx);
        let padding = DynamicSpacing::Base20.px(cx);

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
                    .gap_2()
                    .child(Label::new(text_for_keystrokes(pending_keys, cx)))
                    .child({
                        // Calculate the longest item text to determine column width
                        let binding_texts: Vec<String> = bindings
                            .iter()
                            .map(|binding| {
                                let remaining_keystrokes =
                                    &binding.keystrokes()[pending_keys.len()..];
                                format!(
                                    "{}: {}",
                                    text_for_keystrokes(remaining_keystrokes, cx),
                                    humanize_action_name(binding.action().name()),
                                )
                            })
                            .collect();

                        // Find the longest text
                        let longest_text = binding_texts
                            .iter()
                            .max_by_key(|text| text.len())
                            .cloned()
                            .unwrap_or_default();

                        // Create a temporary label to measure the width
                        let mut temp_label = Label::new(longest_text.clone()).into_any_element();
                        let measured_size =
                            temp_label.layout_as_root(AvailableSpace::min_size(), window, cx);
                        let item_width = measured_size.width;

                        // Calculate available width (window width minus padding)
                        let window_width = window.viewport_size().width;
                        let available_width = window_width
                            - (left_margin + right_margin + (margin * 2.0) + (padding * 2.0));

                        // Calculate number of columns that can fit
                        let gap_width = DynamicSpacing::Base03.px(cx);
                        let columns = ((available_width + gap_width) / (item_width + gap_width))
                            .floor()
                            .max(1.0) as usize;

                        // Create grid
                        v_flex().gap(gap_width).children({
                            let mut rows = Vec::new();
                            for chunk in binding_texts.chunks(columns) {
                                let row = h_flex().gap_2().children(chunk.iter().map(|text| {
                                    div().child(Label::new(text.clone())).min_w(item_width)
                                }));
                                rows.push(row);
                            }
                            rows
                        })
                    }),
            )
    }
}
