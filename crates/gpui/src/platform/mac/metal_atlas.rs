use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Point, Size,
};
use anyhow::{anyhow, Result};
use collections::FxHashMap;
use derive_more::{Deref, DerefMut};
use etagere::BucketedAtlasAllocator;
use metal::Device;
use parking_lot::Mutex;
use std::borrow::Cow;

pub(crate) struct MetalAtlas(Mutex<MetalAtlasState>);

impl MetalAtlas {
    pub(crate) fn new(device: Device) -> Self {
        MetalAtlas(Mutex::new(MetalAtlasState {
            device: AssertSend(device),
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            path_textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
    }

    pub(crate) fn metal_texture(&self, id: AtlasTextureId) -> metal::Texture {
        self.0.lock().texture(id).metal_texture.clone()
    }

    pub(crate) fn allocate(
        &self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> Option<AtlasTile> {
        self.0.lock().allocate(size, texture_kind)
    }

    pub(crate) fn clear_textures(&self, texture_kind: AtlasTextureKind) {
        let mut lock = self.0.lock();
        let textures = match texture_kind {
            AtlasTextureKind::Monochrome => &mut lock.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut lock.polychrome_textures,
            AtlasTextureKind::Path => &mut lock.path_textures,
        };
        for texture in textures {
            texture.clear();
        }
    }
}

struct MetalAtlasState {
    device: AssertSend<Device>,
    monochrome_textures: Vec<MetalAtlasTexture>,
    polychrome_textures: Vec<MetalAtlasTexture>,
    path_textures: Vec<MetalAtlasTexture>,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
}

impl PlatformAtlas for MetalAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>> {
        let mut lock = self.0.lock();
        if let Some(tile) = lock.tiles_by_key.get(key) {
            Ok(Some(tile.clone()))
        } else {
            let Some((size, bytes)) = build()? else {
                return Ok(None);
            };
            let tile = lock
                .allocate(size, key.texture_kind())
                .ok_or_else(|| anyhow!("failed to allocate"))?;
            let texture = lock.texture(tile.texture_id);
            texture.upload(tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile.clone());
            Ok(Some(tile))
        }
    }
}

impl MetalAtlasState {
    fn allocate(
        &mut self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> Option<AtlasTile> {
        let textures = match texture_kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };

        textures
            .iter_mut()
            .rev()
            .find_map(|texture| texture.allocate(size))
            .or_else(|| {
                let texture = self.push_texture(size, texture_kind);
                texture.allocate(size)
            })
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut MetalAtlasTexture {
        const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(1024),
            height: DevicePixels(1024),
        };
        // Max texture size on all modern Apple GPUs. Anything bigger than that crashes in validateWithDevice.
        const MAX_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(16384),
            height: DevicePixels(16384),
        };
        let size = min_size.min(&MAX_ATLAS_SIZE).max(&DEFAULT_ATLAS_SIZE);
        let texture_descriptor = metal::TextureDescriptor::new();
        texture_descriptor.set_width(size.width.into());
        texture_descriptor.set_height(size.height.into());
        let pixel_format;
        let usage;
        match kind {
            AtlasTextureKind::Monochrome => {
                pixel_format = metal::MTLPixelFormat::A8Unorm;
                usage = metal::MTLTextureUsage::ShaderRead;
            }
            AtlasTextureKind::Polychrome => {
                pixel_format = metal::MTLPixelFormat::BGRA8Unorm;
                usage = metal::MTLTextureUsage::ShaderRead;
            }
            AtlasTextureKind::Path => {
                pixel_format = metal::MTLPixelFormat::R16Float;
                usage = metal::MTLTextureUsage::RenderTarget | metal::MTLTextureUsage::ShaderRead;
            }
        }
        texture_descriptor.set_pixel_format(pixel_format);
        texture_descriptor.set_usage(usage);
        let metal_texture = self.device.new_texture(&texture_descriptor);

        let textures = match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };
        let atlas_texture = MetalAtlasTexture {
            id: AtlasTextureId {
                index: textures.len() as u32,
                kind,
            },
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            metal_texture: AssertSend(metal_texture),
        };
        textures.push(atlas_texture);
        textures.last_mut().unwrap()
    }

    fn texture(&self, id: AtlasTextureId) -> &MetalAtlasTexture {
        let textures = match id.kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
            crate::AtlasTextureKind::Path => &self.path_textures,
        };
        &textures[id.index as usize]
    }
}

struct MetalAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    metal_texture: AssertSend<metal::Texture>,
}

impl MetalAtlasTexture {
    fn clear(&mut self) {
        self.allocator.clear();
    }

    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(size.into())?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            bounds: Bounds {
                origin: allocation.rectangle.min.into(),
                size,
            },
            padding: 0,
        };
        Some(tile)
    }

    fn upload(&self, bounds: Bounds<DevicePixels>, bytes: &[u8]) {
        let region = metal::MTLRegion::new_2d(
            bounds.origin.x.into(),
            bounds.origin.y.into(),
            bounds.size.width.into(),
            bounds.size.height.into(),
        );
        self.metal_texture.replace_region(
            region,
            0,
            bytes.as_ptr() as *const _,
            bounds.size.width.to_bytes(self.bytes_per_pixel()) as u64,
        );
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
