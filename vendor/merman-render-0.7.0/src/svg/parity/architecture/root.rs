use std::fmt::Write as _;

use super::super::{escape_xml_display, fmt, root_svg};

pub(super) const VIEWBOX_PLACEHOLDER: &str = "__MERMAID_VIEWBOX__";
pub(super) const MAX_WIDTH_PLACEHOLDER: &str = "__MERMAID_MAX_WIDTH__";

pub(super) struct ArchitectureA11y {
    pub(super) aria_labelledby: Option<String>,
    pub(super) aria_describedby: Option<String>,
    pub(super) nodes: String,
}

pub(super) fn architecture_a11y_nodes(
    diagram_id: &str,
    acc_title: Option<&str>,
    acc_descr: Option<&str>,
) -> ArchitectureA11y {
    let diagram_id_esc = super::super::escape_xml(diagram_id);
    let aria_labelledby = acc_title
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(|_| format!("chart-title-{diagram_id_esc}"));
    let aria_describedby = acc_descr
        .map(str::trim)
        .filter(|t| !t.is_empty())
        .map(|_| format!("chart-desc-{diagram_id_esc}"));

    let mut nodes = String::new();
    if let Some(t) = acc_title.map(str::trim).filter(|t| !t.is_empty()) {
        let _ = write!(
            &mut nodes,
            r#"<title id="chart-title-{}">{}</title>"#,
            escape_xml_display(diagram_id),
            escape_xml_display(t)
        );
    }
    if let Some(d) = acc_descr.map(str::trim).filter(|t| !t.is_empty()) {
        let _ = write!(
            &mut nodes,
            r#"<desc id="chart-desc-{}">{}</desc>"#,
            escape_xml_display(diagram_id),
            escape_xml_display(d)
        );
    }

    ArchitectureA11y {
        aria_labelledby,
        aria_describedby,
        nodes,
    }
}

pub(super) struct ArchitectureRootOpenContext<'a> {
    pub(super) out: &'a mut String,
    pub(super) diagram_id: &'a str,
    pub(super) css: &'a str,
    pub(super) a11y: &'a ArchitectureA11y,
    pub(super) is_empty: bool,
    pub(super) use_max_width: bool,
    pub(super) half_icon: f64,
    pub(super) icon_size_px: f64,
}

pub(super) fn push_architecture_root_open(ctx: ArchitectureRootOpenContext<'_>) {
    let ArchitectureRootOpenContext {
        out,
        diagram_id,
        css,
        a11y,
        is_empty,
        use_max_width,
        half_icon,
        icon_size_px,
    } = ctx;

    if is_empty {
        // Preserve Mermaid's "empty diagram" fallback sizing behavior (no getBBox-derived padding).
        let vb_min_x = -half_icon;
        let vb_min_y = -half_icon;
        let vb_w = icon_size_px.max(1.0);
        let vb_h = icon_size_px.max(1.0);
        // Mermaid Architecture sets `max-width` directly from the computed `viewBox` width.
        let max_width_style = fmt(vb_w);
        let style_attr = if use_max_width {
            format!("max-width: {max_width_style}px; background-color: white;")
        } else {
            "background-color: white;".to_string()
        };
        let viewbox_attr = format!(
            "{} {} {} {}",
            fmt(vb_min_x),
            fmt(vb_min_y),
            fmt(vb_w),
            fmt(vb_h)
        );
        let width = if use_max_width {
            root_svg::SvgRootWidth::Percent100
        } else {
            root_svg::SvgRootWidth::None
        };
        root_svg::push_svg_root_open(
            out,
            root_svg::SvgRootAttrs {
                width,
                style_attr: Some(style_attr.as_str()),
                viewbox_attr: Some(viewbox_attr.as_str()),
                aria_labelledby: a11y.aria_labelledby.as_deref(),
                aria_describedby: a11y.aria_describedby.as_deref(),
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, "architecture")
            },
        );
    } else {
        let style_attr = if use_max_width {
            format!("max-width: {MAX_WIDTH_PLACEHOLDER}px; background-color: white;")
        } else {
            "background-color: white;".to_string()
        };
        let width = if use_max_width {
            root_svg::SvgRootWidth::Percent100
        } else {
            root_svg::SvgRootWidth::None
        };
        root_svg::push_svg_root_open(
            out,
            root_svg::SvgRootAttrs {
                width,
                style_attr: Some(style_attr.as_str()),
                viewbox_attr: Some(VIEWBOX_PLACEHOLDER),
                aria_labelledby: a11y.aria_labelledby.as_deref(),
                aria_describedby: a11y.aria_describedby.as_deref(),
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, "architecture")
            },
        );
    }

    out.push_str(a11y.nodes.as_str());
    let _ = write!(out, "<style>{}</style>", css);
    out.push_str("<g/><g class=\"architecture-edges\">");
}
