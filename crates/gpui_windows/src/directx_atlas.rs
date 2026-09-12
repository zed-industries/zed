use collections::FxHashMap;
use etagere::BucketedAtlasAllocator;
use parking_lot::Mutex;
use windows::Win32::Graphics::{
    Direct3D11::{
        D3D11_BIND_SHADER_RESOURCE, D3D11_BOX, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
        ID3D11Device, ID3D11DeviceContext, ID3D11ShaderResourceView, ID3D11Texture2D,
    },
    Dxgi::Common::*,
};

use gpui::{
    AtlasKey, AtlasTextureId, AtlasTextureKind, AtlasTextureList, AtlasTile, Bounds, DevicePixels,
    PlatformAtlas, Point, Size,
};

pub(crate) struct DirectXAtlas(Mutex<DirectXAtlasState>);

struct DirectXAtlasState {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    resource_generation: ResourceGeneration,
    monochrome_textures: AtlasTextureList<DirectXAtlasTexture>,
    polychrome_textures: AtlasTextureList<DirectXAtlasTexture>,
    subpixel_textures: AtlasTextureList<DirectXAtlasTexture>,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
}

#[derive(Default)]
struct ResourceGeneration(u64);

impl ResourceGeneration {
    fn current(&self) -> u64 {
        self.0
    }

    fn advance(&mut self) {
        self.0 = self
            .0
            .checked_add(1)
            .expect("DirectX atlas resource generation exhausted");
    }
}

struct DirectXAtlasTexture {
    id: AtlasTextureId,
    bytes_per_pixel: u32,
    allocator: BucketedAtlasAllocator,
    texture: ID3D11Texture2D,
    view: [Option<ID3D11ShaderResourceView>; 1],
    live_atlas_keys: u32,
}

impl DirectXAtlas {
    pub(crate) fn new(device: &ID3D11Device, device_context: &ID3D11DeviceContext) -> Self {
        DirectXAtlas(Mutex::new(DirectXAtlasState {
            device: device.clone(),
            device_context: device_context.clone(),
            resource_generation: Default::default(),
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            subpixel_textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
    }

    pub(crate) fn get_texture_view(
        &self,
        id: AtlasTextureId,
    ) -> [Option<ID3D11ShaderResourceView>; 1] {
        let lock = self.0.lock();
        let tex = lock.texture(id);
        tex.view.clone()
    }

    pub(crate) fn handle_device_lost(
        &self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
    ) {
        let mut lock = self.0.lock();
        lock.device = device.clone();
        lock.device_context = device_context.clone();
        lock.reset_resources();
    }
}

impl PlatformAtlas for DirectXAtlas {
    fn get_or_insert_with<'a>(
        &self,
        key: &AtlasKey,
        build: &mut dyn FnMut() -> anyhow::Result<
            Option<(Size<DevicePixels>, std::borrow::Cow<'a, [u8]>)>,
        >,
    ) -> anyhow::Result<Option<AtlasTile>> {
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
            .ok_or_else(|| anyhow::anyhow!("failed to allocate"))?;
            let texture = lock.texture(tile.texture_id);
            texture.upload(&lock.device_context, tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile);
            Ok(Some(tile))
        }
    }

    fn update(
        &self,
        key: &AtlasKey,
        bounds: Bounds<DevicePixels>,
        bytes: &[u8],
    ) -> anyhow::Result<()> {
        let lock = self.0.lock();
        let Some(tile) = lock.tiles_by_key.get(key).copied() else {
            return Ok(());
        };
        let texture = lock.texture(tile.texture_id);
        validate_upload(tile, bounds, bytes, texture.bytes_per_pixel)?;
        let upload_bounds = Bounds {
            origin: Point {
                x: DevicePixels(
                    tile.bounds
                        .origin
                        .x
                        .0
                        .checked_add(bounds.origin.x.0)
                        .ok_or_else(|| {
                            anyhow::anyhow!("texture update horizontal origin overflow")
                        })?,
                ),
                y: DevicePixels(
                    tile.bounds
                        .origin
                        .y
                        .0
                        .checked_add(bounds.origin.y.0)
                        .ok_or_else(|| {
                            anyhow::anyhow!("texture update vertical origin overflow")
                        })?,
                ),
            },
            size: bounds.size,
        };
        texture.upload(&lock.device_context, upload_bounds, bytes);
        Ok(())
    }

    fn resource_generation(&self) -> u64 {
        self.0.lock().resource_generation.current()
    }

    fn max_texture_size(&self) -> Option<Size<DevicePixels>> {
        // D3D11 feature level 11 guarantees 16384x16384 2D textures.
        const MAX: i32 = 16384;
        Some(Size {
            width: DevicePixels(MAX),
            height: DevicePixels(MAX),
        })
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
            AtlasTextureKind::Subpixel => &mut lock.subpixel_textures,
        };

        let Some(texture_slot) = textures.textures.get_mut(id.index as usize) else {
            return;
        };

        if let Some(mut texture) = texture_slot.take() {
            texture.allocator.deallocate(tile.tile_id.into());
            texture.decrement_ref_count();
            if texture.is_unreferenced() {
                textures.free_list.push(texture.id.index as usize);
            } else {
                *texture_slot = Some(texture);
            }
        }
    }
}

impl DirectXAtlasState {
    fn reset_resources(&mut self) {
        self.monochrome_textures = AtlasTextureList::default();
        self.polychrome_textures = AtlasTextureList::default();
        self.subpixel_textures = AtlasTextureList::default();
        self.tiles_by_key.clear();
        self.resource_generation.advance();
    }

