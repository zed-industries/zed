use std::{collections::HashMap, hash::BuildHasherDefault, sync::Arc};

use ::util::ResultExt;
use anyhow::Result;
use collections::FxHasher;
use windows::{
    core::*,
    Win32::{
        Foundation::HWND,
        Graphics::{
            Direct3D::{
                Fxc::{D3DCompileFromFile, D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION},
                ID3DBlob, D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
                D3D_PRIMITIVE_TOPOLOGY, D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
                D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            },
            Direct3D11::{
                D3D11CreateDevice, ID3D11BlendState, ID3D11Buffer, ID3D11Device,
                ID3D11DeviceContext, ID3D11PixelShader, ID3D11RenderTargetView, ID3D11SamplerState,
                ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader,
                D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_SHADER_RESOURCE, D3D11_BLEND_DESC,
                D3D11_BLEND_INV_SRC_ALPHA, D3D11_BLEND_ONE, D3D11_BLEND_OP_ADD,
                D3D11_BLEND_SRC_ALPHA, D3D11_BUFFER_DESC, D3D11_COLOR_WRITE_ENABLE_ALL,
                D3D11_COMPARISON_ALWAYS, D3D11_CPU_ACCESS_WRITE, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                D3D11_CREATE_DEVICE_DEBUG, D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FLOAT32_MAX,
                D3D11_MAP_WRITE_DISCARD, D3D11_RESOURCE_MISC_BUFFER_STRUCTURED, D3D11_SAMPLER_DESC,
                D3D11_SDK_VERSION, D3D11_TEXTURE_ADDRESS_WRAP, D3D11_USAGE_DYNAMIC, D3D11_VIEWPORT,
            },
            DirectComposition::{
                DCompositionCreateDevice, IDCompositionDevice, IDCompositionTarget,
                IDCompositionVisual,
            },
            Dxgi::{
                Common::{
                    DXGI_ALPHA_MODE_IGNORE, DXGI_ALPHA_MODE_PREMULTIPLIED,
                    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_SAMPLE_DESC,
                },
                CreateDXGIFactory2, IDXGIAdapter1, IDXGIDevice, IDXGIFactory6, IDXGISwapChain1,
                DXGI_CREATE_FACTORY_DEBUG, DXGI_GPU_PREFERENCE_MINIMUM_POWER,
                DXGI_MWA_NO_ALT_ENTER, DXGI_SCALING_NONE, DXGI_SCALING_STRETCH,
                DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

use crate::*;

pub(crate) struct DirectXRenderer {
    atlas: Arc<DirectXAtlas>,
    context: DirectXContext,
    render: DirectXRenderContext,
    size: Size<DevicePixels>,
}

struct DirectXContext {
    dxgi_factory: IDXGIFactory6,
    device: ID3D11Device,
    dxgi_device: IDXGIDevice,
    context: ID3D11DeviceContext,
    swap_chain: IDXGISwapChain1,
    back_buffer: [Option<ID3D11RenderTargetView>; 1],
    viewport: [D3D11_VIEWPORT; 1],
    // comp_device: IDCompositionDevice,
    // comp_target: IDCompositionTarget,
    // comp_visual: IDCompositionVisual,
}

struct DirectXRenderContext {
    global_params_buffer: [Option<ID3D11Buffer>; 1],
    sampler: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,
    blend_state_for_pr: ID3D11BlendState,
    shadow_pipeline: PipelineState,
    quad_pipeline: PipelineState,
    path_raster_pipeline: PipelineState,
    paths_pipeline: PipelineState,
    underline_pipeline: PipelineState,
    mono_sprites: PipelineState,
    poly_sprites: PipelineState,
}

impl DirectXRenderer {
    pub(crate) fn new(hwnd: HWND) -> Self {
        let context = DirectXContext::new(hwnd).unwrap();
        let render = DirectXRenderContext::new(&context.device).unwrap();
        DirectXRenderer {
            atlas: Arc::new(DirectXAtlas::new(
                context.device.clone(),
                context.context.clone(),
            )),
            context,
            render,
            size: size(1.into(), 1.into()),
        }
    }

    pub(crate) fn spirite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    pub(crate) fn draw(&mut self, scene: &Scene) {
        let Some(path_tiles) = self.rasterize_paths(scene.paths()) else {
            log::error!("failed to rasterize {} paths", scene.paths().len());
            return;
        };
        pre_draw(
            &self.context.context,
            &self.render.global_params_buffer,
            &self.context.viewport,
            &self.context.back_buffer,
            // [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
            &self.render.blend_state,
            1,
        )
        .unwrap();
        for batch in scene.batches() {
            let ok = match batch {
                PrimitiveBatch::Shadows(shadows) => self.draw_shadows(shadows),
                PrimitiveBatch::Quads(quads) => self.draw_quads(quads),
                PrimitiveBatch::Paths(paths) => self.draw_paths(paths, &path_tiles),
                PrimitiveBatch::Underlines(underlines) => self.draw_underlines(underlines),
                PrimitiveBatch::MonochromeSprites {
                    texture_id,
                    sprites,
                } => self.draw_monochrome_sprites(texture_id, sprites),
                PrimitiveBatch::PolychromeSprites {
                    texture_id,
                    sprites,
                } => self.draw_polychrome_sprites(texture_id, sprites),
                PrimitiveBatch::Surfaces(surfaces) => self.draw_surfaces(surfaces),
            };
            if !ok {
                log::error!("scene too large: {} paths, {} shadows, {} quads, {} underlines, {} mono, {} poly, {} surfaces",
                    scene.paths.len(),
                    scene.shadows.len(),
                    scene.quads.len(),
                    scene.underlines.len(),
                    scene.monochrome_sprites.len(),
                    scene.polychrome_sprites.len(),
                    scene.surfaces.len(),);
                return;
            }
        }
        unsafe { self.context.swap_chain.Present(0, 0).ok().log_err() };
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) {
        unsafe {
            self.size = new_size;
            self.context.context.OMSetRenderTargets(None, None);
            drop(self.context.back_buffer[0].take().unwrap());
            self.context
                .swap_chain
                .ResizeBuffers(
                    BUFFER_COUNT as u32,
                    new_size.width.0 as u32,
                    new_size.height.0 as u32,
                    DXGI_FORMAT_B8G8R8A8_UNORM,
                    0,
                )
                .log_err();
            let backbuffer = set_render_target_view(
                &self.context.swap_chain,
                &self.context.device,
                &self.context.context,
            )
            .unwrap();
            self.context.back_buffer[0] = Some(backbuffer);
            self.context.viewport = set_viewport(
                &self.context.context,
                new_size.width.0 as f32,
                new_size.height.0 as f32,
            );
        }
    }

    pub(crate) fn update_transparency(
        &mut self,
        background_appearance: WindowBackgroundAppearance,
    ) {
        match background_appearance {
            WindowBackgroundAppearance::Opaque => {
                // TODO:
            }
            WindowBackgroundAppearance::Transparent => {
                // TODO:
            }
            WindowBackgroundAppearance::Blurred => {
                // TODO:
            }
        }
    }

    fn draw_shadows(&mut self, shadows: &[Shadow]) -> bool {
        if shadows.is_empty() {
            return true;
        }
        draw_normal(
            &self.context.context,
            &self.render.shadow_pipeline,
            &self.context.viewport,
            &self.render.global_params_buffer,
            shadows,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            shadows.len() as u32,
        )
        .unwrap();
        true
    }

    fn draw_quads(&mut self, quads: &[Quad]) -> bool {
        if quads.is_empty() {
            return true;
        }
        draw_normal(
            &self.context.context,
            &self.render.quad_pipeline,
            &self.context.viewport,
            &self.render.global_params_buffer,
            quads,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            quads.len() as u32,
        )
        .unwrap();
        true
    }

    fn rasterize_paths(
        &mut self,
        paths: &[Path<ScaledPixels>],
    ) -> Option<HashMap<PathId, AtlasTile>> {
        self.atlas.clear_textures(AtlasTextureKind::Path);

        let mut tiles = HashMap::default();
        let mut vertices_by_texture_id: HashMap<
            AtlasTextureId,
            Vec<PathVertex<ScaledPixels>>,
            BuildHasherDefault<FxHasher>,
        > = HashMap::default();
        for path in paths {
            let clipped_bounds = path.bounds.intersect(&path.content_mask.bounds);

            let tile = self
                .atlas
                .allocate(clipped_bounds.size.map(Into::into), AtlasTextureKind::Path)?;
            vertices_by_texture_id
                .entry(tile.texture_id)
                .or_insert(Vec::new())
                .extend(path.vertices.iter().map(|vertex| PathVertex {
                    xy_position: vertex.xy_position - clipped_bounds.origin
                        + tile.bounds.origin.map(Into::into),
                    st_position: vertex.st_position,
                    content_mask: ContentMask {
                        bounds: tile.bounds.map(Into::into),
                    },
                }));
            tiles.insert(path.id, tile);
        }

        for (texture_id, vertices) in vertices_by_texture_id {
            // align_offset(instance_offset);
            // let vertices_bytes_len = mem::size_of_val(vertices.as_slice());
            // let next_offset = *instance_offset + vertices_bytes_len;
            // if next_offset > instance_buffer.size {
            //     return None;
            // }
            let (texture_size, rtv, _) = self.atlas.texture_info(texture_id);
            let viewport = [D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: texture_size.width,
                Height: texture_size.height,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }];
            pre_draw(
                &self.context.context,
                &self.render.global_params_buffer,
                &viewport,
                &rtv,
                [0.0, 0.0, 0.0, 1.0],
                &self.render.blend_state_for_pr,
                0,
            )
            .unwrap();
            draw_normal(
                &self.context.context,
                &self.render.path_raster_pipeline,
                &viewport,
                &self.render.global_params_buffer,
                &vertices,
                D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
                vertices.len() as u32,
                1,
            )
            .unwrap();
        }

        Some(tiles)
    }

    fn draw_paths(
        &mut self,
        paths: &[Path<ScaledPixels>],
        path_tiles: &HashMap<PathId, AtlasTile>,
    ) -> bool {
        if paths.is_empty() {
            return true;
        }
        for path in paths {
            let tile = &path_tiles[&path.id];
            let (_, _, texture) = self.atlas.texture_info(tile.texture_id);
            let origin = path.bounds.intersect(&path.content_mask.bounds).origin;
            let sprites = [PathSprite {
                bounds: Bounds {
                    origin: origin.map(|p| p.floor()),
                    size: tile.bounds.size.map(Into::into),
                },
                color: path.color,
                tile: (*tile).clone(),
            }];

            draw_with_texture(
                &self.context.context,
                &self.render.paths_pipeline,
                &texture,
                &sprites,
                &self.context.viewport,
                &self.render.global_params_buffer,
                &self.render.sampler,
            )
            .unwrap();
        }
        true
    }

    fn draw_underlines(&mut self, underlines: &[Underline]) -> bool {
        if underlines.is_empty() {
            return true;
        }
        draw_normal(
            &self.context.context,
            &self.render.underline_pipeline,
            &self.context.viewport,
            &self.render.global_params_buffer,
            underlines,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            underlines.len() as u32,
        )
        .unwrap();

        true
    }

    fn draw_monochrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[MonochromeSprite],
    ) -> bool {
        if sprites.is_empty() {
            return true;
        }
        let (_, _, texture) = self.atlas.texture_info(texture_id);
        draw_with_texture(
            &self.context.context,
            &self.render.mono_sprites,
            &texture,
            sprites,
            &self.context.viewport,
            &self.render.global_params_buffer,
            &self.render.sampler,
        )
        .unwrap();
        true
    }

    fn draw_polychrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[PolychromeSprite],
    ) -> bool {
        if sprites.is_empty() {
            return true;
        }
        let (_, _, texture) = self.atlas.texture_info(texture_id);
        draw_with_texture(
            &self.context.context,
            &self.render.poly_sprites,
            &texture,
            sprites,
            &self.context.viewport,
            &self.render.global_params_buffer,
            &self.render.sampler,
        )
        .unwrap();
        true
    }

    fn draw_surfaces(&mut self, surfaces: &[Surface]) -> bool {
        if surfaces.is_empty() {
            return true;
        }
        true
    }
}

impl DirectXContext {
    pub fn new(hwnd: HWND) -> Result<Self> {
        let dxgi_factory = get_dxgi_factory()?;
        let adapter = get_adapter(&dxgi_factory)?;
        let (device, context) = {
            let mut device: Option<ID3D11Device> = None;
            let mut context: Option<ID3D11DeviceContext> = None;
            get_device(&adapter, Some(&mut device), Some(&mut context))?;
            (device.unwrap(), context.unwrap())
        };
        let dxgi_device: IDXGIDevice = device.cast().unwrap();
        // let comp_device = get_comp_device(&dxgi_device)?;
        // let swap_chain = create_swap_chain(&dxgi_factory, &device)?;
        let swap_chain = create_swap_chain_default(&dxgi_factory, &device, hwnd)?;
        // let comp_target = unsafe { comp_device.CreateTargetForHwnd(hwnd, true) }?;
        // let comp_visual = unsafe { comp_device.CreateVisual() }?;
        unsafe {
            // comp_visual.SetContent(&swap_chain)?;
            // comp_target.SetRoot(&comp_visual)?;
            // comp_device.Commit()?;
            dxgi_factory.MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER)?;
        }
        let back_buffer = [Some(set_render_target_view(
            &swap_chain,
            &device,
            &context,
        )?)];
        let viewport = set_viewport(&context, 1.0, 1.0);

        Ok(Self {
            dxgi_factory,
            device,
            dxgi_device,
            context,
            swap_chain,
            back_buffer,
            viewport,
            // comp_device,
            // comp_target,
            // comp_visual,
        })
    }
}

