use criterion::{Criterion, criterion_group, criterion_main};
use gpui::{FontFallbacks, FontRun, PlatformTextSystem, font, px};
use gpui_wgpu::CosmicTextSystem;
use std::borrow::Cow;

const LILEX: &[u8] = include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf");
const IBM_PLEX: &[u8] =
    include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");

// ~4 000 chars of typical ASCII code text, as a single display line.
//
// `layout_line` is handed one line at a time, already split on `\n` by its
// callers, so the newlines are replaced rather than kept: leaving them in would
// make this measure the multi-paragraph path instead of the common one, since
// `\n` is itself a bidi paragraph separator.
fn code_text() -> String {
    concat!(
        "    fn compute_run_spans(\n",
        "        text: &str,\n",
        "        run_offset: usize,\n",
        "        run_len: usize,\n",
        "        primary: FontId,\n",
        "        fallback_chain: &[(FontId, SharedString)],\n",
        "        covers: &impl Fn(FontId, char) -> bool,\n",
        "    ) -> SmallVec<[RunSpan; 4]> {\n",
        "        let mut spans = SmallVec::new();\n",
        "        let run_end = run_offset + run_len;\n",
        "        if run_end <= run_offset { return spans; }\n",
        "        let run_text = &text[run_offset..run_end];\n",
        "        let mut span_start = run_offset;\n",
        "        let mut span_slot: Option<usize> = None;\n",
        "        for (ch_idx, ch) in run_text.char_indices() {\n",
        "            let abs = run_offset + ch_idx;\n",
        "            let next = pick_covering_slot(ch, span_slot, primary, fallback_chain, covers);\n",
        "            if next == span_slot { continue; }\n",
        "            if abs > span_start {\n",
        "                spans.push(RunSpan { start: span_start, end: abs, slot: span_slot });\n",
        "            }\n",
        "            span_start = abs;\n",
        "            span_slot = next;\n",
        "        }\n",
        "        spans\n",
        "    }\n",
    )
    .repeat(8) // ~3 800 chars
    .replace('\n', " ")
}

fn bench_layout_line(c: &mut Criterion) {
    let system = CosmicTextSystem::new_without_system_fonts("Lilex");
    system
        .add_fonts(vec![Cow::Borrowed(LILEX), Cow::Borrowed(IBM_PLEX)])
        .unwrap();

    let font_id_no_fallback = system.font_id(&font("Lilex")).unwrap();

    let font_id_with_fallback = {
        let mut f = font("Lilex");
        f.fallbacks = Some(FontFallbacks::from_fonts(vec!["IBM Plex Sans".to_string()]));
        system.font_id(&f).unwrap()
    };

    let text = code_text();

    // Same text, but with a bidi paragraph separator (U+001C) and RTL content
    // forcing the per-paragraph shaping path.
    let text_mixed_direction = text.clone() + "\u{001c}\u{05d0}\u{05d1}";
    assert!(
        !text.contains('\n'),
        "fast-path corpus must contain no separator"
    );

    let runs_no_fallback = vec![FontRun {
        len: text.len(),
        font_id: font_id_no_fallback,
    }];
    let runs_with_fallback = vec![FontRun {
        len: text.len(),
        font_id: font_id_with_fallback,
    }];
    let runs_mixed_direction = vec![FontRun {
        len: text_mixed_direction.len(),
        font_id: font_id_no_fallback,
    }];

    let mut group = c.benchmark_group("layout_line");

    group.bench_function("no_fallback", |b| {
        b.iter(|| system.layout_line(&text, px(14.0), &runs_no_fallback))
    });

    group.bench_function("with_fallback_ascii", |b| {
        b.iter(|| system.layout_line(&text, px(14.0), &runs_with_fallback))
    });

    group.bench_function("mixed_direction_paragraphs", |b| {
        b.iter(|| system.layout_line(&text_mixed_direction, px(14.0), &runs_mixed_direction))
    });

    group.finish();
}

criterion_group!(benches, bench_layout_line);
criterion_main!(benches);
