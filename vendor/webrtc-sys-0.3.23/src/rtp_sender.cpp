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

#include "livekit/rtp_sender.h"
#include "livekit/jsep.h"

#include "rust/cxx.h"
#include "webrtc-sys/src/rtp_sender.rs.h"

namespace livekit_ffi {



RtpSender::RtpSender(
    std::shared_ptr<RtcRuntime> rtc_runtime,
    webrtc::scoped_refptr<webrtc::RtpSenderInterface> sender,
    webrtc::scoped_refptr<webrtc::PeerConnectionInterface> peer_connection)
    : rtc_runtime_(rtc_runtime),
      sender_(std::move(sender)),
      peer_connection_(std::move(peer_connection)) {}

bool RtpSender::set_track(std::shared_ptr<MediaStreamTrack> track) const {
  return sender_->SetTrack(track->rtc_track().get());
}

std::shared_ptr<MediaStreamTrack> RtpSender::track() const {
  return rtc_runtime_->get_or_create_media_stream_track(sender_->track());
}

uint32_t RtpSender::ssrc() const {
  return sender_->ssrc();
}

void RtpSender::get_stats(
    rust::Box<SenderContext> ctx,
    rust::Fn<void(rust::Box<SenderContext>, rust::String)> on_stats) const {
  auto observer =
      webrtc::make_ref_counted<NativeRtcStatsCollector<SenderContext>>(std::move(ctx), on_stats);
  peer_connection_->GetStats(sender_, observer);
}

MediaType RtpSender::media_type() const {
  return static_cast<MediaType>(sender_->media_type());
}

rust::String RtpSender::id() const {
  return sender_->id();
}

rust::Vec<rust::String> RtpSender::stream_ids() const {
  rust::Vec<rust::String> vec;
  for (auto str : sender_->stream_ids())
    vec.push_back(str);

  return vec;
}

void RtpSender::set_streams(const rust::Vec<rust::String>& stream_ids) const {
  std::vector<std::string> std_stream_ids(stream_ids.begin(), stream_ids.end());
  sender_->SetStreams(std_stream_ids);
}

rust::Vec<RtpEncodingParameters> RtpSender::init_send_encodings() const {
  rust::Vec<RtpEncodingParameters> encodings;
  for (auto encoding : sender_->init_send_encodings())
    encodings.push_back(to_rust_rtp_encoding_parameters(encoding));
  return encodings;
}

RtpParameters RtpSender::get_parameters() const {
  return to_rust_rtp_parameters(sender_->GetParameters());
}

void RtpSender::set_parameters(RtpParameters params) const {
  auto error = sender_->SetParameters(to_native_rtp_parameters(params));
  if (!error.ok())
    throw std::runtime_error(serialize_error(to_error(error)));
}

}  // namespace livekit_ffi
