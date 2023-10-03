use crate::{AtlasTextureId, AtlasTile, Bounds, DevicePixels, PlatformAtlas, Point, Size};
use collections::HashMap;
use etagere::BucketedAtlasAllocator;
use foreign_types::ForeignType;
use metal::{Device, TextureDescriptor, TextureDescriptorRef};
use objc::{msg_send, sel, sel_impl};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use std::hash::Hash;

pub struct MetalAtlas<Key>(RwLock<MetalAtlasState<Key>>);

struct MetalAtlasState<Key> {
    device: Device,
    texture_descriptor: TextureDescriptor,
    textures: Vec<MetalAtlasTexture>,
    tiles_by_key: HashMap<Key, AtlasTile>,
}

impl<Key> PlatformAtlas<Key> for MetalAtlas<Key>
where
    Key: Eq + Hash,
{
    fn get_or_insert_with(
        &self,
        key: Key,
        build: impl FnOnce() -> (Size<DevicePixels>, Vec<u8>),
    ) -> AtlasTile {
        let lock = self.0.upgradable_read();
        if let Some(tile) = lock.tiles_by_key.get(&key) {
            return tile.clone();
        } else {
            let mut lock = RwLockUpgradableReadGuard::upgrade(lock);
            let (size, bytes) = build();
            lock.textures
                .iter_mut()
                .rev()
                .find_map(|texture| texture.allocate(size, &bytes))
                .unwrap_or_else(|| {
                    let texture = lock.push_texture(size);
                    texture
                        .allocate(size, &bytes)
                        .expect("could not allocate a tile in new texture")
                })
        }
    }

    fn clear(&self) {
        self.0.write().tiles_by_key.clear();
    }
}

impl<Key> MetalAtlasState<Key> {
    fn push_texture(&mut self, min_size: Size<DevicePixels>) -> &mut MetalAtlasTexture {
        let default_atlas_size = Size {
            width: self.texture_descriptor.width().into(),
            height: self.texture_descriptor.height().into(),
        };
        let size;
        let metal_texture;

        if min_size.width > default_atlas_size.width || min_size.height > default_atlas_size.height
        {
            let descriptor = unsafe {
                let descriptor_ptr: *mut metal::MTLTextureDescriptor =
                    msg_send![self.texture_descriptor, copy];
                metal::TextureDescriptor::from_ptr(descriptor_ptr)
            };
            descriptor.set_width(min_size.width.into());
            descriptor.set_height(min_size.height.into());

            size = min_size;
            metal_texture = self.device.new_texture(&descriptor);
        } else {
            size = default_atlas_size;
            metal_texture = self.device.new_texture(&self.texture_descriptor);
        }

        let atlas_texture = MetalAtlasTexture {
            id: AtlasTextureId(self.textures.len()),
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            metal_texture,
        };
        self.textures.push(atlas_texture);
        self.textures.last_mut().unwrap()
    }
}

struct MetalAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    metal_texture: metal::Texture,
}

impl MetalAtlasTexture {
    fn allocate(&mut self, size: Size<DevicePixels>, bytes: &[u8]) -> Option<AtlasTile> {
        let size = size.into();
        let allocation = self.allocator.allocate(size)?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id,
            bounds_in_atlas: allocation.rectangle.into(),
        };
        let region = metal::MTLRegion::new_2d(
            u32::from(tile.bounds_in_atlas.origin.x) as u64,
            u32::from(tile.bounds_in_atlas.origin.y) as u64,
            u32::from(tile.bounds_in_atlas.size.width) as u64,
            u32::from(tile.bounds_in_atlas.size.height) as u64,
        );
        self.metal_texture.replace_region(
            region,
            0,
            bytes.as_ptr() as *const _,
            u32::from(
                tile.bounds_in_atlas
                    .size
                    .width
                    .to_bytes(self.bytes_per_pixel()),
            ) as u64,
        );
        Some(tile)
    }

    fn bytes_per_pixel(&self) -> u8 {
        use metal::MTLPixelFormat::*;
        match self.metal_texture.pixel_format() {
            A8Unorm | R8Unorm => 1,
            RGBA8Unorm | BGRA8Unorm => 4,
            _ => unimplemented!(),
        }
    }
}

impl From<Size<DevicePixels>> for etagere::Size {
    fn from(size: Size<DevicePixels>) -> Self {
        etagere::Size::new(u32::from(size.width) as i32, u32::from(size.width) as i32)
    }
}

impl From<etagere::Point> for Point<DevicePixels> {
    fn from(value: etagere::Point) -> Self {
        Point {
            x: DevicePixels::from(value.x as u32),
            y: DevicePixels::from(value.y as u32),
        }
    }
}

impl From<etagere::Size> for Size<DevicePixels> {
    fn from(size: etagere::Size) -> Self {
        Size {
            width: DevicePixels::from(size.width as u32),
            height: DevicePixels::from(size.height as u32),
        }
    }
}

impl From<etagere::Rectangle> for Bounds<DevicePixels> {
    fn from(rectangle: etagere::Rectangle) -> Self {
        Bounds {
            origin: rectangle.min.into(),
            size: rectangle.size().into(),
        }
    }
}