impl DirectXRenderContext {
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        let global_params_buffer = unsafe {
            let desc = D3D11_BUFFER_DESC {
                ByteWidth: std::mem::size_of::<GlobalParams>() as u32,
                Usage: D3D11_USAGE_DYNAMIC,
                BindFlags: D3D11_BIND_CONSTANT_BUFFER.0 as u32,
                CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                ..Default::default()
            };
            let mut buffer = None;
            device.CreateBuffer(&desc, None, Some(&mut buffer))?;
            [buffer]
        };

        let sampler = unsafe {
            let desc = D3D11_SAMPLER_DESC {
                Filter: D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: D3D11_TEXTURE_ADDRESS_WRAP,
                AddressV: D3D11_TEXTURE_ADDRESS_WRAP,
                AddressW: D3D11_TEXTURE_ADDRESS_WRAP,
                MipLODBias: 0.0,
                MaxAnisotropy: 1,
                ComparisonFunc: D3D11_COMPARISON_ALWAYS,
                BorderColor: [0.0; 4],
                MinLOD: 0.0,
                MaxLOD: D3D11_FLOAT32_MAX,
            };
            let mut output = None;
            device.CreateSamplerState(&desc, Some(&mut output))?;
            [output]
        };

        let blend_state = create_blend_state(device)?;
        let blend_state_for_pr = create_blend_state_for_path_raster(device)?;

