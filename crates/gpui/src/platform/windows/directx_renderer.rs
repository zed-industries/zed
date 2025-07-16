use std::{mem::ManuallyDrop, sync::Arc};

use ::util::ResultExt;
use anyhow::{Context, Result};
// #[cfg(not(feature = "enable-renderdoc"))]
// use windows::Win32::Graphics::DirectComposition::*;
use windows::{
    Win32::{
        Foundation::{HMODULE, HWND},
        Graphics::{
            Direct3D::*,
            Direct3D11::*,
            Dxgi::{Common::*, *},
        },
    },
    core::*,
};

use crate::*;

const RENDER_TARGET_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM;
const BACK_BUFFER_FORMAT: DXGI_FORMAT = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB;

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
    swap_chain: ManuallyDrop<IDXGISwapChain1>,
    render_target: ManuallyDrop<ID3D11Texture2D>,
    render_target_view: [Option<ID3D11RenderTargetView>; 1],
    msaa_target: ID3D11Texture2D,
    msaa_view: ID3D11RenderTargetView,
    viewport: [D3D11_VIEWPORT; 1],
    // #[cfg(not(feature = "enable-renderdoc"))]
    // direct_composition: DirectComposition,
}

struct DirectXRenderPipelines {
    shadow_pipeline: PipelineState<Shadow>,
    quad_pipeline: PipelineState<Quad>,
    paths_pipeline: PathsPipelineState,
    underline_pipeline: PipelineState<Underline>,
    mono_sprites: PipelineState<MonochromeSprite>,
    poly_sprites: PipelineState<PolychromeSprite>,
}

struct DirectXGlobalElements {
    global_params_buffer: [Option<ID3D11Buffer>; 1],
    sampler: [Option<ID3D11SamplerState>; 1],
    blend_state: ID3D11BlendState,
}

#[repr(C)]
struct DrawInstancedIndirectArgs {
    vertex_count_per_instance: u32,
    instance_count: u32,
    start_vertex_location: u32,
    start_instance_location: u32,
}

