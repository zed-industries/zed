/*
 *  Copyright 2019 The WebRTC Project Authors. All rights reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef COMMON_VIDEO_INCLUDE_QUALITY_LIMITATION_REASON_H_
#define COMMON_VIDEO_INCLUDE_QUALITY_LIMITATION_REASON_H_

namespace webrtc {

// https://w3c.github.io/webrtc-stats/#rtcqualitylimitationreason-enum
enum class QualityLimitationReason {
  kNone,
  kCpu,
  kBandwidth,
  kOther,
};

}  // namespace webrtc

#endif  // COMMON_VIDEO_INCLUDE_QUALITY_LIMITATION_REASON_H_
