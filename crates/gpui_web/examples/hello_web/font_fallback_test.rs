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

    reports.0.lock().expect("report lock").clear();
    text_system.set_missing_glyph_sink(None);
    text_system.layout_line(text, px(24.), &runs);
    assert!(reports.0.lock().expect("report lock").is_empty());
    Ok(())
}
