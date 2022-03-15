use crate::{
    fonts::{FontId, Metrics, Properties},
    geometry::vector::{vec2f, Vector2F},
    platform,
    text_layout::LineWrapper,
};
use anyhow::{anyhow, Result};
use ordered_float::OrderedFloat;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct FamilyId(usize);

struct Family {
    name: Arc<str>,
    font_ids: Vec<FontId>,
}

pub struct FontCache(RwLock<FontCacheState>);

pub struct FontCacheState {
    fonts: Arc<dyn platform::FontSystem>,
    families: Vec<Family>,
    font_selections: HashMap<FamilyId, HashMap<Properties, FontId>>,
    metrics: HashMap<FontId, Metrics>,
    wrapper_pool: HashMap<(FontId, OrderedFloat<f32>), Vec<LineWrapper>>,
}

pub struct LineWrapperHandle {
    wrapper: Option<LineWrapper>,
    font_cache: Arc<FontCache>,
}

unsafe impl Send for FontCache {}

impl FontCache {
    pub fn new(fonts: Arc<dyn platform::FontSystem>) -> Self {
        Self(RwLock::new(FontCacheState {
            fonts,
            families: Default::default(),
            font_selections: Default::default(),
            metrics: Default::default(),
            wrapper_pool: Default::default(),
        }))
    }

    pub fn family_name(&self, family_id: FamilyId) -> Result<Arc<str>> {
        self.0
            .read()
            .families
            .get(family_id.0)
            .ok_or_else(|| anyhow!("invalid family id"))
            .map(|family| family.name.clone())
    }

    pub fn load_family(&self, names: &[&str]) -> Result<FamilyId> {
        for name in names {
            let state = self.0.upgradable_read();

            if let Some(ix) = state.families.iter().position(|f| f.name.as_ref() == *name) {
                return Ok(FamilyId(ix));
            }

            let mut state = RwLockUpgradableReadGuard::upgrade(state);

            if let Ok(font_ids) = state.fonts.load_family(name) {
                if font_ids.is_empty() {
                    continue;
                }

                let family_id = FamilyId(state.families.len());
                for font_id in &font_ids {
                    if state.fonts.glyph_for_char(*font_id, 'm').is_none() {
                        return Err(anyhow!("font must contain a glyph for the 'm' character"));
                    }
                }

                state.families.push(Family {
                    name: Arc::from(*name),
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
            let font_id = inner
                .fonts
                .select_font(&family.font_ids, properties)
                .unwrap_or(family.font_ids[0]);

            inner
                .font_selections
                .entry(family_id)
                .or_default()
                .insert(properties.clone(), font_id);
            Ok(font_id)
        }
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
            let metrics = state.fonts.font_metrics(font_id);
            let metric = f(&metrics);
            let mut state = RwLockUpgradableReadGuard::upgrade(state);
            state.metrics.insert(font_id, metrics);
            metric
        }
    }

    pub fn bounding_box(&self, font_id: FontId, font_size: f32) -> Vector2F {
        let bounding_box = self.metric(font_id, |m| m.bounding_box);
        let width = bounding_box.width() * self.em_scale(font_id, font_size);
        let height = bounding_box.height() * self.em_scale(font_id, font_size);
        vec2f(width, height)
    }

    pub fn em_width(&self, font_id: FontId, font_size: f32) -> f32 {
        let glyph_id;
        let bounds;
        {
            let state = self.0.read();
            glyph_id = state.fonts.glyph_for_char(font_id, 'm').unwrap();
            bounds = state.fonts.typographic_bounds(font_id, glyph_id).unwrap();
        }
        bounds.width() * self.em_scale(font_id, font_size)
    }

    pub fn em_advance(&self, font_id: FontId, font_size: f32) -> f32 {
        let glyph_id;
        let advance;
        {
            let state = self.0.read();
            glyph_id = state.fonts.glyph_for_char(font_id, 'm').unwrap();
            advance = state.fonts.advance(font_id, glyph_id).unwrap();
        }
        advance.x() * self.em_scale(font_id, font_size)
    }

    pub fn line_height(&self, font_size: f32) -> f32 {
        (font_size * 1.618).round()
    }

    pub fn cap_height(&self, font_id: FontId, font_size: f32) -> f32 {
        self.metric(font_id, |m| m.cap_height) * self.em_scale(font_id, font_size)
    }

    pub fn x_height(&self, font_id: FontId, font_size: f32) -> f32 {
        self.metric(font_id, |m| m.x_height) * self.em_scale(font_id, font_size)
    }

    pub fn ascent(&self, font_id: FontId, font_size: f32) -> f32 {
        self.metric(font_id, |m| m.ascent) * self.em_scale(font_id, font_size)
    }

    pub fn descent(&self, font_id: FontId, font_size: f32) -> f32 {
        self.metric(font_id, |m| -m.descent) * self.em_scale(font_id, font_size)
    }

    pub fn em_scale(&self, font_id: FontId, font_size: f32) -> f32 {
        font_size / self.metric(font_id, |m| m.units_per_em as f32)
    }

    pub fn baseline_offset(&self, font_id: FontId, font_size: f32) -> f32 {
        let line_height = self.line_height(font_size);
        let ascent = self.ascent(font_id, font_size);
        let descent = self.descent(font_id, font_size);
        let padding_top = (line_height - ascent - descent) / 2.;
        padding_top + ascent
    }

    pub fn line_wrapper(self: &Arc<Self>, font_id: FontId, font_size: f32) -> LineWrapperHandle {
        let mut state = self.0.write();
        let wrappers = state
            .wrapper_pool
            .entry((font_id, OrderedFloat(font_size)))
            .or_default();
        let wrapper = wrappers
            .pop()
            .unwrap_or_else(|| LineWrapper::new(font_id, font_size, state.fonts.clone()));
        LineWrapperHandle {
            wrapper: Some(wrapper),
            font_cache: self.clone(),
        }
    }
}

impl Drop for LineWrapperHandle {
    fn drop(&mut self) {
        let mut state = self.font_cache.0.write();
        let wrapper = self.wrapper.take().unwrap();
        state
            .wrapper_pool
            .get_mut(&(wrapper.font_id, OrderedFloat(wrapper.font_size)))
            .unwrap()
            .push(wrapper);
    }
}

impl Deref for LineWrapperHandle {
    type Target = LineWrapper;

    fn deref(&self) -> &Self::Target {
        self.wrapper.as_ref().unwrap()
    }
}

impl DerefMut for LineWrapperHandle {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.wrapper.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        fonts::{Style, Weight},
        platform::{test, Platform as _},
    };

    #[test]
    fn test_select_font() {
        let platform = test::platform();
        let fonts = FontCache::new(platform.fonts());
        let arial = fonts.load_family(&["Arial"]).unwrap();
        let arial_regular = fonts.select_font(arial, &Properties::new()).unwrap();
        let arial_italic = fonts
            .select_font(arial, &Properties::new().style(Style::Italic))
            .unwrap();
        let arial_bold = fonts
            .select_font(arial, &Properties::new().weight(Weight::BOLD))
            .unwrap();
        assert_ne!(arial_regular, arial_italic);
        assert_ne!(arial_regular, arial_bold);
        assert_ne!(arial_italic, arial_bold);
    }
}
