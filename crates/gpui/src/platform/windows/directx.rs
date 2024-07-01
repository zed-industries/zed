use std::{collections::HashMap, hash::BuildHasherDefault, sync::Arc};

use ::util::ResultExt;
use anyhow::{Context, Result};
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
                D3D11_CREATE_DEVICE_DEBUG, D3D11_CULL_NONE, D3D11_FILL_SOLID,
                D3D11_FILTER_MIN_MAG_MIP_LINEAR, D3D11_FLOAT32_MAX, D3D11_MAP_WRITE_DISCARD,
                D3D11_RASTERIZER_DESC, D3D11_RESOURCE_MISC_BUFFER_STRUCTURED, D3D11_SAMPLER_DESC,
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
                DXGI_CREATE_FACTORY_DEBUG, DXGI_GPU_PREFERENCE_MINIMUM_POWER, DXGI_SCALING_STRETCH,
                DXGI_SWAP_CHAIN_DESC1, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
    },
};

use crate::*;

pub(crate) struct DirectXRenderer {
    atlas: Arc<DirectXAtlas>,
    devices: DirectXDevices,
    context: DirectXContext,
    globals: DirectXGlobalElements,
    pipelines: DirectXRenderPipelines,
    hwnd: HWND,
    transparent: bool,
}

#[derive(Clone)]
pub(crate) struct DirectXDevices {
    dxgi_factory: IDXGIFactory6,
    dxgi_device: IDXGIDevice,
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
}

struct DirectXContext {
    swap_chain: IDXGISwapChain1,
    back_buffer: [Option<ID3D11RenderTargetView>; 1],
    viewport: [D3D11_VIEWPORT; 1],
    #[cfg(not(target_feature = "enable-renderdoc"))]
    direct_composition: DirectComposition,
}

struct DirectXRenderPipelines {
    shadow_pipeline: PipelineState,
    quad_pipeline: PipelineState,
    path_raster_pipeline: PipelineState,
    paths_pipeline: PipelineState,
    underline_pipeline: PipelineState,
    mono_sprites: PipelineState,
    poly_sprites: PipelineState,
}

struct DirectXGlobalElements {
    global_params_buffer: [Option<ID3D11Buffer>; 1],
    sampler: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,
    blend_state_for_pr: ID3D11BlendState,
}

#[cfg(not(target_feature = "enable-renderdoc"))]
struct DirectComposition {
    comp_device: IDCompositionDevice,
    comp_target: IDCompositionTarget,
    comp_visual: IDCompositionVisual,
}

impl DirectXDevices {
    pub(crate) fn new() -> Result<Self> {
        let dxgi_factory = get_dxgi_factory()?;
        let adapter = get_adapter(&dxgi_factory)?;
        let (device, device_context) = {
            let mut device: Option<ID3D11Device> = None;
            let mut context: Option<ID3D11DeviceContext> = None;
            get_device(&adapter, Some(&mut device), Some(&mut context))?;
            (device.unwrap(), context.unwrap())
        };
        let dxgi_device: IDXGIDevice = device.cast()?;

        Ok(Self {
            dxgi_factory,
            dxgi_device,
            device,
            device_context,
        })
    }
}

impl DirectXRenderer {
    pub(crate) fn new(devices: DirectXDevices, hwnd: HWND, transparent: bool) -> Result<Self> {
        let atlas = Arc::new(DirectXAtlas::new(
            devices.device.clone(),
            devices.device_context.clone(),
        ));
        let context = DirectXContext::new(&devices, hwnd, transparent)?;
        let globals = DirectXGlobalElements::new(&devices.device)?;
        let pipelines = DirectXRenderPipelines::new(&devices.device)?;
        Ok(DirectXRenderer {
            atlas,
            devices,
            context,
            globals,
            pipelines,
            hwnd,
            transparent,
        })
    }

