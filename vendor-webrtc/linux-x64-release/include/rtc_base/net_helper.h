/*
 *  Copyright 2017 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef RTC_BASE_NET_HELPER_H_
#define RTC_BASE_NET_HELPER_H_


#include "absl/strings/string_view.h"
#include "rtc_base/system/rtc_export.h"

// This header contains helper functions and constants used by different types
// of transports.
namespace webrtc {

RTC_EXPORT extern const char UDP_PROTOCOL_NAME[];
RTC_EXPORT extern const char TCP_PROTOCOL_NAME[];
extern const char SSLTCP_PROTOCOL_NAME[];
extern const char TLS_PROTOCOL_NAME[];

constexpr int kTcpHeaderSize = 20;
constexpr int kUdpHeaderSize = 8;

// Get the transport layer overhead per packet based on the protocol.
int GetProtocolOverhead(absl::string_view protocol);

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::GetProtocolOverhead;
using ::webrtc::kTcpHeaderSize;
using ::webrtc::kUdpHeaderSize;
using ::webrtc::SSLTCP_PROTOCOL_NAME;
using ::webrtc::TCP_PROTOCOL_NAME;
using ::webrtc::TLS_PROTOCOL_NAME;
using ::webrtc::UDP_PROTOCOL_NAME;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // RTC_BASE_NET_HELPER_H_
