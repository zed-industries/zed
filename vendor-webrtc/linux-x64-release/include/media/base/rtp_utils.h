/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_RTP_UTILS_H_
#define MEDIA_BASE_RTP_UTILS_H_

#include <cstddef>
#include <cstdint>

#include "absl/strings/string_view.h"
#include "api/array_view.h"
#include "rtc_base/async_packet_socket.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

const size_t kMinRtpPacketLen = 12;
const size_t kMaxRtpPacketLen = 2048;
const size_t kMinRtcpPacketLen = 4;

enum RtcpTypes {
  kRtcpTypeSR = 200,     // Sender report payload type.
  kRtcpTypeRR = 201,     // Receiver report payload type.
  kRtcpTypeSDES = 202,   // SDES payload type.
  kRtcpTypeBye = 203,    // BYE payload type.
  kRtcpTypeApp = 204,    // APP payload type.
  kRtcpTypeRTPFB = 205,  // Transport layer Feedback message payload type.
  kRtcpTypePSFB = 206,   // Payload-specific Feedback message payload type.
};

enum class RtpPacketType {
  kRtp,
  kRtcp,
  kUnknown,
};

bool GetRtcpType(const void* data, size_t len, int* value);
bool GetRtcpSsrc(const void* data, size_t len, uint32_t* value);

// Checks the packet header to determine if it can be an RTP or RTCP packet.
RtpPacketType InferRtpPacketType(ArrayView<const uint8_t> packet);
// True if |payload type| is 0-127.
bool IsValidRtpPayloadType(int payload_type);

// True if `size` is appropriate for the indicated packet type.
bool IsValidRtpPacketSize(RtpPacketType packet_type, size_t size);

// Returns "RTCP", "RTP" or "Unknown" according to `packet_type`.
absl::string_view RtpPacketTypeToString(RtpPacketType packet_type);

// Verifies that a packet has a valid RTP header.
bool RTC_EXPORT ValidateRtpHeader(const uint8_t* rtp,
                                  size_t length,
                                  size_t* header_length);

// Helper method which updates the absolute send time extension if present.
bool UpdateRtpAbsSendTimeExtension(uint8_t* rtp,
                                   size_t length,
                                   int extension_id,
                                   uint64_t time_us);

// Applies specified `options` to the packet. It updates the absolute send time
// extension header if it is present present then updates HMAC.
bool RTC_EXPORT
ApplyPacketOptions(uint8_t* data,
                   size_t length,
                   const PacketTimeUpdateParams& packet_time_params,
                   uint64_t time_us);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::ApplyPacketOptions;
using ::webrtc::GetRtcpSsrc;
using ::webrtc::GetRtcpType;
using ::webrtc::InferRtpPacketType;
using ::webrtc::IsValidRtpPacketSize;
using ::webrtc::IsValidRtpPayloadType;
using ::webrtc::kMaxRtpPacketLen;
using ::webrtc::kMinRtcpPacketLen;
using ::webrtc::kMinRtpPacketLen;
using ::webrtc::kRtcpTypeApp;
using ::webrtc::kRtcpTypeBye;
using ::webrtc::kRtcpTypePSFB;
using ::webrtc::kRtcpTypeRR;
using ::webrtc::kRtcpTypeRTPFB;
using ::webrtc::kRtcpTypeSDES;
using ::webrtc::kRtcpTypeSR;
using ::webrtc::RtcpTypes;
using ::webrtc::RtpPacketType;
using ::webrtc::RtpPacketTypeToString;
using ::webrtc::UpdateRtpAbsSendTimeExtension;
using ::webrtc::ValidateRtpHeader;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_RTP_UTILS_H_
