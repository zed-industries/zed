use mermaid_render::{MermaidBackend, MermaidTheme};

const DIAGRAMS: &[(&str, &str)] = &[
    (
        "flowchart",
        "flowchart TD\n    A[Hello] --> B[World]\n    B --> C{Decision}\n    C -->|Yes| D[OK]\n    C -->|No| E[Fail]",
    ),
    (
        "sequence",
        "sequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi\n    Note over Alice,Bob: A note",
    ),
    (
        "state",
        "stateDiagram-v2\n    [*] --> Active\n    Active --> [*]",
    ),
    (
        "er",
        "erDiagram\n    A { int id PK }\n    B { int id PK }\n    A ||--o{ B : has",
    ),
    (
        "class",
        "classDiagram\n    class Foo {\n        +bar() void\n    }",
    ),
    (
        "pie",
        "pie title Test\n    \"A\" : 42\n    \"B\" : 58",
    ),
    (
        "gantt",
        "gantt\n    title Test\n    dateFormat YYYY-MM-DD\n    section S\n        Task :a1, 2025-01-01, 7d",
    ),
    (
        "mindmap",
        "mindmap\n  root((Root))\n    Child1\n    Child2",
    ),
    (
        "journey",
        "journey\n    title Test\n    section S\n        Task: 5: Actor",
    ),
    (
        "gitgraph",
        "gitGraph\n    commit id: \"init\"\n    branch dev\n    commit id: \"feat\"\n    checkout main\n    merge dev",
    ),
];

fn rgb_theme() -> MermaidTheme {
    MermaidTheme {
        dark_mode: true,
        font_family: "system-ui".to_string(),
        background: "rgb(40, 44, 51)".to_string(),
        primary_color: "rgb(47, 52, 62)".to_string(),
        primary_text_color: "rgb(220, 224, 229)".to_string(),
        primary_border_color: "rgb(70, 75, 87)".to_string(),
        secondary_color: "rgb(46, 52, 62)".to_string(),
        tertiary_color: "rgb(54, 60, 70)".to_string(),
        line_color: "rgb(70, 75, 87)".to_string(),
        text_color: "rgb(220, 224, 229)".to_string(),
        edge_label_background: "rgb(40, 44, 51)".to_string(),
        cluster_background: "rgb(47, 52, 62)".to_string(),
        cluster_border: "rgb(54, 60, 70)".to_string(),
        note_background: "rgb(47, 52, 62)".to_string(),
        note_border: "rgb(54, 60, 70)".to_string(),
        actor_background: "rgb(46, 52, 62)".to_string(),
        actor_border: "rgb(70, 75, 87)".to_string(),
        activation_background: "rgb(54, 60, 70)".to_string(),
        activation_border: "rgb(70, 75, 87)".to_string(),
        git_branch_colors: [
            "rgb(116, 173, 232)".to_string(),
            "rgb(190, 80, 70)".to_string(),
            "rgb(191, 149, 106)".to_string(),
            "rgb(180, 119, 207)".to_string(),
            "rgb(110, 180, 191)".to_string(),
            "rgb(208, 114, 119)".to_string(),
            "rgb(222, 193, 132)".to_string(),
            "rgb(161, 193, 129)".to_string(),
        ],
        git_branch_label_colors: std::array::from_fn(|_| "rgb(255, 255, 255)".to_string()),
        er_attr_bg_odd: "rgb(47, 52, 62)".to_string(),
        er_attr_bg_even: "rgb(46, 52, 62)".to_string(),
        accent_colors: vec![
            mermaid_render::AccentColor { stroke: "rgb(116, 173, 232)".into(), background: "rgb(116, 173, 232)".into() },
            mermaid_render::AccentColor { stroke: "rgb(190, 80, 70)".into(), background: "rgb(190, 80, 70)".into() },
            mermaid_render::AccentColor { stroke: "rgb(191, 149, 106)".into(), background: "rgb(191, 149, 106)".into() },
            mermaid_render::AccentColor { stroke: "rgb(180, 119, 207)".into(), background: "rgb(180, 119, 207)".into() },
            mermaid_render::AccentColor { stroke: "rgb(110, 180, 191)".into(), background: "rgb(110, 180, 191)".into() },
            mermaid_render::AccentColor { stroke: "rgb(208, 114, 119)".into(), background: "rgb(208, 114, 119)".into() },
            mermaid_render::AccentColor { stroke: "rgb(222, 193, 132)".into(), background: "rgb(222, 193, 132)".into() },
            mermaid_render::AccentColor { stroke: "rgb(161, 193, 129)".into(), background: "rgb(161, 193, 129)".into() },
        ],
    }
}

