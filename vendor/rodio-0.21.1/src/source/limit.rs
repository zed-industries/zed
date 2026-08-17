//! Audio peak limiting for dynamic range control.
//!
//! This module implements a feedforward limiter that prevents audio peaks from exceeding
//! a specified threshold while maintaining audio quality. The limiter is based on:
//! Giannoulis, D., Massberg, M., & Reiss, J.D. (2012). Digital Dynamic Range Compressor Design,
//! A Tutorial and Analysis. Journal of The Audio Engineering Society, 60, 399-408.
//!
//! # What is Limiting?
//!
//! A limiter reduces the amplitude of audio signals that exceed a threshold level.
//! For example, with a -6dB threshold, peaks above that level are reduced to stay near the
//! threshold, preventing clipping and maintaining consistent output levels.
//!
//! # Features
//!
//! * **Soft-knee limiting** - Gradual transition into limiting for natural sound
//! * **Per-channel detection** - Decoupled peak detection per channel
//! * **Coupled gain reduction** - Uniform gain reduction across channels preserves stereo imaging
//! * **Configurable timing** - Adjustable attack/release times for different use cases
//! * **Efficient processing** - Optimized implementations for mono, stereo, and multi-channel audio
//!
//! # Usage
//!
//! Use [`LimitSettings`] to configure the limiter, then apply it to any audio source:
//!
//! ```rust
//! use rodio::source::{SineWave, Source, LimitSettings};
//! use std::time::Duration;
//!
//! // Create a loud sine wave
//! let source = SineWave::new(440.0).amplify(2.0);
//!
//! // Apply limiting with -6dB threshold
//! let settings = LimitSettings::default().with_threshold(-6.0);
//! let limited = source.limit(settings);
//! ```
//!
//! # Presets
//!
//! [`LimitSettings`] provides optimized presets for common use cases:
//!
//! * [`LimitSettings::default()`] - General-purpose limiting (-1 dBFS, balanced)
//! * [`LimitSettings::dynamic_content()`] - Music and sound effects (-3 dBFS, transparent)
//! * [`LimitSettings::broadcast()`] - Streaming and voice chat (fast response, consistent)
//! * [`LimitSettings::mastering()`] - Final production stage (-0.5 dBFS, tight peak control)
//! * [`LimitSettings::gaming()`] - Interactive audio (-3 dBFS, responsive dynamics)
//! * [`LimitSettings::live_performance()`] - Real-time applications (ultra-fast protection)
//!
//! ```rust
//! use rodio::source::{SineWave, Source, LimitSettings};
//!
//! // Use preset optimized for music
//! let music = SineWave::new(440.0).amplify(1.5);
//! let limited_music = music.limit(LimitSettings::dynamic_content());
//!
//! // Use preset optimized for streaming
//! let stream = SineWave::new(440.0).amplify(2.0);
//! let limited_stream = stream.limit(LimitSettings::broadcast());
//! ```

use std::time::Duration;

use super::SeekError;
use crate::{
    common::{ChannelCount, Sample, SampleRate},
    math::{self, duration_to_coefficient},
    Float, Source,
};

