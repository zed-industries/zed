/*
 * Copyright 2025 LiveKit, Inc.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include "api/peer_connection_interface.h"
#include "api/scoped_refptr.h"
#include "api/task_queue/task_queue_factory.h"
#include "livekit/audio_device.h"
#include "media_stream.h"
#include "rtp_parameters.h"
#include "rust/cxx.h"
#include "webrtc.h"

namespace livekit_ffi {
class PeerConnectionFactory;
class PeerConnectionObserverWrapper;
}  // namespace livekit_ffi
#include "webrtc-sys/src/peer_connection_factory.rs.h"

namespace livekit_ffi {

class PeerConnection;
struct RtcConfiguration;

webrtc::PeerConnectionInterface::RTCConfiguration to_native_rtc_configuration(
    RtcConfiguration config);

class PeerConnectionFactory {
 public:
  explicit PeerConnectionFactory(std::shared_ptr<RtcRuntime> rtc_runtime);
  ~PeerConnectionFactory();

  std::shared_ptr<PeerConnection> create_peer_connection(
      RtcConfiguration config,
      rust::Box<PeerConnectionObserverWrapper> observer) const;

  std::shared_ptr<VideoTrack> create_video_track(
      rust::String label,
      std::shared_ptr<VideoTrackSource> source) const;

  std::shared_ptr<AudioTrack> create_audio_track(
      rust::String label,
      std::shared_ptr<AudioTrackSource> source) const;

  RtpCapabilities rtp_sender_capabilities(MediaType type) const;

  RtpCapabilities rtp_receiver_capabilities(MediaType type) const;

  std::shared_ptr<RtcRuntime> rtc_runtime() const { return rtc_runtime_; }

 private:
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  webrtc::scoped_refptr<AudioDevice> audio_device_;
  webrtc::scoped_refptr<webrtc::PeerConnectionFactoryInterface> peer_factory_;
  webrtc::TaskQueueFactory* task_queue_factory_;
};

std::shared_ptr<PeerConnectionFactory> create_peer_connection_factory();
}  // namespace livekit_ffi
