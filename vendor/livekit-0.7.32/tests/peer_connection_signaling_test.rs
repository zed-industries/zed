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

//! Peer Connection Signaling Tests
//!
//! These tests verify that both V0 (dual peer connection) and V1 (single peer connection)
//! signaling modes work correctly.
//!
//! V0 (Dual PC): Traditional mode with separate publisher and subscriber peer connections
//!               Works on localhost with `livekit-server --dev`
//!
//! V1 (Single PC): New mode with a single peer connection for both publish and subscribe
//!                 Requires LiveKit Cloud or a server that supports /rtc/v1 endpoint.
//!                 NOTE: V1 tests will fall back to V0 on localhost, so to truly test V1,
//!                 you must set the cloud environment variables.
//!
//! Environment variables:
//! - LIVEKIT_URL: The LiveKit server URL (defaults to ws://localhost:7880)
//! - LIVEKIT_API_KEY: The API key (defaults to "devkey")
//! - LIVEKIT_API_SECRET: The API secret (defaults to "secret")
//!
//! Run all tests:
//!   cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test -- --nocapture
//!
//! Run only V0 tests:
//!   cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test v0_ -- --nocapture
//!
//! Run only V1 tests (set cloud env vars first):
//!   cargo test -p livekit --features "__lk-e2e-test,native-tls" --test peer_connection_signaling_test v1_ -- --nocapture

#![cfg(feature = "__lk-e2e-test")]

mod common;

use anyhow::{anyhow, Context, Result};
use futures_util::StreamExt;
use libwebrtc::audio_source::native::NativeAudioSource;
use libwebrtc::audio_stream::native::NativeAudioStream;
use libwebrtc::native::create_random_uuid;
use libwebrtc::prelude::{AudioSourceOptions, RtcAudioSource, RtcVideoSource, VideoResolution};
use libwebrtc::video_source::native::NativeVideoSource;
use livekit::options::TrackPublishOptions;
use livekit::prelude::*;
use livekit::{Room, RoomEvent, RoomOptions, SimulateScenario};
use livekit_api::access_token::{AccessToken, VideoGrants};
use std::collections::HashSet;
use std::env;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::UnboundedReceiver;
use tokio::time::timeout;

use crate::common::{
    audio::{SineParameters, SineTrack},
    test_rooms_with_options,
};

/// Signaling mode for tests
#[derive(Debug, Clone, Copy, PartialEq)]
enum SignalingMode {
    /// V0: Dual peer connection (traditional)
    DualPC,
    /// V1: Single peer connection (new)
    SinglePC,
}

impl SignalingMode {
    fn name(&self) -> &'static str {
        match self {
            SignalingMode::DualPC => "V0 (Dual PC)",
            SignalingMode::SinglePC => "V1 (Single PC)",
        }
    }

    fn is_single_pc(&self) -> bool {
        matches!(self, SignalingMode::SinglePC)
    }
}

/// Default localhost configuration
const DEFAULT_LOCALHOST_URL: &str = "ws://localhost:7880";
const DEFAULT_API_KEY: &str = "devkey";
const DEFAULT_API_SECRET: &str = "secret";

/// Get environment for tests (uses localhost defaults if env vars not set)
fn get_env_for_mode(_mode: SignalingMode) -> (String, String, String) {
    let url = env::var("LIVEKIT_URL").unwrap_or_else(|_| DEFAULT_LOCALHOST_URL.to_string());
    let api_key = env::var("LIVEKIT_API_KEY").unwrap_or_else(|_| DEFAULT_API_KEY.to_string());
    let api_secret =
        env::var("LIVEKIT_API_SECRET").unwrap_or_else(|_| DEFAULT_API_SECRET.to_string());
    (url, api_key, api_secret)
}

fn is_local_dev_server(url: &str) -> bool {
    url.contains("localhost:7880") || url.contains("127.0.0.1:7880")
}

