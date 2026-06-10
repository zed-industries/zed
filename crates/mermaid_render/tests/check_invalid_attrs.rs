use gpui::Hsla;
use mermaid_render::MermaidTheme;

fn rgb(r: u8, g: u8, b: u8) -> Hsla {
    gpui::Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
    .into()
}

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
    ("pie", "pie title Test\n    \"A\" : 42\n    \"B\" : 58"),
    (
        "gantt",
        "gantt\n    title Test\n    dateFormat YYYY-MM-DD\n    section S\n        Task :a1, 2025-01-01, 7d",
    ),
    ("mindmap", "mindmap\n  root((Root))\n    Child1\n    Child2"),
    (
        "journey",
        "journey\n    title Test\n    section S\n        Task: 5: Actor",
    ),
    (
        "gitgraph",
        "gitGraph\n    commit id: \"init\"\n    branch dev\n    commit id: \"feat\"\n    checkout main\n    merge dev",
    ),
    (
        "quadrant",
        "quadrantChart\n    title Test\n    x-axis Low --> High\n    y-axis Low --> High\n    A: [0.3, 0.8]\n    B: [0.7, 0.4]",
    ),
    (
        "timeline",
        "timeline\n    title Test\n    section 2020s\n        2020 : Event A\n        2022 : Event B",
    ),
    (
        "xychart",
        "xychart-beta\n    title Test\n    x-axis [\"A\", \"B\", \"C\"]\n    y-axis \"Val\" 0 --> 10\n    bar [3, 7, 5]",
    ),
];

