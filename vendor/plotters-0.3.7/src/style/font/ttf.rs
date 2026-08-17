use std::borrow::{Borrow, Cow};
use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use lazy_static::lazy_static;

use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions},
    error::{FontLoadingError, GlyphLoadingError},
    family_name::FamilyName,
    font::Font,
    handle::Handle,
    hinting::HintingOptions,
    properties::{Properties, Style, Weight},
    source::SystemSource,
};

use ttf_parser::{Face, GlyphId};

use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::{Vector2F, Vector2I};

use super::{FontData, FontFamily, FontStyle, LayoutBox};

type FontResult<T> = Result<T, FontError>;

#[derive(Debug, Clone)]
pub enum FontError {
    LockError,
    NoSuchFont(String, String),
    FontLoadError(Arc<FontLoadingError>),
    GlyphError(Arc<GlyphLoadingError>),
}

impl std::fmt::Display for FontError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            FontError::LockError => write!(fmt, "Could not lock mutex"),
            FontError::NoSuchFont(family, style) => {
                write!(fmt, "No such font: {} {}", family, style)
            }
            FontError::FontLoadError(e) => write!(fmt, "Font loading error {}", e),
            FontError::GlyphError(e) => write!(fmt, "Glyph error {}", e),
        }
    }
}

impl std::error::Error for FontError {}

lazy_static! {
    static ref DATA_CACHE: RwLock<HashMap<String, FontResult<Handle>>> =
        RwLock::new(HashMap::new());
}

thread_local! {
    static FONT_SOURCE: SystemSource = SystemSource::new();
    static FONT_OBJECT_CACHE: RefCell<HashMap<String, FontExt>> = RefCell::new(HashMap::new());
}

const PLACEHOLDER_CHAR: char = '�';

#[derive(Clone)]
struct FontExt {
    inner: Font,
    face: Option<Face<'static>>,
}

impl Drop for FontExt {
    fn drop(&mut self) {
        // We should make sure the face object dead first
        self.face.take();
    }
}

impl FontExt {
    fn new(font: Font) -> Self {
        let handle = font.handle();
        let (data, idx) = match handle.as_ref() {
            Some(Handle::Memory { bytes, font_index }) => (&bytes[..], *font_index),
            _ => unreachable!(),
        };
        let face = unsafe {
            std::mem::transmute::<Option<_>, Option<Face<'static>>>(
                ttf_parser::Face::parse(data, idx).ok(),
            )
        };
        Self { inner: font, face }
    }

    fn query_kerning_table(&self, prev: u32, next: u32) -> f32 {
        if let Some(face) = self.face.as_ref() {
            if let Some(kern) = face.tables().kern {
                let kern = kern
                    .subtables
                    .into_iter()
                    .filter(|st| st.horizontal && !st.variable)
                    .filter_map(|st| st.glyphs_kerning(GlyphId(prev as u16), GlyphId(next as u16)))
                    .next()
                    .unwrap_or(0);
                return kern as f32;
            }
        }
        0.0
    }
}

impl std::ops::Deref for FontExt {
    type Target = Font;
    fn deref(&self) -> &Font {
        &self.inner
    }
}

/// Lazily load font data. Font type doesn't own actual data, which
/// lives in the cache.
fn load_font_data(face: FontFamily, style: FontStyle) -> FontResult<FontExt> {
    let key = match style {
        FontStyle::Normal => Cow::Borrowed(face.as_str()),
        _ => Cow::Owned(format!("{}, {}", face.as_str(), style.as_str())),
    };

    // First, we try to find the font object for current thread
    if let Some(font_object) = FONT_OBJECT_CACHE.with(|font_object_cache| {
        font_object_cache
            .borrow()
            .get(Borrow::<str>::borrow(&key))
            .cloned()
    }) {
        return Ok(font_object);
    }

    // Then we need to check if the data cache contains the font data
    let cache = DATA_CACHE.read().unwrap();
    if let Some(data) = cache.get(Borrow::<str>::borrow(&key)) {
        data.clone().map(|handle| {
            handle
                .load()
                .map(FontExt::new)
                .map_err(|e| FontError::FontLoadError(Arc::new(e)))
        })??;
    }
    drop(cache);

    // Otherwise we should load from system
    let mut properties = Properties::new();
    match style {
        FontStyle::Normal => properties.style(Style::Normal),
        FontStyle::Italic => properties.style(Style::Italic),
        FontStyle::Oblique => properties.style(Style::Oblique),
        FontStyle::Bold => properties.weight(Weight::BOLD),
    };

    let family = match face {
        FontFamily::Serif => FamilyName::Serif,
        FontFamily::SansSerif => FamilyName::SansSerif,
        FontFamily::Monospace => FamilyName::Monospace,
        FontFamily::Name(name) => FamilyName::Title(name.to_owned()),
    };

    let make_not_found_error =
        || FontError::NoSuchFont(face.as_str().to_owned(), style.as_str().to_owned());

    if let Ok(handle) = FONT_SOURCE
        .with(|source| source.select_best_match(&[family, FamilyName::SansSerif], &properties))
    {
        let font = handle
            .load()
            .map(FontExt::new)
            .map_err(|e| FontError::FontLoadError(Arc::new(e)));
        let (should_cache, data) = match font.as_ref().map(|f| f.handle()) {
            Ok(None) => (false, Err(FontError::LockError)),
            Ok(Some(handle)) => (true, Ok(handle)),
            Err(e) => (true, Err(e.clone())),
        };

        if should_cache {
            DATA_CACHE
                .write()
                .map_err(|_| FontError::LockError)?
                .insert(key.clone().into_owned(), data);
        }

        if let Ok(font) = font.as_ref() {
            FONT_OBJECT_CACHE.with(|font_object_cache| {
                font_object_cache
                    .borrow_mut()
                    .insert(key.into_owned(), font.clone());
            });
        }

        return font;
    }
    Err(make_not_found_error())
}