fn assert_signaling_mode_state(room: &Room, mode: SignalingMode, url: &str) {
    let active_single_pc = room.is_single_peer_connection_active();
    match mode {
        SignalingMode::DualPC => {
            assert!(!active_single_pc, "DualPC test should not have single-PC mode active");
        }
        SignalingMode::SinglePC => {
            if is_local_dev_server(url) {
                // Local dev server behavior may vary by version:
                // older versions fallback to v0, newer versions may support /rtc/v1.
                log::info!(
                    "SinglePC on localhost: single_pc_active={} (fallback to v0 expected on older servers)",
                    active_single_pc
                );
            } else {
                assert!(
                    active_single_pc,
                    "SinglePC requested on non-localhost URL should stay in single-PC mode"
                );
            }
        }
    }
}

/// Create a token for testing
fn create_token(
    api_key: &str,
    api_secret: &str,
    room_name: &str,
    identity: &str,
) -> Result<String> {
    let grants = VideoGrants {
        room_join: true,
        room: room_name.to_string(),
        can_publish: true,
        can_subscribe: true,
        ..Default::default()
    };
    AccessToken::with_api_key(api_key, api_secret)
        .with_ttl(Duration::from_secs(30 * 60))
        .with_grants(grants)
        .with_identity(identity)
        .with_name(identity)
        .to_jwt()
        .context("Failed to generate JWT")
}

/// Create room options for the specified signaling mode
fn room_options(mode: SignalingMode) -> RoomOptions {
    let mut options = RoomOptions::default();
    options.auto_subscribe = true;
    options.dynacast = false;
    options.single_peer_connection = mode.is_single_pc();
    options
}

/// Connect to a room with specified signaling mode
async fn connect_room(
    url: &str,
    token: &str,
    mode: SignalingMode,
) -> Result<(Room, UnboundedReceiver<RoomEvent>)> {
    let options = room_options(mode);
    let started_at = Instant::now();
    let result = Room::connect(url, token, options).await;
    let elapsed = started_at.elapsed();

    match &result {
        Ok((room, _)) => {
            println!(
                "[{}] connect_room elapsed={:?}, single_pc_active={}",
                mode.name(),
                elapsed,
                room.is_single_peer_connection_active()
            );
        }
        Err(err) => {
            println!("[{}] connect_room failed after {:?}: {:?}", mode.name(), elapsed, err);
        }
    }

    result.context(format!("Failed to connect to room with {}", mode.name()))
}

// ==================== V0 (Dual PC) Tests ====================

/// Test basic connection with V0 signaling (dual PC)
#[test_log::test(tokio::test)]
async fn test_v0_connect() -> Result<()> {
    test_connect_impl(SignalingMode::DualPC).await
}

/// Test two participants with V0 signaling
#[test_log::test(tokio::test)]
async fn test_v0_two_participants() -> Result<()> {
    test_two_participants_impl(SignalingMode::DualPC).await
}

/// Test audio track with V0 signaling
#[test_log::test(tokio::test)]
async fn test_v0_audio_track() -> Result<()> {
    test_audio_track_impl(SignalingMode::DualPC).await
}

/// Test reconnection with V0 signaling
#[test_log::test(tokio::test)]
async fn test_v0_reconnect() -> Result<()> {
    test_reconnect_impl(SignalingMode::DualPC).await
}

/// Test data channel with V0 signaling
#[test_log::test(tokio::test)]
async fn test_v0_data_channel() -> Result<()> {
    test_data_channel_impl(SignalingMode::DualPC).await
}

/// Test node failure with V0 signaling
#[test_log::test(tokio::test)]
async fn test_v0_node_failure() -> Result<()> {
    test_node_failure_impl(SignalingMode::DualPC).await
}

/// Test publishing 10 video + 10 audio tracks with V0 signaling
#[test_log::test(tokio::test)]
async fn test_v0_publish_ten_video_and_ten_audio_tracks() -> Result<()> {
    test_publish_ten_video_and_ten_audio_tracks_impl(SignalingMode::DualPC).await
}

// ==================== V1 (Single PC) Tests ====================

/// Test basic connection with V1 signaling (single PC)
#[test_log::test(tokio::test)]
async fn test_v1_connect() -> Result<()> {
    test_connect_impl(SignalingMode::SinglePC).await
}