// #[cfg(not(feature = "enable-renderdoc"))]
// struct DirectComposition {
//     comp_device: IDCompositionDevice,
//     comp_target: IDCompositionTarget,
//     comp_visual: IDCompositionVisual,
// }

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
    pub(crate) fn new(devices: &DirectXDevices, hwnd: HWND, transparent: bool) -> Result<Self> {
        let atlas = Arc::new(DirectXAtlas::new(
            devices.device.clone(),
            devices.device_context.clone(),
        ));
        let context = DirectXContext::new(devices, hwnd, transparent)?;
        let globals = DirectXGlobalElements::new(&devices.device)?;
        let pipelines = DirectXRenderPipelines::new(&devices.device)?;
        Ok(DirectXRenderer {
            atlas,
            devices: devices.clone(),
            context,
            globals,
            pipelines,
            hwnd,
            transparent,
        })
    }

    pub(crate) fn sprite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    fn pre_draw(&self) -> Result<()> {
        update_buffer(
            &self.devices.device_context,
            self.globals.global_params_buffer[0].as_ref().unwrap(),
            &[GlobalParams {
                viewport_size: [
                    self.context.viewport[0].Width,
                    self.context.viewport[0].Height,
                ],
                ..Default::default()
            }],
        )?;
        unsafe {
            self.devices
                .device_context
                .ClearRenderTargetView(&self.context.msaa_view, &[0.0; 4]);
            self.devices
                .device_context
                .OMSetRenderTargets(Some(&[Some(self.context.msaa_view.clone())]), None);
            self.devices
                .device_context
                .RSSetViewports(Some(&self.context.viewport));
            self.devices.device_context.OMSetBlendState(
                &self.globals.blend_state,
                None,
                0xFFFFFFFF,
            );
        }
        Ok(())
    }

    pub(crate) fn draw(&mut self, scene: &Scene) -> Result<()> {
        // pre_draw(
        //     &self.devices.device_context,
        //     &self.globals.global_params_buffer,
        //     &self.context.viewport,
        //     &self.context.back_buffer,
        //     [0.0, 0.0, 0.0, 0.0],
        //     &self.globals.blend_state,
        // )?;
        println!("Pre-draw: {:?}", self.context.render_target_view);
        self.pre_draw()?;
        for batch in scene.batches() {
            match batch {
                PrimitiveBatch::Shadows(shadows) => self.draw_shadows(shadows),
                PrimitiveBatch::Quads(quads) => self.draw_quads(quads),
                PrimitiveBatch::Paths(paths) => self.draw_paths(paths),
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
        unsafe {
            self.devices.device_context.ResolveSubresource(
                &*self.context.render_target,
                0,
                &self.context.msaa_target,
                0,
                BACK_BUFFER_FORMAT,
            );
            self.devices
                .device_context
                .OMSetRenderTargets(Some(&self.context.render_target_view), None);
            self.context.swap_chain.Present(0, DXGI_PRESENT(0)).ok()?;
        }
        Ok(())
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) -> Result<()> {
        println!("Resize: {:?}", self.context.render_target_view);
        unsafe {
            self.devices.device_context.OMSetRenderTargets(None, None);
            ManuallyDrop::drop(&mut self.context.render_target);
        }
        drop(self.context.render_target_view[0].take().unwrap());
        unsafe {
            self.context
                .swap_chain
                .ResizeBuffers(
                    BUFFER_COUNT as u32,
                    new_size.width.0 as u32,
                    new_size.height.0 as u32,
                    RENDER_TARGET_FORMAT,
                    DXGI_SWAP_CHAIN_FLAG(0),
                )
                .unwrap();
        }
        // let backbuffer = set_render_target_view(
        //     &self.context.swap_chain,
        //     &self.devices.device,
        //     &self.devices.device_context,
        // )?;
        let (render_target, render_target_view) =
            create_render_target_and_its_view(&self.context.swap_chain, &self.devices.device)
                .unwrap();
        self.context.render_target = render_target;
        self.context.render_target_view = render_target_view;
        unsafe {
            self.devices
                .device_context
                .OMSetRenderTargets(Some(&self.context.render_target_view), None);
        }

        let (msaa_target, msaa_view) = create_msaa_target_and_its_view(
            &self.devices.device,
            new_size.width.0 as u32,
            new_size.height.0 as u32,
        )?;
        self.context.msaa_target = msaa_target;
        self.context.msaa_view = msaa_view;

        self.context.viewport = set_viewport(
            &self.devices.device_context,
            new_size.width.0 as f32,
            new_size.height.0 as f32,
        );
        Ok(())
    }

    // #[cfg(not(feature = "enable-renderdoc"))]
    // pub(crate) fn update_transparency(
    //     &mut self,
    //     background_appearance: WindowBackgroundAppearance,
    // ) -> Result<()> {
    //     // We only support setting `Transparent` and `Opaque` for now.
    //     match background_appearance {
    //         WindowBackgroundAppearance::Opaque => {
    //             if self.transparent {
    //                 return Err(anyhow::anyhow!(
    //                     "Set opaque backgroud from transparent background, a restart is required. Or, you can open a new window."
    //                 ));
    //             }
    //         }
    //         WindowBackgroundAppearance::Transparent | WindowBackgroundAppearance::Blurred => {
    //             if !self.transparent {
    //                 return Err(anyhow::anyhow!(
    //                     "Set transparent backgroud from opaque background, a restart is required. Or, you can open a new window."
    //                 ));
    //             }
    //         }
    //     }
    //     Ok(())
    // }

    // #[cfg(feature = "enable-renderdoc")]
    pub(crate) fn update_transparency(
        &mut self,
        background_appearance: WindowBackgroundAppearance,
    ) -> Result<()> {
        let transparent = background_appearance != WindowBackgroundAppearance::Opaque;
        if self.transparent == transparent {
            return Ok(());
        }
        self.transparent = transparent;
        // unsafe {
        //     // recreate the swapchain
        //     self.devices.device_context.OMSetRenderTargets(None, None);
        //     drop(self.context.back_buffer[0].take().unwrap());
        //     ManuallyDrop::drop(&mut self.context.swap_chain);
        //     self.context.swap_chain = create_swap_chain_default(
        //         &self.devices.dxgi_factory,
        //         &self.devices.device,
        //         self.hwnd,
        //         transparent,
        //     )?;
        //     self.context.back_buffer = [Some(set_render_target_view(
        //         &self.context.swap_chain,
        //         &self.devices.device,
        //         &self.devices.device_context,
        //     )?)];
        //     self.context.viewport = set_viewport(
        //         &self.devices.device_context,
        //         self.context.viewport[0].Width,
        //         self.context.viewport[0].Height,
        //     );
        //     set_rasterizer_state(&self.devices.device, &self.devices.device_context)?;
        // }
        Ok(())
    }

    fn draw_shadows(&mut self, shadows: &[Shadow]) -> Result<()> {
        if shadows.is_empty() {
            return Ok(());
        }
        self.pipelines.shadow_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            shadows,
        )?;
        self.pipelines.shadow_pipeline.draw(
            &self.devices.device_context,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            shadows.len() as u32,
        )
    }

    fn draw_quads(&mut self, quads: &[Quad]) -> Result<()> {
        if quads.is_empty() {
            return Ok(());
        }
        self.pipelines.quad_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            quads,
        )?;
        self.pipelines.quad_pipeline.draw(
            &self.devices.device_context,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            quads.len() as u32,
        )
    }

    fn draw_paths(&mut self, paths: &[Path<ScaledPixels>]) -> Result<()> {
        if paths.is_empty() {
            return Ok(());
        }
        let mut vertices = Vec::new();
        let mut sprites = Vec::with_capacity(paths.len());
        let mut draw_indirect_commands = Vec::with_capacity(paths.len());
        let mut start_vertex_location = 0;
        for (i, path) in paths.iter().enumerate() {
            draw_indirect_commands.push(DrawInstancedIndirectArgs {
                vertex_count_per_instance: path.vertices.len() as u32,
                instance_count: 1,
                start_vertex_location,
                start_instance_location: i as u32,
            });
            start_vertex_location += path.vertices.len() as u32;

            vertices.extend(path.vertices.iter().map(|v| PathVertex {
                xy_position: v.xy_position,
                content_mask: ContentMask {
                    bounds: path.content_mask.bounds,
                },
            }));

            sprites.push(PathSprite {
                bounds: path.bounds,
                color: path.color,
            });
        }

        self.pipelines.paths_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            &sprites,
            &vertices,
            &draw_indirect_commands,
        )?;
        self.pipelines.paths_pipeline.draw(
            &self.devices.device_context,
            paths.len(),
            &self.context.viewport,
            &self.globals.global_params_buffer,
        )
    }

    fn draw_underlines(&mut self, underlines: &[Underline]) -> Result<()> {
        if underlines.is_empty() {
            return Ok(());
        }
        self.pipelines.underline_pipeline.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            underlines,
        )?;
        self.pipelines.underline_pipeline.draw(
            &self.devices.device_context,
            &self.context.viewport,
            &self.globals.global_params_buffer,
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
        self.pipelines.mono_sprites.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            sprites,
        )?;
        let texture_view = self.atlas.get_texture_view(texture_id);
        self.pipelines.mono_sprites.draw_with_texture(
            &self.devices.device_context,
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
        self.pipelines.poly_sprites.update_buffer(
            &self.devices.device,
            &self.devices.device_context,
            sprites,
        )?;
        let texture_view = self.atlas.get_texture_view(texture_id);
        self.pipelines.poly_sprites.draw_with_texture(
            &self.devices.device_context,
            &texture_view,
            &self.context.viewport,
            &self.globals.global_params_buffer,
            &self.globals.sampler,
            sprites.len() as u32,
        )
    }

    fn draw_surfaces(&mut self, surfaces: &[PaintSurface]) -> Result<()> {
        if surfaces.is_empty() {
            return Ok(());
        }
        Ok(())
    }
}

