use crate::geometry::{
    rect::RectI,
    vector::{vec2i, Vector2I},
};
use etagere::BucketedAtlasAllocator;
use foreign_types::ForeignType;
use log::warn;
use metal::{Device, TextureDescriptor};
use objc::{msg_send, sel, sel_impl};

pub struct AtlasAllocator {
    device: Device,
    texture_descriptor: TextureDescriptor,
    atlases: Vec<Atlas>,
    free_atlases: Vec<Atlas>,
}

#[derive(Copy, Clone)]
pub struct AllocId {
    pub atlas_id: usize,
    alloc_id: etagere::AllocId,
}

impl AtlasAllocator {
    pub fn new(device: Device, texture_descriptor: TextureDescriptor) -> Self {
        let mut me = Self {
            device,
            texture_descriptor,
            atlases: Vec::new(),
            free_atlases: Vec::new(),
        };
        let atlas = me.new_atlas(Vector2I::zero());
        me.atlases.push(atlas);
        me
    }

    pub fn default_atlas_size(&self) -> Vector2I {
        vec2i(
            self.texture_descriptor.width() as i32,
            self.texture_descriptor.height() as i32,
        )
    }

    pub fn allocate(&mut self, requested_size: Vector2I) -> Option<(AllocId, Vector2I)> {
        let allocation = self
            .atlases
            .last_mut()
            .unwrap()
            .allocate(requested_size)
            .or_else(|| {
                let mut atlas = self.new_atlas(requested_size);
                let (id, origin) = atlas.allocate(requested_size)?;
                self.atlases.push(atlas);
                Some((id, origin))
            });

        if allocation.is_none() {
            warn!(
                "allocation of size {:?} could not be created",
                requested_size,
            );
        }

        let (alloc_id, origin) = allocation?;

        let id = AllocId {
            atlas_id: self.atlases.len() - 1,
            alloc_id,
        };
        Some((id, origin))
    }

    pub fn upload(&mut self, size: Vector2I, bytes: &[u8]) -> Option<(AllocId, RectI)> {
        let (alloc_id, origin) = self.allocate(size)?;
        let bounds = RectI::new(origin, size);
        self.atlases[alloc_id.atlas_id].upload(bounds, bytes);
        Some((alloc_id, bounds))
    }

    pub fn deallocate(&mut self, id: AllocId) {
        if let Some(atlas) = self.atlases.get_mut(id.atlas_id) {
            atlas.deallocate(id.alloc_id);
            if atlas.is_empty() {
                self.free_atlases.push(self.atlases.remove(id.atlas_id));
            }
        }
    }

    pub fn clear(&mut self) {
        for atlas in &mut self.atlases {
            atlas.clear();
        }
        self.free_atlases.extend(self.atlases.drain(1..));
    }

    pub fn texture(&self, atlas_id: usize) -> Option<&metal::TextureRef> {
        self.atlases.get(atlas_id).map(|a| a.texture.as_ref())
    }

    fn new_atlas(&mut self, required_size: Vector2I) -> Atlas {
        if let Some(i) = self.free_atlases.iter().rposition(|atlas| {
            atlas.size().x() >= required_size.x() && atlas.size().y() >= required_size.y()
        }) {
            self.free_atlases.remove(i)
        } else {
            let size = self.default_atlas_size().max(required_size);
            let texture = if size.x() as u64 > self.texture_descriptor.width()
                || size.y() as u64 > self.texture_descriptor.height()
            {
                let descriptor = unsafe {
                    let descriptor_ptr: *mut metal::MTLTextureDescriptor =
                        msg_send![self.texture_descriptor, copy];
                    metal::TextureDescriptor::from_ptr(descriptor_ptr)
                };
                descriptor.set_width(size.x() as u64);
                descriptor.set_height(size.y() as u64);
                self.device.new_texture(&descriptor)
            } else {
                self.device.new_texture(&self.texture_descriptor)
            };
            Atlas::new(size, texture)
        }
    }
}

struct Atlas {
    allocator: BucketedAtlasAllocator,
    texture: metal::Texture,
}

impl Atlas {
    fn new(size: Vector2I, texture: metal::Texture) -> Self {
        Self {
            allocator: BucketedAtlasAllocator::new(etagere::Size::new(size.x(), size.y())),
            texture,
        }
    }

    fn size(&self) -> Vector2I {
        let size = self.allocator.size();
        vec2i(size.width, size.height)
    }

    fn allocate(&mut self, size: Vector2I) -> Option<(etagere::AllocId, Vector2I)> {
        let alloc = self
            .allocator
            .allocate(etagere::Size::new(size.x(), size.y()))?;
        let origin = alloc.rectangle.min;
        Some((alloc.id, vec2i(origin.x, origin.y)))
    }

    fn upload(&mut self, bounds: RectI, bytes: &[u8]) {
        let region = metal::MTLRegion::new_2d(
            bounds.origin().x() as u64,
            bounds.origin().y() as u64,
            bounds.size().x() as u64,
            bounds.size().y() as u64,
        );
        self.texture.replace_region(
            region,
            0,
            bytes.as_ptr() as *const _,
            (bounds.size().x() * self.bytes_per_pixel() as i32) as u64,
        );
    }

    fn bytes_per_pixel(&self) -> u8 {
        use metal::MTLPixelFormat::*;
        match self.texture.pixel_format() {
            A8Unorm | R8Unorm => 1,
            RGBA8Unorm | BGRA8Unorm => 4,
            _ => unimplemented!(),
        }
    }

    fn deallocate(&mut self, id: etagere::AllocId) {
        self.allocator.deallocate(id);
    }

    fn is_empty(&self) -> bool {
        self.allocator.is_empty()
    }

    fn clear(&mut self) {
        self.allocator.clear();
    }
}