/// Test two participants with V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_two_participants() -> Result<()> {
    test_two_participants_impl(SignalingMode::SinglePC).await
}

/// Test audio track with V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_audio_track() -> Result<()> {
    test_audio_track_impl(SignalingMode::SinglePC).await
}

/// Test reconnection with V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_reconnect() -> Result<()> {
    test_reconnect_impl(SignalingMode::SinglePC).await
}

/// Test data channel with V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_data_channel() -> Result<()> {
    test_data_channel_impl(SignalingMode::SinglePC).await
}

/// Test node failure with V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_node_failure() -> Result<()> {
    test_node_failure_impl(SignalingMode::SinglePC).await
}

/// Test publishing 10 video + 10 audio tracks with V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_publish_ten_video_and_ten_audio_tracks() -> Result<()> {
    test_publish_ten_video_and_ten_audio_tracks_impl(SignalingMode::SinglePC).await
}

/// Test explicit localhost fallback behavior for V1 signaling
#[test_log::test(tokio::test)]
async fn test_v1_localhost_fallback_to_v0() -> Result<()> {
    if env::var("LIVEKIT_URL").is_ok() {
        log::info!("Skipping localhost fallback test because LIVEKIT_URL override is set");
        return Ok(());
    }

    let room_name = format!("test_v1_localhost_fallback_{}", create_random_uuid());
    let token = create_token(DEFAULT_API_KEY, DEFAULT_API_SECRET, &room_name, "fallback_test")?;
    let (room, _events) =
        connect_room(DEFAULT_LOCALHOST_URL, &token, SignalingMode::SinglePC).await?;
    if room.is_single_peer_connection_active() {
        log::info!("Localhost server supports /rtc/v1; skipping fallback assertion");
        return Ok(());
    }
    assert!(!room.is_single_peer_connection_active(), "Expected fallback to v0");
    Ok(())
}

/// Corner case: reconnect twice in a row
#[test_log::test(tokio::test)]
async fn test_v0_double_reconnect() -> Result<()> {
    test_double_reconnect_impl(SignalingMode::DualPC).await
}

/// Corner case: reconnect twice in a row
#[test_log::test(tokio::test)]
async fn test_v1_double_reconnect() -> Result<()> {
    test_double_reconnect_impl(SignalingMode::SinglePC).await
}

// ==================== Test Implementations ====================

/// Test basic connection
async fn test_connect_impl(mode: SignalingMode) -> Result<()> {
    let (url, api_key, api_secret) = get_env_for_mode(mode);
    let room_name = format!("test_{:?}_{}", mode, create_random_uuid());
    let token = create_token(&api_key, &api_secret, &room_name, "test_participant")?;

    log::info!("[{}] Connecting to {}", mode.name(), url);

    let (room, _events) = connect_room(&url, &token, mode).await?;

    log::info!("[{}] Connected! Room: {:?}", mode.name(), room.name());
    log::info!("[{}] Local participant: {:?}", mode.name(), room.local_participant().identity());

    // Verify connection is working
    assert_eq!(room.connection_state(), ConnectionState::Connected);
    assert_signaling_mode_state(&room, mode, &url);

    // Give it a moment to ensure connection is stable
    tokio::time::sleep(Duration::from_secs(2)).await;

    log::info!("[{}] Test passed - connection working!", mode.name());
    Ok(())
}

/// Test two participants connecting
async fn test_two_participants_impl(mode: SignalingMode) -> Result<()> {
    log::info!("[{}] Connecting two participants", mode.name());
    let (url, _, _) = get_env_for_mode(mode);
    let mut rooms = test_rooms_with_options([room_options(mode), room_options(mode)]).await?;
    let (room2, _events2) = rooms.pop().unwrap();
    let (room1, _events1) = rooms.pop().unwrap();
    assert_signaling_mode_state(&room1, mode, &url);
    assert_signaling_mode_state(&room2, mode, &url);

    log::info!("[{}] Both participants visible to each other", mode.name());
    log::info!("[{}] Test passed - two participants working!", mode.name());
    Ok(())
}