    pub(crate) fn spirite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    pub(crate) fn draw(&mut self, scene: &Scene) -> Result<()> {
        let Some(path_tiles) = self.rasterize_paths(scene.paths()) else {
            return Err(anyhow::anyhow!(
                "failed to rasterize {} paths",
                scene.paths().len()
            ));
        };
        pre_draw(
            &self.devices.device_context,
            &self.globals.global_params_buffer,
            &self.context.viewport,
            &self.context.back_buffer,
            [0.0, 0.0, 0.0, 0.0],
            &self.globals.blend_state,
        )?;
        for batch in scene.batches() {
            match batch {
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
            }.context(format!("scene too large: {} paths, {} shadows, {} quads, {} underlines, {} mono, {} poly, {} surfaces",
                    scene.paths.len(),
                    scene.shadows.len(),
                    scene.quads.len(),
                    scene.underlines.len(),
                    scene.monochrome_sprites.len(),
                    scene.polychrome_sprites.len(),
                    scene.surfaces.len(),))?;
        }
        unsafe { self.context.swap_chain.Present(0, 0) }.ok()?;
        Ok(())
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) -> Result<()> {
        unsafe { self.devices.device_context.OMSetRenderTargets(None, None) };
        drop(self.context.back_buffer[0].take().unwrap());
        unsafe {
            self.context.swap_chain.ResizeBuffers(
                BUFFER_COUNT as u32,
                new_size.width.0 as u32,
                new_size.height.0 as u32,
                DXGI_FORMAT_B8G8R8A8_UNORM,
                0,
            )?;
        }
        let backbuffer = set_render_target_view(
            &self.context.swap_chain,
            &self.devices.device,
            &self.devices.device_context,
        )?;
        self.context.back_buffer[0] = Some(backbuffer);
        self.context.viewport = set_viewport(
            &self.devices.device_context,
            new_size.width.0 as f32,
            new_size.height.0 as f32,
        );
        Ok(())
    }

    #[cfg(not(target_feature = "enable-renderdoc"))]
    pub(crate) fn update_transparency(
        &mut self,
        background_appearance: WindowBackgroundAppearance,
    ) -> Result<()> {
        // We only support setting `Transparent` and `Opaque` for now.
        match background_appearance {
            WindowBackgroundAppearance::Opaque => {
                if self.transparent {
                    return Err(anyhow::anyhow!("Restart required."));
                }
            }
            WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred => {
                if !self.transparent {
                    return Err(anyhow::anyhow!("Restart required."));
                }
            }
        }
        Ok(())
    }

    #[cfg(target_feature = "enable-renderdoc")]
    pub(crate) fn update_transparency(
        &mut self,
        _background_appearance: WindowBackgroundAppearance,
    ) {
    }

    fn draw_shadows(&mut self, shadows: &[Shadow]) -> Result<()> {
        if shadows.is_empty() {
            return Ok(());
        }
        update_buffer_capacity(
            &self.pipelines.shadow_pipeline,
            std::mem::size_of::<Shadow>(),
            shadows.len(),
            &self.devices.device,
        )
        .map(|input| update_pipeline(&mut self.pipelines.shadow_pipeline, input));
        update_buffer(
            &self.devices.device_context,
            &self.pipelines.shadow_pipeline.buffer,
            shadows,
        )?;
        draw_normal(
            &self.devices.device_context,
            &self.pipelines.shadow_pipeline,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            shadows.len() as u32,
        )
    }