impl DirectXContext {
    pub fn new(devices: &DirectXDevices, hwnd: HWND, transparent: bool) -> Result<Self> {
        // #[cfg(not(feature = "enable-renderdoc"))]
        // let swap_chain = create_swap_chain(&devices.dxgi_factory, &devices.device, transparent)?;
        // #[cfg(feature = "enable-renderdoc")]
        let swap_chain =
            create_swap_chain_default(&devices.dxgi_factory, &devices.device, hwnd, transparent)?;
        // #[cfg(not(feature = "enable-renderdoc"))]
        // let direct_composition = DirectComposition::new(&devices.dxgi_device, hwnd)?;
        // #[cfg(not(feature = "enable-renderdoc"))]
        // direct_composition.set_swap_chain(&swap_chain)?;
        let (render_target, render_target_view) =
            create_render_target_and_its_view(&swap_chain, &devices.device)?;
        // let back_buffer = [Some(set_render_target_view(
        //     &swap_chain,
        //     &devices.device,
        //     &devices.device_context,
        // )?)];
        let (msaa_target, msaa_view) = create_msaa_target_and_its_view(&devices.device, 1, 1)?;
        let viewport = set_viewport(&devices.device_context, 1.0, 1.0);
        unsafe {
            devices
                .device_context
                .OMSetRenderTargets(Some(&render_target_view), None);
        }
        set_rasterizer_state(&devices.device, &devices.device_context)?;

        Ok(Self {
            swap_chain,
            render_target,
            render_target_view,
            msaa_target,
            msaa_view,
            viewport,
            // #[cfg(not(feature = "enable-renderdoc"))]
            // direct_composition,
        })
    }
}

impl DirectXRenderPipelines {
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        let shadow_pipeline = PipelineState::new(
            device,
            "shadow_pipeline",
            "shadow_vertex",
            "shadow_fragment",
            4,
        )?;
        let quad_pipeline =
            PipelineState::new(device, "quad_pipeline", "quad_vertex", "quad_fragment", 64)?;
        let paths_pipeline = PathsPipelineState::new(device)?;
        let underline_pipeline = PipelineState::new(
            device,
            "underline_pipeline",
            "underline_vertex",
            "underline_fragment",
            4,
        )?;
        let mono_sprites = PipelineState::new(
            device,
            "monochrome_sprite_pipeline",
            "monochrome_sprite_vertex",
            "monochrome_sprite_fragment",
            512,
        )?;
        let poly_sprites = PipelineState::new(
            device,
            "polychrome_sprite_pipeline",
            "polychrome_sprite_vertex",
            "polychrome_sprite_fragment",
            16,
        )?;