        let shadow_pipeline =
            create_pipieline::<Shadow>(device, "shadow_vertex", "shadow_fragment", 256)?;
        let quad_pipeline = create_pipieline::<Quad>(device, "quad_vertex", "quad_fragment", 1024)?;
        let path_raster_pipeline = create_pipieline::<PathVertex<ScaledPixels>>(
            device,
            "path_rasterization_vertex",
            "path_rasterization_fragment",
            256,
        )?;
        let paths_pipeline =
            create_pipieline::<PathSprite>(device, "paths_vertex", "paths_fragment", 2)?;
        let underline_pipeline =
            create_pipieline::<Underline>(device, "underline_vertex", "underline_fragment", 256)?;
        let mono_sprites = create_pipieline::<MonochromeSprite>(
            device,
            "monochrome_sprite_vertex",
            "monochrome_sprite_fragment",
            1024,
        )?;
        let poly_sprites = create_pipieline::<PolychromeSprite>(
            device,
            "polychrome_sprite_vertex",
            "polychrome_sprite_fragment",
            128,
        )?;

        Ok(Self {
            global_params_buffer,
            sampler,
            blend_state,
            blend_state_for_pr,
            shadow_pipeline,
            quad_pipeline,
            path_raster_pipeline,
            paths_pipeline,
            underline_pipeline,
            mono_sprites,
            poly_sprites,
        })
    }
}

