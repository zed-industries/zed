use super::{FontData, FontFamily, FontStyle, LayoutBox};
use ab_glyph::{Font, FontRef, ScaleFont};
use core::fmt::{self, Display};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::error::Error;
use std::sync::RwLock;

struct FontMap {
    map: HashMap<String, FontRef<'static>>,
}
impl FontMap {
    fn new() -> Self {
        Self {
            map: HashMap::with_capacity(4),
        }
    }
    fn insert(&mut self, style: FontStyle, font: FontRef<'static>) -> Option<FontRef<'static>> {
        self.map.insert(style.as_str().to_string(), font)
    }
    // fn get(&self, style: FontStyle) -> Option<&FontRef<'static>> {
    //     self.map.get(style.as_str())
    // }
    fn get_fallback(&self, style: FontStyle) -> Option<&FontRef<'static>> {
        self.map
            .get(style.as_str())
            .or_else(|| self.map.get(FontStyle::Normal.as_str()))
    }
}

static FONTS: Lazy<RwLock<HashMap<String, FontMap>>> = Lazy::new(|| RwLock::new(HashMap::new()));
pub struct InvalidFont {
    _priv: (),
}

// Note for future contributors: There is nothing fundamental about the static reference requirement here.
// It would be reasonably easy to add a function which accepts an owned buffer,
// or even a reference counted buffer, instead.
/// Register a font in the fonts table.
///
/// The `name` parameter gives the name this font shall be referred to
/// in the other APIs, like `"sans-serif"`.
///
/// Unprovided font styles for a given name will fallback to `FontStyle::Normal`
/// if that is available for that name, when other functions lookup fonts which
/// are registered with this function.
///
/// The `bytes` parameter should be the complete contents
/// of an OpenType font file, like:
/// ```ignore
/// include_bytes!("FiraGO-Regular.otf")
/// ```
pub fn register_font(
    name: &str,
    style: FontStyle,
    bytes: &'static [u8],
) -> Result<(), InvalidFont> {
    let font = FontRef::try_from_slice(bytes).map_err(|_| InvalidFont { _priv: () })?;
    let mut lock = FONTS.write().unwrap();
    lock.entry(name.to_string())
        .or_insert_with(FontMap::new)
        .insert(style, font);
    Ok(())
}

#[derive(Clone)]
pub struct FontDataInternal {
    font_ref: FontRef<'static>,
}

#[derive(Debug, Clone)]
pub enum FontError {
    /// No idea what the problem is
    Unknown,
    /// No font data available for the requested family and style.
    FontUnavailable,
}
impl Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Since it makes literally no difference to how we'd format
        // this, just delegate to the derived Debug formatter.
        write!(f, "{:?}", self)
    }
}
impl Error for FontError {}

impl FontData for FontDataInternal {
    // TODO: can we rename this to `Error`?
    type ErrorType = FontError;
    fn new(family: FontFamily<'_>, style: FontStyle) -> Result<Self, Self::ErrorType> {
        Ok(Self {
            font_ref: FONTS
                .read()
                .unwrap()
                .get(family.as_str())
                .and_then(|fam| fam.get_fallback(style))
                .ok_or(FontError::FontUnavailable)?
                .clone(),
        })
    }
    // TODO: ngl, it makes no sense that this uses the same error type as `new`
    fn estimate_layout(&self, size: f64, text: &str) -> Result<LayoutBox, Self::ErrorType> {
        let pixel_per_em = size / 1.24;
        // let units_per_em = self.font_ref.units_per_em().unwrap();
        let font = self.font_ref.as_scaled(size as f32);

        let mut x_pixels = 0f32;

        let mut prev = None;
        for c in text.chars() {
            let glyph_id = font.glyph_id(c);
            let size = font.h_advance(glyph_id);
            x_pixels += size;
            if let Some(pc) = prev {
                x_pixels += font.kern(pc, glyph_id);
            }
            prev = Some(glyph_id);
        }

        Ok(((0, 0), (x_pixels as i32, pixel_per_em as i32)))
    }
    fn draw<E, DrawFunc: FnMut(i32, i32, f32) -> Result<(), E>>(
        &self,
        pos: (i32, i32),
        size: f64,
        text: &str,
        mut draw: DrawFunc,
    ) -> Result<Result<(), E>, Self::ErrorType> {
        let font = self.font_ref.as_scaled(size as f32);
        let mut draw = |x: i32, y: i32, c| {
            let (base_x, base_y) = pos;
            draw(base_x + x, base_y + y, c)
        };
        let mut x_shift = 0f32;
        let mut prev = None;
        for c in text.chars() {
            if let Some(pc) = prev {
                x_shift += font.kern(font.glyph_id(pc), font.glyph_id(c));
            }
            prev = Some(c);
            let glyph = font.scaled_glyph(c);
            if let Some(q) = font.outline_glyph(glyph) {
                let rect = q.px_bounds();
                let y_shift = ((size as f32) / 2.0 + rect.min.y) as i32;
                let x_shift = x_shift as i32;
                let mut buf = vec![];
                q.draw(|x, y, c| buf.push((x, y, c)));
                for (x, y, c) in buf {
                    draw(x as i32 + x_shift, y as i32 + y_shift, c).map_err(|_e| {
                        // Note: If ever `plotters` adds a tracing or logging crate,
                        // this would be a good place to use it.
                        FontError::Unknown
                    })?;
                }
            }
            x_shift += font.h_advance(font.glyph_id(c));
        }
        Ok(Ok(()))
    }
}
