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
class MediaStreamTrack;
}  // namespace livekit_ffi
#include "webrtc-sys/src/media_stream_track.rs.h"

namespace livekit_ffi {

class MediaStreamTrack {
 protected:
  MediaStreamTrack(std::shared_ptr<RtcRuntime>,
                   webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> track);

 public:
  rust::String kind() const;
  rust::String id() const;

  bool enabled() const;
  bool set_enabled(bool enable) const;

  TrackState state() const;

  webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> rtc_track() const {
    return track_;
  }

 protected:
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  webrtc::scoped_refptr<webrtc::MediaStreamTrackInterface> track_;
};

static std::shared_ptr<MediaStreamTrack> _shared_media_stream_track() {
  return nullptr;  // Ignore
}

}  // namespace livekit_ffi
