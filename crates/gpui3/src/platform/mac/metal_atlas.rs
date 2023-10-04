use crate::{
    AtlasKey, AtlasTextureId, AtlasTile, Bounds, DevicePixels, PlatformAtlas, Point, Size,
};
use anyhow::{anyhow, Result};
use collections::HashMap;
use derive_more::{Deref, DerefMut};
use etagere::BucketedAtlasAllocator;
use metal::Device;
use parking_lot::Mutex;

pub struct MetalAtlas(Mutex<MetalAtlasState>);

impl MetalAtlas {
    pub fn new(device: Device) -> Self {
        MetalAtlas(Mutex::new(MetalAtlasState {
            device: AssertSend(device),
            textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
    }

    pub(crate) fn texture(&self, id: AtlasTextureId) -> metal::Texture {
        self.0.lock().textures[id.0 as usize].metal_texture.clone()
    }
}

struct MetalAtlasState {
    device: AssertSend<Device>,
    textures: Vec<MetalAtlasTexture>,
    tiles_by_key: HashMap<AtlasKey, AtlasTile>,
}

impl PlatformAtlas for MetalAtlas {
    fn get_or_insert_with(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<(Size<DevicePixels>, Vec<u8>)>,
    ) -> Result<AtlasTile> {
        let mut lock = self.0.lock();
        if let Some(tile) = lock.tiles_by_key.get(key) {
            return Ok(tile.clone());
        } else {
            let (size, bytes) = build()?;
            let tile = lock
                .textures
                .iter_mut()
                .rev()
                .find_map(|texture| {
                    if texture.monochrome == key.is_monochrome() {
                        texture.upload(size, &bytes)
                    } else {
                        None
                    }
                })
                .or_else(|| {
                    let texture = lock.push_texture(size, key.is_monochrome());
                    texture.upload(size, &bytes)
                })
                .ok_or_else(|| anyhow!("could not allocate in new texture"))?;
            lock.tiles_by_key.insert(key.clone(), tile.clone());
            Ok(tile)
        }
    }

    fn clear(&self) {
        self.0.lock().tiles_by_key.clear();
    }
}

impl MetalAtlasState {
    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        monochrome: bool,
    ) -> &mut MetalAtlasTexture {
        const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(1024),
            height: DevicePixels(1024),
        };

        let size = min_size.max(&DEFAULT_ATLAS_SIZE);
        let texture_descriptor = metal::TextureDescriptor::new();
        texture_descriptor.set_width(size.width.into());
        texture_descriptor.set_height(size.height.into());
        if monochrome {
            texture_descriptor.set_pixel_format(metal::MTLPixelFormat::A8Unorm);
        } else {
            texture_descriptor.set_pixel_format(metal::MTLPixelFormat::BGRA8Unorm);
        }
        let metal_texture = self.device.new_texture(&texture_descriptor);

        let atlas_texture = MetalAtlasTexture {
            id: AtlasTextureId(self.textures.len() as u32),
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            metal_texture: AssertSend(metal_texture),
            monochrome,
        };
        self.textures.push(atlas_texture);
        self.textures.last_mut().unwrap()
    }
}

struct MetalAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    metal_texture: AssertSend<metal::Texture>,
    monochrome: bool,
}

impl MetalAtlasTexture {
    fn upload(&mut self, size: Size<DevicePixels>, bytes: &[u8]) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(size.into())?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            bounds: Bounds {
                origin: allocation.rectangle.min.into(),
                size,
            },
        };

        let region = metal::MTLRegion::new_2d(
            tile.bounds.origin.x.into(),
            tile.bounds.origin.y.into(),
            tile.bounds.size.width.into(),
            tile.bounds.size.height.into(),
        );
        self.metal_texture.replace_region(
            region,
            0,
            bytes.as_ptr() as *const _,
            u32::from(tile.bounds.size.width.to_bytes(self.bytes_per_pixel())) as u64,
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
        etagere::Size::new(size.width.into(), size.height.into())
    }
}

impl From<etagere::Point> for Point<DevicePixels> {
    fn from(value: etagere::Point) -> Self {
        Point {
            x: DevicePixels::from(value.x),
            y: DevicePixels::from(value.y),
        }
    }
}

impl From<etagere::Size> for Size<DevicePixels> {
    fn from(size: etagere::Size) -> Self {
        Size {
            width: DevicePixels::from(size.width),
            height: DevicePixels::from(size.height),
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

#[derive(Deref, DerefMut)]
struct AssertSend<T>(T);

unsafe impl<T> Send for AssertSend<T> {}
