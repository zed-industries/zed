use gpui::{App, InlineInspectorState, SharedString};
use ui::{Label, LabelSize, Tooltip, prelude::*};

const PREVIEW_LIMIT: usize = 160;
const ELLIPSIS_LIMIT: usize = 64;

pub(crate) fn render_inline_inspector(state: &InlineInspectorState, cx: &App) -> Div {
    let preview = truncate_preview(&state.text_preview, PREVIEW_LIMIT);
    let truncation_summary = match state.truncation.as_ref() {
        Some(truncation) => {
            let ellipsis = truncate_preview(&truncation.ellipsis, ELLIPSIS_LIMIT);
            format!(
                "Truncation: line {}, clip_x {}, visible_width {}, ellipsis \"{}\"",
                truncation.line_ix, truncation.clip_x, truncation.visible_width, ellipsis
            )
        }
        None => "Truncation: none".to_string(),
    };

    v_flex()
        .gap_1()
        .child(Label::new("Inline").size(LabelSize::Large))
        .child(
            div()
                .text_ui(cx)
                .child(format!(
                    "Bounds: origin {}, bottom_right {}",
                    state.bounds.origin,
                    state.bounds.bottom_right()
                ))
                .child(format!("Size: {}", state.bounds.size)),
        )
        .child(
            div()
                .id("inline-content-size")
                .text_ui(cx)
                .tooltip(Tooltip::text("Size of the inline content"))
                .child(format!("Content size: {}", state.content_size)),
        )
        .child(
            div()
                .id("inline-counts")
                .text_ui(cx)
                .child(format!("Logical len: {}", state.logical_len))
                .child(format!(
                    "Lines: {}, boxes: {}",
                    state.line_count, state.box_count
                )),
        )
        .child(
            div()
                .id("inline-truncation")
                .text_ui(cx)
                .child(truncation_summary),
        )
        .child(
            div()
                .id("inline-preview")
                .text_ui(cx)
                .tooltip(Tooltip::text("Preview of the inline logical text buffer"))
                .child(format!("Text preview: \"{}\"", preview)),
        )
}

fn truncate_preview(text: &SharedString, limit: usize) -> String {
    let text = text.as_ref();
    let mut chars = text.chars();
    let mut preview = String::new();
    for _ in 0..limit {
        if let Some(ch) = chars.next() {
            preview.push(ch);
        } else {
            return preview;
        }
    }
    if chars.next().is_some() {
        preview.push_str("...");
    }
    preview
}
