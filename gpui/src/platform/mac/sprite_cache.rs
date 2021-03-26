use crate::{
    fonts::{FontId, GlyphId},
    geometry::{
        rect::RectI,
        vector::{vec2f, vec2i, Vector2F, Vector2I},
    },
    platform,
};
use etagere::BucketedAtlasAllocator;
use metal::{MTLPixelFormat, TextureDescriptor};
use ordered_float::OrderedFloat;
use std::{collections::HashMap, sync::Arc};

#[derive(Hash, Eq, PartialEq)]
struct GlyphDescriptor {
    font_id: FontId,
    font_size: OrderedFloat<f32>,
    glyph_id: GlyphId,
    subpixel_variant: (u8, u8),
}

#[derive(Clone)]
pub struct GlyphSprite {
    pub atlas_id: usize,
    pub atlas_origin: Vector2I,
    pub offset: Vector2I,
    pub size: Vector2I,
}

pub struct SpriteCache {
    device: metal::Device,
    atlas_size: Vector2I,
    fonts: Arc<dyn platform::FontSystem>,
    atlasses: Vec<Atlas>,
    glyphs: HashMap<GlyphDescriptor, Option<GlyphSprite>>,
}

impl SpriteCache {
    pub fn new(
        device: metal::Device,
        size: Vector2I,
        fonts: Arc<dyn platform::FontSystem>,
    ) -> Self {
        let atlasses = vec![Atlas::new(&device, size)];
        Self {
            device,
            atlas_size: size,
            fonts,
            atlasses,
            glyphs: Default::default(),
        }
    }

    pub fn atlas_size(&self) -> Vector2I {
        self.atlas_size
    }

    pub fn render_glyph(
        &mut self,
        font_id: FontId,
        font_size: f32,
        glyph_id: GlyphId,
        target_position: Vector2F,
        scale_factor: f32,
    ) -> Option<GlyphSprite> {
        const SUBPIXEL_VARIANTS: u8 = 4;

        let target_position = target_position * scale_factor;
        let fonts = &self.fonts;
        let atlasses = &mut self.atlasses;
        let atlas_size = self.atlas_size;
        let device = &self.device;
        let subpixel_variant = (
            (target_position.x().fract() * SUBPIXEL_VARIANTS as f32).round() as u8
                % SUBPIXEL_VARIANTS,
            (target_position.y().fract() * SUBPIXEL_VARIANTS as f32).round() as u8
                % SUBPIXEL_VARIANTS,
        );
        self.glyphs
            .entry(GlyphDescriptor {
                font_id,
                font_size: OrderedFloat(font_size),
                glyph_id,
                subpixel_variant,
            })
            .or_insert_with(|| {
                let subpixel_shift = vec2f(
                    subpixel_variant.0 as f32 / SUBPIXEL_VARIANTS as f32,
                    subpixel_variant.1 as f32 / SUBPIXEL_VARIANTS as f32,
                );
                let (glyph_bounds, mask) = fonts.rasterize_glyph(
                    font_id,
                    font_size,
                    glyph_id,
                    subpixel_shift,
                    scale_factor,
                )?;
                assert!(glyph_bounds.width() < atlas_size.x());
                assert!(glyph_bounds.height() < atlas_size.y());

                let atlas_bounds = atlasses
                    .last_mut()
                    .unwrap()
                    .try_insert(glyph_bounds.size(), &mask)
                    .unwrap_or_else(|| {
                        let mut atlas = Atlas::new(device, atlas_size);
                        let bounds = atlas.try_insert(glyph_bounds.size(), &mask).unwrap();
                        atlasses.push(atlas);
                        bounds
                    });

                Some(GlyphSprite {
                    atlas_id: atlasses.len() - 1,
                    atlas_origin: atlas_bounds.origin(),
                    offset: glyph_bounds.origin(),
                    size: glyph_bounds.size(),
                })
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
            .allocate(etagere::size2(size.x() + 1, size.y() + 1))?;

        let bounds = allocation.rectangle;
        let region = metal::MTLRegion::new_2d(
            bounds.min.x as u64,
            bounds.min.y as u64,
            size.x() as u64,
            size.y() as u64,
        );
        self.texture
            .replace_region(region, 0, mask.as_ptr() as *const _, size.x() as u64);
        Some(RectI::from_points(
            vec2i(bounds.min.x, bounds.min.y),
            vec2i(bounds.max.x, bounds.max.y),
        ))
    }
}