#[derive(Debug, Default)]
#[repr(C)]
struct GlobalParams {
    viewport_size: [f32; 2],
    premultiplied_alpha: u32,
    _pad: u32,
}

struct PipelineState {
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
    buffer: ID3D11Buffer,
    view: [Option<ID3D11ShaderResourceView>; 1],
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
struct PathSprite {
    bounds: Bounds<ScaledPixels>,
    color: Hsla,
    tile: AtlasTile,
}

fn get_dxgi_factory() -> Result<IDXGIFactory6> {
    #[cfg(debug_assertions)]
    let factory_flag = DXGI_CREATE_FACTORY_DEBUG;
    #[cfg(not(debug_assertions))]
    let factory_flag = 0u32;
    unsafe { Ok(CreateDXGIFactory2(factory_flag)?) }
}

fn get_adapter(dxgi_factory: &IDXGIFactory6) -> Result<IDXGIAdapter1> {
    for adapter_index in 0.. {
        let adapter: IDXGIAdapter1 = unsafe {
            dxgi_factory
                .EnumAdapterByGpuPreference(adapter_index, DXGI_GPU_PREFERENCE_MINIMUM_POWER)
        }?;
        {
            let mut desc = unsafe { std::mem::zeroed() };
            unsafe { adapter.GetDesc1(&mut desc) }?;
            println!(
                "Select GPU: {}",
                String::from_utf16_lossy(&desc.Description)
            );
        }
        // Check to see whether the adapter supports Direct3D 11, but don't
        // create the actual device yet.
        if get_device(&adapter, None, None).log_err().is_some() {
            return Ok(adapter);
        }
    }

    unreachable!()
}

fn get_device(
    adapter: &IDXGIAdapter1,
    device: Option<*mut Option<ID3D11Device>>,
    context: Option<*mut Option<ID3D11DeviceContext>>,
) -> Result<()> {
    #[cfg(debug_assertions)]
    let device_flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG;
    #[cfg(not(debug_assertions))]
    let device_flags = D3D11_CREATE_DEVICE_BGRA_SUPPORT;
    Ok(unsafe {
        D3D11CreateDevice(
            adapter,
            D3D_DRIVER_TYPE_UNKNOWN,
            None,
            device_flags,
            Some(&[D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1]),
            D3D11_SDK_VERSION,
            device,
            None,
            context,
        )
    }?)
}

fn get_comp_device(dxgi_device: &IDXGIDevice) -> Result<IDCompositionDevice> {
    Ok(unsafe { DCompositionCreateDevice(dxgi_device)? })
}

fn create_swap_chain(
    dxgi_factory: &IDXGIFactory6,
    device: &ID3D11Device,
) -> Result<IDXGISwapChain1> {
    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Width: 1,
        Height: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        Stereo: false.into(),
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: BUFFER_COUNT as u32,
        // Composition SwapChains only support the DXGI_SCALING_STRETCH Scaling.
        Scaling: DXGI_SCALING_STRETCH,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        // Premultiplied alpha is the only supported format by composition swapchain.
        AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
        Flags: 0,
    };
    Ok(unsafe { dxgi_factory.CreateSwapChainForComposition(device, &desc, None)? })
}

