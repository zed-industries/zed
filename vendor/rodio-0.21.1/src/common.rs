use std::fmt::{Debug, Display};
use std::num::NonZero;

/// Sample rate (a frame rate or samples per second per channel).
pub type SampleRate = NonZero<u32>;

/// Number of channels in a stream. Can never be Zero
pub type ChannelCount = NonZero<u16>;

/// Number of bits per sample. Can never be zero.
pub type BitDepth = NonZero<u32>;

// NOTE on numeric precision:
//
// While `f32` is transparent for typical playback use cases, it does not guarantee preservation of
// full 24-bit source fidelity across arbitrary processing chains. Each floating-point operation
// rounds its result to `f32` precision (~24-bit significand). In DSP pipelines (filters, mixing,
// modulation), many operations are applied per sample and over time, so rounding noise accumulates
// and long-running state (e.g. oscillator phase) can drift.
//
// For use cases where numerical accuracy must be preserved through extended processing (recording,
// editing, analysis, long-running generators, or complex DSP graphs), enabling 64-bit processing
// reduces accumulated rounding error and drift.
//
// This mirrors common practice in professional audio software and DSP libraries, which often use
// 64-bit internal processing even when the final output is 16- or 24-bit.

/// Floating point type used for internal calculations. Can be configured to be
/// either `f32` (default) or `f64` using the `64bit` feature flag.
#[cfg(not(feature = "64bit"))]
pub type Float = f32;

/// Floating point type used for internal calculations. Can be configured to be
/// either `f32` (default) or `f64` using the `64bit` feature flag.
#[cfg(feature = "64bit")]
pub type Float = f64;

/// Represents value of a single sample.
/// Silence corresponds to the value `0.0`. The expected amplitude range is  -1.0...1.0.
/// Values below and above this range are clipped in conversion to other sample types.
/// Use conversion traits from [dasp_sample] crate or [crate::conversions::SampleTypeConverter]
/// to convert between sample types if necessary.
pub type Sample = Float;

/// Used to test at compile time that a struct/enum implements Send, Sync and
/// is 'static. These are common requirements for dynamic error management
/// libs like color-eyre and anyhow
///
/// # Examples
/// ```compile_fail
/// struct NotSend {
///   foo: Rc<String>,
/// }
///
/// assert_error_traits!(NotSend)
/// ```
///
/// ```compile_fail
/// struct NotSync {
///   foo: std::cell::RefCell<String>,
/// }
/// assert_error_traits!(NotSync)
/// ```
///
/// ```compile_fail
/// struct NotStatic<'a> {
///   foo: &'a str,
/// }
///
/// assert_error_traits!(NotStatic)
/// ```
macro_rules! assert_error_traits {
    ($to_test:path) => {
        const _: () = { $crate::common::use_required_traits::<$to_test>() };
    };
}

pub(crate) use assert_error_traits;
#[allow(dead_code)]
pub(crate) const fn use_required_traits<T: Send + Sync + 'static + Display + Debug + Clone>() {}
