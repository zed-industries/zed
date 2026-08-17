// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_H_

#include <optional>

#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_error_detail_type.h"
#include "third_party/blink/renderer/bindings/modules/v8/v8_rtc_error_init.h"
#include "third_party/blink/renderer/core/dom/dom_exception.h"
#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/webrtc/api/rtc_error.h"

namespace blink {

class RTCError final : public DOMException {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static RTCError* Create(const RTCErrorInit* init, String message);
  RTCError(const RTCErrorInit* init, String message);
  explicit RTCError(webrtc::RTCError);

  V8RTCErrorDetailType errorDetail() const;
  std::optional<int32_t> sdpLineNumber() const { return sdp_line_number_; }
  std::optional<int32_t> httpRequestStatusCode() const {
    return http_request_status_code_;
  }
  std::optional<int32_t> sctpCauseCode() const { return sctp_cause_code_; }
  std::optional<uint32_t> receivedAlert() const { return received_alert_; }
  std::optional<uint32_t> sentAlert() const { return sent_alert_; }

 private:
  // idl enum RTCErrorDetailType.
  V8RTCErrorDetailType::Enum error_detail_;
  std::optional<int32_t> sdp_line_number_;
  std::optional<int32_t> http_request_status_code_;
  std::optional<int32_t> sctp_cause_code_;
  std::optional<uint32_t> received_alert_;
  std::optional<uint32_t> sent_alert_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_ERROR_H_
