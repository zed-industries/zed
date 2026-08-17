//! Generates code from entities.json depending on enabled features.
//!
//! The canonical source is <https://html.spec.whatwg.org/entities.json> (see
//! <https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references>).
//!
//! The entities.json file looks like:
//!
//!     {
//!         "&AElig": { "codepoints": [198], "characters": "\u00C6" },
//!         . . .
//!     }

fn main() {
    #[cfg(any(feature = "unescape_fast", feature = "entities"))]
    let entities = load_entities("entities.json");

    #[cfg(feature = "unescape_fast")]
    generate_matcher_rs(&entities);

    #[cfg(feature = "entities")]
    generate_entities_rs(&entities);
}

/// Generate entities.rs file containing all valid HTML entities in a
/// [`phf::Map`] along with a few useful constants. It also generates
/// documentation with all entities in a table.
#[cfg(feature = "entities")]
fn generate_entities_rs(entities: &[(String, String)]) {
    use std::cmp::{max, min};
    use std::env;
    use std::fs::File;
    use std::io::{BufWriter, Write};
    use std::path::Path;

    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("entities.rs");
    let mut out = BufWriter::new(File::create(out_path).unwrap());

    writeln!(out, "\
        /// A map of all valid HTML entities to their expansions.\n\
        ///\n\
        /// The keys of the map are full entity byte strings, e.g. `b\"&copy;\"`, and the\n\
        /// values are their expansions, e.g. `b\"©\"`.\n\
        ///\n\
        /// See the [WHATWG HTML spec][spec] for the canonical list of entities with\n\
        /// their codepoints and glyphs. The [entities.json][] file linked there is\n\
        /// used to generate this constant.\n\
        ///\n\
        /// [spec]: https://html.spec.whatwg.org/multipage/named-characters.html#named-character-references\n\
        /// [entities.json]: https://html.spec.whatwg.org/entities.json\n\
        ///\n\
        /// Entity                         | Codepoints         | Glyph\n\
        /// -------------------------------|--------------------|------").unwrap();

    let mut map_builder = phf_codegen::Map::<&[u8]>::new();
    let mut max_len: usize = 0;
    let mut min_len: usize = usize::MAX;
    let mut bare_max_len: usize = 0;
    for (name, glyph) in entities {
        map_builder.entry(name.as_bytes(), format!("&{:?}", glyph.as_bytes()));
        max_len = max(max_len, name.len());
        min_len = min(min_len, name.len());
        if !name.ends_with(';') {
            bare_max_len = max(bare_max_len, name.len());
        }

        // `{:28}` would pad the output inside the backticks.
        let name = format!("`{name}`");

        let codepoints = glyph
            .chars()
            .map(|c| format!("U+{:06X}", u32::from(c)))
            .collect::<Vec<_>>()
            .join(", ");

        // Suppress a few inconvenient glyphs. Newline adds an extra line, and
        // tab causes a clippy warning. Backticks are actually fine, but it’s
        // correct to escape them.
        let glyph = match glyph.as_str() {
            "\n" | "\t" => "",
            "`" => "\\`",
            v => v,
        };

        writeln!(out, "/// {name:30} | {codepoints:18} | {glyph}").unwrap();
    }

    let map = map_builder.build();
    writeln!(
        out,
        "\
        #[allow(clippy::unreadable_literal)]\n\
        pub static ENTITIES: phf::Map<&[u8], &[u8]> = {map};\n\
        \n\
        /// Length of longest entity including ‘&’ and possibly ‘;’.\n\
        pub const ENTITY_MAX_LENGTH: usize = {max_len};\n\
        \n\
        /// Length of shortest entity including ‘&’ and possibly ‘;’.\n\
        pub const ENTITY_MIN_LENGTH: usize = {min_len};\n\
        \n\
        /// Length of longest semicolon-less entity including ‘&’.\n\
        pub const BARE_ENTITY_MAX_LENGTH: usize = {bare_max_len};"
    )
    .unwrap();
}

/// Generated matcher.rs file containing a function `entity_matcher()` that is
/// basically just a giant nested tree of `match` expressions to check if the
/// next bytes in an iterator are an HTML entity.
#[cfg(feature = "unescape_fast")]
fn generate_matcher_rs(entities: &[(String, String)]) {
    let mut matcher = matchgen::TreeMatcher::new(
        "fn entity_matcher",
        "(bool, &'static [u8])",
    );
    for (name, glyph) in entities {
        matcher.add(
            name.as_bytes(),
            format!("({:?}, &{:?})", name.ends_with(';'), glyph.as_bytes()),
        );
    }
    matcher
        .doc("Used in `match_entity()`.")
        .disable_clippy(true)
        .input_type(matchgen::Input::Slice)
        .write_to_out_dir("matcher.rs")
        .unwrap();
}

/// Load HTML entities as `vec![...("&gt;", ">")...]`.
#[cfg(any(feature = "unescape_fast", feature = "entities"))]
fn load_entities<P: AsRef<std::path::Path>>(path: P) -> Vec<(String, String)> {
    let input = std::fs::read(path.as_ref()).unwrap();
    let input: serde_json::Map<String, serde_json::Value> =
        serde_json::from_slice(&input).unwrap();

    input
        .iter()
        .map(|(name, info)| {
            (
                name.clone(),
                info["characters"].as_str().unwrap().to_owned(),
            )
        })
        .collect()
}
