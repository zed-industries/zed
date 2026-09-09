use std::borrow::Cow;

use collections::FxHashMap;
use etagere::BucketedAtlasAllocator;
use gpui::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTextureList, AtlasTile, Bounds, DevicePixels,
    PlatformAtlas, Point, Size,
};
use parking_lot::{Mutex, MutexGuard};

pub struct SoftwareAtlas(Mutex<SoftwareAtlasState>);

pub(crate) struct SoftwareAtlasState {
    monochrome_textures: AtlasTextureList<SoftwareAtlasTexture>,
    polychrome_textures: AtlasTextureList<SoftwareAtlasTexture>,
    subpixel_textures: AtlasTextureList<SoftwareAtlasTexture>,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
    next_generation: u64,
}

pub(crate) struct SoftwareAtlasTexture {
    id: AtlasTextureId,
    size: Size<DevicePixels>,
    bytes_per_pixel: usize,
    allocator: BucketedAtlasAllocator,
    pixels: Vec<u8>,
    live_atlas_keys: u32,
    generation: u64,
}

impl SoftwareAtlas {
    pub fn new() -> Self {
        Self(Mutex::new(SoftwareAtlasState {
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            subpixel_textures: Default::default(),
            tiles_by_key: Default::default(),
            next_generation: 1,
        }))
    }

    pub(crate) fn lock(&self) -> MutexGuard<'_, SoftwareAtlasState> {
        self.0.lock()
    }
}

impl Default for SoftwareAtlas {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAtlas for SoftwareAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<Option<(Size<DevicePixels>, Cow<'a, [u8]>)>>,
    ) -> anyhow::Result<Option<AtlasTile>> {
        let mut state = self.0.lock();
        if let Some(tile) = state.tiles_by_key.get(key) {
            return Ok(Some(*tile));
        }

        let Some((size, bytes)) = build()? else {
            return Ok(None);
        };
        let tile = state
            .allocate(size, key.texture_kind())
            .ok_or_else(|| anyhow::anyhow!("failed to allocate software atlas tile"))?;
        let upload_result = match state.texture_mut(tile.texture_id) {
            Some(texture) => texture.upload(tile.bounds, &bytes),
            None => Err(anyhow::anyhow!(
                "software atlas allocated a missing texture"
            )),
        };
        if let Err(error) = upload_result {
            state.deallocate(tile);
            return Err(error);
        }
        state.tiles_by_key.insert(key.clone(), tile);
        Ok(Some(tile))
    }

    fn remove(&self, key: &AtlasKey) {
        let mut state = self.0.lock();
        let Some(tile) = state.tiles_by_key.remove(key) else {
            return;
        };
        state.deallocate(tile);
    }
}