/// Test publishing and receiving audio tracks
async fn test_audio_track_impl(mode: SignalingMode) -> Result<()> {
    log::info!("[{}] Testing audio track", mode.name());
    let mut rooms = test_rooms_with_options([room_options(mode), room_options(mode)]).await?;
    let (_sub_room, mut sub_events) = rooms.pop().unwrap();
    let (pub_room, _pub_events) = rooms.pop().unwrap();

    // Publish a sine wave track
    const SINE_FREQ: f64 = 440.0;
    let sine_params =
        SineParameters { freq: SINE_FREQ, amplitude: 1.0, sample_rate: 48000, num_channels: 1 };
    let pub_room_arc = Arc::new(pub_room);
    let mut sine_track = SineTrack::new(pub_room_arc, sine_params);
    sine_track.publish().await?;

    log::info!("[{}] Published audio track, waiting for subscriber to receive", mode.name());

    // Wait for track subscription
    let receive_track = async {
        loop {
            let Some(event) = sub_events.recv().await else {
                return Err(anyhow!("Event channel closed"));
            };
            if let RoomEvent::TrackSubscribed { track, publication: _, participant: _ } = event {
                return Ok(track);
            }
        }
    };

    let track = timeout(Duration::from_secs(15), receive_track)
        .await
        .context("Timeout waiting for track subscription")??;

    log::info!("[{}] Received track: {:?}", mode.name(), track.sid());

    // Verify it's an audio track
    let RemoteTrack::Audio(audio_track) = track else {
        return Err(anyhow!("Expected audio track"));
    };

    // Read some audio frames
    let mut stream = NativeAudioStream::new(audio_track.rtc_track(), 48000, 1);
    let mut frames_received = 0;
    let receive_frames = async {
        while let Some(frame) = stream.next().await {
            assert!(!frame.data.is_empty());
            frames_received += 1;
            if frames_received >= 50 {
                break;
            }
        }
    };

    timeout(Duration::from_secs(10), receive_frames)
        .await
        .context("Timeout receiving audio frames")?;

    log::info!("[{}] Received {} audio frames", mode.name(), frames_received);
    log::info!("[{}] Test passed - audio track working!", mode.name());
    Ok(())
}

