use std::sync::Arc;
use std::time::Instant;

use gpui::{AppContext, Global, ReadGlobal, SharedString};
use parking_lot::RwLock;

#[derive(Default)]
struct FontFamilyCacheState {
    loaded_at: Option<Instant>,
    font_families: Vec<SharedString>,
}

/// A cache for the list of font families.
///
/// Listing the available font families from the text system is expensive,
/// so we do it once and then use the cached values each render.
#[derive(Default)]
pub struct FontFamilyCache {
    state: RwLock<FontFamilyCacheState>,
}

#[derive(Default)]
struct GlobalFontFamilyCache(Arc<FontFamilyCache>);

impl Global for GlobalFontFamilyCache {}

impl FontFamilyCache {
    pub fn init_global(cx: &mut AppContext) {
        cx.default_global::<GlobalFontFamilyCache>();
    }

    pub fn global(cx: &AppContext) -> Arc<Self> {
        GlobalFontFamilyCache::global(cx).0.clone()
    }

    pub fn list_font_families(&self, cx: &AppContext) -> Vec<SharedString> {
        if self.state.read().loaded_at.is_some() {
            return self.state.read().font_families.clone();
        }

        let mut lock = self.state.write();
        lock.font_families = cx
            .text_system()
            .all_font_names()
            .into_iter()
            .map(SharedString::from)
            .collect();
        lock.loaded_at = Some(Instant::now());

        lock.font_families.clone()
    }
}