/// Configuration settings for audio limiting.
///
/// This struct defines how the limiter behaves, including when to start limiting
/// (threshold), how gradually to apply it (knee width), and how quickly to respond
/// to level changes (attack/release times).
///
/// # Parameters
///
/// * **Threshold** - Level in dB where limiting begins (must be negative, typically -1 to -6 dB)
/// * **Knee Width** - Range in dB over which limiting gradually increases (wider = smoother)
/// * **Attack** - Time to respond to level increases (shorter = faster but may distort)
/// * **Release** - Time to recover after level decreases (longer = smoother)
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use rodio::source::{SineWave, Source, LimitSettings};
/// use std::time::Duration;
///
/// // Use default settings (-1 dB threshold, 4 dB knee, 5ms attack, 100ms release)
/// let source = SineWave::new(440.0).amplify(2.0);
/// let limited = source.limit(LimitSettings::default());
/// ```
///
/// ## Custom Settings with Builder Pattern
///
/// ```rust
/// use rodio::source::{SineWave, Source, LimitSettings};
/// use std::time::Duration;
///
/// let source = SineWave::new(440.0).amplify(3.0);
/// let settings = LimitSettings::new()
///     .with_threshold(-6.0)                    // Limit peaks above -6dB
///     .with_knee_width(2.0)                    // 2dB soft knee for smooth limiting
///     .with_attack(Duration::from_millis(3))   // Fast 3ms attack
///     .with_release(Duration::from_millis(50)); // 50ms release
///
/// let limited = source.limit(settings);
/// ```
///
/// ## Common Adjustments
///
/// ```rust
/// use rodio::source::LimitSettings;
/// use std::time::Duration;
///
/// // More headroom for dynamic content
/// let conservative = LimitSettings::default()
///     .with_threshold(-3.0)                    // More headroom
///     .with_knee_width(6.0);                   // Wide knee for transparency
///
/// // Tighter control for broadcast/streaming
/// let broadcast = LimitSettings::default()
///     .with_knee_width(2.0)                     // Narrower knee for firmer limiting
///     .with_attack(Duration::from_millis(3))    // Faster attack
///     .with_release(Duration::from_millis(50)); // Faster release
/// ```
#[derive(Debug, Clone)]
/// Configuration settings for audio limiting.
///
/// # dB vs. dBFS Reference
///
/// This limiter uses **dBFS (decibels relative to Full Scale)** for all level measurements:
/// - **0 dBFS** = maximum possible digital level (1.0 in linear scale)
/// - **Negative dBFS** = levels below maximum (e.g., -6 dBFS = 0.5 in linear scale)
/// - **Positive dBFS** = levels above maximum (causes digital clipping)
///
/// Unlike absolute dB measurements (dB SPL), dBFS is relative to the digital system's
/// maximum representable value, making it the standard for digital audio processing.
///
/// ## Common dBFS Reference Points
/// - **0 dBFS**: Digital maximum (clipping threshold)
/// - **-1 dBFS**: Just below clipping (tight limiting)
/// - **-3 dBFS**: Moderate headroom (balanced limiting)
/// - **-6 dBFS**: Generous headroom (gentle limiting)
/// - **-12 dBFS**: Conservative level (preserves significant dynamics)
/// - **-20 dBFS**: Very quiet level (background/ambient sounds)
pub struct LimitSettings {
    /// Level where limiting begins (dBFS, must be negative).
    ///
    /// Specifies the threshold in dBFS where the limiter starts to reduce gain:
    /// - `-1.0` = limit at -1 dBFS (tight limiting, prevents clipping)
    /// - `-3.0` = limit at -3 dBFS (balanced approach with headroom)
    /// - `-6.0` = limit at -6 dBFS (gentle limiting, preserves dynamics)
    ///
    /// Values must be negative - positive values would attempt limiting above
    /// 0 dBFS, which cannot prevent clipping.
    pub threshold: Float,
    /// Range over which limiting gradually increases (dB).
    ///
    /// Defines the transition zone width in dB where limiting gradually increases
    /// from no effect to full limiting:
    /// - `0.0` = hard limiting (abrupt transition)
    /// - `2.0` = moderate knee (some gradual transition)
    /// - `4.0` = soft knee (smooth, transparent transition)
    /// - `8.0` = very soft knee (very gradual, musical transition)
    pub knee_width: Float,
    /// Time to respond to level increases
    pub attack: Duration,
    /// Time to recover after level decreases
    pub release: Duration,
}

impl Default for LimitSettings {
    fn default() -> Self {
        Self {
            threshold: -1.0,                     // -1 dB
            knee_width: 4.0,                     // 4 dB
            attack: Duration::from_millis(5),    // 5 ms
            release: Duration::from_millis(100), // 100 ms
        }
    }
}

impl LimitSettings {
    /// Creates new limit settings with default values.
    ///
    /// Equivalent to [`LimitSettings::default()`].
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates settings optimized for dynamic content like music and sound effects.
    ///
    /// Designed for content with varying dynamics where you want to preserve
    /// the natural feel while preventing occasional peaks from clipping.
    ///
    /// # Configuration
    ///
    /// - **Threshold**: -3.0 dBFS (more headroom than default)
    /// - **Knee width**: 6.0 dB (wide, transparent transition)
    /// - **Attack**: 5 ms (default, balanced response)
    /// - **Release**: 100 ms (default, smooth recovery)
    ///
    /// # Use Cases
    ///
    /// - Music playback with occasional loud peaks
    /// - Sound effects that need natural dynamics
    /// - Content where transparency is more important than tight control
    /// - Game audio with varying intensity levels
    ///
    /// # Examples
    ///
    /// ```
    /// use rodio::source::{SineWave, Source, LimitSettings};
    ///
    /// let music = SineWave::new(440.0).amplify(1.5);
    /// let limited = music.limit(LimitSettings::dynamic_content());
    /// ```
    #[inline]
    pub fn dynamic_content() -> Self {
        Self::default()
            .with_threshold(-3.0) // More headroom for dynamics
            .with_knee_width(6.0) // Wide knee for transparency
    }

    /// Creates settings optimized for broadcast and streaming applications.
    ///
    /// Designed for consistent loudness and reliable peak control in scenarios
    /// where clipping absolutely cannot occur and consistent levels are critical.
    ///
    /// # Configuration
    ///
    /// - **Threshold**: -1.0 dBFS (default, tight control)
    /// - **Knee width**: 2.0 dB (narrower, more decisive limiting)
    /// - **Attack**: 3 ms (faster response to catch transients)
    /// - **Release**: 50 ms (faster recovery for consistent levels)
    ///
    /// # Use Cases
    ///
    /// - Live streaming where clipping would be catastrophic
    /// - Broadcast audio that must meet loudness standards
    /// - Voice chat applications requiring consistent levels
    /// - Podcast production for consistent listening experience
    /// - Game voice communication systems
    ///
    /// # Examples
    ///
    /// ```
    /// use rodio::source::{SineWave, Source, LimitSettings};
    ///
    /// let voice_chat = SineWave::new(440.0).amplify(2.0);
    /// let limited = voice_chat.limit(LimitSettings::broadcast());
    /// ```
    #[inline]
    pub fn broadcast() -> Self {
        Self::default()
            .with_knee_width(2.0) // Narrower knee for decisive limiting
            .with_attack(Duration::from_millis(3)) // Faster attack for transients
            .with_release(Duration::from_millis(50)) // Faster recovery for consistency
    }

