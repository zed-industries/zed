use mermaid_render::MermaidTheme;

#[test]
fn debug_theme_propagation() {
    // Use obviously wrong colors so we can detect if they're applied
    let mut theme = MermaidTheme::default();
    theme.primary_color = gpui::rgb(0xff0000).into();
    theme.primary_text_color = gpui::rgb(0x00ff00).into();
    theme.primary_border_color = gpui::rgb(0x0000ff).into();
    theme.text_color = gpui::rgb(0x00ff00).into();

    let diagrams = [
        ("flowchart", "flowchart TD\n    A[Hello] --> B[World]"),
        (
            "sequence",
            "sequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi",
        ),
        (
            "er",
            "erDiagram\n    A { int id PK }\n    B { int id PK }\n    A ||--o{ B : has",
        ),
        (
            "state",
            "stateDiagram-v2\n    [*] --> Active\n    Active --> [*]",
        ),
    ];

    for (name, source) in diagrams {
        let svg = mermaid_render::render_to_svg(source, &theme).unwrap();
        let has_red = svg.contains("ff0000") || svg.contains("FF0000");
        let has_default = svg.contains("ECECFF");
        println!(
            "{name:12} | has our #ff0000: {has_red:5} | has default #ECECFF: {has_default:5} | size: {} bytes",
            svg.len()
        );
    }

    // Now test with inline init directive (bypasses site_config entirely)
    println!("\n--- With inline %%{{init}} directive ---");
    let default_theme = MermaidTheme::default();
    let diagrams_with_init = [
        ("flowchart", "%%{init: {\"theme\": \"base\", \"themeVariables\": {\"primaryColor\": \"#ff0000\", \"nodeBkg\": \"#ff0000\"}} }%%\nflowchart TD\n    A[Hello] --> B[World]"),
        ("sequence", "%%{init: {\"theme\": \"base\", \"themeVariables\": {\"primaryColor\": \"#ff0000\", \"actorBkg\": \"#ff0000\"}} }%%\nsequenceDiagram\n    Alice->>Bob: Hello\n    Bob-->>Alice: Hi"),
        ("er", "%%{init: {\"theme\": \"base\", \"themeVariables\": {\"primaryColor\": \"#ff0000\"}} }%%\nerDiagram\n    A { int id PK }\n    B { int id PK }\n    A ||--o{ B : has"),
        ("state", "%%{init: {\"theme\": \"base\", \"themeVariables\": {\"primaryColor\": \"#ff0000\"}} }%%\nstateDiagram-v2\n    [*] --> Active\n    Active --> [*]"),
    ];

    for (name, source) in diagrams_with_init {
        let svg = mermaid_render::render_to_svg(source, &default_theme).unwrap();
        let has_red = svg.contains("ff0000") || svg.contains("FF0000");
        let has_default = svg.contains("ECECFF");
        println!(
            "{name:12} | has our #ff0000: {has_red:5} | has default #ECECFF: {has_default:5}"
        );
    }
}
