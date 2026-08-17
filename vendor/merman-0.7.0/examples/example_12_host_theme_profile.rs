mod support;

use merman::render::{HeadlessRenderer, HostThemeProfile, HostThemeRoles};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = support::read_mermaid_or_default(
        "example_12_host_theme_profile",
        r#"sequenceDiagram
    participant Host
    participant Merman
    Host->>Merman: Render preview
    Note over Host,Merman: Host theme profile
"#,
    )?;

    let profile = HostThemeProfile::builder()
        .font_family("Inter, system-ui, sans-serif")
        .roles(HostThemeRoles {
            canvas: Some("#0f172a".to_string()),
            surface: Some("#111827".to_string()),
            surface_alt: Some("#1f2937".to_string()),
            text: Some("#e5e7eb".to_string()),
            border: Some("#475569".to_string()),
            line: Some("#94a3b8".to_string()),
            note_background: Some("#422006".to_string()),
            note_border: Some("#f59e0b".to_string()),
            note_text: Some("#fef3c7".to_string()),
            ..HostThemeRoles::default()
        })
        .series_palette(["#60a5fa", "#34d399", "#f59e0b"])
        .output(merman::render::HostThemeOutput::resvg_safe_editor())
        .build();

    let renderer = HeadlessRenderer::new()
        .with_host_theme(&profile)
        .with_vendored_text_measurer()
        .with_diagram_id("host-theme-profile-example");

    let Some(svg) = renderer.render_svg_sync(&input)? else {
        return Err("no Mermaid diagram detected".into());
    };

    print!("{svg}");
    Ok(())
}
