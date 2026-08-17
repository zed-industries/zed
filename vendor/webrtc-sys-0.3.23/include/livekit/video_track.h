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
#include "api/video/video_frame.h"
#include "livekit/helper.h"
#include "livekit/media_stream_track.h"
#include "livekit/video_frame.h"
#include "livekit/webrtc.h"
#include "media/base/adapted_video_track_source.h"
#include "rtc_base/synchronization/mutex.h"
#include "rtc_base/timestamp_aligner.h"
#include "rust/cxx.h"

namespace livekit_ffi {
class VideoTrack;
class NativeVideoSink;
class VideoTrackSource;
}  // namespace livekit_ffi
#include "webrtc-sys/src/video_track.rs.h"

namespace livekit_ffi {

class VideoTrack : public MediaStreamTrack {
 private:
  friend RtcRuntime;
  VideoTrack(std::shared_ptr<RtcRuntime> rtc_runtime,
             webrtc::scoped_refptr<webrtc::VideoTrackInterface> track);

 public:
  ~VideoTrack();

  void add_sink(const std::shared_ptr<NativeVideoSink>& sink) const;
  void remove_sink(const std::shared_ptr<NativeVideoSink>& sink) const;

  void set_should_receive(bool should_receive) const;
  bool should_receive() const;
  ContentHint content_hint() const;
  void set_content_hint(ContentHint hint) const;

 private:
  webrtc::VideoTrackInterface* track() const {
    return static_cast<webrtc::VideoTrackInterface*>(track_.get());
  }

  mutable webrtc::Mutex mutex_;

  // Same for AudioTrack:
  // Keep a strong reference to the added sinks, so we don't need to
  // manage the lifetime safety on the Rust side
  mutable std::vector<std::shared_ptr<NativeVideoSink>> sinks_;
};

class NativeVideoSink : public webrtc::VideoSinkInterface<webrtc::VideoFrame> {
 public:
  explicit NativeVideoSink(rust::Box<VideoSinkWrapper> observer);

  void OnFrame(const webrtc::VideoFrame& frame) override;
  void OnDiscardedFrame() override;
  void OnConstraintsChanged(
      const webrtc::VideoTrackSourceConstraints& constraints) override;

 private:
  rust::Box<VideoSinkWrapper> observer_;
};

std::shared_ptr<NativeVideoSink> new_native_video_sink(
    rust::Box<VideoSinkWrapper> observer);

class VideoTrackSource {
  class InternalSource : public webrtc::AdaptedVideoTrackSource {
   public:
    InternalSource(const VideoResolution& resolution,
                   bool is_screencast);  // (0, 0) means no resolution/optional, the
                                         // source will guess the resolution at the
                                         // first captured frame
    ~InternalSource() override;

    bool is_screencast() const override;
    std::optional<bool> needs_denoising() const override;
    SourceState state() const override;
    bool remote() const override;
    VideoResolution video_resolution() const;
    bool on_captured_frame(const webrtc::VideoFrame& frame);

   private:
    mutable webrtc::Mutex mutex_;
    webrtc::TimestampAligner timestamp_aligner_;
    VideoResolution resolution_;
    bool is_screencast_;
  };

 public:
  VideoTrackSource(const VideoResolution& resolution, bool is_screencast);

  VideoResolution video_resolution() const;

  bool on_captured_frame(const std::unique_ptr<VideoFrame>& frame)
      const;  // frames pushed from Rust (+interior mutability)

  webrtc::scoped_refptr<InternalSource> get() const;

 private:
  webrtc::scoped_refptr<InternalSource> source_;
};

std::shared_ptr<VideoTrackSource> new_video_track_source(
    const VideoResolution& resolution, bool is_screencast);

static std::shared_ptr<MediaStreamTrack> video_to_media(
    std::shared_ptr<VideoTrack> track) {
  return track;
}

static std::shared_ptr<VideoTrack> media_to_video(
    std::shared_ptr<MediaStreamTrack> track) {
  return std::static_pointer_cast<VideoTrack>(track);
}

static std::shared_ptr<VideoTrack> _shared_video_track() {
  return nullptr;  // Ignore
}

}  // namespace livekit_ffi
