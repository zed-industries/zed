use anyhow::{Context as _, Result};
use collections::FxHashMap;
use etagere::{BucketedAtlasAllocator, size2};
use gpui::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTextureList, AtlasTile, Bounds, DevicePixels,
    PlatformAtlas, Point, Size,
};
use parking_lot::Mutex;
use std::{borrow::Cow, ops, sync::Arc};

use crate::WgpuContext;

/// Upper bound on the CPU-side bytes held in the pending upload queue before it
/// is flushed. Full-texture updates coalesce, so this only grows for partial
/// updates while no frame is draining the queue (hidden/minimized window).
const MAX_PENDING_UPLOAD_BYTES: usize = 64 * 1024 * 1024;

fn device_size_to_etagere(size: Size<DevicePixels>) -> etagere::Size {
    size2(size.width.0, size.height.0)
}

fn etagere_point_to_device(point: etagere::Point) -> Point<DevicePixels> {
    Point {
        x: DevicePixels(point.x),
        y: DevicePixels(point.y),
    }
}

pub struct WgpuAtlas(Mutex<WgpuAtlasState>);

struct PendingUpload {
    id: AtlasTextureId,
    bounds: Bounds<DevicePixels>,
    data: Vec<u8>,
}

struct WgpuAtlasState {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    max_texture_size: u32,
    color_texture_format: wgpu::TextureFormat,
    storage: WgpuAtlasStorage,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
    pending_uploads: Vec<PendingUpload>,
    pending_upload_bytes: usize,
    max_pending_upload_bytes: usize,
    resource_generation: u64,
}

pub struct WgpuTextureInfo {
    pub view: wgpu::TextureView,
}

impl WgpuAtlas {
    pub fn new(
        device: Arc<wgpu::Device>,
        queue: Arc<wgpu::Queue>,
        color_texture_format: wgpu::TextureFormat,
    ) -> Self {
        let max_texture_size = device.limits().max_texture_dimension_2d;
        WgpuAtlas(Mutex::new(WgpuAtlasState {
            device,
            queue,
            max_texture_size,
            color_texture_format,
            storage: WgpuAtlasStorage::default(),
            tiles_by_key: Default::default(),
            pending_uploads: Vec::new(),
            pending_upload_bytes: 0,
            max_pending_upload_bytes: MAX_PENDING_UPLOAD_BYTES,
            resource_generation: 0,
        }))
    }

    pub fn from_context(context: &WgpuContext) -> Self {
        Self::new(
            context.device.clone(),
            context.queue.clone(),
            context.color_texture_format(),
        )
    }

    pub fn before_frame(&self) {
        let mut lock = self.0.lock();
        lock.flush_uploads();
    }

    #[cfg(test)]
    pub fn set_max_pending_upload_bytes(&self, bytes: usize) {
        self.0.lock().max_pending_upload_bytes = bytes;
    }

    pub fn get_texture_info(&self, id: AtlasTextureId) -> WgpuTextureInfo {
        let lock = self.0.lock();
        let texture = &lock.storage[id];
        WgpuTextureInfo {
            view: texture.view.clone(),
        }
    }

    /// Clears all cached textures and tiles, forcing them to be recreated.
    /// Use this for incremental recovery when the device is still valid.
    pub fn clear(&self) {
        let mut lock = self.0.lock();
        lock.invalidate_resources();
    }

    /// Handles device lost by clearing all textures and cached tiles.
    /// The atlas will lazily recreate textures as needed on subsequent frames.
    pub fn handle_device_lost(&self, context: &WgpuContext) {
        let mut lock = self.0.lock();
        lock.device = context.device.clone();
        lock.queue = context.queue.clone();
        lock.color_texture_format = context.color_texture_format();
        lock.invalidate_resources();
    }
}

