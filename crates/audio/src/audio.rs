use std::time::Duration;

use rodio::{ChannelCount, SampleRate, nz};

pub const REPLAY_DURATION: Duration = Duration::from_secs(30);
pub const SAMPLE_RATE: SampleRate = nz!(48000);
pub const CHANNEL_COUNT: ChannelCount = nz!(2);

#[cfg(not(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd")))]
mod audio_pipeline;

#[cfg(any(all(target_os = "windows", target_env = "gnu"), target_os = "freebsd"))]
mod audio_pipeline {
}

pub use audio_pipeline::{Audio, AudioSettings, Sound, VoipParts};
pub use audio_pipeline::{AudioDeviceInfo, AvailableAudioDevices};
pub use audio_pipeline::{resolve_device, ensure_devices_initialized};
// TODO(audio) replace with input test functionallity in thi audio crate
pub use audio_pipeline::{open_input_stream, open_test_output};
pub use audio_pipeline::RodioExt;
pub use audio_pipeline::init;
