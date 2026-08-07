use anyhow::{Context, Result};
use gpui_util::ResultExt;
use itertools::Itertools;
use windows::Win32::{
    Foundation::HMODULE,
    Graphics::{
        Direct3D::{
            D3D_DRIVER_TYPE_UNKNOWN, D3D_FEATURE_LEVEL, D3D_FEATURE_LEVEL_10_1,
            D3D_FEATURE_LEVEL_11_0, D3D_FEATURE_LEVEL_11_1,
        },
        Direct3D11::{
            D3D11_CREATE_DEVICE_BGRA_SUPPORT, D3D11_CREATE_DEVICE_DEBUG,
            D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS, D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS,
            D3D11_SDK_VERSION, D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
        },
        Dxgi::{
            CreateDXGIFactory2, DXGI_CREATE_FACTORY_DEBUG, DXGI_CREATE_FACTORY_FLAGS,
            IDXGIAdapter1, IDXGIFactory6,
        },
    },
};
use windows::core::Interface;

pub(crate) fn try_to_recover_from_device_lost<T>(mut f: impl FnMut() -> Result<T>) -> Result<T> {
    (0..5)
        .map(|i| {
            if i > 0 {
                // Add a small delay before retrying
                std::thread::sleep(std::time::Duration::from_millis(100 + i * 10));
            }
            f()
        })
        .find_or_last(Result::is_ok)
        .unwrap()
        .context("DirectXRenderer failed to recover from lost device after multiple attempts")
}

#[derive(Clone)]
pub(crate) struct DirectXDevices {
    pub(crate) adapter: IDXGIAdapter1,
    pub(crate) dxgi_factory: IDXGIFactory6,
    pub(crate) device: ID3D11Device,
    pub(crate) device_context: ID3D11DeviceContext,
}

impl DirectXDevices {
    pub(crate) fn new() -> Result<Self> {
        let debug_layer_available = check_debug_layer_available();
        let dxgi_factory =
            get_dxgi_factory(debug_layer_available).context("Creating DXGI factory")?;
        let (adapter, device, device_context, feature_level) =
            get_adapter(&dxgi_factory, debug_layer_available).context("Getting DXGI adapter")?;
        match feature_level {
            D3D_FEATURE_LEVEL_11_1 => {
                log::info!("Created device with Direct3D 11.1 feature level.")
            }
            D3D_FEATURE_LEVEL_11_0 => {
                log::info!("Created device with Direct3D 11.0 feature level.")
            }
            D3D_FEATURE_LEVEL_10_1 => {
                log::info!("Created device with Direct3D 10.1 feature level.")
            }
            // Not `unreachable!()`: the feature level is reported by the
            // driver, and panicking on an unfamiliar one throws away a device
            // that already works.
            other => log::info!("Created device with Direct3D feature level {:#x}.", other.0),
        }

        Ok(Self {
            adapter,
            dxgi_factory,
            device,
            device_context,
        })
    }
}

#[inline]
fn check_debug_layer_available() -> bool {
    #[cfg(debug_assertions)]
    {
        use windows::Win32::Graphics::Dxgi::{DXGIGetDebugInterface1, IDXGIInfoQueue};

        unsafe { DXGIGetDebugInterface1::<IDXGIInfoQueue>(0) }
            .log_err()
            .is_some()
    }
    #[cfg(not(debug_assertions))]
    {
        false
    }
}

#[inline]
fn get_dxgi_factory(debug_layer_available: bool) -> Result<IDXGIFactory6> {
    let factory_flag = if debug_layer_available {
        DXGI_CREATE_FACTORY_DEBUG
    } else {
        #[cfg(debug_assertions)]
        log::warn!(
            "Failed to get DXGI debug interface. DirectX debugging features will be disabled."
        );
        DXGI_CREATE_FACTORY_FLAGS::default()
    };
    unsafe { Ok(CreateDXGIFactory2(factory_flag)?) }
}

/// Why no usable Direct3D 11 adapter could be selected.
///
/// This replaces the `unreachable!()` that used to end `get_adapter`. A machine
/// with no usable adapter is an ordinary machine -- a headless VM, an RDP
/// session, a box with a broken display driver -- and panicking there discards
/// the process instead of letting the caller decide what to do.
#[derive(Debug)]
pub enum AdapterSelectionError {
    /// Hardware enumeration ran out and the WARP software adapter could not be
    /// obtained from the DXGI factory at all.
    WarpUnavailable {
        /// How many adapters `EnumAdapters` handed back before it ran out.
        hardware_adapters_tried: u32,
        source: windows::core::Error,
    },
    /// The WARP adapter exists, but `D3D11CreateDevice` refused it.
    WarpDeviceUnusable {
        hardware_adapters_tried: u32,
        source: anyhow::Error,
    },
}

