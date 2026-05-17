#![recursion_limit = "256"]

use anyhow::{Context as _, Result, anyhow};

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
        // Values mirror the previous `mermaid_rs_renderer::Theme::modern()` defaults so existing
        // tests and call sites that relied on those colors continue to work.
        const GIT_BRANCH_COLORS: [&str; 8] = [
            "hsl(240, 100%, 46.2745098039%)",
            "hsl(60, 100%, 43.5294117647%)",
            "hsl(80, 100%, 46.2745098039%)",
            "hsl(210, 100%, 46.2745098039%)",
            "hsl(180, 100%, 46.2745098039%)",
            "hsl(150, 100%, 46.2745098039%)",
            "hsl(300, 100%, 46.2745098039%)",
            "hsl(0, 100%, 46.2745098039%)",
        ];
        const GIT_BRANCH_LABEL_COLORS: [&str; 8] = [
            "#ffffff", "black", "black", "#ffffff", "black", "black", "black", "black",
        ];

        Self {
            dark_mode: false,
            font_family: "Inter, ui-sans-serif, system-ui, -apple-system, \"Segoe UI\", \"DejaVu Sans\", \"Liberation Sans\", sans-serif, \"Noto Color Emoji\", \"Apple Color Emoji\", \"Segoe UI Emoji\"".to_string(),
            background: "#FFFFFF".to_string(),
            primary_color: "#F8FAFC".to_string(),
            primary_text_color: "#0F172A".to_string(),
            primary_border_color: "#94A3B8".to_string(),
            secondary_color: "#E2E8F0".to_string(),
            tertiary_color: "#FFFFFF".to_string(),
            line_color: "#64748B".to_string(),
            text_color: "#0F172A".to_string(),
            edge_label_background: "#FFFFFF".to_string(),
            cluster_background: "#F1F5F9".to_string(),
            cluster_border: "#CBD5E1".to_string(),
            note_background: "#FFF7ED".to_string(),
            note_border: "#FDBA74".to_string(),
            actor_background: "#F8FAFC".to_string(),
            actor_border: "#94A3B8".to_string(),
            activation_background: "#E2E8F0".to_string(),
            activation_border: "#94A3B8".to_string(),
            git_branch_colors: GIT_BRANCH_COLORS.map(|value| value.to_string()),
            git_branch_label_colors: GIT_BRANCH_LABEL_COLORS.map(|value| value.to_string()),
            er_attr_bg_odd: "#94A3B8".to_string(),
            er_attr_bg_even: "#0F172A".to_string(),
            accent_colors: Vec::new(),
        }
    }
}

