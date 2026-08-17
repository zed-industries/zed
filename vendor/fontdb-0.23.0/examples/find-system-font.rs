#[cfg(not(feature = "fs"))]
fn main() {}

#[cfg(feature = "fs")]
fn main() {
    std::env::set_var("RUST_LOG", "fontdb=trace");
    env_logger::init();

    let mut db = fontdb::Database::new();
    let now = std::time::Instant::now();
    db.load_system_fonts();
    db.set_serif_family("Times New Roman");
    db.set_sans_serif_family("Arial");
    db.set_cursive_family("Comic Sans MS");
    db.set_fantasy_family("Impact");
    db.set_monospace_family("Courier New");
    println!(
        "Loaded {} font faces in {}ms.",
        db.len(),
        now.elapsed().as_millis()
    );

    const FAMILY_NAME: &str = "Times New Roman";
    let query = fontdb::Query {
        families: &[fontdb::Family::Name(FAMILY_NAME), fontdb::Family::SansSerif],
        weight: fontdb::Weight::BOLD,
        ..fontdb::Query::default()
    };

    let now = std::time::Instant::now();
    match db.query(&query) {
        Some(id) => {
            let (src, index) = db.face_source(id).unwrap();
            if let fontdb::Source::File(ref path) = &src {
                println!(
                    "Font '{}':{} found in {}ms.",
                    path.display(),
                    index,
                    now.elapsed().as_micros() as f64 / 1000.0
                );
            }
        }
        None => {
            println!("Error: '{}' not found.", FAMILY_NAME);
        }
    }
}