fn create_swap_chain_default(
    dxgi_factory: &IDXGIFactory6,
    device: &ID3D11Device,
    hwnd: HWND,
) -> Result<IDXGISwapChain1> {
    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Width: 1,
        Height: 1,
        Format: DXGI_FORMAT_B8G8R8A8_UNORM,
        Stereo: false.into(),
        SampleDesc: DXGI_SAMPLE_DESC {
            Count: 1,
            Quality: 0,
        },
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: BUFFER_COUNT as u32,
        Scaling: DXGI_SCALING_NONE,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        // Premultiplied alpha is the only supported format by composition swapchain.
        // AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
        AlphaMode: DXGI_ALPHA_MODE_IGNORE,
        Flags: 0,
    };
    let x = unsafe { dxgi_factory.CreateSwapChainForHwnd(device, hwnd, &desc, None, None) }?;
    Ok(x)
}

fn set_render_target_view(
    swap_chain: &IDXGISwapChain1,
    device: &ID3D11Device,
    device_context: &ID3D11DeviceContext,
) -> Result<ID3D11RenderTargetView> {
    // In dx11, ID3D11RenderTargetView is supposed to always point to the new back buffer.
    // https://stackoverflow.com/questions/65246961/does-the-backbuffer-that-a-rendertargetview-points-to-automagically-change-after
    let back_buffer = unsafe {
        let resource: ID3D11Texture2D = swap_chain.GetBuffer(0)?;
        let mut buffer: Option<ID3D11RenderTargetView> = None;
        device.CreateRenderTargetView(&resource, None, Some(&mut buffer))?;
        buffer.unwrap()
    };
    unsafe { device_context.OMSetRenderTargets(Some(&[Some(back_buffer.clone())]), None) };
    Ok(back_buffer)
}