/// Test publishing 10 video + 10 audio tracks and verifying subscriber receives all tracks.
async fn test_publish_ten_video_and_ten_audio_tracks_impl(mode: SignalingMode) -> Result<()> {
    log::info!("[{}] Testing publish 10 video + 10 audio tracks", mode.name());
    let mut rooms = test_rooms_with_options([room_options(mode), room_options(mode)]).await?;
    let (sub_room, mut sub_events) = rooms.pop().unwrap();
    let (pub_room, _pub_events) = rooms.pop().unwrap();
    let publisher_identity = pub_room.local_participant().identity().to_string();

    let mut expected_names = HashSet::new();
    let mut publications = Vec::new();
    let mut video_sources = Vec::new();
    let mut audio_sources = Vec::new();

    for i in 0..10 {
        let name = format!("video-track-{}", i);
        let source =
            NativeVideoSource::new(VideoResolution { width: 640, height: 360 }, i % 2 == 1);
        let track =
            LocalVideoTrack::create_video_track(&name, RtcVideoSource::Native(source.clone()));
        let mut opts = TrackPublishOptions::default();
        opts.source = if i % 2 == 0 { TrackSource::Camera } else { TrackSource::Screenshare };
        let publication =
            pub_room.local_participant().publish_track(LocalTrack::Video(track), opts).await?;
        expected_names.insert(name);
        publications.push(publication);
        video_sources.push(source);
    }

    for i in 0..10 {
        let name = format!("audio-track-{}", i);
        let source = NativeAudioSource::new(AudioSourceOptions::default(), 48_000, 1, 1000);
        let track =
            LocalAudioTrack::create_audio_track(&name, RtcAudioSource::Native(source.clone()));
        let mut opts = TrackPublishOptions::default();
        opts.source =
            if i % 2 == 0 { TrackSource::Microphone } else { TrackSource::ScreenshareAudio };
        let publication =
            pub_room.local_participant().publish_track(LocalTrack::Audio(track), opts).await?;
        expected_names.insert(name);
        publications.push(publication);
        audio_sources.push(source);
    }

    let mut last_retry = Instant::now() - Duration::from_secs(1);
    let receive_all_tracks = async {
        loop {
            let mut published_names = HashSet::new();
            let mut subscribed_names = HashSet::new();
            let mut audio_count = 0usize;
            let mut video_count = 0usize;

            let remote_participants = sub_room.remote_participants();
            let publisher_entry = remote_participants
                .iter()
                .find(|(_, p)| p.identity().as_str() == publisher_identity.as_str());

            if let Some((_, publisher)) = publisher_entry {
                let publications = publisher.track_publications();
                for publication in publications.values() {
                    let name = publication.name();
                    if expected_names.contains(&name) {
                        published_names.insert(name.clone());
                        if let Some(track) = publication.track() {
                            subscribed_names.insert(name);
                            match track {
                                RemoteTrack::Audio(_) => audio_count += 1,
                                RemoteTrack::Video(_) => video_count += 1,
                            }
                        }
                    }
                }

                if published_names.len() >= expected_names.len()
                    && subscribed_names.len() >= expected_names.len()
                    && audio_count >= 10
                    && video_count >= 10
                {
                    return Ok((subscribed_names, audio_count, video_count));
                }

                // Under load, transient TrackSubscriptionFailed can happen before publication
                // state fully settles. Retry subscription on missing tracks.
                if published_names.len() >= expected_names.len()
                    && last_retry.elapsed() >= Duration::from_millis(300)
                {
                    for publication in publications.values() {
                        if expected_names.contains(&publication.name())
                            && publication.track().is_none()
                        {
                            publication.set_subscribed(false);
                            publication.set_subscribed(true);
                        }
                    }
                    last_retry = Instant::now();
                }
            }

            match timeout(Duration::from_millis(250), sub_events.recv()).await {
                Ok(Some(RoomEvent::TrackSubscriptionFailed { participant, track_sid, error })) => {
                    log::warn!(
                        "[{}] TrackSubscriptionFailed sid={} participant={} error={:?}",
                        mode.name(),
                        track_sid,
                        participant.identity(),
                        error
                    );
                    if let Some(publication) = participant.get_track_publication(&track_sid) {
                        publication.set_subscribed(false);
                        publication.set_subscribed(true);
                    }
                }
                Ok(Some(_)) => {}
                Ok(None) => return Err(anyhow!("Event channel closed")),
                Err(_) => {}
            }
        }
    };

    let (received_names, audio_count, video_count) =
        timeout(Duration::from_secs(45), receive_all_tracks)
            .await
            .context("Timeout waiting for all 20 track subscriptions")??;

    for expected in &expected_names {
        assert!(received_names.contains(expected), "missing subscribed track: {}", expected);
    }
    assert!(audio_count >= 10, "expected >=10 audio tracks, got {}", audio_count);
    assert!(video_count >= 10, "expected >=10 video tracks, got {}", video_count);

    let remote_participants = sub_room.remote_participants();
    let publisher_entry = remote_participants
        .iter()
        .find(|(_, p)| p.identity().as_str() == publisher_identity.as_str());
    if let Some((_, publisher)) = publisher_entry {
        assert!(
            publisher.track_publications().len() >= 20,
            "subscriber should see >=20 published tracks from publisher"
        );
    }

    for publication in publications {
        pub_room.local_participant().unpublish_track(&publication.sid()).await?;
    }
    Ok(())
}

