/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_RTP_RTCP_SOURCE_RTP_RTCP_CONFIG_H_
#define MODULES_RTP_RTCP_SOURCE_RTP_RTCP_CONFIG_H_

// Configuration file for RTP utilities (RTPSender, RTPReceiver ...)
namespace webrtc {
constexpr int kDefaultMaxReorderingThreshold = 50;  // In sequence numbers.
constexpr int kRtcpMaxNackFields = 253;

constexpr int RTCP_MAX_REPORT_BLOCKS = 31;  // RFC 3550 page 37
}  // namespace webrtc

#endif  // MODULES_RTP_RTCP_SOURCE_RTP_RTCP_CONFIG_H_
