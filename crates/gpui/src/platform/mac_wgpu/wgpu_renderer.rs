use std::sync::Arc;

use wgpu::{
    Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, InstanceFlags,
    Limits, Queue, Surface, SurfaceTarget,
};

use crate::MacWindow;

use super::wgpu_atlas::WgpuAtlas;

pub struct WgpuRenderer {
    surface: Surface<'static>,
    device: Device,
    queue: Queue,
    sprite_atlas: Arc<WgpuAtlas>,
}

impl WgpuRenderer {
    pub fn new(window: MacWindow) -> WgpuRenderer {
        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::METAL,
            flags: InstanceFlags::VALIDATION,
            dx12_shader_compiler: wgpu::Dx12Compiler::Dxc {
                dxil_path: None,
                dxc_path: None,
            },
            gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
        });

        let surface = instance
            .create_surface(SurfaceTarget::Window(Box::new(window)))
            .unwrap();

        let adapter = smol::block_on(instance.request_adapter(&wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = smol::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                // TODO
                required_features: Features::default(),
                // TODO: This may bite us
                required_limits: Limits::default(),
            },
            None,
        ))
        .unwrap();

        WgpuRenderer {
            surface,
            device,
            queue,
            sprite_atlas: Arc::new(WgpuAtlas::new()),
        }
    }

    pub fn sprite_atlas(&self) -> &Arc<WgpuAtlas> {
        &self.sprite_atlas
    }
}