impl std::fmt::Display for AdapterSelectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WarpUnavailable {
                hardware_adapters_tried,
                source,
            } => write!(
                f,
                "no Direct3D 11 adapter available: {hardware_adapters_tried} hardware adapter(s) \
                 rejected and the WARP software adapter could not be created ({source})"
            ),
            Self::WarpDeviceUnusable {
                hardware_adapters_tried,
                source,
            } => write!(
                f,
                "no Direct3D 11 adapter available: {hardware_adapters_tried} hardware adapter(s) \
                 rejected and the WARP software adapter could not create a device ({source})"
            ),
        }
    }
}

impl std::error::Error for AdapterSelectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WarpUnavailable { source, .. } => Some(source),
            Self::WarpDeviceUnusable { source, .. } => Some(source.as_ref()),
        }
    }
}

struct Chosen<A, D> {
    adapter: A,
    device: D,
    software: bool,
    hardware_adapters_tried: u32,
}

/// The selection *policy*, with every DXGI call injected.
///
/// Split out so both the fallback and the failure can be exercised without a
/// GPU-less machine -- see the tests at the bottom of this file.
fn choose_adapter<A, D>(
    mut enumerate: impl FnMut(u32) -> std::result::Result<A, windows::core::Error>,
    mut warp: impl FnMut() -> std::result::Result<A, windows::core::Error>,
    mut create_device: impl FnMut(&A) -> Result<D>,
) -> std::result::Result<Chosen<A, D>, AdapterSelectionError> {
    let mut hardware_adapters_tried = 0u32;

    for index in 0.. {
        // This used to be `?`, which turned "we have enumerated every adapter"
        // (`DXGI_ERROR_NOT_FOUND`) into the function's return value and left
        // the WARP fallback below unreachable. Running out of adapters is the
        // normal end of this loop, not an error.
        let Ok(adapter) = enumerate(index) else { break };
        hardware_adapters_tried += 1;
        if let Some(device) = create_device(&adapter).log_err() {
            return Ok(Chosen {
                adapter,
                device,
                software: false,
                hardware_adapters_tried,
            });
        }
    }

    let adapter = warp().map_err(|source| AdapterSelectionError::WarpUnavailable {
        hardware_adapters_tried,
        source,
    })?;
    let device =
        create_device(&adapter).map_err(|source| AdapterSelectionError::WarpDeviceUnusable {
            hardware_adapters_tried,
            source,
        })?;
    Ok(Chosen {
        adapter,
        device,
        software: true,
        hardware_adapters_tried,
    })
}

#[inline]
fn get_adapter(
    dxgi_factory: &IDXGIFactory6,
    debug_layer_available: bool,
) -> Result<(
    IDXGIAdapter1,
    ID3D11Device,
    ID3D11DeviceContext,
    D3D_FEATURE_LEVEL,
)> {
    let Chosen {
        adapter,
        device: (device, context, feature_level),
        software,
        hardware_adapters_tried,
    } = choose_adapter(
        |index| unsafe { dxgi_factory.EnumAdapters(index)?.cast::<IDXGIAdapter1>() },
        || unsafe { dxgi_factory.EnumWarpAdapter::<IDXGIAdapter1>() },
        |adapter: &IDXGIAdapter1| {
            // Check to see whether the adapter supports Direct3D 11 and create
            // the device if it does.
            let mut context: Option<ID3D11DeviceContext> = None;
            let mut feature_level = D3D_FEATURE_LEVEL::default();
            let device = get_device(
                adapter,
                Some(&mut context),
                Some(&mut feature_level),
                debug_layer_available,
            )?;
            let context = context.context("D3D11CreateDevice returned no device context")?;
            Ok((device, context, feature_level))
        },
    )?;

    let gpu_name = unsafe { adapter.GetDesc1() }
        .map(|desc| {
            String::from_utf16_lossy(&desc.Description)
                .trim_matches(char::from(0))
                .to_string()
        })
        .unwrap_or_else(|_| "unknown adapter".to_string());
    if software {
        log::warn!(
            "Using the WARP software rasterizer: {gpu_name} ({hardware_adapters_tried} hardware \
             adapter(s) rejected). Rendering will be slower."
        );
    } else {
        log::info!("Using GPU: {gpu_name}");
    }

    Ok((adapter, device, context, feature_level))
}

