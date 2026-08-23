use crate::{
    AssetSource, DefaultAppearance, DevicePixels, IsZero, RenderImage, Result, SharedString, Size,
    swap_rgba_pa_to_bgra,
};
use image::Frame;
use resvg::tiny_skia::Pixmap;
use smallvec::SmallVec;
use std::{
    borrow::Cow,
    hash::Hash,
    ops::Range,
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

/// A parsed SVG document that can be rasterized at any scale.
///
/// Produced by [`SvgRenderer::parse_svg`] and rasterized by
/// [`SvgRenderer::render_parsed`]. Parsing resolves fonts and converts text
/// to paths, so callers that need to rasterize the same SVG at multiple
/// scales should retain this value to avoid re-paying the parse cost.
pub struct ParsedSvg(usvg::Tree);

/// The size in which to rasterize the SVG.
#[derive(Clone, Copy)]
pub enum SvgSize {
    /// A width in device pixels. The SVG retains its aspect ratio.
    Size(Size<DevicePixels>),
    /// An exact width and height in device pixels.
    ExactSize(Size<DevicePixels>),
    /// A logical scaling factor to apply to the size provided by the SVG.
    ScaleFactor(f32),
}

impl From<f32> for SvgSize {
    fn from(scale_factor: f32) -> Self {
        Self::ScaleFactor(scale_factor)
    }
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

    /// Parses SVG data into a [`ParsedSvg`] that can be rasterized at any scale.
    #[ztracing::instrument(skip_all)]
    pub fn parse_svg(&self, bytes: &[u8]) -> Result<ParsedSvg, usvg::Error> {
        usvg::Tree::from_data(bytes, &self.usvg_options).map(ParsedSvg)
    }

    /// Rasterizes a previously parsed SVG into an image buffer.
    #[ztracing::instrument(skip_all)]
    pub fn render_parsed(
        &self,
        svg: &ParsedSvg,
        size: impl Into<SvgSize>,
    ) -> Result<Arc<RenderImage>, usvg::Error> {
        let (size, image_scale_factor) = match size.into() {
            SvgSize::Size(size) => (SvgSize::Size(size), 1.0),
            SvgSize::ExactSize(size) => (SvgSize::ExactSize(size), 1.0),
            SvgSize::ScaleFactor(scale_factor) => (
                SvgSize::ScaleFactor(scale_factor * SMOOTH_SVG_SCALE_FACTOR),
                SMOOTH_SVG_SCALE_FACTOR,
            ),
        };
        let pixmap = rasterize_tree(&svg.0, size)?;
        let mut buffer =
            image::ImageBuffer::from_raw(pixmap.width(), pixmap.height(), pixmap.take()).unwrap();

        for pixel in buffer.chunks_exact_mut(4) {
            swap_rgba_pa_to_bgra(pixel);
        }

        let mut image = RenderImage::new(SmallVec::from_const([Frame::new(buffer)]));
        image.scale_factor = image_scale_factor;
        Ok(Arc::new(image))
    }

    /// Renders the given bytes into an image buffer.
    pub fn render_single_frame(
        &self,
        bytes: &[u8],
        scale_factor: f32,
    ) -> Result<Arc<RenderImage>, usvg::Error> {
        let svg = self.parse_svg(bytes)?;
        self.render_parsed(&svg, scale_factor)
    }

    /// Parses SVG data into a [`ParsedSvg`] that can be rasterized at any scale.
    ///
    /// Unlike [`SvgRenderer::parse_svg`], `@media (prefers-color-scheme: ...)`
    /// rules are resolved against the provided appearance before parsing. usvg's
    /// CSS parser drops at-rules entirely, so without this such rules would be
    /// ignored and the SVG would always render with its default palette.
    pub fn parse_svg_with_appearance(
        &self,
        bytes: &[u8],
        appearance: DefaultAppearance,
    ) -> Result<ParsedSvg, usvg::Error> {
        self.parse_svg(resolve_preferred_color_scheme(bytes, appearance).as_ref())
    }

    /// Renders the given bytes into an image buffer, resolving
    /// `@media (prefers-color-scheme: ...)` rules against the provided
    /// appearance before parsing. See [`SvgRenderer::parse_svg_with_appearance`].
    pub fn render_single_frame_with_appearance(
        &self,
        bytes: &[u8],
        scale_factor: f32,
        appearance: DefaultAppearance,
    ) -> Result<Arc<RenderImage>, usvg::Error> {
        self.render_single_frame(
            resolve_preferred_color_scheme(bytes, appearance).as_ref(),
            scale_factor,
        )
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
        let tree = usvg::Tree::from_data(bytes, &self.usvg_options)?;
        rasterize_tree(&tree, size)
    }
}

fn rasterize_tree(tree: &usvg::Tree, size: SvgSize) -> Result<Pixmap, usvg::Error> {
    // Cap the size of the rendered pixmap to avoid texture allocation panics
    // Related issue: #56466
    const MAX_SIZE: f32 = 8192.0;

    let svg_size = tree.size();
    let (mut width, mut height) = match size {
        SvgSize::Size(size) => {
            let scale = i32::from(size.width) as f32 / svg_size.width();
            (svg_size.width() * scale, svg_size.height() * scale)
        }
        SvgSize::ExactSize(size) => (i32::from(size.width) as f32, i32::from(size.height) as f32),
        SvgSize::ScaleFactor(scale) => (svg_size.width() * scale, svg_size.height() * scale),
    };

    if width > MAX_SIZE {
        log::warn!("Attempted to render pixmap where width ({width}) > MAX_SIZE ({MAX_SIZE})");
    }
    if height > MAX_SIZE {
        log::warn!("Attempted to render pixmap where height ({height}) > MAX_SIZE ({MAX_SIZE})");
    }
    let scale = (MAX_SIZE / width).min(MAX_SIZE / height).min(1.0);
    width *= scale;
    height *= scale;

    // Render the SVG to a pixmap with the specified width and height.
    let mut pixmap = resvg::tiny_skia::Pixmap::new(width as u32, height as u32)
        .ok_or(usvg::Error::InvalidSize)?;

    let transform = resvg::tiny_skia::Transform::from_scale(
        width / svg_size.width(),
        height / svg_size.height(),
    );

    resvg::render(tree, transform, &mut pixmap.as_mut());

    Ok(pixmap)
}

/// Maximum nesting of `@media` blocks we resolve. Deeply nested media queries
/// are invalid CSS in practice; the cap guards against pathological inputs.
const MAX_MEDIA_QUERY_DEPTH: usize = 8;

/// Resolves `@media (prefers-color-scheme: ...)` rules in the given SVG data
/// for the provided appearance.
///
/// usvg's CSS parser (simplecss) skips all at-rules, so media queries would be
/// dropped and the SVG would always render with its default palette. To match
/// browser behavior, blocks whose condition matches the appearance are unwrapped
/// (their declarations apply), all other `@media` blocks are removed. Conditions
/// that can't be evaluated count as non-matching, which preserves the previous
/// behavior of dropping the block.
pub(crate) fn resolve_preferred_color_scheme<'a>(
    bytes: &'a [u8],
    appearance: DefaultAppearance,
) -> Cow<'a, [u8]> {
    let Ok(svg) = str::from_utf8(bytes) else {
        return Cow::Borrowed(bytes);
    };
    // Fast path: bail out before allocating a lowercased copy.
    if !svg.to_ascii_lowercase().contains("@media") {
        return Cow::Borrowed(bytes);
    }
    Cow::Owned(rewrite_media_queries(svg, appearance, 0).into_bytes())
}

