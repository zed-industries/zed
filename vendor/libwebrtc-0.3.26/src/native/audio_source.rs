// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use cxx::SharedPtr;
use tokio::sync::oneshot;
use webrtc_sys::audio_track as sys_at;

use crate::{audio_frame::AudioFrame, audio_source::AudioSourceOptions, RtcError, RtcErrorType};

#[derive(Clone)]
pub struct NativeAudioSource {
    sys_handle: SharedPtr<sys_at::ffi::AudioTrackSource>,
    sample_rate: u32,
    num_channels: u32,
    queue_size_samples: u32,
}

impl NativeAudioSource {
    /// Creates a new [`NativeAudioSource`].
    ///
    /// # Arguments
    /// * `options` – Configuration options for the source (e.g. echo cancellation, noise suppression).
    /// * `sample_rate` – Sampling rate in Hz (for example, `48000`).
    /// * `num_channels` – Number of audio channels (`1` for mono, `2` for stereo, etc.).
    /// * `queue_size_ms` – Size of the internal buffering queue, in milliseconds.
    ///
    /// # Behavior
    /// - If `queue_size_ms` is **zero**, buffering is **disabled** and audio frames are
    ///   delivered directly to webrtc sinks. In this mode, the caller **must provide 10 ms frames**
    ///   (i.e., `sample_rate / 100` samples per channel) when calling [`capture_frame`].
    /// - If `queue_size_ms` is **non-zero**, buffering is enabled. The value must be a
    ///   **multiple of 10**, representing the total buffering duration in milliseconds.
    ///   Frames will be queued and flushed to sinks asynchronously once the buffer
    ///   reaches the configured threshold.
    ///
    /// # Panics
    /// assert if `queue_size_ms` is not a multiple of 10.
    pub fn new(
        options: AudioSourceOptions,
        sample_rate: u32,
        num_channels: u32,
        queue_size_ms: u32,
    ) -> NativeAudioSource {
        assert!(queue_size_ms % 10 == 0, "queue_size_ms must be a multiple of 10");

        let sys_handle = sys_at::ffi::new_audio_track_source(
            options.into(),
            sample_rate.try_into().unwrap(),
            num_channels.try_into().unwrap(),
            queue_size_ms.try_into().unwrap(),
        );

        let queue_size_samples = (queue_size_ms * sample_rate * num_channels) / 1000;
        Self { sys_handle, sample_rate, num_channels, queue_size_samples }
    }

    pub fn sys_handle(&self) -> SharedPtr<sys_at::ffi::AudioTrackSource> {
        self.sys_handle.clone()
    }

    pub fn set_audio_options(&self, options: AudioSourceOptions) {
        self.sys_handle.set_audio_options(&sys_at::ffi::AudioSourceOptions::from(options))
    }

    pub fn audio_options(&self) -> AudioSourceOptions {
        self.sys_handle.audio_options().into()
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn num_channels(&self) -> u32 {
        self.num_channels
    }

    pub fn clear_buffer(&self) {
        self.sys_handle.clear_buffer();
    }

    pub async fn capture_frame(&self, frame: &AudioFrame<'_>) -> Result<(), RtcError> {
        if self.sample_rate != frame.sample_rate || self.num_channels != frame.num_channels {
            return Err(RtcError {
                error_type: RtcErrorType::InvalidState,
                message: "sample_rate and num_channels don't match".to_owned(),
            });
        }

        // Fast path: no buffering
        if self.queue_size_samples == 0 {
            // frame size must be 10ms for fast path
            let expected_frames_per_ch = (self.sample_rate / 100) as usize;
            if frame.data.len() % (self.num_channels as usize) != 0 {
                return Err(RtcError {
                    error_type: RtcErrorType::InvalidState,
                    message: "frame.data length not divisible by channel count".to_owned(),
                });
            }
            let nb_frames = frame.data.len() / (self.num_channels as usize);
            if nb_frames != expected_frames_per_ch {
                return Err(RtcError {
                    error_type: RtcErrorType::InvalidState,
                    message: format!(
                        "direct capture requires 10ms frames: got {} frames, expected {}",
                        nb_frames, expected_frames_per_ch
                    ),
                });
            }

            // Define a no-op callback for fast path (queue_size_ms=0)
            // This is safer than passing null, which can cause UB in release mode optimizations
            extern "C" fn noop_complete_callback(_ctx: *const sys_at::SourceContext) {
                // No-op: fast path completes synchronously, no callback needed
            }

            unsafe {
                let data: &[i16] = frame.data.as_ref();
                // Use a valid no-op callback instead of null for safety
                // In release mode, transmuting null pointers can cause UB
                let noop_callback = sys_at::CompleteCallback(noop_complete_callback);
                let ok = self.sys_handle.capture_frame(
                    data,
                    self.sample_rate,
                    self.num_channels,
                    nb_frames,
                    std::ptr::null(), // Context is still null - callback won't use it
                    noop_callback,
                );
                if !ok {
                    return Err(RtcError {
                        error_type: RtcErrorType::InvalidState,
                        message: "failed to capture frame without buffering".to_owned(),
                    });
                }
            }
            return Ok(());
        }

        // Buffered path.
        extern "C" fn lk_audio_source_complete(userdata: *const sys_at::SourceContext) {
            let tx = unsafe { Box::from_raw(userdata as *mut oneshot::Sender<()>) };
            let _ = tx.send(());
        }

        // iterate over chunks of self._queue_size_samples
        for chunk in frame.data.chunks(self.queue_size_samples as usize) {
            let nb_frames = chunk.len() / self.num_channels as usize;
            let (tx, rx) = oneshot::channel::<()>();
            let ctx = Box::new(tx);
            let ctx_ptr = Box::into_raw(ctx) as *const sys_at::SourceContext;

            unsafe {
                // In the fast path, C++ never store / invoke on_complete / ctx.
                if !self.sys_handle.capture_frame(
                    chunk,
                    self.sample_rate,
                    self.num_channels,
                    nb_frames,
                    ctx_ptr,
                    sys_at::CompleteCallback(lk_audio_source_complete),
                ) {
                    return Err(RtcError {
                        error_type: RtcErrorType::InvalidState,
                        message: "failed to capture frame".to_owned(),
                    });
                }
            }

            let _ = rx.await;
        }

        Ok(())
    }
}

impl From<sys_at::ffi::AudioSourceOptions> for AudioSourceOptions {
    fn from(options: sys_at::ffi::AudioSourceOptions) -> Self {
        Self {
            echo_cancellation: options.echo_cancellation,
            noise_suppression: options.noise_suppression,
            auto_gain_control: options.auto_gain_control,
        }
    }
}

impl From<AudioSourceOptions> for sys_at::ffi::AudioSourceOptions {
    fn from(options: AudioSourceOptions) -> Self {
        Self {
            echo_cancellation: options.echo_cancellation,
            noise_suppression: options.noise_suppression,
            auto_gain_control: options.auto_gain_control,
        }
    }
}
