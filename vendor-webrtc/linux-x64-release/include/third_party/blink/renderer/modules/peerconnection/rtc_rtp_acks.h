// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_ACKS_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_ACKS_H_

#include "third_party/blink/renderer/bindings/modules/v8/v8_explicit_congestion_notification.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_ack.h"
#include "third_party/blink/renderer/core/dom/dom_high_res_time_stamp.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_vector.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class MODULES_EXPORT RTCRtpAcks final : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCRtpAcks(HeapVector<Member<RTCRtpAck>> acks,
             uint64_t remote_send_timestamp,
             double received_time,
             V8ExplicitCongestionNotification explicit_congestion_notification)
      : acks_(acks),
        remote_send_timestamp_(remote_send_timestamp),
        received_time_(received_time),
        explicit_congestion_notification_(explicit_congestion_notification) {}

  HeapVector<Member<RTCRtpAck>> acks() { return acks_; }

  uint64_t remoteSendTimestamp() { return remote_send_timestamp_; }

  DOMHighResTimeStamp receivedTime() { return received_time_; }

  V8ExplicitCongestionNotification explicitCongestionNotification() {
    return explicit_congestion_notification_;
  }

  void Trace(Visitor* visitor) const override {
    ScriptWrappable::Trace(visitor);
    visitor->Trace(acks_);
  }

 private:
  HeapVector<Member<RTCRtpAck>> acks_;
  uint64_t remote_send_timestamp_;
  DOMHighResTimeStamp received_time_;
  const V8ExplicitCongestionNotification explicit_congestion_notification_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_ACKS_H_
