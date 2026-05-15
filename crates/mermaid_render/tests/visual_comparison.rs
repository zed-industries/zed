use mermaid_render::{MermaidBackend, MermaidTheme};
use std::path::PathBuf;

const BACKENDS: &[(MermaidBackend, &str)] = &[
    (MermaidBackend::MermaidRs, "mermaid-rs"),
    (MermaidBackend::Merman, "merman"),
];

fn one_dark_theme() -> MermaidTheme {
    let editor_background = "#282c33";
    let surface_background = "#2f343e";
    let text = "#dce0e5";
    let border = "#464b57";
    let border_variant = "#363c46";
    let element_background = "#2e343e";
    let ghost_element_hover = "#363c46";
    let panel_background = "#2f343e";

    let player_cursors = [
        "#74ade8", "#be5046", "#bf956a", "#b477cf",
        "#6eb4bf", "#d07277", "#dec184", "#a1c181",
    ];

    MermaidTheme {
        dark_mode: true,
        font_family: "Zed Plex Sans, system-ui".to_string(),
        background: editor_background.to_string(),
        primary_color: surface_background.to_string(),
        primary_text_color: text.to_string(),
        primary_border_color: border.to_string(),
        secondary_color: element_background.to_string(),
        tertiary_color: ghost_element_hover.to_string(),
        line_color: border.to_string(),
        text_color: text.to_string(),
        edge_label_background: editor_background.to_string(),
        cluster_background: panel_background.to_string(),
        cluster_border: border_variant.to_string(),
        note_background: surface_background.to_string(),
        note_border: border_variant.to_string(),
        actor_background: element_background.to_string(),
        actor_border: border.to_string(),
        activation_background: ghost_element_hover.to_string(),
        activation_border: border.to_string(),
        git_branch_colors: std::array::from_fn(|i| {
            player_cursors[i % player_cursors.len()].to_string()
        }),
        git_branch_label_colors: std::array::from_fn(|_| "#fff".to_string()),
        er_attr_bg_odd: surface_background.to_string(),
        er_attr_bg_even: element_background.to_string(),
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

    for (_, backend_slug) in BACKENDS {
        std::fs::create_dir_all(output_dir.join(backend_slug))
            .expect("failed to create backend output dir");
    }

    for entry in &corpus_files {
        let path = entry.path();
        let stem = path.file_stem().unwrap().to_string_lossy();
        let diagram_source =
            std::fs::read_to_string(&path).expect("failed to read corpus file");

        for (backend, backend_slug) in BACKENDS {
            let filename = format!("{stem}.svg");
            let out_path = output_dir.join(backend_slug).join(&filename);

            match mermaid_render::render_to_svg(&diagram_source, &theme, *backend) {
                Ok(svg) => {
                    std::fs::write(&out_path, &svg).expect("failed to write SVG");
                    println!("OK   {backend_slug}/{filename}");
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
                    println!("FAIL {backend_slug}/{filename}: {err}");
                }
            }
        }
    }

    let mut html = String::from(
        r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Mermaid Renderer Comparison</title>
<style>
  body { font-family: system-ui, sans-serif; margin: 16px; }
  table { border-collapse: collapse; width: 100%; table-layout: fixed; }
  td { border: 1px solid #999; padding: 8px; vertical-align: top; }
  td img { width: 100%; height: auto; }
  h2 { margin-top: 2em; }
</style>
</head>
<body>
<h1>Mermaid Renderer Comparison</h1>
"#,
    );

    for entry in &corpus_files {
        let stem = entry
            .path()
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();
        html.push_str(&format!("<h2>{stem}</h2>\n<table><tr>\n"));
        for (_, backend_slug) in BACKENDS {
            let svg_path = format!("{backend_slug}/{stem}.svg");
            html.push_str(&format!(
                "<td><h4>{backend_slug}</h4><img src=\"{svg_path}\"></td>\n",
            ));
        }
        html.push_str("</tr></table>\n");
    }

    html.push_str("</body>\n</html>\n");

    let html_path = output_dir.join("comparison.html");
    std::fs::write(&html_path, &html).expect("failed to write comparison HTML");

    let canonical = html_path
        .canonicalize()
        .unwrap_or_else(|_| html_path.clone());
    println!("\n=== Visual comparison HTML written to ===");
    println!("{}", canonical.display());
    println!("=========================================\n");
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
