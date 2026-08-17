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

#include <memory>

#include "api/media_stream_interface.h"
#include "api/rtp_receiver_interface.h"
#include "api/rtp_sender_interface.h"
#include "livekit/helper.h"
#include "rtc_base/logging.h"
#include "rtc_base/physical_socket_server.h"
#include "rtc_base/ssl_adapter.h"
#include "rtc_base/thread.h"
#include "rust/cxx.h"

#ifdef WEBRTC_WIN
#include "rtc_base/win32_socket_init.h"
#endif

namespace livekit_ffi {
class RtcRuntime;
class LogSink;
}  // namespace livekit_ffi
#include "webrtc-sys/src/webrtc.rs.h"

namespace livekit_ffi {

class MediaStreamTrack;
class RtpReceiver;
class RtpSender;

// Using a shared_ptr in RtcRuntime allows us to keep a strong reference to it
// on resources that depend on it. (e.g: AudioTrack, VideoTrack).
class RtcRuntime : public std::enable_shared_from_this<RtcRuntime> {
 public:
  [[nodiscard]] static std::shared_ptr<RtcRuntime> create() {
    return std::shared_ptr<RtcRuntime>(new RtcRuntime());
  }

  RtcRuntime(const RtcRuntime&) = delete;
  RtcRuntime& operator=(const RtcRuntime&) = delete;
  ~RtcRuntime();

  webrtc::Thread* network_thread() const;
  webrtc::Thread* worker_thread() const;
  webrtc::Thread* signaling_thread() const;

  std::shared_ptr<MediaStreamTrack> get_or_create_media_stream_track(
      webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> track);

  std::shared_ptr<AudioTrack> get_or_create_audio_track(
      webrtc::scoped_refptr<webrtc::AudioTrackInterface> track);

  std::shared_ptr<VideoTrack> get_or_create_video_track(
      webrtc::scoped_refptr<webrtc::VideoTrackInterface> track);

 private:
  RtcRuntime();

  std::unique_ptr<webrtc::Thread> network_thread_;
  std::unique_ptr<webrtc::Thread> worker_thread_;
  std::unique_ptr<webrtc::Thread> signaling_thread_;

  // Lists used to make sure we don't create multiple wrappers for one
  // underlying webrtc object. (e.g: webrtc::VideoTrackInterface should only
  // have one livekit_ffi::VideoTrack associated with it).
  // The only reason we to do that is to allow to add states inside our
  // wrappers (e.g: the sinks_ member inside AudioTrack)
  // DataChannel and the PeerConnectionFactory don't need to do this (There's no
  // way to retrieve them after creation)
  webrtc::Mutex mutex_;
  std::vector<std::weak_ptr<MediaStreamTrack>> media_stream_tracks_;
  // We don't have additonal state in RtpReceiver and RtpSender atm..
  // std::vector<std::weak_ptr<RtpReceiver>> rtp_receivers_;
  // std::vector<std::weak_ptr<RtpSender>> rtp_senders_;

#ifdef WEBRTC_WIN
  // webrtc::WinsockInitializer winsock_;
  // webrtc::PhysicalSocketServer ss_;
  // webrtc::AutoSocketServerThread main_thread_{&ss_};
#endif
};

class LogSink : public webrtc::LogSink {
 public:
  LogSink(rust::Fn<void(rust::String message, LoggingSeverity severity)> fnc);
  ~LogSink();

  void OnLogMessage(const std::string& message,
                    webrtc::LoggingSeverity severity) override;

  void OnLogMessage(const std::string& message) override {}

 private:
  rust::Fn<void(rust::String message, LoggingSeverity severity)> fnc_;
};

std::unique_ptr<LogSink> new_log_sink(
    rust::Fn<void(rust::String, LoggingSeverity)> fnc);

rust::String create_random_uuid();

}  // namespace livekit_ffi