        Ok(Self {
            shadow_pipeline,
            quad_pipeline,
            paths_pipeline,
            underline_pipeline,
            mono_sprites,
            poly_sprites,
        })
    }
}

// #[cfg(not(feature = "enable-renderdoc"))]
// impl DirectComposition {
//     pub fn new(dxgi_device: &IDXGIDevice, hwnd: HWND) -> Result<Self> {
//         let comp_device = get_comp_device(&dxgi_device)?;
//         let comp_target = unsafe { comp_device.CreateTargetForHwnd(hwnd, true) }?;
//         let comp_visual = unsafe { comp_device.CreateVisual() }?;

//         Ok(Self {
//             comp_device,
//             comp_target,
//             comp_visual,
//         })
//     }

//     pub fn set_swap_chain(&self, swap_chain: &IDXGISwapChain1) -> Result<()> {
//         unsafe {
//             self.comp_visual.SetContent(swap_chain)?;
//             self.comp_target.SetRoot(&self.comp_visual)?;
//             self.comp_device.Commit()?;
//         }
//         Ok(())
//     }
// }

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

        Ok(Self {
            global_params_buffer,
            sampler,
            blend_state,
        })
    }
}

#[derive(Debug, Default)]
#[repr(C)]
struct GlobalParams {
    viewport_size: [f32; 2],
    _pad: u64,
}

struct PipelineState<T> {
    label: &'static str,
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
    buffer: ID3D11Buffer,
    buffer_size: usize,
    view: [Option<ID3D11ShaderResourceView>; 1],
    _marker: std::marker::PhantomData<T>,
}

struct PathsPipelineState {
    vertex: ID3D11VertexShader,
    fragment: ID3D11PixelShader,
    buffer: ID3D11Buffer,
    buffer_size: usize,
    vertex_buffer: Option<ID3D11Buffer>,
    vertex_buffer_size: usize,
    indirect_draw_buffer: ID3D11Buffer,
    indirect_buffer_size: usize,
    input_layout: ID3D11InputLayout,
    view: [Option<ID3D11ShaderResourceView>; 1],
}

impl<T> PipelineState<T> {
    fn new(
        device: &ID3D11Device,
        label: &'static str,
        vertex_entry: &str,
        fragment_entry: &str,
        buffer_size: usize,
    ) -> Result<Self> {
        let vertex = {
            let shader_blob = shader_resources::build_shader_blob(vertex_entry, "vs_5_0")?;
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    shader_blob.GetBufferPointer() as *mut u8,
                    shader_blob.GetBufferSize(),
                )
            };
            create_vertex_shader(device, bytes)?
        };
        let fragment = {
            let shader_blob = shader_resources::build_shader_blob(fragment_entry, "ps_5_0")?;
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    shader_blob.GetBufferPointer() as *mut u8,
                    shader_blob.GetBufferSize(),
                )
            };
            create_fragment_shader(device, bytes)?
        };
        let buffer = create_buffer(device, std::mem::size_of::<T>(), buffer_size)?;
        let view = create_buffer_view(device, &buffer)?;

        Ok(PipelineState {
            label,
            vertex,
            fragment,
            buffer,
            buffer_size,
            view,
            _marker: std::marker::PhantomData,
        })
    }

    fn update_buffer(
        &mut self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
        data: &[T],
    ) -> Result<()> {
        if self.buffer_size < data.len() {
            let new_buffer_size = data.len().next_power_of_two();
            log::info!(
                "Updating {} buffer size from {} to {}",
                self.label,
                self.buffer_size,
                new_buffer_size
            );
            let buffer = create_buffer(device, std::mem::size_of::<T>(), new_buffer_size)?;
            let view = create_buffer_view(device, &buffer)?;
            self.buffer = buffer;
            self.view = view;
            self.buffer_size = new_buffer_size;
        }
        update_buffer(device_context, &self.buffer, data)
    }

    fn draw(
        &self,
        device_context: &ID3D11DeviceContext,
        viewport: &[D3D11_VIEWPORT],
        global_params: &[Option<ID3D11Buffer>],
        instance_count: u32,
    ) -> Result<()> {
        set_pipeline_state(
            device_context,
            &self.view,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            viewport,
            &self.vertex,
            &self.fragment,
            global_params,
        );
        unsafe {
            device_context.DrawInstanced(4, instance_count, 0, 0);
        }
        Ok(())
    }

    fn draw_with_texture(
        &self,
        device_context: &ID3D11DeviceContext,
        texture: &[Option<ID3D11ShaderResourceView>],
        viewport: &[D3D11_VIEWPORT],
        global_params: &[Option<ID3D11Buffer>],
        sampler: &[Option<ID3D11SamplerState>],
        instance_count: u32,
    ) -> Result<()> {
        set_pipeline_state(
            device_context,
            &self.view,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
            viewport,
            &self.vertex,
            &self.fragment,
            global_params,
        );
        unsafe {
            device_context.PSSetSamplers(0, Some(sampler));
            device_context.VSSetShaderResources(0, Some(texture));
            device_context.PSSetShaderResources(0, Some(texture));

            device_context.DrawInstanced(4, instance_count, 0, 0);
        }
        Ok(())
    }
}

