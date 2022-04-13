use super::atlas::{AllocId, AtlasAllocator};
use crate::{
    fonts::{FontId, GlyphId},
    geometry::{rect::RectI, vector::Vector2I},
    platform::RasterizationOptions,
    scene::ImageGlyph,
    FontSystem, ImageData,
};
use anyhow::anyhow;
use metal::{MTLPixelFormat, TextureDescriptor, TextureRef};
use ordered_float::OrderedFloat;
use std::{collections::HashMap, mem, sync::Arc};

#[derive(Hash, Eq, PartialEq)]
struct GlyphDescriptor {
    font_id: FontId,
    font_size: OrderedFloat<f32>,
    glyph_id: GlyphId,
}

pub struct ImageCache {
    prev_frame: HashMap<usize, (AllocId, RectI)>,
    curr_frame: HashMap<usize, (AllocId, RectI)>,
    image_glyphs: HashMap<GlyphDescriptor, Option<(AllocId, RectI, Vector2I)>>,
    atlases: AtlasAllocator,
    scale_factor: f32,
    fonts: Arc<dyn FontSystem>,
}

impl ImageCache {
    pub fn new(
        device: metal::Device,
        size: Vector2I,
        scale_factor: f32,
        fonts: Arc<dyn FontSystem>,
    ) -> Self {
        let descriptor = TextureDescriptor::new();
        descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        descriptor.set_width(size.x() as u64);
        descriptor.set_height(size.y() as u64);
        Self {
            prev_frame: Default::default(),
            curr_frame: Default::default(),
            image_glyphs: Default::default(),
            atlases: AtlasAllocator::new(device, descriptor),
            scale_factor,
            fonts,
        }
    }

    pub fn set_scale_factor(&mut self, scale_factor: f32) {
        if scale_factor != self.scale_factor {
            self.scale_factor = scale_factor;
            for (_, glyph) in self.image_glyphs.drain() {
                if let Some((alloc_id, _, _)) = glyph {
                    self.atlases.deallocate(alloc_id);
                }
            }
        }
    }

    pub fn render(&mut self, image: &ImageData) -> (AllocId, RectI) {
        let (alloc_id, atlas_bounds) = self
            .prev_frame
            .remove(&image.id)
            .or_else(|| self.curr_frame.get(&image.id).copied())
            .or_else(|| self.atlases.upload(image.size(), image.as_bytes()))
            .ok_or_else(|| anyhow!("could not upload image of size {:?}", image.size()))
            .unwrap();
        self.curr_frame.insert(image.id, (alloc_id, atlas_bounds));
        (alloc_id, atlas_bounds)
    }

    pub fn render_glyph(&mut self, image_glyph: &ImageGlyph) -> Option<(AllocId, RectI, Vector2I)> {
        *self
            .image_glyphs
            .entry(GlyphDescriptor {
                font_id: image_glyph.font_id,
                font_size: OrderedFloat(image_glyph.font_size),
                glyph_id: image_glyph.id,
            })
            .or_insert_with(|| {
                let (glyph_bounds, bytes) = self.fonts.rasterize_glyph(
                    image_glyph.font_id,
                    image_glyph.font_size,
                    image_glyph.id,
                    Default::default(),
                    self.scale_factor,
                    RasterizationOptions::Bgra,
                )?;
                let (alloc_id, atlas_bounds) = self
                    .atlases
                    .upload(glyph_bounds.size(), &bytes)
                    .ok_or_else(|| {
                        anyhow!(
                            "could not upload image glyph of size {:?}",
                            glyph_bounds.size()
                        )
                    })
                    .unwrap();
                Some((alloc_id, atlas_bounds, glyph_bounds.origin()))
            })
    }

    pub fn finish_frame(&mut self) {
        mem::swap(&mut self.prev_frame, &mut self.curr_frame);
        for (_, (id, _)) in self.curr_frame.drain() {
            self.atlases.deallocate(id);
        }
    }

    pub fn atlas_texture(&self, atlas_id: usize) -> Option<&TextureRef> {
        self.atlases.texture(atlas_id)
    }
}
