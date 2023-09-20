use crate::{
    px, Bounds, FontFeatures, FontStyle, FontWeight, Pixels, PlatformTextSystem, Result, Size,
};
use anyhow::anyhow;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use std::{collections::HashMap, sync::Arc};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontFamilyId(usize);

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FontId(pub usize);

pub(crate) struct FontCache(RwLock<FontCacheState>);

pub(crate) struct FontCacheState {
    platform_text_system: Arc<dyn PlatformTextSystem>,
    families: Vec<Family>,
    default_family: Option<FontFamilyId>,
    font_selections: HashMap<FontFamilyId, HashMap<(FontWeight, FontStyle), FontId>>,
    metrics: HashMap<FontId, FontMetrics>,
}

unsafe impl Send for FontCache {}

impl FontCache {
    pub fn new(fonts: Arc<dyn PlatformTextSystem>) -> Self {
        Self(RwLock::new(FontCacheState {
            platform_text_system: fonts,
            families: Default::default(),
            default_family: None,
            font_selections: Default::default(),
            metrics: Default::default(),
        }))
    }

    pub fn family_name(&self, family_id: FontFamilyId) -> Result<Arc<str>> {
        self.0
            .read()
            .families
            .get(family_id.0)
            .ok_or_else(|| anyhow!("invalid family id"))
            .map(|family| family.name.clone())
    }

    pub fn load_family(&self, names: &[&str], features: &FontFeatures) -> Result<FontFamilyId> {
        for name in names {
            let state = self.0.upgradable_read();

            if let Some(ix) = state
                .families
                .iter()
                .position(|f| f.name.as_ref() == *name && f.font_features == *features)
            {
                return Ok(FontFamilyId(ix));
            }

            let mut state = RwLockUpgradableReadGuard::upgrade(state);

            if let Ok(font_ids) = state.platform_text_system.load_family(name, features) {
                if font_ids.is_empty() {
                    continue;
                }

                let family_id = FontFamilyId(state.families.len());
                for font_id in &font_ids {
                    if state
                        .platform_text_system
                        .glyph_for_char(*font_id, 'm')
                        .is_none()
                    {
                        return Err(anyhow!("font must contain a glyph for the 'm' character"));
                    }
                }

                state.families.push(Family {
                    name: Arc::from(*name),
                    font_features: features.clone(),
                    font_ids,
                });
                return Ok(family_id);
            }
        }

        Err(anyhow!(
            "could not find a non-empty font family matching one of the given names"
        ))
    }

    /// Returns an arbitrary font family that is available on the system.
    pub fn known_existing_family(&self) -> FontFamilyId {
        if let Some(family_id) = self.0.read().default_family {
            return family_id;
        }

        let default_family = self
            .load_family(
                &["Courier", "Helvetica", "Arial", "Verdana"],
                &Default::default(),
            )
            .unwrap_or_else(|_| {
                let all_family_names = self.0.read().platform_text_system.all_families();
                let all_family_names: Vec<_> = all_family_names
                    .iter()
                    .map(|string| string.as_str())
                    .collect();
                self.load_family(&all_family_names, &Default::default())
                    .expect("could not load any default font family")
            });

        self.0.write().default_family = Some(default_family);
        default_family
    }

    pub fn default_font(&self, family_id: FontFamilyId) -> FontId {
        self.select_font(family_id, Default::default(), Default::default())
            .unwrap()
    }

    pub fn select_font(
        &self,
        family_id: FontFamilyId,
        weight: FontWeight,
        style: FontStyle,
    ) -> Result<FontId> {
        let inner = self.0.upgradable_read();
        if let Some(font_id) = inner
            .font_selections
            .get(&family_id)
            .and_then(|fonts| fonts.get(&(weight, style)))
        {
            Ok(*font_id)
        } else {
            let mut inner = RwLockUpgradableReadGuard::upgrade(inner);
            let family = &inner.families[family_id.0];
            let font_id = inner
                .platform_text_system
                .select_font(&family.font_ids, weight, style)
                .unwrap_or(family.font_ids[0]);
            inner
                .font_selections
                .entry(family_id)
                .or_default()
                .insert((weight, style), font_id);
            Ok(font_id)
        }
    }

