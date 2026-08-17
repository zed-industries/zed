#![allow(non_upper_case_globals)]
#![allow(unexpected_cfgs)]
use crate::*;

// CA_PREFER_FIXED_POINT = TARGET_OS_IPHONE && !TARGET_OS_MACCATALYST
#[cfg(all(
    all(target_vendor = "apple", not(target_os = "macos")),
    not(target_env = "macabi"),
))]
type Inner = f32;
#[cfg(not(all(
    all(target_vendor = "apple", not(target_os = "macos")),
    not(target_env = "macabi"),
)))]
type Inner = i32;

#[allow(unused)]
const kAudioUnitSampleFractionBits: AudioFormatFlags = 24;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/audiosampletype?language=objc)
#[deprecated]
pub type AudioSampleType = Inner;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/audiounitsampletype?language=objc)
#[deprecated]
pub type AudioUnitSampleType = Inner;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/kaudioformatflagscanonical?language=objc)
#[cfg(all(
    all(target_vendor = "apple", not(target_os = "macos")),
    not(target_env = "macabi"),
))]
#[deprecated]
pub const kAudioFormatFlagsCanonical: AudioFormatFlags =
    kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/kaudioformatflagsaudiounitcanonical?language=objc)
#[cfg(all(
    all(target_vendor = "apple", not(target_os = "macos")),
    not(target_env = "macabi"),
))]
#[deprecated]
pub const kAudioFormatFlagsAudioUnitCanonical: AudioFormatFlags = kAudioFormatFlagIsFloat
    | kAudioFormatFlagsNativeEndian
    | kAudioFormatFlagIsPacked
    | kAudioFormatFlagIsNonInterleaved;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/kaudioformatflagscanonical?language=objc)
#[cfg(not(all(
    all(target_vendor = "apple", not(target_os = "macos")),
    not(target_env = "macabi"),
)))]
#[deprecated]
pub const kAudioFormatFlagsCanonical: AudioFormatFlags =
    kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;

/// [Apple's documentation](https://developer.apple.com/documentation/coreaudiotypes/kaudioformatflagsaudiounitcanonical?language=objc)
#[cfg(not(all(
    all(target_vendor = "apple", not(target_os = "macos")),
    not(target_env = "macabi"),
)))]
#[deprecated]
pub const kAudioFormatFlagsAudioUnitCanonical: AudioFormatFlags = kAudioFormatFlagIsSignedInteger
    | kAudioFormatFlagsNativeEndian
    | kAudioFormatFlagIsPacked
    | kAudioFormatFlagIsNonInterleaved
    | (kAudioUnitSampleFractionBits << kLinearPCMFormatFlagsSampleFractionShift);
