/*
 *  Copyright 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_PRIORITY_H_
#define API_PRIORITY_H_

#include <stdint.h>

#include "rtc_base/checks.h"
#include "rtc_base/strong_alias.h"

namespace webrtc {

// GENERATED_JAVA_ENUM_PACKAGE: org.webrtc
enum class Priority {
  kVeryLow,
  kLow,
  kMedium,
  kHigh,
};

class PriorityValue
    : public webrtc::StrongAlias<class PriorityValueTag, uint16_t> {
 public:
  explicit PriorityValue(Priority priority) {
    switch (priority) {
      case Priority::kVeryLow:
        value_ = 128;
        break;
      case Priority::kLow:
        value_ = 256;
        break;
      case Priority::kMedium:
        value_ = 512;
        break;
      case Priority::kHigh:
        value_ = 1024;
        break;
      default:
        RTC_CHECK_NOTREACHED();
    }
  }

  explicit PriorityValue(uint16_t priority) : StrongAlias(priority) {}
};

}  // namespace webrtc

#endif  // API_PRIORITY_H_