pub fn render_to_svg(source: &str, theme: &MermaidTheme) -> Result<String> {
    render_with_merman(source, theme)
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

    let svg = renderer
        .render_svg_sync(source)
        .context("merman render failed")?
        .ok_or_else(|| anyhow!("merman returned no SVG for the given input"))?;

    // Merman doesn't interpret \n as a line break in labels — it passes
    // the literal backslash-n through to the foreignObject HTML. Convert
    // to <br/> so the fallback function can split them into separate lines.
    let svg = svg.replace(r"\n", "<br/>");

    // Word-wrap foreignObject text that is wider than its container.
    // Merman's text measurer can underestimate width, causing overflow
    // in the fallback <text> elements (which don't support CSS wrapping).
    let svg = wrap_foreignobject_labels(&svg);

    // Convert foreignObject labels to plain SVG <text> elements so that
    // renderers like usvg (which don't support foreignObject) can display them.
    let svg = merman::render::foreign_object_label_fallback_svg_text(&svg);

    // Fix double-escaping in fallback text: merman's fallback strips HTML
    // tags but preserves HTML entities (e.g. `&lt;`), then XML-escapes the
    // result, turning `&lt;` into `&amp;lt;`. This matters for mermaid
    // generics like `List~Animal~` which render as `List<Animal>`.
    let svg = svg
        .replace("&amp;lt;", "&lt;")
        .replace("&amp;gt;", "&gt;");

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
        .node rect, .node path {{ fill: {primary} !important; stroke: {border} !important; }}
        .node polygon {{ fill: {primary} !important; stroke: {border} !important; }}
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
    let mut in_fallback_group = false;
    let mut skip_element = false;

    let accent_styles: Vec<AccentStyle> = theme
        .accent_colors
        .iter()
        .map(|accent| {
            let (fill, text) = accent_fill_and_text(&accent.background, theme.dark_mode);
            let stroke = to_hex(&accent.stroke);
            AccentStyle {
                fill,
                stroke,
                text: text.to_string(),
            }
        })
        .collect();
    let mut accent_g_stack: Vec<Option<usize>> = Vec::new();
    let mut node_accent_counter: usize = 0;

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
                            let mut g_accent_idx: Option<usize> = None;
                            if let Some(class_attr) = e.try_get_attribute("class")? {
                                let class_val = class_attr.unescape_value()?;
                                if class_val.as_ref() == "plot" {
                                    in_plot_group = true;
                                    plot_g_depth = 1;
                                    plot_path_done = false;
                                } else if class_val.as_ref() == "legend" {
                                    in_legend = true;
                                }
                                if !accent_styles.is_empty()
                                    && has_class_token(&class_val, "node")
                                    && !has_class_token(&class_val, "mindmap-node")
                                {
                                    let idx =
                                        node_accent_counter % accent_styles.len();
                                    node_accent_counter += 1;
                                    g_accent_idx = Some(idx);
                                }
                            }
                            if let Some(attr) = e.try_get_attribute("data-merman-foreignobject")? {
                                if attr.unescape_value()?.as_ref() == "fallback" {
                                    in_fallback_group = true;
                                }
                            }
                            if is_start {
                                accent_g_stack.push(g_accent_idx);
                            }
                            None
                        }

                        b"rect" if in_fallback_group => {
                            in_fallback_group = false;
                            let mut ne = BytesStart::new("rect");
                            for attr in e.attributes() {
                                let attr = attr?;
                                if attr.key.local_name().as_ref() == b"fill" {
                                    ne.push_attribute((
                                        "fill",
                                        theme.edge_label_background.as_str(),
                                    ));
                                } else {
                                    ne.push_attribute(attr);
                                }
                            }
                            Some(ne)
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
                            let height_val = e
                                .try_get_attribute("height")?
                                .map(|a| a.unescape_value().map(|v| v.to_string()))
                                .transpose()?;
                            let bad_width =
                                matches!(width_val.as_deref(), Some("") | None);
                            let bad_height =
                                matches!(height_val.as_deref(), Some("") | None);
                            if bad_width || bad_height {
                                skip_element = true;
                                None
                            } else {
                                None
                            }
                        }

                        _ => None,
                    }
                };

                let elem = new_elem.unwrap_or(e);

                // Automatically apply accent colors to shapes and text
                // inside node groups. The accent index is assigned
                // round-robin from the theme's accent palette when
                // entering a <g class="node ..."> group.
                let elem = if !accent_styles.is_empty() {
                    let (is_shape, is_text, tag_string) = {
                        let tag = elem.name().local_name();
                        let tag_bytes = tag.as_ref();
                        (
                            matches!(
                                tag_bytes,
                                b"rect"
                                    | b"path"
                                    | b"circle"
                                    | b"polygon"
                                    | b"ellipse"
                            ),
                            tag_bytes == b"text",
                            String::from_utf8_lossy(tag_bytes).into_owned(),
                        )
                    };

                    if is_shape || is_text {
                        let accent_idx =
                            accent_g_stack.iter().rev().find_map(|x| *x);

                        if let Some(idx) = accent_idx {
                            let style = &accent_styles[idx];
                            let mut ne = BytesStart::new(tag_string);
                            let mut had_fill = false;
                            let mut had_stroke = false;

                            for attr in elem.attributes() {
                                let attr = attr.context("accent attr")?;
                                match attr.key.local_name().as_ref() {
                                    b"fill" => {
                                        had_fill = true;
                                        let color = if is_text {
                                            &style.text
                                        } else {
                                            &style.fill
                                        };
                                        ne.push_attribute(("fill", color.as_str()));
                                    }
                                    b"stroke" if is_shape => {
                                        had_stroke = true;
                                        ne.push_attribute((
                                            "stroke",
                                            style.stroke.as_str(),
                                        ));
                                    }
                                    _ => ne.push_attribute(attr),
                                }
                            }

                            if !had_fill {
                                let color = if is_text {
                                    &style.text
                                } else {
                                    &style.fill
                                };
                                ne.push_attribute(("fill", color.as_str()));
                            }
                            if is_shape && !had_stroke {
                                ne.push_attribute((
                                    "stroke",
                                    style.stroke.as_str(),
                                ));
                            }

                            ne
                        } else {
                            elem
                        }
                    } else {
                        elem
                    }
                } else {
                    elem
                };

                if skip_element {
                    skip_element = false;
                } else if is_start {
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

                if e.name().local_name().as_ref() == b"g" {
                    accent_g_stack.pop();
                    if in_plot_group {
                        plot_g_depth -= 1;
                        if plot_g_depth == 0 {
                            in_plot_group = false;
                        }
                    }
                }

                writer.write_event(Event::End(e))?;
            }

            event => writer.write_event(event)?,
        }
    }

    let svg = String::from_utf8(writer.into_inner()).context("SVG output is not valid UTF-8")?;
    let svg = sanitize_nan_colors(&svg);
    Ok(strip_unsupported_css(&svg))
}

