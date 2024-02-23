use super::{BladeBelt, BladeBeltDescriptor};
use crate::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTile, Bounds, DevicePixels, PlatformAtlas,
    Point, Size,
};
use anyhow::Result;
use blade_graphics as gpu;
use collections::FxHashMap;
use etagere::BucketedAtlasAllocator;
use parking_lot::Mutex;
use std::{borrow::Cow, ops, sync::Arc};

pub(crate) const PATH_TEXTURE_FORMAT: gpu::TextureFormat = gpu::TextureFormat::R16Float;

pub(crate) struct BladeAtlas(Mutex<BladeAtlasState>);

struct PendingUpload {
    id: AtlasTextureId,
    bounds: Bounds<DevicePixels>,
    data: gpu::BufferPiece,
}

struct BladeAtlasState {
    gpu: Arc<gpu::Context>,
    upload_belt: BladeBelt,
    storage: BladeAtlasStorage,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
    initializations: Vec<AtlasTextureId>,
    uploads: Vec<PendingUpload>,
}

impl BladeAtlasState {
    fn destroy(&mut self) {
        self.storage.destroy(&self.gpu);
        self.upload_belt.destroy(&self.gpu);
    }
}

pub struct BladeTextureInfo {
    pub size: gpu::Extent,
    pub raw_view: gpu::TextureView,
}

impl BladeAtlas {
    pub(crate) fn new(gpu: &Arc<gpu::Context>) -> Self {
        BladeAtlas(Mutex::new(BladeAtlasState {
            gpu: Arc::clone(gpu),
            upload_belt: BladeBelt::new(BladeBeltDescriptor {
                memory: gpu::Memory::Upload,
                min_chunk_size: 0x10000,
                alignment: 64, // Vulkan `optimalBufferCopyOffsetAlignment` on Intel XE
            }),
            storage: BladeAtlasStorage::default(),
            tiles_by_key: Default::default(),
            initializations: Vec::new(),
            uploads: Vec::new(),
        }))
    }

    pub(crate) fn destroy(&self) {
        self.0.lock().destroy();
    }

    pub(crate) fn clear_textures(&self, texture_kind: AtlasTextureKind) {
        let mut lock = self.0.lock();
        let textures = &mut lock.storage[texture_kind];
        for texture in textures {
            texture.clear();
        }
    }

    /// Allocate a rectangle and make it available for rendering immediately (without waiting for `before_frame`)
    pub fn allocate_for_rendering(
        &self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
        gpu_encoder: &mut gpu::CommandEncoder,
    ) -> AtlasTile {
        let mut lock = self.0.lock();
        let tile = lock.allocate(size, texture_kind);
        lock.flush_initializations(gpu_encoder);
        tile
    }

    pub fn before_frame(&self, gpu_encoder: &mut gpu::CommandEncoder) {
        let mut lock = self.0.lock();
        lock.flush(gpu_encoder);
    }

    pub fn after_frame(&self, sync_point: &gpu::SyncPoint) {
        let mut lock = self.0.lock();
        lock.upload_belt.flush(sync_point);
    }

