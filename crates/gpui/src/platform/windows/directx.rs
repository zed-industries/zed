use std::sync::Arc;

use ::util::ResultExt;
use anyhow::Result;
use windows::{
    core::*,
    Win32::{
        Foundation::{HWND, RECT},
        Graphics::{
            Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1},
            Direct3D11::{
                D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, ID3D11RenderTargetView,
                ID3D11Texture2D, D3D11_BIND_CONSTANT_BUFFER, D3D11_BIND_FLAG,
                D3D11_BIND_VERTEX_BUFFER, D3D11_BUFFER_DESC, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION, D3D11_SUBRESOURCE_DATA,
                D3D11_USAGE_IMMUTABLE, D3D11_VIEWPORT,
            },
            DirectComposition::{
                DCompositionCreateDevice, DCompositionCreateDevice2, IDCompositionDesktopDevice,
                IDCompositionDevice, IDCompositionSurface, IDCompositionTarget,
                IDCompositionVisual, IDCompositionVisual2,
            },
            Dxgi::{
                Common::{
                    DXGI_ALPHA_MODE_PREMULTIPLIED, DXGI_FORMAT_B8G8R8A8_UNORM, DXGI_FORMAT_UNKNOWN,
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
}

struct DirectXContext {
    dxgi_factory: IDXGIFactory6,
    device: ID3D11Device,
    dxgi_device: IDXGIDevice,
    context: ID3D11DeviceContext,
    swap_chain: IDXGISwapChain1,
    back_buffer: Option<ID3D11RenderTargetView>,

    comp_device: IDCompositionDevice,
    comp_target: IDCompositionTarget,
    comp_visual: IDCompositionVisual,
}

struct DirectXRenderContext {}

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
        }
    }

    pub(crate) fn spirite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    pub(crate) fn draw(&mut self, scene: &Scene) {
        unsafe {
            self.context.context.ClearRenderTargetView(
                self.context.back_buffer.as_ref().unwrap(),
                &[0.0, 0.2, 0.4, 0.6],
            );
        }
        self.draw_primitives(scene);
        unsafe { self.context.swap_chain.Present(0, 0).ok().log_err() };
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) {
        // TODO:
        unsafe {
            self.context.context.OMSetRenderTargets(None, None);
            drop(self.context.back_buffer.take().unwrap());
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
            self.context.back_buffer = Some(backbuffer);
            set_viewport(
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
        for batch in scene.batches() {
            let ok = match batch {
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
        true
    }

    // TODO:
    fn draw_quads(&mut self, quads: &[Quad]) -> bool {
        if quads.is_empty() {
            return true;
        }
        true
    }

    // TODO:
    fn draw_paths(&mut self, paths: &[Path<ScaledPixels>]) -> bool {
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
        let comp_device = get_comp_device(&dxgi_device)?;
        let swap_chain = get_swap_chain(&dxgi_factory, &device)?;
        let comp_target = unsafe { comp_device.CreateTargetForHwnd(hwnd, true) }?;
        let comp_visual = unsafe { comp_device.CreateVisual() }?;
        unsafe {
            comp_visual.SetContent(&swap_chain)?;
            comp_target.SetRoot(&comp_visual)?;
            comp_device.Commit()?;
            dxgi_factory.MakeWindowAssociation(hwnd, DXGI_MWA_NO_ALT_ENTER)?;
        }
        let back_buffer = Some(set_render_target_view(&swap_chain, &device, &context)?);
        set_viewport(&context, 1.0, 1.0);

        Ok(Self {
            dxgi_factory,
            device,
            dxgi_device,
            context,
            swap_chain,
            back_buffer,
            comp_device,
            comp_target,
            comp_visual,
        })
    }
}

impl DirectXRenderContext {
    pub fn new(device: &ID3D11Device) -> Result<Self> {
        fn to_float2_bits(point: Point<f32>) -> u64 {
            let mut output = point.y.to_bits() as u64;
            output <<= 32;
            output |= point.x.to_bits() as u64;
            output
        }
        let unit_vertices = [
            to_float2_bits(point(0., 0.)),
            to_float2_bits(point(1., 0.)),
            to_float2_bits(point(0., 1.)),
            to_float2_bits(point(0., 1.)),
            to_float2_bits(point(1., 0.)),
            to_float2_bits(point(1., 1.)),
        ];
        let uint_vertices_buffer = unsafe {
            let desc = D3D11_BUFFER_DESC {
                ByteWidth: std::mem::size_of_val(&unit_vertices) as u32,
                Usage: D3D11_USAGE_IMMUTABLE,
                BindFlags: D3D11_BIND_VERTEX_BUFFER.0 as u32,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 8,
            };
            let data = D3D11_SUBRESOURCE_DATA {
                pSysMem: unit_vertices.as_ptr() as _,
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };
            let mut buffer = None;
            device.CreateBuffer(&desc, Some(&data), Some(&mut buffer))?;
            buffer.unwrap()
        };

        Ok(Self {})
    }
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
    // Check to see whether the adapter supports Direct3D 11, but don't
    // create the actual device yet.
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

fn get_swap_chain(dxgi_factory: &IDXGIFactory6, device: &ID3D11Device) -> Result<IDXGISwapChain1> {
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

fn set_viewport(device_context: &ID3D11DeviceContext, width: f32, height: f32) {
    let viewport = D3D11_VIEWPORT {
        TopLeftX: 0.0,
        TopLeftY: 0.0,
        Width: width,
        Height: height,
        MinDepth: 0.0,
        MaxDepth: 1.0,
    };
    unsafe { device_context.RSSetViewports(Some(&[viewport])) };
}

const BUFFER_COUNT: usize = 3;
