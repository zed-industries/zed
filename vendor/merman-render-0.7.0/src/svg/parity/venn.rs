use super::theme::VennTheme;
use super::*;
use merman_core::diagrams::venn::VennDiagramRenderModel;
use std::collections::{BTreeMap, HashMap};

fn stable_sets_key(sets: &[String]) -> String {
    sets.join("|")
}

fn escape_css_attr(value: &str) -> String {
    escape_attr(value)
}

fn data_sets_attr(sets: &[String]) -> String {
    sets.join("_")
}

fn build_style_by_key(model: &VennDiagramRenderModel) -> HashMap<String, BTreeMap<String, String>> {
    let mut out = HashMap::new();
    for entry in &model.style_entries {
        let key = stable_sets_key(&entry.targets);
        out.entry(key)
            .or_insert_with(BTreeMap::new)
            .extend(entry.styles.clone());
    }
    out
}

fn style_value<'a>(styles: Option<&'a BTreeMap<String, String>>, key: &str) -> Option<&'a str> {
    styles
        .and_then(|styles| styles.get(key))
        .map(String::as_str)
        .filter(|value| !value.trim().is_empty())
}

fn render_label(area: &crate::model::VennAreaLayout) -> &str {
    if let Some(label) = area.label.as_deref().filter(|label| !label.is_empty()) {
        label
    } else if area.sets.len() == 1 {
        area.sets[0].as_str()
    } else {
        ""
    }
}

fn root_open(
    out: &mut String,
    diagram_id: &str,
    layout: &VennDiagramLayout,
    aria_labelledby: Option<&str>,
    aria_describedby: Option<&str>,
) {
    let width_attr = fmt_string(layout.width.max(1.0));
    let height_attr = fmt_string(layout.height.max(1.0));
    let viewbox_attr = format!("0 0 {width_attr} {height_attr}");
    if layout.use_max_width {
        let style_attr = format!("max-width: {width_attr}px; background-color: white;");
        root_svg::push_svg_root_open(
            out,
            root_svg::SvgRootAttrs {
                width: root_svg::SvgRootWidth::Percent100,
                style_attr: Some(style_attr.as_str()),
                viewbox_attr: Some(viewbox_attr.as_str()),
                aria_labelledby,
                aria_describedby,
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, "venn")
            },
        );
    } else {
        let tail_attrs: [(&str, &str); 1] = [("style", "background-color: white;")];
        root_svg::push_svg_root_open(
            out,
            root_svg::SvgRootAttrs {
                width: root_svg::SvgRootWidth::Fixed(&width_attr),
                height_attr: Some(&height_attr),
                viewbox_attr: Some(viewbox_attr.as_str()),
                tail_attrs: &tail_attrs,
                fixed_height_placement: root_svg::SvgRootFixedHeightPlacement::AfterXmlns,
                aria_labelledby,
                aria_describedby,
                trailing_newline: false,
                ..root_svg::SvgRootAttrs::new(diagram_id, "venn")
            },
        );
    }
}

fn venn_css(diagram_id: &str, theme: &VennTheme) -> String {
    let id = escape_xml(diagram_id);
    format!(
        "#{id} .venn-title{{font-size:32px;fill:{title_color};font-family:{font_family};}}\
#{id} .venn-circle text{{font-size:48px;font-family:{font_family};}}\
#{id} .venn-intersection text{{font-size:48px;fill:{set_text_color};font-family:{font_family};}}\
#{id} .venn-text-node{{font-family:{font_family};color:{set_text_color};}}",
        title_color = theme.title_color,
        font_family = theme.font_family_css,
        set_text_color = theme.set_text_color,
    )
}

pub(super) fn render_venn_diagram_svg(
    layout: &VennDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    let model: VennDiagramRenderModel = crate::json::from_value_ref(semantic)?;
    render_venn_diagram_svg_model(layout, &model, effective_config, diagram_title, options)
}