fn rewrite_media_queries(svg: &str, appearance: DefaultAppearance, depth: usize) -> String {
    // Media queries are case-insensitive; lowercase once for scanning. This is
    // byte-for-byte length preserving, so offsets stay valid for `svg`.
    let lowered = svg.to_ascii_lowercase();
    let mut output = String::with_capacity(svg.len());
    let mut cursor = 0;

    while let Some(at_rule_start) = lowered[cursor..]
        .find("@media")
        .map(|offset| cursor + offset)
    {
        let Some(query) = scan_media_query(svg, &lowered, at_rule_start, appearance) else {
            // Malformed at-rule: keep everything from here on untouched rather
            // than risk corrupting the SVG.
            break;
        };
        output.push_str(&svg[cursor..query.at_rule_start]);
        if query.matches {
            if depth < MAX_MEDIA_QUERY_DEPTH {
                output.push_str(&rewrite_media_queries(
                    &svg[query.contents.clone()],
                    appearance,
                    depth + 1,
                ));
            } else {
                output.push_str(&svg[query.contents.clone()]);
            }
        }
        cursor = query.end;
    }

    output.push_str(&svg[cursor..]);
    output
}

struct MediaQueryScan {
    at_rule_start: usize,
    /// Offset just past the closing brace of the at-rule's block.
    end: usize,
    matches: bool,
    contents: Range<usize>,
}