impl PathsPipelineState {
    fn new(device: &ID3D11Device) -> Result<Self> {
        let (vertex, vertex_shader) = {
            let shader_blob = shader_resources::build_shader_blob("paths_vertex", "vs_5_0")?;
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    shader_blob.GetBufferPointer() as *mut u8,
                    shader_blob.GetBufferSize(),
                )
            };
            (create_vertex_shader(device, bytes)?, shader_blob)
        };
        let fragment = {
            let shader_blob = shader_resources::build_shader_blob("paths_fragment", "ps_5_0")?;
            let bytes = unsafe {
                std::slice::from_raw_parts(
                    shader_blob.GetBufferPointer() as *mut u8,
                    shader_blob.GetBufferSize(),
                )
            };
            create_fragment_shader(device, bytes)?
        };
        let buffer = create_buffer(device, std::mem::size_of::<PathSprite>(), 32)?;
        let view = create_buffer_view(device, &buffer)?;
        let vertex_buffer = Some(create_buffer(
            device,
            std::mem::size_of::<PathVertex<ScaledPixels>>(),
            32,
        )?);
        let indirect_draw_buffer = create_indirect_draw_buffer(device, 32)?;
        // Create input layout
        let input_layout = unsafe {
            let shader_bytes = std::slice::from_raw_parts(
                vertex_shader.GetBufferPointer() as *const u8,
                vertex_shader.GetBufferSize(),
            );
            let mut layout = None;
            device.CreateInputLayout(
                &[
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: windows::core::s!("POSITION"),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 0,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: windows::core::s!("TEXCOORD"),
                        SemanticIndex: 0,
                        Format: DXGI_FORMAT_R32G32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 8,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                    D3D11_INPUT_ELEMENT_DESC {
                        SemanticName: windows::core::s!("TEXCOORD"),
                        SemanticIndex: 1,
                        Format: DXGI_FORMAT_R32G32_FLOAT,
                        InputSlot: 0,
                        AlignedByteOffset: 16,
                        InputSlotClass: D3D11_INPUT_PER_VERTEX_DATA,
                        InstanceDataStepRate: 0,
                    },
                ],
                shader_bytes,
                Some(&mut layout),
            )?;
            layout.unwrap()
        };

        Ok(Self {
            vertex,
            fragment,
            buffer,
            buffer_size: 32,
            vertex_buffer,
            vertex_buffer_size: 32,
            indirect_draw_buffer,
            indirect_buffer_size: 32,
            input_layout,
            view,
        })
    }

    fn update_buffer(
        &mut self,
        device: &ID3D11Device,
        device_context: &ID3D11DeviceContext,
        buffer_data: &[PathSprite],
        vertices_data: &[PathVertex<ScaledPixels>],
        draw_commands: &[DrawInstancedIndirectArgs],
    ) -> Result<()> {
        if self.buffer_size < buffer_data.len() {
            let new_buffer_size = buffer_data.len().next_power_of_two();
            log::info!(
                "Updating Paths Pipeline buffer size from {} to {}",
                self.buffer_size,
                new_buffer_size
            );
            let buffer = create_buffer(device, std::mem::size_of::<PathSprite>(), new_buffer_size)?;
            let view = create_buffer_view(device, &buffer)?;
            self.buffer = buffer;
            self.view = view;
            self.buffer_size = new_buffer_size;
        }
        update_buffer(device_context, &self.buffer, buffer_data)?;
        if self.vertex_buffer_size < vertices_data.len() {
            let new_vertex_buffer_size = vertices_data.len().next_power_of_two();
            log::info!(
                "Updating Paths Pipeline vertex buffer size from {} to {}",
                self.vertex_buffer_size,
                new_vertex_buffer_size
            );
            let vertex_buffer = create_buffer(
                device,
                std::mem::size_of::<PathVertex<ScaledPixels>>(),
                new_vertex_buffer_size,
            )?;
            self.vertex_buffer = Some(vertex_buffer);
            self.vertex_buffer_size = new_vertex_buffer_size;
        }
        update_buffer(
            device_context,
            self.vertex_buffer.as_ref().unwrap(),
            vertices_data,
        )?;
        if self.indirect_buffer_size < draw_commands.len() {
            let new_indirect_buffer_size = draw_commands.len().next_power_of_two();
            log::info!(
                "Updating Paths Pipeline indirect buffer size from {} to {}",
                self.indirect_buffer_size,
                new_indirect_buffer_size
            );
            let indirect_draw_buffer =
                create_indirect_draw_buffer(device, new_indirect_buffer_size)?;
            self.indirect_draw_buffer = indirect_draw_buffer;
            self.indirect_buffer_size = new_indirect_buffer_size;
        }
        update_buffer(device_context, &self.indirect_draw_buffer, draw_commands)?;
        Ok(())
    }

    fn draw(
        &self,
        device_context: &ID3D11DeviceContext,
        count: usize,
        viewport: &[D3D11_VIEWPORT],
        global_params: &[Option<ID3D11Buffer>],
    ) -> Result<()> {
        set_pipeline_state(
            device_context,
            &self.view,
            D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            viewport,
            &self.vertex,
            &self.fragment,
            global_params,
        );
        unsafe {
            const STRIDE: u32 = std::mem::size_of::<PathVertex<ScaledPixels>>() as u32;
            device_context.IASetVertexBuffers(
                0,
                1,
                Some(&self.vertex_buffer),
                Some(&STRIDE),
                Some(&0),
            );
            device_context.IASetInputLayout(&self.input_layout);
        }
        for i in 0..count {
            unsafe {
                device_context.DrawInstancedIndirect(
                    &self.indirect_draw_buffer,
                    (i * std::mem::size_of::<DrawInstancedIndirectArgs>()) as u32,
                );
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[repr(C)]
struct PathSprite {
    bounds: Bounds<ScaledPixels>,
    color: Background,
}

impl Drop for DirectXContext {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.render_target);
            ManuallyDrop::drop(&mut self.swap_chain);
        }
    }
}

