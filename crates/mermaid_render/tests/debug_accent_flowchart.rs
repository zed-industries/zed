use gpui::Hsla;
use mermaid_render::{AccentColor, MermaidTheme};

fn rgb(r: u8, g: u8, b: u8) -> Hsla {
    gpui::Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: 1.0,
    }
    .into()
}

fn base_theme(accent_colors: Vec<AccentColor>) -> MermaidTheme {
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
        git_branch_colors: std::array::from_fn(|_| rgb(128, 128, 128)),
        git_branch_label_colors: std::array::from_fn(|_| rgb(255, 255, 255)),
        er_attr_bg_odd: rgb(47, 52, 62),
        er_attr_bg_even: rgb(46, 52, 62),
        accent_colors,
    }
}

fn accent(r: u8, g: u8, b: u8) -> AccentColor {
    let c = rgb(r, g, b);
    AccentColor {
        foreground: c,
        background: c,
    }
}

#[test]
fn debug_accent_flowchart_svg() {
    let theme = base_theme(vec![
        accent(116, 173, 232),
        accent(190, 80, 70),
        accent(191, 149, 106),
        accent(180, 119, 207),
        accent(110, 180, 191),
        accent(208, 114, 119),
        accent(222, 193, 132),
        accent(161, 193, 129),
    ]);

    let source = r#"flowchart TD
    A([Customer Places Order]) --> B[Validate Cart]
    B --> C{Items In Stock?}
    C -- No --> D[Notify Customer]
    C -- Yes --> E[Charge Payment]
    E --> F{Payment OK?}
    F -- No --> D
    F -- Yes --> G[Fulfill Order]
    G --> H[Ship Package]
    H --> I([Delivery Complete])"#;

    let svg = mermaid_render::render_to_svg(source, &theme)
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
                    if (val.is_empty() || val.contains("NaN")) && key != "style" {
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
    std::fs::write(out_dir.join("auto_colored_flowchart.svg"), &svg).ok();
    eprintln!("SVG written to target/mermaid_debug/auto_colored_flowchart.svg");

    assert!(issues.is_empty(), "Found {} issues:\n{}", issues.len(), issues.join("\n"));
}

#[test]
fn generics_not_double_escaped() {
    let theme = base_theme(vec![accent(116, 173, 232)]);
    let source = "classDiagram\n    class Shelter {\n        -List~Animal~ animals\n        +adopt(Animal a) bool\n    }";
    let svg = mermaid_render::render_to_svg(source, &theme)
        .expect("render failed");
    assert!(!svg.contains("&amp;lt;"), "Double-escaped &amp;lt; found in SVG");
    assert!(!svg.contains("&amp;gt;"), "Double-escaped &amp;gt; found in SVG");
}

#[test]
fn backslash_n_converted_to_line_break() {
    let theme = base_theme(vec![accent(116, 173, 232)]);
    let source = r#"graph TD
    L7["Layer 7\nHTTP, FTP"]
    L6["Layer 6\nEncryption"]
    L7 --> L6"#;
    let svg = mermaid_render::render_to_svg(source, &theme)
        .expect("render failed");
    assert!(
        !svg.contains(r"\n"),
        "Literal \\n should not appear in SVG output"
    );
    assert!(
        svg.contains(">Layer 7<") && svg.contains(">HTTP, FTP<"),
        "Label lines should be split into separate <text> elements"
    );
}