pub(super) fn render_venn_diagram_svg_model(
    layout: &VennDiagramLayout,
    model: &VennDiagramRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    options: &SvgRenderOptions,
) -> Result<String> {
    let diagram_id = options.diagram_id.as_deref().unwrap_or("venn");
    let diagram_id_esc = escape_xml(diagram_id);
    let title = model
        .title
        .as_deref()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .or_else(|| {
            diagram_title
                .map(str::trim)
                .filter(|title| !title.is_empty())
        });
    let has_acc_title = model
        .acc_title
        .as_deref()
        .is_some_and(|title| !title.trim().is_empty());
    let has_acc_descr = model
        .acc_descr
        .as_deref()
        .is_some_and(|descr| !descr.trim().is_empty());
    let aria_labelledby = has_acc_title.then(|| format!("chart-title-{diagram_id_esc}"));
    let aria_describedby = has_acc_descr.then(|| format!("chart-desc-{diagram_id_esc}"));

    let mut out = String::new();
    root_open(
        &mut out,
        diagram_id,
        layout,
        aria_labelledby.as_deref(),
        aria_describedby.as_deref(),
    );

    if has_acc_title {
        let _ = write!(
            &mut out,
            r#"<title id="chart-title-{id}">{text}</title>"#,
            id = diagram_id_esc,
            text = escape_xml(model.acc_title.as_deref().unwrap_or_default())
        );
    }
    if has_acc_descr {
        let _ = write!(
            &mut out,
            r#"<desc id="chart-desc-{id}">{text}</desc>"#,
            id = diagram_id_esc,
            text = escape_xml(model.acc_descr.as_deref().unwrap_or_default())
        );
    }

    let theme = PresentationTheme::new(effective_config).venn();
    let css = venn_css(diagram_id, &theme);
    let _ = write!(&mut out, r#"<style>{css}</style>"#);
    out.push_str("<g/>");

    if let Some(title) = title {
        let _ = write!(
            &mut out,
            r#"<text class="venn-title" font-size="{font_size}px" text-anchor="middle" dominant-baseline="middle" x="50%" y="{y}" style="fill: {fill};">{text}</text>"#,
            font_size = fmt(32.0 * layout.scale),
            y = fmt(32.0 * layout.scale),
            fill = escape_xml(&theme.title_color),
            text = escape_xml(title)
        );
    }

    let _ = write!(
        &mut out,
        r#"<g transform="translate(0, {title_height})">"#,
        title_height = fmt(layout.title_height)
    );

    let style_by_key = build_style_by_key(model);
    let mut circle_index = 0usize;

    for area in &layout.areas {
        let sets_key = stable_sets_key(&area.sets);
        let styles = style_by_key.get(&sets_key);
        if area.sets.len() == 1 {
            let base_color = style_value(styles, "fill")
                .map(str::to_string)
                .unwrap_or_else(|| {
                    theme
                        .circle_colors
                        .get(circle_index % theme.circle_colors.len().max(1))
                        .cloned()
                        .unwrap_or_else(|| theme.primary_color.clone())
                });
            let fill_opacity = style_value(styles, "fill-opacity").unwrap_or("0.1");
            let stroke_color = style_value(styles, "stroke").unwrap_or(base_color.as_str());
            let stroke_width = style_value(styles, "stroke-width")
                .map(str::to_string)
                .unwrap_or_else(|| fmt_string(5.0 * layout.scale));
            let text_color = style_value(styles, "color")
                .map(str::to_string)
                .unwrap_or_else(|| theme.circle_text_color(&base_color));
            let _ = write!(
                &mut out,
                r#"<g class="venn-area venn-circle venn-set-{set_class}" data-venn-sets="{sets}"><path d="{path}" style="fill: {fill}; fill-opacity: {fill_opacity}; stroke: {stroke}; stroke-width: {stroke_width}; stroke-opacity: 0.95;"/><text class="label" text-anchor="middle" dy=".35em" x="{x}" y="{y}" style="font-size: {font_size}px; fill: {text_fill};"><tspan x="{x}" y="{y}" dy="0.35em">{label}</tspan></text></g>"#,
                set_class = circle_index % 8,
                sets = escape_attr(&data_sets_attr(&area.sets)),
                path = escape_attr(&area.path),
                fill = escape_css_attr(&base_color),
                fill_opacity = escape_css_attr(fill_opacity),
                stroke = escape_css_attr(stroke_color),
                stroke_width = escape_css_attr(&stroke_width),
                x = fmt(area.text_x),
                y = fmt(area.text_y),
                font_size = fmt(48.0 * layout.scale),
                text_fill = escape_css_attr(&text_color),
                label = escape_xml(render_label(area))
            );
            circle_index += 1;
        } else {
            let fill = style_value(styles, "fill").unwrap_or("transparent");
            let fill_opacity = if style_value(styles, "fill").is_some() {
                "1"
            } else {
                "0"
            };
            let text_color = style_value(styles, "color").unwrap_or(theme.set_text_color.as_str());
            let _ = write!(
                &mut out,
                r#"<g class="venn-area venn-intersection" data-venn-sets="{sets}"><path d="{path}" style="fill-opacity: {fill_opacity}; fill: {fill};"/><text class="label" text-anchor="middle" dy=".35em" x="{x}" y="{y}" style="font-size: {font_size}px; fill: {text_fill};"><tspan x="{x}" y="{y}" dy="0.35em">{label}</tspan></text></g>"#,
                sets = escape_attr(&data_sets_attr(&area.sets)),
                path = escape_attr(&area.path),
                fill_opacity = fill_opacity,
                fill = escape_css_attr(fill),
                x = fmt(area.text_x),
                y = fmt(area.text_y),
                font_size = fmt(48.0 * layout.scale),
                text_fill = escape_css_attr(text_color),
                label = escape_xml(render_label(area))
            );
        }
    }

    if !layout.text_areas.is_empty() {
        let mut nodes_by_key: HashMap<String, Vec<&crate::model::VennTextNodeLayout>> =
            HashMap::new();
        for node in &layout.text_nodes {
            nodes_by_key
                .entry(stable_sets_key(&node.sets))
                .or_default()
                .push(node);
        }

        out.push_str(r#"<g class="venn-text-nodes">"#);
        for text_area in &layout.text_areas {
            let key = stable_sets_key(&text_area.sets);
            let nodes = nodes_by_key.get(&key).map(Vec::as_slice).unwrap_or(&[]);
            let _ = write!(
                &mut out,
                r#"<g class="venn-text-area" font-size="{font_size}px">"#,
                font_size = fmt(text_area.font_size)
            );
            if layout.use_debug_layout {
                let _ = write!(
                    &mut out,
                    r#"<circle class="venn-text-debug-circle" cx="{cx}" cy="{cy}" r="{r}" fill="none" stroke="purple" stroke-width="{stroke_width}" stroke-dasharray="{dash} {gap}"/>"#,
                    cx = fmt(text_area.center_x),
                    cy = fmt(text_area.center_y),
                    r = fmt(text_area.inner_radius),
                    stroke_width = fmt(1.5 * layout.scale),
                    dash = fmt(6.0 * layout.scale),
                    gap = fmt(4.0 * layout.scale)
                );
                for cell in &text_area.debug_cells {
                    let _ = write!(
                        &mut out,
                        r#"<rect class="venn-text-debug-cell" x="{x}" y="{y}" width="{width}" height="{height}" fill="none" stroke="teal" stroke-width="{stroke_width}" stroke-dasharray="{dash} {gap}"/>"#,
                        x = fmt(cell.x),
                        y = fmt(cell.y),
                        width = fmt(cell.width),
                        height = fmt(cell.height),
                        stroke_width = fmt(layout.scale),
                        dash = fmt(4.0 * layout.scale),
                        gap = fmt(3.0 * layout.scale)
                    );
                }
            }

            for node in nodes {
                let text_color = style_by_key
                    .get(&node.id)
                    .and_then(|styles| style_value(Some(styles), "color"));
                let mut span_style = "display: flex; width: 100%; height: 100%; white-space: normal; align-items: center; justify-content: center; text-align: center; overflow-wrap: normal; word-break: normal;".to_string();
                if let Some(text_color) = text_color {
                    span_style.push_str(" color: ");
                    span_style.push_str(text_color);
                    span_style.push(';');
                }
                let label = node.label.as_deref().unwrap_or(node.id.as_str());
                let _ = write!(
                    &mut out,
                    r#"<foreignObject class="venn-text-node-fo" width="{width}" height="{height}" x="{x}" y="{y}" overflow="visible"><span xmlns="http://www.w3.org/1999/xhtml" class="venn-text-node" style="{style}">{label}</span></foreignObject>"#,
                    width = fmt(node.width),
                    height = fmt(node.height),
                    x = fmt(node.x),
                    y = fmt(node.y),
                    style = escape_attr(&span_style),
                    label = escape_xml(label)
                );
            }
            out.push_str("</g>");
        }
        out.push_str("</g>");
    }

    out.push_str("</g></svg>\n");
    Ok(out)
}