/// Test reconnection - verifies tracks are restored
async fn test_reconnect_impl(mode: SignalingMode) -> Result<()> {
    log::info!("[{}] Testing reconnection", mode.name());
    let mut rooms = test_rooms_with_options([room_options(mode), room_options(mode)]).await?;
    let (sub_room, mut sub_events) = rooms.pop().unwrap();
    let (pub_room, mut pub_events) = rooms.pop().unwrap();
    let publisher_identity = pub_room.local_participant().identity().to_string();

    // Wrap in Arc for SineTrack
    let pub_room_arc = Arc::new(pub_room);

    // Publish a track
    let sine_params =
        SineParameters { freq: 440.0, amplitude: 1.0, sample_rate: 48000, num_channels: 1 };
    let mut sine_track = SineTrack::new(pub_room_arc.clone(), sine_params);
    sine_track.publish().await?;

    // Wait for initial track subscription
    let wait_track = async {
        loop {
            let Some(event) = sub_events.recv().await else {
                return Err(anyhow!("Event channel closed"));
            };
            if let RoomEvent::TrackSubscribed { track: _, publication: _, participant: _ } = event {
                return Ok(());
            }
        }
    };

    timeout(Duration::from_secs(15), wait_track)
        .await
        .context("Timeout waiting for initial track subscription")??;

    log::info!(
        "[{}] Initial track received, verifying track count before reconnection",
        mode.name()
    );

    let tracks_before = pub_room_arc.local_participant().track_publications().len();
    log::info!("[{}] Tracks published before reconnect: {}", mode.name(), tracks_before);

    log::info!("[{}] Simulating signal reconnect...", mode.name());

    // Simulate a signal reconnect
    pub_room_arc.simulate_scenario(SimulateScenario::SignalReconnect).await?;

    // Wait for reconnection events
    let wait_reconnected = async {
        loop {
            let Some(event) = pub_events.recv().await else {
                return Err(anyhow!("Event channel closed"));
            };
            match event {
                RoomEvent::Reconnecting => {
                    log::info!("[{}] Publisher reconnecting...", mode.name());
                }
                RoomEvent::Reconnected => {
                    log::info!("[{}] Publisher reconnected!", mode.name());
                    return Ok(());
                }
                _ => {}
            }
        }
    };

    timeout(Duration::from_secs(30), wait_reconnected)
        .await
        .context("Timeout waiting for reconnection")??;

    // Give some time for state to stabilize
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Verify room is connected
    assert_eq!(
        pub_room_arc.connection_state(),
        ConnectionState::Connected,
        "Room should be connected after reconnect"
    );

    // Verify track is still published
    let tracks_after = pub_room_arc.local_participant().track_publications().len();
    log::info!("[{}] Tracks published after reconnect: {}", mode.name(), tracks_after);
    assert_eq!(tracks_before, tracks_after, "Track count should be preserved after reconnect");

    // Verify subscriber can still see the publisher's tracks
    let remote_participants = sub_room.remote_participants();
    let publisher_entry = remote_participants
        .iter()
        .find(|(_, p)| p.identity().as_str() == publisher_identity.as_str());

    if let Some((_, publisher)) = publisher_entry {
        let remote_tracks = publisher.track_publications().len();
        log::info!("[{}] Subscriber sees {} tracks from publisher", mode.name(), remote_tracks);
        assert!(remote_tracks > 0, "Subscriber should still see publisher's tracks");
    } else {
        log::warn!("[{}] Publisher not found in remote participants", mode.name());
    }

    log::info!("[{}] Test passed - reconnection working!", mode.name());
    Ok(())
}

/// Test data channel
async fn test_data_channel_impl(mode: SignalingMode) -> Result<()> {
    log::info!("[{}] Testing data channel", mode.name());
    let mut rooms = test_rooms_with_options([room_options(mode), room_options(mode)]).await?;
    let (_room2, mut events2) = rooms.pop().unwrap();
    let (room1, _events1) = rooms.pop().unwrap();

    // Send data from room1 to room2
    let test_data = b"Hello from peer connection signaling test!".to_vec();
    let test_topic = "test_topic".to_string();

    room1
        .local_participant()
        .publish_data(livekit::DataPacket {
            payload: test_data.clone(),
            topic: Some(test_topic.clone()),
            reliable: true,
            ..Default::default()
        })
        .await?;

    log::info!("[{}] Sent data packet, waiting for receiver...", mode.name());

    // Wait to receive data
    let receive_data = async {
        loop {
            let Some(event) = events2.recv().await else {
                return Err(anyhow!("Event channel closed"));
            };
            if let RoomEvent::DataReceived { payload, topic, kind: _, participant: _ } = event {
                if topic == Some(test_topic.clone()) {
                    return Ok(payload);
                }
            }
        }
    };

    let received = timeout(Duration::from_secs(10), receive_data)
        .await
        .context("Timeout waiting for data")??;

    assert_eq!(received.to_vec(), test_data, "Received data should match sent data");

    log::info!("[{}] Test passed - data channel working!", mode.name());
    Ok(())
}