/// Strip CSS rules that usvg's `simplecss` parser cannot handle:
/// `@keyframes` blocks, `:root` declarations, and any remaining `:not()`
/// selectors. These produce log warnings and are never applied.
fn strip_unsupported_css(svg: &str) -> String {
    let Some(style_start) = svg.find("<style>") else {
        return svg.to_string();
    };
    let content_start = style_start + "<style>".len();
    let Some(content_len) = svg[content_start..].find("</style>") else {
        return svg.to_string();
    };
    let content_end = content_start + content_len;
    let css = &svg[content_start..content_end];

    let mut cleaned = String::with_capacity(css.len());
    let bytes = css.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if css[i..].starts_with("@keyframes") {
            i = skip_css_block(bytes, i);
            continue;
        }

        let Some(brace_offset) = css[i..].find('{') else {
            cleaned.push_str(&css[i..]);
            break;
        };
        let selector = &css[i..i + brace_offset];
        if selector.contains(":root") || selector.contains(":not(") {
            i = skip_css_block(bytes, i);
            continue;
        }

        let block_end = skip_css_block(bytes, i);
        cleaned.push_str(&css[i..block_end]);
        i = block_end;
    }

    // Strip CSS angle units (`deg`) from rotate() — usvg parses these as
    // SVG transform values which use bare numbers, not CSS angle units.
    let cleaned = strip_css_angle_units(&cleaned);

    let mut result = String::with_capacity(svg.len());
    result.push_str(&svg[..content_start]);
    result.push_str(&cleaned);
    result.push_str(&svg[content_end..]);
    result
}

fn strip_css_angle_units(css: &str) -> String {
    let mut result = String::with_capacity(css.len());
    let mut remaining = css;
    while let Some(pos) = remaining.find("deg)") {
        result.push_str(&remaining[..pos]);
        result.push(')');
        remaining = &remaining[pos + 4..];
    }
    result.push_str(remaining);
    result
}

