/*!
This module contains functions that convert from one PCM format to another.

This includes conversion between sample formats, channels or sample rates.
*/

pub use self::channels::ChannelCountConverter;
pub use self::sample::SampleTypeConverter;
pub use self::sample_rate::SampleRateConverter;

mod channels;
mod sample;
mod sample_rate;
