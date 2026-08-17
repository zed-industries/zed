use alloc::{string::String, sync::Arc, vec::Vec};

use parking_lot::RwLock;
use windows::Win32::{Foundation, Graphics::Dxgi};

use super::SurfaceTarget;
use crate::{
    auxil,
    dx12::{
        device_creation::DeviceFactory, shader_compilation::CompilerContainer, D3D12Lib, DCompLib,
    },
};

impl crate::Instance for super::Instance {
    type A = super::Api;

    unsafe fn init(desc: &crate::InstanceDescriptor<'_>) -> Result<Self, crate::InstanceError> {
        profiling::scope!("Init DX12 Backend");
        let lib_main = D3D12Lib::new().map_err(|e| {
            crate::InstanceError::with_source(String::from("failed to load d3d12.dll"), e)
        })?;

        // Create DeviceFactory first so we know which debug path to use
        let device_factory =
            DeviceFactory::new(&lib_main, desc.backend_options.dx12.agility_sdk.as_ref())?;

        device_factory.enable_debug_layer(&lib_main, desc.flags);

        let (lib_dxgi, factory) = auxil::dxgi::factory::create_factory(desc.flags)?;

        // Create IDXGIFactoryMedia
        let factory_media = lib_dxgi.create_factory_media().ok();

        let mut supports_allow_tearing = false;
        if let Some(factory5) = factory.as_factory5() {
            let mut allow_tearing = Foundation::FALSE;
            let hr = unsafe {
                factory5.CheckFeatureSupport(
                    Dxgi::DXGI_FEATURE_PRESENT_ALLOW_TEARING,
                    <*mut _>::cast(&mut allow_tearing),
                    size_of_val(&allow_tearing) as u32,
                )
            };

            match hr {
                Err(err) => log::warn!("Unable to check for tearing support: {err}"),
                Ok(()) => supports_allow_tearing = true,
            }
        }

        // Initialize the shader compiler
        let compiler_container = match desc.backend_options.dx12.shader_compiler.clone() {
            wgt::Dx12Compiler::DynamicDxc { dxc_path } => {
                CompilerContainer::new_dynamic_dxc(dxc_path.into()).map_err(|e| {
                    crate::InstanceError::with_source(String::from("Failed to load dynamic DXC"), e)
                })?
            }
            wgt::Dx12Compiler::StaticDxc => CompilerContainer::new_static_dxc().map_err(|e| {
                crate::InstanceError::with_source(String::from("Failed to load static DXC"), e)
            })?,
            wgt::Dx12Compiler::Fxc => CompilerContainer::new_fxc().map_err(|e| {
                crate::InstanceError::with_source(String::from("Failed to load FXC"), e)
            })?,
            wgt::Dx12Compiler::Auto => {
                if cfg!(feature = "static-dxc") {
                    // Prefer static DXC if its compiled in
                    CompilerContainer::new_static_dxc().map_err(|e| {
                        crate::InstanceError::with_source(
                            String::from("Failed to load static DXC"),
                            e,
                        )
                    })?
                } else {
                    // Try to load dynamic DXC
                    let dynamic = CompilerContainer::new_dynamic_dxc("dxcompiler.dll".into());
                    match dynamic {
                        Ok(v) => v,
                        Err(super::shader_compilation::GetContainerError::FailedToLoad(..)) => {
                            // If it can't be found load FXC
                            CompilerContainer::new_fxc().map_err(|e| {
                                crate::InstanceError::with_source(
                                    String::from("Failed to load FXC"),
                                    e,
                                )
                            })?
                        }
                        Err(e) => {
                            // If another error occurs when loading static DXC return that error
                            return Err(crate::InstanceError::with_source(
                                String::from("Failed to load dynamic DXC"),
                                e,
                            ));
                        }
                    }
                }
            }
        };

        match compiler_container {
            CompilerContainer::DynamicDxc(..) => {
                log::debug!("Using dynamic DXC for shader compilation")
            }
            CompilerContainer::StaticDxc(..) => {
                log::debug!("Using static DXC for shader compilation")
            }
            CompilerContainer::Fxc(..) => {
                log::debug!("Using FXC for shader compilation")
            }
        }

        Ok(Self {
            // The call to create_factory will only succeed if we get a factory4, so this is safe.
            factory,
            factory_media,
            library: Arc::new(lib_main),
            device_factory: Arc::new(device_factory),
            dcomp_lib: Arc::new(DCompLib::new()),
            presentation_system: desc.backend_options.dx12.presentation_system,
            _lib_dxgi: lib_dxgi,
            supports_allow_tearing,
            flags: desc.flags,
            memory_budget_thresholds: desc.memory_budget_thresholds,
            compiler_container: Arc::new(compiler_container),
            options: desc.backend_options.dx12.clone(),
            telemetry: desc.telemetry,
        })
    }

    unsafe fn create_surface(
        &self,
        display_handle: raw_window_handle::RawDisplayHandle,
        window_handle: raw_window_handle::RawWindowHandle,
    ) -> Result<super::Surface, crate::InstanceError> {
        assert!(matches!(
            display_handle,
            raw_window_handle::RawDisplayHandle::Windows(_)
        ));
        match window_handle {
            raw_window_handle::RawWindowHandle::Win32(handle) => {
                // https://github.com/rust-windowing/raw-window-handle/issues/171
                let handle = Foundation::HWND(handle.hwnd.get() as *mut _);
                let target = match self.presentation_system {
                    wgt::Dx12SwapchainKind::DxgiFromHwnd => SurfaceTarget::WndHandle(handle),
                    wgt::Dx12SwapchainKind::DxgiFromVisual => SurfaceTarget::VisualFromWndHandle {
                        handle,
                        dcomp_state: Default::default(),
                    },
                };

                Ok(super::Surface {
                    factory: self.factory.clone(),
                    factory_media: self.factory_media.clone(),
                    target,
                    supports_allow_tearing: self.supports_allow_tearing,
                    swap_chain: RwLock::new(None),
                    options: self.options.clone(),
                })
            }
            _ => Err(crate::InstanceError::new(format!(
                "window handle {window_handle:?} is not a Win32 handle"
            ))),
        }
    }

    unsafe fn enumerate_adapters(
        &self,
        _surface_hint: Option<&super::Surface>,
    ) -> Vec<crate::ExposedAdapter<super::Api>> {
        let adapters = auxil::dxgi::factory::enumerate_adapters(self.factory.clone());

        adapters
            .into_iter()
            .filter_map(|raw| {
                super::Adapter::expose(
                    raw,
                    &self.library,
                    &self.device_factory,
                    &self.dcomp_lib,
                    self.flags,
                    self.memory_budget_thresholds,
                    self.compiler_container.clone(),
                    self.options.clone(),
                    self.telemetry,
                )
            })
            .collect()
    }
}
