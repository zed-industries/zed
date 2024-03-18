use std::borrow::Cow;

use anyhow::{anyhow, Result};
use collections::HashMap;
use itertools::Itertools;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use smallvec::SmallVec;
use windows::{
    core::{HSTRING, PCWSTR},
    Win32::{
        Foundation::BOOL,
        Globalization::GetUserDefaultLocaleName,
        Graphics::DirectWrite::{
            DWriteCreateFactory, IDWriteFactory5, IDWriteFontCollection, IDWriteFontFaceReference,
            IDWriteFontFamily, IDWriteFontSet, IDWriteFontSetBuilder1,
            IDWriteInMemoryFontFileLoader, DWRITE_FACTORY_TYPE_SHARED,
            DWRITE_FONT_PROPERTY_ID_FAMILY_NAME, DWRITE_FONT_PROPERTY_ID_FULL_NAME,
            DWRITE_FONT_STRETCH_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_WEIGHT,
        },
    },
};

use crate::{
    Bounds, DevicePixels, Font, FontFeatures, FontId, FontMetrics, FontRun, GlyphId, LineLayout,
    Pixels, PlatformTextSystem, RenderGlyphParams, SharedString, Size,
};

#[derive(Clone, PartialEq, Eq, Hash)]
struct FontKey {
    font_family: SharedString,
    font_features: FontFeatures,
}

pub(crate) struct DirectWriteTextSystem(RwLock<DirectWriteState>);

struct DirectWriteComponent {
    locale: String,
    factory: IDWriteFactory5,
    in_memory_loader: IDWriteInMemoryFontFileLoader,
    builder: IDWriteFontSetBuilder1,
}

struct DirectWriteState {
    components: DirectWriteComponent,
    system_set: IDWriteFontSet,
    font_sets: Vec<IDWriteFontSet>,
    fonts: Vec<IDWriteFontFaceReference>,
    font_selections: HashMap<Font, FontId>,
    postscript_names_by_font_id: HashMap<FontId, String>,
}

impl DirectWriteComponent {
    pub fn new() -> Self {
        unsafe {
            let factory: IDWriteFactory5 = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED).unwrap();
            let in_memory_loader = factory.CreateInMemoryFontFileLoader().unwrap();
            factory.RegisterFontFileLoader(&in_memory_loader).unwrap();
            let builder = factory.CreateFontSetBuilder2().unwrap();
            let mut locale_vec = vec![0u16; 512];
            GetUserDefaultLocaleName(&mut locale_vec);
            let locale = String::from_utf16_lossy(&locale_vec);

            DirectWriteComponent {
                locale,
                factory,
                in_memory_loader,
                builder,
            }
        }
    }
}

impl DirectWriteTextSystem {
    pub(crate) fn new() -> Self {
        let components = DirectWriteComponent::new();
        let system_set = unsafe { components.factory.GetSystemFontSet().unwrap() };
        Self(RwLock::new(DirectWriteState {
            components: DirectWriteComponent::new(),
            system_set,
            font_sets: Vec::new(),
            fonts: Vec::new(),
            font_selections: HashMap::default(),
            postscript_names_by_font_id: HashMap::default(),
        }))
    }
}

impl Default for DirectWriteTextSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformTextSystem for DirectWriteTextSystem {
    fn add_fonts(&self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        self.0.write().add_fonts(fonts)
    }

    fn all_font_names(&self) -> Vec<String> {
        todo!()
    }

    fn all_font_families(&self) -> Vec<String> {
        todo!()
    }

    fn font_id(&self, font: &Font) -> Result<FontId> {
        let lock = self.0.upgradable_read();
        if let Some(font_id) = lock.font_selections.get(font) {
            Ok(*font_id)
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let font_id = lock
                .select_font_using_memory(font)
                .or_else(|| lock.select_font_using_system(font))
                .unwrap();
            lock.font_selections.insert(font.clone(), font_id);
            println!("Get font id: {:#?}", font_id);
            Ok(font_id)
        }
    }