    fn draw_quads(&mut self, quads: &[Quad]) -> Result<()> {
        if quads.is_empty() {
            return Ok(());
        }
        update_buffer_capacity(
            &self.pipelines.quad_pipeline,
            std::mem::size_of::<Quad>(),
            quads.len(),
            &self.devices.device,
        )
        .map(|input| update_pipeline(&mut self.pipelines.quad_pipeline, input));
        update_buffer(
            &self.devices.device_context,
            &self.pipelines.quad_pipeline.buffer,
            quads,
        )?;
        draw_normal(
            &self.devices.device_context,
            &self.pipelines.quad_pipeline,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            quads.len() as u32,
        )
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
            let (texture_size, rtv) = self.atlas.get_texture_drawing_info(texture_id);
            let viewport = [D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: texture_size.width,
                Height: texture_size.height,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }];
            pre_draw(
                &self.devices.device_context,
                &self.globals.global_params_buffer,
                &viewport,
                &rtv,
                [0.0, 0.0, 0.0, 1.0],
                &self.globals.blend_state_for_pr,
            )
            .log_err()?;
            update_buffer_capacity(
                &self.pipelines.path_raster_pipeline,
                std::mem::size_of::<PathVertex<ScaledPixels>>(),
                vertices.len(),
                &self.devices.device,
            )
            .map(|input| update_pipeline(&mut self.pipelines.path_raster_pipeline, input));
            update_buffer(
                &self.devices.device_context,
                &self.pipelines.path_raster_pipeline.buffer,
                &vertices,
            )
            .log_err()?;
            draw_normal(
                &self.devices.device_context,
                &self.pipelines.path_raster_pipeline,
                &viewport,
                &self.globals.global_params_buffer,
                D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
                vertices.len() as u32,
                1,
            )
            .log_err()?;
        }
        Some(tiles)
    }

    fn draw_paths(
        &mut self,
        paths: &[Path<ScaledPixels>],
        path_tiles: &HashMap<PathId, AtlasTile>,
    ) -> Result<()> {
        if paths.is_empty() {
            return Ok(());
        }
        for path in paths {
            let tile = &path_tiles[&path.id];
            let texture_view = self.atlas.get_texture_view(tile.texture_id);
            let origin = path.bounds.intersect(&path.content_mask.bounds).origin;
            let sprites = [PathSprite {
                bounds: Bounds {
                    origin: origin.map(|p| p.floor()),
                    size: tile.bounds.size.map(Into::into),
                },
                color: path.color,
                tile: (*tile).clone(),
            }];
            update_buffer_capacity(
                &self.pipelines.paths_pipeline,
                std::mem::size_of::<PathSprite>(),
                1,
                &self.devices.device,
            )
            .map(|input| update_pipeline(&mut self.pipelines.paths_pipeline, input));
            update_buffer(
                &self.devices.device_context,
                &self.pipelines.paths_pipeline.buffer,
                &sprites,
            )?;
            draw_with_texture(
                &self.devices.device_context,
                &self.pipelines.paths_pipeline,
                &texture_view,
                &self.context.viewport,
                &self.globals.global_params_buffer,
                &self.globals.sampler,
                1,
            )?;
        }
        Ok(())
    }

    fn draw_underlines(&mut self, underlines: &[Underline]) -> Result<()> {
        if underlines.is_empty() {
            return Ok(());
        }
        update_buffer_capacity(
            &self.pipelines.underline_pipeline,
            std::mem::size_of::<Underline>(),
            underlines.len(),
            &self.devices.device,
        )
        .map(|input| update_pipeline(&mut self.pipelines.underline_pipeline, input));
        update_buffer(
            &self.devices.device_context,
            &self.pipelines.underline_pipeline.buffer,
            underlines,
        )?;
        draw_normal(
            &self.devices.device_context,
            &self.pipelines.underline_pipeline,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            4,
            underlines.len() as u32,
        )
    }

    fn draw_monochrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[MonochromeSprite],
    ) -> Result<()> {
        if sprites.is_empty() {
            return Ok(());
        }
        let texture_view = self.atlas.get_texture_view(texture_id);
        update_buffer_capacity(
            &self.pipelines.mono_sprites,
            std::mem::size_of::<MonochromeSprite>(),
            sprites.len(),
            &self.devices.device,
        )
        .map(|input| update_pipeline(&mut self.pipelines.mono_sprites, input));
        update_buffer(
            &self.devices.device_context,
            &self.pipelines.mono_sprites.buffer,
            sprites,
        )?;
        draw_with_texture(
            &self.devices.device_context,
            &self.pipelines.mono_sprites,
            &texture_view,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            &self.globals.sampler,
            sprites.len() as u32,
        )
    }

    fn draw_polychrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[PolychromeSprite],
    ) -> Result<()> {
        if sprites.is_empty() {
            return Ok(());
        }
        let texture_view = self.atlas.get_texture_view(texture_id);
        update_buffer_capacity(
            &self.pipelines.poly_sprites,
            std::mem::size_of::<PolychromeSprite>(),
            sprites.len(),
            &self.devices.device,
        )
        .map(|input| update_pipeline(&mut self.pipelines.poly_sprites, input));
        update_buffer(
            &self.devices.device_context,
            &self.pipelines.poly_sprites.buffer,
            sprites,
        )?;
        draw_with_texture(
            &self.devices.device_context,
            &self.pipelines.poly_sprites,
            &texture_view,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            &self.globals.sampler,
            sprites.len() as u32,
        )
    }

    fn draw_surfaces(&mut self, surfaces: &[Surface]) -> Result<()> {
        if surfaces.is_empty() {
            return Ok(());
        }
        Ok(())
    }
}

impl DirectXContext {
    pub fn new(devices: &DirectXDevices, hwnd: HWND, transparent: bool) -> Result<Self> {
        #[cfg(not(target_feature = "enable-renderdoc"))]
        let swap_chain = create_swap_chain(&devices.dxgi_factory, &devices.device, transparent)?;
        #[cfg(target_feature = "enable-renderdoc")]
        let swap_chain =
            create_swap_chain_default(&devices.dxgi_factory, &devices.device, hwnd, transparent)?;
        #[cfg(not(target_feature = "enable-renderdoc"))]
        let direct_composition = DirectComposition::new(&devices.dxgi_device, hwnd)?;
        #[cfg(not(target_feature = "enable-renderdoc"))]
        direct_composition.set_swap_chain(&swap_chain)?;
        let back_buffer = [Some(set_render_target_view(
            &swap_chain,
            &devices.device,
            &devices.device_context,
        )?)];
        let viewport = set_viewport(&devices.device_context, 1.0, 1.0);
        set_rasterizer_state(&devices.device, &devices.device_context)?;

        Ok(Self {
            swap_chain,
            back_buffer,
            viewport,
            #[cfg(not(target_feature = "enable-renderdoc"))]
            direct_composition,
        })
    }
}