fn set_viewport(
    device_context: &ID3D11DeviceContext,
    width: f32,
    height: f32,
) -> [D3D11_VIEWPORT; 1] {
    let viewport = [D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: width,
        Height: height,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    }];
    unsafe { device_context.RSSetViewports(Some(&viewport)) };
    viewport
}

fn build_shader_blob(entry: &str, target: &str) -> Result<ID3DBlob> {
    unsafe {
        let mut entry = entry.to_owned();
        let mut target = target.to_owned();
        let mut compile_blob = None;
        let mut error_blob = None;
        // let shader_path = std::path::PathBuf::from("crates/gpui/src/platform/windows/shaders.hlsl")
        let shader_path = std::path::PathBuf::from(
            "D:/projects/zed/crates/gpui/src/platform/windows/shaders.hlsl",
        )
        // let shader_path = std::path::PathBuf::from(
        //     "D:/projects/zed/crates/gpui/src/platform/windows/test_shader.hlsl",
        // )
        .canonicalize()
        .unwrap();
        entry.push_str("\0");
        target.push_str("\0");
        let entry_point = PCSTR::from_raw(entry.as_ptr());
        let target_cstr = PCSTR::from_raw(target.as_ptr());
        let ret = D3DCompileFromFile(
            &HSTRING::from(shader_path.as_os_str()),
            None,
            None,
            entry_point,
            target_cstr,
            D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION,
            0,
            &mut compile_blob,
            Some(&mut error_blob),
        );
        if ret.is_err() {
            let Some(error_blob) = error_blob else {
                return Err(anyhow::anyhow!("{ret:?}"));
            };
            let string_len = error_blob.GetBufferSize();
            let error_string_encode = Vec::from_raw_parts(
                error_blob.GetBufferPointer() as *mut u8,
                string_len,
                string_len,
            );
            return Err(anyhow::anyhow!(
                "Compile error: {}",
                String::from_utf8_lossy(&error_string_encode)
            ));
        }
        Ok(compile_blob.unwrap())
    }
}

fn create_vertex_shader(device: &ID3D11Device, blob: &ID3DBlob) -> Result<ID3D11VertexShader> {
    unsafe {
        let mut shader = None;
        device.CreateVertexShader(
            std::slice::from_raw_parts(blob.GetBufferPointer() as *mut u8, blob.GetBufferSize()),
            None,
            Some(&mut shader),
        )?;
        Ok(shader.unwrap())
    }
}

