use super::*;

struct GitGraphCss {
    css: String,
    defs: String,
    font_family: String,
    commit_label_font_size_px: f64,
    tag_label_font_size_px: f64,
}

const GITGRAPH_NAMED_COLOR_COUNT: usize = 8;

fn gitgraph_theme_name(effective_config: &serde_json::Value) -> String {
    config_string(effective_config, &["theme"]).unwrap_or_else(|| "default".to_string())
}

fn gitgraph_theme_is_redux_geometry(theme: &str) -> bool {
    matches!(
        theme,
        "redux" | "redux-dark" | "redux-color" | "redux-dark-color"
    )
}

fn gitgraph_theme_is_color(theme: &str) -> bool {
    matches!(theme, "redux-color" | "redux-dark-color")
}

fn gitgraph_theme_is_neo(theme: &str) -> bool {
    matches!(theme, "neo" | "neo-dark")
}

fn gitgraph_theme_is_dark(theme: &str) -> bool {
    matches!(
        theme,
        "dark" | "redux-dark" | "redux-dark-color" | "neo-dark"
    )
}

fn gitgraph_theme_uses_color_gen(theme: &str) -> bool {
    matches!(
        theme,
        "redux" | "redux-dark" | "redux-color" | "redux-dark-color" | "neo" | "neo-dark"
    )
}