    pub fn read_metric<F, T>(&self, font_id: FontId, f: F) -> T
    where
        F: FnOnce(&FontMetrics) -> T,
        T: 'static,
    {
        let state = self.0.upgradable_read();
        if let Some(metrics) = state.metrics.get(&font_id) {
            f(metrics)
        } else {
            let metrics = state.platform_text_system.font_metrics(font_id);
            let metric = f(&metrics);
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            state.metrics.insert(font_id, metrics);
            metric
        }
    }

    pub fn bounding_box(&self, font_id: FontId, font_size: Pixels) -> Size<Pixels> {
        let bounding_box = self.read_metric(font_id, |m| m.bounding_box);

        let width = px(bounding_box.size.width) * self.em_size(font_id, font_size);
        let height = px(bounding_box.size.height) * self.em_size(font_id, font_size);
        Size { width, height }
    }

    pub fn em_width(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        let glyph_id;
        let bounds;
        {
            let state = self.0.read();
            glyph_id = state
                .platform_text_system
                .glyph_for_char(font_id, 'm')
                .unwrap();
            bounds = state
                .platform_text_system
                .typographic_bounds(font_id, glyph_id)
                .unwrap();
        }
        self.em_size(font_id, font_size) * bounds.size.width
    }

    pub fn em_advance(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        let glyph_id;
        let advance;
        {
            let state = self.0.read();
            glyph_id = state
                .platform_text_system
                .glyph_for_char(font_id, 'm')
                .unwrap();
            advance = state
                .platform_text_system
                .advance(font_id, glyph_id)
                .unwrap();
        }
        self.em_size(font_id, font_size) * advance.width
    }

    pub fn line_height(&self, font_size: Pixels) -> Pixels {
        (font_size * 1.618).round()
    }

    pub fn cap_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.em_size(font_id, font_size) * self.read_metric(font_id, |m| m.cap_height)
    }

    pub fn x_height(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.em_size(font_id, font_size) * self.read_metric(font_id, |m| m.x_height)
    }

    pub fn ascent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.em_size(font_id, font_size) * self.read_metric(font_id, |m| m.ascent)
    }

    pub fn descent(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        self.em_size(font_id, font_size) * self.read_metric(font_id, |m| -m.descent)
    }

    pub fn em_size(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        font_size / self.read_metric(font_id, |m| m.units_per_em as f32)
    }

    pub fn baseline_offset(&self, font_id: FontId, font_size: Pixels) -> Pixels {
        let line_height = self.line_height(font_size);
        let ascent = self.ascent(font_id, font_size);
        let descent = self.descent(font_id, font_size);
        let padding_top = (line_height - ascent - descent) / 2.;
        padding_top + ascent
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FontMetrics {
    pub units_per_em: u32,
    pub ascent: f32,
    pub descent: f32,
    pub line_gap: f32,
    pub underline_position: f32,
    pub underline_thickness: f32,
    pub cap_height: f32,
    pub x_height: f32,
    pub bounding_box: Bounds<f32>,
}

struct Family {
    name: Arc<str>,
    font_features: FontFeatures,
    font_ids: Vec<FontId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FontStyle, FontWeight, Platform, TestPlatform};

    #[test]
    fn test_select_font() {
        let platform = TestPlatform::new();
        let fonts = FontCache::new(platform.text_system());
        let arial = fonts
            .load_family(
                &["Arial"],
                &FontFeatures {
                    calt: Some(false),
                    ..Default::default()
                },
            )
            .unwrap();
        let arial_regular = fonts
            .select_font(arial, FontWeight::default(), FontStyle::default())
            .unwrap();
        let arial_italic = fonts
            .select_font(arial, FontWeight::default(), FontStyle::Italic)
            .unwrap();
        let arial_bold = fonts
            .select_font(arial, FontWeight::BOLD, FontStyle::default())
            .unwrap();
        assert_ne!(arial_regular, arial_italic);
        assert_ne!(arial_regular, arial_bold);
        assert_ne!(arial_italic, arial_bold);

        let arial_with_calt = fonts
            .load_family(
                &["Arial"],
                &FontFeatures {
                    calt: Some(true),
                    ..Default::default()
                },
            )
            .unwrap();
        assert_ne!(arial_with_calt, arial);
    }
}
