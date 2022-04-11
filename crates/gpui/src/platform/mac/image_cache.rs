use anyhow::anyhow;
use metal::{MTLPixelFormat, TextureDescriptor, TextureRef};

use super::atlas::{AllocId, AtlasAllocator};
use crate::{
    geometry::{rect::RectI, vector::Vector2I},
    ImageData,
};
use std::{collections::HashMap, mem};

pub struct ImageCache {
    prev_frame: HashMap<usize, (AllocId, RectI)>,
    curr_frame: HashMap<usize, (AllocId, RectI)>,
    atlases: AtlasAllocator,
}

impl ImageCache {
    pub fn new(device: metal::Device, size: Vector2I) -> Self {
        let descriptor = TextureDescriptor::new();
        descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        descriptor.set_width(size.x() as u64);
        descriptor.set_height(size.y() as u64);
        Self {
            prev_frame: Default::default(),
            curr_frame: Default::default(),
            atlases: AtlasAllocator::new(device, descriptor),
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
