// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_SOURCE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_SOURCE_H_

#include <optional>

#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/webrtc/api/rtp_receiver_interface.h"

namespace base {
class TimeTicks;
}

namespace webrtc {
class RtpSource;
}

namespace blink {

class PLATFORM_EXPORT RTCRtpSource {
 public:
  enum class Type {
    kSSRC,
    kCSRC,
  };

  explicit RTCRtpSource(const webrtc::RtpSource& source);
  RTCRtpSource(const RTCRtpSource&) = delete;
  RTCRtpSource& operator=(const RTCRtpSource&) = delete;
  ~RTCRtpSource();

  Type SourceType() const;
  base::TimeTicks Timestamp() const;
  uint32_t Source() const;
  std::optional<double> AudioLevel() const;
  uint32_t RtpTimestamp() const;
  std::optional<int64_t> CaptureTimestamp() const;
  std::optional<int64_t> SenderCaptureTimeOffset() const;

 private:
  const webrtc::RtpSource source_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_SOURCE_H_
