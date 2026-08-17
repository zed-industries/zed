// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SEND_RESULT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SEND_RESULT_H_

#include "third_party/blink/renderer/bindings/core/v8/script_promise.h"
#include "third_party/blink/renderer/bindings/core/v8/script_promise_resolver.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_packet_init.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_send_options.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_rtp_send_result.h"
#include "third_party/blink/renderer/modules/peerconnection/rtc_rtp_sent.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class V8RTCRtpUnsentReason;

class RTCRtpSendResult : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCRtpSent* sent();
  std::optional<V8RTCRtpUnsentReason> unsent();
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SEND_RESULT_H_
