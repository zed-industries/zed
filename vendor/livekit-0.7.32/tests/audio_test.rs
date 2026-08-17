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

#[cfg(feature = "__lk-e2e-test")]
use {
    anyhow::{anyhow, Ok, Result},
    common::{
        audio::{ChannelIterExt, FreqAnalyzer, SineParameters, SineTrack},
        test_rooms,
    },
    futures_util::StreamExt,
    libwebrtc::audio_stream::native::NativeAudioStream,
    livekit::prelude::*,
    std::{sync::Arc, time::Duration},
    tokio::time::timeout,
};

mod common;

struct TestParams {
    pub_rate_hz: u32,
    pub_channels: u32,
    sub_rate_hz: u32,
    sub_channels: u32,
}

#[cfg(feature = "__lk-e2e-test")]
#[test_log::test(tokio::test)]
async fn test_audio() -> Result<()> {
    let test_params = [
        TestParams { pub_rate_hz: 48_000, pub_channels: 1, sub_rate_hz: 48_000, sub_channels: 1 },
        TestParams { pub_rate_hz: 48_000, pub_channels: 2, sub_rate_hz: 48_000, sub_channels: 2 },
        TestParams { pub_rate_hz: 48_000, pub_channels: 2, sub_rate_hz: 24_000, sub_channels: 2 },
        TestParams { pub_rate_hz: 24_000, pub_channels: 2, sub_rate_hz: 24_000, sub_channels: 1 },
    ];
    for params in test_params {
        log::info!("Testing with {}", params);
        test_audio_with(params).await?;
    }
    Ok(())
}

/// Tests audio transfer between two participants.
///
/// Verifies that audio can be published and received correctly
/// between two participants by detecting the frequency of the sine wave on the subscriber end.
///
#[cfg(feature = "__lk-e2e-test")]
async fn test_audio_with(params: TestParams) -> Result<()> {
    let mut rooms = test_rooms(2).await?;
    let (pub_room, _) = rooms.pop().unwrap();
    let (_, mut sub_room_events) = rooms.pop().unwrap();

    const SINE_FREQ: f64 = 60.0;
    const SINE_AMPLITUDE: f64 = 1.0;
    const FRAMES_TO_ANALYZE: usize = 100;

    let sine_params = SineParameters {
        freq: SINE_FREQ,
        amplitude: SINE_AMPLITUDE,
        sample_rate: params.pub_rate_hz,
        num_channels: params.pub_channels,
    };
    let mut sine_track = SineTrack::new(Arc::new(pub_room), sine_params);
    sine_track.publish().await?;

    let analyze_frames = async move {
        let track: RemoteTrack = loop {
            let Some(event) = sub_room_events.recv().await else {
                Err(anyhow!("Never received track"))?
            };
            let RoomEvent::TrackSubscribed { track, publication: _, participant: _ } = event else {
                continue;
            };
            break track.into();
        };
        let RemoteTrack::Audio(track) = track else { Err(anyhow!("Expected audio track"))? };
        let mut stream = NativeAudioStream::new(
            track.rtc_track(),
            params.sub_rate_hz as i32,
            params.sub_channels as i32,
        );

        tokio::spawn(async move {
            let mut frames_analyzed = 0;
            let mut analyzers = vec![FreqAnalyzer::new(); params.sub_channels as usize];

            while let Some(frame) = stream.next().await {
                assert!(frame.data.len() > 0);
                assert_eq!(frame.num_channels, params.sub_channels);
                assert_eq!(frame.sample_rate, params.sub_rate_hz);
                assert_eq!(frame.samples_per_channel, frame.data.len() as u32 / frame.num_channels);

                for channel_idx in 0..params.sub_channels as usize {
                    analyzers[channel_idx].analyze(frame.channel_iter(channel_idx));
                }
                frames_analyzed += 1;
                if frames_analyzed >= FRAMES_TO_ANALYZE {
                    break;
                }
            }
            assert_eq!(frames_analyzed, FRAMES_TO_ANALYZE);

            for (channel_idx, detected_freq) in analyzers
                .into_iter()
                .map(|analyzer| analyzer.estimated_freq(params.sub_rate_hz))
                .enumerate()
            {
                assert!(
                    (detected_freq - SINE_FREQ).abs() < 20.0, // Expect within 20Hz
                    "Detected sine frequency not within range for channel {}: {}Hz",
                    channel_idx,
                    detected_freq
                );
            }
        })
        .await?;
        Ok(())
    };
    timeout(Duration::from_secs(15), analyze_frames).await??;
    Ok(())
}

impl std::fmt::Display for TestParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}Hz, {}ch. -> {}Hz, {}ch.",
            self.pub_rate_hz, self.pub_channels, self.sub_rate_hz, self.sub_channels
        )
    }
}