#[inline]
fn get_device(
    adapter: &IDXGIAdapter1,
    context: Option<*mut Option<ID3D11DeviceContext>>,
    feature_level: Option<*mut D3D_FEATURE_LEVEL>,
    debug_layer_available: bool,
) -> Result<ID3D11Device> {
    let mut device: Option<ID3D11Device> = None;
    let device_flags = if debug_layer_available {
        D3D11_CREATE_DEVICE_BGRA_SUPPORT | D3D11_CREATE_DEVICE_DEBUG
    } else {
        D3D11_CREATE_DEVICE_BGRA_SUPPORT
    };
    unsafe {
        D3D11CreateDevice(
            adapter,
            D3D_DRIVER_TYPE_UNKNOWN,
            HMODULE::default(),
            device_flags,
            // 4x MSAA is required for Direct3D Feature Level 10.1 or better
            Some(&[
                D3D_FEATURE_LEVEL_11_1,
                D3D_FEATURE_LEVEL_11_0,
                D3D_FEATURE_LEVEL_10_1,
            ]),
            D3D11_SDK_VERSION,
            Some(&mut device),
            feature_level,
            context,
        )?;
    }
    let device = device.unwrap();
    let mut data = D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS::default();
    unsafe {
        device
            .CheckFeatureSupport(
                D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS,
                &mut data as *mut _ as _,
                std::mem::size_of::<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS>() as u32,
            )
            .context("Checking GPU device feature support")?;
    }
    if data
        .ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x
        .as_bool()
    {
        Ok(device)
    } else {
        Err(anyhow::anyhow!(
            "Required feature StructuredBuffer is not supported by GPU/driver"
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::{AdapterSelectionError, choose_adapter};
    use windows::Win32::Graphics::Dxgi::DXGI_ERROR_NOT_FOUND;

    fn not_found() -> windows::core::Error {
        windows::core::Error::from(DXGI_ERROR_NOT_FOUND)
    }

    #[test]
    fn hardware_adapter_wins_when_one_works() {
        let chosen = choose_adapter(
            |index| if index == 0 { Ok("gpu0") } else { Err(not_found()) },
            || Ok("warp"),
            |_| Ok(()),
        )
        .expect("a working hardware adapter must be selected");
        assert_eq!(chosen.adapter, "gpu0");
        assert!(!chosen.software);
        assert_eq!(chosen.hardware_adapters_tried, 1);
    }

    #[test]
    fn falls_back_to_warp_when_enumeration_finds_nothing() {
        // The GPU-less machine: EnumAdapters returns DXGI_ERROR_NOT_FOUND on
        // the very first call. The old code propagated that error and never
        // reached WARP.
        let chosen = choose_adapter(|_| Err(not_found()), || Ok("warp"), |_| Ok(()))
            .expect("WARP must be selected when no hardware adapter is enumerated");
        assert_eq!(chosen.adapter, "warp");
        assert!(chosen.software);
        assert_eq!(chosen.hardware_adapters_tried, 0);
    }

    #[test]
    fn falls_back_to_warp_when_every_hardware_adapter_is_rejected() {
        let chosen = choose_adapter(
            |index| if index < 2 { Ok("gpu") } else { Err(not_found()) },
            || Ok("warp"),
            |adapter: &&str| {
                if *adapter == "warp" {
                    Ok(())
                } else {
                    Err(anyhow::anyhow!("driver rejected the device"))
                }
            },
        )
        .expect("WARP must be selected when device creation fails on every GPU");
        assert_eq!(chosen.adapter, "warp");
        assert!(chosen.software);
        assert_eq!(chosen.hardware_adapters_tried, 2);
    }

    #[test]
    fn no_adapter_at_all_is_a_typed_error_not_a_panic() {
        let error =
            choose_adapter::<&str, ()>(|_| Err(not_found()), || Err(not_found()), |_| Ok(()))
                .map(|_| ())
                .expect_err("selection must fail when neither hardware nor WARP is available");
        match error {
            AdapterSelectionError::WarpUnavailable {
                hardware_adapters_tried,
                ..
            } => assert_eq!(hardware_adapters_tried, 0),
            other => panic!("expected WarpUnavailable, got {other:?}"),
        }
    }

    #[test]
    fn warp_that_cannot_make_a_device_is_a_distinct_typed_error() {
        let error = choose_adapter::<&str, ()>(
            |index| if index == 0 { Ok("gpu0") } else { Err(not_found()) },
            || Ok("warp"),
            |_| Err(anyhow::anyhow!("no structured buffer support")),
        )
        .map(|_| ())
        .expect_err("selection must fail when WARP cannot create a device");
        match error {
            AdapterSelectionError::WarpDeviceUnusable {
                hardware_adapters_tried,
                ..
            } => assert_eq!(hardware_adapters_tried, 1),
            other => panic!("expected WarpDeviceUnusable, got {other:?}"),
        }
    }
}
