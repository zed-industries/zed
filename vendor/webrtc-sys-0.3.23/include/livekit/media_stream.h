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
#include "livekit/helper.h"
#include "livekit/webrtc.h"
#include "rust/cxx.h"

namespace livekit_ffi {
class MediaStream;
}  // namespace livekit_ffi
#include "webrtc-sys/src/media_stream.rs.h"

namespace livekit_ffi {

class MediaStream {
 public:
  MediaStream(std::shared_ptr<RtcRuntime> rtc_runtime,
              webrtc::scoped_refptr<webrtc::MediaStreamInterface> stream);

  rust::String id() const;
  rust::Vec<VideoTrackPtr> get_video_tracks() const;
  rust::Vec<AudioTrackPtr> get_audio_tracks() const;

  std::shared_ptr<AudioTrack> find_audio_track(rust::String track_id) const;
  std::shared_ptr<VideoTrack> find_video_track(rust::String track_id) const;

  bool add_track(std::shared_ptr<MediaStreamTrack> track) const;
  bool remove_track(std::shared_ptr<MediaStreamTrack> track) const;

 private:
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  webrtc::scoped_refptr<webrtc::MediaStreamInterface> media_stream_;
};

static std::shared_ptr<MediaStream> _shared_media_stream() {
  return nullptr;  // Ignore
}

}  // namespace livekit_ffi
