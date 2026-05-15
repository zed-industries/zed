use mermaid_render::{AccentColor, MermaidBackend, MermaidTheme};

#[test]
fn debug_accent_flowchart_svg() {
    let theme = MermaidTheme {
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
        git_branch_colors: std::array::from_fn(|_| "rgb(128,128,128)".to_string()),
        git_branch_label_colors: std::array::from_fn(|_| "rgb(255,255,255)".to_string()),
        er_attr_bg_odd: "rgb(47, 52, 62)".to_string(),
        er_attr_bg_even: "rgb(46, 52, 62)".to_string(),
        accent_colors: vec![
            AccentColor { stroke: "rgb(116, 173, 232)".into(), background: "rgb(116, 173, 232)".into() },
            AccentColor { stroke: "rgb(190, 80, 70)".into(), background: "rgb(190, 80, 70)".into() },
            AccentColor { stroke: "rgb(191, 149, 106)".into(), background: "rgb(191, 149, 106)".into() },
            AccentColor { stroke: "rgb(180, 119, 207)".into(), background: "rgb(180, 119, 207)".into() },
            AccentColor { stroke: "rgb(110, 180, 191)".into(), background: "rgb(110, 180, 191)".into() },
            AccentColor { stroke: "rgb(208, 114, 119)".into(), background: "rgb(208, 114, 119)".into() },
            AccentColor { stroke: "rgb(222, 193, 132)".into(), background: "rgb(222, 193, 132)".into() },
            AccentColor { stroke: "rgb(161, 193, 129)".into(), background: "rgb(161, 193, 129)".into() },
        ],
    };

    let source = r#"flowchart TD
    A([Customer Places Order]):::accent0 --> B[Validate Cart]:::accent1
    B --> C{Items In Stock?}:::accent2
    C -- No --> D[Notify Customer]:::accent3
    C -- Yes --> E[Charge Payment]:::accent4
    E --> F{Payment OK?}:::accent2
    F -- No --> D
    F -- Yes --> G[Fulfill Order]:::accent5
    G --> H[Ship Package]:::accent6
    H --> I([Delivery Complete]):::accent7"#;

    let svg = mermaid_render::render_to_svg(source, &theme, MermaidBackend::Merman)
        .expect("render failed");

    // Find ALL elements with problematic attributes using quick-xml
    use quick_xml::events::Event;
    let mut reader = quick_xml::Reader::from_str(&svg);
    let mut issues = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Eof) => break,
            Ok(Event::Start(e)) | Ok(Event::Empty(e)) => {
                let tag = String::from_utf8_lossy(e.name().local_name().as_ref()).to_string();
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.local_name().as_ref()).to_string();
                    let val = attr.unescape_value().unwrap_or_default();
                    if val.is_empty() || val.contains("NaN") {
                        issues.push(format!("<{tag}> {key}=\"{val}\""));
                    }
                }
            }
            Err(e) => { panic!("XML error: {e}"); }
            _ => {}
        }
    }

    for issue in &issues {
        eprintln!("ISSUE: {issue}");
    }

    // Also write the SVG for manual inspection
    let out_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/mermaid_debug");
    std::fs::create_dir_all(&out_dir).ok();
    std::fs::write(out_dir.join("accent_flowchart.svg"), &svg).ok();
    eprintln!("SVG written to target/mermaid_debug/accent_flowchart.svg");

    assert!(issues.is_empty(), "Found {} issues:\n{}", issues.len(), issues.join("\n"));
}