fn create_fragment_shader(device: &ID3D11Device, blob: &ID3DBlob) -> Result<ID3D11PixelShader> {
    unsafe {
        let mut shader = None;
        device.CreatePixelShader(
            std::slice::from_raw_parts(blob.GetBufferPointer() as *mut u8, blob.GetBufferSize()),
            None,
            Some(&mut shader),
        )?;
        Ok(shader.unwrap())
    }
}

// https://learn.microsoft.com/en-us/windows/win32/api/d3d11/ns-d3d11-d3d11_blend_desc
fn create_blend_state(device: &ID3D11Device) -> Result<ID3D11BlendState> {
    // If the feature level is set to greater than D3D_FEATURE_LEVEL_9_3, the display
    // device performs the blend in linear space, which is ideal.
    let mut desc = D3D11_BLEND_DESC::default();
    desc.RenderTarget[0].BlendEnable = true.into();
    desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
    desc.RenderTarget[0].SrcBlend = D3D11_BLEND_SRC_ALPHA;
    desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
    desc.RenderTarget[0].DestBlend = D3D11_BLEND_INV_SRC_ALPHA;
    desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ONE;
    desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;
    unsafe {
        let mut state = None;
        device.CreateBlendState(&desc, Some(&mut state))?;
        Ok(state.unwrap())
    }
}

fn create_blend_state_for_path_raster(device: &ID3D11Device) -> Result<ID3D11BlendState> {
    // If the feature level is set to greater than D3D_FEATURE_LEVEL_9_3, the display
    // device performs the blend in linear space, which is ideal.
    let mut desc = D3D11_BLEND_DESC::default();
    desc.RenderTarget[0].BlendEnable = true.into();
    desc.RenderTarget[0].BlendOp = D3D11_BLEND_OP_ADD;
    desc.RenderTarget[0].BlendOpAlpha = D3D11_BLEND_OP_ADD;
    desc.RenderTarget[0].SrcBlend = D3D11_BLEND_ONE;
    desc.RenderTarget[0].SrcBlendAlpha = D3D11_BLEND_ONE;
    desc.RenderTarget[0].DestBlend = D3D11_BLEND_ONE;
    desc.RenderTarget[0].DestBlendAlpha = D3D11_BLEND_ONE;
    desc.RenderTarget[0].RenderTargetWriteMask = D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8;
    unsafe {
        let mut state = None;
        device.CreateBlendState(&desc, Some(&mut state))?;
        Ok(state.unwrap())
    }
}

fn create_pipieline<T>(
    device: &ID3D11Device,
    vertex_entry: &str,
    fragment_entry: &str,
    buffer_size: u32,
) -> Result<PipelineState> {
    let vertex_shader_blob = build_shader_blob(vertex_entry, "vs_5_0").unwrap();
    let vertex = create_vertex_shader(device, &vertex_shader_blob)?;
    let fragment_shader_blob = build_shader_blob(fragment_entry, "ps_5_0").unwrap();
    let fragment = create_fragment_shader(device, &fragment_shader_blob)?;
    let buffer = {
        let desc = D3D11_BUFFER_DESC {
            ByteWidth: std::mem::size_of::<T>() as u32 * buffer_size,
            Usage: D3D11_USAGE_DYNAMIC,
            BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
            CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
            MiscFlags: D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0 as u32,
            StructureByteStride: std::mem::size_of::<T>() as u32,
        };
        let mut buffer = None;
        unsafe { device.CreateBuffer(&desc, None, Some(&mut buffer)) }?;
        buffer.unwrap()
    };
    let view = {
        let mut view = None;
        unsafe { device.CreateShaderResourceView(&buffer, None, Some(&mut view)) }?;
        [view]
    };
    Ok(PipelineState {
        vertex,
        fragment,
        buffer,
        view,
    })
}

fn update_global_params(
    device_context: &ID3D11DeviceContext,
    buffer: &[Option<ID3D11Buffer>; 1],
    globals: GlobalParams,
) -> Result<()> {
    let buffer = buffer[0].as_ref().unwrap();
    unsafe {
        let mut data = std::mem::zeroed();
        device_context.Map(buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut data))?;
        std::ptr::copy_nonoverlapping(&globals, data.pData as *mut _, 1);
        device_context.Unmap(buffer, 0);
    }
    Ok(())
}