    /// Creates settings optimized for mastering and final audio production.
    ///
    /// Designed for the final stage of audio production where tight peak control
    /// is needed while maintaining audio quality and preventing any clipping.
    ///
    /// # Configuration
    ///
    /// - **Threshold**: -0.5 dBFS (very tight, maximum loudness)
    /// - **Knee width**: 1.0 dB (narrow, precise control)
    /// - **Attack**: 1 ms (very fast, catches all transients)
    /// - **Release**: 200 ms (slower, maintains natural envelope)
    ///
    /// # Use Cases
    ///
    /// - Final mastering stage for tight peak control
    /// - Preparing audio for streaming platforms (after loudness processing)
    /// - Album mastering where consistent peak levels are critical
    /// - Audio post-production for film/video
    ///
    /// # Examples
    ///
    /// ```
    /// use rodio::source::{SineWave, Source, LimitSettings};
    ///
    /// let master_track = SineWave::new(440.0).amplify(3.0);
    /// let mastered = master_track.limit(LimitSettings::mastering());
    /// ```
    #[inline]
    pub fn mastering() -> Self {
        Self {
            threshold: -0.5,                     // Very tight for peak control
            knee_width: 1.0,                     // Narrow knee for precise control
            attack: Duration::from_millis(1),    // Very fast attack
            release: Duration::from_millis(200), // Slower release for natural envelope
        }
    }

    /// Creates settings optimized for live performance and real-time applications.
    ///
    /// Designed for scenarios where low latency is critical and the limiter
    /// must respond quickly to protect equipment and audiences.
    ///
    /// # Configuration
    ///
    /// - **Threshold**: -2.0 dBFS (some headroom for safety)
    /// - **Knee width**: 3.0 dB (moderate, good compromise)
    /// - **Attack**: 0.5 ms (extremely fast for protection)
    /// - **Release**: 30 ms (fast recovery for live feel)
    ///
    /// # Use Cases
    ///
    /// - Live concert sound reinforcement
    /// - DJ mixing and live electronic music
    /// - Real-time audio processing where latency matters
    /// - Equipment protection in live settings
    /// - Interactive audio applications and games
    ///
    /// # Examples
    ///
    /// ```
    /// use rodio::source::{SineWave, Source, LimitSettings};
    ///
    /// let live_input = SineWave::new(440.0).amplify(2.5);
    /// let protected = live_input.limit(LimitSettings::live_performance());
    /// ```
    #[inline]
    pub fn live_performance() -> Self {
        Self {
            threshold: -2.0,                    // Some headroom for safety
            knee_width: 3.0,                    // Moderate knee
            attack: Duration::from_micros(500), // Extremely fast for protection
            release: Duration::from_millis(30), // Fast recovery for live feel
        }
    }

    /// Creates settings optimized for gaming and interactive audio.
    ///
    /// Designed for games where audio levels can vary dramatically between
    /// quiet ambient sounds and loud action sequences, requiring responsive
    /// limiting that maintains immersion.
    ///
    /// # Configuration
    ///
    /// - **Threshold**: -3.0 dBFS (balanced headroom for dynamic range)
    /// - **Knee width**: 3.0 dB (moderate transition for natural feel)
    /// - **Attack**: 2 ms (fast enough for sound effects, not harsh)
    /// - **Release**: 75 ms (quick recovery for interactive responsiveness)
    ///
    /// # Use Cases
    ///
    /// - Game audio mixing for consistent player experience
    /// - Interactive audio applications requiring dynamic response
    /// - VR/AR audio where sudden loud sounds could be jarring
    /// - Mobile games needing battery-efficient processing
    /// - Streaming gameplay audio for viewers
    ///
    /// # Examples
    ///
    /// ```
    /// use rodio::source::{SineWave, Source, LimitSettings};
    ///
    /// let game_audio = SineWave::new(440.0).amplify(2.0);
    /// let limited = game_audio.limit(LimitSettings::gaming());
    /// ```
    #[inline]
    pub fn gaming() -> Self {
        Self {
            threshold: -3.0,                    // Balanced headroom for dynamics
            knee_width: 3.0,                    // Moderate for natural feel
            attack: Duration::from_millis(2),   // Fast but not harsh
            release: Duration::from_millis(75), // Quick for interactivity
        }
    }

