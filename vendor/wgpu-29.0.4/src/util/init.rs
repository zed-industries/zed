use crate::{Adapter, Instance, RequestAdapterOptions, Surface};

#[cfg(doc)]
use crate::Backends;

/// Initialize the adapter obeying the `WGPU_ADAPTER_NAME` environment variable.
#[cfg(wgpu_core)]
#[cfg_attr(not(std), expect(unused_variables, unreachable_code))]
pub async fn initialize_adapter_from_env(
    instance: &Instance,
    compatible_surface: Option<&Surface<'_>>,
) -> Result<Adapter, wgt::RequestAdapterError> {
    let desired_adapter_name: alloc::string::String = {
        cfg_if::cfg_if! {
            if #[cfg(std)] {
                std::env::var("WGPU_ADAPTER_NAME")
                    .as_deref()
                    .map(str::to_lowercase)
                    .map_err(|_| wgt::RequestAdapterError::EnvNotSet)?
            } else {
                return Err(wgt::RequestAdapterError::EnvNotSet)
            }
        }
    };

    let adapters = instance.enumerate_adapters(crate::Backends::all()).await;

    let mut chosen_adapter = None;
    for adapter in adapters {
        let info = adapter.get_info();

        if let Some(surface) = compatible_surface {
            if !adapter.is_surface_supported(surface) {
                continue;
            }
        }

        if info.name.to_lowercase().contains(&desired_adapter_name) {
            chosen_adapter = Some(adapter);
            break;
        }
    }

    Ok(chosen_adapter.expect("WGPU_ADAPTER_NAME set but no matching adapter found!"))
}

/// Initialize the adapter obeying the `WGPU_ADAPTER_NAME` environment variable.
#[cfg(not(wgpu_core))]
pub async fn initialize_adapter_from_env(
    _instance: &Instance,
    _compatible_surface: Option<&Surface<'_>>,
) -> Result<Adapter, wgt::RequestAdapterError> {
    Err(wgt::RequestAdapterError::EnvNotSet)
}

/// Initialize the adapter obeying the `WGPU_ADAPTER_NAME` environment variable and if it doesn't exist fall back on a default adapter.
pub async fn initialize_adapter_from_env_or_default(
    instance: &Instance,
    compatible_surface: Option<&Surface<'_>>,
) -> Result<Adapter, wgt::RequestAdapterError> {
    match initialize_adapter_from_env(instance, compatible_surface).await {
        Ok(a) => Ok(a),
        Err(_) => {
            instance
                .request_adapter(&RequestAdapterOptions {
                    power_preference: crate::PowerPreference::from_env().unwrap_or_default(),
                    force_fallback_adapter: false,
                    compatible_surface,
                })
                .await
        }
    }
}

/// Determines whether the [`Backends::BROWSER_WEBGPU`] backend is supported.
///
/// The result can only be true if this is called from the main thread or a dedicated worker.
/// For convenience, this is also supported on non-wasm targets, always returning false there.
pub async fn is_browser_webgpu_supported() -> bool {
    #[cfg(webgpu)]
    {
        // In theory it should be enough to check for the presence of the `gpu` property...
        let gpu = crate::backend::get_browser_gpu_property();
        let Ok(Some(gpu)) = gpu else {
            return false;
        };

        // ...but in practice, we also have to try to create an adapter, since as of writing
        // Chrome on Linux has the `gpu` property but doesn't support WebGPU.
        let adapter_promise = gpu.request_adapter();
        wasm_bindgen_futures::JsFuture::from(adapter_promise)
            .await
            .is_ok_and(|adapter| !adapter.is_undefined() && !adapter.is_null())
    }
    #[cfg(not(webgpu))]
    {
        false
    }
}

/// Create an new instance of wgpu, but disabling [`Backends::BROWSER_WEBGPU`] if no WebGPU support was detected.
///
/// If the instance descriptor enables [`Backends::BROWSER_WEBGPU`],
/// this checks via [`is_browser_webgpu_supported`] for WebGPU support before forwarding
/// the descriptor with or without [`Backends::BROWSER_WEBGPU`] respectively to [`Instance::new`].
///
/// You should prefer this method over [`Instance::new`] if you want to target WebGPU and automatically
/// fall back to WebGL if WebGPU is not available.
/// This is because WebGPU support has to be decided upon instance creation and [`Instance::new`]
/// (being a `sync` function) can't establish WebGPU support (details see [`is_browser_webgpu_supported`]).
///
/// # Panics
///
/// If no backend feature for the active target platform is enabled,
/// this method will panic, see [`Instance::enabled_backend_features()`].
pub async fn new_instance_with_webgpu_detection(
    mut instance_desc: wgt::InstanceDescriptor,
) -> crate::Instance {
    if instance_desc
        .backends
        .contains(wgt::Backends::BROWSER_WEBGPU)
        && !is_browser_webgpu_supported().await
    {
        instance_desc.backends.remove(wgt::Backends::BROWSER_WEBGPU);
    }

    crate::Instance::new(instance_desc)
}
