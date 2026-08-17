use alloc::string::String;
use core::{fmt, mem};

use crate::{link_to_wgpu_docs, Backend, Backends};

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

#[cfg(doc)]
use crate::{Features, TextureUsages};

/// A set of requested capabilities when choosing a physical adapter.
///
/// Corresponds to the defined values of [WebGPU feature level string](
/// https://gpuweb.github.io/gpuweb/#feature-level-string).
///
/// `wgpu` does not support compatibility-level adapters per se.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum FeatureLevel {
    #[default]
    /// The `core` capability set
    Core,
    /// The `compatibility` capability set
    Compatibility,
}

/// Options for requesting adapter.
///
/// Corresponds to [WebGPU `GPURequestAdapterOptions`](
/// https://gpuweb.github.io/gpuweb/#dictdef-gpurequestadapteroptions).
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RequestAdapterOptions<S> {
    /// Power preference for the adapter.
    pub power_preference: PowerPreference,
    /// Indicates that only a fallback adapter can be returned. This is generally a "software"
    /// implementation on the system.
    pub force_fallback_adapter: bool,
    /// Surface that is required to be presentable with the requested adapter. This does not
    /// create the surface, only guarantees that the adapter can present to said surface.
    /// For WebGL, this is strictly required, as an adapter can not be created without a surface.
    pub compatible_surface: Option<S>,
}

impl<S> Default for RequestAdapterOptions<S> {
    fn default() -> Self {
        Self {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
        }
    }
}

/// Power Preference when choosing a physical adapter.
///
/// Corresponds to [WebGPU `GPUPowerPreference`](
/// https://gpuweb.github.io/gpuweb/#enumdef-gpupowerpreference).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum PowerPreference {
    #[default]
    /// Power usage is not considered when choosing an adapter.
    None = 0,
    /// Adapter that uses the least possible power. This is often an integrated GPU.
    LowPower = 1,
    /// Adapter that has the highest performance. This is often a discrete GPU.
    HighPerformance = 2,
}

impl PowerPreference {
    /// Get a power preference from the environment variable `WGPU_POWER_PREF`.
    pub fn from_env() -> Option<Self> {
        let env = crate::env::var("WGPU_POWER_PREF")?;
        match env.to_lowercase().as_str() {
            "low" => Some(Self::LowPower),
            "high" => Some(Self::HighPerformance),
            "none" => Some(Self::None),
            _ => None,
        }
    }
}

/// Supported physical device types.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeviceType {
    /// Other or Unknown.
    Other,
    /// Integrated GPU with shared CPU/GPU memory.
    IntegratedGpu,
    /// Discrete GPU with separate CPU/GPU memory.
    DiscreteGpu,
    /// Virtual / Hosted.
    VirtualGpu,
    /// Cpu / Software Rendering.
    Cpu,
}

//TODO: convert `vendor` and `device` to `u32`

/// Information about an adapter.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AdapterInfo {
    /// Adapter name
    pub name: String,
    /// [`Backend`]-specific vendor ID of the adapter
    ///
    /// This generally is a 16-bit PCI vendor ID in the least significant bytes of this field.
    /// However, more significant bytes may be non-zero if the backend uses a different
    /// representation.
    ///
    /// * For [`Backend::Vulkan`], the [`VkPhysicalDeviceProperties::vendorID`] is used, which is
    ///   a superset of PCI IDs.
    ///
    /// [`VkPhysicalDeviceProperties::vendorID`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html
    pub vendor: u32,
    /// [`Backend`]-specific device ID of the adapter
    ///
    ///
    /// This generally is a 16-bit PCI device ID in the least significant bytes of this field.
    /// However, more significant bytes may be non-zero if the backend uses a different
    /// representation.
    ///
    /// * For [`Backend::Vulkan`], the [`VkPhysicalDeviceProperties::deviceID`] is used, which is
    ///   a superset of PCI IDs.
    ///
    /// [`VkPhysicalDeviceProperties::deviceID`]: https://registry.khronos.org/vulkan/specs/1.3-extensions/man/html/VkPhysicalDeviceProperties.html
    pub device: u32,
    /// Type of device
    pub device_type: DeviceType,
    /// [`Backend`]-specific PCI bus ID of the adapter.
    ///
    /// * For [`Backend::Vulkan`], [`VkPhysicalDevicePCIBusInfoPropertiesEXT`] is used,
    ///   if available, in the form `bus:device.function`, e.g. `0000:01:00.0`.
    ///
    /// [`VkPhysicalDevicePCIBusInfoPropertiesEXT`]: https://registry.khronos.org/vulkan/specs/latest/man/html/VkPhysicalDevicePCIBusInfoPropertiesEXT.html
    pub device_pci_bus_id: String,
    /// Driver name
    pub driver: String,
    /// Driver info
    pub driver_info: String,
    /// Backend used for device
    pub backend: Backend,
    /// Minimum possible size of a subgroup on this adapter. Will
    /// never be lower than [`crate::MINIMUM_SUBGROUP_MIN_SIZE`].
    ///
    /// This will vary from device to device. Typical values are listed below.
    ///
    /// - NVIDIA: 32
    /// - AMD GCN/Vega: 64
    /// - AMD RDNA+: 32
    /// - Intel: 8 or 16
    /// - Qualcomm: 64
    /// - WARP: 4
    /// - lavapipe: 8
    pub subgroup_min_size: u32,
    /// Maximum possible size of a subgroup on this adapter. Will
    /// never be higher than [`crate::MAXIMUM_SUBGROUP_MAX_SIZE`].
    ///
    /// This will vary from device to device. Typical values are listed below:
    ///
    /// - NVIDIA: 32
    /// - AMD GCN/Vega: 64
    /// - AMD RDNA+: 64
    /// - Intel: 16 or 32
    /// - Qualcomm: 128
    /// - WARP: 4 or 128
    /// - lavapipe: 8
    pub subgroup_max_size: u32,
    /// If true, adding [`TextureUsages::TRANSIENT`] to a texture will decrease memory usage.
    pub transient_saves_memory: bool,
}