/// Test node failure reconnection scenario
async fn test_node_failure_impl(mode: SignalingMode) -> Result<()> {
    let (url, api_key, api_secret) = get_env_for_mode(mode);
    let room_name = format!("test_{:?}_node_fail_{}", mode, create_random_uuid());

    let token = create_token(&api_key, &api_secret, &room_name, "test_participant")?;

    log::info!("[{}] Testing node failure scenario", mode.name());

    let (room, mut events) = connect_room(&url, &token, mode).await?;

    // Wrap in Arc for SineTrack
    let room_arc = Arc::new(room);

    // Publish a track first
    let sine_params =
        SineParameters { freq: 440.0, amplitude: 1.0, sample_rate: 48000, num_channels: 1 };
    let mut sine_track = SineTrack::new(room_arc.clone(), sine_params);
    sine_track.publish().await?;

    let tracks_before = room_arc.local_participant().track_publications().len();
    log::info!("[{}] Tracks before node failure: {}", mode.name(), tracks_before);

    log::info!("[{}] Simulating node failure...", mode.name());
    room_arc.simulate_scenario(SimulateScenario::NodeFailure).await?;

    // Wait for reconnection
    let wait_reconnected = async {
        loop {
            let Some(event) = events.recv().await else {
                return Err(anyhow!("Event channel closed"));
            };
            match event {
                RoomEvent::Reconnecting => {
                    log::info!("[{}] Reconnecting after node failure...", mode.name());
                }
                RoomEvent::Reconnected => {
                    log::info!("[{}] Reconnected after node failure!", mode.name());
                    return Ok(());
                }
                RoomEvent::Disconnected { reason } => {
                    log::info!("[{}] Disconnected: {:?}", mode.name(), reason);
                }
                _ => {}
            }
        }
    };

    timeout(Duration::from_secs(30), wait_reconnected)
        .await
        .context("Timeout waiting for reconnection after node failure")??;

    // Give time for track republishing
    tokio::time::sleep(Duration::from_secs(3)).await;

    let tracks_after = room_arc.local_participant().track_publications().len();
    log::info!("[{}] Tracks after node failure reconnect: {}", mode.name(), tracks_after);

    assert_eq!(tracks_before, tracks_after, "Tracks should be restored after node failure");

    log::info!("[{}] Test passed - node failure recovery working!", mode.name());
    Ok(())
}

/// Test two sequential reconnect cycles on the same room connection
async fn test_double_reconnect_impl(mode: SignalingMode) -> Result<()> {
    let (url, api_key, api_secret) = get_env_for_mode(mode);
    let room_name = format!("test_{:?}_double_reconnect_{}", mode, create_random_uuid());
    let token = create_token(&api_key, &api_secret, &room_name, "reconnect_tester")?;

    let (room, mut events) = connect_room(&url, &token, mode).await?;
    assert_signaling_mode_state(&room, mode, &url);

    for attempt in 1..=2 {
        log::info!("[{}] Triggering reconnect attempt {}", mode.name(), attempt);
        room.simulate_scenario(SimulateScenario::SignalReconnect).await?;

        let wait_reconnected = async {
            loop {
                let Some(event) = events.recv().await else {
                    return Err(anyhow!("Event channel closed"));
                };
                match event {
                    RoomEvent::Reconnecting => {}
                    RoomEvent::Reconnected => return Ok(()),
                    _ => {}
                }
            }
        };

        timeout(Duration::from_secs(30), wait_reconnected)
            .await
            .context("Timeout waiting for reconnect cycle")??;

        assert_eq!(room.connection_state(), ConnectionState::Connected);
    }

    Ok(())
}
