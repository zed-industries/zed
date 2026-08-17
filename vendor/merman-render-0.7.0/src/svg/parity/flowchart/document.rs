use super::super::util::{escape_attr, escape_attr_into, escape_xml_into};
use super::super::{apply_root_viewport_override, fmt, fmt_max_width_px, fmt_string, root_svg};

pub(super) struct FlowchartSvgDocumentRequest<'a> {
    pub diagram_id: &'a str,
    pub diagram_type: &'a str,
    pub model: &'a crate::flowchart::FlowchartV2Model,
    pub use_max_width: bool,
    pub apply_root_overrides: bool,
    pub diagram_padding: f64,
    pub bbox_min_x: f64,
    pub bbox_min_y: f64,
    pub bbox_max_x: f64,
    pub bbox_max_y: f64,
}

pub(super) struct FlowchartSvgDocument<'a> {
    diagram_id: &'a str,
    diagram_type: &'a str,
    use_max_width: bool,
    viewbox_attr: String,
    max_w_attr: String,
    w_attr: String,
    h_attr: String,
    acc_title: Option<&'a str>,
    acc_descr: Option<&'a str>,
    aria_labelledby_raw: Option<String>,
    aria_describedby_raw: Option<String>,
    aria_labelledby_attr: Option<String>,
    aria_describedby_attr: Option<String>,
}

pub(super) fn prepare_flowchart_svg_document(
    request: FlowchartSvgDocumentRequest<'_>,
) -> FlowchartSvgDocument<'_> {
    // Chromium's `getBBox()` values frequently land on an `f32` lattice. Mermaid then computes the
    // root viewport in JS double space:
    // - viewBox.x/y = bbox.x/y - padding
    // - viewBox.w/h = bbox.width/height + 2*padding
    //
    // Mirror that by quantizing the content bounds to `f32` first, then applying padding in `f64`.
    let bbox_min_x_f32 = request.bbox_min_x as f32;
    let bbox_min_y_f32 = request.bbox_min_y as f32;
    let bbox_max_x_f32 = request.bbox_max_x as f32;
    let bbox_max_y_f32 = request.bbox_max_y as f32;
    let bbox_has_area = (bbox_max_x_f32 - bbox_min_x_f32).abs() >= 1e-6
        || (bbox_max_y_f32 - bbox_min_y_f32).abs() >= 1e-6;
    let bbox_w_f32 = if bbox_has_area {
        (bbox_max_x_f32 - bbox_min_x_f32).max(1.0)
    } else {
        0.0
    };
    let bbox_h_f32 = if bbox_has_area {
        (bbox_max_y_f32 - bbox_min_y_f32).max(1.0)
    } else {
        0.0
    };

    let vb_min_x = (bbox_min_x_f32 as f64) - request.diagram_padding;
    let vb_min_y = (bbox_min_y_f32 as f64) - request.diagram_padding;
    let vb_w = ((bbox_w_f32 as f64) + request.diagram_padding * 2.0).max(1.0);
    let vb_h = ((bbox_h_f32 as f64) + request.diagram_padding * 2.0).max(1.0);

    let mut viewbox_attr = format!(
        "{} {} {} {}",
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w),
        fmt(vb_h)
    );
    let mut max_w_attr = fmt_max_width_px(vb_w);
    let mut w_attr = fmt_string(vb_w);
    let mut h_attr = fmt_string(vb_h);
    if request.apply_root_overrides {
        apply_root_viewport_override(
            request.diagram_id,
            &mut viewbox_attr,
            &mut w_attr,
            &mut h_attr,
            &mut max_w_attr,
            crate::generated::flowchart_root_overrides_11_12_2::lookup_flowchart_root_viewport_override,
        );
    }

    let acc_title = request
        .model
        .acc_title
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    let acc_descr = request
        .model
        .acc_descr
        .as_deref()
        .map(|s| s.trim_end_matches('\n'))
        .filter(|s| !s.trim().is_empty());
    let aria_labelledby_raw = acc_title.map(|_| format!("chart-title-{}", request.diagram_id));
    let aria_describedby_raw = acc_descr.map(|_| format!("chart-desc-{}", request.diagram_id));
    let aria_labelledby_attr = aria_labelledby_raw.as_deref().map(escape_attr);
    let aria_describedby_attr = aria_describedby_raw.as_deref().map(escape_attr);

    FlowchartSvgDocument {
        diagram_id: request.diagram_id,
        diagram_type: request.diagram_type,
        use_max_width: request.use_max_width,
        viewbox_attr,
        max_w_attr,
        w_attr,
        h_attr,
        acc_title,
        acc_descr,
        aria_labelledby_raw,
        aria_describedby_raw,
        aria_labelledby_attr,
        aria_describedby_attr,
    }
}

impl FlowchartSvgDocument<'_> {
    pub(super) fn push_root_open(&self, out: &mut String) {
        if self.use_max_width {
            let style_attr = format!("max-width: {}px; background-color: white;", self.max_w_attr);
            root_svg::push_svg_root_open(
                out,
                root_svg::SvgRootAttrs {
                    class: Some("flowchart"),
                    width: root_svg::SvgRootWidth::Percent100,
                    style_attr: Some(style_attr.as_str()),
                    viewbox_attr: Some(self.viewbox_attr.as_str()),
                    aria_labelledby: self.aria_labelledby_attr.as_deref(),
                    aria_describedby: self.aria_describedby_attr.as_deref(),
                    trailing_newline: false,
                    ..root_svg::SvgRootAttrs::new(self.diagram_id, self.diagram_type)
                },
            );
        } else {
            let after_roledescription_attrs: [(&str, &str); 1] =
                [("style", "background-color: white;")];
            root_svg::push_svg_root_open(
                out,
                root_svg::SvgRootAttrs {
                    class: Some("flowchart"),
                    width: root_svg::SvgRootWidth::Fixed(self.w_attr.as_str()),
                    height_attr: Some(self.h_attr.as_str()),
                    viewbox_attr: Some(self.viewbox_attr.as_str()),
                    style_viewbox_order: root_svg::SvgRootStyleViewBoxOrder::ViewBoxThenStyle,
                    aria_labelledby: self.aria_labelledby_attr.as_deref(),
                    aria_describedby: self.aria_describedby_attr.as_deref(),
                    after_roledescription_attrs: &after_roledescription_attrs,
                    fixed_height_placement: root_svg::SvgRootFixedHeightPlacement::AfterClass,
                    trailing_newline: false,
                    ..root_svg::SvgRootAttrs::new(self.diagram_id, self.diagram_type)
                },
            );
        }
    }

    pub(super) fn push_accessibility_metadata(&self, out: &mut String) {
        if let (Some(id), Some(title)) = (self.aria_labelledby_raw.as_deref(), self.acc_title) {
            out.push_str(r#"<title id=""#);
            escape_attr_into(out, id);
            out.push_str(r#"">"#);
            escape_xml_into(out, title);
            out.push_str("</title>");
        }
        if let (Some(id), Some(descr)) = (self.aria_describedby_raw.as_deref(), self.acc_descr) {
            out.push_str(r#"<desc id=""#);
            escape_attr_into(out, id);
            out.push_str(r#"">"#);
            escape_xml_into(out, descr);
            out.push_str("</desc>");
        }
    }
}
