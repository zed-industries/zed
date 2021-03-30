use crate::geometry::vector::{vec2i, Vector2I};
use etagere::BucketedAtlasAllocator;
use foreign_types::ForeignType;
use metal::{self, Device, TextureDescriptor};
use objc::{msg_send, sel, sel_impl};

pub struct AtlasAllocator {
    device: Device,
    texture_descriptor: TextureDescriptor,
    atlases: Vec<Atlas>,
    free_atlases: Vec<Atlas>,
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

    pub fn allocate(&mut self, requested_size: Vector2I) -> anyhow::Result<(usize, Vector2I)> {
        let origin = self
            .atlases
            .last_mut()
            .unwrap()
            .allocate(requested_size)
            .unwrap_or_else(|| {
                let mut atlas = self.new_atlas(requested_size);
                let origin = atlas.allocate(requested_size).unwrap();
                self.atlases.push(atlas);
                origin
            });

        Ok((self.atlases.len() - 1, origin))
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

    fn allocate(&mut self, size: Vector2I) -> Option<Vector2I> {
        let origin = self
            .allocator
            .allocate(etagere::Size::new(size.x(), size.y()))?
            .rectangle
            .min;
        Some(vec2i(origin.x, origin.y))
    }

    fn clear(&mut self) {
        self.allocator.clear();
    }
}