fn pre_draw(
    device_context: &ID3D11DeviceContext,
    global_params_buffer: &[Option<ID3D11Buffer>; 1],
    view_port: &[D3D11_VIEWPORT; 1],
    render_target_view: &[Option<ID3D11RenderTargetView>; 1],
    clear_color: [f32; 4],
    blend_state: &ID3D11BlendState,
    premultiplied_alpha: u32,
) -> Result<()> {
    update_global_params(
        device_context,
        global_params_buffer,
        GlobalParams {
            viewport_size: [view_port[0].Width, view_port[0].Height],
            premultiplied_alpha,
            _pad: 0,
        },
    )?;
    unsafe {
        device_context.RSSetViewports(Some(view_port));
        device_context.OMSetRenderTargets(Some(render_target_view), None);
        device_context.ClearRenderTargetView(render_target_view[0].as_ref().unwrap(), &clear_color);
        device_context.OMSetBlendState(blend_state, None, 0xFFFFFFFF);
    }
    Ok(())
}

fn update_buffer<T>(
    device_context: &ID3D11DeviceContext,
    buffer: &ID3D11Buffer,
    data: &[T],
) -> Result<()> {
    unsafe {
        let mut dest = std::mem::zeroed();
        device_context.Map(buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut dest))?;
        std::ptr::copy_nonoverlapping(data.as_ptr(), dest.pData as _, data.len());
        device_context.Unmap(buffer, 0);
    }
    Ok(())
}

fn draw_normal<T>(
    device_context: &ID3D11DeviceContext,
    pipeline: &PipelineState,
    viewport: &[D3D11_VIEWPORT],
    global_params: &[Option<ID3D11Buffer>],
    data: &[T],
    topology: D3D_PRIMITIVE_TOPOLOGY,
    vertex_count: u32,
    instance_count: u32,
) -> Result<()> {
    update_buffer(device_context, &pipeline.buffer, data)?;
    unsafe {
        device_context.VSSetShaderResources(1, Some(&pipeline.view));
        device_context.PSSetShaderResources(1, Some(&pipeline.view));
        device_context.IASetPrimitiveTopology(topology);
        device_context.RSSetViewports(Some(viewport));
        device_context.VSSetShader(&pipeline.vertex, None);
        device_context.PSSetShader(&pipeline.fragment, None);
        device_context.VSSetConstantBuffers(0, Some(global_params));
        device_context.PSSetConstantBuffers(0, Some(global_params));

        device_context.DrawInstanced(vertex_count, instance_count, 0, 0);
    }
    Ok(())
}

fn draw_with_texture<T>(
    device_context: &ID3D11DeviceContext,
    pipeline: &PipelineState,
    texture: &[Option<ID3D11ShaderResourceView>],
    data: &[T],
    viewport: &[D3D11_VIEWPORT],
    global_params: &[Option<ID3D11Buffer>],
    sampler: &[Option<ID3D11SamplerState>],
) -> Result<()> {
    update_buffer(device_context, &pipeline.buffer, data)?;
    unsafe {
        device_context.IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
        device_context.RSSetViewports(Some(viewport));
        device_context.VSSetShader(&pipeline.vertex, None);
        device_context.PSSetShader(&pipeline.fragment, None);
        device_context.VSSetConstantBuffers(0, Some(global_params));
        device_context.PSSetConstantBuffers(0, Some(global_params));
        device_context.VSSetShaderResources(1, Some(&pipeline.view));
        device_context.PSSetShaderResources(1, Some(&pipeline.view));
        device_context.PSSetSamplers(0, Some(sampler));
        device_context.VSSetShaderResources(0, Some(texture));
        device_context.PSSetShaderResources(0, Some(texture));

        device_context.DrawInstanced(4, data.len() as u32, 0, 0);
    }
    Ok(())
}

const BUFFER_COUNT: usize = 3;
const BUFFER_SIZE: usize = 2 * 1024 * 1024;
