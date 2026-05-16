use crate::{
    AssetSource, DevicePixels, IsZero, RenderImage, Result, SharedString, Size,
    swap_rgba_pa_to_bgra,
};
use image::Frame;
use resvg::tiny_skia::Pixmap;
use smallvec::SmallVec;
use std::{
    hash::Hash,
    sync::{Arc, LazyLock, OnceLock},
};

#[cfg(target_os = "macos")]
const EMOJI_FONT_FAMILIES: &[&str] = &["Apple Color Emoji", ".AppleColorEmojiUI"];

#[cfg(target_os = "windows")]
const EMOJI_FONT_FAMILIES: &[&str] = &["Segoe UI Emoji", "Segoe UI Symbol"];

#[cfg(any(target_os = "linux", target_os = "freebsd"))]
const EMOJI_FONT_FAMILIES: &[&str] = &[
    "Noto Color Emoji",
    "Emoji One",
    "Twitter Color Emoji",
    "JoyPixels",
];

#[cfg(not(any(
    target_os = "macos",
    target_os = "windows",
    target_os = "linux",
    target_os = "freebsd",
)))]
const EMOJI_FONT_FAMILIES: &[&str] = &[];

fn is_emoji_presentation(c: char) -> bool {
    static EMOJI_PRESENTATION_REGEX: LazyLock<regex::Regex> =
        LazyLock::new(|| regex::Regex::new("\\p{Emoji_Presentation}").unwrap());
    let mut buf = [0u8; 4];
    EMOJI_PRESENTATION_REGEX.is_match(c.encode_utf8(&mut buf))
}

fn font_has_char(db: &usvg::fontdb::Database, id: usvg::fontdb::ID, ch: char) -> bool {
    db.with_face_data(id, |font_data, face_index| {
        ttf_parser::Face::parse(font_data, face_index)
            .ok()
            .and_then(|face| face.glyph_index(ch))
            .is_some()
    })
    .unwrap_or(false)
}

fn select_emoji_font(
    ch: char,
    fonts: &[usvg::fontdb::ID],
    db: &usvg::fontdb::Database,
    families: &[&str],
) -> Option<usvg::fontdb::ID> {
    for family_name in families {
        let query = usvg::fontdb::Query {
            families: &[usvg::fontdb::Family::Name(family_name)],
            weight: usvg::fontdb::Weight(400),
            stretch: usvg::fontdb::Stretch::Normal,
            style: usvg::fontdb::Style::Normal,
        };

        let Some(id) = db.query(&query) else {
            continue;
        };

        if fonts.contains(&id) || !font_has_char(db, id, ch) {
            continue;
        }

        return Some(id);
    }

    None
}

static FOREIGN_OBJECT_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<foreignObject([^>]*)>([\s\S]*?)</foreignObject>").unwrap());

static TAG_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r"<[^>]+>").unwrap());