    /// Sets the threshold level where limiting begins.
    ///
    /// # Arguments
    ///
    /// * `threshold` - Level in dBFS where limiting starts (must be negative)
    ///   - `-1.0` = limiting starts at -1 dBFS (tight limiting, prevents clipping)
    ///   - `-3.0` = limiting starts at -3 dBFS (balanced approach with headroom)
    ///   - `-6.0` = limiting starts at -6 dBFS (gentle limiting, preserves dynamics)
    ///   - `-12.0` = limiting starts at -12 dBFS (very aggressive, significantly reduces dynamics)
    ///
    /// # dBFS Context
    ///
    /// Remember that 0 dBFS is the digital maximum. Negative dBFS values represent
    /// levels below this maximum:
    /// - `-1 dBFS` ≈ 89% of maximum amplitude (very loud, limiting triggers late)
    /// - `-3 dBFS` ≈ 71% of maximum amplitude (loud, moderate limiting)
    /// - `-6 dBFS` ≈ 50% of maximum amplitude (moderate, gentle limiting)
    /// - `-12 dBFS` ≈ 25% of maximum amplitude (quiet, aggressive limiting)
    ///
    /// Lower thresholds (more negative) trigger limiting earlier and reduce dynamics more.
    /// Only negative values are meaningful - positive values would attempt limiting
    /// above 0 dBFS, which cannot prevent clipping.
    #[inline]
    pub fn with_threshold(mut self, threshold: Float) -> Self {
        self.threshold = threshold;
        self
    }

    /// Sets the knee width - range over which limiting gradually increases.
    ///
    /// # Arguments
    ///
    /// * `knee_width` - Range in dB over which limiting transitions from off to full effect
    ///   - `0.0` dB = hard knee (abrupt limiting, may sound harsh)
    ///   - `1.0-2.0` dB = moderate knee (noticeable but controlled limiting)
    ///   - `4.0` dB = soft knee (smooth, transparent limiting) \[default\]
    ///   - `6.0-8.0` dB = very soft knee (very gradual, musical limiting)
    ///
    /// # How Knee Width Works
    ///
    /// The knee creates a transition zone around the threshold. For example, with
    /// `threshold = -3.0` dBFS and `knee_width = 4.0` dB:
    /// - No limiting below -5 dBFS (threshold - knee_width/2)
    /// - Gradual limiting from -5 dBFS to -1 dBFS
    /// - Full limiting above -1 dBFS (threshold + knee_width/2)
    #[inline]
    pub fn with_knee_width(mut self, knee_width: Float) -> Self {
        self.knee_width = knee_width;
        self
    }

    /// Sets the attack time - how quickly the limiter responds to level increases.
    ///
    /// # Arguments
    ///
    /// * `attack` - Time duration for the limiter to react to peaks
    ///   - Shorter (1-5 ms) = faster response, may cause distortion
    ///   - Longer (10-20 ms) = smoother sound, may allow brief overshoots
    #[inline]
    pub fn with_attack(mut self, attack: Duration) -> Self {
        self.attack = attack;
        self
    }

    /// Sets the release time - how quickly the limiter recovers after level decreases.
    ///
    /// # Arguments
    ///
    /// * `release` - Time duration for the limiter to stop limiting
    ///   - Shorter (10-50 ms) = quick recovery, may sound pumping
    ///   - Longer (100-500 ms) = smooth recovery, more natural sound
    #[inline]
    pub fn with_release(mut self, release: Duration) -> Self {
        self.release = release;
        self
    }
}

/// Creates a limiter that processes the input audio source.
///
/// This function applies the specified limiting settings to control audio peaks.
/// The limiter uses feedforward processing with configurable attack/release times
/// and soft-knee characteristics for natural-sounding dynamic range control.
///
/// # Arguments
///
/// * `input` - Audio source to process
/// * `settings` - Limiter configuration (threshold, knee, timing)
///
/// # Returns
///
/// A [`Limit`] source that applies the limiting to the input audio.
///
/// # Example
///
/// ```rust
/// use rodio::source::{SineWave, Source, LimitSettings};
///
/// let source = SineWave::new(440.0).amplify(2.0);
/// let settings = LimitSettings::default().with_threshold(-6.0);
/// let limited = source.limit(settings);
/// ```
pub(crate) fn limit<I: Source>(input: I, settings: LimitSettings) -> Limit<I> {
    let sample_rate = input.sample_rate();
    let attack = duration_to_coefficient(settings.attack, sample_rate);
    let release = duration_to_coefficient(settings.release, sample_rate);
    let channels = input.channels().get() as usize;

    let base = LimitBase::new(settings.threshold, settings.knee_width, attack, release);

    let inner = match channels {
        1 => LimitInner::Mono(LimitMono {
            input,
            base,
            limiter_integrator: 0.0,
            limiter_peak: 0.0,
        }),
        2 => LimitInner::Stereo(LimitStereo {
            input,
            base,
            limiter_integrators: [0.0; 2],
            limiter_peaks: [0.0; 2],
            position: 0,
        }),
        n => LimitInner::MultiChannel(LimitMulti {
            input,
            base,
            limiter_integrators: vec![0.0; n],
            limiter_peaks: vec![0.0; n],
            position: 0,
        }),
    };

    Limit(inner)
}

