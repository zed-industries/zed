use gpui::{FallbackFontClass, FontRun, MissingGlyph, MissingGlyphSink, font, px};
use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};
use wasm_bindgen::prelude::*;

fn main() {}

#[derive(Default)]
struct Reports(Mutex<Vec<MissingGlyph>>);

impl MissingGlyphSink for Reports {
    fn report(&self, missing: Vec<MissingGlyph>) {
        self.0.lock().expect("report lock").extend(missing);
    }
}

#[wasm_bindgen]
pub fn test_missing_glyph_notifications() -> Result<(), JsValue> {
    let platform = gpui_platform::current_platform(false);
    let text_system = platform.text_system();
    text_system
        .add_fonts(vec![Cow::Borrowed(include_bytes!(
            "../../../../assets/fonts/lilex/Lilex-Regular.ttf"
        ))])
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let font_id = text_system
        .font_id(&font("Lilex"))
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
    let reports = Arc::new(Reports::default());
    text_system.set_missing_glyph_sink(Some(reports.clone()));
    let text = "A界😀👨‍👩‍👧‍👦か\u{3099}";
    let runs = [FontRun {
        len: text.len(),
        font_id,
    }];
    let layout = text_system.layout_line(text, px(24.), &runs);
    assert!(
        layout
            .runs
            .iter()
            .any(|run| run.glyphs.iter().any(|glyph| glyph.is_emoji))
    );
    let missing = reports.0.lock().expect("report lock").clone();
    assert_eq!(
        missing
            .iter()
            .map(|report| report.grapheme())
            .collect::<Vec<_>>(),
        ["界", "か\u{3099}"],
    );
    assert!(
        missing
            .iter()
            .all(|report| report.font_class() == FallbackFontClass::Monospace)
    );

    for graphemes in [
        vec!["🕸", "🕸", "🕸"],
        vec!["🕸", "🕸\u{fe0f}", "🕸\u{fe0e}", "🕸"],
    ] {
        reports.0.lock().expect("report lock").clear();
        let text = graphemes.concat();
        let layout = text_system.layout_line(
            &text,
            px(24.),
            &[FontRun {
                len: text.len(),
                font_id,
            }],
        );
        let glyphs = layout
            .runs
            .iter()
            .flat_map(|run| run.glyphs.iter().map(move |glyph| (run.font_id, glyph)))
            .collect::<Vec<_>>();
        assert_eq!(glyphs.len(), graphemes.len());
        let mut source_index = 0;
        for ((resolved_font, glyph), grapheme) in glyphs.iter().zip(&graphemes) {
            assert_ne!(
                *resolved_font, font_id,
                "Missing Lilex glyph must fall back"
            );
            assert_eq!(glyph.index, source_index, "Preserve each source grapheme");
            assert_eq!(glyph.is_emoji, grapheme.ends_with('\u{fe0f}'));
            source_index += grapheme.len();
        }
        assert_eq!(source_index, text.len());
        assert!(
            reports.0.lock().expect("report lock").is_empty(),
            "Every spider-web glyph must resolve, not just the first cached raster"
        );
    }

    reports.0.lock().expect("report lock").clear();
    let bundled = "Hello ©";
    let layout = text_system.layout_line(
        bundled,
        px(24.),
        &[FontRun {
            len: bundled.len(),
            font_id,
        }],
    );
    for run in &layout.runs {
        assert_eq!(run.font_id, font_id, "Retain existing bundled text glyphs");
        assert!(run.glyphs.iter().all(|glyph| !glyph.is_emoji));
    }
    assert!(reports.0.lock().expect("report lock").is_empty());

    reports.0.lock().expect("report lock").clear();
    text_system.set_missing_glyph_sink(None);
    text_system.layout_line(text, px(24.), &runs);
    assert!(reports.0.lock().expect("report lock").is_empty());
    Ok(())
}
