use crate::geometry::{
    rect::RectI,
    transform2d::Transform2F,
    vector::{vec2f, Vector2F},
};
use anyhow::{anyhow, Result};
use cocoa::appkit::{CGFloat, CGPoint};
use core_graphics::{
    base::CGGlyph, color_space::CGColorSpace, context::CGContext, geometry::CGAffineTransform,
};
pub use font_kit::properties::{Properties, Weight};
use font_kit::{
    canvas::RasterizationOptions, font::Font, hinting::HintingOptions,
    loaders::core_text::NativeFont, metrics::Metrics, source::SystemSource,
};
use ordered_float::OrderedFloat;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use std::{collections::HashMap, sync::Arc};

#[allow(non_upper_case_globals)]
const kCGImageAlphaOnly: u32 = 7;

pub type GlyphId = u32;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FamilyId(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontId(usize);

pub struct FontCache(RwLock<FontCacheState>);

pub struct FontCacheState {
    source: SystemSource,
    families: Vec<Family>,
    fonts: Vec<Arc<Font>>,
    font_names: Vec<Arc<String>>,
    font_selections: HashMap<FamilyId, HashMap<Properties, FontId>>,
    metrics: HashMap<FontId, Metrics>,
    native_fonts: HashMap<(FontId, OrderedFloat<f32>), NativeFont>,
    fonts_by_name: HashMap<Arc<String>, FontId>,
    emoji_font_id: Option<FontId>,
}

unsafe impl Send for FontCache {}

struct Family {
    name: String,
    font_ids: Vec<FontId>,
}

impl FontCache {
    pub fn new() -> Self {
        Self(RwLock::new(FontCacheState {
            source: SystemSource::new(),
            families: Vec::new(),
            fonts: Vec::new(),
            font_names: Vec::new(),
            font_selections: HashMap::new(),
            metrics: HashMap::new(),
            native_fonts: HashMap::new(),
            fonts_by_name: HashMap::new(),
            emoji_font_id: None,
        }))
    }

    pub fn load_family(&self, names: &[&str]) -> Result<FamilyId> {
        for name in names {
            let state = self.0.upgradable_read();

            if let Some(ix) = state.families.iter().position(|f| f.name == *name) {
                return Ok(FamilyId(ix));
            }

            let mut state = RwLockUpgradableReadGuard::upgrade(state);

            if let Ok(handle) = state.source.select_family_by_name(name) {
                if handle.is_empty() {
                    continue;
                }

                let family_id = FamilyId(state.families.len());
                let mut font_ids = Vec::new();
                for font in handle.fonts() {
                    let font = font.load()?;
                    if font.glyph_for_char('m').is_none() {
                        return Err(anyhow!("font must contain a glyph for the 'm' character"));
                    }
                    font_ids.push(push_font(&mut state, font));
                }

                state.families.push(Family {
                    name: String::from(*name),
                    font_ids,
                });
                return Ok(family_id);
            }
        }

        Err(anyhow!(
            "could not find a non-empty font family matching one of the given names"
        ))
    }

    pub fn default_font(&self, family_id: FamilyId) -> FontId {
        self.select_font(family_id, &Properties::default()).unwrap()
    }

    pub fn select_font(&self, family_id: FamilyId, properties: &Properties) -> Result<FontId> {
        let inner = self.0.upgradable_read();
        if let Some(font_id) = inner
            .font_selections
            .get(&family_id)
            .and_then(|f| f.get(properties))
        {
            Ok(*font_id)
        } else {
            let mut inner = RwLockUpgradableReadGuard::upgrade(inner);
            let family = &inner.families[family_id.0];
            let candidates = family
                .font_ids
                .iter()
                .map(|font_id| inner.fonts[font_id.0].properties())
                .collect::<Vec<_>>();
            let idx = font_kit::matching::find_best_match(&candidates, properties)?;
            let font_id = family.font_ids[idx];

            inner
                .font_selections
                .entry(family_id)
                .or_default()
                .insert(properties.clone(), font_id);
            Ok(font_id)
        }
    }

    pub fn font(&self, font_id: FontId) -> Arc<Font> {
        self.0.read().fonts[font_id.0].clone()
    }

    pub fn font_name(&self, font_id: FontId) -> Arc<String> {
        self.0.read().font_names[font_id.0].clone()
    }

    pub fn metric<F, T>(&self, font_id: FontId, f: F) -> T
    where
        F: FnOnce(&Metrics) -> T,
        T: 'static,
    {
        let state = self.0.upgradable_read();
        if let Some(metrics) = state.metrics.get(&font_id) {
            f(metrics)
        } else {
            let metrics = state.fonts[font_id.0].metrics();
            let metric = f(&metrics);
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            state.metrics.insert(font_id, metrics);
            metric
        }
    }

    pub fn is_emoji(&self, font_id: FontId) -> bool {
        self.0
            .read()
            .emoji_font_id
            .map_or(false, |emoji_font_id| emoji_font_id == font_id)
    }

    pub fn bounding_box(&self, font_id: FontId, font_size: f32) -> Vector2F {
        let bounding_box = self.metric(font_id, |m| m.bounding_box);
        let width = self.scale_metric(bounding_box.width(), font_id, font_size);
        let height = self.scale_metric(bounding_box.height(), font_id, font_size);
        vec2f(width, height)
    }

    pub fn line_height(&self, font_id: FontId, font_size: f32) -> f32 {
        let bounding_box = self.metric(font_id, |m| m.bounding_box);
        self.scale_metric(bounding_box.height(), font_id, font_size)
    }

    pub fn cap_height(&self, font_id: FontId, font_size: f32) -> f32 {
        self.scale_metric(self.metric(font_id, |m| m.cap_height), font_id, font_size)
    }

    pub fn ascent(&self, font_id: FontId, font_size: f32) -> f32 {
        self.scale_metric(self.metric(font_id, |m| m.ascent), font_id, font_size)
    }

    pub fn descent(&self, font_id: FontId, font_size: f32) -> f32 {
        self.scale_metric(self.metric(font_id, |m| m.descent), font_id, font_size)
    }

    pub fn render_glyph(
        &self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        scale_factor: f32,
    ) -> Option<(RectI, Vec<u8>)> {
        let font = self.font(font_id);
        let scale = Transform2F::from_scale(scale_factor);
        let bounds = font
            .raster_bounds(
                glyph_id,
                font_size,
                scale,
                HintingOptions::None,
                RasterizationOptions::GrayscaleAa,
            )
            .ok()?;

        if bounds.width() == 0 || bounds.height() == 0 {
            None
        } else {
            let mut pixels = vec![0; bounds.width() as usize * bounds.height() as usize];
            let ctx = CGContext::create_bitmap_context(
                Some(pixels.as_mut_ptr() as *mut _),
                bounds.width() as usize,
                bounds.height() as usize,
                8,
                bounds.width() as usize,
                &CGColorSpace::create_device_gray(),
                kCGImageAlphaOnly,
            );

            // Move the origin to bottom left and account for scaling, this
            // makes drawing text consistent with the font-kit's raster_bounds.
            ctx.translate(0.0, bounds.height() as CGFloat);
            let transform = scale.translate(-bounds.origin().to_f32());
            ctx.set_text_matrix(&CGAffineTransform {
                a: transform.matrix.m11() as CGFloat,
                b: -transform.matrix.m21() as CGFloat,
                c: -transform.matrix.m12() as CGFloat,
                d: transform.matrix.m22() as CGFloat,
                tx: transform.vector.x() as CGFloat,
                ty: -transform.vector.y() as CGFloat,
            });

            ctx.set_font(&font.native_font().copy_to_CGFont());
            ctx.set_font_size(font_size as CGFloat);
            ctx.show_glyphs_at_positions(&[glyph_id as CGGlyph], &[CGPoint::new(0.0, 0.0)]);

            Some((bounds, pixels))
        }
    }

    fn emoji_font_id(&self) -> Result<FontId> {
        let state = self.0.upgradable_read();

        if let Some(font_id) = state.emoji_font_id {
            Ok(font_id)
        } else {
            let handle = state.source.select_family_by_name("Apple Color Emoji")?;
            let font = handle
                .fonts()
                .first()
                .ok_or(anyhow!("no fonts in Apple Color Emoji font family"))?
                .load()?;
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            let font_id = push_font(&mut state, font);
            state.emoji_font_id = Some(font_id);
            Ok(font_id)
        }
    }

    pub fn scale_metric(&self, metric: f32, font_id: FontId, font_size: f32) -> f32 {
        metric * font_size / self.metric(font_id, |m| m.units_per_em as f32)
    }

    pub fn native_font(&self, font_id: FontId, size: f32) -> NativeFont {
        let native_key = (font_id, OrderedFloat(size));

        let state = self.0.upgradable_read();
        if let Some(native_font) = state.native_fonts.get(&native_key).cloned() {
            native_font
        } else {
            let native_font = state.fonts[font_id.0]
                .native_font()
                .clone_with_font_size(size as f64);
            RwLockUpgradableReadGuard::upgrade(state)
                .native_fonts
                .insert(native_key, native_font.clone());
            native_font
        }
    }

    pub fn font_id_for_native_font(&self, native_font: NativeFont) -> FontId {
        let postscript_name = native_font.postscript_name();
        let state = self.0.upgradable_read();
        if let Some(font_id) = state.fonts_by_name.get(&postscript_name) {
            *font_id
        } else {
            push_font(&mut RwLockUpgradableReadGuard::upgrade(state), unsafe {
                Font::from_native_font(native_font.clone())
            })
        }
    }
}

fn push_font(state: &mut FontCacheState, font: Font) -> FontId {
    let font_id = FontId(state.fonts.len());
    let name = Arc::new(font.postscript_name().unwrap());
    if *name == "AppleColorEmoji" {
        state.emoji_font_id = Some(font_id);
    }
    state.fonts.push(Arc::new(font));
    state.font_names.push(name.clone());
    state.fonts_by_name.insert(name, font_id);
    font_id
}

// #[cfg(test)]
// mod tests {
//     use std::{fs::File, io::BufWriter, path::Path};

//     use super::*;

//     #[test]
//     fn test_render_glyph() {
//         let cache = FontCache::new();
//         let family_id = cache.load_family(&["Fira Code"]).unwrap();
//         let font_id = cache.select_font(family_id, &Default::default()).unwrap();
//         let glyph_id = cache.font(font_id).glyph_for_char('G').unwrap();
//         let (bounds, bytes) = cache.render_glyph(font_id, 16.0, glyph_id, 1.).unwrap();

//         let path = Path::new(r"/Users/as-cii/Desktop/image.png");
//         let file = File::create(path).unwrap();
//         let ref mut w = BufWriter::new(file);

//         let mut encoder = png::Encoder::new(w, bounds.width() as u32, bounds.height() as u32);
//         encoder.set_color(png::ColorType::Grayscale);
//         encoder.set_depth(png::BitDepth::Eight);
//         let mut writer = encoder.write_header().unwrap();
//         writer.write_image_data(&bytes).unwrap();
//     }
// }