impl SoftwareAtlasState {
    fn deallocate(&mut self, tile: AtlasTile) {
        let generation = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1);
        let textures = self.textures_mut(tile.texture_id.kind);
        let Some(texture_slot) = textures.textures.get_mut(tile.texture_id.index as usize) else {
            return;
        };
        let Some(mut texture) = texture_slot.take() else {
            return;
        };
        texture.generation = generation;
        texture.allocator.deallocate(tile.tile_id.into());
        texture.live_atlas_keys = texture.live_atlas_keys.saturating_sub(1);
        if texture.live_atlas_keys == 0 {
            textures.free_list.push(tile.texture_id.index as usize);
        } else {
            *texture_slot = Some(texture);
        }
    }

    fn textures_mut(
        &mut self,
        kind: AtlasTextureKind,
    ) -> &mut AtlasTextureList<SoftwareAtlasTexture> {
        match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Subpixel => &mut self.subpixel_textures,
        }
    }

    fn textures(&self, kind: AtlasTextureKind) -> &AtlasTextureList<SoftwareAtlasTexture> {
        match kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures,
            AtlasTextureKind::Polychrome => &self.polychrome_textures,
            AtlasTextureKind::Subpixel => &self.subpixel_textures,
        }
    }

    fn allocate(&mut self, size: Size<DevicePixels>, kind: AtlasTextureKind) -> Option<AtlasTile> {
        if size.width.0 <= 0 || size.height.0 <= 0 || size.width.0 > 16384 || size.height.0 > 16384
        {
            return None;
        }
        if let Some(tile) = self
            .textures_mut(kind)
            .iter_mut()
            .rev()
            .find_map(|texture| texture.allocate(size))
        {
            return Some(tile);
        }
        self.push_texture(size, kind)?.allocate(size)
    }

    fn push_texture(
        &mut self,
        minimum_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> Option<&mut SoftwareAtlasTexture> {
        let size = Size {
            width: DevicePixels(minimum_size.width.0.max(1024)),
            height: DevicePixels(minimum_size.height.0.max(1024)),
        };
        let bytes_per_pixel = match kind {
            AtlasTextureKind::Monochrome => 1,
            AtlasTextureKind::Polychrome | AtlasTextureKind::Subpixel => 4,
        };
        let generation = self.next_generation;
        self.next_generation = self.next_generation.wrapping_add(1);
        let textures = self.textures_mut(kind);
        let index = textures.free_list.pop();
        let id = AtlasTextureId {
            index: index.unwrap_or(textures.textures.len()) as u32,
            kind,
        };
        let texture = SoftwareAtlasTexture {
            id,
            size,
            bytes_per_pixel,
            allocator: BucketedAtlasAllocator::new(device_size_to_etagere(size)),
            pixels: vec![0; size.width.0 as usize * size.height.0 as usize * bytes_per_pixel],
            live_atlas_keys: 0,
            generation,
        };
        if let Some(index) = index {
            textures.textures[index] = Some(texture);
            textures.textures.get_mut(index)?.as_mut()
        } else {
            textures.textures.push(Some(texture));
            textures.textures.last_mut()?.as_mut()
        }
    }

    fn texture_mut(&mut self, id: AtlasTextureId) -> Option<&mut SoftwareAtlasTexture> {
        self.textures_mut(id.kind)
            .textures
            .get_mut(id.index as usize)?
            .as_mut()
    }

    pub(crate) fn texture_generation(&self, id: AtlasTextureId) -> Option<u64> {
        self.texture(id).map(|texture| texture.generation)
    }

    pub(crate) fn texture_for_tile(
        &self,
        tile: AtlasTile,
        kind: AtlasTextureKind,
    ) -> Option<&SoftwareAtlasTexture> {
        if tile.texture_id.kind != kind {
            return None;
        }
        let texture = self.texture(tile.texture_id)?;
        let origin = tile.bounds.origin;
        let size = tile.bounds.size;
        if origin.x.0 < 0 || origin.y.0 < 0 || size.width.0 <= 0 || size.height.0 <= 0 {
            return None;
        }
        if origin.x.0.checked_add(size.width.0)? > texture.size.width.0
            || origin.y.0.checked_add(size.height.0)? > texture.size.height.0
        {
            return None;
        }
        Some(texture)
    }

    pub(crate) fn texture(&self, id: AtlasTextureId) -> Option<&SoftwareAtlasTexture> {
        self.textures(id.kind)
            .textures
            .get(id.index as usize)?
            .as_ref()
    }
}

