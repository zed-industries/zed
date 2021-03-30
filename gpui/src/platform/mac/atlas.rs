use crate::geometry::vector::vec2i;
use crate::geometry::vector::Vector2I;
use anyhow::anyhow;
use etagere::BucketedAtlasAllocator;
use metal::{self, Device, TextureDescriptor};

pub struct AtlasAllocator {
    device: Device,
    texture_descriptor: TextureDescriptor,
    atlasses: Vec<Atlas>,
    free_atlasses: Vec<Atlas>,
}

impl AtlasAllocator {
    pub fn new(device: Device, texture_descriptor: TextureDescriptor) -> Self {
        let me = Self {
            device,
            texture_descriptor,
            atlasses: Vec::new(),
            free_atlasses: Vec::new(),
        };
        me.atlasses.push(me.new_atlas());
        me
    }

    fn atlas_size(&self) -> Vector2I {
        vec2i(
            self.texture_descriptor.width() as i32,
            self.texture_descriptor.height() as i32,
        )
    }

    pub fn allocate(&mut self, requested_size: Vector2I) -> anyhow::Result<(usize, Vector2I)> {
        let atlas_size = self.atlas_size();
        if requested_size.x() > atlas_size.x() || requested_size.y() > atlas_size.y() {
            return Err(anyhow!(
                "requested size {:?} too large for atlas {:?}",
                requested_size,
                atlas_size
            ));
        }

        let origin = self
            .atlasses
            .last_mut()
            .unwrap()
            .allocate(requested_size)
            .unwrap_or_else(|| {
                let mut atlas = self.new_atlas();
                let origin = atlas.allocate(requested_size).unwrap();
                self.atlasses.push(atlas);
                origin
            });

        Ok((self.atlasses.len() - 1, origin))
    }

    pub fn clear(&mut self) {
        for atlas in &mut self.atlasses {
            atlas.clear();
        }
        self.free_atlasses.extend(self.atlasses.drain(1..));
    }

    fn new_atlas(&mut self) -> Atlas {
        self.free_atlasses.pop().unwrap_or_else(|| {
            Atlas::new(
                self.atlas_size(),
                self.device.new_texture(&self.texture_descriptor),
            )
        })
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