fn rgb_theme() -> MermaidTheme {
    MermaidTheme {
        dark_mode: true,
        font_family: "system-ui".to_string(),
        background: rgb(40, 44, 51),
        primary_color: rgb(47, 52, 62),
        primary_text_color: rgb(220, 224, 229),
        primary_border_color: rgb(70, 75, 87),
        secondary_color: rgb(46, 52, 62),
        tertiary_color: rgb(54, 60, 70),
        line_color: rgb(70, 75, 87),
        text_color: rgb(220, 224, 229),
        edge_label_background: rgb(40, 44, 51),
        cluster_background: rgb(47, 52, 62),
        cluster_border: rgb(54, 60, 70),
        note_background: rgb(47, 52, 62),
        note_border: rgb(54, 60, 70),
        actor_background: rgb(46, 52, 62),
        actor_border: rgb(70, 75, 87),
        activation_background: rgb(54, 60, 70),
        activation_border: rgb(70, 75, 87),
        git_branch_colors: [
            rgb(116, 173, 232),
            rgb(190, 80, 70),
            rgb(191, 149, 106),
            rgb(180, 119, 207),
            rgb(110, 180, 191),
            rgb(208, 114, 119),
            rgb(222, 193, 132),
            rgb(161, 193, 129),
        ],
        git_branch_label_colors: [
            rgb(116, 173, 232),
            rgb(190, 80, 70),
            rgb(191, 149, 106),
            rgb(180, 119, 207),
            rgb(110, 180, 191),
            rgb(208, 114, 119),
            rgb(222, 193, 132),
            rgb(161, 193, 129),
        ]
        .map(mermaid_render::text_color_for_background),
        er_attr_bg_odd: rgb(47, 52, 62),
        er_attr_bg_even: rgb(46, 52, 62),
        error_color: rgb(220, 38, 38),
        warning_color: rgb(217, 119, 6),
        accent_colors: vec![
            mermaid_render::AccentColor {
                foreground: rgb(116, 173, 232),
                background: rgb(116, 173, 232),
            },
            mermaid_render::AccentColor {
                foreground: rgb(190, 80, 70),
                background: rgb(190, 80, 70),
            },
            mermaid_render::AccentColor {
                foreground: rgb(191, 149, 106),
                background: rgb(191, 149, 106),
            },
            mermaid_render::AccentColor {
                foreground: rgb(180, 119, 207),
                background: rgb(180, 119, 207),
            },
            mermaid_render::AccentColor {
                foreground: rgb(110, 180, 191),
                background: rgb(110, 180, 191),
            },
            mermaid_render::AccentColor {
                foreground: rgb(208, 114, 119),
                background: rgb(208, 114, 119),
            },
            mermaid_render::AccentColor {
                foreground: rgb(222, 193, 132),
                background: rgb(222, 193, 132),
            },
            mermaid_render::AccentColor {
                foreground: rgb(161, 193, 129),
                background: rgb(161, 193, 129),
            },
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
                        "fill"
                            | "stroke"
                            | "width"
                            | "height"
                            | "x"
                            | "y"
                            | "r"
                            | "cx"
                            | "cy"
                            | "rx"
                            | "ry"
                            | "stroke-width"
                    );
                    if visual_attr && val.is_empty() {
                        issues.push(format!("{name}: <{tag}> has empty {key}=\"\"\n"));
                    }
                    // Check for CSS length units that usvg can't parse
                    if visual_attr
                        && matches!(key.as_str(), "width" | "height")
                        && val.ends_with("px")
                    {
                        issues.push(format!("{name}: <{tag}> has {key}=\"{val}\" (px suffix)\n"));
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
fn accent_colors_auto_applied_to_nodes() {
    let theme = rgb_theme();

    // A plain state diagram with no :::accent syntax should get
    // automatic accent colors applied to its node groups.
    let source = "stateDiagram-v2\n    [*] --> Idle\n    Idle --> Processing\n    Processing --> Done\n    Done --> [*]";

    let svg = mermaid_render::render_to_svg(source, &theme).expect("render failed");

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
fn generics_not_double_escaped() {
    let theme = rgb_theme();
    let source = "classDiagram\n    class Shelter {\n        -List~Animal~ animals\n        +adopt(Animal a) bool\n    }";
    let svg = mermaid_render::render_to_svg(source, &theme).expect("render failed");
    assert!(
        !svg.contains("&amp;lt;"),
        "Double-escaped &amp;lt; found in SVG"
    );
    assert!(
        !svg.contains("&amp;gt;"),
        "Double-escaped &amp;gt; found in SVG"
    );
}

#[test]
fn backslash_n_converted_to_line_break() {
    let theme = rgb_theme();
    let source = r#"graph TD
    L7["Layer 7\nHTTP, FTP"]
    L6["Layer 6\nEncryption"]
    L7 --> L6"#;
    let svg = mermaid_render::render_to_svg(source, &theme).expect("render failed");
    assert!(
        !svg.contains(r"\n"),
        "Literal \\n should not appear in SVG output"
    );
    assert!(
        svg.contains(">Layer 7<") && svg.contains(">HTTP, FTP<"),
        "Label lines should be split into separate <text> elements"
    );
}

#[test]
fn class_diagram_fallback_text_uses_accent_classes() {
    let theme = rgb_theme();
    let source = r#"classDiagram
    class Animal {
        +String name
        +makeSound() void
    }
    class Dog {
        +String breed
        +bark() void
    }
    Dog --|> Animal"#;

    let svg = mermaid_render::render_to_svg(source, &theme).expect("render failed");

    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(&svg);
    let mut in_fallback = false;
    let mut accent_classes: Vec<String> = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) => {
                if e.name().as_ref() == b"g" {
                    if let Ok(Some(attr)) = e.try_get_attribute("data-merman-foreignobject") {
                        if attr.value.as_ref() == b"fallback" {
                            in_fallback = true;
                        }
                    }
                }
                if in_fallback && e.name().as_ref() == b"text" {
                    if let Ok(Some(class_attr)) = e.try_get_attribute("class") {
                        let class = class_attr.unescape_value().unwrap_or_default().to_string();
                        for token in class.split_whitespace() {
                            if token.starts_with("zed-accent-") {
                                accent_classes.push(token.to_string());
                            }
                        }
                    }
                }
            }
            Ok(Event::End(e)) if e.name().as_ref() == b"g" => {
                in_fallback = false;
            }
            _ => {}
        }
    }

    assert!(
        !accent_classes.is_empty(),
        "expected zed-accent-N classes on text elements in fallback groups",
    );
}

#[test]
fn sequence_diagram_tspan_uses_accent_classes() {
    let theme = rgb_theme();
    let source = "sequenceDiagram\n    participant Database";
    let svg = mermaid_render::render_to_svg(source, &theme).expect("render failed");

    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(&svg);
    let mut accent_classes: Vec<String> = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) if e.name().as_ref() == b"tspan" => {
                if let Ok(Some(class_attr)) = e.try_get_attribute("class") {
                    let class = class_attr.unescape_value().unwrap_or_default().to_string();
                    for token in class.split_whitespace() {
                        if token.starts_with("zed-accent-") {
                            accent_classes.push(token.to_string());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    assert!(
        !accent_classes.is_empty(),
        "expected zed-accent-N classes on tspan elements in sequence diagram",
    );
}

#[test]
fn no_empty_attributes_or_nan_with_rgb_theme() {
    let theme = rgb_theme();
    let mut all_issues = Vec::new();

    for (name, source) in DIAGRAMS {
        match mermaid_render::render_to_svg(source, &theme) {
            Ok(svg) => all_issues.extend(check_svg_issues(name, &svg)),
            Err(e) => eprintln!("{name}: render failed (skipped): {e}"),
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