/// A source filter that applies audio limiting to prevent peaks from exceeding a threshold.
///
/// This filter reduces the amplitude of audio signals that exceed the configured threshold
/// level, helping to prevent clipping and maintain consistent output levels. The limiter
/// automatically adapts to mono, stereo, or multi-channel audio sources by using the
/// appropriate internal implementation.
///
/// # How It Works
///
/// The limiter detects peaks in each audio channel independently but applies gain reduction
/// uniformly across all channels. This preserves stereo imaging while ensuring that loud
/// peaks in any channel are controlled. The limiting uses:
///
/// - **Soft-knee compression**: Gradual gain reduction around the threshold
/// - **Attack/release timing**: Configurable response speed to level changes
/// - **Peak detection**: Tracks maximum levels across all channels
/// - **Gain smoothing**: Prevents audible artifacts from rapid gain changes
///
/// # Created By
///
/// Use [`Source::limit()`] with [`LimitSettings`] to create a `Limit` source:
///
/// ```
/// use rodio::source::{SineWave, Source};
/// use rodio::source::LimitSettings;
/// use std::time::Duration;
///
/// let source = SineWave::new(440.0).amplify(2.0);
/// let settings = LimitSettings::default()
///     .with_threshold(-6.0)  // -6 dBFS threshold
///     .with_attack(Duration::from_millis(5))
///     .with_release(Duration::from_millis(100));
/// let limited = source.limit(settings);
/// ```
///
/// # Performance
///
/// The limiter automatically selects the most efficient implementation based on channel count:
/// - **Mono**: Single-channel optimized processing
/// - **Stereo**: Two-channel optimized with interleaved processing
/// - **Multi-channel**: Generic implementation for 3+ channels
///
/// # Channel Count Stability
///
/// **Important**: The limiter is optimized for sources with fixed channel counts.
/// Most audio files (music, podcasts, etc.) maintain constant channel counts,
/// making this optimization safe and beneficial.
///
/// If the underlying source changes channel count mid-stream (rare), the limiter
/// will continue to function but performance may be degraded. For such cases,
/// recreate the limiter when the channel count changes.
///
/// # Type Parameters
///
/// * `I` - The input audio source type that implements [`Source`]
#[derive(Clone, Debug)]
pub struct Limit<I>(LimitInner<I>)
where
    I: Source;

impl<I> Source for Limit<I>
where
    I: Source,
{
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.0.current_span_len()
    }

    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.0.sample_rate()
    }

    #[inline]
    fn channels(&self) -> ChannelCount {
        self.0.channels()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.0.total_duration()
    }

    #[inline]
    fn try_seek(&mut self, position: Duration) -> Result<(), SeekError> {
        self.0.try_seek(position)
    }
}

impl<I> Limit<I>
where
    I: Source,
{
    /// Returns a reference to the inner audio source.
    ///
    /// This allows access to the original source's properties and methods without
    /// consuming the limiter. Useful for inspecting source characteristics like
    /// sample rate, channels, or duration.
    ///
    /// Useful for inspecting source properties without consuming the filter.
    #[inline]
    pub fn inner(&self) -> &I {
        self.0.inner()
    }

    /// Returns a mutable reference to the inner audio source.
    ///
    /// This allows modification of the original source while keeping the limiter
    /// wrapper. Essential for operations like seeking that need to modify the
    /// underlying source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        self.0.inner_mut()
    }

    /// Consumes the limiter and returns the inner audio source.
    ///
    /// This dismantles the limiter wrapper to extract the original source,
    /// allowing the audio pipeline to continue without limiting overhead.
    /// Useful when limiting is no longer needed but the source should continue.
    #[inline]
    pub fn into_inner(self) -> I {
        self.0.into_inner()
    }
}

impl<I> Iterator for Limit<I>
where
    I: Source,
{
    type Item = I::Item;

    /// Provides the next limited sample.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }

    /// Provides size hints from the inner limiter.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.0.size_hint()
    }
}

/// Internal limiter implementation that adapts to different channel configurations.
///
/// This enum is private and automatically selects the most efficient implementation
/// based on the number of audio channels:
/// - **Mono**: Single-channel optimized processing with minimal state
/// - **Stereo**: Two-channel optimized with fixed-size arrays for performance
/// - **Multi-channel**: Generic implementation using vectors for arbitrary channel counts
///
/// The enum is wrapped by the public [`Limit`] struct to provide a clean API while
/// maintaining internal optimization flexibility.
///
/// # Channel-Specific Optimizations
///
/// - **Mono**: Direct processing without channel indexing overhead
/// - **Stereo**: Fixed-size arrays avoid heap allocation and provide cache efficiency
/// - **Multi-channel**: Dynamic vectors handle surround sound and custom configurations
///
/// # Type Parameters
///
/// * `I` - The input audio source type that implements [`Source`]
#[derive(Clone, Debug)]
enum LimitInner<I: Source>
where
    I: Source,
{
    /// Mono channel limiter
    Mono(LimitMono<I>),
    /// Stereo channel limiter
    Stereo(LimitStereo<I>),
    /// Multi-channel limiter for arbitrary channel counts
    MultiChannel(LimitMulti<I>),
}

