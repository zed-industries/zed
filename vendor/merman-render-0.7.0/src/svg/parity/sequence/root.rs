use super::super::*;
use super::model::SequenceSvgModel;

pub(super) struct SequenceRootMetrics {
    pub(super) viewbox_width: f64,
}

pub(super) fn write_sequence_svg_root_open(
    out: &mut String,
    layout: &SequenceDiagramLayout,
    model: &SequenceSvgModel,
    diagram_id: &str,
    apply_root_overrides: bool,
) -> SequenceRootMetrics {
    let diagram_id_esc = escape_xml(diagram_id);

    let bounds = layout.bounds.clone().unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    });
    // Upstream Mermaid viewports are driven by browser layout pipelines and often land on an `f32`
    // lattice (e.g. `...49998474121094`). Mirror that by quantizing the extrema to `f32` first,
    // then computing width/height in `f32` space.
    let min_x_f32 = bounds.min_x as f32;
    let min_y_f32 = bounds.min_y as f32;
    let max_x_f32 = bounds.max_x as f32;
    let max_y_f32 = bounds.max_y as f32;

    let vb_min_x = min_x_f32 as f64;
    let vb_min_y = min_y_f32 as f64;
    let vb_w = ((max_x_f32 - min_x_f32).max(1.0)) as f64;
    let vb_h = ((max_y_f32 - min_y_f32).max(1.0)) as f64;

    let aria_labelledby_attr = model
        .acc_title
        .as_deref()
        .map(|_| format!("chart-title-{diagram_id_esc}"));
    let aria_describedby_attr = model
        .acc_descr
        .as_deref()
        .map(|_| format!("chart-desc-{diagram_id_esc}"));
    let mut max_w_attr = fmt_string(vb_w);
    let mut viewbox_attr = format!(
        "{} {} {} {}",
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w),
        fmt(vb_h)
    );
    let mut width_attr = fmt_string(vb_w);
    let mut height_attr = fmt_string(vb_h);
    if apply_root_overrides {
        apply_root_viewport_override(
            diagram_id,
            &mut viewbox_attr,
            &mut width_attr,
            &mut height_attr,
            &mut max_w_attr,
            crate::generated::sequence_root_overrides_11_12_2::lookup_sequence_root_viewport_override,
        );
    }

    let style_attr = format!("max-width: {max_w_attr}px; background-color: white;");
    root_svg::push_svg_root_open(
        out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(viewbox_attr.as_str()),
            aria_labelledby: aria_labelledby_attr.as_deref(),
            aria_describedby: aria_describedby_attr.as_deref(),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "sequence")
        },
    );

    if let Some(title) = model.acc_title.as_deref() {
        let _ = write!(
            out,
            r#"<title id="chart-title-{id}">{text}</title>"#,
            id = diagram_id_esc,
            text = escape_xml_display(title)
        );
    }
    if let Some(desc) = model.acc_descr.as_deref() {
        let _ = write!(
            out,
            r#"<desc id="chart-desc-{id}">{text}</desc>"#,
            id = diagram_id_esc,
            text = escape_xml_display(desc)
        );
    }

    SequenceRootMetrics {
        viewbox_width: vb_w,
    }
}
