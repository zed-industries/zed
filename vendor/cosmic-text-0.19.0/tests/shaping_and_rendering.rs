use common::DrawTestCfg;
use cosmic_text::Attrs;
use fontdb::Family;

mod common;

#[test]
fn test_hebrew_word_rendering() {
    let attrs = Attrs::new().family(Family::Name("Noto Sans"));
    DrawTestCfg::new("a_hebrew_word")
        .font_size(36., 40.)
        .font_attrs(attrs)
        .text("בדיקה")
        .canvas(120, 60)
        .validate_text_rendering();
}

#[test]
fn test_hebrew_paragraph_rendering() {
    let paragraph = "השועל החום המהיר קופץ מעל הכלב העצלן";
    let attrs = Attrs::new().family(Family::Name("Noto Sans"));
    DrawTestCfg::new("a_hebrew_paragraph")
        .font_size(36., 40.)
        .font_attrs(attrs)
        .text(paragraph)
        .canvas(400, 110)
        .validate_text_rendering();
}

#[test]
fn test_english_mixed_with_hebrew_paragraph_rendering() {
    let paragraph = "Many computer programs fail to display bidirectional text correctly. For example, this page is mostly LTR English script, and here is the RTL Hebrew name Sarah: שרה, spelled sin (ש) on the right, resh (ר) in the middle, and heh (ה) on the left.";
    let attrs = Attrs::new().family(Family::Name("Noto Sans"));
    DrawTestCfg::new("some_english_mixed_with_hebrew")
        .font_size(16., 20.)
        .font_attrs(attrs)
        .text(paragraph)
        .canvas(400, 120)
        .validate_text_rendering();
}

#[test]
fn test_arabic_word_rendering() {
    let attrs = Attrs::new().family(Family::Name("Noto Sans"));
    DrawTestCfg::new("an_arabic_word")
        .font_size(36., 40.)
        .font_attrs(attrs)
        .text("خالصة")
        .canvas(120, 60)
        .validate_text_rendering();
}

#[test]
fn test_arabic_paragraph_rendering() {
    let paragraph = "الثعلب البني السريع يقفز فوق الكلب الكسول";
    let attrs = Attrs::new().family(Family::Name("Noto Sans"));
    DrawTestCfg::new("an_arabic_paragraph")
        .font_size(36., 40.)
        .font_attrs(attrs)
        .text(paragraph)
        .canvas(400, 110)
        .validate_text_rendering();
}

#[test]
fn test_english_mixed_with_arabic_paragraph_rendering() {
    let paragraph = "I like to render اللغة العربية in Rust!";
    let attrs = Attrs::new().family(Family::Name("Noto Sans"));
    DrawTestCfg::new("some_english_mixed_with_arabic")
        .font_size(36., 40.)
        .font_attrs(attrs)
        .text(paragraph)
        .canvas(400, 110)
        .validate_text_rendering();
}

#[test]
fn test_ligature_segmentation() {
    use cosmic_text::{Buffer, FontSystem, Metrics, Shaping};

    let mut font_system =
        FontSystem::new_with_locale_and_db("en-US".into(), fontdb::Database::new());
    let font = std::fs::read("fonts/Inter-Regular.ttf").unwrap();
    font_system.db_mut().load_font_data(font);
    let metrics = Metrics::new(14.0, 20.0);

    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);

    buffer.set_text("|>", &Attrs::new(), Shaping::Advanced, None);
    let _ = buffer.layout_runs();

    let line = &buffer.lines[0];
    let shape = line.shape_opt().expect("ShapeLine not found");
    let span = &shape.spans[0];

    // Inter-Regular HAS a contextual alternate for |> (changing the glyph ID),
    // so our probe detects it and keeps them together.
    assert_eq!(
        span.words.len(),
        1,
        "Expected '|>' to be 1 word (contextual alternate in Inter), but found {} words.",
        span.words.len()
    );

    // Test -> (Arrow), which is a common ligature.
    buffer.set_text("->", &Attrs::new(), Shaping::Advanced, None);
    let _ = buffer.layout_runs();
    let line = &buffer.lines[0];
    let shape = line.shape_opt().expect("ShapeLine not found");

    assert_eq!(
        shape.spans[0].words.len(),
        1,
        "Expected '->' to be a single word (ligature), but found {} words.",
        shape.spans[0].words.len()
    );

    // Test !=
    buffer.set_text("!=", &Attrs::new(), Shaping::Advanced, None);
    let _ = buffer.layout_runs();
    let line = &buffer.lines[0];
    let shape = line.shape_opt().expect("ShapeLine not found");
    // Inter has a contextual alternate for != too.
    assert_eq!(
        shape.spans[0].words.len(),
        1,
        "Expected '!=' to be 1 word (contextual alternate), but found {} words.",
        shape.spans[0].words.len()
    );

    // Test ++
    buffer.set_text("++", &Attrs::new(), Shaping::Advanced, None);
    let _ = buffer.layout_runs();
    let line = &buffer.lines[0];
    let shape = line.shape_opt().expect("ShapeLine not found");
    // Inter does not have a ++ ligature.
    assert_eq!(
        shape.spans[0].words.len(),
        2,
        "Expected '++' to be 2 words, but found {} words.",
        shape.spans[0].words.len()
    );
}
