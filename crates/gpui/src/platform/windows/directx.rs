use std::sync::Arc;

use anyhow::Result;
use util::ResultExt;
use windows::Win32::Graphics::{
    Direct3D::{D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1},
    Direct3D11::{
        D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        D3D11_CREATE_DEVICE_DEBUG, D3D11_SDK_VERSION,
    },
    Dxgi::{
        CreateDXGIFactory2, IDXGIAdapter1, IDXGIFactory6, DXGI_CREATE_FACTORY_DEBUG,
        DXGI_GPU_PREFERENCE_MINIMUM_POWER,
    },
};

use crate::{DevicePixels, DirectXAtlas, PlatformAtlas, Scene, Size, WindowBackgroundAppearance};

pub(crate) struct DirectXRenderer {
    atlas: Arc<DirectXAtlas>,
    context: DirectXContext,
}

struct DirectXContext {
    dxgi_factory: IDXGIFactory6,
    device: ID3D11Device,
    context: ID3D11DeviceContext,
}

impl DirectXRenderer {
    pub(crate) fn new() -> Self {
        DirectXRenderer {
            atlas: Arc::new(DirectXAtlas::new()),
            context: DirectXContext::new().unwrap(),
        }
    }

    pub(crate) fn spirite_atlas(&self) -> Arc<dyn PlatformAtlas> {
        self.atlas.clone()
    }

    pub(crate) fn draw(&mut self, scene: &Scene) {
        // TODO:
    }

    pub(crate) fn resize(&mut self, new_size: Size<DevicePixels>) {
        // TODO:
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
}

impl DirectXContext {
    pub fn new() -> Result<Self> {
        let dxgi_factory = get_dxgi_factory()?;
        let adapter = get_adapter(&dxgi_factory)?;
        let mut device: Option<ID3D11Device> = None;
        let mut context: Option<ID3D11DeviceContext> = None;
        get_device(&adapter, Some(&mut device), Some(&mut context))?;
        Ok(Self {
            dxgi_factory,
            device: device.unwrap(),
            context: context.unwrap(),
        })
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
