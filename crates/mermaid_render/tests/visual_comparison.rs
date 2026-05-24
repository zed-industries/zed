use gpui::Hsla;
use mermaid_render::MermaidTheme;
use std::path::PathBuf;

fn hex(value: u32) -> Hsla {
    gpui::rgb(value).into()
}

fn one_dark_theme() -> MermaidTheme {
    let editor_background = hex(0x282c33);
    let surface_background = hex(0x2f343e);
    let text = hex(0xdce0e5);
    let border = hex(0x464b57);
    let border_variant = hex(0x363c46);
    let element_background = hex(0x2e343e);
    let ghost_element_hover = hex(0x363c46);
    let panel_background = hex(0x2f343e);

    let player_cursors = [
        hex(0x74ade8), hex(0xbe5046), hex(0xbf956a), hex(0xb477cf),
        hex(0x6eb4bf), hex(0xd07277), hex(0xdec184), hex(0xa1c181),
    ];

    let git_branch_colors = std::array::from_fn(|i| player_cursors[i % player_cursors.len()]);

    MermaidTheme {
        dark_mode: true,
        font_family: "Zed Plex Sans, system-ui".to_string(),
        background: editor_background,
        primary_color: surface_background,
        primary_text_color: text,
        primary_border_color: border,
        secondary_color: element_background,
        tertiary_color: ghost_element_hover,
        line_color: border,
        text_color: text,
        edge_label_background: editor_background,
        cluster_background: panel_background,
        cluster_border: border_variant,
        note_background: surface_background,
        note_border: border_variant,
        actor_background: element_background,
        actor_border: border,
        activation_background: ghost_element_hover,
        activation_border: border,
        git_branch_colors,
        git_branch_label_colors: git_branch_colors.map(mermaid_render::text_color_for_background),
        er_attr_bg_odd: surface_background,
        er_attr_bg_even: element_background,
        error_color: hex(0xdc2626),
        warning_color: hex(0xd97706),
        accent_colors: Vec::new(),
    }
}

#[test]
fn generate_visual_comparison() {
    let theme = one_dark_theme();

    let corpus_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/corpus");
    let output_dir =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/mermaid_visual_comparison");

    let mut corpus_files: Vec<_> = std::fs::read_dir(&corpus_dir)
        .expect("failed to read tests/corpus/ directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .is_some_and(|ext| ext == "mmd")
        })
        .collect();
    corpus_files.sort_by_key(|e| e.file_name());

    assert!(
        !corpus_files.is_empty(),
        "no .mmd files found in {}",
        corpus_dir.display()
    );

    std::fs::create_dir_all(&output_dir).expect("failed to create output dir");

    for entry in &corpus_files {
        let path = entry.path();
        let stem = path.file_stem().unwrap().to_string_lossy();
        let diagram_source =
            std::fs::read_to_string(&path).expect("failed to read corpus file");

        let filename = format!("{stem}.svg");
        let out_path = output_dir.join(&filename);

        match mermaid_render::render_to_svg(&diagram_source, &theme) {
            Ok(svg) => {
                std::fs::write(&out_path, &svg).expect("failed to write SVG");
                println!("OK   {filename}");
            }
            Err(err) => {
                let truncated = xml_escape(
                    &err.to_string().chars().take(120).collect::<String>(),
                );
                let error_svg = format!(
                    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"400\" height=\"80\">\
                     <rect width=\"400\" height=\"80\" fill=\"#fff5f5\" stroke=\"#ffcccc\" rx=\"4\"/>\
                     <text x=\"10\" y=\"25\" font-family=\"monospace\" font-size=\"12\" fill=\"#cc0000\">RENDER FAILED</text>\
                     <text x=\"10\" y=\"50\" font-family=\"monospace\" font-size=\"10\" fill=\"#666\">{}</text>\
                     </svg>",
                    truncated,
                );
                std::fs::write(&out_path, &error_svg).expect("failed to write error SVG");
                println!("FAIL {filename}: {err}");
            }
        }
    }

    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Mermaid Renderer Corpus</title>
<style>
  body { font-family: system-ui, sans-serif; margin: 16px; }
  table { border-collapse: collapse; width: 100%; table-layout: fixed; }
  td { border: 1px solid #999; padding: 8px; vertical-align: top; }
  td img { width: 100%; height: auto; }
  h2 { margin-top: 2em; }
</style>
</head>
<body>
<h1>Mermaid Renderer Corpus</h1>
"#,
    );

    for entry in &corpus_files {
        let stem = entry
            .path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let svg_path = format!("{stem}.svg");
        html.push_str(&format!(
            "<h2>{stem}</h2>\n<table><tr>\n<td><img src=\"{svg_path}\"></td>\n</tr></table>\n",
        ));
    }

    html.push_str("</body>\n</html>\n");

    let html_path = output_dir.join("comparison.html");
    std::fs::write(&html_path, &html).expect("failed to write comparison HTML");

    let canonical = html_path
        .canonicalize()
        .unwrap_or_else(|_| html_path.clone());
    println!("\n=== Corpus HTML written to ===");
    println!("{}", canonical.display());
    println!("==============================\n");
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