#[inline]
fn get_dxgi_factory() -> Result<IDXGIFactory6> {
    #[cfg(debug_assertions)]
    let factory_flag = DXGI_CREATE_FACTORY_DEBUG;
    #[cfg(not(debug_assertions))]
    let factory_flag = DXGI_CREATE_FACTORY_FLAGS::default();
    unsafe { Ok(CreateDXGIFactory2(factory_flag)?) }
}

fn get_adapter(dxgi_factory: &IDXGIFactory6) -> Result<IDXGIAdapter1> {
    for adapter_index in 0.. {
        let adapter: IDXGIAdapter1 = unsafe {
            dxgi_factory
                .EnumAdapterByGpuPreference(adapter_index, DXGI_GPU_PREFERENCE_MINIMUM_POWER)
        }?;
        {
            let desc = unsafe { adapter.GetDesc1() }?;
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
            HMODULE::default(),
            device_flags,
            Some(&[D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1]),
            D3D11_SDK_VERSION,
            device,
            None,
            context,
        )?
    })
}

// #[cfg(not(feature = "enable-renderdoc"))]
// fn get_comp_device(dxgi_device: &IDXGIDevice) -> Result<IDCompositionDevice> {
//     Ok(unsafe { DCompositionCreateDevice(dxgi_device)? })
// }

// fn create_swap_chain(
//     dxgi_factory: &IDXGIFactory6,
//     device: &ID3D11Device,
//     transparent: bool,
// ) -> Result<IDXGISwapChain1> {
//     let alpha_mode = if transparent {
//         DXGI_ALPHA_MODE_PREMULTIPLIED
//     } else {
//         DXGI_ALPHA_MODE_IGNORE
//     };
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
//         AlphaMode: alpha_mode,
//         Flags: 0,
//     };
//     Ok(unsafe { dxgi_factory.CreateSwapChainForComposition(device, &desc, None)? })
// }