static ATTR_RE: LazyLock<regex::Regex> =
    LazyLock::new(|| regex::Regex::new(r#"(\w[-\w]*)\s*=\s*["']?([^"'<>\s]*)["'"]?"#).unwrap());

/// Preprocess SVG content to convert foreignObject elements to native SVG text elements.
/// This is needed because usvg/resvg don't support foreignObject (SVG 2 feature).
pub fn preprocess_foreign_objects(svg_content: &str) -> String {
    if !svg_content.contains("foreignObject") {
        return svg_content.to_string();
    }

    FOREIGN_OBJECT_RE.replace_all(svg_content, |caps: &regex::Captures| {
        let attrs = &caps[1];
        let inner_content = &caps[2];

            let mut width: Option<f32> = None;
        let mut height: Option<f32> = None;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;

        for cap in ATTR_RE.captures_iter(attrs) {
            let name = &cap[1];
            let value = &cap[2];
            match name {
                "width" => width = parse_length(value),
                "height" => height = parse_length(value),
                "x" => x = parse_length(value).unwrap_or(0.0),
                "y" => y = parse_length(value).unwrap_or(0.0),
                _ => {}
            }
        }

            let text = strip_html_tags(inner_content);

        if text.trim().is_empty() {
            String::new()
        } else {
                    let center_x = width.map(|w| x + w / 2.0).unwrap_or(x);
            let center_y = height.map(|h| y + h / 2.0).unwrap_or(y);

            // Estimate font size (use height if available, otherwise default)
            let font_size = height.unwrap_or(16.0) * 0.7;

                    let escaped_text = escape_xml(&text);

            format!(
                r#"<text x="{}" y="{}" font-size="{}" text-anchor="middle" dominant-baseline="middle" fill="black">{}</text>"#,
                center_x, center_y, font_size, escaped_text
            )
        }
    }).to_string()
}

fn parse_length(value: &str) -> Option<f32> {
    let mut chars = Vec::new();
    let mut has_dot = false;
    for c in value.chars() {
        if c.is_numeric() {
            chars.push(c);
        } else if c == '.' && !has_dot {
            chars.push(c);
            has_dot = true;
        }
    }
    let numeric_str: String = chars.into_iter().collect();
    if numeric_str.is_empty() || numeric_str == "." {
        return None;
    }
    numeric_str.parse().ok()
}

fn strip_html_tags(html: &str) -> String {
    let text = TAG_RE.replace_all(html, "");
    let mut result = String::new();
    let mut last_was_space = true;
    for c in text.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
        } else {
            result.push(c);
            last_was_space = false;
        }
    }
    result.trim().to_string()
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// When rendering SVGs, we render them at twice the size to get a higher-quality result.
pub const SMOOTH_SVG_SCALE_FACTOR: f32 = 2.;

#[derive(Clone, PartialEq, Hash, Eq)]
#[expect(missing_docs)]
pub struct RenderSvgParams {
    pub path: SharedString,
    pub size: Size<DevicePixels>,
}

#[derive(Clone)]
/// A struct holding everything necessary to render SVGs.
pub struct SvgRenderer {
    asset_source: Arc<dyn AssetSource>,
    usvg_options: Arc<usvg::Options<'static>>,
}

/// The size in which to render the SVG.
pub enum SvgSize {
    /// An absolute size in device pixels.
    Size(Size<DevicePixels>),
    /// A scaling factor to apply to the size provided by the SVG.
    ScaleFactor(f32),
}

impl SvgRenderer {
    /// Creates a new SVG renderer with the provided asset source.
    pub fn new(asset_source: Arc<dyn AssetSource>) -> Self {
        static SYSTEM_FONT_DB: LazyLock<Arc<usvg::fontdb::Database>> = LazyLock::new(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_system_fonts();
            Arc::new(db)
        });

        // Build the enriched font DB lazily on first SVG render rather than
        // eagerly at construction time. This avoids the expensive deep-clone
        // of the system font database for code paths that never render SVGs
        // (e.g. tests).
        let enriched_fontdb: Arc<OnceLock<Arc<usvg::fontdb::Database>>> = Arc::new(OnceLock::new());

        let default_font_resolver = usvg::FontResolver::default_font_selector();
        let font_resolver = Box::new({
            let asset_source = asset_source.clone();
            move |font: &usvg::Font, db: &mut Arc<usvg::fontdb::Database>| {
                if db.is_empty() {
                    let fontdb = enriched_fontdb.get_or_init(|| {
                        let mut db = (**SYSTEM_FONT_DB).clone();
                        load_bundled_fonts(&*asset_source, &mut db);
                        fix_generic_font_families(&mut db);
                        Arc::new(db)
                    });
                    *db = fontdb.clone();
                }
                if let Some(id) = default_font_resolver(font, db) {
                    return Some(id);
                }
                // fontdb doesn't recognize CSS system font keywords like "system-ui"
                // or "ui-sans-serif", so fall back to sans-serif before any face.
                let sans_query = usvg::fontdb::Query {
                    families: &[usvg::fontdb::Family::SansSerif],
                    ..Default::default()
                };
                db.query(&sans_query)
                    .or_else(|| db.faces().next().map(|f| f.id))
            }
        });
        let default_fallback_selection = usvg::FontResolver::default_fallback_selector();
        let fallback_selection = Box::new(
            move |ch: char, fonts: &[usvg::fontdb::ID], db: &mut Arc<usvg::fontdb::Database>| {
                if is_emoji_presentation(ch) {
                    if let Some(id) = select_emoji_font(ch, fonts, db.as_ref(), EMOJI_FONT_FAMILIES)
                    {
                        return Some(id);
                    }
                }

                default_fallback_selection(ch, fonts, db)
            },
        );
        let options = usvg::Options {
            font_resolver: usvg::FontResolver {
                select_font: font_resolver,
                select_fallback: fallback_selection,
            },
            ..Default::default()
        };
        Self {
            asset_source,
            usvg_options: Arc::new(options),
        }
    }

    /// Renders the given bytes into an image buffer.
    pub fn render_single_frame(
        &self,
        bytes: &[u8],
        scale_factor: f32,
    ) -> Result<Arc<RenderImage>, usvg::Error> {
        self.render_pixmap(
            bytes,
            SvgSize::ScaleFactor(scale_factor * SMOOTH_SVG_SCALE_FACTOR),
        )
        .map(|pixmap| {
            let mut buffer =
                image::ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take())
                    .unwrap();

            for pixel in buffer.chunks_exact_mut(4) {
                swap_rgba_pa_to_bgra(pixel);
            }

            let mut image = RenderImage::new(SmallVec::from_const([Frame::new(buffer)]));
            image.scale_factor = SMOOTH_SVG_SCALE_FACTOR;
            Arc::new(image)
        })
    }

    pub(crate) fn render_alpha_mask(
        &self,
        params: &RenderSvgParams,
        bytes: Option<&[u8]>,
    ) -> Result<Option<(Size<DevicePixels>, Vec<u8>)>> {
        anyhow::ensure!(!params.size.is_zero(), "can't render at a zero size");

        let render_pixmap = |bytes| {
            let pixmap = self.render_pixmap(bytes, SvgSize::Size(params.size))?;

            // Convert the pixmap's pixels into an alpha mask.
            let size = Size::new(
                DevicePixels(pixmap.width() as i32),
                DevicePixels(pixmap.height() as i32),
            );
            let alpha_mask = pixmap
                .pixels()
                .iter()
                .map(|p| p.alpha())
                .collect::<Vec<_>>();

            Ok(Some((size, alpha_mask)))
        };

        if let Some(bytes) = bytes {
            render_pixmap(bytes)
        } else if let Some(bytes) = self.asset_source.load(&params.path)? {
            render_pixmap(&bytes)
        } else {
            Ok(None)
        }
    }

    fn render_pixmap(&self, bytes: &[u8], size: SvgSize) -> Result<Pixmap, usvg::Error> {
        let svg_content = String::from_utf8_lossy(bytes);
        let processed_svg = preprocess_foreign_objects(&svg_content);
        let processed_bytes = processed_svg.as_bytes();

        let tree = usvg::Tree::from_data(processed_bytes, &self.usvg_options)?;
        let svg_size = tree.size();
        let scale = match size {
            SvgSize::Size(size) => size.width.0 as f32 / svg_size.width(),
            SvgSize::ScaleFactor(scale) => scale,
        };

        // Render the SVG to a pixmap with the specified width and height.
        let mut pixmap = resvg::tiny_skia::Pixmap::new(
            (svg_size.width() * scale) as u32,
            (svg_size.height() * scale) as u32,
        )
        .ok_or(usvg::Error::InvalidSize)?;

        let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);

        resvg::render(&tree, transform, &mut pixmap.as_mut());

        Ok(pixmap)
    }
}

