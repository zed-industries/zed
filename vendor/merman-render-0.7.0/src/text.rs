mod deterministic;
mod flowchart_parity;
mod font_metrics;
mod heuristic;
mod icons;
mod markdown;
mod markdown_label;
mod measure;
mod metrics;
mod overrides;
mod svg_metrics;
mod types;
mod wrap;

pub use deterministic::DeterministicTextMeasurer;
pub use flowchart_parity::{
    flowchart_apply_mermaid_string_whitespace_height_parity,
    flowchart_apply_mermaid_styled_node_height_parity, flowchart_html_has_inline_style_tags,
    flowchart_html_line_height_px,
};
pub use font_metrics::VendoredFontMetricsTextMeasurer;
pub(crate) use heuristic::{estimate_char_width_em, estimate_line_width_px};
pub use icons::replace_fontawesome_icons;
pub(crate) use markdown::{
    MermaidMarkdownWordType, mermaid_markdown_contains_html_tags, mermaid_markdown_to_lines,
};
pub(crate) use markdown_label::{
    mermaid_markdown_contains_raw_blocks, mermaid_markdown_to_html_label_fragment,
    mermaid_markdown_to_xhtml_label_fragment, mermaid_markdown_wants_paragraph_wrap,
};
pub use measure::TextMeasurer;
#[cfg(test)]
pub(crate) use metrics::measure_flowchart_svg_like_precise_width_px;
pub(crate) use metrics::{
    flowchart_default_bold_delta_em, flowchart_default_bold_kern_delta_em,
    flowchart_default_bold_svg_right_overhang_em, is_flowchart_default_font,
    measure_wrapped_markdown_with_flowchart_bold_deltas, mermaid_markdown_to_wrapped_word_lines,
    style_requests_bold_font_weight,
};
pub use metrics::{
    measure_html_with_flowchart_bold_deltas, measure_markdown_svg_like_precise_width_px,
    measure_markdown_with_flowchart_bold_deltas, mermaid_default_bold_width_delta_px,
    mermaid_default_italic_width_delta_px,
};
pub(crate) use svg_metrics::{
    FLOWCHART_DEFAULT_FONT_KEY, flowchart_svg_edge_label_background_y_px,
    font_key_uses_courier_metrics, normalize_font_key, svg_create_text_bbox_y_offset_px,
    svg_title_bbox_vertical_extents_px, svg_wrapped_first_line_bbox_height_px,
};
pub use types::{TextMetrics, TextStyle, WrapMode};
pub(crate) use wrap::wrap_svg_text_lines_by_measurement;
pub use wrap::{
    ceil_to_1_64_px, round_to_1_64_px, round_to_1_64_px_ties_to_even, split_html_br_lines,
    wrap_label_like_mermaid_lines, wrap_label_like_mermaid_lines_floored_bbox,
    wrap_label_like_mermaid_lines_relaxed, wrap_text_lines_measurer, wrap_text_lines_px,
};

#[cfg(test)]
mod tests;
