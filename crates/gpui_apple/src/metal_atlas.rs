use anyhow::{Context as _, Result};
use collections::FxHashMap;
use derive_more::{Deref, DerefMut};
use etagere::BucketedAtlasAllocator;
use gpui::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTextureList, AtlasTile, Bounds, DevicePixels,
    PlatformAtlas, Point, Size,
};
use metal::Device;
use parking_lot::Mutex;
use std::borrow::Cow;

const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
    width: DevicePixels(1024),
    height: DevicePixels(1024),
};

// Max texture size on all modern Apple GPUs. Anything bigger than that crashes in validateWithDevice.
const MAX_ATLAS_SIZE: Size<DevicePixels> = Size {
    width: DevicePixels(16384),
    height: DevicePixels(16384),
};

pub struct MetalAtlas(Mutex<MetalAtlasState>);

impl MetalAtlas {
    pub(crate) fn new(device: Device, is_apple_gpu: bool) -> Self {
        MetalAtlas(Mutex::new(MetalAtlasState {
            device: AssertSend(device),
            is_apple_gpu,
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
    }

    pub(crate) fn metal_texture(&self, id: AtlasTextureId) -> metal::Texture {
        self.0.lock().texture(id).metal_texture.clone()
    }
}

struct MetalAtlasState {
    device: AssertSend<Device>,
    is_apple_gpu: bool,
    monochrome_textures: AtlasTextureList<MetalAtlasTexture>,
    polychrome_textures: AtlasTextureList<MetalAtlasTexture>,
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
            Ok(Some(*tile))
        } else {
            let Some((size, bytes)) = build()? else {
                return Ok(None);
            };
            let tile = if matches!(key, AtlasKey::DynamicTexture(_)) {
                lock.allocate_dedicated(size, key.texture_kind())
            } else {
                lock.allocate(size, key.texture_kind())
            }
            .context("failed to allocate")?;
            let texture = lock.texture(tile.texture_id);
            texture.upload(tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile);
            Ok(Some(tile))
        }
    }

    fn update(&self, key: &AtlasKey, bounds: Bounds<DevicePixels>, bytes: &[u8]) -> Result<()> {
        let lock = self.0.lock();
        let Some(tile) = lock.tiles_by_key.get(key).copied() else {
            return Ok(());
        };
        let texture = lock.texture(tile.texture_id);
        validate_upload(tile, bounds, bytes, texture.bytes_per_pixel())?;
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
        texture.upload(upload_bounds, bytes);
        Ok(())
    }

    fn resource_generation(&self) -> u64 {
        0
    }

    fn remove(&self, key: &AtlasKey) {
        let mut lock = self.0.lock();
        let Some(tile) = lock.tiles_by_key.remove(key) else {
            return;
        };
        let id = tile.texture_id;

        let textures = match id.kind {
            AtlasTextureKind::Monochrome => &mut lock.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut lock.polychrome_textures,
            AtlasTextureKind::Subpixel => unreachable!(),
        };

        let Some(texture_slot) = textures
            .textures
            .iter_mut()
            .find(|texture| texture.as_ref().is_some_and(|v| v.id == id))
        else {
            return;
        };

        if let Some(mut texture) = texture_slot.take() {
            texture.allocator.deallocate(tile.tile_id.into());
            texture.decrement_ref_count();
            if texture.is_unreferenced() {
                textures.free_list.push(id.index as usize);
            } else {
                *texture_slot = Some(texture);
            }
        }
    }
}

impl MetalAtlasState {
    fn allocate_dedicated(
        &mut self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> Option<AtlasTile> {
        if size.width.0 <= 0
            || size.height.0 <= 0
            || size.width > MAX_ATLAS_SIZE.width
            || size.height > MAX_ATLAS_SIZE.height
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
            let textures = match texture_kind {
                AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
                AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
                AtlasTextureKind::Subpixel => unreachable!(),
            };

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
    ) -> &mut MetalAtlasTexture {
        let size = min_size.min(&MAX_ATLAS_SIZE).max(&DEFAULT_ATLAS_SIZE);
        self.push_texture_with_size(size, kind)
    }

    fn push_texture_with_size(
        &mut self,
        size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> &mut MetalAtlasTexture {
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
            AtlasTextureKind::Subpixel => unreachable!(),
        }
        texture_descriptor.set_pixel_format(pixel_format);
        texture_descriptor.set_usage(usage);
        // Shared memory mode can be used only on Apple GPU families
        // https://developer.apple.com/documentation/metal/mtlresourceoptions/storagemodeshared
        texture_descriptor.set_storage_mode(if self.is_apple_gpu {
            metal::MTLStorageMode::Shared
        } else {
            metal::MTLStorageMode::Managed
        });
        let metal_texture = self.device.new_texture(&texture_descriptor);

        let texture_list = match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Subpixel => unreachable!(),
        };

        let index = texture_list.free_list.pop();

        let atlas_texture = MetalAtlasTexture {
            id: AtlasTextureId {
                index: index.unwrap_or(texture_list.textures.len()) as u32,
                kind,
            },
            allocator: etagere::BucketedAtlasAllocator::new(size_to_etagere(size)),
            metal_texture: AssertSend(metal_texture),
            live_atlas_keys: 0,
        };

        if let Some(ix) = index {
            texture_list.textures[ix] = Some(atlas_texture);
            texture_list.textures.get_mut(ix)
        } else {
            texture_list.textures.push(Some(atlas_texture));
            texture_list.textures.last_mut()
        }
        .unwrap()
        .as_mut()
        .unwrap()
    }

