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

#include "livekit/rtp_transceiver.h"

#include "api/peer_connection_interface.h"
#include "api/scoped_refptr.h"

namespace livekit_ffi {

webrtc::RtpTransceiverInit to_native_rtp_transceiver_init(
    RtpTransceiverInit init) {
  {
    webrtc::RtpTransceiverInit native{};
    native.direction =
        static_cast<webrtc::RtpTransceiverDirection>(init.direction);
    native.stream_ids = std::vector<std::string>(init.stream_ids.begin(),
                                                 init.stream_ids.end());
    for (auto encoding : init.send_encodings)
      native.send_encodings.push_back(
          to_native_rtp_encoding_paramters(encoding));
    return native;
  }
}

RtpTransceiver::RtpTransceiver(
    std::shared_ptr<RtcRuntime> rtc_runtime,
    webrtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver,
    webrtc::scoped_refptr<webrtc::PeerConnectionInterface> peer_connection)
    : rtc_runtime_(rtc_runtime),
      transceiver_(std::move(transceiver)),
      peer_connection_(std::move(peer_connection)) {}

MediaType RtpTransceiver::media_type() const {
  return static_cast<MediaType>(transceiver_->media_type());
}

rust::String RtpTransceiver::mid() const {
  // The error/Result is converted into an Option in Rust (Wait for Option
  // suport in cxx.rs) (value throws an error if there's no value)
  return transceiver_->mid().value();
}

std::shared_ptr<RtpSender> RtpTransceiver::sender() const {
  return std::make_shared<RtpSender>(rtc_runtime_, transceiver_->sender(),
                                     peer_connection_);
}

std::shared_ptr<RtpReceiver> RtpTransceiver::receiver() const {
  return std::make_shared<RtpReceiver>(rtc_runtime_, transceiver_->receiver(),
                                       peer_connection_);
}

bool RtpTransceiver::stopped() const {
  return transceiver_->stopped();
}

bool RtpTransceiver::stopping() const {
  return transceiver_->stopping();
}

RtpTransceiverDirection RtpTransceiver::direction() const {
  return static_cast<RtpTransceiverDirection>(transceiver_->direction());
}

void RtpTransceiver::set_direction(RtpTransceiverDirection direction) const {
  auto error = transceiver_->SetDirectionWithError(
      static_cast<webrtc::RtpTransceiverDirection>(direction));

  if (!error.ok()) {
    throw std::runtime_error(serialize_error(to_error(error)));
  }
}

RtpTransceiverDirection RtpTransceiver::current_direction() const {
  return static_cast<RtpTransceiverDirection>(
      transceiver_->current_direction().value());
}

RtpTransceiverDirection RtpTransceiver::fired_direction() const {
  return static_cast<RtpTransceiverDirection>(
      transceiver_->fired_direction().value());
}

void RtpTransceiver::stop_standard() const {
  auto error = transceiver_->StopStandard();
  if (!error.ok())
    throw std::runtime_error(serialize_error(to_error(error)));
}

void RtpTransceiver::set_codec_preferences(
    rust::Vec<RtpCodecCapability> codecs) const {
  std::vector<webrtc::RtpCodecCapability> std_codecs;

  for (auto codec : codecs)
    std_codecs.push_back(to_native_rtp_codec_capability(codec));

  auto error = transceiver_->SetCodecPreferences(std_codecs);
  if (!error.ok())
    throw std::runtime_error(serialize_error(to_error(error)));
}

rust::Vec<RtpCodecCapability> RtpTransceiver::codec_preferences() const {
  rust::Vec<RtpCodecCapability> rust;
  for (auto codec : transceiver_->codec_preferences())
    rust.push_back(to_rust_rtp_codec_capability(codec));

  return rust;
}

rust::Vec<RtpHeaderExtensionCapability>
RtpTransceiver::header_extensions_to_negotiate() const {
  rust::Vec<RtpHeaderExtensionCapability> rust;
  for (auto header : transceiver_->GetHeaderExtensionsToNegotiate())
    rust.push_back(to_rust_rtp_header_extension_capability(header));

  return rust;
}

rust::Vec<RtpHeaderExtensionCapability>
RtpTransceiver::negotiated_header_extensions() const {
  rust::Vec<RtpHeaderExtensionCapability> rust;
  for (auto header : transceiver_->GetNegotiatedHeaderExtensions())
    rust.push_back(to_rust_rtp_header_extension_capability(header));

  return rust;
}

void RtpTransceiver::set_header_extensions_to_negotiate(
    rust::Vec<RtpHeaderExtensionCapability> header_extensions_to_offer) const {
  std::vector<webrtc::RtpHeaderExtensionCapability> headers;

  for (auto header : header_extensions_to_offer)
    headers.push_back(to_native_rtp_header_extension_capability(header));

  auto error = transceiver_->SetHeaderExtensionsToNegotiate(headers);
  if (!error.ok())
    throw std::runtime_error(serialize_error(to_error(error)));
}

}  // namespace livekit_ffi