    fn font_metrics(&self, font_id: FontId) -> FontMetrics {
        todo!()
    }

    fn typographic_bounds(&self, font_id: FontId, glyph_id: GlyphId) -> Result<Bounds<f32>> {
        todo!()
    }

    fn advance(&self, font_id: FontId, glyph_id: GlyphId) -> anyhow::Result<Size<f32>> {
        todo!()
    }

    fn glyph_for_char(&self, font_id: FontId, ch: char) -> Option<GlyphId> {
        todo!()
    }

    fn glyph_raster_bounds(
        &self,
        params: &RenderGlyphParams,
    ) -> anyhow::Result<Bounds<DevicePixels>> {
        todo!()
    }

    fn rasterize_glyph(
        &self,
        params: &RenderGlyphParams,
        raster_bounds: Bounds<DevicePixels>,
    ) -> anyhow::Result<(Size<DevicePixels>, Vec<u8>)> {
        todo!()
    }

    fn layout_line(&self, text: &str, font_size: Pixels, runs: &[FontRun]) -> LineLayout {
        todo!()
    }

    fn wrap_line(
        &self,
        text: &str,
        font_id: FontId,
        font_size: Pixels,
        width: Pixels,
    ) -> Vec<usize> {
        todo!()
    }
}

impl DirectWriteState {
    fn add_fonts(&mut self, fonts: Vec<Cow<'static, [u8]>>) -> Result<()> {
        for font_data in fonts {
            match font_data {
                Cow::Borrowed(data) => unsafe {
                    let font_file = self
                        .components
                        .in_memory_loader
                        .CreateInMemoryFontFileReference(
                            &self.components.factory,
                            data.as_ptr() as _,
                            data.len() as _,
                            &self.components.factory,
                        )?;
                    self.components.builder.AddFontFile(&font_file)?;
                },
                Cow::Owned(data) => unsafe {
                    let font_file = self
                        .components
                        .in_memory_loader
                        .CreateInMemoryFontFileReference(
                            &self.components.factory,
                            data.as_ptr() as _,
                            data.len() as _,
                            &self.components.factory,
                        )?;
                    self.components.builder.AddFontFile(&font_file)?;
                },
            }
        }
        let set = unsafe { self.components.builder.CreateFontSet()? };
        self.font_sets.push(set);

        Ok(())
    }

    fn select_font_using_memory(&mut self, font: &Font) -> Option<FontId> {
        unsafe {
            for fontset in self.font_sets.iter() {
                let font = fontset
                    .GetMatchingFonts(
                        &HSTRING::from(font.family.to_string()),
                        DWRITE_FONT_WEIGHT(font.weight.0 as _),
                        DWRITE_FONT_STRETCH_NORMAL,
                        DWRITE_FONT_STYLE_NORMAL,
                    )
                    .unwrap();
                if font.GetFontCount() != 0 {
                    let font_id = FontId(self.fonts.len());
                    let font_face = font.GetFontFaceReference(0).unwrap();
                    self.fonts.push(font_face);
                    return Some(font_id);
                }
            }
            None
        }
    }

    fn select_font_using_system(&mut self, font: &Font) -> Option<FontId> {
        unsafe {
            let fontset = self
                .system_set
                .GetMatchingFonts(
                    &HSTRING::from(font.family.to_string()),
                    DWRITE_FONT_WEIGHT(font.weight.0 as _),
                    DWRITE_FONT_STRETCH_NORMAL,
                    DWRITE_FONT_STYLE_NORMAL,
                )
                .unwrap();
            if fontset.GetFontCount() == 0 {
                return None;
            }
            let font_id = FontId(self.fonts.len());
            let font_face = fontset.GetFontFaceReference(0).unwrap();
            self.fonts.push(font_face);
            Some(font_id)
        }
    }
}

impl Drop for DirectWriteState {
    fn drop(&mut self) {
        unsafe {
            let _ = self
                .components
                .factory
                .UnregisterFontFileLoader(&self.components.in_memory_loader);
        }
    }
}
