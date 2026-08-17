use super::theme::TimelineTheme;
use super::*;
use merman_core::diagrams::timeline::TimelineDiagramRenderModel;

fn timeline_css(
    diagram_id: &str,
    effective_config: &serde_json::Value,
    theme: &TimelineTheme,
) -> String {
    let id = escape_xml(diagram_id);

    // Keep `:root` last (matches upstream Mermaid timeline SVG baselines).
    let parts = info_css_parts_with_config(diagram_id, effective_config);
    let root_rule = parts.root_rule;
    let mut out = parts.css_prefix;
    let scoped_drop_shadow = if diagram_id.is_empty() {
        theme.drop_shadow.clone()
    } else {
        format!("url(#{id}-drop-shadow)")
    };

    let _ = write!(&mut out, r#"#{} .edge{{stroke-width:3;}}"#, id);
    for (i, section_theme) in theme.sections.iter().enumerate() {
        let section = i as i64 - 1;
        let sw = 17 - 3 * (i as i64);

        if theme.is_redux_theme {
            let border_color = theme
                .border_colors
                .get(i)
                .cloned()
                .unwrap_or_else(|| theme.node_border.clone());
            let redux_fill = if theme.is_color_theme && !theme.is_dark_theme {
                border_color.clone()
            } else {
                theme.main_bkg.clone()
            };
            let redux_stroke = if theme.is_color_theme {
                border_color
            } else {
                theme.node_border.clone()
            };
            let _ = write!(
                &mut out,
                r#"#{} .section-{} rect,#{} .section-{} path,#{} .section-{} circle{{fill:{};stroke:{};stroke-width:{};filter:{};}}#{} .section-{} text{{fill:{};font-weight:{};}}#{} .node-icon-{}{{font-size:40px;color:{};}}#{} .section-edge-{}{{stroke:{};}}#{} .edge-depth-{}{{stroke-width:{};}}#{} .section-{} line{{stroke:{};stroke-width:3;}}#{} .lineWrapper line{{stroke:{};stroke-width:{};}}#{} .disabled,#{} .disabled circle,#{} .disabled text{{fill:{};}}#{} .disabled text{{fill:{};}}"#,
                id,
                section,
                id,
                section,
                id,
                section,
                redux_fill,
                redux_stroke,
                theme.stroke_width,
                scoped_drop_shadow,
                id,
                section,
                theme.node_border,
                theme.font_weight,
                id,
                section,
                section_theme.c_scale_label,
                id,
                section,
                section_theme.c_scale,
                id,
                section,
                sw,
                id,
                section,
                section_theme.c_scale_inv,
                id,
                theme.node_border,
                theme.stroke_width,
                id,
                id,
                id,
                theme.disabled_fill,
                id,
                theme.disabled_text_fill,
            );
        } else {
            let _ = write!(
                &mut out,
                r#"#{} .section-{} rect,#{} .section-{} path,#{} .section-{} circle,#{} .section-{} path{{fill:{};}}#{} .section-{} text{{fill:{};}}#{} .node-icon-{}{{font-size:40px;color:{};}}#{} .section-edge-{}{{stroke:{};}}#{} .edge-depth-{}{{stroke-width:{};}}#{} .section-{} line{{stroke:{};stroke-width:3;}}#{} .lineWrapper line{{stroke:{};}}#{} .disabled,#{} .disabled circle,#{} .disabled text{{fill:{};}}#{} .disabled text{{fill:{};}}"#,
                id,
                section,
                id,
                section,
                id,
                section,
                id,
                section,
                section_theme.c_scale,
                id,
                section,
                section_theme.c_scale_label,
                id,
                section,
                section_theme.c_scale_label,
                id,
                section,
                section_theme.c_scale,
                id,
                section,
                sw,
                id,
                section,
                section_theme.c_scale_inv,
                id,
                section_theme.c_scale_label,
                id,
                id,
                id,
                theme.disabled_fill,
                id,
                theme.disabled_text_fill,
            );
        }
    }

    let _ = write!(
        &mut out,
        r#"#{} .section-root rect,#{} .section-root path,#{} .section-root circle{{fill:{};}}#{} .section-root text{{fill:{};}}#{} .icon-container{{height:100%;display:flex;justify-content:center;align-items:center;}}#{} .edge{{fill:none;}}#{} .eventWrapper{{filter:brightness(120%);}}"#,
        id, id, id, theme.root_fill, id, theme.root_label, id, id, id
    );

    out.push_str(&root_rule);
    out
}

pub(super) fn render_timeline_diagram_svg(
    layout: &TimelineDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let _ = semantic;
    render_timeline_diagram_svg_inner(layout, effective_config, diagram_title, measurer, options)
}

pub(super) fn render_timeline_diagram_svg_model(
    layout: &TimelineDiagramLayout,
    _model: &TimelineDiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    render_timeline_diagram_svg_inner(layout, effective_config, diagram_title, measurer, options)
}

fn render_timeline_diagram_svg_inner(
    layout: &TimelineDiagramLayout,
    effective_config: &serde_json::Value,
    _diagram_title: Option<&str>,
    _measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");
    let theme = PresentationTheme::new(effective_config).timeline();
    let is_redux_theme = theme.is_redux_theme;

    let bounds = layout.bounds.clone().unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    });
    // Mermaid's root viewport is derived from browser `getBBox()` values, which frequently land on
    // a single-precision lattice. Mirror that by quantizing extrema to `f32`, then computing
    // width/height in `f32` space.
    let min_x_f32 = bounds.min_x as f32;
    let min_y_f32 = bounds.min_y as f32;
    let max_x_f32 = bounds.max_x as f32;
    let max_y_f32 = bounds.max_y as f32;

    let vb_min_x = min_x_f32 as f64;
    let vb_min_y = min_y_f32 as f64;
    let vb_w = ((max_x_f32 - min_x_f32).max(1.0)) as f64;
    let vb_h = ((max_y_f32 - min_y_f32).max(1.0)) as f64;

    fn node_line_class(section_class: &str) -> String {
        let rest = section_class
            .strip_prefix("section-")
            .unwrap_or(section_class);
        format!("node-line-{rest}")
    }

    fn render_node(
        out: &mut String,
        diagram_id: &str,
        node_count: &mut usize,
        n: &crate::model::TimelineNodeLayout,
        is_redux_theme: bool,
        is_event: bool,
    ) {
        let node_id = scoped_svg_id(diagram_id, &format!("node-{node_count}"));
        *node_count += 1;
        let w = n.width.max(1.0);
        let h = n.height.max(1.0);
        let rd = 5.0;
        let d = if is_redux_theme {
            format!(
                "M0 {y0} v{v1} h{w} v{h} H0 Z",
                y0 = fmt(h - rd),
                v1 = fmt(-(h - rd)),
                w = fmt(w),
                h = fmt(h),
            )
        } else {
            format!(
                "M0 {y0} v{v1} q0,-5 5,-5 h{hw} q5,0 5,5 v{v2} H0 Z",
                y0 = fmt(h - rd),
                v1 = fmt(-h + 2.0 * rd),
                hw = fmt(w - 2.0 * rd),
                v2 = fmt(h - rd),
            )
        };

        let _ = write!(
            out,
            r#"<g class="timeline-node {section_class}">"#,
            section_class = escape_attr(&n.section_class)
        );
        out.push_str("<g>");
        let _ = write!(
            out,
            r#"<path id="{node_id}" class="node-bkg node-undefined" d="{d}"/>"#,
            node_id = escape_attr(&node_id),
            d = escape_attr(&d)
        );
        if !is_redux_theme {
            let _ = write!(
                out,
                r#"<line class="{line_class}" x1="0" y1="{y}" x2="{x2}" y2="{y}"/>"#,
                line_class = escape_attr(&node_line_class(&n.section_class)),
                y = fmt(h),
                x2 = fmt(w)
            );
        }
        out.push_str("</g>");

        let tx = w / 2.0;
        let ty = if is_redux_theme {
            if is_event {
                n.padding / 2.0 + 3.0
            } else {
                n.padding
            }
        } else {
            n.padding / 2.0
        };
        let _ = write!(
            out,
            r#"<g transform="translate({x}, {y})">"#,
            x = fmt(tx),
            y = fmt(ty)
        );
        out.push_str(r#"<text dy="1em" alignment-baseline="middle" dominant-baseline="middle" text-anchor="middle">"#);
        for (idx, line) in n.label_lines.iter().enumerate() {
            let dy = if idx == 0 { "1em" } else { "1.1em" };
            let _ = write!(
                out,
                r#"<tspan x="0" dy="{dy}">{text}</tspan>"#,
                dy = dy,
                text = escape_xml(line)
            );
        }
        out.push_str("</text></g></g>");
    }

    let mut max_w_attr = fmt_max_width_px(vb_w);
    let mut viewbox_attr = format!(
        "{} {} {} {}",
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w),
        fmt(vb_h)
    );
    let mut w_attr = fmt(vb_w).to_string();
    let mut h_attr = fmt(vb_h).to_string();
    apply_root_viewport_override(
        diagram_id,
        &mut viewbox_attr,
        &mut w_attr,
        &mut h_attr,
        &mut max_w_attr,
        crate::generated::timeline_root_overrides_11_12_2::lookup_timeline_root_viewport_override,
    );

    let mut out = String::new();
    let style_attr = format!("max-width: {max_w_attr}px; background-color: white;");
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(&viewbox_attr),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "timeline")
        },
    );
    let css = timeline_css(diagram_id, effective_config, &theme);
    let _ = write!(&mut out, r#"<style>{}</style>"#, css);
    out.push_str(r#"<g/>"#);
    out.push_str(r#"<g/>"#);
    let mut node_count = 0usize;
    let arrowhead_id = scoped_svg_id(diagram_id, "arrowhead");
    let arrowhead_url = scoped_svg_url(diagram_id, "arrowhead");
    let _ = write!(
        &mut out,
        r#"<defs><marker id="{}" refX="5" refY="2" markerWidth="6" markerHeight="4" orient="auto"><path d="M 0,0 V 4 L6,2 Z"/></marker></defs>"#,
        escape_attr(&arrowhead_id)
    );

    for section in &layout.sections {
        let node = &section.node;
        let _ = write!(
            &mut out,
            r#"<g transform="translate({x}, {y})">"#,
            x = fmt(node.x),
            y = fmt(node.y)
        );
        render_node(
            &mut out,
            diagram_id,
            &mut node_count,
            node,
            is_redux_theme,
            false,
        );
        out.push_str("</g>");

        for task in &section.tasks {
            let task_node = &task.node;
            let _ = write!(
                &mut out,
                r#"<g class="taskWrapper" transform="translate({x}, {y})">"#,
                x = fmt(task_node.x),
                y = fmt(task_node.y)
            );
            render_node(
                &mut out,
                diagram_id,
                &mut node_count,
                task_node,
                is_redux_theme,
                false,
            );
            out.push_str("</g>");

            let _ = write!(
                &mut out,
                r#"<g class="lineWrapper"><line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke-width="2" stroke="black" marker-end="{marker_end}" stroke-dasharray="5,5"/></g>"#,
                x1 = fmt(task.connector.x1),
                y1 = fmt(task.connector.y1),
                x2 = fmt(task.connector.x2),
                y2 = fmt(task.connector.y2),
                marker_end = escape_attr(&arrowhead_url),
            );

            for ev in &task.events {
                let _ = write!(
                    &mut out,
                    r#"<g class="eventWrapper" transform="translate({x}, {y})">"#,
                    x = fmt(ev.x),
                    y = fmt(ev.y)
                );
                render_node(
                    &mut out,
                    diagram_id,
                    &mut node_count,
                    ev,
                    is_redux_theme,
                    true,
                );
                out.push_str("</g>");
            }
        }
    }

    for task in &layout.orphan_tasks {
        let task_node = &task.node;
        let _ = write!(
            &mut out,
            r#"<g class="taskWrapper" transform="translate({x}, {y})">"#,
            x = fmt(task_node.x),
            y = fmt(task_node.y)
        );
        render_node(
            &mut out,
            diagram_id,
            &mut node_count,
            task_node,
            is_redux_theme,
            false,
        );
        out.push_str("</g>");

        let _ = write!(
            &mut out,
            r#"<g class="lineWrapper"><line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke-width="2" stroke="black" marker-end="{marker_end}" stroke-dasharray="5,5"/></g>"#,
            x1 = fmt(task.connector.x1),
            y1 = fmt(task.connector.y1),
            x2 = fmt(task.connector.x2),
            y2 = fmt(task.connector.y2),
            marker_end = escape_attr(&arrowhead_url),
        );

        for ev in &task.events {
            let _ = write!(
                &mut out,
                r#"<g class="eventWrapper" transform="translate({x}, {y})">"#,
                x = fmt(ev.x),
                y = fmt(ev.y)
            );
            render_node(
                &mut out,
                diagram_id,
                &mut node_count,
                ev,
                is_redux_theme,
                true,
            );
            out.push_str("</g>");
        }
    }

    if let Some(title) = layout.title.as_deref().filter(|t| !t.trim().is_empty()) {
        let _ = write!(
            &mut out,
            r#"<text x="{x}" font-size="4ex" font-weight="bold" y="{y}">{text}</text>"#,
            x = fmt(layout.title_x),
            y = fmt(layout.title_y),
            text = escape_xml(title)
        );
    }

    let _ = write!(
        &mut out,
        r#"<g class="lineWrapper"><line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke-width="4" stroke="black" marker-end="{marker_end}"/></g>"#,
        x1 = fmt(layout.activity_line.x1),
        y1 = fmt(layout.activity_line.y1),
        x2 = fmt(layout.activity_line.x2),
        y2 = fmt(layout.activity_line.y2),
        marker_end = escape_attr(&arrowhead_url),
    );

    out.push_str("</svg>\n");
    Ok(out)
}
