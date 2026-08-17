//! Shared types for WebGPU errors. See also:
//! <https://gpuweb.github.io/gpuweb/#errors-and-debugging>

/// A classification of WebGPU error for implementers of the WebGPU API to use in their own error
/// layer(s).
///
/// Strongly correlates to the [`GPUError`] and [`GPUErrorFilter`] types in the WebGPU API, with an
/// additional [`Self::DeviceLost`] variant.
///
/// [`GPUError`]: https://gpuweb.github.io/gpuweb/#gpuerror
/// [`GPUErrorFilter`]: https://gpuweb.github.io/gpuweb/#enumdef-gpuerrorfilter
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ErrorType {
    /// A [`GPUInternalError`].
    ///
    /// [`GPUInternalError`]: https://gpuweb.github.io/gpuweb/#gpuinternalerror
    Internal,
    /// A [`GPUOutOfMemoryError`].
    ///
    /// [`GPUOutOfMemoryError`]: https://gpuweb.github.io/gpuweb/#gpuoutofmemoryerror
    OutOfMemory,
    /// A [`GPUValidationError`].
    ///
    /// [`GPUValidationError`]: https://gpuweb.github.io/gpuweb/#gpuvalidationerror
    Validation,
    /// Indicates that device loss occurred. In JavaScript, this means the [`GPUDevice.lost`]
    /// property should be `resolve`d.
    ///
    /// [`GPUDevice.lost`]: https://www.w3.org/TR/webgpu/#dom-gpudevice-lost
    DeviceLost,
}

/// A trait for querying the [`ErrorType`] classification of an error.
///
/// This is intended to be used as a convenience by implementations of WebGPU to classify errors
/// returned by [`wgpu_core`](crate).
pub trait WebGpuError: core::error::Error + 'static {
    /// Determine the classification of this error as a WebGPU [`ErrorType`].
    fn webgpu_error_type(&self) -> ErrorType;
}
