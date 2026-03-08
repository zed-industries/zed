use super::*;
use hayro::vello_cpu::color::palette::css::WHITE;
use std::time::Instant;

#[test]
fn dump_real_pdf_glyphs() {
    let pdf_path =
        std::env::var("PDF_DUMP_PATH").expect("set PDF_DUMP_PATH to a PDF file path");
    let pdf_bytes = std::fs::read(&pdf_path).expect("Failed to read PDF file");
    let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
    let layout = extract_page_text(&pdf, 0).expect("Failed to extract text");

    eprintln!("--- Page 1: {} glyphs ---", layout.glyphs.len());
    let mut prev_y: Option<f32> = None;
    for (i, g) in layout.glyphs.iter().enumerate() {
        let line_break = prev_y
            .map(|py| (g.y - py).abs() > g.font_size * SAME_LINE_THRESHOLD)
            .unwrap_or(false);
        if line_break {
            let py = prev_y.unwrap_or(0.0);
            let gap = (g.y - py).abs();
            let size = g.font_size;
            eprintln!(
                "  ---- line break: y_gap={:.1} fs={:.1} base_offset={:.2} ----",
                gap,
                size,
                gap / size.max(1.0)
            );
        }
        if i < 300 || line_break {
            eprintln!(
                "  [{:>4}] '{}' x={:.1} y={:.1} w={:.1} fs={:.1}",
                i,
                g.character.escape_debug(),
                g.x,
                g.y,
                g.width,
                g.font_size
            );
        }
        prev_y = Some(g.y);
    }

    eprintln!("\n--- full_text (first 2000 chars) ---");
    let text = layout.full_text();
    eprintln!("{}", &text[..text.len().min(2000)]);
}

#[test]
fn bench_render_phases() {
    let pdf_path =
        std::env::var("PDF_BENCH_PATH").expect("set PDF_BENCH_PATH to a PDF file path");
    let scale = std::env::var("PDF_BENCH_SCALE")
        .ok()
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(2.0);

    let t0 = Instant::now();
    let pdf_bytes = std::fs::read(&pdf_path).expect("Failed to read PDF file");
    let read_elapsed = t0.elapsed();
    eprintln!(
        "Phase 1 — Read file:      {:>8.2?}  ({} bytes)",
        read_elapsed,
        pdf_bytes.len()
    );

    let t1 = Instant::now();
    let metadata = parse_metadata(&pdf_bytes).expect("Failed to parse PDF");
    let parse_elapsed = t1.elapsed();
    eprintln!(
        "Phase 2 — Parse metadata: {:>8.2?}  ({} pages)",
        parse_elapsed, metadata.page_count
    );

    let pdf = open_pdf(&pdf_bytes).expect("Failed to open PDF");
    let pages = pdf.pages();
    let interpreter_settings = InterpreterSettings::default();

    let mut render_times = Vec::with_capacity(metadata.page_count);
    let mut convert_times = Vec::with_capacity(metadata.page_count);

    for (index, page) in pages.iter().enumerate() {
        let render_settings = RenderSettings {
            x_scale: scale,
            y_scale: scale,
            bg_color: WHITE,
            ..Default::default()
        };

        let t_render = Instant::now();
        let pixmap = hayro::render(&page, &interpreter_settings, &render_settings);
        let render_elapsed = t_render.elapsed();
        render_times.push(render_elapsed);

        let t_convert = Instant::now();
        let _image = pixmap_to_render_image(&pixmap).expect("Failed to convert pixmap");
        let convert_elapsed = t_convert.elapsed();
        convert_times.push(convert_elapsed);

        if index < 5 || index == metadata.page_count - 1 {
            eprintln!(
                "  Page {:>3}: render {:>8.2?}, convert {:>8.2?}  ({}×{})",
                index + 1,
                render_elapsed,
                convert_elapsed,
                pixmap.width(),
                pixmap.height()
            );
        } else if index == 5 {
            eprintln!("  ...");
        }
    }

    let total_render: std::time::Duration = render_times.iter().sum();
    let total_convert: std::time::Duration = convert_times.iter().sum();
    let avg_render = total_render / metadata.page_count as u32;
    let avg_convert = total_convert / metadata.page_count as u32;

    eprintln!();
    eprintln!(
        "Phase 3 — Render pages:   {:>8.2?}  (avg {:>8.2?}/page)",
        total_render, avg_render
    );
    eprintln!(
        "Phase 4 — Convert pixmaps:{:>8.2?}  (avg {:>8.2?}/page)",
        total_convert, avg_convert
    );
    eprintln!();

    let total = read_elapsed + parse_elapsed + total_render + total_convert;
    eprintln!("Total:                    {:>8.2?}", total);
    eprintln!();
    eprintln!("Breakdown:");
    eprintln!(
        "  Read:    {:>5.1}%",
        read_elapsed.as_secs_f64() / total.as_secs_f64() * 100.0
    );
    eprintln!(
        "  Parse:   {:>5.1}%",
        parse_elapsed.as_secs_f64() / total.as_secs_f64() * 100.0
    );
    eprintln!(
        "  Render:  {:>5.1}%",
        total_render.as_secs_f64() / total.as_secs_f64() * 100.0
    );
    eprintln!(
        "  Convert: {:>5.1}%",
        total_convert.as_secs_f64() / total.as_secs_f64() * 100.0
    );
}