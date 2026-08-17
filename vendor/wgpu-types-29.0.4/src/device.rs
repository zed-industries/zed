use core::ops::Range;

#[cfg(any(feature = "serde", test))]
use serde::{Deserialize, Serialize};

/// Describes a [`Device`](../wgpu/struct.Device.html).
///
/// Corresponds to [WebGPU `GPUDeviceDescriptor`](
/// https://gpuweb.github.io/gpuweb/#gpudevicedescriptor).
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct DeviceDescriptor<L> {
    /// Debug label for the device.
    pub label: L,
    /// Specifies the features that are required by the device request.
    /// The request will fail if the adapter cannot provide these features.
    ///
    /// Exactly the specified set of features, and no more or less,
    /// will be allowed in validation of API calls on the resulting device.
    pub required_features: crate::Features,
    /// Specifies the limits that are required by the device request.
    /// The request will fail if the adapter cannot provide these limits.
    ///
    /// Exactly the specified limits, and no better or worse,
    /// will be allowed in validation of API calls on the resulting device.
    pub required_limits: crate::Limits,
    /// Specifies whether `self.required_features` is allowed to contain experimental features.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub experimental_features: crate::ExperimentalFeatures,
    /// Hints for memory allocation strategies.
    pub memory_hints: MemoryHints,
    /// Whether API tracing for debugging is enabled,
    /// and where the trace is written if so.
    pub trace: Trace,
}

impl<L> DeviceDescriptor<L> {
    /// Takes a closure and maps the label of the device descriptor into another.
    #[must_use]
    pub fn map_label<K>(&self, fun: impl FnOnce(&L) -> K) -> DeviceDescriptor<K> {
        DeviceDescriptor {
            label: fun(&self.label),
            required_features: self.required_features,
            required_limits: self.required_limits.clone(),
            experimental_features: self.experimental_features,
            memory_hints: self.memory_hints.clone(),
            trace: self.trace.clone(),
        }
    }
}

/// Hints to the device about the memory allocation strategy.
///
/// Some backends may ignore these hints.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MemoryHints {
    /// Favor performance over memory usage (the default value).
    #[default]
    Performance,
    /// Favor memory usage over performance.
    MemoryUsage,
    /// Applications that have control over the content that is rendered
    /// (typically games) may find an optimal compromise between memory
    /// usage and performance by specifying the allocation configuration.
    Manual {
        /// Defines the range of allowed memory block sizes for sub-allocated
        /// resources.
        ///
        /// The backend may attempt to group multiple resources into fewer
        /// device memory blocks (sub-allocation) for performance reasons.
        /// The start of the provided range specifies the initial memory
        /// block size for sub-allocated resources. After running out of
        /// space in existing memory blocks, the backend may chose to
        /// progressively increase the block size of subsequent allocations
        /// up to a limit specified by the end of the range.
        ///
        /// This does not limit resource sizes. If a resource does not fit
        /// in the specified range, it will typically be placed in a dedicated
        /// memory block.
        suballocated_device_memory_block_size: Range<u64>,
    },
}

/// Controls API call tracing and specifies where the trace is written.
#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
// This enum must be non-exhaustive so that enabling the "trace" feature is not a semver break.
#[non_exhaustive]
pub enum Trace {
    /// Tracing disabled.
    #[default]
    Off,

    /// Write trace to disk.
    #[cfg(feature = "trace")]
    // This must be owned rather than `&'a Path`, because if it were that, then the lifetime
    // parameter would be unused when the "trace" feature is disabled, which is prohibited.
    Directory(std::path::PathBuf),

    /// Store trace in memory.
    #[cfg(feature = "trace")]
    Memory,
}
