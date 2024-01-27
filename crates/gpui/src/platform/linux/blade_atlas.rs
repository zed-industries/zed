use super::{BladeBelt, BladeBeltDescriptor};
use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Point, Size,
};
use anyhow::Result;
use collections::FxHashMap;
use derive_more::{Deref, DerefMut};
use etagere::BucketedAtlasAllocator;
use parking_lot::Mutex;
use std::{borrow::Cow, sync::Arc};

pub(crate) struct BladeAtlas(Mutex<BladeAtlasState>);

struct BladeAtlasState {
    gpu: Arc<blade::Context>,
    gpu_encoder: blade::CommandEncoder,
    upload_belt: BladeBelt,
    monochrome_textures: Vec<BladeAtlasTexture>,
    polychrome_textures: Vec<BladeAtlasTexture>,
    path_textures: Vec<BladeAtlasTexture>,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
}

impl BladeAtlas {
    pub(crate) fn new(gpu: &Arc<blade::Context>) -> Self {
        BladeAtlas(Mutex::new(BladeAtlasState {
            gpu: Arc::clone(gpu),
            gpu_encoder: gpu.create_command_encoder(blade::CommandEncoderDesc {
                name: "atlas",
                buffer_count: 3,
            }),
            upload_belt: BladeBelt::new(BladeBeltDescriptor {
                memory: blade::Memory::Upload,
                min_chunk_size: 0x10000,
            }),
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            path_textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
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

    pub fn start_frame(&self) {
        let mut lock = self.0.lock();
        lock.gpu_encoder.start();
    }

    pub fn finish_frame(&self) -> blade::SyncPoint {
        let mut lock = self.0.lock();
        let gpu = lock.gpu.clone();
        let sync_point = gpu.submit(&mut lock.gpu_encoder);
        lock.upload_belt.flush(&sync_point);
        sync_point
    }
}

impl PlatformAtlas for BladeAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<(Size<DevicePixels>, Cow<'a, [u8]>)>,
    ) -> Result<AtlasTile> {
        let mut lock = self.0.lock();
        if let Some(tile) = lock.tiles_by_key.get(key) {
            Ok(tile.clone())
        } else {
            let (size, bytes) = build()?;
            let tile = lock.allocate(size, key.texture_kind());
            lock.upload_texture(tile.texture_id, tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile.clone());
            Ok(tile)
        }
    }
}

impl BladeAtlasState {
    fn allocate(&mut self, size: Size<DevicePixels>, texture_kind: AtlasTextureKind) -> AtlasTile {
        let textures = match texture_kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };
        textures
            .iter_mut()
            .rev()
            .find_map(|texture| texture.allocate(size))
            .unwrap_or_else(|| {
                let texture = self.push_texture(size, texture_kind);
                texture.allocate(size).unwrap()
            })
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut BladeAtlasTexture {
        const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(1024),
            height: DevicePixels(1024),
        };

        let size = min_size.max(&DEFAULT_ATLAS_SIZE);
        let format;
        let usage;
        match kind {
            AtlasTextureKind::Monochrome => {
                format = blade::TextureFormat::R8Unorm;
                usage = blade::TextureUsage::COPY | blade::TextureUsage::RESOURCE;
            }
            AtlasTextureKind::Polychrome => {
                format = blade::TextureFormat::Bgra8Unorm;
                usage = blade::TextureUsage::COPY | blade::TextureUsage::RESOURCE;
            }
            AtlasTextureKind::Path => {
                format = blade::TextureFormat::R16Float;
                usage = blade::TextureUsage::COPY
                    | blade::TextureUsage::RESOURCE
                    | blade::TextureUsage::TARGET;
            }
        }

        let raw = self.gpu.create_texture(blade::TextureDesc {
            name: "",
            format,
            size: blade::Extent {
                width: size.width.into(),
                height: size.height.into(),
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            dimension: blade::TextureDimension::D2,
            usage,
        });

        let textures = match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };
        let atlas_texture = BladeAtlasTexture {
            id: AtlasTextureId {
                index: textures.len() as u32,
                kind,
            },
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            format,
            raw,
        };
        textures.push(atlas_texture);
        textures.last_mut().unwrap()
    }

    fn upload_texture(&mut self, id: AtlasTextureId, bounds: Bounds<DevicePixels>, bytes: &[u8]) {
        let textures = match id.kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
            crate::AtlasTextureKind::Path => &self.path_textures,
        };
        let texture = &textures[id.index as usize];

        let src_data = self.upload_belt.alloc_data(bytes, &self.gpu);

        let mut transfers = self.gpu_encoder.transfer();
        transfers.copy_buffer_to_texture(
            src_data,
            bounds.size.width.to_bytes(texture.bytes_per_pixel()),
            blade::TexturePiece {
                texture: texture.raw,
                mip_level: 0,
                array_layer: 0,
                origin: [bounds.origin.x.into(), bounds.origin.y.into(), 0],
            },
            blade::Extent {
                width: bounds.size.width.into(),
                height: bounds.size.height.into(),
                depth: 1,
            },
        );
    }
}

struct BladeAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    raw: blade::Texture,
    format: blade::TextureFormat,
}

impl BladeAtlasTexture {
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
        };
        Some(tile)
    }

    fn bytes_per_pixel(&self) -> u8 {
        self.format.block_info().size
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
