// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SESSION_DESCRIPTION_REQUEST_PROMISE_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SESSION_DESCRIPTION_REQUEST_PROMISE_IMPL_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_session_description_enums.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_session_description_request.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class RTCPeerConnection;
class RTCSessionDescriptionInit;
class RTCSessionDescriptionPlatform;

// TODO(https://crbug.com/908468): Split up the operation-specific codepaths
// into separate request implementations and find a way to consolidate the
// shared code as to not repeat the majority of the implementations.
class RTCSessionDescriptionRequestPromiseImpl final
    : public RTCSessionDescriptionRequest {
 public:
  static RTCSessionDescriptionRequestPromiseImpl* Create(
      RTCPeerConnection*,
      ScriptPromiseResolver<RTCSessionDescriptionInit>*);

  RTCSessionDescriptionRequestPromiseImpl(
      RTCPeerConnection*,
      ScriptPromiseResolver<RTCSessionDescriptionInit>*);
  ~RTCSessionDescriptionRequestPromiseImpl() override;

  // RTCSessionDescriptionRequest
  void RequestSucceeded(RTCSessionDescriptionPlatform*) override;
  void RequestFailed(const webrtc::RTCError& error) override;

  void Trace(Visitor*) const override;

 private:
  void Clear();

  Member<RTCPeerConnection> requester_;
  Member<ScriptPromiseResolver<RTCSessionDescriptionInit>> resolver_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_SESSION_DESCRIPTION_REQUEST_PROMISE_IMPL_H_