/// Common parameters and processing logic shared across all limiter variants.
///
/// Handles:
/// * Parameter storage (threshold, knee width, attack/release coefficients)
/// * Per-channel state updates for peak detection
/// * Gain computation through soft-knee limiting
#[derive(Clone, Debug)]
struct LimitBase {
    /// Level where limiting begins (dB)
    threshold: Float,
    /// Width of the soft-knee region (dB)
    knee_width: Float,
    /// Inverse of 8 times the knee width (precomputed for efficiency)
    inv_knee_8: Float,
    /// Attack time constant (ms)
    attack: Float,
    /// Release time constant (ms)
    release: Float,
}

/// Mono channel limiter optimized for single-channel processing.
///
/// This variant is automatically selected by [`Limit`] for mono audio sources.
/// It uses minimal state (single integrator and peak detector) for optimal
/// performance with single-channel audio.
///
/// # Internal Use
///
/// This struct is used internally by [`LimitInner::Mono`] and is not intended
/// for direct construction. Use [`Source::limit()`] instead.
#[derive(Clone, Debug)]
pub struct LimitMono<I> {
    /// Input audio source
    input: I,
    /// Common limiter parameters
    base: LimitBase,
    /// Peak detection integrator state
    limiter_integrator: Float,
    /// Peak detection state
    limiter_peak: Float,
}

/// Stereo channel limiter with optimized two-channel processing.
///
/// This variant is automatically selected by [`Limit`] for stereo audio sources.
/// It uses fixed-size arrays instead of vectors for better cache performance
/// and avoids heap allocation overhead common in stereo audio processing.
///
/// # Performance
///
/// The fixed arrays and channel position tracking provide optimal performance
/// for interleaved stereo sample processing, avoiding the dynamic allocation
/// overhead of the multi-channel variant.
///
/// # Internal Use
///
/// This struct is used internally by [`LimitInner::Stereo`] and is not intended
/// for direct construction. Use [`Source::limit()`] instead.
#[derive(Clone, Debug)]
pub struct LimitStereo<I> {
    /// Input audio source
    input: I,
    /// Common limiter parameters
    base: LimitBase,
    /// Peak detection integrator states for left and right channels
    limiter_integrators: [Float; 2],
    /// Peak detection states for left and right channels
    limiter_peaks: [Float; 2],
    /// Current channel position (0 = left, 1 = right)
    position: u8,
}

/// Generic multi-channel limiter for surround sound or other configurations.
///
/// This variant is automatically selected by [`Limit`] for audio sources with
/// 3 or more channels. It uses dynamic vectors to handle arbitrary channel
/// counts, making it suitable for surround sound (5.1, 7.1) and other
/// multi-channel audio configurations.
///
/// # Flexibility vs Performance
///
/// While this variant has slightly more overhead than the mono/stereo variants
/// due to vector allocation and dynamic indexing, it provides the flexibility
/// needed for complex audio setups while maintaining good performance.
///
/// # Internal Use
///
/// This struct is used internally by [`LimitInner::MultiChannel`] and is not
/// intended for direct construction. Use [`Source::limit()`] instead.
#[derive(Clone, Debug)]
pub struct LimitMulti<I> {
    /// Input audio source
    input: I,
    /// Common limiter parameters
    base: LimitBase,
    /// Peak detector integrator states (one per channel)
    limiter_integrators: Vec<Float>,
    /// Peak detector states (one per channel)
    limiter_peaks: Vec<Float>,
    /// Current channel position (0 to channels-1)
    position: usize,
}

/// Computes the gain reduction amount in dB based on input level.
///
/// Implements soft-knee compression with three regions:
/// 1. Below threshold - knee_width: No compression (returns 0.0)
/// 2. Within knee region: Gradual compression with quadratic curve
/// 3. Above threshold + knee_width: Linear compression
///
/// Optimized for the most common case where samples are below threshold and no limiting is needed
/// (returns `0.0` early).
///
/// # Arguments
///
/// * `sample` - Input sample value (with initial gain applied)
/// * `threshold` - Level where limiting begins (dB)
/// * `knee_width` - Width of soft knee region (dB)
/// * `inv_knee_8` - Precomputed value: 1.0 / (8.0 * knee_width) for efficiency
///
/// # Returns
///
/// Amount of gain reduction to apply in dB
#[inline]
fn process_sample(
    sample: Sample,
    threshold: Float,
    knee_width: Float,
    inv_knee_8: Float,
) -> Sample {
    // Add slight DC offset. Some samples are silence, which is -inf dB and gets the limiter stuck.
    // Adding a small positive offset prevents this.
    let bias_db = math::linear_to_db(sample.abs() + Sample::MIN_POSITIVE) - threshold;
    let knee_boundary_db = bias_db * 2.0;
    if knee_boundary_db < -knee_width {
        0.0
    } else if knee_boundary_db.abs() <= knee_width {
        // Faster than powi(2)
        let x = knee_boundary_db + knee_width;
        x * x * inv_knee_8
    } else {
        bias_db
    }
}