fn check_svg_issues(name: &str, svg: &str) -> Vec<String> {
    let bad_patterns = [
        "fill=\"\"",
        "stroke=\"\"",
        "width=\"\"",
        "height=\"\"",
        "NaN",
        // Also check for empty values in style attributes
        "fill: ;",
        "fill:;",
        "stroke: ;",
        "stroke:;",
        // Check for attributes with just whitespace
        "fill=\" \"",
    ];
    let mut issues = Vec::new();
    for pattern in &bad_patterns {
        let mut start = 0;
        while let Some(pos) = svg[start..].find(pattern) {
            let abs = start + pos;
            let ctx_start = abs.saturating_sub(100);
            let ctx_end = (abs + pattern.len() + 60).min(svg.len());
            issues.push(format!(
                "{name}: found `{pattern}` at byte {abs}:\n  ...{}...\n",
                &svg[ctx_start..ctx_end]
            ));
            start = abs + pattern.len();
        }
    }

    // Parse with quick-xml to find ANY empty attribute values on visual elements
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(svg);
    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let tag = String::from_utf8_lossy(e.name().local_name().as_ref()).to_string();
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.local_name().as_ref()).to_string();
                    let val = attr.unescape_value().unwrap_or_default();
                    let visual_attr = matches!(
                        key.as_str(),
                        "fill" | "stroke" | "width" | "height" | "x" | "y" | "r"
                            | "cx" | "cy" | "rx" | "ry" | "stroke-width"
                    );
                    if visual_attr && val.is_empty() {
                        issues.push(format!(
                            "{name}: <{tag}> has empty {key}=\"\"\n"
                        ));
                    }
                    // Check for CSS length units that usvg can't parse
                    if visual_attr
                        && matches!(key.as_str(), "width" | "height")
                        && val.ends_with("px")
                    {
                        issues.push(format!(
                            "{name}: <{tag}> has {key}=\"{val}\" (px suffix)\n"
                        ));
                    }
                }
            }
            Err(e) => {
                issues.push(format!("{name}: XML parse error: {e}\n"));
                break;
            }
            _ => {}
        }
    }

    issues
}

#[test]
fn no_empty_attributes_or_nan_in_merman_output() {
    let theme = MermaidTheme::default();
    let bad_patterns = ["fill=\"\"", "stroke=\"\"", "width=\"\"", "height=\"\"", "NaN"];

    let mut all_issues = Vec::new();

    for (name, source) in DIAGRAMS {
        let svg = mermaid_render::render_to_svg(source, &theme, MermaidBackend::Merman)
            .unwrap_or_else(|e| panic!("{name}: render failed: {e}"));

        for pattern in &bad_patterns {
            let mut start = 0;
            while let Some(pos) = svg[start..].find(pattern) {
                let abs = start + pos;
                let ctx_start = abs.saturating_sub(100);
                let ctx_end = (abs + pattern.len() + 60).min(svg.len());
                all_issues.push(format!(
                    "{name}: found `{pattern}` at byte {abs}:\n  ...{}...\n",
                    &svg[ctx_start..ctx_end]
                ));
                start = abs + pattern.len();
            }
        }
    }

    if !all_issues.is_empty() {
        panic!(
            "Found {} issues in merman SVG output:\n\n{}",
            all_issues.len(),
            all_issues.join("\n")
        );
    }
}

#[test]
fn accent_colors_auto_applied_to_nodes() {
    let theme = rgb_theme();

    // A plain state diagram with no :::accent syntax should get
    // automatic accent colors applied to its node groups.
    let source = "stateDiagram-v2\n    [*] --> Idle\n    Idle --> Processing\n    Processing --> Done\n    Done --> [*]";

    let svg = mermaid_render::render_to_svg(source, &theme, MermaidBackend::Merman)
        .expect("render failed");

    // accent_fill_and_text darkens the background color for dark mode.
    // The stroke colors are direct hex conversions of the accent rgb values.
    // With 3 states (Idle, Processing, Done), we expect at least accent0 and
    // accent1 stroke colors to appear.
    let accent0_stroke = "#74ade8"; // rgb(116, 173, 232) -> hex
    let accent1_stroke = "#be5046"; // rgb(190, 80, 70) -> hex

    assert!(
        svg.contains(accent0_stroke),
        "Expected accent0 stroke color ({accent0_stroke}) in auto-colored state diagram SVG.\n\
         This means auto-coloring did not apply accent colors to node groups.\n\
         SVG snippet: {}...",
        &svg[..svg.len().min(2000)]
    );
    assert!(
        svg.contains(accent1_stroke),
        "Expected accent1 stroke color ({accent1_stroke}) in auto-colored state diagram SVG."
    );
}

#[test]
fn no_empty_attributes_or_nan_with_rgb_theme() {
    let theme = rgb_theme();
    let mut all_issues = Vec::new();

    for (name, source) in DIAGRAMS {
        match mermaid_render::render_to_svg(source, &theme, MermaidBackend::Merman) {
            Ok(svg) => all_issues.extend(check_svg_issues(name, &svg)),
            Err(e) => eprintln!("{name}: render failed (skipped): {e}"),
        }
    }

    // Also test the full corpus files if available
    let corpus_dir =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/corpus");
    if corpus_dir.exists() {
        let mut corpus_files: Vec<_> = std::fs::read_dir(&corpus_dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().is_some_and(|ext| ext == "mmd"))
            .collect();
        corpus_files.sort_by_key(|e| e.file_name());

        for entry in &corpus_files {
            let path = entry.path();
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            let source = std::fs::read_to_string(&path).unwrap();
            match mermaid_render::render_to_svg(&source, &theme, MermaidBackend::Merman) {
                Ok(svg) => all_issues.extend(check_svg_issues(&name, &svg)),
                Err(e) => eprintln!("corpus/{name}.mmd: render failed: {e}"),
            }
        }
    }

    if !all_issues.is_empty() {
        panic!(
            "Found {} issues in merman SVG output (rgb theme):\n\n{}",
            all_issues.len(),
            all_issues.join("\n")
        );
    }
}