fn scan_media_query(
    svg: &str,
    lowered: &str,
    at_rule_start: usize,
    appearance: DefaultAppearance,
) -> Option<MediaQueryScan> {
    // Media conditions may hold several parenthesized features, so rather than
    // matching a single paren pair, take everything between the keyword and the
    // block's opening brace as the condition.
    let condition_start = skip_whitespace(&lowered[at_rule_start + "@media".len()..])
        + at_rule_start
        + "@media".len();
    let brace_open = lowered[condition_start..].find('{')? + condition_start;
    let condition = lowered[condition_start..brace_open].trim();
    if !is_plausible_media_condition(condition) {
        return None;
    }
    let matches = evaluate_color_scheme_condition(condition, appearance);

    let (contents, end) = scan_balanced_block(svg, brace_open)?;
    Some(MediaQueryScan {
        at_rule_start,
        end,
        matches,
        contents,
    })
}

/// Guards against treating occurrences of `@media` in SVG text content as
/// at-rules: real conditions are short, and every comma-separated query starts
/// with a media feature, a leading `not`/`only`, or a legacy media type keyword.
fn is_plausible_media_condition(condition: &str) -> bool {
    const MAX_CONDITION_LEN: usize = 256;
    if condition.is_empty() || condition.len() > MAX_CONDITION_LEN {
        return false;
    }
    let body = condition
        .strip_prefix("not ")
        .or_else(|| condition.strip_prefix("only "))
        .map(str::trim)
        .unwrap_or(condition);
    body.split(',').any(|query| {
        let query = query.trim();
        if query.starts_with('(') {
            return true;
        }
        let media_type = query
            .split_once(char::is_whitespace)
            .map(|(media_type, _)| media_type)
            .unwrap_or(query);
        matches!(media_type, "all" | "print" | "screen" | "speech")
    })
}

/// Returns the number of bytes of leading whitespace.
fn skip_whitespace(text: &str) -> usize {
    text.find(|c: char| !c.is_whitespace())
        .unwrap_or(text.len())
}

fn evaluate_color_scheme_condition(condition: &str, appearance: DefaultAppearance) -> bool {
    let mut body = condition.trim();
    let mut negated = false;
    if let Some(rest) = body.strip_prefix("not ") {
        negated = true;
        body = rest.trim();
    } else if let Some(rest) = body.strip_prefix("only ") {
        body = rest.trim();
    }
    // Normalize whitespace so logical keywords can be matched reliably even
    // though feature values may contain spaces.
    let normalized = body.split_whitespace().collect::<Vec<_>>().join(" ");
    // Comma-separated queries match if any of them matches.
    let matches = normalized.split(',').any(|query| {
        query.split(" or ").any(|disjunct| {
            disjunct
                .split(" and ")
                .all(|feature| evaluate_color_scheme_feature(feature, appearance).unwrap_or(false))
        })
    });
    matches != negated
}

/// Evaluates one media feature or legacy media type. Returns `None` for
/// features we can't evaluate, which makes their whole `and`-chain non-matching.
fn evaluate_color_scheme_feature(feature: &str, appearance: DefaultAppearance) -> Option<bool> {
    let feature = feature.trim();
    let feature = feature
        .strip_prefix('(')
        .and_then(|feature| feature.strip_suffix(')'))
        .unwrap_or(feature);
    let Some((name, value)) = feature.split_once(':') else {
        // Legacy media types: Zed renders to a screen.
        return match feature.trim() {
            "all" | "screen" => Some(true),
            "print" | "speech" => Some(false),
            _ => None,
        };
    };
    if name.trim() != "prefers-color-scheme" {
        return None;
    }
    match value.trim() {
        "dark" => Some(appearance == DefaultAppearance::Dark),
        "light" | "no-preference" => Some(appearance == DefaultAppearance::Light),
        _ => None,
    }
}

