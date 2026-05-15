#![recursion_limit = "256"]

use anyhow::{Context as _, Result, anyhow};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MermaidBackend {
    MermaidRs,
    Merman,
}

#[derive(Debug, Clone)]
pub struct AccentColor {
    /// The accent stroke/border color (e.g. `"rgb(116, 173, 232)"`).
    pub stroke: String,
    /// The base background color from which fill and text colors are derived.
    pub background: String,
}

#[derive(Debug, Clone)]
pub struct MermaidTheme {
    pub dark_mode: bool,
    pub font_family: String,
    pub background: String,
    pub primary_color: String,
    pub primary_text_color: String,
    pub primary_border_color: String,
    pub secondary_color: String,
    pub tertiary_color: String,
    pub line_color: String,
    pub text_color: String,
    pub edge_label_background: String,
    pub cluster_background: String,
    pub cluster_border: String,
    pub note_background: String,
    pub note_border: String,
    pub actor_background: String,
    pub actor_border: String,
    pub activation_background: String,
    pub activation_border: String,
    pub git_branch_colors: [String; 8],
    pub git_branch_label_colors: [String; 8],
    pub er_attr_bg_odd: String,
    pub er_attr_bg_even: String,
    pub accent_colors: Vec<AccentColor>,
}

impl Default for MermaidTheme {
    fn default() -> Self {
        let theme = mermaid_rs_renderer::Theme::modern();
        Self {
            dark_mode: false,
            font_family: theme.font_family,
            background: theme.background,
            primary_color: theme.primary_color,
            primary_text_color: theme.primary_text_color,
            primary_border_color: theme.primary_border_color.clone(),
            secondary_color: theme.secondary_color,
            tertiary_color: theme.tertiary_color,
            line_color: theme.line_color,
            text_color: theme.text_color.clone(),
            edge_label_background: theme.edge_label_background,
            cluster_background: theme.cluster_background,
            cluster_border: theme.cluster_border,
            note_background: theme.sequence_note_fill,
            note_border: theme.sequence_note_border,
            actor_background: theme.sequence_actor_fill,
            actor_border: theme.sequence_actor_border,
            activation_background: theme.sequence_activation_fill,
            activation_border: theme.sequence_activation_border,
            git_branch_colors: theme.git_colors,
            git_branch_label_colors: theme.git_branch_label_colors,
            er_attr_bg_odd: theme.primary_border_color,
            er_attr_bg_even: theme.text_color,
            accent_colors: Vec::new(),
        }
    }
}

pub fn render_to_svg(
    source: &str,
    theme: &MermaidTheme,
    backend: MermaidBackend,
) -> Result<String> {
    match backend {
        MermaidBackend::MermaidRs => render_with_mermaid_rs(source, theme),
        MermaidBackend::Merman => render_with_merman(source, theme),
    }
}

fn source_with_accent_classdefs(source: &str, theme: &MermaidTheme) -> String {
    if theme.accent_colors.is_empty() {
        return source.to_string();
    }

    let trimmed = source.trim_start();
    let supports_classdef = trimmed.starts_with("flowchart")
        || trimmed.starts_with("graph")
        || trimmed.starts_with("classDiagram");
    if !supports_classdef {
        return source.to_string();
    }

    use std::fmt::Write;
    let mut full_source = source.to_string();
    for (i, accent) in theme.accent_colors.iter().enumerate() {
        let (fill, text) = accent_fill_and_text(&accent.background, theme.dark_mode);
        write!(
            full_source,
            "\nclassDef accent{i} fill:{fill},stroke:{},color:{text}",
            accent.stroke,
        )
        .ok();
    }
    full_source
}

fn render_with_mermaid_rs(source: &str, theme: &MermaidTheme) -> Result<String> {
    let full_source = source_with_accent_classdefs(source, theme);
    let options = mermaid_rs_renderer::RenderOptions {
        theme: to_mermaid_rs_theme(theme),
        layout: mermaid_rs_renderer::LayoutConfig::default(),
    };
    mermaid_rs_renderer::render_with_options(&full_source, options)
}