/// Error when [`Instance::request_adapter()`] fails.
///
/// This type is not part of the WebGPU standard, where `requestAdapter()` would simply return null.
///
#[doc = link_to_wgpu_docs!(["`Instance::request_adapter()`"]: "struct.Instance.html#method.request_adapter")]
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[non_exhaustive]
pub enum RequestAdapterError {
    /// No adapter available via the instance’s backends matched the request’s adapter criteria.
    NotFound {
        // These fields must be set by wgpu-core and wgpu, but are not intended to be stable API,
        // only data for the production of the error message.
        #[doc(hidden)]
        active_backends: Backends,
        #[doc(hidden)]
        requested_backends: Backends,
        #[doc(hidden)]
        supported_backends: Backends,
        #[doc(hidden)]
        no_fallback_backends: Backends,
        #[doc(hidden)]
        no_adapter_backends: Backends,
        #[doc(hidden)]
        incompatible_surface_backends: Backends,
    },

    /// Attempted to obtain adapter specified by environment variable, but the environment variable
    /// was not set.
    EnvNotSet,
}

impl core::error::Error for RequestAdapterError {}
impl fmt::Display for RequestAdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RequestAdapterError::NotFound {
                active_backends,
                requested_backends,
                supported_backends,
                no_fallback_backends,
                no_adapter_backends,
                incompatible_surface_backends,
            } => {
                write!(f, "No suitable graphics adapter found; ")?;
                let mut first = true;
                for backend in Backend::ALL {
                    let bit = Backends::from(backend);
                    let comma = if mem::take(&mut first) { "" } else { ", " };
                    let explanation = if !requested_backends.contains(bit) {
                        // We prefer reporting this, because it makes the error most stable with
                        // respect to what is directly controllable by the caller, as opposed to
                        // compilation options or the run-time environment.
                        "not requested"
                    } else if !supported_backends.contains(bit) {
                        "support not compiled in"
                    } else if no_adapter_backends.contains(bit) {
                        "found no adapters"
                    } else if incompatible_surface_backends.contains(bit) {
                        "not compatible with provided surface"
                    } else if no_fallback_backends.contains(bit) {
                        "had no fallback adapters"
                    } else if !active_backends.contains(bit) {
                        // Backend requested but not active in this instance
                        if backend == Backend::Noop {
                            "not explicitly enabled"
                        } else {
                            "drivers/libraries could not be loaded"
                        }
                    } else {
                        // This path should be unreachable, but don't crash.
                        "[unknown reason]"
                    };
                    write!(f, "{comma}{backend} {explanation}")?;
                }
            }
            RequestAdapterError::EnvNotSet => f.write_str("WGPU_ADAPTER_NAME not set")?,
        }
        Ok(())
    }
}

/// The underlying scalar type of the cooperative matrix component.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CooperativeScalarType {
    /// 32-bit floating point.
    F32,
    /// 16-bit floating point.
    F16,
    /// 32-bit signed integer.
    I32,
    /// 32-bit unsigned integer.
    U32,
}

/// Describes a supported cooperative matrix configuration.
///
/// Cooperative matrices perform the operation `C = A * B + C` where:
/// - `A` is an M×K matrix
/// - `B` is a K×N matrix
/// - `C` is an M×N matrix (both input accumulator and output)
#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct CooperativeMatrixProperties {
    /// Number of rows in matrices A and C (M dimension)
    pub m_size: u32,
    /// Number of columns in matrices B and C (N dimension)
    pub n_size: u32,
    /// Number of columns in A / rows in B (K dimension)
    pub k_size: u32,
    /// Element type for input matrices A and B
    pub ab_type: CooperativeScalarType,
    /// Element type for accumulator matrix C and the result
    pub cr_type: CooperativeScalarType,
    /// Whether saturating accumulation is supported.
    ///
    /// When true, the multiply-add operation clamps the result to prevent overflow.
    pub saturating_accumulation: bool,
}
