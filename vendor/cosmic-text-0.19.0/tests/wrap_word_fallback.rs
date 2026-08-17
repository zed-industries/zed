use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, Wrap};

// Tests the ability to fallback to glyph wrapping when a word can't fit on a line by itself.
// No line should ever overflow the buffer size.
#[test]
fn wrap_word_fallback() {
    let mut font_system =
        FontSystem::new_with_locale_and_db("en-US".into(), fontdb::Database::new());
    let font = std::fs::read("fonts/Inter-Regular.ttf").unwrap();
    font_system.db_mut().load_font_data(font);
    let metrics = Metrics::new(14.0, 20.0);

    let mut buffer = Buffer::new(&mut font_system, metrics);

    let mut buffer = buffer.borrow_with(&mut font_system);

    buffer.set_wrap(Wrap::WordOrGlyph);
    buffer.set_size(Some(50.0), Some(1000.0));
    buffer.set_text("Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.", &Attrs::new().family(cosmic_text::Family::Name("Inter")), Shaping::Advanced, None);

    let measured_size = buffer
        .layout_runs()
        .fold(0.0f32, |width, run| width.max(run.line_w));

    assert!(
        measured_size <= buffer.size().0.unwrap_or(0.0),
        "Measured width is larger than buffer width\n{} <= {}",
        measured_size,
        buffer.size().0.unwrap_or(0.0)
    );
}
