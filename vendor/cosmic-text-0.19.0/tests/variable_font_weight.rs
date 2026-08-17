use std::path::PathBuf;

use cosmic_text::{fontdb, Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Weight};

/// Variable fonts must be matched at all weights within their `wght` axis
/// range, not just the default weight they register at in fontdb, otherwise
/// they will fall back to a system font despite being able to provide the
/// requested weight.
#[test]
fn variable_font_all_weights_match() {
    let repo_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let fonts_path = PathBuf::from(&repo_dir).join("fonts");

    let mut font_system = FontSystem::new();
    font_system
        .db_mut()
        .load_font_data(std::fs::read(fonts_path.join("InterVariable.ttf")).unwrap());

    for w in [100, 200, 300, 400, 500, 600, 700, 800, 900] {
        let metrics = Metrics::new(16.0, 20.0);
        let mut buffer = Buffer::new(&mut font_system, metrics);

        let glyph_font_ids: Vec<fontdb::ID>;
        {
            let mut buffer = buffer.borrow_with(&mut font_system);
            let attrs = Attrs::new()
                .family(Family::Name("Inter Variable"))
                .weight(Weight(w));
            buffer.set_size(Some(300.0), Some(100.0));
            buffer.set_text("Hello world", &attrs, Shaping::Advanced, None);
            buffer.shape_until_scroll(true);

            glyph_font_ids = buffer
                .layout_runs()
                .flat_map(|run| run.glyphs.iter().map(|g| g.font_id))
                .collect();
        }

        assert!(!glyph_font_ids.is_empty(), "Weight {w}: no glyphs produced");

        for id in &glyph_font_ids {
            let face = font_system.db().face(*id).unwrap();
            let family = &face.families[0].0;
            assert!(
                family.contains("Inter"),
                "Weight {w}: expected Inter, got \"{family}\""
            );
        }
    }
}
