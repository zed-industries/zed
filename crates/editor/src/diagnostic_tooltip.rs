use crate::{
    hover_popover::{MIN_POPOVER_CHARACTER_WIDTH, MIN_POPOVER_LINE_HEIGHT},
    Point,
};

use gpui::{
    size, Hitbox, Hsla, MouseButton, ScrollHandle, Size, TextStyleRefinement, View, VisualContext,
    WindowContext,
};
use language::DiagnosticEntry;
use lsp::DiagnosticSeverity;
use markdown::{Markdown, MarkdownStyle};
use serde_json::Value;
use settings::Settings;
use theme::ThemeSettings;
use ui::{prelude::*, tooltip_container, window_is_transparent};

pub(crate) struct DiagnosticTooltip {
    source: String,
    code: String,
    content: View<Markdown>,
    border_color: Option<Hsla>,
    background_color: Option<Hsla>,
    size: Size<Pixels>,
    scroll_handle: ScrollHandle,
}

impl DiagnosticTooltip {
    pub(crate) fn new(
        diagnostic: DiagnosticEntry<Point>,
        hitbox: &Hitbox,
        line_height: Pixels,
        em_width: Pixels,
        cx: &mut WindowContext,
    ) -> Self {
        let mut text = diagnostic.diagnostic.message.clone();
        if diagnostic.diagnostic.is_primary {
            if let Some(Value::Object(data)) = diagnostic.diagnostic.data.as_ref() {
                if let Some(Value::String(rendered)) = data.get("rendered") {
                    text = rendered.clone();
                }
            }
        }

        let mut border_color: Option<Hsla> = None;
        let mut background_color: Option<Hsla> = None;
        let size = size(
            (120. * em_width) // Default size
                .min(hitbox.size.width / 2.) // Shrink to half of the editor width
                .max(MIN_POPOVER_CHARACTER_WIDTH * em_width), // Apply minimum width of 20 characters
            (16. * line_height) // Default size
                .min(hitbox.size.height / 2.) // Shrink to half of the editor height
                .max(MIN_POPOVER_LINE_HEIGHT * line_height), // Apply minimum height of 4 lines
        );

        let content = cx.new_view(|cx| {
            let status_colors = cx.theme().status();

            match diagnostic.diagnostic.severity {
                DiagnosticSeverity::ERROR => {
                    background_color = Some(status_colors.error_background);
                    border_color = Some(status_colors.error_border);
                }
                DiagnosticSeverity::WARNING => {
                    background_color = Some(status_colors.warning_background);
                    border_color = Some(status_colors.warning_border);
                }
                DiagnosticSeverity::INFORMATION => {
                    background_color = Some(status_colors.info_background);
                    border_color = Some(status_colors.info_border);
                }
                DiagnosticSeverity::HINT => {
                    background_color = Some(status_colors.hint_background);
                    border_color = Some(status_colors.hint_border);
                }
                _ => {
                    background_color = Some(status_colors.ignored_background);
                    border_color = Some(status_colors.ignored_border);
                }
            };
            let settings = ThemeSettings::get_global(cx);
            let mut base_text_style = cx.text_style();
            base_text_style.refine(&TextStyleRefinement {
                font_family: Some(settings.ui_font.family.clone()),
                font_fallbacks: settings.ui_font.fallbacks.clone(),
                font_size: Some(settings.ui_font_size.into()),
                color: Some(cx.theme().colors().editor_foreground),
                background_color: Some(gpui::transparent_black()),

                ..Default::default()
            });
            let markdown_style = MarkdownStyle {
                base_text_style,
                selection_background_color: { cx.theme().players().local().selection },
                link: TextStyleRefinement {
                    underline: Some(gpui::UnderlineStyle {
                        thickness: px(1.),
                        color: Some(cx.theme().colors().editor_foreground),
                        wavy: false,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            };
            Markdown::new_text(text, markdown_style.clone(), None, None, cx)
        });

        DiagnosticTooltip {
            source: diagnostic
                .diagnostic
                .source
                .map_or_else(|| "<UNKNOWN>".to_string(), |s| s.clone()),
            code: diagnostic
                .diagnostic
                .code
                .map_or_else(|| "<UNKNOWN>".to_string(), |c| c.clone()),
            content,
            border_color,
            background_color,
            size,
            scroll_handle: ScrollHandle::new(),
        }
    }
}

impl Render for DiagnosticTooltip {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let mut markdown_div = div().py_1().px_2().child(self.content.clone());

        if let Some(background_color) = &self.background_color {
            markdown_div = markdown_div.bg(*background_color);
        }

        if let Some(border_color) = &self.border_color {
            markdown_div = markdown_div
                .border_1()
                .border_color(*border_color)
                .rounded_lg();
        }

        let diagnostic_div = div()
            .id("diagnostic")
            .block()
            .overflow_y_scroll()
            // Don't draw the background color if the theme
            // allows transparent surfaces.
            .when(window_is_transparent(cx), |this| {
                this.bg(gpui::transparent_black())
            })
            .child(markdown_div);

        tooltip_container(cx, move |this, cx| {
            this.occlude()
                .on_mouse_move(|_, cx| cx.stop_propagation())
                .on_mouse_down(MouseButton::Left, move |_, cx| {
                    cx.stop_propagation();
                })
                .child(
                    v_flex()
                        .w(self.size.width)
                        .gap_2()
                        .child(
                            h_flex()
                                .pb_1p5()
                                .gap_x_2()
                                .overflow_x_hidden()
                                .flex_wrap()
                                .justify_between()
                                .child(self.source.clone())
                                .child(self.code.clone())
                                .border_b_1()
                                .border_color(cx.theme().colors().border_variant),
                        )
                        .child(
                            div()
                                .id("inline-diagnostic-message")
                                .occlude()
                                .child(diagnostic_div)
                                .max_h(self.size.height)
                                .overflow_y_scroll()
                                .track_scroll(&self.scroll_handle),
                        ),
                )
        })
    }
}
