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

use libwebrtc::{
    audio_source::native::NativeAudioSource,
    prelude::{AudioFrame, AudioSourceOptions, RtcAudioSource},
};
use livekit::{
    options::TrackPublishOptions,
    track::{LocalAudioTrack, LocalTrack},
    Room, RoomResult,
};
use std::sync::Arc;
use tokio::{sync::oneshot, task::JoinHandle};

/// Parameters for the sine wave generated with [`SineTrack`].
#[derive(Clone, Debug)]
pub struct SineParameters {
    pub sample_rate: u32,
    pub freq: f64,
    pub amplitude: f64,
    pub num_channels: u32,
}

/// Audio track which generates and publishes a sine wave.
///
/// This implementation was taken from the *wgpu_room* example.
///
pub struct SineTrack {
    rtc_source: NativeAudioSource,
    params: SineParameters,
    room: Arc<Room>,
    handle: Option<TrackHandle>,
}

struct TrackHandle {
    close_tx: oneshot::Sender<()>,
    track: LocalAudioTrack,
    task: JoinHandle<()>,
}

impl SineTrack {
    pub fn new(room: Arc<Room>, params: SineParameters) -> Self {
        Self {
            rtc_source: NativeAudioSource::new(
                AudioSourceOptions::default(),
                params.sample_rate,
                params.num_channels,
                1000,
            ),
            params,
            room,
            handle: None,
        }
    }

    pub async fn publish(&mut self) -> RoomResult<()> {
        let (close_tx, close_rx) = oneshot::channel();
        let track = LocalAudioTrack::create_audio_track(
            "sine-track",
            RtcAudioSource::Native(self.rtc_source.clone()),
        );
        let task =
            tokio::spawn(Self::track_task(close_rx, self.rtc_source.clone(), self.params.clone()));
        self.room
            .local_participant()
            .publish_track(LocalTrack::Audio(track.clone()), TrackPublishOptions::default())
            .await?;
        let handle = TrackHandle { close_tx, track, task };
        self.handle = Some(handle);
        Ok(())
    }

    pub async fn unpublish(&mut self) -> RoomResult<()> {
        if let Some(handle) = self.handle.take() {
            handle.close_tx.send(()).ok();
            handle.task.await.ok();
            self.room.local_participant().unpublish_track(&handle.track.sid()).await?;
        }
        Ok(())
    }

    async fn track_task(
        mut close_rx: oneshot::Receiver<()>,
        rtc_source: NativeAudioSource,
        params: SineParameters,
    ) {
        let num_channels = params.num_channels as usize;
        let samples_count = (params.sample_rate / 100) as usize * num_channels;
        let mut samples_10ms = vec![0; samples_count];
        let mut phase = 0;
        loop {
            if close_rx.try_recv().is_ok() {
                break;
            }
            for i in (0..samples_count).step_by(num_channels) {
                let val = params.amplitude
                    * f64::sin(
                        std::f64::consts::PI
                            * 2.0
                            * params.freq
                            * (phase as f64 / params.sample_rate as f64),
                    );
                phase += 1;
                for c in 0..num_channels {
                    // WebRTC uses 16-bit signed PCM
                    samples_10ms[i + c] = (val * 32768.0) as i16;
                }
            }
            let frame = AudioFrame {
                data: samples_10ms.as_slice().into(),
                sample_rate: params.sample_rate,
                num_channels: params.num_channels,
                samples_per_channel: samples_count as u32 / params.num_channels,
            };
            rtc_source.capture_frame(&frame).await.unwrap();
        }
    }
}

/// Analyzes samples to estimate the frequency of the signal using the zero crossing method.
#[derive(Clone)]
pub struct FreqAnalyzer {
    zero_crossings: usize,
    samples_analyzed: usize,
}

impl FreqAnalyzer {
    pub fn new() -> Self {
        Self { zero_crossings: 0, samples_analyzed: 0 }
    }

    pub fn analyze(&mut self, samples: impl IntoIterator<Item = i16>) {
        let mut iter = samples.into_iter();
        let mut prev = match iter.next() {
            Some(v) => v,
            None => return,
        };
        let mut count = 0;
        for curr in iter {
            if (prev >= 0 && curr < 0) || (prev < 0 && curr >= 0) {
                self.zero_crossings += 1;
            }
            prev = curr;
            count += 1;
        }
        self.samples_analyzed += count + 1;
    }

    pub fn estimated_freq(&self, sample_rate: u32) -> f64 {
        let num_cycles = self.zero_crossings as f64 / 2.0;
        let duration_seconds = self.samples_analyzed as f64 / sample_rate as f64;
        if duration_seconds == 0.0 {
            return 0.0;
        }
        num_cycles / duration_seconds
    }
}

pub trait ChannelIterExt<'a> {
    /// Returns an iterator over the samples in a specific channel.
    ///
    /// # Arguments
    /// * `channel_index` - Index of the channel to iterate over (must be less than `num_channels`).
    ///
    /// # Panics
    /// Panics if `channel_index` is greater than or equal to `num_channels`.
    ///
    fn channel_iter(&'a self, channel_index: usize) -> ChannelIter<'a>;
}

impl<'a> ChannelIterExt<'a> for AudioFrame<'a> {
    fn channel_iter(&'a self, channel_index: usize) -> ChannelIter<'a> {
        assert!(channel_index < self.num_channels as usize);
        ChannelIter { frame: self, channel_index, index: 0 }
    }
}

/// Iterator over an individual channel in an interleaved [`AudioFrame`].
pub struct ChannelIter<'a> {
    frame: &'a AudioFrame<'a>,
    channel_index: usize,
    index: usize,
}

impl<'a> Iterator for ChannelIter<'a> {
    type Item = i16;

    fn next(&mut self) -> Option<Self::Item> {
        let inner_index =
            self.index * (self.frame.num_channels as usize) + (self.channel_index as usize);
        if inner_index >= self.frame.data.len() {
            return None;
        }
        let sample = self.frame.data[inner_index];
        self.index += 1;
        Some(sample)
    }
}
