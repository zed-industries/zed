// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_RECEIVER_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_RECEIVER_PLATFORM_H_

#include <memory>
#include <optional>

#include "third_party/blink/renderer/platform/peerconnection/rtc_stats.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/dtls_transport_interface.h"
#include "third_party/webrtc/api/rtp_parameters.h"
#include "third_party/webrtc/api/stats/rtc_stats.h"

namespace blink {

class RTCEncodedAudioStreamTransformer;
class RTCEncodedVideoStreamTransformer;
class RTCRtpSource;
class MediaStreamComponent;

// Implementations of this interface keep the corresponding WebRTC-layer
// receiver alive through reference counting. Multiple |RTCRtpReceiverPlatform|s
// could reference the same receiver, see |id|.
// https://w3c.github.io/webrtc-pc/#rtcrtpreceiver-interface
class PLATFORM_EXPORT RTCRtpReceiverPlatform {
 public:
  virtual ~RTCRtpReceiverPlatform();

  virtual std::unique_ptr<RTCRtpReceiverPlatform> ShallowCopy() const = 0;
  // Two |RTCRtpReceiverPlatform|s referencing the same WebRTC-layer receiver
  // have the same |id|.
  virtual uintptr_t Id() const = 0;
  virtual webrtc::scoped_refptr<webrtc::DtlsTransportInterface>
  DtlsTransport() = 0;
  // Note: For convenience, DtlsTransportInformation always returns a value.
  // The information is only interesting if DtlsTransport() is non-null.
  virtual webrtc::DtlsTransportInformation DtlsTransportInformation() = 0;
  virtual MediaStreamComponent* Track() const = 0;
  virtual Vector<String> StreamIds() const = 0;
  // If called from any other thread than the WebRTC worker thread, this causes
  // a block-invoke by the PROXY.
  virtual Vector<std::unique_ptr<RTCRtpSource>> GetSources() = 0;
  virtual void GetStats(RTCStatsReportCallback) = 0;
  virtual std::unique_ptr<webrtc::RtpParameters> GetParameters() const = 0;
  virtual void SetJitterBufferMinimumDelay(
      std::optional<double> delay_seconds) = 0;
  virtual RTCEncodedAudioStreamTransformer* GetEncodedAudioStreamTransformer()
      const {
    return nullptr;
  }
  virtual RTCEncodedVideoStreamTransformer* GetEncodedVideoStreamTransformer()
      const {
    return nullptr;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_RECEIVER_PLATFORM_H_