fn load_bundled_fonts(asset_source: &dyn AssetSource, db: &mut usvg::fontdb::Database) {
    let font_paths = [
        "fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf",
        "fonts/lilex/Lilex-Regular.ttf",
    ];
    for path in font_paths {
        match asset_source.load(path) {
            Ok(Some(data)) => db.load_font_data(data.into_owned()),
            Ok(None) => log::warn!("Bundled font not found: {path}"),
            Err(error) => log::warn!("Failed to load bundled font {path}: {error}"),
        }
    }
}

// fontdb defaults generic families to Microsoft fonts ("Arial", "Times New Roman")
// which aren't installed on most Linux systems. fontconfig normally overrides these,
// but when it fails the defaults remain and all generic family queries return None.
fn fix_generic_font_families(db: &mut usvg::fontdb::Database) {
    use usvg::fontdb::{Family, Query};

    let families_and_fallbacks: &[(Family<'_>, &str)] = &[
        (Family::SansSerif, "IBM Plex Sans"),
        // No serif font bundled; use sans-serif as best available fallback.
        (Family::Serif, "IBM Plex Sans"),
        (Family::Monospace, "Lilex"),
        (Family::Cursive, "IBM Plex Sans"),
        (Family::Fantasy, "IBM Plex Sans"),
    ];

    for (family, fallback_name) in families_and_fallbacks {
        let query = Query {
            families: &[*family],
            ..Default::default()
        };
        if db.query(&query).is_none() {
            match family {
                Family::SansSerif => db.set_sans_serif_family(*fallback_name),
                Family::Serif => db.set_serif_family(*fallback_name),
                Family::Monospace => db.set_monospace_family(*fallback_name),
                Family::Cursive => db.set_cursive_family(*fallback_name),
                Family::Fantasy => db.set_fantasy_family(*fallback_name),
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use usvg::fontdb::{Database, Family, Query};

    const IBM_PLEX_REGULAR: &[u8] =
        include_bytes!("../../../assets/fonts/ibm-plex-sans/IBMPlexSans-Regular.ttf");
    const LILEX_REGULAR: &[u8] = include_bytes!("../../../assets/fonts/lilex/Lilex-Regular.ttf");

    fn db_with_bundled_fonts() -> Database {
        let mut db = Database::new();
        db.load_font_data(IBM_PLEX_REGULAR.to_vec());
        db.load_font_data(LILEX_REGULAR.to_vec());
        db
    }

    #[test]
    fn test_is_emoji_presentation() {
        let cases = [
            ("a", false),
            ("Z", false),
            ("1", false),
            ("#", false),
            ("*", false),
            ("漢", false),
            ("中", false),
            ("カ", false),
            ("©", false),
            ("♥", false),
            ("😀", true),
            ("✅", true),
            ("🇺🇸", true),
            // SVG fallback is not cluster-aware yet
            ("©️", false),
            ("♥️", false),
            ("1️⃣", false),
        ];
        for (s, expected) in cases {
            assert_eq!(
                is_emoji_presentation(s.chars().next().unwrap()),
                expected,
                "for char {:?}",
                s
            );
        }
    }

    #[test]
    fn test_preprocess_foreign_objects_simple() {
        let svg = r#"<svg><foreignObject x="10" y="20" width="100" height="30"><div>Hello World</div></foreignObject></svg>"#;
        let result = preprocess_foreign_objects(svg);
        assert!(result.contains("<text"));
        assert!(result.contains("Hello World"));
        assert!(!result.contains("foreignObject"));
    }

    #[test]
    fn test_preprocess_foreign_objects_with_span() {
        let svg = r#"<svg><foreignObject width="135.921875" height="24"><div xmlns="http://www.w3.org/1999/xhtml"><span class="nodeLabel"><p>Start</p></span></div></foreignObject></svg>"#;
        let result = preprocess_foreign_objects(svg);
        assert!(result.contains("Start"));
        assert!(!result.contains("foreignObject"));
    }

    #[test]
    fn test_preprocess_foreign_objects_no_change() {
        let svg = r#"<svg><rect width="100" height="100"/></svg>"#;
        let result = preprocess_foreign_objects(svg);
        assert_eq!(result, svg);
    }

    #[test]
    fn test_preprocess_foreign_objects_empty() {
        let svg = r#"<svg><foreignObject width="100" height="30"><div></div></foreignObject></svg>"#;
        let result = preprocess_foreign_objects(svg);
        assert!(!result.contains("text"));
        assert!(!result.contains("foreignObject"));
    }

    #[test]
    fn test_escape_xml() {
        assert_eq!(escape_xml("a < b & c > d"), "a &lt; b &amp; c &gt; d");
    }

    #[test]
    fn fix_generic_font_families_sets_all_families() {
        let mut db = db_with_bundled_fonts();
        fix_generic_font_families(&mut db);

        let families = [
            Family::SansSerif,
            Family::Serif,
            Family::Monospace,
            Family::Cursive,
            Family::Fantasy,
        ];

        for family in families {
            let query = Query {
                families: &[family],
                ..Default::default()
            };
            assert!(
                db.query(&query).is_some(),
                "Expected generic family {family:?} to resolve after fix_generic_font_families"
            );
        }
    }

    #[test]
    fn test_select_emoji_font_skips_family_without_glyph() {
        let mut db = db_with_bundled_fonts();

        let ibm_plex_sans = db
            .query(&usvg::fontdb::Query {
                families: &[usvg::fontdb::Family::Name("IBM Plex Sans")],
                weight: usvg::fontdb::Weight(400),
                stretch: usvg::fontdb::Stretch::Normal,
                style: usvg::fontdb::Style::Normal,
            })
            .unwrap();
        let lilex = db
            .query(&usvg::fontdb::Query {
                families: &[usvg::fontdb::Family::Name("Lilex")],
                weight: usvg::fontdb::Weight(400),
                stretch: usvg::fontdb::Stretch::Normal,
                style: usvg::fontdb::Style::Normal,
            })
            .unwrap();
        let selected = select_emoji_font('│', &[], &db, &["IBM Plex Sans", "Lilex"]).unwrap();

        assert_eq!(selected, lilex);
        assert!(!font_has_char(&db, ibm_plex_sans, '│'));
        assert!(font_has_char(&db, selected, '│'));
    }


    #[test]
    fn test_parse_length_preserves_decimals() {
        assert_eq!(parse_length("12.5"), Some(12.5));
        assert_eq!(parse_length("100.123"), Some(100.123));
        assert_eq!(parse_length(".5"), Some(0.5));
        assert_eq!(parse_length("100"), Some(100.0));
    }

    #[test]
    fn test_parse_length_handles_trailing_dot() {
        assert_eq!(parse_length("100."), Some(100.0));
        assert_eq!(parse_length("."), None);
    }

    #[test]
    fn test_preprocess_foreign_objects_with_decimals() {
        let svg = r#"<svg><foreignObject x="10.5" y="20.75" width="100.5" height="30.25"><div>Test</div></foreignObject></svg>"#;
        let result = preprocess_foreign_objects(svg);
        assert!(result.contains("Test"), "text content should be preserved");
        assert!(result.contains("<text"), "should convert to text element");
        assert!(result.contains("font-size"), "font-size should be derived from height");
        assert!(!result.contains("foreignObject"), "foreignObject should be removed");
    }

    #[test]
    fn test_preprocess_foreign_objects_with_quoted_attrs() {
        let svg = r#"<svg><foreignObject x="10" y="20" width="100" height="30"><div>Hello</div></foreignObject></svg>"#;
        let result = preprocess_foreign_objects(svg);
        assert!(result.contains("Hello"));
        assert!(!result.contains("foreignObject"));
    }
    #[test]
    fn fix_generic_font_families_monospace_resolves_to_lilex() {
        let mut db = db_with_bundled_fonts();
        fix_generic_font_families(&mut db);

        let query = Query {
            families: &[Family::Monospace],
            ..Default::default()
        };
        let id = db.query(&query).expect("Monospace should resolve");
        let face = db.face(id).expect("Face should exist");
        assert!(
            face.families.iter().any(|(name, _)| name.contains("Lilex")),
            "Monospace should map to Lilex, got {:?}",
            face.families
        );
    }
}
