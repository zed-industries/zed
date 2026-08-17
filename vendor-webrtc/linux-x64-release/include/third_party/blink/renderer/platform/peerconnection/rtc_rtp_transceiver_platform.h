// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_TRANSCEIVER_PLATFORM_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_TRANSCEIVER_PLATFORM_H_

#include <memory>
#include <optional>

#include "base/notreached.h"
#include "third_party/blink/renderer/platform/peerconnection/rtc_rtp_receiver_platform.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"
#include "third_party/webrtc/api/rtp_transceiver_interface.h"

namespace blink {

class RTCRtpReceiverPlatform;
class RTCRtpSenderPlatform;

// Interface for content to implement as to allow the surfacing of transceivers.
// TODO(hbos): [Onion Soup] Remove the content layer versions of this class and
// rely on webrtc directly from blink. Requires coordination with senders and
// receivers. https://crbug.com/787254
class PLATFORM_EXPORT RTCRtpTransceiverPlatform {
 public:
  virtual ~RTCRtpTransceiverPlatform();

  // Identifies the webrtc-layer transceiver. Multiple RTCRtpTransceiverPlatform
  // can exist for the same webrtc-layer transceiver.
  virtual uintptr_t Id() const = 0;
  virtual String Mid() const = 0;
  virtual std::unique_ptr<RTCRtpSenderPlatform> Sender() const = 0;
  virtual std::unique_ptr<RTCRtpReceiverPlatform> Receiver() const = 0;
  virtual webrtc::RtpTransceiverDirection Direction() const = 0;
  virtual webrtc::RTCError SetDirection(webrtc::RtpTransceiverDirection) = 0;
  virtual std::optional<webrtc::RtpTransceiverDirection> CurrentDirection()
      const = 0;
  virtual std::optional<webrtc::RtpTransceiverDirection> FiredDirection()
      const = 0;
  virtual webrtc::RTCError Stop() = 0;
  virtual webrtc::RTCError SetCodecPreferences(
      Vector<webrtc::RtpCodecCapability>) = 0;
  virtual webrtc::RTCError SetHeaderExtensionsToNegotiate(
      Vector<webrtc::RtpHeaderExtensionCapability> header_extensions) = 0;
  virtual Vector<webrtc::RtpHeaderExtensionCapability>
  GetNegotiatedHeaderExtensions() const = 0;
  virtual Vector<webrtc::RtpHeaderExtensionCapability>
  GetHeaderExtensionsToNegotiate() const = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_PEERCONNECTION_RTC_RTP_TRANSCEIVER_PLATFORM_H_
