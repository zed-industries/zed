use std::{collections::HashMap, sync::Arc};

use crate::{
    fonts::{FontId, GlyphId},
    geometry::{rect::RectI, vector::Vector2I},
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
    font_cache: Arc<FontCache>,
    device: metal::Device,
    size: Vector2I,
    atlasses: Vec<Atlas>,
    glyphs: HashMap<GlyphDescriptor, (usize, RectI)>,
}

impl SpriteCache {
    pub fn new(device: metal::Device, size: Vector2I) -> Self {
        Self {
            device,
            size,
            atlasses: vec![Atlas::new(&device, size)],
            glyphs: Default::default(),
        }
    }

    pub fn render_glyph(
        &mut self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
    ) -> (usize, RectI) {
        self.glyphs
            .entry(GlyphDescriptor {
                font_id,
                font_size: OrderedFloat(font_size),
                glyph_id,
            })
            .or_insert_with(|| {
                let rendered_glyph = self.font_cache.render_glyph(font_id, font_size, glyph_id);
                // let atlas = self.atlasses.last_mut().unwrap();
                todo!()
            })
            .clone()
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
}
