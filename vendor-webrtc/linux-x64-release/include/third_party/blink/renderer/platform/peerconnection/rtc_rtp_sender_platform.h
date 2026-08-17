// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_SENDER_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_SENDER_PLATFORM_H_

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

class RtcDtmfSenderHandler;
class RTCEncodedAudioStreamTransformer;
class RTCEncodedVideoStreamTransformer;
class RTCVoidRequest;
class MediaStreamComponent;

// Implementations of this interface keep the corresponding WebRTC-layer sender
// alive through reference counting. Multiple |RTCRtpSenderPlatform|s could
// reference the same sender; check for equality with |id|.
// https://w3c.github.io/webrtc-pc/#rtcrtpsender-interface
class PLATFORM_EXPORT RTCRtpSenderPlatform {
 public:
  virtual ~RTCRtpSenderPlatform();
  virtual std::unique_ptr<RTCRtpSenderPlatform> ShallowCopy() const = 0;

  // Two |RTCRtpSenderPlatform|s referencing the same WebRTC-layer sender have
  // the same |id|. IDs are guaranteed to be unique amongst senders but they are
  // allowed to be reused after a sender is destroyed.
  virtual uintptr_t Id() const = 0;
  virtual webrtc::scoped_refptr<webrtc::DtlsTransportInterface>
  DtlsTransport() = 0;
  // Note: For convenience, DtlsTransportInformation always returns a value.
  // The information is only interesting if DtlsTransport() is non-null.
  virtual webrtc::DtlsTransportInformation DtlsTransportInformation() = 0;
  virtual MediaStreamComponent* Track() const = 0;
  virtual Vector<String> StreamIds() const = 0;
  // TODO(hbos): Replace RTCVoidRequest by something resolving promises based
  // on RTCError, as to surface both exception type and error message.
  // https://crbug.com/790007
  virtual void ReplaceTrack(MediaStreamComponent*, RTCVoidRequest*) = 0;
  virtual std::unique_ptr<RtcDtmfSenderHandler> GetDtmfSender() const = 0;
  virtual std::unique_ptr<webrtc::RtpParameters> GetParameters() const = 0;
  virtual void SetParameters(Vector<webrtc::RtpEncodingParameters>,
                             std::optional<webrtc::DegradationPreference>,
                             RTCVoidRequest*) = 0;
  virtual void GetStats(RTCStatsReportCallback) = 0;
  virtual void SetStreams(const Vector<String>& stream_ids) = 0;
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

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_SENDER_PLATFORM_H_