#[derive(Clone)]
pub struct FontDataInternal(FontExt);

impl FontData for FontDataInternal {
    type ErrorType = FontError;

    fn new(family: FontFamily, style: FontStyle) -> Result<Self, FontError> {
        Ok(FontDataInternal(load_font_data(family, style)?))
    }

    fn estimate_layout(&self, size: f64, text: &str) -> Result<LayoutBox, Self::ErrorType> {
        let font = &self.0;
        let pixel_per_em = size / 1.24;
        let metrics = font.metrics();

        let font = &self.0;

        let mut x_in_unit = 0f32;

        let mut prev = None;
        let place_holder = font.glyph_for_char(PLACEHOLDER_CHAR);

        for c in text.chars() {
            if let Some(glyph_id) = font.glyph_for_char(c).or(place_holder) {
                if let Ok(size) = font.advance(glyph_id) {
                    x_in_unit += size.x();
                }
                if let Some(pc) = prev {
                    x_in_unit += font.query_kerning_table(pc, glyph_id);
                }
                prev = Some(glyph_id);
            }
        }

        let x_pixels = x_in_unit * pixel_per_em as f32 / metrics.units_per_em as f32;

        Ok(((0, 0), (x_pixels as i32, pixel_per_em as i32)))
    }

    fn draw<E, DrawFunc: FnMut(i32, i32, f32) -> Result<(), E>>(
        &self,
        (base_x, mut base_y): (i32, i32),
        size: f64,
        text: &str,
        mut draw: DrawFunc,
    ) -> Result<Result<(), E>, Self::ErrorType> {
        let em = (size / 1.24) as f32;

        let mut x = base_x as f32;
        let font = &self.0;
        let metrics = font.metrics();

        let canvas_size = size as usize;

        base_y -= (0.24 * em) as i32;

        let mut prev = None;
        let place_holder = font.glyph_for_char(PLACEHOLDER_CHAR);

        let mut result = Ok(());

        for c in text.chars() {
            if let Some(glyph_id) = font.glyph_for_char(c).or(place_holder) {
                if let Some(pc) = prev {
                    x += font.query_kerning_table(pc, glyph_id) * em / metrics.units_per_em as f32;
                }

                let mut canvas = Canvas::new(Vector2I::splat(canvas_size as i32), Format::A8);

                result = font
                    .rasterize_glyph(
                        &mut canvas,
                        glyph_id,
                        em,
                        Transform2F::from_translation(Vector2F::new(0.0, em)),
                        HintingOptions::None,
                        RasterizationOptions::GrayscaleAa,
                    )
                    .map_err(|e| FontError::GlyphError(Arc::new(e)))
                    .and(result);

                let base_x = x as i32;

                for dy in 0..canvas_size {
                    for dx in 0..canvas_size {
                        let alpha = canvas.pixels[dy * canvas_size + dx] as f32 / 255.0;
                        if let Err(e) = draw(base_x + dx as i32, base_y + dy as i32, alpha) {
                            return Ok(Err(e));
                        }
                    }
                }

                x += font.advance(glyph_id).map(|size| size.x()).unwrap_or(0.0) * em
                    / metrics.units_per_em as f32;

                prev = Some(glyph_id);
            }
        }
        result?;
        Ok(Ok(()))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_font_cache() -> FontResult<()> {
        // We cannot only check the size of font cache, because
        // the test case may be run in parallel. Thus the font cache
        // may contains other fonts.
        let _a = load_font_data(FontFamily::Serif, FontStyle::Normal)?;
        assert!(DATA_CACHE.read().unwrap().contains_key("serif"));

        let _b = load_font_data(FontFamily::Serif, FontStyle::Normal)?;
        assert!(DATA_CACHE.read().unwrap().contains_key("serif"));

        // TODO: Check they are the same

        Ok(())
    }
}