// #[cfg(feature = "enable-renderdoc")]
fn create_swap_chain_default(
    dxgi_factory: &IDXGIFactory6,
    device: &ID3D11Device,
    hwnd: HWND,
    _transparent: bool,
) -> Result<ManuallyDrop<IDXGISwapChain1>> {
    use windows::Win32::Graphics::Dxgi::DXGI_MWA_NO_ALT_ENTER;

    let desc = DXGI_SWAP_CHAIN_DESC1 {
        Width: 1,
        Height: 1,
        Format: RENDER_TARGET_FORMAT,
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
    Ok(ManuallyDrop::new(swap_chain))
}

#[inline]
fn create_render_target_and_its_view(
    swap_chain: &IDXGISwapChain1,
    device: &ID3D11Device,
) -> Result<(
    ManuallyDrop<ID3D11Texture2D>,
    [Option<ID3D11RenderTargetView>; 1],
)> {
    let render_target: ID3D11Texture2D = unsafe { swap_chain.GetBuffer(0) }?;
    let desc = D3D11_RENDER_TARGET_VIEW_DESC {
        Format: BACK_BUFFER_FORMAT,
        ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2D,
        ..Default::default()
    };
    let mut render_target_view = None;
    unsafe {
        device.CreateRenderTargetView(&render_target, Some(&desc), Some(&mut render_target_view))?
    };
    Ok((
        ManuallyDrop::new(render_target),
        [Some(render_target_view.unwrap())],
    ))
}

#[inline]
fn set_render_target_view(
    swap_chain: &IDXGISwapChain1,
    device: &ID3D11Device,
    device_context: &ID3D11DeviceContext,
) -> Result<ID3D11RenderTargetView> {
    // In dx11, ID3D11RenderTargetView is supposed to always point to the new back buffer.
    // https://stackoverflow.com/questions/65246961/does-the-backbuffer-that-a-rendertargetview-points-to-automagically-change-after
    let back_buffer = unsafe {
        let resource: ID3D11Texture2D = swap_chain.GetBuffer(0)?;
        let desc = D3D11_RENDER_TARGET_VIEW_DESC {
            Format: BACK_BUFFER_FORMAT,
            ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2D,
            ..Default::default()
        };
        let mut buffer: Option<ID3D11RenderTargetView> = None;
        device.CreateRenderTargetView(&resource, Some(&desc), Some(&mut buffer))?;
        buffer.unwrap()
    };
    unsafe { device_context.OMSetRenderTargets(Some(&[Some(back_buffer.clone())]), None) };
    Ok(back_buffer)
}

#[inline]
fn create_msaa_target_and_its_view(
    device: &ID3D11Device,
    width: u32,
    height: u32,
) -> Result<(ID3D11Texture2D, ID3D11RenderTargetView)> {
    let msaa_target = unsafe {
        let mut output = None;
        let desc = D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: BACK_BUFFER_FORMAT,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 4,
                Quality: D3D11_STANDARD_MULTISAMPLE_PATTERN.0 as u32,
            },
            Usage: D3D11_USAGE_DEFAULT,
            BindFlags: D3D11_BIND_RENDER_TARGET.0 as u32,
            CPUAccessFlags: 0,
            MiscFlags: 0,
        };
        device.CreateTexture2D(&desc, None, Some(&mut output))?;
        output.unwrap()
    };
    let msaa_view = unsafe {
        let desc = D3D11_RENDER_TARGET_VIEW_DESC {
            Format: BACK_BUFFER_FORMAT,
            ViewDimension: D3D11_RTV_DIMENSION_TEXTURE2DMS,
            ..Default::default()
        };
        let mut output = None;
        device.CreateRenderTargetView(&msaa_target, Some(&desc), Some(&mut output))?;
        output.unwrap()
    };
    Ok((msaa_target, msaa_view))
}

#[inline]
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