fn gitgraph_theme_array(effective_config: &serde_json::Value, key: &str) -> Vec<String> {
    effective_config
        .get("themeVariables")
        .and_then(|v| v.get(key))
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn gitgraph_gradient_defs(diagram_id: &str, effective_config: &serde_json::Value) -> String {
    if !config_bool(effective_config, &["themeVariables", "useGradient"]).unwrap_or(false) {
        return String::new();
    }

    let gradient_start = config_string(effective_config, &["themeVariables", "gradientStart"])
        .or_else(|| config_string(effective_config, &["themeVariables", "primaryBorderColor"]))
        .unwrap_or_else(|| "#9370DB".to_string());
    let gradient_stop = config_string(effective_config, &["themeVariables", "gradientStop"])
        .or_else(|| {
            config_string(
                effective_config,
                &["themeVariables", "secondaryBorderColor"],
            )
        })
        .unwrap_or_else(|| gradient_start.clone());

    format!(
        r#"<defs><linearGradient id="{}-gradient" gradientUnits="objectBoundingBox" x1="0%" y1="0%" x2="100%" y2="0%"><stop offset="0%" stop-color="{}" stop-opacity="1"/><stop offset="100%" stop-color="{}" stop-opacity="1"/></linearGradient></defs>"#,
        escape_xml(diagram_id),
        escape_xml(&gradient_start),
        escape_xml(&gradient_stop)
    )
}

fn gitgraph_css(diagram_id: &str, effective_config: &serde_json::Value) -> GitGraphCss {
    let id = escape_xml(diagram_id);
    let parts = info_css_parts_with_theme_font_size_only(diagram_id, effective_config);
    let font_family = parts.font_family.clone();
    let theme_name = gitgraph_theme_name(effective_config);
    let use_redux_geometry = gitgraph_theme_is_redux_geometry(&theme_name);
    let use_color_theme = gitgraph_theme_is_color(&theme_name);
    let use_neo_theme = gitgraph_theme_is_neo(&theme_name);
    let use_dark_theme = gitgraph_theme_is_dark(&theme_name);
    let use_color_gen = gitgraph_theme_uses_color_gen(&theme_name);

    fn default_git_color(i: usize) -> &'static str {
        match i {
            0 => "hsl(240, 100%, 46.2745098039%)",
            1 => "hsl(60, 100%, 43.5294117647%)",
            2 => "hsl(80, 100%, 46.2745098039%)",
            3 => "hsl(210, 100%, 46.2745098039%)",
            4 => "hsl(180, 100%, 46.2745098039%)",
            5 => "hsl(150, 100%, 46.2745098039%)",
            6 => "hsl(300, 100%, 46.2745098039%)",
            _ => "hsl(0, 100%, 46.2745098039%)",
        }
    }

    fn default_git_branch_label(i: usize) -> &'static str {
        match i {
            0 | 3 => "#ffffff",
            _ => "black",
        }
    }

    fn default_git_inv(i: usize) -> &'static str {
        match i {
            0 => "hsl(60, 100%, 3.7254901961%)",
            1 => "rgb(0, 0, 160.5)",
            2 => "rgb(48.8333333334, 0, 146.5000000001)",
            3 => "rgb(146.5000000001, 73.2500000001, 0)",
            4 => "rgb(146.5000000001, 0, 0)",
            5 => "rgb(146.5000000001, 0, 73.2500000001)",
            6 => "rgb(0, 146.5000000001, 0)",
            _ => "rgb(0, 146.5000000001, 146.5000000001)",
        }
    }

    let commit_label_font_size =
        config_string(effective_config, &["themeVariables", "commitLabelFontSize"])
            .unwrap_or_else(|| "10px".to_string());
    let tag_label_font_size =
        config_string(effective_config, &["themeVariables", "tagLabelFontSize"])
            .unwrap_or_else(|| "10px".to_string());
    let commit_label_font_size_px = parse_gitgraph_label_font_size_px(&commit_label_font_size);
    let tag_label_font_size_px = parse_gitgraph_label_font_size_px(&tag_label_font_size);
    let commit_label_color = theme_color(effective_config, "commitLabelColor", "#000021");
    let commit_label_background = theme_color(effective_config, "commitLabelBackground", "#ffffde");
    let tag_label_color = theme_color(effective_config, "tagLabelColor", "#131300");
    let tag_label_background = theme_color(effective_config, "tagLabelBackground", "#ECECFF");
    let tag_label_border = theme_color(
        effective_config,
        "tagLabelBorder",
        "hsl(240, 60%, 86.2745098039%)",
    );
    let theme_color_limit = config_f64(effective_config, &["themeVariables", "THEME_COLOR_LIMIT"])
        .map(|value| value.max(1.0).min(64.0) as usize)
        .unwrap_or(12);
    let stroke_width = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "strokeWidth"],
    )
    .unwrap_or_else(|| "1".to_string());
    let commit_line_color = config_string(effective_config, &["themeVariables", "commitLineColor"])
        .unwrap_or_else(|| parts.line_color.clone());
    let primary_color = theme_color(effective_config, "primaryColor", "#ECECFF");
    let node_border = theme_color(effective_config, "nodeBorder", "#9370DB");
    let main_bkg = theme_color(effective_config, "mainBkg", "#ECECFF");
    let note_font_weight = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "noteFontWeight"],
    )
    .unwrap_or_else(|| "normal".to_string());
    let note_font_weight_decl = if use_redux_geometry {
        format!("font-weight:{};", note_font_weight)
    } else {
        String::new()
    };
    let drop_shadow = crate::config::config_css_number_or_string(
        effective_config,
        &["themeVariables", "dropShadow"],
    )
    .unwrap_or_else(|| "none".to_string());
    let use_gradient =
        config_bool(effective_config, &["themeVariables", "useGradient"]).unwrap_or(false);
    let border_color_array = gitgraph_theme_array(effective_config, "borderColorArray");
    let defs = if use_neo_theme {
        gitgraph_gradient_defs(diagram_id, effective_config)
    } else {
        String::new()
    };
    let mut out = parts.css_prefix;
    let _ = write!(
        &mut out,
        r#"#{} .commit-id,#{} .commit-msg,#{} .branch-label{{fill:lightgrey;color:lightgrey;font-family:'trebuchet ms',verdana,arial,sans-serif;font-family:var(--mermaid-font-family);}}"#,
        id, id, id
    );
    for i in 0..theme_color_limit {
        let ci = i % GITGRAPH_NAMED_COLOR_COUNT;
        if use_color_gen {
            if use_neo_theme {
                if i == 0 {
                    let _ = write!(
                        &mut out,
                        r#"#{} .branch-label{}{{fill:{};}}#{} .commit{}{{stroke:{};}}#{} .commit-highlight{}{{stroke:{};fill:{};}}#{} .arrow{}{{stroke:{};}}#{} .commit-bullets{{fill:{};}}#{} .commit-cherry-pick{}{{stroke:{};}}"#,
                        id,
                        i,
                        node_border,
                        id,
                        i,
                        node_border,
                        id,
                        i,
                        node_border,
                        node_border,
                        id,
                        i,
                        node_border,
                        id,
                        node_border,
                        id,
                        i,
                        node_border
                    );
                    if use_gradient {
                        for label_i in 0..theme_color_limit {
                            let _ = write!(
                                &mut out,
                                r#"#{} .label{}{{fill:{};stroke:url(#{}-gradient);stroke-width:{};}}"#,
                                id, label_i, main_bkg, id, stroke_width
                            );
                        }
                    }
                } else {
                    let git =
                        theme_color(effective_config, &format!("git{ci}"), default_git_color(ci));
                    let branch_label = theme_color(
                        effective_config,
                        &format!("gitBranchLabel{ci}"),
                        default_git_branch_label(ci),
                    );
                    let git_inv = theme_color(
                        effective_config,
                        &format!("gitInv{ci}"),
                        default_git_inv(ci),
                    );
                    let _ = write!(
                        &mut out,
                        r#"#{} .branch-label{}{{fill:{};}}#{} .commit{}{{stroke:{};fill:{};}}#{} .commit-highlight{}{{stroke:{};fill:{};}}#{} .arrow{}{{stroke:{};}}"#,
                        id, i, branch_label, id, i, git, git, id, i, git_inv, git_inv, id, i, git
                    );
                }
            } else if !use_color_theme {
                let _ = write!(
                    &mut out,
                    r#"#{} .branch-label{}{{fill:{};{}}}#{} .commit{}{{stroke:{};}}#{} .commit-highlight{}{{stroke:{};fill:{};}}#{} .label{}{{fill:{};stroke:{};stroke-width:{};{}}}#{} .arrow{}{{stroke:{};}}#{} .commit-bullets{{fill:{};}}#{} .commit-cherry-pick{}{{stroke:{};}}"#,
                    id,
                    i,
                    node_border,
                    note_font_weight_decl,
                    id,
                    i,
                    node_border,
                    id,
                    i,
                    node_border,
                    node_border,
                    id,
                    i,
                    main_bkg,
                    node_border,
                    stroke_width,
                    note_font_weight_decl,
                    id,
                    i,
                    node_border,
                    id,
                    node_border,
                    id,
                    i,
                    node_border
                );
            } else if i == 0 {
                let _ = write!(
                    &mut out,
                    r#"#{} .branch-label{}{{fill:{};{}}}#{} .commit{}{{stroke:{};}}#{} .commit-highlight{}{{stroke:{};fill:{};}}#{} .label{}{{fill:{};stroke:{};stroke-width:{};{}}}#{} .arrow{}{{stroke:{};}}#{} .commit-bullets{{fill:{};}}"#,
                    id,
                    i,
                    node_border,
                    note_font_weight_decl,
                    id,
                    i,
                    node_border,
                    id,
                    i,
                    node_border,
                    main_bkg,
                    id,
                    i,
                    main_bkg,
                    node_border,
                    stroke_width,
                    note_font_weight_decl,
                    id,
                    i,
                    node_border,
                    id,
                    node_border
                );
            } else {
                let border_color = border_color_array
                    .get(i % border_color_array.len().max(1))
                    .cloned()
                    .unwrap_or_else(|| node_border.clone());
                let label_fill = if use_dark_theme {
                    main_bkg.as_str()
                } else {
                    border_color.as_str()
                };
                let _ = write!(
                    &mut out,
                    r#"#{} .branch-label{}{{fill:{};{}}}#{} .commit{}{{stroke:{};fill:{};}}#{} .commit-highlight{}{{stroke:{};fill:{};}}#{} .label{}{{fill:{};stroke:{};stroke-width:{};}}#{} .arrow{}{{stroke:{};}}"#,
                    id,
                    i,
                    node_border,
                    note_font_weight_decl,
                    id,
                    i,
                    border_color,
                    border_color,
                    id,
                    i,
                    border_color,
                    border_color,
                    id,
                    i,
                    label_fill,
                    border_color,
                    stroke_width,
                    id,
                    i,
                    border_color
                );
            }
        } else {
            let git = theme_color(effective_config, &format!("git{ci}"), default_git_color(ci));
            let branch_label = theme_color(
                effective_config,
                &format!("gitBranchLabel{ci}"),
                default_git_branch_label(ci),
            );
            let git_inv = theme_color(
                effective_config,
                &format!("gitInv{ci}"),
                default_git_inv(ci),
            );
            let _ = write!(
                &mut out,
                r#"#{} .branch-label{}{{fill:{};}}#{} .commit{}{{stroke:{};fill:{};}}#{} .commit-highlight{}{{stroke:{};fill:{};}}#{} .label{}{{fill:{};}}#{} .arrow{}{{stroke:{};}}"#,
                id,
                i,
                branch_label,
                id,
                i,
                git,
                git,
                id,
                i,
                git_inv,
                git_inv,
                id,
                i,
                git,
                id,
                i,
                git
            );
        }
    }
    let branch_dasharray = if use_color_gen { "4 2" } else { "2" };
    let commit_label_fill = if use_color_gen {
        node_border.as_str()
    } else {
        commit_label_color.as_str()
    };
    let commit_label_weight = if use_color_gen {
        format!("font-weight:{};", note_font_weight)
    } else {
        String::new()
    };
    let commit_label_bkg_fill = if use_color_gen {
        "transparent"
    } else {
        commit_label_background.as_str()
    };
    let commit_label_bkg_opacity = if use_color_gen { "" } else { "opacity:0.5;" };
    let tag_label_bkg_fill = if use_color_gen {
        main_bkg.as_str()
    } else {
        tag_label_background.as_str()
    };
    let tag_label_bkg_stroke = if use_color_gen {
        node_border.as_str()
    } else {
        tag_label_border.as_str()
    };
    let tag_label_bkg_filter = if use_color_gen {
        format!("filter:{};", drop_shadow)
    } else {
        String::new()
    };
    let state_fill = if use_color_gen {
        main_bkg.as_str()
    } else {
        primary_color.as_str()
    };
    let reverse_stroke_width = if use_color_gen {
        stroke_width.as_str()
    } else {
        "3"
    };
    let arrow_stroke_width = if use_redux_geometry {
        stroke_width.as_str()
    } else {
        "8"
    };
    let _ = write!(
        &mut out,
        r#"#{} .branch{{stroke-width:{};stroke:{};stroke-dasharray:{};}}#{} .arrow{{stroke-width:{};stroke-linecap:round;fill:none;}}#{} .commit-label{{font-size:{};fill:{};{}}}#{} .commit-label-bkg{{font-size:{};fill:{};{}}}#{} .tag-label{{font-size:{};fill:{};}}#{} .tag-label-bkg{{fill:{};stroke:{};{}}}#{} .tag-hole{{fill:{};}}#{} .commit-merge{{stroke:{};fill:{};}}#{} .commit-reverse{{stroke:{};fill:{};stroke-width:{};}}#{} .commit-highlight-outer{{}}#{} .commit-highlight-inner{{stroke:{};fill:{};}}#{} .gitTitleText{{text-anchor:middle;font-size:18px;fill:{};}}"#,
        id,
        stroke_width,
        commit_line_color,
        branch_dasharray,
        id,
        arrow_stroke_width,
        id,
        commit_label_font_size,
        commit_label_fill,
        commit_label_weight,
        id,
        commit_label_font_size,
        commit_label_bkg_fill,
        commit_label_bkg_opacity,
        id,
        tag_label_font_size,
        tag_label_color,
        id,
        tag_label_bkg_fill,
        tag_label_bkg_stroke,
        tag_label_bkg_filter,
        id,
        parts.text_color,
        id,
        state_fill,
        state_fill,
        id,
        state_fill,
        state_fill,
        reverse_stroke_width,
        id,
        id,
        state_fill,
        state_fill,
        id,
        parts.text_color
    );
    out.push_str(&parts.root_rule);
    GitGraphCss {
        css: out,
        defs,
        font_family,
        commit_label_font_size_px,
        tag_label_font_size_px,
    }
}