    pub fn get_texture_info(&self, id: AtlasTextureId) -> BladeTextureInfo {
        let lock = self.0.lock();
        let texture = &lock.storage[id];
        let size = texture.allocator.size();
        BladeTextureInfo {
            size: gpu::Extent {
                width: size.width as u32,
                height: size.height as u32,
                depth: 1,
            },
            raw_view: texture.raw_view,
        }
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
            profiling::scope!("new tile");
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
        let textures = &mut self.storage[texture_kind];
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
                format = gpu::TextureFormat::R8Unorm;
                usage = gpu::TextureUsage::COPY | gpu::TextureUsage::RESOURCE;
            }
            AtlasTextureKind::Polychrome => {
                format = gpu::TextureFormat::Bgra8Unorm;
                usage = gpu::TextureUsage::COPY | gpu::TextureUsage::RESOURCE;
            }
            AtlasTextureKind::Path => {
                format = PATH_TEXTURE_FORMAT;
                usage = gpu::TextureUsage::COPY
                    | gpu::TextureUsage::RESOURCE
                    | gpu::TextureUsage::TARGET;
            }
        }

        let raw = self.gpu.create_texture(gpu::TextureDesc {
            name: "atlas",
            format,
            size: gpu::Extent {
                width: size.width.into(),
                height: size.height.into(),
                depth: 1,
            },
            array_layer_count: 1,
            mip_level_count: 1,
            dimension: gpu::TextureDimension::D2,
            usage,
        });
        let raw_view = self.gpu.create_texture_view(gpu::TextureViewDesc {
            name: "",
            texture: raw,
            format,
            dimension: gpu::ViewDimension::D2,
            subresources: &Default::default(),
        });

        let textures = &mut self.storage[kind];
        let atlas_texture = BladeAtlasTexture {
            id: AtlasTextureId {
                index: textures.len() as u32,
                kind,
            },
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            format,
            raw,
            raw_view,
        };

        self.initializations.push(atlas_texture.id);
        textures.push(atlas_texture);
        textures.last_mut().unwrap()
    }

    fn upload_texture(&mut self, id: AtlasTextureId, bounds: Bounds<DevicePixels>, bytes: &[u8]) {
        let data = unsafe { self.upload_belt.alloc_data(bytes, &self.gpu) };
        self.uploads.push(PendingUpload { id, bounds, data });
    }

    fn flush_initializations(&mut self, encoder: &mut gpu::CommandEncoder) {
        for id in self.initializations.drain(..) {
            let texture = &self.storage[id];
            encoder.init_texture(texture.raw);
        }
    }

    fn flush(&mut self, encoder: &mut gpu::CommandEncoder) {
        self.flush_initializations(encoder);

        let mut transfers = encoder.transfer();
        for upload in self.uploads.drain(..) {
            let texture = &self.storage[upload.id];
            transfers.copy_buffer_to_texture(
                upload.data,
                upload.bounds.size.width.to_bytes(texture.bytes_per_pixel()),
                gpu::TexturePiece {
                    texture: texture.raw,
                    mip_level: 0,
                    array_layer: 0,
                    origin: [
                        upload.bounds.origin.x.into(),
                        upload.bounds.origin.y.into(),
                        0,
                    ],
                },
                gpu::Extent {
                    width: upload.bounds.size.width.into(),
                    height: upload.bounds.size.height.into(),
                    depth: 1,
                },
            );
        }
    }
}

#[derive(Default)]
struct BladeAtlasStorage {
    monochrome_textures: Vec<BladeAtlasTexture>,
    polychrome_textures: Vec<BladeAtlasTexture>,
    path_textures: Vec<BladeAtlasTexture>,
}

impl ops::Index<AtlasTextureKind> for BladeAtlasStorage {
    type Output = Vec<BladeAtlasTexture>;
    fn index(&self, kind: AtlasTextureKind) -> &Self::Output {
        match kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
            crate::AtlasTextureKind::Path => &self.path_textures,
        }
    }
}

impl ops::IndexMut<AtlasTextureKind> for BladeAtlasStorage {
    fn index_mut(&mut self, kind: AtlasTextureKind) -> &mut Self::Output {
        match kind {
            crate::AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            crate::AtlasTextureKind::Path => &mut self.path_textures,
        }
    }
}

impl ops::Index<AtlasTextureId> for BladeAtlasStorage {
    type Output = BladeAtlasTexture;
    fn index(&self, id: AtlasTextureId) -> &Self::Output {
        let textures = match id.kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
            crate::AtlasTextureKind::Path => &self.path_textures,
        };
        &textures[id.index as usize]
    }
}

impl BladeAtlasStorage {
    fn destroy(&mut self, gpu: &gpu::Context) {
        for mut texture in self.monochrome_textures.drain(..) {
            texture.destroy(gpu);
        }
        for mut texture in self.polychrome_textures.drain(..) {
            texture.destroy(gpu);
        }
        for mut texture in self.path_textures.drain(..) {
            texture.destroy(gpu);
        }
    }
}

struct BladeAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    raw: gpu::Texture,
    raw_view: gpu::TextureView,
    format: gpu::TextureFormat,
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
            padding: 0,
            bounds: Bounds {
                origin: allocation.rectangle.min.into(),
                size,
            },
        };
        Some(tile)
    }

    fn destroy(&mut self, gpu: &gpu::Context) {
        gpu.destroy_texture(self.raw);
        gpu.destroy_texture_view(self.raw_view);
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
