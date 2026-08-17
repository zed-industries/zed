// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SENT_H_

#include <optional>

#include "third_party/blink/renderer/platform/bindings/script_wrappable.h"

namespace blink {

class RTCRtpSent : public ScriptWrappable {
  DEFINE_WRAPPERTYPEINFO();

 public:
  RTCRtpSent(double time, uint64_t ackId, uint64_t size)
      : time_(time), ackId_(ackId), size_(size) {}

  // Implements rtc_rtp_sent.idl
  double time();
  std::optional<uint64_t> ackId();
  uint64_t size();

 private:
  double time_;
  uint64_t ackId_;
  uint64_t size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_PEERCONNECTION_RTC_RTP_SENT_H_