fn parse_gitgraph_label_font_size_px(raw: &str) -> f64 {
    let raw = raw.trim().trim_end_matches(';').trim();
    let raw = raw.trim_end_matches("!important").trim();
    raw.strip_suffix("px")
        .unwrap_or(raw)
        .trim()
        .parse::<f64>()
        .ok()
        .filter(|value| value.is_finite())
        .unwrap_or(10.0)
        .max(1.0)
}

pub(super) fn render_gitgraph_diagram_svg(
    layout: &crate::model::GitGraphDiagramLayout,
    semantic: &serde_json::Value,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let acc_title = semantic
        .get("accTitle")
        .and_then(|v| v.as_str())
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    let acc_descr = semantic
        .get("accDescr")
        .and_then(|v| v.as_str())
        .map(|s| s.trim_end_matches('\n'))
        .filter(|s| !s.is_empty());

    render_gitgraph_diagram_svg_with_accessibility(
        layout,
        acc_title,
        acc_descr,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

pub(super) fn render_gitgraph_diagram_svg_model(
    layout: &crate::model::GitGraphDiagramLayout,
    model: &merman_core::diagrams::git_graph::GitGraphRenderModel,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    let acc_title = model
        .acc_title
        .as_deref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());
    let acc_descr = model
        .acc_descr
        .as_deref()
        .map(|s| s.trim_end_matches('\n'))
        .filter(|s| !s.is_empty());

    render_gitgraph_diagram_svg_with_accessibility(
        layout,
        acc_title,
        acc_descr,
        effective_config,
        diagram_title,
        measurer,
        options,
    )
}

fn render_gitgraph_diagram_svg_with_accessibility(
    layout: &crate::model::GitGraphDiagramLayout,
    acc_title: Option<&str>,
    acc_descr: Option<&str>,
    effective_config: &serde_json::Value,
    diagram_title: Option<&str>,
    measurer: &dyn TextMeasurer,
    options: &SvgRenderOptions,
) -> Result<String> {
    const THEME_COLOR_LIMIT: i64 = 8;
    const PX: f64 = 4.0;
    const PY: f64 = 2.0;
    const VIEWBOX_PLACEHOLDER: &str = "__MERMAID_VIEWBOX__";
    const MAX_WIDTH_PLACEHOLDER: &str = "__MERMAID_MAX_WIDTH__";
    const TITLE_X_PLACEHOLDER: &str = "__MERMAID_GITGRAPH_TITLE_X__";
    const VIEWBOX_PADDING_PX: f64 = 8.0;
    const TITLE_FONT_SIZE_PX: f64 = 18.0;

    fn gitgraph_commit_tag_label_width_px(
        measurer: &dyn TextMeasurer,
        text: &str,
        style: &crate::text::TextStyle,
    ) -> f64 {
        crate::text::round_to_1_64_px(measurer.measure_svg_text_computed_length_px(text, style))
            .max(0.0)
    }

    fn gitgraph_commit_tag_label_height_px(
        measurer: &dyn TextMeasurer,
        text: &str,
        style: &crate::text::TextStyle,
    ) -> f64 {
        if text.trim_end().is_empty() {
            return 0.0;
        }
        if style.font_size <= 10.0 {
            return measurer
                .measure_svg_simple_text_bbox_height_px(text, style)
                .max(0.0);
        }
        crate::text::svg_wrapped_first_line_bbox_height_px(style).max(0.0)
    }

    fn include_gitgraph_branch_line_bounds(
        bounds: &mut Bounds,
        layout: &crate::model::GitGraphDiagramLayout,
    ) {
        if !layout.show_branches {
            return;
        }

        fn include_point(bounds: &mut Bounds, x: f64, y: f64) {
            bounds.min_x = bounds.min_x.min(x);
            bounds.min_y = bounds.min_y.min(y);
            bounds.max_x = bounds.max_x.max(x);
            bounds.max_y = bounds.max_y.max(y);
        }

        for branch in &layout.branches {
            match layout.direction.as_str() {
                "TB" => {
                    include_point(bounds, branch.pos, 30.0);
                    include_point(bounds, branch.pos, layout.max_pos);
                }
                "BT" => {
                    include_point(bounds, branch.pos, layout.max_pos);
                    include_point(bounds, branch.pos, 30.0);
                }
                _ => {
                    include_point(bounds, 0.0, branch.pos);
                    include_point(bounds, layout.max_pos, branch.pos);
                }
            }
        }
    }

    let diagram_id = options.diagram_id.as_deref().unwrap_or("merman");

    let bounds = layout.bounds.clone().unwrap_or(Bounds {
        min_x: 0.0,
        min_y: 0.0,
        max_x: 100.0,
        max_y: 100.0,
    });
    let vb_min_x = bounds.min_x;
    let vb_min_y = bounds.min_y;
    let vb_w = (bounds.max_x - bounds.min_x).max(1.0);
    let vb_h = (bounds.max_y - bounds.min_y).max(1.0);

    let aria_title_id = format!("chart-title-{diagram_id}");
    let aria_desc_id = format!("chart-desc-{diagram_id}");

    let mut out = String::new();
    let aria_describedby = acc_descr.is_some().then(|| escape_attr(&aria_desc_id));
    let aria_labelledby = acc_title.is_some().then(|| escape_attr(&aria_title_id));
    let style_attr = format!("max-width: {MAX_WIDTH_PLACEHOLDER}px; background-color: white;");
    root_svg::push_svg_root_open(
        &mut out,
        root_svg::SvgRootAttrs {
            width: root_svg::SvgRootWidth::Percent100,
            style_attr: Some(style_attr.as_str()),
            viewbox_attr: Some(VIEWBOX_PLACEHOLDER),
            aria_labelledby: aria_labelledby.as_deref(),
            aria_describedby: aria_describedby.as_deref(),
            trailing_newline: false,
            ..root_svg::SvgRootAttrs::new(diagram_id, "gitGraph")
        },
    );

    if let Some(t) = acc_title {
        let _ = write!(
            &mut out,
            r#"<title id="{}">{}</title>"#,
            escape_attr(&aria_title_id),
            escape_xml(t)
        );
    }
    if let Some(d) = acc_descr {
        let _ = write!(
            &mut out,
            r#"<desc id="{}">{}</desc>"#,
            escape_attr(&aria_desc_id),
            escape_xml(d)
        );
    }

    let css = gitgraph_css(diagram_id, effective_config);
    let title_style = crate::text::TextStyle {
        font_family: Some(css.font_family.clone()),
        font_size: TITLE_FONT_SIZE_PX,
        font_weight: None,
    };
    let _ = write!(&mut out, r#"<style>{}</style>"#, css.css);

    out.push_str(r#"<g/>"#);
    out.push_str(&css.defs);
    out.push_str(r#"<g class="commit-bullets"/>"#);
    out.push_str(r#"<g class="commit-labels"/>"#);

    let mut branch_idx: std::collections::HashMap<&str, i64> = std::collections::HashMap::new();
    for b in &layout.branches {
        branch_idx.insert(b.name.as_str(), b.index);
    }

    let direction = layout.direction.as_str();

    if layout.show_branches {
        out.push_str("<g>");
        for b in &layout.branches {
            let idx = b.index % THEME_COLOR_LIMIT;
            let pos = b.pos;

            if direction == "TB" {
                let _ = write!(
                    &mut out,
                    r#"<line x1="{x1}" y1="30" x2="{x2}" y2="{y2}" class="branch branch{idx}"/>"#,
                    x1 = fmt(pos),
                    x2 = fmt(pos),
                    y2 = fmt(layout.max_pos),
                    idx = idx
                );
            } else if direction == "BT" {
                let _ = write!(
                    &mut out,
                    r#"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="30" class="branch branch{idx}"/>"#,
                    x1 = fmt(pos),
                    y1 = fmt(layout.max_pos),
                    x2 = fmt(pos),
                    idx = idx
                );
            } else {
                let _ = write!(
                    &mut out,
                    r#"<line x1="0" y1="{y1}" x2="{x2}" y2="{y2}" class="branch branch{idx}"/>"#,
                    y1 = fmt(pos),
                    x2 = fmt(layout.max_pos),
                    y2 = fmt(pos),
                    idx = idx
                );
            }

            let name = escape_xml(&b.name);
            let bbox_w = b.bbox_width.max(0.0);
            let bbox_h = b.bbox_height.max(0.0);

            let bkg_class = format!(r#"branchLabelBkg label{idx}"#);
            let label_class = format!(r#"label branch-label{idx}"#);

            if direction == "TB" {
                let x = pos - bbox_w / 2.0 - 10.0;
                let _ = write!(
                    &mut out,
                    r#"<rect class="{cls}" rx="4" ry="4" x="{x}" y="0" width="{w}" height="{h}"/>"#,
                    cls = bkg_class,
                    x = fmt(x),
                    w = fmt(bbox_w + 18.0),
                    h = fmt(bbox_h + 4.0),
                );
                let tx = pos - bbox_w / 2.0 - 5.0;
                let _ = write!(
                    &mut out,
                    r#"<g class="branchLabel"><g class="{cls}" transform="translate({x}, 0)"><text><tspan xml:space="preserve" dy="1em" x="0" class="row">{name}</tspan></text></g></g>"#,
                    cls = label_class,
                    x = fmt(tx),
                    name = name
                );
            } else if direction == "BT" {
                let x = pos - bbox_w / 2.0 - 10.0;
                let _ = write!(
                    &mut out,
                    r#"<rect class="{cls}" rx="4" ry="4" x="{x}" y="{y}" width="{w}" height="{h}"/>"#,
                    cls = bkg_class,
                    x = fmt(x),
                    y = fmt(layout.max_pos),
                    w = fmt(bbox_w + 18.0),
                    h = fmt(bbox_h + 4.0),
                );
                let tx = pos - bbox_w / 2.0 - 5.0;
                let _ = write!(
                    &mut out,
                    r#"<g class="branchLabel"><g class="{cls}" transform="translate({x}, {y})"><text><tspan xml:space="preserve" dy="1em" x="0" class="row">{name}</tspan></text></g></g>"#,
                    cls = label_class,
                    x = fmt(tx),
                    y = fmt(layout.max_pos),
                    name = name
                );
            } else {
                let rotate_pad = if layout.rotate_commit_label {
                    30.0
                } else {
                    0.0
                };
                let x = -bbox_w - 4.0 - rotate_pad;
                let y = -bbox_h / 2.0 + 8.0;
                let _ = write!(
                    &mut out,
                    r#"<rect class="{cls}" rx="4" ry="4" x="{x}" y="{y}" width="{w}" height="{h}" transform="translate(-19, {ty})"/>"#,
                    cls = bkg_class,
                    x = fmt(x),
                    y = fmt(y),
                    w = fmt(bbox_w + 18.0),
                    h = fmt(bbox_h + 4.0),
                    ty = fmt(pos - bbox_h / 2.0),
                );
                let tx = -bbox_w - 14.0 - rotate_pad;
                let _ = write!(
                    &mut out,
                    r#"<g class="branchLabel"><g class="{cls}" transform="translate({x}, {y})"><text><tspan xml:space="preserve" dy="1em" x="0" class="row">{name}</tspan></text></g></g>"#,
                    cls = label_class,
                    x = fmt(tx),
                    y = fmt(pos - bbox_h / 2.0 - 1.0),
                    name = name
                );
            }
        }
        out.push_str("</g>");
    }

    out.push_str(r#"<g class="commit-arrows">"#);
    for a in &layout.arrows {
        let _ = write!(
            &mut out,
            r#"<path d="{d}" class="arrow arrow{idx}"/>"#,
            d = escape_attr(&a.d),
            idx = a.class_index % THEME_COLOR_LIMIT
        );
    }
    out.push_str("</g>");

    fn commit_class_type(symbol_type: i64) -> &'static str {
        match symbol_type {
            0 => "commit-normal",
            1 => "commit-reverse",
            2 => "commit-highlight",
            3 => "commit-merge",
            4 => "commit-cherry-pick",
            _ => "commit-normal",
        }
    }

    fn commit_symbol_type(commit: &crate::model::GitGraphCommitLayout) -> i64 {
        commit.custom_type.unwrap_or(commit.commit_type)
    }

    out.push_str(r#"<g class="commit-bullets">"#);
    for c in &layout.commits {
        let branch_i = branch_idx.get(c.branch.as_str()).copied().unwrap_or(0);
        let symbol_type = commit_symbol_type(c);
        let type_class = commit_class_type(symbol_type);
        let idx = branch_i % THEME_COLOR_LIMIT;
        let id = escape_attr(&c.id);

        if symbol_type == 2 {
            let _ = write!(
                &mut out,
                r#"<rect x="{x}" y="{y}" width="20" height="20" class="commit {id} commit-highlight{idx} {type_class}-outer"/>"#,
                x = fmt(c.x - 10.0),
                y = fmt(c.y - 10.0),
                id = id,
                idx = idx,
                type_class = type_class
            );
            let _ = write!(
                &mut out,
                r#"<rect x="{x}" y="{y}" width="12" height="12" class="commit {id} commit{idx} {type_class}-inner"/>"#,
                x = fmt(c.x - 6.0),
                y = fmt(c.y - 6.0),
                id = id,
                idx = idx,
                type_class = type_class
            );
        } else if symbol_type == 4 {
            let _ = write!(
                &mut out,
                r#"<circle cx="{x}" cy="{y}" r="10" class="commit {id} {type_class}"/>"#,
                x = fmt(c.x),
                y = fmt(c.y),
                id = id,
                type_class = type_class
            );
            let _ = write!(
                &mut out,
                r##"<circle cx="{x}" cy="{y}" r="2.75" fill="#fff" class="commit {id} {type_class}"/>"##,
                x = fmt(c.x - 3.0),
                y = fmt(c.y + 2.0),
                id = id,
                type_class = type_class
            );
            let _ = write!(
                &mut out,
                r##"<circle cx="{x}" cy="{y}" r="2.75" fill="#fff" class="commit {id} {type_class}"/>"##,
                x = fmt(c.x + 3.0),
                y = fmt(c.y + 2.0),
                id = id,
                type_class = type_class
            );
            let _ = write!(
                &mut out,
                r##"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="#fff" class="commit {id} {type_class}"/>"##,
                x1 = fmt(c.x + 3.0),
                y1 = fmt(c.y + 1.0),
                x2 = fmt(c.x),
                y2 = fmt(c.y - 5.0),
                id = id,
                type_class = type_class
            );
            let _ = write!(
                &mut out,
                r##"<line x1="{x1}" y1="{y1}" x2="{x2}" y2="{y2}" stroke="#fff" class="commit {id} {type_class}"/>"##,
                x1 = fmt(c.x - 3.0),
                y1 = fmt(c.y + 1.0),
                x2 = fmt(c.x),
                y2 = fmt(c.y - 5.0),
                id = id,
                type_class = type_class
            );
        } else {
            let r = if c.commit_type == 3 { 9.0 } else { 10.0 };
            let _ = write!(
                &mut out,
                r#"<circle cx="{x}" cy="{y}" r="{r}" class="commit {id} commit{idx}"/>"#,
                x = fmt(c.x),
                y = fmt(c.y),
                r = fmt(r),
                id = id,
                idx = idx
            );
            if symbol_type == 3 {
                let _ = write!(
                    &mut out,
                    r#"<circle cx="{x}" cy="{y}" r="6" class="commit {type_class} {id} commit{idx}"/>"#,
                    x = fmt(c.x),
                    y = fmt(c.y),
                    type_class = type_class,
                    id = id,
                    idx = idx
                );
            }
            if symbol_type == 1 {
                let d = format!(
                    "M {},{}L{},{}M {},{}L{},{}",
                    fmt(c.x - 5.0),
                    fmt(c.y - 5.0),
                    fmt(c.x + 5.0),
                    fmt(c.y + 5.0),
                    fmt(c.x - 5.0),
                    fmt(c.y + 5.0),
                    fmt(c.x + 5.0),
                    fmt(c.y - 5.0)
                );
                let _ = write!(
                    &mut out,
                    r#"<path d="{d}" class="commit {type_class} {id} commit{idx}"/>"#,
                    d = escape_attr(&d),
                    type_class = type_class,
                    id = id,
                    idx = idx
                );
            }
        }
    }
    out.push_str("</g>");

    let commit_font_family = config_string(effective_config, &["fontFamily"])
        .or_else(|| config_string(effective_config, &["themeVariables", "fontFamily"]))
        .map(|s| s.trim().trim_end_matches(';').trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "\"trebuchet ms\", verdana, arial, sans-serif".to_string());
    let commit_label_style = crate::text::TextStyle {
        font_family: Some(commit_font_family),
        font_size: css.commit_label_font_size_px,
        font_weight: None,
    };
    let tag_label_style = crate::text::TextStyle {
        font_family: commit_label_style.font_family.clone(),
        font_size: css.tag_label_font_size_px,
        font_weight: None,
    };

    out.push_str(r#"<g class="commit-labels">"#);
    for c in &layout.commits {
        let show = (c.commit_type != 3 || c.custom_id.unwrap_or(false))
            && c.commit_type != 4
            && layout.show_commit_label;
        if show {
            let bbox_w = gitgraph_commit_tag_label_width_px(measurer, &c.id, &commit_label_style);
            let bbox_h = gitgraph_commit_tag_label_height_px(measurer, &c.id, &commit_label_style);

            let mut wrapper_transform: Option<String> = None;
            let mut rect_transform: Option<String> = None;
            let mut text_transform: Option<String> = None;

            let mut rect_x = c.pos_with_offset - bbox_w / 2.0 - PY;
            let mut rect_y = c.y + 13.5;
            let rect_w = bbox_w + 2.0 * PY;
            let rect_h = bbox_h + 2.0 * PY;
            let mut text_x = c.pos_with_offset - bbox_w / 2.0;
            let mut text_y = c.y + 25.0;

            if direction == "TB" || direction == "BT" {
                rect_x = c.x - (bbox_w + 4.0 * PX + 5.0);
                rect_y = c.y - 12.0;
                text_x = c.x - (bbox_w + 4.0 * PX);
                text_y = c.y + bbox_h - 12.0;
            }

            if layout.rotate_commit_label {
                if direction == "TB" || direction == "BT" {
                    let t = format!("rotate(-45, {}, {})", fmt(c.x), fmt(c.y));
                    rect_transform = Some(t.clone());
                    text_transform = Some(t);
                } else {
                    let r_x = -7.5 - ((bbox_w + 10.0) / 25.0) * 9.5;
                    let r_y = 10.0 + (bbox_w / 25.0) * 8.5;
                    wrapper_transform = Some(format!(
                        "translate({}, {}) rotate(-45, {}, {})",
                        fmt(r_x),
                        fmt(r_y),
                        fmt(c.pos),
                        fmt(c.y)
                    ));
                }
            }

            out.push_str("<g");
            if let Some(t) = &wrapper_transform {
                let _ = write!(&mut out, r#" transform="{}""#, escape_attr(t));
            }
            out.push('>');

            out.push_str(r#"<rect class="commit-label-bkg""#);
            let _ = write!(
                &mut out,
                r#" x="{}" y="{}" width="{}" height="{}""#,
                fmt(rect_x),
                fmt(rect_y),
                fmt(rect_w),
                fmt(rect_h)
            );
            if let Some(t) = &rect_transform {
                let _ = write!(&mut out, r#" transform="{}""#, escape_attr(t));
            }
            out.push_str("/>");

            out.push_str(r#"<text class="commit-label""#);
            let _ = write!(
                &mut out,
                r#" x="{}" y="{}""#,
                fmt_display(text_x),
                fmt_display(text_y)
            );
            if let Some(t) = &text_transform {
                let _ = write!(&mut out, r#" transform="{}""#, escape_attr(t));
            }
            let _ = write!(&mut out, ">{}</text>", escape_xml(&c.id));
            out.push_str("</g>");
        }

        if !c.tags.is_empty() {
            let mut y_offset = 0.0;
            let mut max_w: f64 = 0.0;
            let mut max_h: f64 = 0.0;
            let mut tag_values = c.tags.clone();
            tag_values.reverse();

            struct TagGeom {
                y_offset: f64,
            }
            let mut elems: Vec<TagGeom> = Vec::new();
            for tag_value in &tag_values {
                let bbox_w =
                    gitgraph_commit_tag_label_width_px(measurer, tag_value, &tag_label_style);
                let bbox_h =
                    gitgraph_commit_tag_label_height_px(measurer, tag_value, &tag_label_style);
                max_w = max_w.max(bbox_w.max(0.0));
                max_h = max_h.max(bbox_h.max(0.0));
                elems.push(TagGeom { y_offset });
                y_offset += 20.0;
            }

            for (i, tag_value) in tag_values.iter().enumerate() {
                let y_off = elems.get(i).map(|e| e.y_offset).unwrap_or(0.0);
                let h2 = max_h / 2.0;
                let ly = c.y - 19.2 - y_off;

                if direction == "TB" || direction == "BT" {
                    let y_origin = c.pos + y_off;
                    let points = format!(
                        "{} {} {} {} {} {} {} {} {} {} {} {}",
                        fmt(c.x),
                        fmt(y_origin + 2.0),
                        fmt(c.x),
                        fmt(y_origin - 2.0),
                        fmt(c.x + 10.0),
                        fmt(y_origin - h2 - 2.0),
                        fmt(c.x + 10.0 + max_w + 4.0),
                        fmt(y_origin - h2 - 2.0),
                        fmt(c.x + 10.0 + max_w + 4.0),
                        fmt(y_origin + h2 + 2.0),
                        fmt(c.x + 10.0),
                        fmt(y_origin + h2 + 2.0)
                    );
                    let poly_t =
                        format!("translate(12,12) rotate(45, {},{})", fmt(c.x), fmt(c.pos));
                    let hole_t =
                        format!("translate(12,12) rotate(45, {},{})", fmt(c.x), fmt(c.pos));
                    let text_t =
                        format!("translate(14,14) rotate(45, {},{})", fmt(c.x), fmt(c.pos));

                    let _ = write!(
                        &mut out,
                        r#"<polygon class="tag-label-bkg" points="{pts}" transform="{t}"/>"#,
                        pts = escape_attr(&points),
                        t = escape_attr(&poly_t)
                    );
                    let _ = write!(
                        &mut out,
                        r#"<circle cy="{cy}" cx="{cx}" r="1.5" class="tag-hole" transform="{t}"/>"#,
                        cy = fmt(y_origin),
                        cx = fmt(c.x + PX / 2.0),
                        t = escape_attr(&hole_t)
                    );
                    let _ = write!(
                        &mut out,
                        r#"<text y="{y}" class="tag-label" x="{x}" transform="{t}">{txt}</text>"#,
                        y = fmt(y_origin + 3.0),
                        x = fmt(c.x + 5.0),
                        t = escape_attr(&text_t),
                        txt = escape_xml(tag_value)
                    );
                } else {
                    let points = format!(
                        "{} {} {} {} {} {} {} {} {} {} {} {}",
                        fmt(c.pos - max_w / 2.0 - PX / 2.0),
                        fmt(ly + PY),
                        fmt(c.pos - max_w / 2.0 - PX / 2.0),
                        fmt(ly - PY),
                        fmt(c.pos_with_offset - max_w / 2.0 - PX),
                        fmt(ly - h2 - PY),
                        fmt(c.pos_with_offset + max_w / 2.0 + PX),
                        fmt(ly - h2 - PY),
                        fmt(c.pos_with_offset + max_w / 2.0 + PX),
                        fmt(ly + h2 + PY),
                        fmt(c.pos_with_offset - max_w / 2.0 - PX),
                        fmt(ly + h2 + PY)
                    );
                    let _ = write!(
                        &mut out,
                        r#"<polygon class="tag-label-bkg" points="{pts}"/>"#,
                        pts = escape_attr(&points)
                    );
                    let _ = write!(
                        &mut out,
                        r#"<circle cy="{cy}" cx="{cx}" r="1.5" class="tag-hole"/>"#,
                        cy = fmt(ly),
                        cx = fmt(c.pos - max_w / 2.0 + PX / 2.0)
                    );
                    let _ = write!(
                        &mut out,
                        r#"<text y="{y}" class="tag-label" x="{x}">{txt}</text>"#,
                        y = fmt(c.y - 16.0 - y_off),
                        x = fmt(c.pos_with_offset - max_w / 2.0),
                        txt = escape_xml(tag_value)
                    );
                }
            }
        }
    }
    out.push_str("</g>");

    let title = diagram_title.map(str::trim).filter(|t| !t.is_empty());
    let title_top_margin = config_f64(effective_config, &["gitGraph", "titleTopMargin"])
        .unwrap_or(25.0)
        .max(0.0);
    let title_y = -title_top_margin;

    if let Some(title) = title {
        let _ = write!(
            &mut out,
            r#"<text text-anchor="middle" x="{x}" y="{y}" class="gitTitleText" xmlns="http://www.w3.org/2000/svg">{text}</text>"#,
            x = TITLE_X_PLACEHOLDER,
            y = fmt(title_y),
            text = escape_xml(title),
        );
    }

    out.push_str("</svg>\n");

    // GitGraph renders rotated commit labels (e.g. `rotate(-45, ...)`) that are not represented
    // in the precomputed layout bounds. Mirror Mermaid's `setupGraphViewbox(svg.getBBox() + pad)`
    // by computing a headless SVG bbox and patching the root viewBox/max-width.
    let mut bb_dbg = SvgEmittedBoundsDebug {
        bounds: Bounds {
            min_x: 0.0,
            min_y: 0.0,
            max_x: 0.0,
            max_y: 0.0,
        },
        min_x: None,
        min_y: None,
        max_x: None,
        max_y: None,
    };
    let mut b = svg_emitted_bounds_from_svg_inner(&out, Some(&mut bb_dbg)).unwrap_or(Bounds {
        min_x: vb_min_x,
        min_y: vb_min_y,
        max_x: vb_min_x + vb_w,
        max_y: vb_min_y + vb_h,
    });
    include_gitgraph_branch_line_bounds(&mut b, layout);
    let title_anchor_x = (b.min_x + b.max_x) / 2.0;
    let mut title_expands_x_bounds = false;
    if let Some(title) = title {
        let (title_left, title_right) = measurer.measure_svg_title_bbox_x(title, &title_style);
        let (ascent, descent) = crate::text::svg_title_bbox_vertical_extents_px(&title_style);
        let title_min_x = title_anchor_x - title_left;
        let title_max_x = title_anchor_x + title_right;
        title_expands_x_bounds = title_min_x < b.min_x && title_max_x > b.max_x;
        b.min_x = b.min_x.min(title_min_x);
        b.max_x = b.max_x.max(title_max_x);
        b.min_y = b.min_y.min(title_y - ascent);
        b.max_y = b.max_y.max(title_y + descent);
    }

    // Mermaid computes the root viewBox from `svg.getBBox()` + padding.
    //
    // Our `svg_emitted_bounds_from_svg` implementation already evaluates transforms in `f32` and
    // applies a small outward bias on rotated minima (matching Chromium quirks seen in Mermaid's
    // gitGraph baselines). Use those bounds directly as an `f32` bbox here.
    let pad = VIEWBOX_PADDING_PX as f32;

    fn next_down_f32(v: f32) -> f32 {
        if v.is_nan() || v == f32::NEG_INFINITY {
            return v;
        }
        if v == 0.0 {
            return -f32::from_bits(1);
        }
        let bits = v.to_bits();
        if v > 0.0 {
            f32::from_bits(bits - 1)
        } else {
            f32::from_bits(bits + 1)
        }
    }

    fn next_up_f32(v: f32) -> f32 {
        if v.is_nan() || v == f32::INFINITY {
            return v;
        }
        if v == 0.0 {
            return f32::from_bits(1);
        }
        let bits = v.to_bits();
        if v > 0.0 {
            f32::from_bits(bits + 1)
        } else {
            f32::from_bits(bits - 1)
        }
    }

    fn f32_round_up(v: f64) -> f32 {
        let q = v as f32;
        if !q.is_finite() {
            return q;
        }
        if (q as f64) < v { next_up_f32(q) } else { q }
    }

    let bbox_x = b.min_x as f32;
    let bbox_y = b.min_y as f32;
    let dbg_viewbox = std::env::var("MERMAN_DEBUG_GITGRAPH_VIEWBOX").is_ok();

    if dbg_viewbox {
        if let Some(c) = &bb_dbg.min_x {
            let raw = c.bounds.min_x as f32;
            eprintln!(
                "gitgraph viewbox dbg: before bbox_x={bbox_x:?} raw_min_x={raw:?} next_down={:?}",
                next_down_f32(raw)
            );
        } else {
            eprintln!("gitgraph viewbox dbg: before bbox_x={bbox_x:?} raw_min_x=<none>");
        }
    }
    if dbg_viewbox {
        eprintln!(
            "gitgraph viewbox dbg: after bbox_x={bbox_x:?} bbox_y={bbox_y:?} b.min_x={:?} b.max_x={:?}",
            b.min_x, b.max_x
        );
    }

    // Match Chromium's `getBBox()` behavior more closely:
    // - x/y: `f32`-quantized extrema
    // - w/h: computed in `f64`, then rounded to `f32` with an upward bias
    let title_bbox_w_bias = if title_expands_x_bounds {
        // GitGraph centers titles after measuring the emitted graph bbox. When the title alone
        // expands both horizontal bounds, Chromium's final root width behaves one 1/128px lattice
        // cell wider than the deterministic symmetric title half-width.
        1.0 / 128.0
    } else {
        0.0
    };
    let bbox_w = f32_round_up((b.max_x - b.min_x) + title_bbox_w_bias);
    let bbox_h = f32_round_up(b.max_y - b.min_y);
    let _ = &bb_dbg;

    // Mermaid sets the root viewBox from `getBBox()` + padding in JS `Number` (double) space.
    // Keep these computations in `f64` so we match the upstream stringified values exactly.
    let vb_min_x = (bbox_x as f64) - (pad as f64);
    let vb_min_y = (bbox_y as f64) - (pad as f64);
    let vb_w = (bbox_w as f64) + 2.0 * (pad as f64);
    let vb_h = (bbox_h as f64) + 2.0 * (pad as f64);
    if dbg_viewbox {
        eprintln!(
            "gitgraph viewbox dbg: bbox_h={bbox_h:?} bbox_h_bits={} pad={pad:?} vb_min_x={vb_min_x:?} vb_min_y={vb_min_y:?} vb_w={vb_w:?} vb_h={vb_h:?}",
            bbox_h.to_bits()
        );
    }
    let mut view_box_attr = format!(
        "{} {} {} {}",
        fmt(vb_min_x),
        fmt(vb_min_y),
        fmt(vb_w),
        fmt(vb_h)
    );
    let mut max_width_attr = fmt_string(vb_w);
    let mut width_attr = max_width_attr.clone();
    let mut height_attr = fmt_string(vb_h);

    apply_root_viewport_override(
        diagram_id,
        &mut view_box_attr,
        &mut width_attr,
        &mut height_attr,
        &mut max_width_attr,
        crate::generated::gitgraph_root_overrides_11_12_2::lookup_gitgraph_root_viewport_override,
    );

    out = out.replacen(VIEWBOX_PLACEHOLDER, &view_box_attr, 1);
    // Mermaid gitGraph baselines stringify `max-width` directly from the computed `viewBox` width
    // (no fixed precision rounding), so keep the full `Number#toString()`-like output here.
    out = out.replacen(MAX_WIDTH_PLACEHOLDER, &max_width_attr, 1);
    out = out.replacen(TITLE_X_PLACEHOLDER, &fmt_string(title_anchor_x), 1);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn gitgraph_css_includes_mermaid_11_15_branch_theme_rules() {
        let css = gitgraph_css("git", &json!({})).css;

        assert!(css.contains("#git .branch-label0{fill:#ffffff;}"));
        assert!(css.contains(
            "#git .commit0{stroke:hsl(240, 100%, 46.2745098039%);fill:hsl(240, 100%, 46.2745098039%);}"
        ));
        assert!(css.contains("#git .label0{fill:hsl(240, 100%, 46.2745098039%);}"));
        assert!(css.contains("#git .arrow0{stroke:hsl(240, 100%, 46.2745098039%);}"));
        assert!(css.contains("#git .commit-merge{stroke:#ECECFF;fill:#ECECFF;}"));
        assert!(css.contains("#git .commit-highlight-inner{stroke:#ECECFF;fill:#ECECFF;}"));
    }

    #[test]
    fn gitgraph_css_uses_redux_geometry_theme_rules() {
        let css = gitgraph_css(
            "git",
            &json!({
                "theme": "redux",
                "themeVariables": {
                    "nodeBorder": "#101010",
                    "mainBkg": "#ffffff",
                    "strokeWidth": 2,
                    "noteFontWeight": 600,
                    "commitLineColor": "#202020"
                }
            }),
        );

        assert!(css.defs.is_empty());
        assert!(
            css.css
                .contains("#git .branch-label0{fill:#101010;font-weight:600;}")
        );
        assert!(css.css.contains("#git .commit0{stroke:#101010;}"));
        assert!(
            css.css.contains(
                "#git .label0{fill:#ffffff;stroke:#101010;stroke-width:2;font-weight:600;}"
            )
        );
        assert!(
            css.css
                .contains("#git .branch{stroke-width:2;stroke:#202020;stroke-dasharray:4 2;}")
        );
        assert!(
            css.css
                .contains("#git .arrow{stroke-width:2;stroke-linecap:round;fill:none;}")
        );
        assert!(
            css.css
                .contains("#git .commit-label{font-size:10px;fill:#101010;font-weight:600;}")
        );
        assert!(
            css.css
                .contains("#git .commit-label-bkg{font-size:10px;fill:transparent;}")
        );
        assert!(
            css.css
                .contains("#git .commit-merge{stroke:#ffffff;fill:#ffffff;}")
        );
        assert!(
            css.css
                .contains("#git .commit-reverse{stroke:#ffffff;fill:#ffffff;stroke-width:2;}")
        );
    }

    #[test]
    fn gitgraph_css_uses_redux_color_theme_rules() {
        let css = gitgraph_css(
            "git",
            &json!({
                "theme": "redux-color",
                "themeVariables": {
                    "nodeBorder": "#101010",
                    "mainBkg": "#ffffff",
                    "strokeWidth": 2,
                    "noteFontWeight": 600,
                    "borderColorArray": ["#aa0000", "#00aa00"]
                }
            }),
        )
        .css;

        assert!(css.contains("#git .commit0{stroke:#101010;}"));
        assert!(css.contains("#git .commit-highlight0{stroke:#101010;fill:#ffffff;}"));
        assert!(
            css.contains(
                "#git .label0{fill:#ffffff;stroke:#101010;stroke-width:2;font-weight:600;}"
            )
        );
        assert!(css.contains("#git .commit1{stroke:#00aa00;fill:#00aa00;}"));
        assert!(css.contains("#git .label1{fill:#00aa00;stroke:#00aa00;stroke-width:2;}"));
        assert!(css.contains("#git .arrow1{stroke:#00aa00;}"));
    }

    #[test]
    fn gitgraph_css_uses_neo_gradient_theme_rules() {
        let css = gitgraph_css(
            "git",
            &json!({
                "theme": "neo",
                "themeVariables": {
                    "nodeBorder": "#101010",
                    "mainBkg": "#ffffff",
                    "strokeWidth": 2,
                    "useGradient": true,
                    "gradientStart": "#112233",
                    "gradientStop": "#445566",
                    "git1": "#00aa00",
                    "gitInv1": "#aa00aa",
                    "gitBranchLabel1": "#202020"
                }
            }),
        );

        assert!(
            css.defs
                .contains(r#"<defs><linearGradient id="git-gradient""#)
        );
        assert!(css.defs.contains(r##"stop-color="#112233""##));
        assert!(css.defs.contains(r##"stop-color="#445566""##));
        assert!(css.css.contains("#git .branch-label0{fill:#101010;}"));
        assert!(css.css.contains("#git .commit0{stroke:#101010;}"));
        assert!(css.css.contains("#git .commit-bullets{fill:#101010;}"));
        assert!(
            css.css
                .contains("#git .label0{fill:#ffffff;stroke:url(#git-gradient);stroke-width:2;}")
        );
        assert!(
            css.css
                .contains("#git .label11{fill:#ffffff;stroke:url(#git-gradient);stroke-width:2;}")
        );
        assert!(css.css.contains("#git .branch-label1{fill:#202020;}"));
        assert!(
            css.css
                .contains("#git .commit1{stroke:#00aa00;fill:#00aa00;}")
        );
        assert!(
            css.css
                .contains("#git .commit-highlight1{stroke:#aa00aa;fill:#aa00aa;}")
        );
    }
}
