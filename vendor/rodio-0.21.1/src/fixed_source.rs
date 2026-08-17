//! Sources of sound and various filters which never change sample rate or
//! channel count.
use std::time::Duration;

use crate::{ChannelCount, Sample, SampleRate};

/// Similar to `Source`, something that can produce interleaved samples for a
/// fixed amount of channels at a fixed sample rate. Those parameters never
/// change.
pub trait FixedSource: Iterator<Item = Sample> {
    /// May NEVER return something else once its returned a value
    fn channels(&self) -> ChannelCount;
    /// May NEVER return something else once its returned a value
    fn sample_rate(&self) -> SampleRate;
    /// Returns the total duration of this source, if known.
    ///
    /// `None` indicates at the same time "infinite" or "unknown".
    fn total_duration(&self) -> Option<Duration>;
}
