use std::{collections::HashMap, hash::BuildHasherDefault, sync::Arc};

use ::util::ResultExt;
use anyhow::Result;
use collections::FxHasher;
use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::{
            Direct3D::{
                Fxc::{D3DCompileFromFile, D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION},
                ID3DBlob, D3D11_SRV_DIMENSION_BUFFER, D3D_DRIVER_TYPE_UNKNOWN,
                D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
                D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST, D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            },
            Direct3D11::{
                D3D11CreateDevice, ID3D11Buffer, ID3D11Device, ID3D11DeviceContext,
                ID3D11InputLayout, ID3D11PixelShader, ID3D11RenderTargetView,
                ID3D11ShaderResourceView, ID3D11Texture2D, ID3D11VertexShader,
                D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_FLAG, D3D11_BIND_SHADER_RESOURCE,
                D3D11_BIND_VERTEX_BUFFER, D3D11_BUFFER_DESC, D3D11_CPU_ACCESS_WRITE,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG,
                D3D11_INPUT_ELEMENT_DESC, D3D11_INPUT_PER_VERTEX_DATA, D3D11_MAP_WRITE_DISCARD,
                D3D11_RESOURCE_MISC_BUFFER_STRUCTURED, D3D11_SDK_VERSION,
                D3D11_SHADER_RESOURCE_VIEW_DESC, D3D11_SHADER_RESOURCE_VIEW_DESC_0,
                D3D11_SUBRESOURCE_DATA, D3D11_USAGE_DYNAMIC, D3D11_USAGE_IMMUTABLE, D3D11_VIEWPORT,
            },
            DirectComposition::{
                DCompositionCreateDevice, DCompositionCreateDevice2, IDCompositionDesktopDevice,
                IDCompositionDevice, IDCompositionSurface, IDCompositionTarget,
                IDCompositionVisual, IDCompositionVisual2,
            },
            Dxgi::{
                Common::{
                    DXGI_ALPHA_MODE_IGNORE, DXGI_ALPHA_MODE_PREMULTIPLIED,
                    DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_R32G32_FLOAT, DXGI_FORMAT_UNKNOWN,
                    DXGI_SAMPLE_DESC,
                },
                CreateDXGIFactory2, IDXGIAdapter1, IDXGIDevice, IDXGIFactory6, IDXGISwapChain1,
                DXGI_CREATE_FACTORY_DEBUG, DXGI_GPU_PREFERENCE_MINIMUM_POWER,
                DXGI_MWA_NO_ALT_ENTER, DXGI_SCALING_STRETCH, DXGI_SWAP_CHAIN_DESC1,
                DXGI_SWAP_EFFECT_FLIP_DISCARD, DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
                DXGI_USAGE_RENDER_TARGET_OUTPUT,
            },
        },
        UI::WindowsAndMessaging::GetClientRect,
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
    shadow_pipeline: PipelineState,
    quad_pipeline: PipelineState,
    raster_paths_pipeline: PipelineStateEx,
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
        self.update_buffers().log_err();
        unsafe {
            self.context
                .context
                .RSSetViewports(Some(&self.context.viewport));
            self.context
                .context
                .OMSetRenderTargets(Some(&self.context.back_buffer), None);
            self.context.context.ClearRenderTargetView(
                self.context.back_buffer[0].as_ref().unwrap(),
                &[0.0, 0.2, 0.4, 0.6],
            );
        }
        self.draw_primitives(scene);
        unsafe { self.context.swap_chain.Present(0, 0).ok().log_err() };
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) {
        // TODO:
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

    fn draw_primitives(&mut self, scene: &Scene) {
        let Some(path_tiles) = self.rasterize_paths(scene.paths()) else {
            log::error!("failed to rasterize {} paths", scene.paths().len());
            return;
        };
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
    }

    // TODO:
    fn draw_shadows(&mut self, shadows: &[Shadow]) -> bool {
        if shadows.is_empty() {
            return true;
        }
        unsafe {
            {
                let mut resource = std::mem::zeroed();
                self.context
                    .context
                    .Map(
                        &self.render.shadow_pipeline.buffer,
                        0,
                        D3D11_MAP_WRITE_DISCARD,
                        0,
                        Some(&mut resource),
                    )
                    .unwrap();
                std::ptr::copy_nonoverlapping(
                    shadows.as_ptr(),
                    resource.pData as *mut Shadow,
                    shadows.len(),
                );
                self.context
                    .context
                    .Unmap(&self.render.shadow_pipeline.buffer, 0);
                self.context
                    .context
                    .VSSetShaderResources(1, Some(&self.render.shadow_pipeline.view));
                self.context
                    .context
                    .PSSetShaderResources(1, Some(&self.render.shadow_pipeline.view));
            }
            self.context
                .context
                .IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            self.context
                .context
                .RSSetViewports(Some(&self.context.viewport));
            self.context
                .context
                .VSSetShader(&self.render.shadow_pipeline.vertex, None);
            self.context
                .context
                .PSSetShader(&self.render.shadow_pipeline.fragment, None);
            self.context
                .context
                .VSSetConstantBuffers(0, Some(&self.render.global_params_buffer));
            self.context
                .context
                .PSSetConstantBuffers(0, Some(&self.render.global_params_buffer));
            self.context
                .context
                .DrawInstanced(4, shadows.len() as u32, 0, 0);
        }
        true
    }

    // TODO:
    fn draw_quads(&mut self, quads: &[Quad]) -> bool {
        if quads.is_empty() {
            return true;
        }
        unsafe {
            {
                let mut resource = std::mem::zeroed();
                self.context
                    .context
                    .Map(
                        &self.render.quad_pipeline.buffer,
                        0,
                        D3D11_MAP_WRITE_DISCARD,
                        0,
                        Some(&mut resource),
                    )
                    .unwrap();
                std::ptr::copy_nonoverlapping(
                    quads.as_ptr(),
                    resource.pData as *mut Quad,
                    quads.len(),
                );
                self.context
                    .context
                    .Unmap(&self.render.quad_pipeline.buffer, 0);
                self.context
                    .context
                    .VSSetShaderResources(1, Some(&self.render.quad_pipeline.view));
                self.context
                    .context
                    .PSSetShaderResources(1, Some(&self.render.quad_pipeline.view));
            }
            self.context
                .context
                .IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP);
            self.context
                .context
                .RSSetViewports(Some(&self.context.viewport));
            self.context
                .context
                .VSSetShader(&self.render.quad_pipeline.vertex, None);
            self.context
                .context
                .PSSetShader(&self.render.quad_pipeline.fragment, None);
            self.context
                .context
                .VSSetConstantBuffers(0, Some(&self.render.global_params_buffer));
            // self.context.context.VSSetConstantBuffers(startslot, ppconstantbuffers)
            self.context
                .context
                .PSSetConstantBuffers(0, Some(&self.render.global_params_buffer));

            self.context
                .context
                .DrawInstanced(4, quads.len() as u32, 0, 0);
        }
        true
    }

    // TODO:
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
            let (texture, texture_size, rtv) = self.atlas.texture_info(texture_id);
            let globals = GlobalParams {
                viewport_size: [texture_size.width, texture_size.height],
                premultiplied_alpha: 0,
                _pad: 0,
            };
            let viewport = [D3D11_VIEWPORT {
                TopLeftX: 0.0,
                TopLeftY: 0.0,
                Width: texture_size.width,
                Height: texture_size.height,
                MinDepth: 0.0,
                MaxDepth: 1.0,
            }];
            unsafe {
                let mut resource = std::mem::zeroed();
                self.context
                    .context
                    .Map(
                        &self.render.raster_paths_pipeline.vertex_buffer,
                        0,
                        D3D11_MAP_WRITE_DISCARD,
                        0,
                        Some(&mut resource),
                    )
                    .unwrap();
                std::ptr::copy_nonoverlapping(
                    vertices.as_ptr(),
                    resource.pData as _,
                    vertices.len(),
                );
                self.context
                    .context
                    .Unmap(&self.render.raster_paths_pipeline.vertex_buffer, 0);

                self.context.context.RSSetViewports(Some(&viewport));
                self.context.context.OMSetRenderTargets(Some(&rtv), None);
                self.context
                    .context
                    .ClearRenderTargetView(rtv[0].as_ref().unwrap(), &[0., 0., 0., 1.]);

                self.context
                    .context
                    .IASetPrimitiveTopology(D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
                self.context
                    .context
                    .IASetInputLayout(&self.render.raster_paths_pipeline.layout);
                self.context.context.IASetVertexBuffers(
                    0,
                    1,
                    Some(&Some(
                        self.render.raster_paths_pipeline.vertex_buffer.clone(),
                    )),
                    Some(&(std::mem::size_of::<PathVertex<ScaledPixels>>() as u32)),
                    Some(&0),
                );
                self.context
                    .context
                    .VSSetShader(&self.render.raster_paths_pipeline.vertex, None);
                self.context
                    .context
                    .VSSetConstantBuffers(1, Some(&self.render.global_params_buffer));
                self.context
                    .context
                    .PSSetShader(&self.render.raster_paths_pipeline.fragment, None);
                self.context
                    .context
                    .PSSetConstantBuffers(1, Some(&self.render.global_params_buffer));
                self.context
                    .context
                    .DrawInstanced(vertices.len() as u32, 1, 0, 0);
            }
        }

        Some(tiles)
    }

    // TODO:
    fn draw_paths(
        &mut self,
        paths: &[Path<ScaledPixels>],
        path_tiles: &HashMap<PathId, AtlasTile>,
    ) -> bool {
        if paths.is_empty() {
            return true;
        }
        true
    }

    // TODO:
    fn draw_underlines(&mut self, underlines: &[Underline]) -> bool {
        if underlines.is_empty() {
            return true;
        }
        true
    }

    // TODO:
    fn draw_monochrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[MonochromeSprite],
    ) -> bool {
        if sprites.is_empty() {
            return true;
        }
        true
    }

    // TODO:
    fn draw_polychrome_sprites(
        &mut self,
        texture_id: AtlasTextureId,
        sprites: &[PolychromeSprite],
    ) -> bool {
        if sprites.is_empty() {
            return true;
        }
        true
    }

    // TODO:
    fn draw_surfaces(&mut self, surfaces: &[Surface]) -> bool {
        if surfaces.is_empty() {
            return true;
        }
        true
    }

    fn update_buffers(&self) -> Result<()> {
        unsafe {
            let buffer = self.render.global_params_buffer[0].as_ref().unwrap();
            let mut resource = std::mem::zeroed();
            self.context
                .context
                .Map(buffer, 0, D3D11_MAP_WRITE_DISCARD, 0, Some(&mut resource))?;
            let globals = resource.pData as *mut GlobalParams;
            (*globals).viewport_size = [self.size.width.0 as f32, self.size.height.0 as f32];
            (*globals).premultiplied_alpha = 1;
            self.context.context.Unmap(buffer, 0);
        }
        Ok(())
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
        // let swap_chain = get_swap_chain(&dxgi_factory, &device)?;
        let swap_chain = get_swap_chain(&dxgi_factory, &device, hwnd)?;
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
                // ByteWidth must be a multiple of 16, per the docs
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

        let shadow_pipeline = unsafe {
            let vertex_shader_blob = build_shader_blob("shadow_vertex", "vs_5_0").unwrap();
            let vertex = create_vertex_shader(device, &vertex_shader_blob)?;
            let fragment_shader_blob = build_shader_blob("shadow_fragment", "ps_5_0").unwrap();
            let fragment = create_fragment_shader(device, &fragment_shader_blob)?;
            let buffer = {
                let desc = D3D11_BUFFER_DESC {
                    // ByteWidth: BUFFER_SIZE as u32,
                    ByteWidth: std::mem::size_of::<Shadow>() as u32 * 1024,
                    Usage: D3D11_USAGE_DYNAMIC,
                    BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                    CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                    MiscFlags: D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0 as u32,
                    StructureByteStride: std::mem::size_of::<Shadow>() as u32,
                };
                let mut buffer = None;
                device.CreateBuffer(&desc, None, Some(&mut buffer))?;
                buffer.unwrap()
            };
            let view = {
                let mut view = None;
                device
                    .CreateShaderResourceView(&buffer, None, Some(&mut view))
                    .unwrap();
                [view]
            };
            PipelineState {
                // layout,
                vertex,
                fragment,
                buffer,
                view,
            }
        };

        let quad_pipeline = unsafe {
            let vertex_shader_blob = build_shader_blob("quad_vertex", "vs_5_0").unwrap();
            let vertex = create_vertex_shader(device, &vertex_shader_blob)?;
            let fragment_shader_blob = build_shader_blob("quad_fragment", "ps_5_0").unwrap();
            let fragment = create_fragment_shader(device, &fragment_shader_blob)?;
            let buffer = {
                let desc = D3D11_BUFFER_DESC {
                    ByteWidth: std::mem::size_of::<Quad>() as u32 * 1024,
                    Usage: D3D11_USAGE_DYNAMIC,
                    BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                    CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                    MiscFlags: D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0 as u32,
                    StructureByteStride: std::mem::size_of::<Quad>() as u32,
                };
                let mut buffer = None;
                device.CreateBuffer(&desc, None, Some(&mut buffer))?;
                buffer.unwrap()
            };
            let view = {
                let mut view = None;
                device
                    .CreateShaderResourceView(&buffer, None, Some(&mut view))
                    .unwrap();
                [view]
            };
            PipelineState {
                // layout,
                vertex,
                fragment,
                buffer,
                view,
            }
        };

        let raster_paths_pipeline = unsafe {
            let vertex_shader_blob =
                build_shader_blob("path_rasterization_vertex", "vs_5_0").unwrap();
            let layout = {
                let desc = D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: windows::core::s!("POSITION"),
                    SemanticIndex: 0,
                    Format: DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                };
                let mut input_layout = None;
                device
                    .CreateInputLayout(
                        &[desc],
                        std::slice::from_raw_parts(
                            vertex_shader_blob.GetBufferPointer() as *mut u8,
                            vertex_shader_blob.GetBufferSize(),
                        ),
                        Some(&mut input_layout),
                    )
                    .unwrap();
                input_layout.unwrap()
            };
            let vertex = create_vertex_shader(device, &vertex_shader_blob)?;
            let fragment_shader_blob =
                build_shader_blob("path_rasterization_fragment", "ps_5_0").unwrap();
            let fragment = create_fragment_shader(device, &fragment_shader_blob)?;
            let vertex_buffer = {
                let desc = D3D11_BUFFER_DESC {
                    ByteWidth: std::mem::size_of::<PathVertex<ScaledPixels>>() as u32 * 1024,
                    Usage: D3D11_USAGE_DYNAMIC,
                    BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
                    CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                    ..Default::default()
                };
                let mut buffer = None;
                device.CreateBuffer(&desc, None, Some(&mut buffer))?;
                buffer.unwrap()
            };
            let buffer = {
                let desc = D3D11_BUFFER_DESC {
                    ByteWidth: std::mem::size_of::<PathVertex<ScaledPixels>>() as u32 * 1024,
                    Usage: D3D11_USAGE_DYNAMIC,
                    BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
                    CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
                    MiscFlags: D3D11_RESOURCE_MISC_BUFFER_STRUCTURED.0 as u32,
                    StructureByteStride: std::mem::size_of::<Quad>() as u32,
                };
                let mut buffer = None;
                device.CreateBuffer(&desc, None, Some(&mut buffer))?;
                buffer.unwrap()
            };
            let view = {
                let mut view = None;
                device
                    .CreateShaderResourceView(&buffer, None, Some(&mut view))
                    .unwrap();
                [view]
            };
            PipelineStateEx {
                layout,
                vertex_buffer,
                vertex,
                fragment,
                buffer,
                view,
            }
        };

        Ok(Self {
            global_params_buffer,
            shadow_pipeline,
            quad_pipeline,
            raster_paths_pipeline,
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
    // layout: ID3D11InputLayout,
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
    buffer: ID3D11Buffer,
    view: [Option<ID3D11ShaderResourceView>; 1],
}

struct PipelineStateEx {
    vertex_buffer: ID3D11Buffer,
    layout: ID3D11InputLayout,
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
    buffer: ID3D11Buffer,
    view: [Option<ID3D11ShaderResourceView>; 1],
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

// fn get_swap_chain(dxgi_factory: &IDXGIFactory6, device: &ID3D11Device) -> Result<IDXGISwapChain1> {
//     let desc = DXGI_SWAP_CHAIN_DESC1 {
//         Width: 1,
//         Height: 1,
//         Format: DXGI_FORMAT_B8G8R8A8_UNORM,
//         Stereo: false.into(),
//         SampleDesc: DXGI_SAMPLE_DESC {
//             Count: 1,
//             Quality: 0,
//         },
//         BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
//         BufferCount: BUFFER_COUNT as u32,
//         // Composition SwapChains only support the DXGI_SCALING_STRETCH Scaling.
//         Scaling: DXGI_SCALING_STRETCH,
//         SwapEffect: DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
//         // Premultiplied alpha is the only supported format by composition swapchain.
//         AlphaMode: DXGI_ALPHA_MODE_PREMULTIPLIED,
//         Flags: 0,
//     };
//     Ok(unsafe { dxgi_factory.CreateSwapChainForComposition(device, &desc, None)? })
// }

fn get_swap_chain(
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
        // Composition SwapChains only support the DXGI_SCALING_STRETCH Scaling.
        Scaling: DXGI_SCALING_STRETCH,
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

const BUFFER_COUNT: usize = 3;
const BUFFER_SIZE: usize = 2 * 1024 * 1024;
