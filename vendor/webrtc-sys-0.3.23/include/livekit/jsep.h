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

#include "api/jsep.h"
#include "api/ref_counted_base.h"
#include "api/set_local_description_observer_interface.h"
#include "api/set_remote_description_observer_interface.h"
#include "api/stats/rtc_stats_collector_callback.h"
#include "livekit/rtc_error.h"
#include "rtc_base/ref_count.h"
#include "rust/cxx.h"

namespace livekit_ffi {
class IceCandidate;
class SessionDescription;
};  // namespace livekit_ffi
#include "webrtc-sys/src/jsep.rs.h"

namespace livekit_ffi {

class PeerContext;

class IceCandidate {
 public:
  explicit IceCandidate(
      std::unique_ptr<webrtc::IceCandidateInterface> ice_candidate);

  rust::String sdp_mid() const;
  int sdp_mline_index() const;
  rust::String candidate() const;  // TODO(theomonnom) Return livekit_ffi::Candidate
                                   // instead of rust::String

  rust::String stringify() const;
  std::unique_ptr<webrtc::IceCandidateInterface> release();

 private:
  std::unique_ptr<webrtc::IceCandidateInterface> ice_candidate_;
};

std::shared_ptr<IceCandidate> create_ice_candidate(rust::String sdp_mid,
                                                   int sdp_mline_index,
                                                   rust::String sdp);

static std::shared_ptr<IceCandidate> _shared_ice_candidate() {
  return nullptr;  // Ignore
}

class SessionDescription {
 public:
  explicit SessionDescription(
      std::unique_ptr<webrtc::SessionDescriptionInterface> session_description);

  SdpType sdp_type() const;
  rust::String stringify() const;
  std::unique_ptr<SessionDescription> clone() const;
  std::unique_ptr<webrtc::SessionDescriptionInterface> release();

 private:
  std::unique_ptr<webrtc::SessionDescriptionInterface> session_description_;
};

std::unique_ptr<SessionDescription> create_session_description(
    SdpType type,
    rust::String sdp);

static std::unique_ptr<SessionDescription> _unique_session_description() {
  return nullptr;  // Ignore
}

class NativeCreateSdpObserver
    : public webrtc::CreateSessionDescriptionObserver {
 public:
  NativeCreateSdpObserver(
      rust::Box<PeerContext> ctx,
      rust::Fn<void(rust::Box<PeerContext> ctx,
                    std::unique_ptr<SessionDescription>)> on_success,
      rust::Fn<void(rust::Box<PeerContext> ctx, RtcError)> on_error);

  void OnSuccess(webrtc::SessionDescriptionInterface* desc) override;
  void OnFailure(webrtc::RTCError error) override;

 private:
  rust::Box<PeerContext> ctx_;
  rust::Fn<void(rust::Box<PeerContext>, std::unique_ptr<SessionDescription>)>
      on_success_;
  rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_error_;
};

class NativeSetLocalSdpObserver
    : public webrtc::SetLocalDescriptionObserverInterface {
 public:
  NativeSetLocalSdpObserver(
      rust::Box<PeerContext> ctx,
      rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_complete);

  void OnSetLocalDescriptionComplete(webrtc::RTCError error) override;

 private:
  rust::Box<PeerContext> ctx_;
  rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_complete_;
};

class NativeSetRemoteSdpObserver
    : public webrtc::SetRemoteDescriptionObserverInterface {
 public:
  NativeSetRemoteSdpObserver(
      rust::Box<PeerContext> ctx,
      rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_complete);

  void OnSetRemoteDescriptionComplete(webrtc::RTCError error) override;

 private:
  rust::Box<PeerContext> ctx_;
  rust::Fn<void(rust::Box<PeerContext>, RtcError)> on_complete_;
};

template <class T>  // Context type
class NativeRtcStatsCollector : public webrtc::RTCStatsCollectorCallback {
 public:
  NativeRtcStatsCollector(rust::Box<T> ctx,
                          rust::Fn<void(rust::Box<T>, rust::String)> on_stats)
      : ctx_(std::move(ctx)), on_stats_(on_stats) {}

  void OnStatsDelivered(
      const webrtc::scoped_refptr<const webrtc::RTCStatsReport>& report) override {
    on_stats_(std::move(ctx_), report->ToJson());
  }

 private:
  rust::Box<T> ctx_;
  rust::Fn<void(rust::Box<T>, rust::String)> on_stats_;
};

}  // namespace livekit_ffi