/// Given the offset of an opening brace, finds its matching closing brace,
/// skipping braces inside quoted strings and comments. Returns the contents
/// range and the offset just past the closing brace.
fn scan_balanced_block(svg: &str, open_brace: usize) -> Option<(Range<usize>, usize)> {
    let mut depth = 1usize;
    let mut chars = svg[open_brace + 1..].char_indices().peekable();
    while let Some((offset, c)) = chars.next() {
        match c {
            '"' | '\'' => {
                let mut escaped = false;
                for (_, quoted) in chars.by_ref() {
                    if escaped {
                        escaped = false;
                    } else if quoted == '\\' {
                        escaped = true;
                    } else if quoted == c {
                        break;
                    }
                }
            }
            '/' if chars.peek().is_some_and(|(_, next)| *next == '*') => {
                chars.next();
                let mut pending_star = false;
                for (_, commented) in chars.by_ref() {
                    if pending_star && commented == '/' {
                        break;
                    }
                    pending_star = commented == '*';
                }
            }
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    let close = open_brace + 1 + offset;
                    return Some((open_brace + 1..close, close + 1));
                }
            }
            _ => {}
        }
    }
    None
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

    fn color_scheme_svg(default_fill: &str, media_fill: &str, scheme: &str) -> Vec<u8> {
        format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><style>rect{{fill:{default_fill}}}@media (prefers-color-scheme: {scheme}){{rect{{fill:{media_fill}}}}}</style><rect width="10" height="10"/></svg>"#
        )
        .into_bytes()
    }

    #[test]
    fn unwraps_matching_media_block_and_removes_non_matching() {
        let svg = color_scheme_svg("black", "white", "dark");
        let resolved = resolve_preferred_color_scheme(&svg, DefaultAppearance::Dark);
        assert_eq!(
            String::from_utf8(resolved.into_owned()).unwrap(),
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><style>rect{fill:black}rect{fill:white}</style><rect width="10" height="10"/></svg>"#
        );

        let svg = color_scheme_svg("black", "white", "dark");
        let resolved = resolve_preferred_color_scheme(&svg, DefaultAppearance::Light);
        assert_eq!(
            String::from_utf8(resolved.into_owned()).unwrap(),
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><style>rect{fill:black}</style><rect width="10" height="10"/></svg>"#
        );
    }

    #[test]
    fn unwrapped_declarations_override_in_cascade_order() {
        // A dark-first SVG whose light palette arrives via the media query must
        // end up with the light declaration last so it wins the cascade.
        let svg = color_scheme_svg("white", "black", "light");
        let resolved = resolve_preferred_color_scheme(&svg, DefaultAppearance::Light);
        assert_eq!(
            String::from_utf8(resolved.into_owned()).unwrap(),
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><style>rect{fill:white}rect{fill:black}</style><rect width="10" height="10"/></svg>"#
        );
    }

    #[test]
    fn media_keyword_and_condition_are_case_insensitive() {
        let svg =
            br#"<svg><style>@MEDIA ( PREFERS-COLOR-SCHEME: DARK ){a{fill:red}}</style></svg>"#;
        let resolved = resolve_preferred_color_scheme(svg, DefaultAppearance::Dark);
        assert_eq!(
            String::from_utf8(resolved.into_owned()).unwrap(),
            "<svg><style>a{fill:red}</style></svg>"
        );
    }

    #[test]
    fn conditions_with_unevaluable_features_never_match() {
        let svg = br#"<svg><style>@media (min-width: 10px) and (prefers-color-scheme: dark){a{fill:red}}</style></svg>"#;
        for appearance in [DefaultAppearance::Light, DefaultAppearance::Dark] {
            let resolved = resolve_preferred_color_scheme(svg, appearance);
            assert_eq!(
                String::from_utf8(resolved.into_owned()).unwrap(),
                "<svg><style></style></svg>"
            );
        }
    }

    #[test]
    fn legacy_media_type_queries_never_match() {
        let svg = br#"<svg><style>@media print {a{fill:red}}</style><style>@media screen and (prefers-color-scheme: dark) {b{fill:red}}</style></svg>"#;
        for appearance in [DefaultAppearance::Light, DefaultAppearance::Dark] {
            let resolved = resolve_preferred_color_scheme(svg, appearance);
            assert_eq!(
                String::from_utf8(resolved.into_owned()).unwrap(),
                if appearance == DefaultAppearance::Dark {
                    "<svg><style></style><style>b{fill:red}</style></svg>"
                } else {
                    "<svg><style></style><style></style></svg>"
                }
            );
        }
    }

    #[test]
    fn negated_conditions_invert_the_result() {
        let svg =
            br#"<svg><style>@media not (prefers-color-scheme: dark){a{fill:red}}</style></svg>"#;
        let light = resolve_preferred_color_scheme(svg, DefaultAppearance::Light);
        assert_eq!(
            String::from_utf8(light.into_owned()).unwrap(),
            "<svg><style>a{fill:red}</style></svg>"
        );
        let dark = resolve_preferred_color_scheme(svg, DefaultAppearance::Dark);
        assert_eq!(
            String::from_utf8(dark.into_owned()).unwrap(),
            "<svg><style></style></svg>"
        );
    }

    #[test]
    fn only_prefix_does_not_affect_matching() {
        let svg = br#"<svg><style>@media only screen and (prefers-color-scheme: dark){a{fill:red}}</style></svg>"#;
        let dark = resolve_preferred_color_scheme(svg, DefaultAppearance::Dark);
        assert_eq!(
            String::from_utf8(dark.into_owned()).unwrap(),
            "<svg><style>a{fill:red}</style></svg>"
        );
        let light = resolve_preferred_color_scheme(svg, DefaultAppearance::Light);
        assert_eq!(
            String::from_utf8(light.into_owned()).unwrap(),
            "<svg><style></style></svg>"
        );
    }

    #[test]
    fn comma_separated_queries_match_if_any_query_matches() {
        let svg =
            br#"<svg><style>@media print, (prefers-color-scheme: dark){a{fill:red}}</style></svg>"#;
        let dark = resolve_preferred_color_scheme(svg, DefaultAppearance::Dark);
        assert_eq!(
            String::from_utf8(dark.into_owned()).unwrap(),
            "<svg><style>a{fill:red}</style></svg>"
        );
        let light = resolve_preferred_color_scheme(svg, DefaultAppearance::Light);
        assert_eq!(
            String::from_utf8(light.into_owned()).unwrap(),
            "<svg><style></style></svg>"
        );
    }

    #[test]
    fn braces_inside_strings_and_comments_do_not_confuse_block_scanning() {
        let svg =
            br#"<svg><style>@media (prefers-color-scheme: dark) {/* } */a{content:"}"}}</style></svg>"#;
        let resolved = resolve_preferred_color_scheme(svg, DefaultAppearance::Dark);
        assert_eq!(
            String::from_utf8(resolved.into_owned()).unwrap(),
            "<svg><style>/* } */a{content:\"}\"}</style></svg>"
        );
    }

    #[test]
    fn malformed_media_rules_are_left_untouched() {
        let svg = br#"<svg><text>@media screen</text><style>a{fill:red}</style></svg>"#;
        let resolved = resolve_preferred_color_scheme(svg, DefaultAppearance::Dark);
        assert!(matches!(resolved, Cow::Owned(_)));
        assert_eq!(
            String::from_utf8(resolved.into_owned()).unwrap(),
            String::from_utf8_lossy(svg)
        );
    }

    #[test]
    fn data_without_media_queries_is_passed_through_unchanged() {
        let svg = br#"<svg><style>a{fill:red}</style></svg>"#;
        match resolve_preferred_color_scheme(svg, DefaultAppearance::Dark) {
            Cow::Borrowed(borrowed) => assert_eq!(borrowed, svg),
            Cow::Owned(_) => panic!("expected borrowed data"),
        }

        let invalid_utf8: &[u8] = &[0xff, 0xfe, b'@', b'm', b'e'];
        match resolve_preferred_color_scheme(invalid_utf8, DefaultAppearance::Dark) {
            Cow::Borrowed(borrowed) => assert_eq!(borrowed, invalid_utf8),
            Cow::Owned(_) => panic!("expected borrowed data"),
        }
    }

    #[test]
    fn rendered_pixels_follow_prefers_color_scheme() -> Result<()> {
        let renderer = SvgRenderer::new(Arc::new(()));
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 10 10"><style>rect{fill:black}@media (prefers-color-scheme: dark){rect{fill:white}}</style><rect width="10" height="10"/></svg>"#;

        let luminance_range = |image: &RenderImage| -> Option<(u32, u32)> {
            let mut min_luminance = u32::MAX;
            let mut max_luminance = 0;
            for pixel in image.as_bytes(0)?.chunks_exact(4) {
                let alpha = pixel[3] as u32;
                if alpha < 128 {
                    continue;
                }
                let blue = pixel[0] as u32;
                let green = pixel[1] as u32;
                let red = pixel[2] as u32;
                let luminance = (red * 299 + green * 587 + blue * 114) / 1000;
                min_luminance = min_luminance.min(luminance);
                max_luminance = max_luminance.max(luminance);
            }
            Some((min_luminance, max_luminance))
        };

        let dark_image =
            renderer.render_single_frame_with_appearance(svg, 1.0, DefaultAppearance::Dark)?;
        let (_, dark_max_luminance) = luminance_range(&dark_image).unwrap();
        assert!(
            dark_max_luminance > 200,
            "expected white pixels when rendering for dark appearance"
        );

        let light_image =
            renderer.render_single_frame_with_appearance(svg, 1.0, DefaultAppearance::Light)?;
        let (light_min_luminance, _) = luminance_range(&light_image).unwrap();
        assert!(
            light_min_luminance < 50,
            "expected black pixels when rendering for light appearance"
        );

        // Without appearance resolution the media query is dropped and only the
        // default palette renders.
        let legacy_image = renderer.render_single_frame(svg, 1.0)?;
        let (_, legacy_max_luminance) = luminance_range(&legacy_image).unwrap();
        assert!(
            legacy_max_luminance < 50,
            "expected legacy rendering to keep the default palette"
        );

        Ok(())
    }

    #[test]
    fn parse_svg_with_appearance_accepts_issue_report() -> Result<()> {
        let renderer = SvgRenderer::new(Arc::new(()));
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 120 40">
  <style>
      text {
          fill: black;
      }
      @media (prefers-color-scheme: dark) {
          text {
              fill: white;
          }
      }
  </style> <text x="10" y="25">Hello</text>
</svg>"#;
        let dark = renderer.parse_svg_with_appearance(svg, DefaultAppearance::Dark)?;
        let _image = renderer.render_parsed(&dark, SvgSize::ScaleFactor(1.0))?;
        let light = renderer.parse_svg_with_appearance(svg, DefaultAppearance::Light)?;
        let _image = renderer.render_parsed(&light, SvgSize::ScaleFactor(1.0))?;
        Ok(())
    }

    #[test]
    fn renders_parsed_svg_at_requested_size() -> Result<()> {
        let renderer = SvgRenderer::new(Arc::new(()));
        let svg = renderer.parse_svg(
            br#"<svg xmlns="http://www.w3.org/2000/svg" width="24pt" height="12pt"></svg>"#,
        )?;
        let requested_size = Size::new(DevicePixels(24), DevicePixels(12));
        let image = renderer.render_parsed(&svg, SvgSize::ExactSize(requested_size))?;

        assert_eq!(image.size(0), requested_size);
        Ok(())
    }

    #[test]
    fn preserves_aspect_ratio_for_width_constrained_size() -> Result<()> {
        let renderer = SvgRenderer::new(Arc::new(()));
        let svg = renderer.parse_svg(
            br#"<svg xmlns="http://www.w3.org/2000/svg" width="24pt" height="12pt"></svg>"#,
        )?;
        let image = renderer.render_parsed(
            &svg,
            SvgSize::Size(Size::new(DevicePixels(24), DevicePixels(24))),
        )?;

        assert_eq!(image.size(0), Size::new(DevicePixels(24), DevicePixels(12)));
        Ok(())
    }

    fn db_with_bundled_fonts() -> Database {
        let mut db = Database::new();
        db.load_font_data(IBM_PLEX_REGULAR.to_vec());
        db.load_font_data(LILEX_REGULAR.to_vec());
        db
    }

    #[test]
    fn text_with_split_glyph_clusters_in_mixed_fonts_does_not_panic() {
        let mut db = Database::new();
        db.load_font_data(IBM_PLEX_REGULAR.to_vec());
        db.load_font_data(LILEX_REGULAR.to_vec());
        let options = usvg::Options {
            fontdb: std::sync::Arc::new(db),
            ..Default::default()
        };

        // A base letter followed by a stack of combining marks. Under HarfBuzz's
        // default cluster merging every mark glyph shares the base's byte index,
        // which is the "glyph splitting" condition that triggered the panic. The
        // chunk must use two different fonts so the buggy merge path runs.
        let zalgo = "e\u{0301}\u{0302}\u{0303}\u{0304}\u{0306}\u{0307}\u{0308}\u{030a}";
        let svg = format!(
            r#"<svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg"><text font-family="Lilex" font-size="32">{zalgo}<tspan font-family="IBM Plex Sans">{zalgo}</tspan></text></svg>"#
        );

        // Before the fix this aborts via panic with a message like
        // "removal index (is 5) should be < len (is 5)".
        usvg::Tree::from_data(svg.as_bytes(), &options)
            .expect("SVG with mixed-font text should parse");
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
