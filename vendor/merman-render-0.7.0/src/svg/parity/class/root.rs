use super::*;
use std::ops::Range;

pub(super) const CLASS_GRAPH_MARGIN_PX: f64 = 8.0;

const VIEWBOX_PLACEHOLDER: &str = "__MERMAID_VIEWBOX__";
const MAX_WIDTH_PLACEHOLDER: &str = "__MERMAID_MAX_WIDTH__";

pub(super) struct ClassSvgRootOpen {
    pub(super) viewbox_placeholder_range: Range<usize>,
    pub(super) max_width_placeholder_range: Range<usize>,
    pub(super) has_acc_title: bool,
    pub(super) has_acc_descr: bool,
}

pub(super) fn write_class_svg_root_open(
    out: &mut String,
    model: &ClassSvgModel,
    diagram_id: &str,
    aria_roledescription: &str,
) -> crate::Result<ClassSvgRootOpen> {
    let has_acc_title = model
        .acc_title
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty());
    let has_acc_descr = model
        .acc_descr
        .as_deref()
        .is_some_and(|s| !s.trim().is_empty());

    let aria_labelledby = has_acc_title.then(|| format!("chart-title-{}", escape_xml(diagram_id)));
    let aria_describedby = has_acc_descr.then(|| format!("chart-desc-{}", escape_xml(diagram_id)));
    let aria_roledescription_attr = super::util::escape_attr(aria_roledescription);
    let style_attr = format!("max-width: {MAX_WIDTH_PLACEHOLDER}px; background-color: white;");
    root_svg::push_svg_root_open(
        out,
        root_svg::SvgRootAttrs {
            class: Some("classDiagram"),
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(VIEWBOX_PLACEHOLDER),
            aria_labelledby: aria_labelledby.as_deref(),
            aria_describedby: aria_describedby.as_deref(),
            trailing_newline: false,
            aria_attr_order: root_svg::SvgRootAriaAttrOrder::LabelledbyThenDescribedby,
            ..root_svg::SvgRootAttrs::new(diagram_id, aria_roledescription_attr.as_str())
        },
    );

    let viewbox_pos = out
        .find(VIEWBOX_PLACEHOLDER)
        .ok_or_else(|| crate::Error::InvalidModel {
            message: "class svg root missing viewBox placeholder".to_string(),
        })?;
    let viewbox_placeholder_range = viewbox_pos..(viewbox_pos + VIEWBOX_PLACEHOLDER.len());
    let max_width_pos =
        out.find(MAX_WIDTH_PLACEHOLDER)
            .ok_or_else(|| crate::Error::InvalidModel {
                message: "class svg root missing max-width placeholder".to_string(),
            })?;
    let max_width_placeholder_range = max_width_pos..(max_width_pos + MAX_WIDTH_PLACEHOLDER.len());

    if has_acc_title {
        let _ = write!(
            out,
            r#"<title id="chart-title-{}">{}"#,
            escape_xml_display(diagram_id),
            escape_xml_display(model.acc_title.as_deref().unwrap_or_default())
        );
        out.push_str("</title>");
    }
    if has_acc_descr {
        let _ = write!(
            out,
            r#"<desc id="chart-desc-{}">{}"#,
            escape_xml_display(diagram_id),
            escape_xml_display(model.acc_descr.as_deref().unwrap_or_default())
        );
        out.push_str("</desc>");
    }

    Ok(ClassSvgRootOpen {
        viewbox_placeholder_range,
        max_width_placeholder_range,
        has_acc_title,
        has_acc_descr,
    })
}
