const DEMO_TTF: &[u8] = include_bytes!("./fonts/Tuffy.ttf");
use std::sync::Arc;

#[test]
fn add_fonts_and_get_ids_back() {
    env_logger::init();
    let mut font_db = fontdb::Database::new();
    let ids = font_db.load_font_source(fontdb::Source::Binary(Arc::new(DEMO_TTF)));

    assert_eq!(ids.len(), 1);
    let id = ids[0];

    let font = font_db.face(id).unwrap();
    assert!(font.families.iter().any(|(name, _)| name == "Tuffy"));
}