impl DirectXRenderPipelines {
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        let shadow_pipeline = create_pipieline(
            device,
            "shadow_vertex",
            "shadow_fragment",
            std::mem::size_of::<Shadow>(),
            32,
        )?;
        let quad_pipeline = create_pipieline(
            device,
            "quad_vertex",
            "quad_fragment",
            std::mem::size_of::<Quad>(),
            32,
        )?;
        let path_raster_pipeline = create_pipieline(
            device,
            "path_rasterization_vertex",
            "path_rasterization_fragment",
            std::mem::size_of::<PathVertex<ScaledPixels>>(),
            32,
        )?;
        let paths_pipeline = create_pipieline(
            device,
            "paths_vertex",
            "paths_fragment",
            std::mem::size_of::<PathSprite>(),
            1,
        )?;
        let underline_pipeline = create_pipieline(
            device,
            "underline_vertex",
            "underline_fragment",
            std::mem::size_of::<Underline>(),
            32,
        )?;
        let mono_sprites = create_pipieline(
            device,
            "monochrome_sprite_vertex",
            "monochrome_sprite_fragment",
            std::mem::size_of::<MonochromeSprite>(),
            32,
        )?;
        let poly_sprites = create_pipieline(
            device,
            "polychrome_sprite_vertex",
            "polychrome_sprite_fragment",
            std::mem::size_of::<PolychromeSprite>(),
            32,
        )?;

        Ok(Self {
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

impl DirectComposition {
    pub fn new(dxgi_device: &IDXGIDevice, hwnd: HWND) -> Result<Self> {
        let comp_device = get_comp_device(&dxgi_device)?;
        let comp_target = unsafe { comp_device.CreateTargetForHwnd(hwnd, true) }?;
        let comp_visual = unsafe { comp_device.CreateVisual() }?;

        Ok(Self {
            comp_device,
            comp_target,
            comp_visual,
        })
    }

    pub fn set_swap_chain(&self, swap_chain: &IDXGISwapChain1) -> Result<()> {
        unsafe {
            self.comp_visual.SetContent(swap_chain)?;
            self.comp_target.SetRoot(&self.comp_visual)?;
            self.comp_device.Commit()?;
        }
        Ok(())
    }
}

impl DirectXGlobalElements {
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

        Ok(Self {
            global_params_buffer,
            sampler,
            blend_state,
            blend_state_for_pr,
        })
    }
}

#[derive(Debug, Default)]
#[repr(C)]
struct GlobalParams {
    viewport_size: [f32; 2],
    _pad: u64,
}

struct PipelineState {
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
    buffer: ID3D11Buffer,
    buffer_size: usize,
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
        )?
    })
}

fn get_comp_device(dxgi_device: &IDXGIDevice) -> Result<IDCompositionDevice> {
    Ok(unsafe { DCompositionCreateDevice(dxgi_device)? })
}

fn create_swap_chain(
    dxgi_factory: &IDXGIFactory6,
    device: &ID3D11Device,
    transparent: bool,
) -> Result<IDXGISwapChain1> {
    let alpha_mode = if transparent {
        DXGI_ALPHA_MODE_PREMULTIPLIED
    } else {
        DXGI_ALPHA_MODE_IGNORE
    };
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
        AlphaMode: alpha_mode,
        Flags: 0,
    };
    Ok(unsafe { dxgi_factory.CreateSwapChainForComposition(device, &desc, None)? })
}

#[cfg(target_feature = "enable-renderdoc")]
fn create_swap_chain_default(
    dxgi_factory: &IDXGIFactory6,
    device: &ID3D11Device,
    hwnd: HWND,
    _transparent: bool,
) -> Result<IDXGISwapChain1> {
    use windows::Win32::Graphics::Dxgi::DXGI_MWA_NO_ALT_ENTER;

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
        Scaling: DXGI_SCALING_STRETCH,
        SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
        AlphaMode: DXGI_ALPHA_MODE_IGNORE,
        Flags: 0,
    };
    let swap_chain =
        unsafe { dxgi_factory.CreateSwapChainForHwnd(device, hwnd, &desc, None, None) }?;
    unsafe { dxgi_factory.MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER) }?;
    Ok(swap_chain)
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