/// Insert `<br/>` at word boundaries in foreignObject text that is wider
/// than its container. This runs before the foreignObject-to-text fallback
/// conversion so that the fallback function splits the text into multiple
/// `<text>` elements.
fn wrap_foreignobject_labels(svg: &str) -> String {
    const AVG_CHAR_WIDTH: f64 = 8.5;
    // Merman's vendored text measurer underestimates character widths
    // by roughly 40%. Scale the foreignObject width up so we only wrap
    // text that genuinely overflows the node at actual rendering size.
    const WIDTH_SCALE: f64 = 1.4;

    let fo_tag = "<foreignObject";
    let fo_close = "</foreignObject>";

    let mut result = String::with_capacity(svg.len() + 256);
    let mut remaining = svg;

    while let Some(fo_start) = remaining.find(fo_tag) {
        let Some(tag_end) = remaining[fo_start..].find('>') else {
            break;
        };
        let tag = &remaining[fo_start..fo_start + tag_end + 1];

        let width = tag
            .find("width=\"")
            .and_then(|i| {
                let after = &tag[i + 7..];
                after.find('"').and_then(|end| after[..end].parse::<f64>().ok())
            })
            .unwrap_or(0.0);

        let content_start = fo_start + tag_end + 1;
        let Some(close_rel) = remaining[content_start..].find(fo_close) else {
            break;
        };
        let content_end = content_start + close_rel;
        let fo_end = content_end + fo_close.len();

        let available_width = width * WIDTH_SCALE;
        if available_width <= 0.0 {
            result.push_str(&remaining[..fo_end]);
            remaining = &remaining[fo_end..];
            continue;
        }

        let content = &remaining[content_start..content_end];

        // If the content already has explicit line breaks, skip wrapping.
        if content.contains("<br") {
            result.push_str(&remaining[..fo_end]);
            remaining = &remaining[fo_end..];
            continue;
        }

        // Extract plain text (strip HTML tags) for width estimation.
        let plain: String = {
            let mut text = String::new();
            let mut in_tag = false;
            for ch in content.chars() {
                match ch {
                    '<' => in_tag = true,
                    '>' => in_tag = false,
                    _ if !in_tag => text.push(ch),
                    _ => {}
                }
            }
            text.trim().to_string()
        };

        let estimated_width = plain.len() as f64 * AVG_CHAR_WIDTH;
        if estimated_width <= available_width || plain.split_whitespace().count() <= 1 {
            result.push_str(&remaining[..fo_end]);
            remaining = &remaining[fo_end..];
            continue;
        }

        // Build wrapped text with <br/> at word boundaries.
        let mut lines: Vec<String> = Vec::new();
        let mut current_line = String::new();
        for word in plain.split_whitespace() {
            let candidate_width = if current_line.is_empty() {
                word.len() as f64 * AVG_CHAR_WIDTH
            } else {
                (current_line.len() + 1 + word.len()) as f64 * AVG_CHAR_WIDTH
            };
            if !current_line.is_empty() && candidate_width > available_width {
                lines.push(current_line);
                current_line = word.to_string();
            } else if current_line.is_empty() {
                current_line = word.to_string();
            } else {
                current_line.push(' ');
                current_line.push_str(word);
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        let wrapped_html = lines.join("<br/>");

        // Replace the inner text in the original HTML.
        // Find the deepest text node and replace it.
        let new_content = if let Some(last_open) = content.rfind('>') {
            if let Some(next_close) = content[last_open..].find('<') {
                let text_start = last_open + 1;
                let text_end = last_open + next_close;
                let mut new = String::new();
                new.push_str(&content[..text_start]);
                new.push_str(&wrapped_html);
                new.push_str(&content[text_end..]);
                new
            } else {
                wrapped_html
            }
        } else {
            wrapped_html
        };

        result.push_str(&remaining[..content_start]);
        result.push_str(&new_content);
        result.push_str(&remaining[content_end..fo_end]);
        remaining = &remaining[fo_end..];
    }

    result.push_str(remaining);
    result
}

fn skip_css_block(bytes: &[u8], start: usize) -> usize {
    let mut depth = 0;
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            depth += 1;
        } else if bytes[i] == b'}' {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
        i += 1;
    }
    i
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

fn to_hex(color: &str) -> String {
    let (r, g, b) = parse_rgb(color).unwrap_or((128, 128, 128));
    format!("#{r:02x}{g:02x}{b:02x}")
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

fn has_class_token(class: &str, token: &str) -> bool {
    class.split_whitespace().any(|t| t == token)
}

struct AccentStyle {
    fill: String,
    stroke: String,
    text: String,
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
    let fill = format!("#{fr:02x}{fg:02x}{fb:02x}");
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
        "flowchart": {
            "padding": 16,
        },
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

            "quadrant1Fill": theme.primary_color,
            "quadrant2Fill": theme.primary_color,
            "quadrant3Fill": theme.primary_color,
            "quadrant4Fill": theme.primary_color,
            "quadrant1TextFill": theme.text_color,
            "quadrant2TextFill": theme.text_color,
            "quadrant3TextFill": theme.text_color,
            "quadrant4TextFill": theme.text_color,
            "quadrantPointFill": theme.line_color,
            "quadrantPointTextFill": theme.text_color,
            "quadrantTitleFill": theme.text_color,
            "quadrantXAxisTextFill": theme.text_color,
            "quadrantYAxisTextFill": theme.text_color,
            "quadrantExternalBorderStrokeFill": theme.primary_border_color,
            "quadrantInternalBorderStrokeFill": theme.primary_border_color,
        }
    }))
}