    fn allocate_dedicated(
        &mut self,
        size: Size<DevicePixels>,
        texture_kind: AtlasTextureKind,
    ) -> Option<AtlasTile> {
        const MAX_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(16384),
            height: DevicePixels(16384),
        };
        if size.width.0 <= 0
            || size.height.0 <= 0
            || size.width > MAX_ATLAS_SIZE.width
            || size.height > MAX_ATLAS_SIZE.height
        {
            return None;
        }

        let texture = self.push_texture_with_size(size, texture_kind)?;
        texture.allocate(size)
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
                AtlasTextureKind::Subpixel => &mut self.subpixel_textures,
            };

            if let Some(tile) = textures
                .iter_mut()
                .rev()
                .find_map(|texture| texture.allocate(size))
            {
                return Some(tile);
            }
        }

        let texture = self.push_texture(size, texture_kind)?;
        texture.allocate(size)
    }

    fn push_texture(
        &mut self,
        min_size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> Option<&mut DirectXAtlasTexture> {
        const DEFAULT_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(1024),
            height: DevicePixels(1024),
        };
        // Max texture size for DirectX. See:
        // https://learn.microsoft.com/en-us/windows/win32/direct3d11/overviews-direct3d-11-resources-limits
        const MAX_ATLAS_SIZE: Size<DevicePixels> = Size {
            width: DevicePixels(16384),
            height: DevicePixels(16384),
        };
        let size = min_size.min(&MAX_ATLAS_SIZE).max(&DEFAULT_ATLAS_SIZE);
        self.push_texture_with_size(size, kind)
    }

    fn push_texture_with_size(
        &mut self,
        size: Size<DevicePixels>,
        kind: AtlasTextureKind,
    ) -> Option<&mut DirectXAtlasTexture> {
        let pixel_format;
        let bind_flag;
        let bytes_per_pixel;
        match kind {
            AtlasTextureKind::Monochrome => {
                pixel_format = DXGI_FORMAT_R8_UNORM;
                bind_flag = D3D11_BIND_SHADER_RESOURCE;
                bytes_per_pixel = 1;
            }
            AtlasTextureKind::Polychrome => {
                pixel_format = DXGI_FORMAT_B8G8R8A8_UNORM;
                bind_flag = D3D11_BIND_SHADER_RESOURCE;
                bytes_per_pixel = 4;
            }
            AtlasTextureKind::Subpixel => {
                pixel_format = DXGI_FORMAT_R8G8B8A8_UNORM;
                bind_flag = D3D11_BIND_SHADER_RESOURCE;
                bytes_per_pixel = 4;
            }
        }
        let texture_desc = D3D11_TEXTURE2D_DESC {
            Width: size.width.0 as u32,
            Height: size.height.0 as u32,
            MipLevels: 1,
            ArraySize: 1,
            Format: pixel_format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: bind_flag.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        let mut texture: Option<ID3D11Texture2D> = None;
        unsafe {
            // This only returns None if the device is lost, which we will recreate later.
            // So it's ok to return None here.
            self.device
                .CreateTexture2D(&texture_desc, None, Some(&mut texture))
                .ok()?;
        }
        let texture = texture.unwrap();

        let texture_list = match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Subpixel => &mut self.subpixel_textures,
        };
        let index = texture_list.free_list.pop();
        let view = unsafe {
            let mut view = None;
            self.device
                .CreateShaderResourceView(&texture, None, Some(&mut view))
                .ok()?;
            [view]
        };
        let atlas_texture = DirectXAtlasTexture {
            id: AtlasTextureId {
                index: index.unwrap_or(texture_list.textures.len()) as u32,
                kind,
            },
            bytes_per_pixel,
            allocator: etagere::BucketedAtlasAllocator::new(device_size_to_etagere(size)),
            texture,
            view,
            live_atlas_keys: 0,
        };
        if let Some(ix) = index {
            texture_list.textures[ix] = Some(atlas_texture);
            texture_list.textures.get_mut(ix).unwrap().as_mut()
        } else {
            texture_list.textures.push(Some(atlas_texture));
            texture_list.textures.last_mut().unwrap().as_mut()
        }
    }

    fn texture(&self, id: AtlasTextureId) -> &DirectXAtlasTexture {
        match id.kind {
            AtlasTextureKind::Monochrome => &self.monochrome_textures[id.index as usize]
                .as_ref()
                .unwrap(),
            AtlasTextureKind::Polychrome => &self.polychrome_textures[id.index as usize]
                .as_ref()
                .unwrap(),
            AtlasTextureKind::Subpixel => {
                &self.subpixel_textures[id.index as usize].as_ref().unwrap()
            }
        }
    }
}

