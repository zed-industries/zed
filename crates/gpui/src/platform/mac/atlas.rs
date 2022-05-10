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
    last_used_atlas_id: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct AllocId {
    pub atlas_id: usize,
    alloc_id: etagere::AllocId,
}

impl AtlasAllocator {
    pub fn new(device: Device, texture_descriptor: TextureDescriptor) -> Self {
        let mut this = Self {
            device,
            texture_descriptor,
            atlases: vec![],
            last_used_atlas_id: 0,
        };
        let atlas = this.new_atlas(Vector2I::zero());
        this.atlases.push(atlas);
        this
    }

    pub fn default_atlas_size(&self) -> Vector2I {
        vec2i(
            self.texture_descriptor.width() as i32,
            self.texture_descriptor.height() as i32,
        )
    }

    pub fn allocate(&mut self, requested_size: Vector2I) -> Option<(AllocId, Vector2I)> {
        let atlas_id = self.last_used_atlas_id;
        if let Some((alloc_id, origin)) = self.atlases[atlas_id].allocate(requested_size) {
            return Some((AllocId { atlas_id, alloc_id }, origin));
        }

        for (atlas_id, atlas) in self.atlases.iter_mut().enumerate() {
            if atlas_id == self.last_used_atlas_id {
                continue;
            }
            if let Some((alloc_id, origin)) = atlas.allocate(requested_size) {
                self.last_used_atlas_id = atlas_id;
                return Some((AllocId { atlas_id, alloc_id }, origin));
            }
        }

        let atlas_id = self.atlases.len();
        let mut atlas = self.new_atlas(requested_size);
        let allocation = atlas
            .allocate(requested_size)
            .map(|(alloc_id, origin)| (AllocId { atlas_id, alloc_id }, origin));
        self.atlases.push(atlas);

        if allocation.is_none() {
            warn!(
                "allocation of size {:?} could not be created",
                requested_size,
            );
        }

        allocation
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
        }
    }

    pub fn clear(&mut self) {
        for atlas in &mut self.atlases {
            atlas.clear();
        }
    }

    pub fn texture(&self, atlas_id: usize) -> Option<&metal::TextureRef> {
        self.atlases.get(atlas_id).map(|a| a.texture.as_ref())
    }

    fn new_atlas(&mut self, required_size: Vector2I) -> Atlas {
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

    fn clear(&mut self) {
        self.allocator.clear();
    }
}
