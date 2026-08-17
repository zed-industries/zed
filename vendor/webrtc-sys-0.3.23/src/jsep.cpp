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

#include "livekit/jsep.h"

#include <iomanip>
#include <memory>

#include "livekit/rtc_error.h"
#include "rtc_base/ref_counted_object.h"
#include "rust/cxx.h"

namespace livekit_ffi {

std::string serialize_sdp_error(webrtc::SdpParseError error) {
  std::stringstream ss;
  ss << std::hex << std::setfill('0');
  ss << std::setw(8) << (uint32_t)error.line.length();
  ss << std::dec << std::setw(1) << error.line;
  ss << std::dec << std::setw(1) << error.description;
  return ss.str();
}

IceCandidate::IceCandidate(
    std::unique_ptr<webrtc::IceCandidateInterface> ice_candidate)
    : ice_candidate_(std::move(ice_candidate)) {}

rust::String IceCandidate::sdp_mid() const {
  return ice_candidate_->sdp_mid();
}

int IceCandidate::sdp_mline_index() const {
  return ice_candidate_->sdp_mline_index();
}

rust::String IceCandidate::candidate() const {
  return stringify();
}

rust::String IceCandidate::stringify() const {
  std::string str;
  ice_candidate_->ToString(&str);
  return rust::String::lossy(str);
}

std::unique_ptr<webrtc::IceCandidateInterface> IceCandidate::release() {
  return std::move(ice_candidate_);
}

std::shared_ptr<IceCandidate> create_ice_candidate(rust::String sdp_mid,
                                                   int sdp_mline_index,
                                                   rust::String sdp) {
  webrtc::SdpParseError error;
  auto ice_rtc = webrtc::CreateIceCandidate(sdp_mid.c_str(), sdp_mline_index,
                                            sdp.c_str(), &error);
  if (!ice_rtc) {
    throw std::runtime_error(serialize_sdp_error(error));
  }

  return std::make_shared<IceCandidate>(
      std::unique_ptr<webrtc::IceCandidateInterface>(ice_rtc));
}

SessionDescription::SessionDescription(
    std::unique_ptr<webrtc::SessionDescriptionInterface> session_description)
    : session_description_(std::move(session_description)) {}

SdpType SessionDescription::sdp_type() const {
  return static_cast<SdpType>(session_description_->GetType());
}

rust::String SessionDescription::stringify() const {
  std::string str;
  session_description_->ToString(&str);
  return rust::String::lossy(str);
}

std::unique_ptr<SessionDescription> SessionDescription::clone() const {
  return std::make_unique<SessionDescription>(session_description_->Clone());
}

std::unique_ptr<webrtc::SessionDescriptionInterface>
SessionDescription::release() {
  return std::move(session_description_);
}

std::unique_ptr<SessionDescription> create_session_description(
    SdpType type,
    rust::String sdp) {
  webrtc::SdpParseError error;
  auto rtc_sdp = webrtc::CreateSessionDescription(
      static_cast<webrtc::SdpType>(type), sdp.c_str(), &error);
  if (!rtc_sdp) {
    throw std::runtime_error(serialize_sdp_error(error));
  }

  return std::make_unique<SessionDescription>(std::move(rtc_sdp));
}

NativeCreateSdpObserver::NativeCreateSdpObserver(
    rust::Box<PeerContext> ctx,
    rust::Fn<void(rust::Box<PeerContext>, std::unique_ptr<SessionDescription>)>
        on_success,
    rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_error)
    : ctx_(std::move(ctx)), on_success_(on_success), on_error_(on_error) {}

void NativeCreateSdpObserver::OnSuccess(
    webrtc::SessionDescriptionInterface* desc) {
  // We have ownership of desc
  on_success_(std::move(ctx_),
              std::make_unique<SessionDescription>(
                  std::unique_ptr<webrtc::SessionDescriptionInterface>(desc)));
}

void NativeCreateSdpObserver::OnFailure(webrtc::RTCError error) {
  on_error_(std::move(ctx_), to_error(error));
}

NativeSetLocalSdpObserver::NativeSetLocalSdpObserver(
    rust::Box<PeerContext> ctx,
    rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_complete)
    : ctx_(std::move(ctx)), on_complete_(on_complete) {}

void NativeSetLocalSdpObserver::OnSetLocalDescriptionComplete(
    webrtc::RTCError error) {
  on_complete_(std::move(ctx_), to_error(error));
}

NativeSetRemoteSdpObserver::NativeSetRemoteSdpObserver(
    rust::Box<PeerContext> ctx,
    rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_complete)
    : ctx_(std::move(ctx)), on_complete_(on_complete) {}

void NativeSetRemoteSdpObserver::OnSetRemoteDescriptionComplete(
    webrtc::RTCError error) {
  on_complete_(std::move(ctx_), to_error(error));
}
}  // namespace livekit_ffi