fn validate_upload(
    tile: AtlasTile,
    bounds: Bounds<DevicePixels>,
    bytes: &[u8],
    bytes_per_pixel: u32,
) -> anyhow::Result<()> {
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
        .ok_or_else(|| anyhow::anyhow!("texture update horizontal bounds overflow"))?;
    let bottom = bounds
        .origin
        .y
        .0
        .checked_add(bounds.size.height.0)
        .ok_or_else(|| anyhow::anyhow!("texture update vertical bounds overflow"))?;
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
        .ok_or_else(|| anyhow::anyhow!("texture update byte size overflow"))?;
    anyhow::ensure!(
        bytes.len() == expected_len,
        "texture update contains {} bytes, expected {expected_len}",
        bytes.len()
    );
    Ok(())
}

impl DirectXAtlasTexture {
    fn allocate(&mut self, size: Size<DevicePixels>) -> Option<AtlasTile> {
        let allocation = self.allocator.allocate(device_size_to_etagere(size))?;
        let tile = AtlasTile {
            texture_id: self.id,
            tile_id: allocation.id.into(),
            bounds: Bounds {
                origin: etagere_point_to_device(allocation.rectangle.min),
                size,
            },
            padding: 0,
        };
        self.live_atlas_keys += 1;
        Some(tile)
    }

    fn upload(
        &self,
        device_context: &ID3D11DeviceContext,
        bounds: Bounds<DevicePixels>,
        bytes: &[u8],
    ) {
        // `UpdateSubresource` reads `row_pitch * height` bytes from `bytes` based on the
        // `D3D11_BOX` below. If the caller hands us a slice shorter than that, the driver would
        // over-read past the end of the source buffer (potentially by multiple megabytes), so bail
        // out instead. This is a first-insert path rather than a per-frame one, so the check is
        // effectively free.
        let row_bytes = bounds.size.width.to_bytes(self.bytes_per_pixel as u8) as usize;
        let expected = row_bytes * bounds.size.height.0.max(0) as usize;
        if bytes.len() < expected {
            log::error!(
                "DirectXAtlasTexture::upload: source slice is {} bytes but the {}x{} region \
                 requires {} bytes; skipping upload to avoid a driver over-read",
                bytes.len(),
                bounds.size.width.0,
                bounds.size.height.0,
                expected,
            );
            return;
        }
        unsafe {
            device_context.UpdateSubresource(
                &self.texture,
                0,
                Some(&D3D11_BOX {
                    left: bounds.left().0 as u32,
                    top: bounds.top().0 as u32,
                    front: 0,
                    right: bounds.right().0 as u32,
                    bottom: bounds.bottom().0 as u32,
                    back: 1,
                }),
                bytes.as_ptr() as _,
                bounds.size.width.to_bytes(self.bytes_per_pixel as u8),
                0,
            );
        }
    }

    fn decrement_ref_count(&mut self) {
        self.live_atlas_keys -= 1;
    }

    fn is_unreferenced(&mut self) -> bool {
        self.live_atlas_keys == 0
    }
}

fn device_size_to_etagere(size: Size<DevicePixels>) -> etagere::Size {
    etagere::Size::new(size.width.into(), size.height.into())
}