impl PlatformAtlas for WgpuAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> Result<Option<AtlasTile>> {
        let mut lock = self.0.lock();
        if let Some(tile) = lock.tiles_by_key.get(key) {
            Ok(Some(*tile))
        } else {
            profiling::scope!("new tile");
            let Some((size, bytes)) = build()? else {
                return Ok(None);
            };
            let tile = if matches!(key, AtlasKey::DynamicTexture(_)) {
                lock.allocate_dedicated(size, key.texture_kind())
            } else {
                lock.allocate(size, key.texture_kind())
            }
            .context("failed to allocate")?;
            lock.upload_texture(tile.texture_id, tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile);
            Ok(Some(tile))
        }
    }

    fn update(&self, key: &AtlasKey, bounds: Bounds<DevicePixels>, bytes: &[u8]) -> Result<()> {
        let mut lock = self.0.lock();
        let Some(tile) = lock.tiles_by_key.get(key).copied() else {
            return Ok(());
        };

        lock.validate_upload(tile, bounds, bytes)?;
        let upload_bounds = Bounds {
            origin: Point {
                x: DevicePixels(
                    tile.bounds
                        .origin
                        .x
                        .0
                        .checked_add(bounds.origin.x.0)
                        .context("texture update horizontal origin overflow")?,
                ),
                y: DevicePixels(
                    tile.bounds
                        .origin
                        .y
                        .0
                        .checked_add(bounds.origin.y.0)
                        .context("texture update vertical origin overflow")?,
                ),
            },
            size: bounds.size,
        };
        lock.upload_texture(tile.texture_id, upload_bounds, bytes);
        Ok(())
    }

    fn remove(&self, key: &AtlasKey) {
        let mut lock = self.0.lock();

        let Some(tile) = lock.tiles_by_key.remove(key) else {
            return;
        };
        let id = tile.texture_id;

        let Some(texture_slot) = lock.storage[id.kind].textures.get_mut(id.index as usize) else {
            return;
        };

        if let Some(mut texture) = texture_slot.take() {
            texture.allocator.deallocate(tile.tile_id.into());
            texture.decrement_ref_count();
            if texture.is_unreferenced() {
                lock.pending_uploads
                    .retain(|upload| upload.id != texture.id);
                lock.pending_upload_bytes = lock
                    .pending_uploads
                    .iter()
                    .map(|upload| upload.data.len())
                    .sum();
                lock.storage[id.kind]
                    .free_list
                    .push(texture.id.index as usize);
            } else {
                *texture_slot = Some(texture);
            }
        }
    }

    fn resource_generation(&self) -> u64 {
        self.0.lock().resource_generation
    }

    fn max_texture_size(&self) -> Option<Size<DevicePixels>> {
        let max = self.0.lock().max_texture_size as i32;
        Some(Size {
            width: DevicePixels(max),
            height: DevicePixels(max),
        })
    }
}

impl WgpuAtlasState {
    fn invalidate_resources(&mut self) {
        self.storage = WgpuAtlasStorage::default();
        self.tiles_by_key.clear();
        self.pending_uploads.clear();
        self.pending_upload_bytes = 0;
        self.resource_generation = self
            .resource_generation
            .checked_add(1)
            .expect("WGPU atlas resource generation exhausted");
    }