    fn texture(&self, id: AtlasTextureId) -> &MetalAtlasTexture {
        let textures = match id.kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures,
            AtlasTextureKind::Polychrome => &self.polychrome_textures,
            AtlasTextureKind::Subpixel => unreachable!(),
        };
        textures[id.index as usize].as_ref().unwrap()
    }
}

fn validate_upload(
    tile: AtlasTile,
    bounds: Bounds<DevicePixels>,
    bytes: &[u8],
    bytes_per_pixel: u8,
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
    let expected_len = usize::try_from(bounds.size.width.0)
        .ok()
        .and_then(|width| {
            usize::try_from(bounds.size.height.0)
                .ok()
                .and_then(|height| width.checked_mul(height))
        })
        .and_then(|pixels| pixels.checked_mul(bytes_per_pixel as usize))
        .context("texture update byte size overflow")?;
    anyhow::ensure!(
        bytes.len() == expected_len,
        "texture update contains {} bytes, expected {expected_len}",
        bytes.len()
    );
    Ok(())
}

struct MetalAtlasTexture {
    id: AtlasTextureId,
    allocator: BucketedAtlasAllocator,
    metal_texture: AssertSend<metal::Texture>,
    live_atlas_keys: u32,
}

impl MetalAtlasTexture {
    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(size_to_etagere(size))?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            bounds: Bounds {
                origin: point_from_etagere(allocation.rectangle.min),
                size,
            },
            padding: 0,
        };
        self.live_atlas_keys += 1;
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

    fn decrement_ref_count(&mut self) {
        self.live_atlas_keys -= 1;
    }

    fn is_unreferenced(&mut self) -> bool {
        self.live_atlas_keys == 0
    }
}

fn size_to_etagere(size: Size<DevicePixels>) -> etagere::Size {
    etagere::Size::new(size.width.into(), size.height.into())
}

fn point_from_etagere(value: etagere::Point) -> Point<DevicePixels> {
    Point {
        x: DevicePixels::from(value.x),
        y: DevicePixels::from(value.y),
    }
}

#[derive(Deref, DerefMut)]
struct AssertSend<T>(T);

