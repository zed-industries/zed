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
        git_branch_label_colors: std::array::from_fn(|_| mermaid_render::text_color_for_background(rgb(128, 128, 128))),
        er_attr_bg_odd: rgb(47, 52, 62),
        er_attr_bg_even: rgb(46, 52, 62),
        error_color: rgb(220, 38, 38),
        warning_color: rgb(217, 119, 6),
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

#[test]
fn class_diagram_fallback_text_uses_accent_classes() {
    let theme = base_theme(vec![
        accent(190, 80, 70),   // red
        accent(116, 173, 232), // blue
    ]);
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
    let theme = base_theme(vec![accent(190, 80, 70)]);
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






