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

#include "livekit/media_stream.h"

#include <algorithm>
#include <iostream>
#include <memory>

#include "api/media_stream_interface.h"
#include "api/video/video_frame.h"
#include "api/video/video_rotation.h"
#include "audio/remix_resample.h"
#include "common_audio/include/audio_util.h"
#include "rtc_base/logging.h"
#include "rtc_base/ref_counted_object.h"
#include "rtc_base/time_utils.h"

namespace livekit_ffi {

MediaStream::MediaStream(
    std::shared_ptr<RtcRuntime> rtc_runtime,
    webrtc::scoped_refptr<webrtc::MediaStreamInterface> stream)
    : rtc_runtime_(rtc_runtime), media_stream_(std::move(stream)) {}

rust::String MediaStream::id() const {
  return media_stream_->id();
}

rust::Vec<VideoTrackPtr> MediaStream::get_video_tracks() const {
  rust::Vec<VideoTrackPtr> rust;
  for (auto video : media_stream_->GetVideoTracks())
    rust.push_back(
        VideoTrackPtr{rtc_runtime_->get_or_create_video_track(video)});

  return rust;
}

rust::Vec<AudioTrackPtr> MediaStream::get_audio_tracks() const {
  rust::Vec<AudioTrackPtr> rust;
  for (auto audio : media_stream_->GetAudioTracks())
    rust.push_back(
        AudioTrackPtr{rtc_runtime_->get_or_create_audio_track(audio)});

  return rust;
}

std::shared_ptr<AudioTrack> MediaStream::find_audio_track(
    rust::String track_id) const {
  return rtc_runtime_->get_or_create_audio_track(
      media_stream_->FindAudioTrack(track_id.c_str()));
}

std::shared_ptr<VideoTrack> MediaStream::find_video_track(
    rust::String track_id) const {
  return rtc_runtime_->get_or_create_video_track(
      media_stream_->FindVideoTrack(track_id.c_str()));
}

bool MediaStream::add_track(std::shared_ptr<MediaStreamTrack> track) const {
  if (track->kind() == webrtc::MediaStreamTrackInterface::kVideoKind) {
    return media_stream_->AddTrack(
        webrtc::scoped_refptr<webrtc::VideoTrackInterface>(
            static_cast<webrtc::VideoTrackInterface*>(
                track->rtc_track().get())));
  } else {
    return media_stream_->AddTrack(
        webrtc::scoped_refptr<webrtc::AudioTrackInterface>(
            static_cast<webrtc::AudioTrackInterface*>(
                track->rtc_track().get())));
  }
}

bool MediaStream::remove_track(std::shared_ptr<MediaStreamTrack> track) const {
  if (track->kind() == webrtc::MediaStreamTrackInterface::kVideoKind) {
    return media_stream_->RemoveTrack(
        webrtc::scoped_refptr<webrtc::VideoTrackInterface>(
            static_cast<webrtc::VideoTrackInterface*>(
                track->rtc_track().get())));
  } else {
    return media_stream_->RemoveTrack(
        webrtc::scoped_refptr<webrtc::AudioTrackInterface>(
            static_cast<webrtc::AudioTrackInterface*>(
                track->rtc_track().get())));
  }
}

}  // namespace livekit_ffi