unsafe impl<T> Send for AssertSend<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{DynamicTextureId, DynamicTextureParams, PlatformAtlas};
    use std::borrow::Cow;

    fn create_atlas() -> Option<MetalAtlas> {
        let device = metal::Device::system_default()?;
        Some(MetalAtlas::new(device, true))
    }

    fn make_dynamic_texture_key(texture_id: usize) -> AtlasKey {
        AtlasKey::DynamicTexture(DynamicTextureParams {
            texture_id: DynamicTextureId(texture_id),
        })
    }

    fn make_image_key(image_id: usize, frame_index: usize) -> AtlasKey {
        AtlasKey::Image(gpui::RenderImageParams {
            image_id: gpui::ImageId(image_id),
            frame_index,
        })
    }

    fn insert_tile(atlas: &MetalAtlas, key: &AtlasKey, size: Size<DevicePixels>) -> AtlasTile {
        atlas
            .get_or_insert_with(key, &mut || {
                let byte_count = (size.width.0 as usize) * (size.height.0 as usize) * 4;
                Ok(Some((size, Cow::Owned(vec![0u8; byte_count]))))
            })
            .expect("allocation should succeed")
            .expect("callback returns Some")
    }

    #[test]
    fn test_remove_clears_stale_keys_from_tiles_by_key() {
        let Some(atlas) = create_atlas() else {
            return;
        };

        let small = Size {
            width: DevicePixels(64),
            height: DevicePixels(64),
        };

        let key_a = make_image_key(1, 0);
        let key_b = make_image_key(2, 0);
        let key_c = make_image_key(3, 0);

        let tile_a = insert_tile(&atlas, &key_a, small);
        let tile_b = insert_tile(&atlas, &key_b, small);
        let tile_c = insert_tile(&atlas, &key_c, small);

        assert_eq!(tile_a.texture_id, tile_b.texture_id);
        assert_eq!(tile_b.texture_id, tile_c.texture_id);

        // Remove A: texture still has B and C, so it stays.
        // The key for A must be removed from tiles_by_key.
        atlas.remove(&key_a);

        // Remove B: texture still has C.
        atlas.remove(&key_b);

        // Remove C: texture becomes unreferenced and is deleted.
        atlas.remove(&key_c);

        // Re-inserting A must allocate a fresh tile on a new texture,
        // NOT return a stale tile referencing the deleted texture.
        let tile_a2 = insert_tile(&atlas, &key_a, small);

        // The texture must actually exist — this would panic before the fix.
        let _texture = atlas.metal_texture(tile_a2.texture_id);
    }

    #[test]
    fn test_remove_deallocates_tile_space_for_reuse() {
        let Some(atlas) = create_atlas() else {
            return;
        };

        let small = Size {
            width: DevicePixels(64),
            height: DevicePixels(64),
        };
        let big = Size {
            width: DevicePixels(700),
            height: DevicePixels(700),
        };

        let keeper_key = make_image_key(1, 0);
        let big_key_a = make_image_key(2, 0);
        let big_key_b = make_image_key(3, 0);

        let keeper_tile = insert_tile(&atlas, &keeper_key, small);
        let tile_a = insert_tile(&atlas, &big_key_a, big);
        assert_eq!(keeper_tile.texture_id, tile_a.texture_id);

        atlas.remove(&big_key_a);
        let tile_b = insert_tile(&atlas, &big_key_b, big);
        assert_eq!(tile_b.texture_id, keeper_tile.texture_id);
    }

    #[test]
    fn test_remove_nonexistent_key_is_noop() {
        let Some(atlas) = create_atlas() else {
            return;
        };
        let key = make_image_key(999, 0);
        atlas.remove(&key);
    }

    #[test]
    fn dynamic_textures_use_exact_sized_dedicated_metal_textures() {
        let Some(atlas) = create_atlas() else {
            return;
        };
        let size = Size {
            width: DevicePixels(64),
            height: DevicePixels(32),
        };

        let first = insert_tile(&atlas, &make_dynamic_texture_key(1), size);
        let second = insert_tile(&atlas, &make_dynamic_texture_key(2), size);

        assert_ne!(first.texture_id, second.texture_id);
        let first_texture = atlas.metal_texture(first.texture_id);
        assert_eq!(first_texture.width(), 64);
        assert_eq!(first_texture.height(), 32);
        assert_eq!(atlas.resource_generation(), 0);
    }

    #[test]
    fn dynamic_texture_updates_upload_only_the_requested_region() {
        let Some(atlas) = create_atlas() else {
            return;
        };
        let size = Size {
            width: DevicePixels(2),
            height: DevicePixels(2),
        };
        let key = make_dynamic_texture_key(3);
        let tile = insert_tile(&atlas, &key, size);
        let dirty_pixel = [1, 2, 3, 4];

        atlas
            .update(
                &key,
                Bounds {
                    origin: Point::new(DevicePixels(1), DevicePixels(1)),
                    size: Size {
                        width: DevicePixels(1),
                        height: DevicePixels(1),
                    },
                },
                &dirty_pixel,
            )
            .expect("partial upload should succeed");

        let texture = atlas.metal_texture(tile.texture_id);
        let mut uploaded = [0u8; 16];
        texture.get_bytes(
            uploaded.as_mut_ptr().cast(),
            8,
            metal::MTLRegion::new_2d(0, 0, 2, 2),
            0,
        );
        assert_eq!(&uploaded[..12], &[0; 12]);
        assert_eq!(&uploaded[12..], &dirty_pixel);
    }

    #[test]
    fn dynamic_texture_updates_reject_invalid_regions_and_payloads() {
        let tile = AtlasTile {
            texture_id: AtlasTextureId {
                index: 0,
                kind: AtlasTextureKind::Polychrome,
            },
            tile_id: gpui::TileId(0),
            bounds: Bounds {
                origin: Point::new(DevicePixels(0), DevicePixels(0)),
                size: Size {
                    width: DevicePixels(2),
                    height: DevicePixels(2),
                },
            },
            padding: 0,
        };

        assert!(
            validate_upload(
                tile,
                Bounds {
                    origin: Point::new(DevicePixels(-1), DevicePixels(0)),
                    size: Size {
                        width: DevicePixels(1),
                        height: DevicePixels(1),
                    },
                },
                &[0; 4],
                4,
            )
            .is_err()
        );
        assert!(
            validate_upload(
                tile,
                Bounds {
                    origin: Point::new(DevicePixels(1), DevicePixels(1)),
                    size: Size {
                        width: DevicePixels(2),
                        height: DevicePixels(1),
                    },
                },
                &[0; 8],
                4,
            )
            .is_err()
        );
        assert!(
            validate_upload(
                tile,
                Bounds {
                    origin: Point::new(DevicePixels(0), DevicePixels(0)),
                    size: Size {
                        width: DevicePixels(1),
                        height: DevicePixels(1),
                    },
                },
                &[0; 3],
                4,
            )
            .is_err()
        );
    }
}
