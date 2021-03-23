use std::{collections::HashMap, sync::Arc};

use crate::{
    fonts::{FontId, GlyphId},
    geometry::{
        rect::RectI,
        vector::{vec2i, Vector2I},
    },
    FontCache,
};
use etagere::BucketedAtlasAllocator;
use metal::{MTLPixelFormat, TextureDescriptor};
use ordered_float::OrderedFloat;

#[derive(Hash, Eq, PartialEq)]
struct GlyphDescriptor {
    font_id: FontId,
    font_size: OrderedFloat<f32>,
    glyph_id: GlyphId,
}

pub struct SpriteCache {
    device: metal::Device,
    atlas_size: Vector2I,
    font_cache: Arc<FontCache>,
    atlasses: Vec<Atlas>,
    glyphs: HashMap<GlyphDescriptor, Option<(usize, RectI)>>,
}

impl SpriteCache {
    pub fn new(device: metal::Device, size: Vector2I, font_cache: Arc<FontCache>) -> Self {
        let atlasses = vec![Atlas::new(&device, size)];
        Self {
            device,
            atlas_size: size,
            font_cache,
            atlasses,
            glyphs: Default::default(),
        }
    }

    pub fn render_glyph(
        &mut self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        scale_factor: f32,
    ) -> Option<(usize, RectI)> {
        let font_cache = &self.font_cache;
        let atlasses = &mut self.atlasses;
        let atlas_size = self.atlas_size;
        let device = &self.device;
        self.glyphs
            .entry(GlyphDescriptor {
                font_id,
                font_size: OrderedFloat(font_size),
                glyph_id,
            })
            .or_insert_with(|| {
                let (size, mask) =
                    font_cache.render_glyph(font_id, font_size, glyph_id, scale_factor)?;
                assert!(size.x() < atlas_size.x());
                assert!(size.y() < atlas_size.y());

                let atlas = atlasses.last_mut().unwrap();
                if let Some(bounds) = atlas.try_insert(size, &mask) {
                    Some((atlasses.len() - 1, bounds))
                } else {
                    let mut atlas = Atlas::new(device, atlas_size);
                    let bounds = atlas.try_insert(size, &mask).unwrap();
                    atlasses.push(atlas);
                    Some((atlasses.len() - 1, bounds))
                }
            })
            .clone()
    }

    pub fn atlas_texture(&self, atlas_id: usize) -> Option<&metal::TextureRef> {
        self.atlasses.get(atlas_id).map(|a| a.texture.as_ref())
    }
}

struct Atlas {
    allocator: BucketedAtlasAllocator,
    texture: metal::Texture,
}

impl Atlas {
    fn new(device: &metal::DeviceRef, size: Vector2I) -> Self {
        let descriptor = TextureDescriptor::new();
        descriptor.set_pixel_format(MTLPixelFormat::A8Unorm);
        descriptor.set_width(size.x() as u64);
        descriptor.set_height(size.y() as u64);

        Self {
            allocator: BucketedAtlasAllocator::new(etagere::Size::new(size.x(), size.y())),
            texture: device.new_texture(&descriptor),
        }
    }

    fn try_insert(&mut self, size: Vector2I, mask: &[u8]) -> Option<RectI> {
        let allocation = self
            .allocator
            .allocate(etagere::size2(size.x(), size.y()))?;

        let bounds = allocation.rectangle;
        let region = metal::MTLRegion::new_2d(
            bounds.min.x as u64,
            bounds.min.y as u64,
            bounds.width() as u64,
            bounds.height() as u64,
        );
        self.texture
            .replace_region(region, 0, mask.as_ptr() as *const _, size.x() as u64);
        Some(RectI::from_points(
            vec2i(bounds.min.x, bounds.min.y),
            vec2i(bounds.max.x, bounds.max.y),
        ))
    }
}
