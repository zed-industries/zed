// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSCEIVER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSCEIVER_H_

#include <optional>

#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_codec_capability.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_transceiver_direction.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_transceiver_init.h"
#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/platform/bindings/exception_state.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/heap/visitor.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_transceiver_platform.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/rtp_transceiver_interface.h"

namespace blink {

class RTCPeerConnection;
class RTCRtpHeaderExtensionCapability;
class RTCRtpReceiver;
class RTCRtpSender;

webrtc::RtpTransceiverInit ToRtpTransceiverInit(ExecutionContext* context,
                                                const RTCRtpTransceiverInit*,
                                                const String& kind);

class RTCRtpTransceiver final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCRtpTransceiver(RTCPeerConnection* pc,
                    std::unique_ptr<RTCRtpTransceiverPlatform>,
                    RTCRtpSender*,
                    RTCRtpReceiver*);

  // rtc_rtp_transceiver.idl
  String mid() const;
  RTCRtpSender* sender() const;
  RTCRtpReceiver* receiver() const;
  bool stopped() const;
  // Enum type RTCRtpTransceiverDirection
  V8RTCRtpTransceiverDirection direction() const;
  void setDirection(const V8RTCRtpTransceiverDirection& direction,
                    ExceptionState&);
  std::optional<V8RTCRtpTransceiverDirection> currentDirection() const;
  void stop(ExceptionState&);
  void setCodecPreferences(
      const HeapVector<Member<RTCRtpCodecCapability>>& codecs,
      ExceptionState& exception_state);

  // Updates the transceiver attributes by fetching values from
  // |platform_transceiver_|. This is made an explicit operation (rather than
  // fetching the values from the |platform_transceivers_| directly every time
  // an attribute is read) to make it possible to look at before/after snapshots
  // of the attributes when a session description has been applied. This is
  // necessary in order for blink to know when to process the addition/removal
  // of remote tracks:
  // https://w3c.github.io/webrtc-pc/#set-the-rtcsessiondescription.
  void UpdateMembers();
  // Stopped transceivers are removed, but we don't have access to removed
  // transceivers' internal states. This method updates the states to reflect
  // being stopped.
  void OnTransceiverStopped();

  RTCRtpTransceiverPlatform* platform_transceiver() const;
  std::optional<webrtc::RtpTransceiverDirection> fired_direction() const;
  bool DirectionHasSend() const;
  bool DirectionHasRecv() const;
  bool FiredDirectionHasRecv() const;

  void setHeaderExtensionsToNegotiate(
      const HeapVector<Member<RTCRtpHeaderExtensionCapability>>& extensions,
      ExceptionState& exception_state);
  HeapVector<Member<RTCRtpHeaderExtensionCapability>>
  getHeaderExtensionsToNegotiate() const;
  HeapVector<Member<RTCRtpHeaderExtensionCapability>>
  getNegotiatedHeaderExtensions() const;

  void Trace(Visitor*) const override;

 private:
  Member<RTCPeerConnection> pc_;
  std::unique_ptr<RTCRtpTransceiverPlatform> platform_transceiver_;
  Member<RTCRtpSender> sender_;
  Member<RTCRtpReceiver> receiver_;
  String mid_;
  V8RTCRtpTransceiverDirection::Enum direction_;
  std::optional<V8RTCRtpTransceiverDirection::Enum> current_direction_;
  std::optional<webrtc::RtpTransceiverDirection> fired_direction_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSCEIVER_H_
