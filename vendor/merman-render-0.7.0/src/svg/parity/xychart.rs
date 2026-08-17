use super::*;

// XYChart diagram SVG renderer implementation (split from parity.rs).

pub(super) fn render_xychart_diagram_svg(
    layout: &XyChartDiagramLayout,
    _semantic: &serde_json::Value,
    _effective_config: &serde_json::Value,
    options: &SvgRenderOptions,
) -> Result<String> {
    use std::collections::{HashMap, hash_map::Entry};

    struct Node {
        tag: &'static str,
        attrs: Vec<(&'static str, String)>,
        text: Option<String>,
        children: Vec<usize>,
    }

    impl Node {
        fn attr(&mut self, name: &'static str, value: impl Into<String>) {
            self.attrs.push((name, value.into()));
        }
    }

    fn node(tag: &'static str) -> Node {
        Node {
            tag,
            attrs: Vec::new(),
            text: None,
            children: Vec::new(),
        }
    }

    fn push_child(arena: &mut Vec<Node>, parent: usize, child: Node) -> usize {
        let id = arena.len();
        arena.push(child);
        arena[parent].children.push(id);
        id
    }

    fn render_node(out: &mut String, arena: &[Node], id: usize) {
        let n = &arena[id];
        out.push('<');
        out.push_str(n.tag);
        for (k, v) in &n.attrs {
            let _ = write!(out, r#" {k}="{v}""#);
        }
        if n.children.is_empty() && n.text.as_deref().unwrap_or("").is_empty() {
            out.push_str("/>");
            return;
        }
        out.push('>');
        if let Some(t) = n.text.as_deref() {
            out.push_str(t);
        }
        for c in &n.children {
            render_node(out, arena, *c);
        }
        let _ = write!(out, "</{}>", n.tag);
    }

    fn text_anchor(horizontal_pos: &str) -> &'static str {
        match horizontal_pos {
            "left" => "start",
            "right" => "end",
            _ => "middle",
        }
    }

    fn ensure_group_path(
        arena: &mut Vec<Node>,
        groups_by_path: &mut HashMap<(usize, String), usize>,
        group_texts: &[String],
    ) -> usize {
        let mut parent = 0usize;
        for seg in group_texts {
            let gid = match groups_by_path.entry((parent, seg.clone())) {
                Entry::Occupied(entry) => *entry.get(),
                Entry::Vacant(entry) => {
                    let mut g = node("g");
                    g.attr("class", seg.as_str());
                    let id = push_child(arena, parent, g);
                    entry.insert(id);
                    id
                }
            };
            parent = gid;
        }
        parent
    }

    fn dominant_baseline(vertical_pos: &str) -> &'static str {
        if vertical_pos == "top" {
            "text-before-edge"
        } else {
            "middle"
        }
    }

    fn fmt_xy(v: f64) -> String {
        if v.is_nan() {
            return "NaN".to_string();
        }
        if !v.is_finite() {
            return "NaN".to_string();
        }
        fmt_string(v)
    }

    fn data_label_color(effective_config: &serde_json::Value) -> String {
        let configured = config_string(
            effective_config,
            &["themeVariables", "xyChart", "dataLabelColor"],
        );
        configured
            .or_else(|| config_string(effective_config, &["themeVariables", "primaryTextColor"]))
            .unwrap_or_else(|| "black".to_string())
    }

    let diagram_id = options.diagram_id.as_deref().unwrap_or("xychart");
    let show_data_label_outside_bar =
        config_bool(_effective_config, &["xyChart", "showDataLabelOutsideBar"]).unwrap_or(false);
    let data_label_color = data_label_color(_effective_config);

    let mut out = String::new();
    let w_attr = fmt(layout.width.max(1.0)).to_string();
    let h_attr = fmt(layout.height.max(1.0)).to_string();
    let viewbox_attr = format!("0 0 {w_attr} {h_attr}");
    let style_attr = format!("max-width: {w_attr}px; background-color: white;");
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(viewbox_attr.as_str()),
            style_viewbox_order: root_svg::SvgRootStyleViewBoxOrder::ViewBoxThenStyle,
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "xychart")
        },
    );

    out.push_str("<style>");
    push_xychart_css(&mut out, diagram_id);
    out.push_str("</style>");

    // Mermaid always includes an empty `<g/>` placeholder after `<style>`.
    out.push_str(r#"<g/>"#);

    // Build the `.main` group as an ordered DOM tree, matching Mermaid's D3 `getGroup()` behavior.
    let mut arena: Vec<Node> = Vec::new();
    arena.push(node("g"));
    arena[0].attr("class", "main");

    // Background rectangle.
    let mut bg = node("rect");
    bg.attr("width", fmt_xy(layout.width));
    bg.attr("height", fmt_xy(layout.height));
    bg.attr("class", "background");
    bg.attr("fill", escape_xml(&layout.background_color));
    push_child(&mut arena, 0, bg);

    let mut groups_by_path: HashMap<(usize, String), usize> = HashMap::new();

    for shape in &layout.drawables {
        match shape {
            crate::model::XyChartDrawableElem::Rect { group_texts, data } => {
                if data.is_empty() {
                    continue;
                }
                let parent = ensure_group_path(&mut arena, &mut groups_by_path, group_texts);

                // Append rect elements.
                for r in data {
                    let mut n = node("rect");
                    n.attr("x", fmt_xy(r.x));
                    if !r.y.is_nan() {
                        n.attr("y", fmt_xy(r.y));
                    }
                    n.attr("width", fmt_xy(r.width));
                    n.attr("height", fmt_xy(r.height));
                    n.attr("fill", escape_xml(&r.fill));
                    n.attr("stroke", escape_xml(&r.stroke_fill));
                    n.attr("stroke-width", fmt_xy(r.stroke_width));
                    push_child(&mut arena, parent, n);
                }

                // Optional bar data labels (Mermaid emits these in the renderer, not the DB).
                if layout.show_data_label {
                    let bar_data_label_scale_factor = 0.7;
                    let bar_data_label_inset_px = 10.0;

                    #[derive(Clone)]
                    struct BarItem<'a> {
                        rect: &'a crate::model::XyChartRectData,
                        label: &'a str,
                    }

                    let mut valid_items: Vec<BarItem<'_>> = Vec::new();
                    for (idx, r) in data.iter().enumerate() {
                        let Some(label) = layout.label_data.get(idx) else {
                            continue;
                        };
                        if r.width > 0.0 && r.height > 0.0 {
                            valid_items.push(BarItem { rect: r, label });
                        }
                    }

                    if !valid_items.is_empty() {
                        if layout.chart_orientation == "horizontal" {
                            fn fits(
                                item: &BarItem<'_>,
                                font_size: f64,
                                char_width_factor: f64,
                                inset_px: f64,
                            ) -> bool {
                                let text_w = font_size
                                    * (item.label.chars().count() as f64)
                                    * char_width_factor;
                                text_w <= item.rect.width - inset_px
                            }

                            let mut min_font = f64::INFINITY;
                            for item in &valid_items {
                                let mut fs = item.rect.height * bar_data_label_scale_factor;
                                while !fits(
                                    item,
                                    fs,
                                    bar_data_label_scale_factor,
                                    bar_data_label_inset_px,
                                ) && fs > 0.0
                                {
                                    fs -= 1.0;
                                }
                                min_font = min_font.min(fs);
                            }
                            let uniform = min_font.floor().max(0.0);
                            for item in &valid_items {
                                let mut t = node("text");
                                let x = if show_data_label_outside_bar {
                                    item.rect.x + item.rect.width + bar_data_label_inset_px
                                } else {
                                    item.rect.x + item.rect.width - bar_data_label_inset_px
                                };
                                t.attr("x", fmt_xy(x));
                                t.attr("y", fmt_xy(item.rect.y + item.rect.height / 2.0));
                                t.attr(
                                    "text-anchor",
                                    if show_data_label_outside_bar {
                                        "start"
                                    } else {
                                        "end"
                                    },
                                );
                                t.attr("dominant-baseline", "middle");
                                t.attr("fill", escape_xml(&data_label_color));
                                t.attr("font-size", format!("{}px", fmt_xy(uniform)));
                                t.text = Some(escape_xml(item.label));
                                push_child(&mut arena, parent, t);
                            }
                        } else {
                            let y_offset = bar_data_label_inset_px;
                            fn fits(
                                item: &BarItem<'_>,
                                font_size: f64,
                                char_width_factor: f64,
                                y_offset: f64,
                            ) -> bool {
                                let text_w = font_size
                                    * (item.label.chars().count() as f64)
                                    * char_width_factor;
                                let center_x = item.rect.x + item.rect.width / 2.0;
                                let left = center_x - text_w / 2.0;
                                let right = center_x + text_w / 2.0;
                                let horizontal =
                                    left >= item.rect.x && right <= item.rect.x + item.rect.width;
                                let vertical = item.rect.y + y_offset + font_size
                                    <= item.rect.y + item.rect.height;
                                horizontal && vertical
                            }

                            let mut min_font = f64::INFINITY;
                            for item in &valid_items {
                                let denom = (item.label.chars().count() as f64)
                                    * bar_data_label_scale_factor;
                                let mut fs = if denom <= 0.0 {
                                    0.0
                                } else {
                                    item.rect.width / denom
                                };
                                while !fits(item, fs, bar_data_label_scale_factor, y_offset)
                                    && fs > 0.0
                                {
                                    fs -= 1.0;
                                }
                                min_font = min_font.min(fs);
                            }
                            let uniform = min_font.floor().max(0.0);
                            for item in &valid_items {
                                let mut t = node("text");
                                t.attr("x", fmt_xy(item.rect.x + item.rect.width / 2.0));
                                let y = if show_data_label_outside_bar {
                                    item.rect.y - y_offset
                                } else {
                                    item.rect.y + y_offset
                                };
                                t.attr("y", fmt_xy(y));
                                t.attr("text-anchor", "middle");
                                t.attr(
                                    "dominant-baseline",
                                    if show_data_label_outside_bar {
                                        "auto"
                                    } else {
                                        "hanging"
                                    },
                                );
                                t.attr("fill", escape_xml(&data_label_color));
                                t.attr("font-size", format!("{}px", fmt_xy(uniform)));
                                t.text = Some(escape_xml(item.label));
                                push_child(&mut arena, parent, t);
                            }
                        }
                    }
                }
            }
            crate::model::XyChartDrawableElem::Text { group_texts, data } => {
                if data.is_empty() {
                    continue;
                }
                let parent = ensure_group_path(&mut arena, &mut groups_by_path, group_texts);

                for t in data {
                    let mut n = node("text");
                    n.attr("x", "0");
                    n.attr("y", "0");
                    n.attr("fill", escape_xml(&t.fill));
                    n.attr("font-size", fmt_string(t.font_size));
                    n.attr("dominant-baseline", dominant_baseline(&t.vertical_pos));
                    n.attr("text-anchor", text_anchor(&t.horizontal_pos));
                    let rot = t.rotation;
                    n.attr(
                        "transform",
                        format!(
                            "translate({}, {}) rotate({})",
                            fmt_xy(t.x),
                            fmt_xy(t.y),
                            fmt_xy(rot)
                        ),
                    );
                    n.text = Some(escape_xml(&t.text));
                    push_child(&mut arena, parent, n);
                }
            }
            crate::model::XyChartDrawableElem::Path { group_texts, data } => {
                if data.is_empty() {
                    continue;
                }
                let parent = ensure_group_path(&mut arena, &mut groups_by_path, group_texts);

                for p in data {
                    let mut n = node("path");
                    n.attr("d", escape_xml(&p.path));
                    n.attr("fill", escape_xml(p.fill.as_deref().unwrap_or("none")));
                    n.attr("stroke", escape_xml(&p.stroke_fill));
                    n.attr("stroke-width", fmt_xy(p.stroke_width));
                    push_child(&mut arena, parent, n);
                }
            }
        }
    }

    render_node(&mut out, &arena, 0);
    out.push_str(r#"<g class="mermaid-tmp-group"/>"#);
    out.push_str("</svg>\n");
    Ok(out)
}