impl LimitBase {
    fn new(threshold: Float, knee_width: Float, attack: Float, release: Float) -> Self {
        let inv_knee_8 = 1.0 / (8.0 * knee_width);
        Self {
            threshold,
            knee_width,
            inv_knee_8,
            attack,
            release,
        }
    }

    /// Updates the channel's envelope detection state.
    ///
    /// For each channel, processes:
    /// 1. Initial gain and dB conversion
    /// 2. Soft-knee limiting calculation
    /// 3. Envelope detection with attack/release filtering
    /// 4. Peak level tracking
    ///
    /// The envelope detection uses a dual-stage approach:
    /// - First stage: Max of current signal and smoothed release
    /// - Second stage: Attack smoothing of the peak detector output
    ///
    /// Note: Only updates state, gain application is handled by the variant implementations to
    /// allow for coupled gain reduction across channels.
    #[must_use]
    #[inline]
    fn process_channel(&self, sample: Sample, integrator: &mut Float, peak: &mut Float) -> Sample {
        // step 1-4: half-wave rectification and conversion into dB, and gain computer with soft
        // knee and subtractor
        let limiter_db = process_sample(sample, self.threshold, self.knee_width, self.inv_knee_8);

        // step 5: smooth, decoupled peak detector
        *integrator = Float::max(
            limiter_db,
            self.release * *integrator + (1.0 - self.release) * limiter_db,
        );
        *peak = self.attack * *peak + (1.0 - self.attack) * *integrator;

        sample
    }
}

impl<I> LimitMono<I>
where
    I: Source,
{
    /// Processes the next mono sample through the limiter.
    ///
    /// Single channel implementation with direct state updates.
    #[inline]
    fn process_next(&mut self, sample: I::Item) -> I::Item {
        let processed =
            self.base
                .process_channel(sample, &mut self.limiter_integrator, &mut self.limiter_peak);

        // steps 6-8: conversion into level and multiplication into gain stage
        processed * math::db_to_linear(-self.limiter_peak)
    }
}

impl<I> LimitStereo<I>
where
    I: Source,
{
    /// Processes the next stereo sample through the limiter.
    ///
    /// Uses efficient channel position tracking with XOR toggle and direct array access for state
    /// updates.
    #[inline]
    fn process_next(&mut self, sample: I::Item) -> I::Item {
        let channel = self.position as usize;
        self.position ^= 1;

        let processed = self.base.process_channel(
            sample,
            &mut self.limiter_integrators[channel],
            &mut self.limiter_peaks[channel],
        );

        // steps 6-8: conversion into level and multiplication into gain stage. Find maximum peak
        // across both channels to couple the gain and maintain stereo imaging.
        let max_peak = Float::max(self.limiter_peaks[0], self.limiter_peaks[1]);
        processed * math::db_to_linear(-max_peak)
    }
}

impl<I> LimitMulti<I>
where
    I: Source,
{
    /// Processes the next multi-channel sample through the limiter.
    ///
    /// Generic implementation supporting arbitrary channel counts with `Vec`-based state storage.
    #[inline]
    fn process_next(&mut self, sample: I::Item) -> I::Item {
        let channel = self.position;
        self.position = (self.position + 1) % self.limiter_integrators.len();

        let processed = self.base.process_channel(
            sample,
            &mut self.limiter_integrators[channel],
            &mut self.limiter_peaks[channel],
        );

        // steps 6-8: conversion into level and multiplication into gain stage. Find maximum peak
        // across all channels to couple the gain and maintain multi-channel imaging.
        let max_peak = self
            .limiter_peaks
            .iter()
            .fold(0.0, |max, &peak| Float::max(max, peak));
        processed * math::db_to_linear(-max_peak)
    }
}

impl<I> LimitInner<I>
where
    I: Source,
{
    /// Returns a reference to the inner audio source.
    ///
    /// This allows access to the original source's properties and methods without
    /// consuming the limiter. Useful for inspecting source characteristics like
    /// sample rate, channels, or duration.
    #[inline]
    pub fn inner(&self) -> &I {
        match self {
            LimitInner::Mono(mono) => &mono.input,
            LimitInner::Stereo(stereo) => &stereo.input,
            LimitInner::MultiChannel(multi) => &multi.input,
        }
    }

    /// Returns a mutable reference to the inner audio source.
    ///
    /// This allows modification of the original source while keeping the limiter
    /// wrapper. Essential for operations like seeking that need to modify the
    /// underlying source.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut I {
        match self {
            LimitInner::Mono(mono) => &mut mono.input,
            LimitInner::Stereo(stereo) => &mut stereo.input,
            LimitInner::MultiChannel(multi) => &mut multi.input,
        }
    }

    /// Consumes the filter and returns the inner audio source.
    ///
    /// This dismantles the limiter wrapper to extract the original source,
    /// allowing the audio pipeline to continue without limiting overhead.
    /// Useful when limiting is no longer needed but the source should continue.
    #[inline]
    pub fn into_inner(self) -> I {
        match self {
            LimitInner::Mono(mono) => mono.input,
            LimitInner::Stereo(stereo) => stereo.input,
            LimitInner::MultiChannel(multi) => multi.input,
        }
    }
}

