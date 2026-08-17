// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSPORT_PROCESSOR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSPORT_PROCESSOR_H_

#include "base/memory/weak_ptr.h"
#include "third_party/blink/renderer/core/execution_context/execution_context_lifecycle_observer.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_acks.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_sent.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_deque.h"
#include "third_party/webrtc/api/transport/network_control.h"

namespace blink {

class FeedbackProvider;

class MODULES_EXPORT RTCRtpTransportProcessor : public ScriptWrappable,
                                                public ExecutionContextClient {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit RTCRtpTransportProcessor(ExecutionContext* context);
  ~RTCRtpTransportProcessor() override;

  // Implementation of IDL type.
  HeapVector<Member<RTCRtpAcks>> readReceivedAcks(uint32_t maxCount);
  HeapVector<Member<RTCRtpSent>> readSentRtp(uint32_t maxCount);

  uint64_t customMaxBandwidth() { return custom_max_bitrate_bps_; }
  void setCustomMaxBandwidth(uint64_t custom_max_bitrate_bps);

  // Endpoints called on BWE events from libwebrtc.
  webrtc::NetworkControlUpdate OnFeedback(
      webrtc::TransportPacketsFeedback feedback);
  void OnSentPacket(webrtc::SentPacket sp);

  void SetFeedbackProviders(
      Vector<scoped_refptr<FeedbackProvider>> feedback_providers);

  void Trace(Visitor* visitor) const override;

 private:
  HeapDeque<Member<RTCRtpAcks>> acks_messages_;
  HeapDeque<Member<RTCRtpSent>> sents_;

  Vector<scoped_refptr<FeedbackProvider>> feedback_providers_;
  uint64_t custom_max_bitrate_bps_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_TRANSPORT_PROCESSOR_H_