impl SoftwareAtlasTexture {
    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(device_size_to_etagere(size))?;
        self.live_atlas_keys += 1;
        Some(AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            padding: 0,
            bounds: Bounds {
                origin: Point {
                    x: DevicePixels(allocation.rectangle.min.x),
                    y: DevicePixels(allocation.rectangle.min.y),
                },
                size,
            },
        })
    }

    fn upload(&mut self, bounds: Bounds<DevicePixels>, bytes: &[u8]) -> anyhow::Result<()> {
        let width = bounds.size.width.0 as usize;
        let height = bounds.size.height.0 as usize;
        let row_bytes = width * self.bytes_per_pixel;
        let expected = row_bytes * height;
        anyhow::ensure!(
            bytes.len() >= expected,
            "software atlas upload has {} bytes, expected at least {expected}",
            bytes.len()
        );
        let texture_stride = self.size.width.0 as usize * self.bytes_per_pixel;
        let destination_x = bounds.origin.x.0 as usize * self.bytes_per_pixel;
        let destination_y = bounds.origin.y.0 as usize;
        for row in 0..height {
            let source_start = row * row_bytes;
            let destination_start = (destination_y + row) * texture_stride + destination_x;
            self.pixels[destination_start..destination_start + row_bytes]
                .copy_from_slice(&bytes[source_start..source_start + row_bytes]);
        }
        Ok(())
    }

    pub(crate) fn size(&self) -> Size<DevicePixels> {
        self.size
    }

    pub(crate) fn bytes_per_pixel(&self) -> usize {
        self.bytes_per_pixel
    }

    pub(crate) fn pixels(&self) -> &[u8] {
        &self.pixels
    }
}

fn device_size_to_etagere(size: Size<DevicePixels>) -> etagere::Size {
    etagere::Size::new(size.width.0, size.height.0)
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use gpui::{ImageId, RenderImageParams};

    use super::*;

    fn image_key(id: usize) -> AtlasKey {
        AtlasKey::Image(RenderImageParams {
            image_id: ImageId(id),
            frame_index: 0,
        })
    }

    #[test]
    fn stores_uploaded_pixels_and_reuses_freed_space() {
        let atlas = SoftwareAtlas::new();
        let size = Size {
            width: DevicePixels(2),
            height: DevicePixels(2),
        };
        let first = atlas
            .get_or_insert_with(&image_key(1), &mut || {
                Ok(Some((size, Cow::Borrowed(&[1u8; 16]))))
            })
            .expect("atlas insertion failed")
            .expect("builder returned a tile");
        let state = atlas.lock();
        let texture = state
            .texture(first.texture_id)
            .expect("texture was retired early");
        let stride = texture.size.width.0 as usize * 4;
        let offset =
            first.bounds.origin.y.0 as usize * stride + first.bounds.origin.x.0 as usize * 4;
        assert_eq!(&texture.pixels[offset..offset + 8], &[1; 8]);
        drop(state);

        atlas.remove(&image_key(1));
        let second = atlas
            .get_or_insert_with(&image_key(2), &mut || {
                Ok(Some((size, Cow::Borrowed(&[2u8; 16]))))
            })
            .expect("atlas insertion failed")
            .expect("builder returned a tile");
        assert_eq!(first.texture_id, second.texture_id);
    }
    #[test]
    fn rejects_invalid_atlas_tiles() {
        let atlas = SoftwareAtlas::new();
        let size = Size {
            width: DevicePixels(8),
            height: DevicePixels(2),
        };
        let tile = atlas
            .get_or_insert_with(&image_key(1), &mut || {
                Ok(Some((size, Cow::Borrowed(&[255; 64]))))
            })
            .expect("insert")
            .expect("tile");
        let state = atlas.lock();
        assert!(
            state
                .texture_for_tile(tile, AtlasTextureKind::Polychrome)
                .is_some()
        );
        assert!(
            state
                .texture_for_tile(tile, AtlasTextureKind::Monochrome)
                .is_none()
        );
        for (x, y, width, height) in [
            (-1, 0, 8, 2),
            (0, -1, 8, 2),
            (0, 0, 0, 2),
            (0, 0, 8, -1),
            (1020, 0, 8, 2),
            (0, 1023, 8, 2),
            (i32::MAX, 0, 8, 2),
            (0, i32::MAX, 8, 2),
        ] {
            let mut invalid = tile;
            invalid.bounds = Bounds::new(
                Point::new(DevicePixels(x), DevicePixels(y)),
                Size {
                    width: DevicePixels(width),
                    height: DevicePixels(height),
                },
            );
            assert!(
                state
                    .texture_for_tile(invalid, AtlasTextureKind::Polychrome)
                    .is_none()
            );
        }
    }
}