fn set_rasterizer_state(device: &ID3D11Device, device_context: &ID3D11DeviceContext) -> Result<()> {
    let desc = D3D11_RASTERIZER_DESC {
        FillMode: D3D11_FILL_SOLID,
        CullMode: D3D11_CULL_NONE,
        FrontCounterClockwise: false.into(),
        DepthBias: 0,
        DepthBiasClamp: 0.0,
        SlopeScaledDepthBias: 0.0,
        DepthClipEnable: true.into(),
        ScissorEnable: false.into(),
        MultisampleEnable: false.into(),
        AntialiasedLineEnable: false.into(),
    };
    let rasterizer_state = unsafe {
        let mut state = None;
        device.CreateRasterizerState(&desc, Some(&mut state))?;
        state.unwrap()
    };
    unsafe { device_context.RSSetState(&rasterizer_state) };
    Ok(())
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

fn create_pipieline(
    device: &ID3D11Device,
    vertex_entry: &str,
    fragment_entry: &str,
    element_size: usize,
    buffer_size: usize,
) -> Result<PipelineState> {
    let vertex_shader_blob = build_shader_blob(vertex_entry, "vs_5_0")?;
    let vertex = create_vertex_shader(device, &vertex_shader_blob)?;
    let fragment_shader_blob = build_shader_blob(fragment_entry, "ps_5_0")?;
    let fragment = create_fragment_shader(device, &fragment_shader_blob)?;
    let buffer = create_buffer(device, element_size, buffer_size)?;
    let view = create_buffer_view(device, &buffer)?;
    Ok(PipelineState {
        vertex,
        fragment,
        buffer,
        buffer_size,
        view,
    })
}

fn create_buffer(
    device: &ID3D11Device,
    element_size: usize,
    buffer_size: usize,
) -> Result<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: (element_size * buffer_size) as u32,
        Usage: D3D11_USAGE_DYNAMIC,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
        MiscFlags: D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0 as u32,
        StructureByteStride: element_size as u32,
    };
    let mut buffer = None;
    unsafe { device.CreateBuffer(&desc, None, Some(&mut buffer)) }?;
    Ok(buffer.unwrap())
}

fn create_buffer_view(
    device: &ID3D11Device,
    buffer: &ID3D11Buffer,
) -> Result<[Option<ID3D11ShaderResourceView>; 1]> {
    let mut view = None;
    unsafe { device.CreateShaderResourceView(buffer, None, Some(&mut view)) }?;
    Ok([view])
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
) -> Result<()> {
    update_global_params(
        device_context,
        global_params_buffer,
        GlobalParams {
            viewport_size: [view_port[0].Width, view_port[0].Height],
            ..Default::default()
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

fn update_buffer_capacity(
    pipeline: &PipelineState,
    element_size: usize,
    data_size: usize,
    device: &ID3D11Device,
) -> Option<(ID3D11Buffer, usize, [Option<ID3D11ShaderResourceView>; 1])> {
    if pipeline.buffer_size >= data_size {
        return None;
    }
    println!("buffer too small: {} < {}", pipeline.buffer_size, data_size);
    let buffer_size = data_size.next_power_of_two();
    println!("New size: {}", buffer_size);
    let buffer = create_buffer(device, element_size, buffer_size).unwrap();
    let view = create_buffer_view(device, &buffer).unwrap();
    Some((buffer, buffer_size, view))
}

fn update_pipeline(
    pipeline: &mut PipelineState,
    input: (ID3D11Buffer, usize, [Option<ID3D11ShaderResourceView>; 1]),
) {
    pipeline.buffer = input.0;
    pipeline.buffer_size = input.1;
    pipeline.view = input.2;
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

fn draw_normal(
    device_context: &ID3D11DeviceContext,
    pipeline: &PipelineState,
    viewport: &[D3D11_VIEWPORT],
    global_params: &[Option<ID3D11Buffer>],
    topology: D3D_PRIMITIVE_TOPOLOGY,
    vertex_count: u32,
    instance_count: u32,
) -> Result<()> {
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

fn draw_with_texture(
    device_context: &ID3D11DeviceContext,
    pipeline: &PipelineState,
    texture: &[Option<ID3D11ShaderResourceView>],
    viewport: &[D3D11_VIEWPORT],
    global_params: &[Option<ID3D11Buffer>],
    sampler: &[Option<ID3D11SamplerState>],
    instance_count: u32,
) -> Result<()> {
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

        device_context.DrawInstanced(4, instance_count, 0, 0);
    }
    Ok(())
}

const BUFFER_COUNT: usize = 3;