impl<I> Iterator for LimitInner<I>
where
    I: Source,
{
    type Item = I::Item;

    /// Provides the next processed sample.
    ///
    /// Routes processing to the appropriate channel-specific implementation:
    /// * `Mono`: Direct single-channel processing
    /// * `Stereo`: Optimized two-channel processing
    /// * `MultiChannel`: Generic multi-channel processing
    ///
    /// # Channel Count Changes
    ///
    /// **Important**: This limiter assumes a fixed channel count determined at creation time.
    /// Most audio sources (files, streams) maintain constant channel counts, making this
    /// assumption safe for typical usage.
    ///
    /// If the underlying source changes its channel count mid-stream (rare), the limiter
    /// will continue to function but may experience timing and imaging issues. For optimal
    /// performance, recreate the limiter when the channel count changes.
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            LimitInner::Mono(mono) => {
                let sample = mono.input.next()?;
                Some(mono.process_next(sample))
            }
            LimitInner::Stereo(stereo) => {
                let sample = stereo.input.next()?;
                Some(stereo.process_next(sample))
            }
            LimitInner::MultiChannel(multi) => {
                let sample = multi.input.next()?;
                Some(multi.process_next(sample))
            }
        }
    }

    /// Provides size hints from the inner source.
    ///
    /// Delegates directly to the source to maintain accurate collection sizing.
    /// Used by collection operations for optimization.
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner().size_hint()
    }
}

impl<I> Source for LimitInner<I>
where
    I: Source,
{
    /// Returns the number of samples in the current audio frame.
    ///
    /// Delegates to inner source to maintain frame alignment.
    #[inline]
    fn current_span_len(&self) -> Option<usize> {
        self.inner().current_span_len()
    }

    /// Returns the number of channels in the audio stream.
    ///
    /// Channel count determines which limiter variant is used:
    /// * 1: Mono
    /// * 2: Stereo
    /// * >2: MultiChannel
    #[inline]
    fn channels(&self) -> ChannelCount {
        self.inner().channels()
    }

    /// Returns the audio sample rate in Hz.
    #[inline]
    fn sample_rate(&self) -> SampleRate {
        self.inner().sample_rate()
    }

    /// Returns the total duration of the audio.
    ///
    /// Returns None for streams without known duration.
    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.inner().total_duration()
    }

    /// Attempts to seek to the specified position.
    ///
    /// Resets limiter state to prevent artifacts after seeking:
    /// * Mono: Direct reset of integrator and peak values
    /// * Stereo: Efficient array fill for both channels
    /// * `MultiChannel`: Resets all channel states via fill
    ///
    /// # Arguments
    ///
    /// * `target` - Position to seek to
    ///
    /// # Errors
    ///
    /// Returns error if the underlying source fails to seek
    fn try_seek(&mut self, target: Duration) -> Result<(), SeekError> {
        self.inner_mut().try_seek(target)?;

        match self {
            LimitInner::Mono(mono) => {
                mono.limiter_integrator = 0.0;
                mono.limiter_peak = 0.0;
            }
            LimitInner::Stereo(stereo) => {
                stereo.limiter_integrators.fill(0.0);
                stereo.limiter_peaks.fill(0.0);
            }
            LimitInner::MultiChannel(multi) => {
                multi.limiter_integrators.fill(0.0);
                multi.limiter_peaks.fill(0.0);
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buffer::SamplesBuffer;
    use crate::math::nz;
    use crate::source::{SineWave, Source};
    use std::time::Duration;

    fn create_test_buffer(
        samples: Vec<Sample>,
        channels: ChannelCount,
        sample_rate: SampleRate,
    ) -> SamplesBuffer {
        SamplesBuffer::new(channels, sample_rate, samples)
    }

    #[test]
    fn test_limiter_creation() {
        // Test mono
        let buffer = create_test_buffer(vec![0.5, 0.8, 1.0, 0.3], nz!(1), nz!(44100));
        let limiter = limit(buffer, LimitSettings::default());
        assert_eq!(limiter.channels(), nz!(1));
        assert_eq!(limiter.sample_rate(), nz!(44100));
        matches!(limiter.0, LimitInner::Mono(_));

        // Test stereo
        let buffer = create_test_buffer(
            vec![0.5, 0.8, 1.0, 0.3, 0.2, 0.6, 0.9, 0.4],
            nz!(2),
            nz!(44100),
        );
        let limiter = limit(buffer, LimitSettings::default());
        assert_eq!(limiter.channels(), nz!(2));
        matches!(limiter.0, LimitInner::Stereo(_));

        // Test multichannel
        let buffer = create_test_buffer(vec![0.5; 12], nz!(3), nz!(44100));
        let limiter = limit(buffer, LimitSettings::default());
        assert_eq!(limiter.channels(), nz!(3));
        matches!(limiter.0, LimitInner::MultiChannel(_));
    }
}
