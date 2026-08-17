/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer
 *    in the documentation and/or other materials provided with the
 *    distribution.
 * 3. Neither the name of Google Inc. nor the names of its contributors
 *    may be used to endorse or promote products derived from this
 *    software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ICE_CANDIDATE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ICE_CANDIDATE_H_

#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_ice_candidate_platform.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"

namespace blink {

class RTCIceCandidateInit;
class ExceptionState;
class ExecutionContext;
class ScriptObject;
class ScriptState;
class V8RTCIceCandidateType;
class V8RTCIceComponent;
class V8RTCIceProtocol;
class V8RTCIceServerTransportProtocol;
class V8RTCIceTcpCandidateType;

class MODULES_EXPORT RTCIceCandidate final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static RTCIceCandidate* Create(ExecutionContext*,
                                 const RTCIceCandidateInit*,
                                 ExceptionState&);
  static RTCIceCandidate* Create(RTCIceCandidatePlatform*);

  explicit RTCIceCandidate(RTCIceCandidatePlatform*);

  String candidate() const;
  String sdpMid() const;
  std::optional<uint16_t> sdpMLineIndex() const;
  String foundation() const;
  std::optional<V8RTCIceComponent> component() const;
  std::optional<uint32_t> priority() const;
  String address() const;
  std::optional<V8RTCIceProtocol> protocol() const;
  std::optional<uint16_t> port() const;
  std::optional<V8RTCIceCandidateType> type() const;
  std::optional<V8RTCIceTcpCandidateType> tcpType() const;
  String relatedAddress() const;
  std::optional<uint16_t> relatedPort() const;
  String usernameFragment() const;
  std::optional<V8RTCIceServerTransportProtocol> relayProtocol() const;
  String url() const;

  ScriptObject toJSONForBinding(ScriptState*);

  RTCIceCandidatePlatform* PlatformCandidate() const;

  void Trace(Visitor*) const override;

 private:
  Member<RTCIceCandidatePlatform> platform_candidate_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ICE_CANDIDATE_H_