#[inline]
fn set_rasterizer_state(device: &ID3D11Device, device_context: &ID3D11DeviceContext) -> Result<()> {
    let desc = D3D11_RASTERIZER_DESC {
        FillMode: D3D11_FILL_SOLID,
        CullMode: D3D11_CULL_NONE,
        // FrontCounterClockwise: true.into(),
        FrontCounterClockwise: false.into(),
        DepthBias: 0,
        DepthBiasClamp: 0.0,
        SlopeScaledDepthBias: 0.0,
        DepthClipEnable: true.into(),
        ScissorEnable: false.into(),
        // MultisampleEnable: false.into(),
        MultisampleEnable: true.into(),
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

// https://learn.microsoft.com/en-us/windows/win32/api/d3d11/ns-d3d11-d3d11_blend_desc
#[inline]
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

#[inline]
fn create_vertex_shader(device: &ID3D11Device, bytes: &[u8]) -> Result<ID3D11VertexShader> {
    unsafe {
        let mut shader = None;
        device.CreateVertexShader(bytes, None, Some(&mut shader))?;
        Ok(shader.unwrap())
    }
}

#[inline]
fn create_fragment_shader(device: &ID3D11Device, bytes: &[u8]) -> Result<ID3D11PixelShader> {
    unsafe {
        let mut shader = None;
        device.CreatePixelShader(bytes, None, Some(&mut shader))?;
        Ok(shader.unwrap())
    }
}

#[inline]
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

#[inline]
fn create_buffer_view(
    device: &ID3D11Device,
    buffer: &ID3D11Buffer,
) -> Result<[Option<ID3D11ShaderResourceView>; 1]> {
    let mut view = None;
    unsafe { device.CreateShaderResourceView(buffer, None, Some(&mut view)) }?;
    Ok([view])
}

#[inline]
fn create_indirect_draw_buffer(device: &ID3D11Device, buffer_size: usize) -> Result<ID3D11Buffer> {
    let desc = D3D11_BUFFER_DESC {
        ByteWidth: (std::mem::size_of::<DrawInstancedIndirectArgs>() * buffer_size) as u32,
        Usage: D3D11_USAGE_DYNAMIC,
        BindFlags: D3D11_BIND_SHADER_RESOURCE.0 as u32,
        CPUAccessFlags: D3D11_CPU_ACCESS_WRITE.0 as u32,
        MiscFlags: D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS.0 as u32,
        StructureByteStride: std::mem::size_of::<DrawInstancedIndirectArgs>() as u32,
    };
    let mut buffer = None;
    unsafe { device.CreateBuffer(&desc, None, Some(&mut buffer)) }?;
    Ok(buffer.unwrap())
}

#[inline]
fn pre_draw(
    device_context: &ID3D11DeviceContext,
    global_params_buffer: &[Option<ID3D11Buffer>; 1],
    view_port: &[D3D11_VIEWPORT; 1],
    render_target_view: &[Option<ID3D11RenderTargetView>; 1],
    clear_color: [f32; 4],
    blend_state: &ID3D11BlendState,
) -> Result<()> {
    let global_params = global_params_buffer[0].as_ref().unwrap();
    update_buffer(
        device_context,
        global_params,
        &[GlobalParams {
            viewport_size: [view_port[0].Width, view_port[0].Height],
            ..Default::default()
        }],
    )?;
    unsafe {
        device_context.RSSetViewports(Some(view_port));
        device_context.OMSetRenderTargets(Some(render_target_view), None);
        device_context.ClearRenderTargetView(render_target_view[0].as_ref().unwrap(), &clear_color);
        device_context.OMSetBlendState(blend_state, None, 0xFFFFFFFF);
    }
    Ok(())
}

#[inline]
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

#[inline]
fn set_pipeline_state(
    device_context: &ID3D11DeviceContext,
    buffer_view: &[Option<ID3D11ShaderResourceView>],
    topology: D3D_PRIMITIVE_TOPOLOGY,
    viewport: &[D3D11_VIEWPORT],
    vertex_shader: &ID3D11VertexShader,
    fragment_shader: &ID3D11PixelShader,
    global_params: &[Option<ID3D11Buffer>],
) {
    unsafe {
        device_context.VSSetShaderResources(1, Some(buffer_view));
        device_context.PSSetShaderResources(1, Some(buffer_view));
        device_context.IASetPrimitiveTopology(topology);
        device_context.RSSetViewports(Some(viewport));
        device_context.VSSetShader(vertex_shader, None);
        device_context.PSSetShader(fragment_shader, None);
        device_context.VSSetConstantBuffers(0, Some(global_params));
        device_context.PSSetConstantBuffers(0, Some(global_params));
    }
}

const BUFFER_COUNT: usize = 3;

mod shader_resources {
    use anyhow::Result;
    use windows::Win32::Graphics::Direct3D::{
        Fxc::{D3DCOMPILE_DEBUG, D3DCOMPILE_SKIP_OPTIMIZATION, D3DCompileFromFile},
        ID3DBlob,
    };
    use windows_core::{HSTRING, PCSTR};

    pub(super) fn build_shader_blob(entry: &str, target: &str) -> Result<ID3DBlob> {
        unsafe {
            let mut entry = entry.to_owned();
            let mut target = target.to_owned();
            let mut compile_blob = None;
            let mut error_blob = None;
            let shader_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("src/platform/windows/shaders.hlsl")
                .canonicalize()
                .unwrap();
            entry.push_str("\0");
            target.push_str("\0");
            let entry_point = PCSTR::from_raw(entry.as_ptr());
            let target_cstr = PCSTR::from_raw(target.as_ptr());
            #[cfg(debug_assertions)]
            let compile_flag = D3DCOMPILE_DEBUG | D3DCOMPILE_SKIP_OPTIMIZATION;
            #[cfg(not(debug_assertions))]
            let compile_flag = 0;
            let ret = D3DCompileFromFile(
                &HSTRING::from(shader_path.to_str().unwrap()),
                None,
                None,
                entry_point,
                target_cstr,
                compile_flag,
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
                let error_string = String::from_utf8_lossy(&error_string_encode).to_string();
                log::error!("Shader compile error: {}", error_string);
                return Err(anyhow::anyhow!("Compile error: {}", error_string));
            }
            Ok(compile_blob.unwrap())
        }
    }
}
