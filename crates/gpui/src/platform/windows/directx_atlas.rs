use collections::FxHashMap;
use etagere::BucketedAtlasAllocator;
use parking_lot::Mutex;
use windows::Win32::Graphics::{
    Direct3D11::{
        ID3D11Device, ID3D11DeviceContext, ID3D11RenderTargetView, ID3D11ShaderResourceView,
        ID3D11Texture2D, D3D11_BIND_RENDER_TARGET, D3D11_BIND_SHADER_RESOURCE, D3D11_BOX,
        D3D11_CPU_ACCESS_WRITE, D3D11_MAP_WRITE, D3D11_MAP_WRITE_DISCARD,
        D3D11_MAP_WRITE_NO_OVERWRITE, D3D11_TEXTURE2D_DESC, D3D11_USAGE_DEFAULT,
        D3D11_USAGE_DYNAMIC,
    },
    Dxgi::Common::{
        DXGI_FORMAT_A8_UNORM, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_R16_FLOAT, DXGI_SAMPLE_DESC,
    },
};

use crate::*;

pub(crate) struct DirectXAtlas(Mutex<DirectXAtlasState>);

struct DirectXAtlasState {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    monochrome_textures: Vec<DirectXAtlasTexture>,
    polychrome_textures: Vec<DirectXAtlasTexture>,
    path_textures: Vec<DirectXAtlasTexture>,
    tiles_by_key: FxHashMap<AtlasKey, AtlasTile>,
}

struct DirectXAtlasTexture {
    id: AtlasTextureId,
    bytes_per_pixel: u32,
    allocator: BucketedAtlasAllocator,
    texture: ID3D11Texture2D,
    rtv: [Option<ID3D11RenderTargetView>; 1],
    view: [Option<ID3D11ShaderResourceView>; 1],
}

impl DirectXAtlas {
    pub(crate) fn new(device: ID3D11Device, device_context: ID3D11DeviceContext) -> Self {
        DirectXAtlas(Mutex::new(DirectXAtlasState {
            device,
            device_context,
            monochrome_textures: Default::default(),
            polychrome_textures: Default::default(),
            path_textures: Default::default(),
            tiles_by_key: Default::default(),
        }))
    }

    pub(crate) fn texture_info(
        &self,
        id: AtlasTextureId,
    ) -> (
        Size<f32>,
        [Option<ID3D11RenderTargetView>; 1],
        [Option<ID3D11ShaderResourceView>; 1],
    ) {
        let lock = self.0.lock();
        let tex = lock.texture(id);
        let size = tex.allocator.size();
        (
            Size {
                width: size.width as f32,
                height: size.height as f32,
            },
            tex.rtv.clone(),
            tex.view.clone(),
        )
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
            Ok(Some(tile.clone()))
        } else {
            let Some((size, bytes)) = build()? else {
                return Ok(None);
            };
            let tile = lock
                .allocate(size, key.texture_kind())
                .ok_or_else(|| anyhow::anyhow!("failed to allocate"))?;
            let texture = lock.texture(tile.texture_id);
            texture.upload(&lock.device_context, tile.bounds, &bytes);
            lock.tiles_by_key.insert(key.clone(), tile.clone());
            Ok(Some(tile))
        }
    }
}

impl DirectXAtlasState {
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
    ) -> &mut DirectXAtlasTexture {
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
        let pixel_format;
        let bind_flag;
        let bytes_per_pixel;
        match kind {
            AtlasTextureKind::Monochrome => {
                pixel_format = DXGI_FORMAT_A8_UNORM;
                bind_flag = D3D11_BIND_SHADER_RESOURCE;
                bytes_per_pixel = 1;
            }
            AtlasTextureKind::Polychrome => {
                pixel_format = DXGI_FORMAT_B8G8R8A8_UNORM;
                bind_flag = D3D11_BIND_SHADER_RESOURCE;
                bytes_per_pixel = 4;
            }
            AtlasTextureKind::Path => {
                pixel_format = DXGI_FORMAT_R16_FLOAT;
                bind_flag = D3D11_BIND_SHADER_RESOURCE | D3D11_BIND_RENDER_TARGET;
                bytes_per_pixel = 2;
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
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: 0,
        };
        let mut texture: Option<ID3D11Texture2D> = None;
        unsafe {
            self.device
                .CreateTexture2D(&texture_desc, None, Some(&mut texture))
                .unwrap();
        }
        let texture = texture.unwrap();

        let textures = match kind {
            AtlasTextureKind::Monochrome => &mut self.monochrome_textures,
            AtlasTextureKind::Polychrome => &mut self.polychrome_textures,
            AtlasTextureKind::Path => &mut self.path_textures,
        };
        let rtv = match kind {
            AtlasTextureKind::Path => unsafe {
                let mut view: Option<ID3D11RenderTargetView> = None;
                self.device
                    .CreateRenderTargetView(&texture, None, Some(&mut view))
                    .unwrap();
                [view]
            },
            _ => [None],
        };
        let view = unsafe {
            let mut view = None;
            self.device
                .CreateShaderResourceView(&texture, None, Some(&mut view))
                .unwrap();
            [view]
        };
        let atlas_texture = DirectXAtlasTexture {
            id: AtlasTextureId {
                index: textures.len() as u32,
                kind,
            },
            bytes_per_pixel,
            allocator: etagere::BucketedAtlasAllocator::new(size.into()),
            texture,
            rtv,
            view,
        };
        textures.push(atlas_texture);
        textures.last_mut().unwrap()
    }

    fn texture(&self, id: AtlasTextureId) -> &DirectXAtlasTexture {
        let textures = match id.kind {
            crate::AtlasTextureKind::Monochrome => &self.monochrome_textures,
            crate::AtlasTextureKind::Polychrome => &self.polychrome_textures,
            crate::AtlasTextureKind::Path => &self.path_textures,
        };
        &textures[id.index as usize]
    }
}

impl DirectXAtlasTexture {
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

    fn upload(
        &self,
        device_context: &ID3D11DeviceContext,
        bounds: Bounds<DevicePixels>,
        bytes: &[u8],
    ) {
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
}

// impl From<Size<DevicePixels>> for etagere::Size {
//     fn from(size: Size<DevicePixels>) -> Self {
//         etagere::Size::new(size.width.into(), size.height.into())
//     }
// }

// impl From<etagere::Point> for Point<DevicePixels> {
//     fn from(value: etagere::Point) -> Self {
//         Point {
//             x: DevicePixels::from(value.x),
//             y: DevicePixels::from(value.y),
//         }
//     }
// }

// impl From<etagere::Size> for Size<DevicePixels> {
//     fn from(size: etagere::Size) -> Self {
//         Size {
//             width: DevicePixels::from(size.width),
//             height: DevicePixels::from(size.height),
//         }
//     }
// }

// impl From<etagere::Rectangle> for Bounds<DevicePixels> {
//     fn from(rectangle: etagere::Rectangle) -> Self {
//         Bounds {
//             origin: rectangle.min.into(),
//             size: rectangle.size().into(),
//         }
//     }
// }