fn render_with_merman(source: &str, theme: &MermaidTheme) -> Result<String> {
    static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let diagram_id = format!("merman-{id}");

    let config = to_merman_config(theme);
    let renderer = merman::render::HeadlessRenderer::new()
        .with_site_config(config)
        .with_vendored_text_measurer()
        .with_diagram_id(&diagram_id);

    let full_source = source_with_accent_classdefs(source, theme);
    let svg = renderer
        .render_svg_sync(&full_source)
        .context("merman render failed")?
        .ok_or_else(|| anyhow!("merman returned no SVG for the given input"))?;

    // Convert foreignObject labels to plain SVG <text> elements so that
    // renderers like usvg (which don't support foreignObject) can display them.
    let svg = merman::render::foreign_object_label_fallback_svg_text(&svg);

    let injected_css = format!(
        r#"
        /* text in foreignObject labels */
        foreignObject div, foreignObject span, foreignObject p {{
            font-family: {font};
            font-size: 16px;
            color: {text};
        }}
        foreignObject p {{ margin: 0; }}
        foreignObject {{ overflow: visible; }}
        foreignObject div {{ max-width: none !important; }}
        .label-group foreignObject {{ font-weight: bold; }}

        /* mindmap section colors (use git branch accent palette) */
        {mindmap_section_css}

        /* state diagram */
        g.stateGroup rect {{ fill: {primary} !important; stroke: {border} !important; }}
        g.stateGroup text {{ fill: {text} !important; }}
        g.stateGroup .state-title {{ fill: {text} !important; }}
        .stateGroup .composit {{ fill: {background} !important; }}
        .stateGroup .alt-composit {{ fill: {tertiary} !important; }}
        .state-note {{ stroke: {note_border} !important; fill: {note_bg} !important; }}
        .state-note text {{ fill: {note_text} !important; }}
        .stateLabel .box {{ fill: {primary} !important; }}
        .stateLabel text {{ fill: {text} !important; }}
        .node circle.state-start {{ fill: {line} !important; stroke: {line} !important; }}
        .node .fork-join {{ fill: {line} !important; stroke: {line} !important; }}
        .node circle.state-end {{ fill: {border} !important; stroke: {background} !important; }}
        .end-state-inner {{ fill: {background} !important; }}
        .node:not(.mindmap-node) rect, .node:not(.mindmap-node) path {{ fill: {primary} !important; stroke: {border} !important; }}
        .node:not(.mindmap-node) polygon {{ fill: {primary} !important; stroke: {border} !important; }}
        .label-container path {{ fill: {primary} !important; stroke: {border} !important; }}
        .statediagram-cluster rect {{ fill: {primary} !important; stroke: {border} !important; }}
        .statediagram-cluster.statediagram-cluster .inner {{ fill: {background} !important; }}
        .statediagram-cluster.statediagram-cluster-alt .inner {{ fill: {tertiary} !important; }}
        .statediagram-state rect.divider {{ fill: {tertiary} !important; }}
        .statediagram-note rect {{ fill: {note_bg} !important; stroke: {note_border} !important; }}
        .statediagram-note text {{ fill: {note_text} !important; }}
        .statediagramTitleText {{ fill: {text} !important; }}
        .transition {{ stroke: {line} !important; }}
        .cluster-label, .nodeLabel {{ color: {text} !important; }}
        defs #statediagram-barbEnd {{ fill: {line} !important; stroke: {line} !important; }}
        #statediagram-barbEnd {{ fill: {line} !important; }}
        .edgeLabel .label rect {{ fill: {primary} !important; }}
        .edgeLabel rect {{ fill: {primary} !important; background-color: {primary} !important; }}
        .edgeLabel .label text {{ fill: {text} !important; }}
        .edgeLabel p {{ background-color: {primary} !important; }}
        .edgeLabel {{ background-color: {primary} !important; }}

        /* sequence diagram */
        .actor {{ stroke: {actor_border}; fill: {actor_bg} !important; }}
        text.actor>tspan {{ fill: {actor_text} !important; stroke: none; }}
        .labelText, .labelText>tspan {{ fill: {actor_text} !important; }}
        .actor-line {{ stroke: {actor_border} !important; }}
        .messageLine0 {{ stroke: {text} !important; }}
        .messageLine1 {{ stroke: {text} !important; }}
        #arrowhead path {{ fill: {text} !important; stroke: {text} !important; }}
        #crosshead path {{ fill: {text} !important; stroke: {text} !important; }}
        .messageText {{ fill: {text} !important; }}
        .loopText, .loopText>tspan {{ fill: {text} !important; }}
        .loopLine {{ stroke: {actor_border} !important; fill: {actor_border} !important; }}
        .note {{ stroke: {note_border} !important; fill: {note_bg} !important; }}
        .noteText, .noteText>tspan {{ fill: {note_text} !important; }}
        .activation0, .activation1, .activation2 {{ fill: {secondary} !important; stroke: {border} !important; }}
        .labelBox {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .actor-man line {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}
        .actor-man circle {{ stroke: {actor_border} !important; fill: {actor_bg} !important; }}

        /* pie chart */
        .pieTitleText {{ fill: {text} !important; }}
        .slice {{ fill: {text} !important; }}
        .legend text {{ fill: {text} !important; }}
        .pieOuterCircle {{ stroke: {border} !important; }}
        .pieCircle {{ stroke: {border} !important; }}

        /* journey diagram section/task fills */
        .task-type-0, .section-type-0 {{ fill: {primary} !important; }}
        .task-type-1, .section-type-1 {{ fill: {secondary} !important; }}
        .task-type-2, .section-type-2 {{ fill: {tertiary} !important; }}
        .task-type-3, .section-type-3 {{ fill: {primary} !important; }}
        .task-type-4, .section-type-4 {{ fill: {secondary} !important; }}
        .task-type-5, .section-type-5 {{ fill: {tertiary} !important; }}
        .task-type-6, .section-type-6 {{ fill: {primary} !important; }}
        .task-type-7, .section-type-7 {{ fill: {secondary} !important; }}

        /* ER diagram */
        .relationshipLabelBox {{ fill: {tertiary} !important; opacity: 0.7; background-color: {tertiary} !important; }}
        .labelBkg {{ background-color: {tertiary} !important; }}
        .edgeLabel .label {{ fill: {border} !important; }}
        .label {{ color: {text} !important; }}
        .relationshipLine {{ stroke: {line} !important; fill: none !important; }}
        .entityBox {{ fill: {primary}; stroke: {border}; }}
        .node .row-rect-odd path {{ fill: {er_odd} !important; }}
        .node .row-rect-even path {{ fill: {er_even} !important; }}

        /* edges and markers */
        .edge-thickness-normal {{ stroke-width: 1px; }}
        .relation {{ stroke: {line}; stroke-width: 1; fill: none; }}
        .edgePaths path {{ fill: none; }}
        .marker {{ fill: {line} !important; stroke: {line} !important; }}
        .marker.er {{ fill: none !important; stroke: {line} !important; }}
        .composition {{ fill: {line} !important; stroke: {line} !important; stroke-width: 1; }}
        .extension {{ fill: transparent !important; stroke: {line} !important; stroke-width: 1; }}
        .aggregation {{ fill: transparent !important; stroke: {line} !important; stroke-width: 1; }}
        .dependency {{ fill: {line} !important; stroke: {line} !important; stroke-width: 1; }}
        .lollipop {{ fill: {primary} !important; stroke: {line} !important; stroke-width: 1; }}

        /* gantt chart overrides (need !important to beat #id-scoped rules) */
        .section0 {{ fill: {tertiary} !important; }}
        .section2 {{ fill: {primary} !important; }}
        .section1, .section3 {{ fill: {secondary} !important; opacity: 0.2; }}
        .sectionTitle0, .sectionTitle1, .sectionTitle2, .sectionTitle3 {{ fill: {text} !important; }}
        .sectionTitle {{ font-family: {font} !important; }}
        .task0, .task1, .task2, .task3 {{ fill: {primary} !important; stroke: {border} !important; }}
        .taskText0, .taskText1, .taskText2, .taskText3 {{ fill: {text} !important; }}
        .taskTextOutside0, .taskTextOutside1, .taskTextOutside2, .taskTextOutside3 {{ fill: {text} !important; }}
        .taskTextOutsideRight {{ fill: {text} !important; font-family: {font} !important; }}
        .taskTextOutsideLeft {{ fill: {text} !important; }}
        .active0, .active1, .active2, .active3 {{ fill: {secondary} !important; stroke: {border} !important; }}
        .activeText0, .activeText1, .activeText2, .activeText3 {{ fill: {text} !important; }}
        .done0, .done1, .done2, .done3 {{ stroke: {border} !important; fill: {secondary} !important; stroke-width: 2; }}
        .doneText0, .doneText1, .doneText2, .doneText3 {{ fill: {text} !important; }}
        .doneCritText0, .doneCritText1, .doneCritText2, .doneCritText3 {{ fill: {text} !important; }}
        .activeCritText0, .activeCritText1, .activeCritText2, .activeCritText3 {{ fill: {text} !important; }}
        .titleText {{ fill: {text} !important; font-family: {font} !important; }}
        .grid .tick text {{ fill: {text} !important; font-family: {font} !important; }}
        .grid .tick {{ stroke: {border} !important; }}

        /* gitgraph branch colors */
        {git_branch_css}
        .commit-merge  {{ stroke: {primary}; fill: {primary}; }}
        .commit-reverse {{ stroke: {primary}; fill: {primary}; stroke-width: 3; }}
        .commit-highlight-inner {{ stroke: {primary}; fill: {primary}; }}
        .tag-label {{ font-size: 10px; }}
        .tag-label-bkg {{ fill: {primary}; stroke: {border}; }}
        .tag-hole {{ fill: {line}; }}
        .commit-label {{ fill: {text}; }}
        .commit-label-bkg {{ fill: {edge_label_bg}; }}
        .commit-id, .commit-msg, .branch-label {{
            fill: {text}; color: {text};
            font-family: {font};
        }}
    "#,
        font = theme.font_family,
        text = theme.text_color,
        line = theme.line_color,
        primary = theme.primary_color,
        border = theme.primary_border_color,
        secondary = theme.secondary_color,
        tertiary = theme.tertiary_color,
        background = theme.background,
        edge_label_bg = theme.edge_label_background,
        actor_bg = theme.actor_background,
        actor_border = theme.actor_border,
        actor_text = text_color_for_bg(&theme.actor_background),
        note_bg = theme.note_background,
        note_border = theme.note_border,
        note_text = text_color_for_bg(&theme.note_background),
        er_odd = theme.er_attr_bg_odd,
        er_even = theme.er_attr_bg_even,
        mindmap_section_css = mindmap_section_css(theme),
        git_branch_css = git_branch_css(theme),
    );

    postprocess_merman_svg(&svg, theme, &injected_css)
}

fn postprocess_merman_svg(
    svg: &str,
    theme: &MermaidTheme,
    injected_css: &str,
) -> Result<String> {
    use quick_xml::events::{BytesStart, Event};

    let mut reader = quick_xml::Reader::from_str(svg);
    reader.config_mut().check_end_names = false;
    let mut writer = quick_xml::Writer::new(Vec::new());

    let mut svg_id = String::from("merman");
    let mut in_plot_group = false;
    let mut plot_g_depth: usize = 0;
    let mut plot_path_done = false;
    let mut pie_color_idx: usize = 0;
    let mut in_legend = false;
    let mut legend_color_idx: usize = 0;
    let mut foreign_object_depth: usize = 0;

    loop {
        let event = reader.read_event().context("SVG parse error")?.into_owned();
        let is_start = matches!(&event, Event::Start(_));

        // Skip everything inside <foreignObject> — usvg can't handle them
        // and we already have <text> fallbacks from foreign_object_label_fallback_svg_text.
        if foreign_object_depth > 0 {
            match &event {
                Event::Start(e) if e.name().local_name().as_ref() == b"foreignObject" => {
                    foreign_object_depth += 1;
                    continue;
                }
                Event::End(e) if e.name().local_name().as_ref() == b"foreignObject" => {
                    foreign_object_depth -= 1;
                    continue;
                }
                Event::Eof => break,
                _ if foreign_object_depth > 0 => continue,
                _ => {}
            }
        }

        match event {
            Event::Eof => break,

            Event::Start(e) | Event::Empty(e) if e.name().local_name().as_ref() == b"foreignObject" => {
                if is_start {
                    foreign_object_depth = 1;
                }
                continue;
            }

            Event::Start(e) | Event::Empty(e) => {
                let new_elem: Option<BytesStart<'static>> = {
                    let tag = e.name().local_name();
                    match tag.as_ref() {
                        b"svg" => {
                            let bg = format!("background-color: {}", theme.background);
                            let mut ne = BytesStart::new("svg");
                            for attr in e.attributes() {
                                let attr = attr.context("invalid SVG attribute")?;
                                let key = attr.key.local_name();
                                let val =
                                    attr.unescape_value().context("SVG attribute value")?;
                                match key.as_ref() {
                                    b"id" => {
                                        svg_id = val.to_string();
                                        ne.push_attribute(("id", val.as_ref()));
                                    }
                                    b"style" => {
                                        let fixed =
                                            val.replace("background-color: white", &bg);
                                        ne.push_attribute(("style", fixed.as_str()));
                                    }
                                    _ => ne.push_attribute(attr),
                                }
                            }
                            Some(ne)
                        }

                        b"g" => {
                            if in_plot_group {
                                plot_g_depth += 1;
                            }
                            if let Some(class_attr) = e.try_get_attribute("class")? {
                                let class_val = class_attr.unescape_value()?;
                                if class_val.as_ref() == "plot" {
                                    in_plot_group = true;
                                    plot_g_depth = 1;
                                    plot_path_done = false;
                                } else if class_val.as_ref() == "legend" {
                                    in_legend = true;
                                }
                            }
                            None
                        }

                        b"rect" if in_legend => {
                            if legend_color_idx < theme.git_branch_colors.len() {
                                let color = &theme.git_branch_colors[legend_color_idx];
                                legend_color_idx += 1;
                                let mut ne = BytesStart::new("rect");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"style" {
                                        let new_style =
                                            format!("fill: {color}; stroke: {color};");
                                        ne.push_attribute(("style", new_style.as_str()));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                in_legend = false;
                                Some(ne)
                            } else {
                                in_legend = false;
                                None
                            }
                        }

                        b"rect" if in_plot_group => {
                            let bar_color = &theme.git_branch_colors[0];
                            let mut ne = BytesStart::new("rect");
                            for attr in e.attributes() {
                                let attr = attr?;
                                match attr.key.local_name().as_ref() {
                                    b"fill" => {
                                        ne.push_attribute(("fill", bar_color.as_str()));
                                    }
                                    b"stroke" => {
                                        ne.push_attribute(("stroke", bar_color.as_str()));
                                    }
                                    _ => ne.push_attribute(attr),
                                }
                            }
                            Some(ne)
                        }

                        b"path" => {
                            let class_str = e
                                .try_get_attribute("class")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;

                            if class_str.as_deref() == Some("pieCircle")
                                && pie_color_idx < theme.git_branch_colors.len()
                            {
                                let color = &theme.git_branch_colors[pie_color_idx];
                                pie_color_idx += 1;
                                let mut ne = BytesStart::new("path");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"fill" {
                                        ne.push_attribute(("fill", color.as_str()));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else if in_plot_group
                                && !plot_path_done
                                && e.try_get_attribute("stroke")?.is_some()
                            {
                                plot_path_done = true;
                                let line_color = &theme.git_branch_colors[1];
                                let mut ne = BytesStart::new("path");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"stroke" {
                                        ne.push_attribute(("stroke", line_color.as_str()));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else {
                                None
                            }
                        }

                        b"text" => {
                            let fill_val = e
                                .try_get_attribute("fill")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;
                            let needs_fix = matches!(
                                fill_val.as_deref(),
                                Some("#333") | Some("")
                            );
                            if needs_fix {
                                let mut ne = BytesStart::new("text");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"fill" {
                                        ne.push_attribute((
                                            "fill",
                                            theme.text_color.as_str(),
                                        ));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else {
                                None
                            }
                        }

                        b"rect" if !in_plot_group && !in_legend => {
                            let width_val = e
                                .try_get_attribute("width")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;
                            if matches!(width_val.as_deref(), Some("")) {
                                let mut ne = BytesStart::new("rect");
                                for attr in e.attributes() {
                                    let attr = attr?;
                                    if attr.key.local_name().as_ref() == b"width" {
                                        ne.push_attribute(("width", "0"));
                                    } else {
                                        ne.push_attribute(attr);
                                    }
                                }
                                Some(ne)
                            } else {
                                None
                            }
                        }

                        _ => None,
                    }
                };

                let elem = new_elem.unwrap_or(e);
                if is_start {
                    writer.write_event(Event::Start(elem))?;
                } else {
                    writer.write_event(Event::Empty(elem))?;
                }
            }

            Event::End(e) => {
                if e.name().local_name().as_ref() == b"style" {
                    let scoped_css: String = injected_css
                        .lines()
                        .map(|line| {
                            let trimmed = line.trim();
                            if trimmed.starts_with('.')
                                || trimmed.starts_with("foreignObject")
                            {
                                if let Some(brace) = trimmed.find('{') {
                                    let (selectors, rest) = trimmed.split_at(brace);
                                    let scoped_selectors: Vec<String> = selectors
                                        .split(',')
                                        .map(|s| format!("#{svg_id} {}", s.trim()))
                                        .collect();
                                    format!(
                                        "        {}{}\n",
                                        scoped_selectors.join(", "),
                                        rest
                                    )
                                } else {
                                    format!("{line}\n")
                                }
                            } else {
                                format!("{line}\n")
                            }
                        })
                        .collect();
                    writer.get_mut().extend_from_slice(scoped_css.as_bytes());
                }

                if e.name().local_name().as_ref() == b"g" && in_plot_group {
                    plot_g_depth -= 1;
                    if plot_g_depth == 0 {
                        in_plot_group = false;
                    }
                }

                writer.write_event(Event::End(e))?;
            }

            event => writer.write_event(event)?,
        }
    }

    let svg = String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")?;
    Ok(sanitize_nan_colors(&svg))
}

/// Replace any `hsl(…, NaN%)` values that merman's internal color derivation
/// can produce. These are unparseable by usvg and fall back to black.
fn sanitize_nan_colors(svg: &str) -> String {
    let mut result = String::with_capacity(svg.len());
    let mut remaining = svg;
    while let Some(start) = remaining.find("hsl(") {
        let after_hsl = start + 4;
        if let Some(end) = remaining[after_hsl..].find(')') {
            let hsl_body = &remaining[after_hsl..after_hsl + end];
            if hsl_body.contains("NaN") {
                result.push_str(&remaining[..start]);
                result.push_str("transparent");
                remaining = &remaining[after_hsl + end + 1..];
                continue;
            }
        }
        result.push_str(&remaining[..after_hsl]);
        remaining = &remaining[after_hsl..];
    }
    result.push_str(remaining);
    result
}

fn parse_rgb(color: &str) -> Option<(u8, u8, u8)> {
    if let Some(inner) = color.strip_prefix("rgb(").and_then(|s| s.strip_suffix(')')) {
        let parts: Vec<u8> = inner
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();
        if parts.len() >= 3 {
            return Some((parts[0], parts[1], parts[2]));
        }
    } else {
        let hex = color.trim_start_matches('#');
        if hex.len() >= 6 {
            return Some((
                u8::from_str_radix(&hex[0..2], 16).unwrap_or(0),
                u8::from_str_radix(&hex[2..4], 16).unwrap_or(0),
                u8::from_str_radix(&hex[4..6], 16).unwrap_or(0),
            ));
        }
    }
    None
}

fn luma(r: u8, g: u8, b: u8) -> f64 {
    fn linearize(c: f64) -> f64 {
        let c = c / 255.0;
        if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) }
    }
    0.2126 * linearize(r as f64) + 0.7152 * linearize(g as f64) + 0.0722 * linearize(b as f64)
}

fn text_color_for_bg(color: &str) -> &'static str {
    let (r, g, b) = parse_rgb(color).unwrap_or((0, 0, 0));
    if luma(r, g, b) > 0.4 { "#000" } else { "#fff" }
}

/// Derive a fill color and contrasting text color from an accent background.
///
/// On dark themes the fill is darkened until text contrast is sufficient;
/// on light themes it is lightened. Returns `(fill_rgb_string, text_hex)`.
fn accent_fill_and_text(background: &str, dark_mode: bool) -> (String, &'static str) {
    let (r, g, b) = parse_rgb(background).unwrap_or((128, 128, 128));

    // Convert to HSL for lightness adjustment.
    let (h, s, mut l) = rgb_to_hsl(r, g, b);

    if dark_mode {
        for _ in 0..50 {
            let (cr, cg, cb) = hsl_to_rgb(h, s, l);
            if luma(cr, cg, cb) <= 0.18 {
                break;
            }
            l = (l - 0.02).max(0.0);
        }
    } else {
        for _ in 0..50 {
            let (cr, cg, cb) = hsl_to_rgb(h, s, l);
            if luma(cr, cg, cb) >= 0.35 {
                break;
            }
            l = (l + 0.02).min(1.0);
        }
    }

    let (fr, fg, fb) = hsl_to_rgb(h, s, l);
    let fill = format!("rgb({fr}, {fg}, {fb})");
    let text = if dark_mode { "#fff" } else { "#000" };
    (fill, text)
}

fn rgb_to_hsl(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
    let r = r as f64 / 255.0;
    let g = g as f64 / 255.0;
    let b = b as f64 / 255.0;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let l = (max + min) / 2.0;
    if (max - min).abs() < 1e-10 {
        return (0.0, 0.0, l);
    }
    let d = max - min;
    let s = if l > 0.5 { d / (2.0 - max - min) } else { d / (max + min) };
    let h = if (max - r).abs() < 1e-10 {
        ((g - b) / d + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if (max - g).abs() < 1e-10 {
        ((b - r) / d + 2.0) / 6.0
    } else {
        ((r - g) / d + 4.0) / 6.0
    };
    (h, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    if s.abs() < 1e-10 {
        let v = (l * 255.0).round() as u8;
        return (v, v, v);
    }
    fn hue_to_rgb(p: f64, q: f64, mut t: f64) -> f64 {
        if t < 0.0 { t += 1.0; }
        if t > 1.0 { t -= 1.0; }
        if t < 1.0 / 6.0 { return p + (q - p) * 6.0 * t; }
        if t < 1.0 / 2.0 { return q; }
        if t < 2.0 / 3.0 { return p + (q - p) * (2.0 / 3.0 - t) * 6.0; }
        p
    }
    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;
    let r = (hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0).round() as u8;
    let g = (hue_to_rgb(p, q, h) * 255.0).round() as u8;
    let b = (hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0).round() as u8;
    (r, g, b)
}

fn mindmap_section_css(theme: &MermaidTheme) -> String {
    let colors = &theme.git_branch_colors;
    let mut css = String::new();

    let emit = |css: &mut String, selector: &str, color: &str| {
        let txt = text_color_for_bg(color);
        css.push_str(&format!(
            "{selector} rect, {selector} path, {selector} circle, {selector} polygon \
             {{ fill: {color} !important; }}\n\
             {selector} text, {selector} span \
             {{ fill: {txt} !important; color: {txt} !important; }}\n\
             {selector} foreignObject div, {selector} foreignObject span, {selector} foreignObject p \
             {{ color: {txt} !important; }}\n\
             .section-edge{} {{ stroke: {color} !important; }}\n",
            selector.trim_start_matches(".section"),
        ));
    };

    emit(&mut css, ".section-root.section--1", &colors[0]);
    emit(&mut css, ".section--1", &colors[1]);

    for i in 0..=10 {
        let ci = 2 + (i % 6);
        emit(&mut css, &format!(".section-{i}"), &colors[ci]);
    }

    css
}

fn git_branch_css(theme: &MermaidTheme) -> String {
    let mut css = String::new();
    for i in 0..8 {
        let c = &theme.git_branch_colors[i];
        let lbl = &theme.git_branch_label_colors[i];
        css.push_str(&format!(
            ".commit{i} {{ stroke: {c}; fill: {c}; }}
             .arrow{i} {{ stroke: {c}; }}
             .label{i} {{ fill: {c}; }}
             .branch-label{i} {{ fill: {lbl}; }}\n"
        ));
    }
    css
}

fn to_merman_config(theme: &MermaidTheme) -> merman::MermaidConfig {
    merman::MermaidConfig::from_value(serde_json::json!({
        "theme": "base",
        "darkMode": theme.dark_mode,
        "themeVariables": {
            "primaryColor": theme.primary_color,
            "primaryTextColor": theme.primary_text_color,
            "primaryBorderColor": theme.primary_border_color,
            "lineColor": theme.line_color,
            "secondaryColor": theme.secondary_color,
            "tertiaryColor": theme.tertiary_color,
            "background": theme.background,
            "mainBkg": theme.primary_color,
            "nodeBorder": theme.primary_border_color,
            "clusterBkg": theme.cluster_background,
            "clusterBorder": theme.cluster_border,
            "titleColor": theme.text_color,
            "edgeLabelBackground": theme.edge_label_background,
            "textColor": theme.text_color,
            "fontFamily": theme.font_family,
            "noteBkgColor": theme.note_background,
            "noteBorderColor": theme.note_border,
            "actorBkg": theme.actor_background,
            "actorBorder": theme.actor_border,
            "actorTextColor": theme.primary_text_color,
            "activationBkgColor": theme.activation_background,
            "activationBorderColor": theme.activation_border,
            "attributeBackgroundColorOdd": theme.er_attr_bg_odd,
            "attributeBackgroundColorEven": theme.er_attr_bg_even,
            "cScale0": theme.git_branch_colors[0],
            "cScale1": theme.git_branch_colors[1],
            "cScale2": theme.git_branch_colors[2],
            "cScale3": theme.git_branch_colors[3],
            "cScale4": theme.git_branch_colors[4],
            "cScale5": theme.git_branch_colors[5],
            "cScale6": theme.git_branch_colors[6],
            "cScale7": theme.git_branch_colors[7],
            "cScaleLabel0": theme.git_branch_label_colors[0],
            "cScaleLabel1": theme.git_branch_label_colors[1],
            "cScaleLabel2": theme.git_branch_label_colors[2],
            "cScaleLabel3": theme.git_branch_label_colors[3],
            "cScaleLabel4": theme.git_branch_label_colors[4],
            "cScaleLabel5": theme.git_branch_label_colors[5],
            "cScaleLabel6": theme.git_branch_label_colors[6],
            "cScaleLabel7": theme.git_branch_label_colors[7],
            "pie1": theme.git_branch_colors[0],
            "pie2": theme.git_branch_colors[1],
            "pie3": theme.git_branch_colors[2],
            "pie4": theme.git_branch_colors[3],
            "pie5": theme.git_branch_colors[4],
            "pie6": theme.git_branch_colors[5],
            "pie7": theme.git_branch_colors[6],
            "pie8": theme.git_branch_colors[7],
            "pieTitleTextColor": theme.text_color,
            "pieSectionTextColor": theme.text_color,
            "pieLegendTextColor": theme.text_color,
            "pieStrokeColor": theme.primary_border_color,
            "pieOuterStrokeColor": theme.primary_border_color,
        }
    }))
}

fn to_mermaid_rs_theme(theme: &MermaidTheme) -> mermaid_rs_renderer::Theme {
    let pie_colors: [String; 12] =
        std::array::from_fn(|i| theme.git_branch_colors[i % theme.git_branch_colors.len()].clone());

    mermaid_rs_renderer::Theme {
        font_family: theme.font_family.clone(),
        background: theme.background.clone(),
        text_color: theme.text_color.clone(),
        primary_color: theme.primary_color.clone(),
        primary_text_color: theme.primary_text_color.clone(),
        primary_border_color: theme.primary_border_color.clone(),
        line_color: theme.line_color.clone(),
        secondary_color: theme.secondary_color.clone(),
        tertiary_color: theme.tertiary_color.clone(),
        edge_label_background: theme.edge_label_background.clone(),
        cluster_background: theme.cluster_background.clone(),
        cluster_border: theme.cluster_border.clone(),
        sequence_actor_fill: theme.actor_background.clone(),
        sequence_actor_border: theme.actor_border.clone(),
        sequence_actor_line: theme.line_color.clone(),
        sequence_note_fill: theme.note_background.clone(),
        sequence_note_border: theme.note_border.clone(),
        sequence_activation_fill: theme.activation_background.clone(),
        sequence_activation_border: theme.activation_border.clone(),
        pie_colors,
        pie_title_text_color: theme.text_color.clone(),
        pie_section_text_color: theme.text_color.clone(),
        pie_legend_text_color: theme.text_color.clone(),
        pie_stroke_color: theme.primary_border_color.clone(),
        pie_outer_stroke_color: theme.primary_border_color.clone(),
        git_colors: theme.git_branch_colors.clone(),
        git_inv_colors: theme.git_branch_colors.clone(),
        git_branch_label_colors: theme.git_branch_label_colors.clone(),
        git_commit_label_color: theme.text_color.clone(),
        git_commit_label_background: theme.edge_label_background.clone(),
        git_tag_label_color: theme.text_color.clone(),
        git_tag_label_background: theme.primary_color.clone(),
        git_tag_label_border: theme.primary_border_color.clone(),
        ..mermaid_rs_renderer::Theme::modern()
    }
}