    fn allocate_dedicated(
        &mut self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> Option<AtlasTile> {
        let max_texture_size = self.max_texture_size as i32;
        if size.width.0 <= 0
            || size.height.0 <= 0
            || size.width.0 > max_texture_size
            || size.height.0 > max_texture_size
        {
            return None;
        }

        self.push_texture_with_size(size, texture_kind)
            .allocate(size)
    }

    fn allocate(
        &mut self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> Option<AtlasTile> {
        {
            let textures = &mut self.storage[texture_kind];

            if let Some(tile) = textures
                .iter_mut()
                .rev()
                .find_map(|texture| texture.allocate(size))
            {
                return Some(tile);
            }
        }

        let texture = self.push_texture(size, texture_kind);
        texture.allocate(size)
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut WgpuAtlasTexture {
        const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(1024),
            height: DevicePixels(1024),
        };
        let max_texture_size = self.max_texture_size as i32;
        let max_atlas_size = Size {
            width: DevicePixels(max_texture_size),
            height: DevicePixels(max_texture_size),
        };

        let size = min_size.min(&max_atlas_size).max(&DEFAULT_ATLAS_SIZE);
        self.push_texture_with_size(size, kind)
    }

    fn push_texture_with_size(
        &mut self,
        size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut WgpuAtlasTexture {
        let format = match kind {
            AtlasTextureKind::Monochrome => wgpu::TextureFormat::R8Unorm,
            AtlasTextureKind::Subpixel | AtlasTextureKind::Polychrome => self.color_texture_format,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("atlas"),
            size: wgpu::Extent3d {
                width: size.width.0 as u32,
                height: size.height.0 as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_list = &mut self.storage[kind];
        let index = texture_list.free_list.pop();

        let atlas_texture = WgpuAtlasTexture {
            id: AtlasTextureId {
                index: index.unwrap_or(texture_list.textures.len()) as u32,
                kind,
            },
            allocator: BucketedAtlasAllocator::new(device_size_to_etagere(size)),
            format,
            size,
            texture,
            view,
            live_atlas_keys: 0,
        };

        if let Some(ix) = index {
            texture_list.textures[ix] = Some(atlas_texture);
            texture_list
                .textures
                .get_mut(ix)
                .and_then(|t| t.as_mut())
                .expect("texture must exist")
        } else {
            texture_list.textures.push(Some(atlas_texture));
            texture_list
                .textures
                .last_mut()
                .and_then(|t| t.as_mut())
                .expect("texture must exist")
        }
    }

    fn upload_texture(&mut self, id: AtlasTextureId, bounds: Bounds<DevicePixels>, bytes: &[u8]) {
        let (data, texture_size) = match self.storage.get(id) {
            Some(texture) => (swizzle_upload_data(bytes, texture.format), texture.size),
            None => (
                bytes.to_vec(),
                Size {
                    width: DevicePixels(0),
                    height: DevicePixels(0),
                },
            ),
        };

        let is_full_update = bounds.origin == Point::default() && bounds.size == texture_size;
        if is_full_update {
            // A newer full-frame upload supersedes earlier queued uploads for the
            // same texture, so it cannot grow the pending queue without bound.
            self.pending_uploads.retain(|upload| upload.id != id);
            self.pending_upload_bytes = self
                .pending_uploads
                .iter()
                .map(|upload| upload.data.len())
                .sum();
        }

        if self.pending_upload_bytes.saturating_add(data.len()) > self.max_pending_upload_bytes {
            // No frame is draining the queue (e.g. a hidden window): flush what is
            // queued so CPU memory stays bounded while updates keep arriving.
            self.flush_uploads();
        }

        self.pending_upload_bytes = self.pending_upload_bytes.saturating_add(data.len());
        self.pending_uploads
            .push(PendingUpload { id, bounds, data });
    }

    fn validate_upload(
        &self,
        tile: AtlasTile,
        bounds: Bounds<DevicePixels>,
        bytes: &[u8],
    ) -> Result<()> {
        anyhow::ensure!(
            bounds.origin.x.0 >= 0 && bounds.origin.y.0 >= 0,
            "texture update origin must be non-negative"
        );
        anyhow::ensure!(
            bounds.size.width.0 > 0 && bounds.size.height.0 > 0,
            "texture update size must be positive"
        );
        let right = bounds
            .origin
            .x
            .0
            .checked_add(bounds.size.width.0)
            .context("texture update horizontal bounds overflow")?;
        let bottom = bounds
            .origin
            .y
            .0
            .checked_add(bounds.size.height.0)
            .context("texture update vertical bounds overflow")?;
        anyhow::ensure!(
            right <= tile.bounds.size.width.0 && bottom <= tile.bounds.size.height.0,
            "texture update exceeds the allocated tile bounds"
        );

        let texture = self
            .storage
            .get(tile.texture_id)
            .context("texture update references a missing texture")?;
        let expected_len = usize::try_from(bounds.size.width.0)
            .ok()
            .and_then(|width| {
                usize::try_from(bounds.size.height.0)
                    .ok()
                    .and_then(|height| width.checked_mul(height))
            })
            .and_then(|pixels| pixels.checked_mul(texture.bytes_per_pixel() as usize))
            .context("texture update byte size overflow")?;
        anyhow::ensure!(
            bytes.len() == expected_len,
            "texture update contains {} bytes, expected {expected_len}",
            bytes.len()
        );
        Ok(())
    }

    fn flush_uploads(&mut self) {
        self.pending_upload_bytes = 0;
        for upload in self.pending_uploads.drain(..) {
            let Some(texture) = self.storage.get(upload.id) else {
                continue;
            };
            let bytes_per_pixel = texture.bytes_per_pixel();

            self.queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture.texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d {
                        x: upload.bounds.origin.x.0 as u32,
                        y: upload.bounds.origin.y.0 as u32,
                        z: 0,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &upload.data,
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(upload.bounds.size.width.0 as u32 * bytes_per_pixel as u32),
                    rows_per_image: None,
                },
                wgpu::Extent3d {
                    width: upload.bounds.size.width.0 as u32,
                    height: upload.bounds.size.height.0 as u32,
                    depth_or_array_layers: 1,
                },
            );
        }
    }
}

#[derive(Default)]
struct WgpuAtlasStorage {
    monochrome_textures: AtlasTextureList<WgpuAtlasTexture>,
    subpixel_textures: AtlasTextureList<WgpuAtlasTexture>,
    polychrome_textures: AtlasTextureList<WgpuAtlasTexture>,
}

impl ops::Index<AtlasTextureKind> for WgpuAtlasStorage {
    type Output = AtlasTextureList<WgpuAtlasTexture>;
    fn index(&self, kind: AtlasTextureKind) -> &Self::Output {
        match kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures,
            AtlasTextureKind::Subpixel => &self.subpixel_textures,
            AtlasTextureKind::Polychrome => &self.polychrome_textures,
        }
    }
}

impl ops::IndexMut<AtlasTextureKind> for WgpuAtlasStorage {
    fn index_mut(&mut self, kind: AtlasTextureKind) -> &mut Self::Output {
        match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Subpixel => &mut self.subpixel_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
        }
    }
}

impl WgpuAtlasStorage {
    fn get(&self, id: AtlasTextureId) -> Option<&WgpuAtlasTexture> {
        self[id.kind]
            .textures
            .get(id.index as usize)
            .and_then(|t| t.as_ref())
    }
}

impl ops::Index<AtlasTextureId> for WgpuAtlasStorage {
    type Output = WgpuAtlasTexture;
    fn index(&self, id: AtlasTextureId) -> &Self::Output {
        let textures = match id.kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures,
            AtlasTextureKind::Subpixel => &self.subpixel_textures,
            AtlasTextureKind::Polychrome => &self.polychrome_textures,
        };
        textures[id.index as usize]
            .as_ref()
            .expect("texture must exist")
    }
}

struct WgpuAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    texture: wgpu::Texture,
    view: wgpu::TextureView,
    format: wgpu::TextureFormat,
    size: Size<DevicePixels>,
    live_atlas_keys: u32,
}

