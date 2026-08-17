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

#include "rust/cxx.h"

namespace livekit_ffi {
class MediaStream;
class AudioTrack;
class VideoTrack;
class Candidate;
class RtpSender;
class RtpReceiver;
class RtpTransceiver;
}  // namespace livekit_ffi
#include "webrtc-sys/src/helper.rs.h"

namespace livekit_ffi {

// Impl not needed
static rust::Vec<MediaStreamPtr> _vec_media_stream_ptr() {
  throw;
}
static rust::Vec<CandidatePtr> _vec_candidate_ptr() {
  throw;
}
static rust::Vec<AudioTrackPtr> _vec_audio_track_ptr() {
  throw;
}
static rust::Vec<VideoTrackPtr> _vec_video_track_ptr() {
  throw;
}
static rust::Vec<RtpSenderPtr> _vec_rtp_sender_ptr() {
  throw;
}
static rust::Vec<RtpReceiverPtr> _vec_rtp_receiver_ptr() {
  throw;
}
static rust::Vec<RtpTransceiverPtr> _vec_rtp_transceiver_ptr() {
  throw;
}

}  // namespace livekit_ffi
