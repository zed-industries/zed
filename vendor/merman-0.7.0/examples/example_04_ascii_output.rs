mod support;

use merman::ascii::{AsciiRenderOptions, HeadlessAsciiRenderer};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_04_ascii_output",
        r#"sequenceDiagram
    participant User
    participant System
    User->>System: Request ASCII output
    System-->>User: Rendered text
"#,
    )?;

    // Pass `-- --ascii` for terminals or logs that should avoid Unicode box drawing.
    let options = if std::env::args().any(|arg| arg == "--ascii") {
        AsciiRenderOptions::ascii()
    } else {
        AsciiRenderOptions::unicode()
    };
    let renderer = HeadlessAsciiRenderer::new()
        .with_strict_parsing()
        .with_ascii_options(options);
    let Some(text) = renderer.render_ascii_sync(&input)? else {
        return Err("no Mermaid diagram detected".into());
    };

    print!("{text}");
    Ok(())
}