impl WgpuAtlasTexture {
    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(device_size_to_etagere(size))?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            padding: 0,
            bounds: Bounds {
                origin: etagere_point_to_device(allocation.rectangle.min),
                size,
            },
        };
        self.live_atlas_keys += 1;
        Some(tile)
    }

    fn bytes_per_pixel(&self) -> u8 {
        match self.format {
            wgpu::TextureFormat::R8Unorm => 1,
            wgpu::TextureFormat::Bgra8Unorm | wgpu::TextureFormat::Rgba8Unorm => 4,
            _ => 4,
        }
    }

    fn decrement_ref_count(&mut self) {
        self.live_atlas_keys -= 1;
    }

    fn is_unreferenced(&self) -> bool {
        self.live_atlas_keys == 0
    }
}

fn swizzle_upload_data(bytes: &[u8], format: wgpu::TextureFormat) -> Vec<u8> {
    match format {
        wgpu::TextureFormat::Rgba8Unorm => {
            let mut data = bytes.to_vec();
            for pixel in data.chunks_exact_mut(4) {
                pixel.swap(0, 2);
            }
            data
        }
        _ => bytes.to_vec(),
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use gpui::block_on;
    use gpui::{DynamicTextureId, DynamicTextureParams, ImageId, RenderImageParams};
    use std::sync::Arc;

    fn dynamic_texture_key(id: usize) -> AtlasKey {
        AtlasKey::DynamicTexture(DynamicTextureParams {
            texture_id: DynamicTextureId(id),
        })
    }

    fn texture_size(width: i32, height: i32) -> Size<DevicePixels> {
        Size {
            width: DevicePixels(width),
            height: DevicePixels(height),
        }
    }

    fn texture_bounds(x: i32, y: i32, width: i32, height: i32) -> Bounds<DevicePixels> {
        Bounds {
            origin: Point {
                x: DevicePixels(x),
                y: DevicePixels(y),
            },
            size: texture_size(width, height),
        }
    }

    fn test_device_and_queue() -> anyhow::Result<(Arc<wgpu::Device>, Arc<wgpu::Queue>)> {
        block_on(async {
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: wgpu::Backends::all(),
                flags: wgpu::InstanceFlags::default(),
                backend_options: wgpu::BackendOptions::default(),
                memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
                display: None,
            });
            let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::LowPower,
                    compatible_surface: None,
                    force_fallback_adapter: false,
                })
                .await
                .map_err(|error| anyhow::anyhow!("failed to request adapter: {error}"))?;
            let (device, queue) = adapter
                .request_device(&wgpu::DeviceDescriptor {
                    label: Some("wgpu_atlas_test_device"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_defaults()
                        .using_resolution(adapter.limits())
                        .using_alignment(adapter.limits()),
                    memory_hints: wgpu::MemoryHints::MemoryUsage,
                    trace: wgpu::Trace::Off,
                    experimental_features: wgpu::ExperimentalFeatures::disabled(),
                })
                .await
                .map_err(|error| anyhow::anyhow!("failed to request device: {error}"))?;
            Ok((Arc::new(device), Arc::new(queue)))
        })
    }

    #[test]
    fn before_frame_skips_uploads_for_removed_texture() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;

        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let key = AtlasKey::Image(RenderImageParams {
            image_id: ImageId(1),
            frame_index: 0,
        });
        let size = Size {
            width: DevicePixels(1),
            height: DevicePixels(1),
        };
        let mut build = || Ok(Some((size, Cow::Owned(vec![0, 0, 0, 255]))));

        // Regression test: before the fix, this panicked in flush_uploads
        atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("tile should be created");
        atlas.remove(&key);
        atlas.before_frame();
        Ok(())
    }

    #[test]
    fn remove_deallocates_tile_space_for_reuse() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);

        let small = Size {
            width: DevicePixels(64),
            height: DevicePixels(64),
        };
        let big = Size {
            width: DevicePixels(700),
            height: DevicePixels(700),
        };

        let make_key = |image_id: usize| {
            AtlasKey::Image(RenderImageParams {
                image_id: ImageId(image_id),
                frame_index: 0,
            })
        };
        let insert = |key: &AtlasKey, size: Size<DevicePixels>| {
            let byte_count = (size.width.0 as usize) * (size.height.0 as usize) * 4;
            atlas
                .get_or_insert_with(key, &mut || {
                    Ok(Some((size, Cow::Owned(vec![0u8; byte_count]))))
                })
                .expect("allocation should succeed")
                .expect("callback returns Some")
        };

        let keeper_key = make_key(1);
        let big_key_a = make_key(2);
        let big_key_b = make_key(3);

        let keeper_tile = insert(&keeper_key, small);
        let tile_a = insert(&big_key_a, big);
        assert_eq!(keeper_tile.texture_id, tile_a.texture_id);

        atlas.remove(&big_key_a);
        let tile_b = insert(&big_key_b, big);
        assert_eq!(tile_b.texture_id, keeper_tile.texture_id);
        Ok(())
    }

    #[test]
    fn swizzle_upload_data_preserves_bgra_uploads() {
        let input = vec![0x10, 0x20, 0x30, 0x40];
        assert_eq!(
            swizzle_upload_data(&input, wgpu::TextureFormat::Bgra8Unorm),
            input
        );
    }

    #[test]
    fn swizzle_upload_data_converts_bgra_to_rgba() {
        let input = vec![0x10, 0x20, 0x30, 0x40, 0xAA, 0xBB, 0xCC, 0xDD];
        assert_eq!(
            swizzle_upload_data(&input, wgpu::TextureFormat::Rgba8Unorm),
            vec![0x30, 0x20, 0x10, 0x40, 0xCC, 0xBB, 0xAA, 0xDD]
        );
    }

    #[test]
    fn dynamic_textures_receive_dedicated_exact_size_allocations() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let size = texture_size(3, 2);
        let bytes = vec![0; 3 * 2 * 4];

        let first_key = dynamic_texture_key(1);
        let second_key = dynamic_texture_key(2);
        let mut first_build = || Ok(Some((size, Cow::Borrowed(bytes.as_slice()))));
        let mut second_build = || Ok(Some((size, Cow::Borrowed(bytes.as_slice()))));
        let first_tile = atlas
            .get_or_insert_with(&first_key, &mut first_build)?
            .expect("first dynamic texture should be allocated");
        let second_tile = atlas
            .get_or_insert_with(&second_key, &mut second_build)?
            .expect("second dynamic texture should be allocated");

        assert_ne!(first_tile.texture_id, second_tile.texture_id);
        assert_eq!(first_tile.bounds, texture_bounds(0, 0, 3, 2));
        assert_eq!(second_tile.bounds, texture_bounds(0, 0, 3, 2));

        let lock = atlas.0.lock();
        let first_texture = lock
            .storage
            .get(first_tile.texture_id)
            .expect("first backing texture should exist");
        assert_eq!(first_texture.texture.width(), 3);
        assert_eq!(first_texture.texture.height(), 2);
        Ok(())
    }

    #[test]
    fn dynamic_texture_update_offsets_and_swizzles_pending_upload() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Rgba8Unorm);
        let key = dynamic_texture_key(3);
        let size = texture_size(2, 2);
        let initial_bytes = vec![0; 2 * 2 * 4];
        let mut build = || Ok(Some((size, Cow::Borrowed(initial_bytes.as_slice()))));
        atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("dynamic texture should be allocated");

        let dirty_bytes = [0x10, 0x20, 0x30, 0x40, 0xAA, 0xBB, 0xCC, 0xDD];
        atlas.update(&key, texture_bounds(1, 0, 1, 2), &dirty_bytes)?;

        let lock = atlas.0.lock();
        let upload = lock
            .pending_uploads
            .last()
            .expect("dirty upload should be queued");
        assert_eq!(upload.bounds, texture_bounds(1, 0, 1, 2));
        assert_eq!(
            upload.data,
            vec![0x30, 0x20, 0x10, 0x40, 0xCC, 0xBB, 0xAA, 0xDD]
        );
        Ok(())
    }

    #[test]
    fn removing_dynamic_texture_drops_its_pending_uploads() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let key = dynamic_texture_key(4);
        let size = texture_size(2, 2);
        let bytes = vec![0; 2 * 2 * 4];
        let mut build = || Ok(Some((size, Cow::Borrowed(bytes.as_slice()))));
        let tile = atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("dynamic texture should be allocated");

        atlas.remove(&key);

        let lock = atlas.0.lock();
        assert!(
            lock.pending_uploads
                .iter()
                .all(|upload| upload.id != tile.texture_id)
        );
        assert!(lock.storage.get(tile.texture_id).is_none());
        Ok(())
    }

    #[test]
    fn clearing_atlas_advances_resource_generation_and_drops_uploads() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let key = dynamic_texture_key(5);
        let size = texture_size(1, 1);
        let bytes = vec![0; 4];
        let mut build = || Ok(Some((size, Cow::Borrowed(bytes.as_slice()))));
        atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("dynamic texture should be allocated");

        assert_eq!(atlas.resource_generation(), 0);
        atlas.clear();
        assert_eq!(atlas.resource_generation(), 1);
        assert!(atlas.0.lock().pending_uploads.is_empty());
        atlas.clear();
        assert_eq!(atlas.resource_generation(), 2);
        Ok(())
    }

    #[test]
    fn dynamic_texture_update_rejects_invalid_bounds_and_byte_count() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let key = dynamic_texture_key(6);
        let size = texture_size(2, 2);
        let bytes = vec![0; 2 * 2 * 4];
        let mut build = || Ok(Some((size, Cow::Borrowed(bytes.as_slice()))));
        atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("dynamic texture should be allocated");

        let out_of_bounds = atlas.update(&key, texture_bounds(2, 0, 1, 1), &[0; 4]);
        assert!(out_of_bounds.is_err());
        let wrong_byte_count = atlas.update(&key, texture_bounds(0, 0, 1, 1), &[0; 3]);
        assert!(wrong_byte_count.is_err());
        Ok(())
    }

    #[test]
    fn full_updates_coalesce_in_the_pending_queue() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let key = dynamic_texture_key(7);
        let size = texture_size(4, 4);
        let initial = vec![0u8; 4 * 4 * 4];
        let mut build = || Ok(Some((size, Cow::Borrowed(initial.as_slice()))));
        atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("dynamic texture should be allocated");
        atlas.before_frame();

        for value in [1u8, 2, 3] {
            let frame = vec![value; 4 * 4 * 4];
            atlas.update(&key, texture_bounds(0, 0, 4, 4), &frame)?;
        }

        let lock = atlas.0.lock();
        assert_eq!(lock.pending_uploads.len(), 1);
        assert_eq!(lock.pending_upload_bytes, 4 * 4 * 4);
        Ok(())
    }

    #[test]
    fn partial_uploads_flush_when_the_bounded_queue_is_exceeded() -> anyhow::Result<()> {
        let (device, queue) = test_device_and_queue()?;
        let atlas = WgpuAtlas::new(device, queue, wgpu::TextureFormat::Bgra8Unorm);
        let max_pending_bytes = 8;
        atlas.set_max_pending_upload_bytes(max_pending_bytes);
        let key = dynamic_texture_key(8);
        let size = texture_size(4, 4);
        let initial = vec![0u8; 4 * 4 * 4];
        let mut build = || Ok(Some((size, Cow::Borrowed(initial.as_slice()))));
        atlas
            .get_or_insert_with(&key, &mut build)?
            .expect("dynamic texture should be allocated");
        atlas.before_frame();

        for _ in 0..8 {
            atlas.update(&key, texture_bounds(0, 0, 1, 1), &[1, 2, 3, 4])?;
        }

        let lock = atlas.0.lock();
        assert!(lock.pending_upload_bytes <= max_pending_bytes);
        assert!(lock.pending_uploads.len() <= 2);
        Ok(())
    }
}