fn etagere_point_to_device(value: etagere::Point) -> Point<DevicePixels> {
    Point {
        x: DevicePixels::from(value.x),
        y: DevicePixels::from(value.y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{ImageId, RenderImageParams};
    use std::borrow::Cow;
    use windows::Win32::{
        Foundation::HMODULE,
        Graphics::{
            Direct3D::D3D_DRIVER_TYPE_WARP,
            Direct3D11::{D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_SDK_VERSION, D3D11CreateDevice},
        },
    };

    fn create_atlas() -> Option<DirectXAtlas> {
        let mut device: Option<ID3D11Device> = None;
        let mut device_context: Option<ID3D11DeviceContext> = None;
        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE_WARP,
                HMODULE::default(),
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                None,
                D3D11_SDK_VERSION,
                Some(&mut device),
                None,
                Some(&mut device_context),
            )
        }
        .ok()?;
        Some(DirectXAtlas::new(&device?, &device_context?))
    }

    fn make_image_key(image_id: usize) -> AtlasKey {
        AtlasKey::Image(RenderImageParams {
            image_id: ImageId(image_id),
            frame_index: 0,
        })
    }

    fn insert_tile(atlas: &DirectXAtlas, key: &AtlasKey, size: Size<DevicePixels>) -> AtlasTile {
        atlas
            .get_or_insert_with(key, &mut || {
                let byte_count = (size.width.0 as usize) * (size.height.0 as usize) * 4;
                Ok(Some((size, Cow::Owned(vec![0u8; byte_count]))))
            })
            .expect("allocation should succeed")
            .expect("callback returns Some")
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

        let keeper_key = make_image_key(1);
        let big_key_a = make_image_key(2);
        let big_key_b = make_image_key(3);

        let keeper_tile = insert_tile(&atlas, &keeper_key, small);
        let tile_a = insert_tile(&atlas, &big_key_a, big);
        assert_eq!(keeper_tile.texture_id, tile_a.texture_id);

        atlas.remove(&big_key_a);

        let tile_b = insert_tile(&atlas, &big_key_b, big);
        assert_eq!(tile_b.texture_id, keeper_tile.texture_id);
    }

    fn make_dynamic_texture_key(texture_id: usize) -> AtlasKey {
        AtlasKey::DynamicTexture(gpui::DynamicTextureParams {
            texture_id: gpui::DynamicTextureId(texture_id),
        })
    }

    #[test]
    fn resource_generation_advances_once_per_invalidation() {
        let mut generation = ResourceGeneration::default();

        assert_eq!(generation.current(), 0);
        generation.advance();
        assert_eq!(generation.current(), 1);
        generation.advance();
        assert_eq!(generation.current(), 2);
    }

    #[test]
    fn validate_upload_rejects_invalid_regions_and_payloads() {
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
        let unit = |x: i32, y: i32| Bounds {
            origin: Point::new(DevicePixels(x), DevicePixels(y)),
            size: Size {
                width: DevicePixels(1),
                height: DevicePixels(1),
            },
        };

        assert!(validate_upload(tile, unit(-1, 0), &[0; 4], 4).is_err());
        assert!(validate_upload(tile, unit(2, 0), &[0; 4], 4).is_err());
        assert!(validate_upload(tile, unit(0, 2), &[0; 4], 4).is_err());
        assert!(validate_upload(tile, unit(0, 0), &[0; 3], 4).is_err());
        assert!(validate_upload(tile, unit(0, 0), &[0; 4], 4).is_ok());
    }

    #[test]
    fn dynamic_textures_use_exact_sized_dedicated_textures() {
        let Some(atlas) = create_atlas() else {
            return;
        };
        let size = Size {
            width: DevicePixels(37),
            height: DevicePixels(19),
        };
        let key = make_dynamic_texture_key(1);

        let tile = insert_tile(&atlas, &key, size);
        assert_eq!(
            tile.bounds,
            Bounds {
                origin: Point::new(DevicePixels(0), DevicePixels(0)),
                size,
            }
        );

        let lock = atlas.0.lock();
        let texture = lock.texture(tile.texture_id);
        let mut description: D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        unsafe { texture.texture.GetDesc(&mut description) };
        assert_eq!(description.Width, 37);
        assert_eq!(description.Height, 19);
        assert_eq!(lock.resource_generation.current(), 0);

        // A second dynamic texture must not share the first one's backing texture.
        let other = insert_tile(&atlas, &make_dynamic_texture_key(2), size);
        assert_ne!(other.texture_id, tile.texture_id);
    }
}
