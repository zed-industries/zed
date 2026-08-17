// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.
#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_PERFORMANCE_PERFORMANCE_TIMELINE_CONSTANTS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_PERFORMANCE_PERFORMANCE_TIMELINE_CONSTANTS_H_

#include "base/time/time.h"

namespace blink {

inline constexpr uint32_t kSoftNavigationCountDefaultValue = 0;

struct SoftNavigationMetrics {
  uint32_t count = kSoftNavigationCountDefaultValue;
  base::TimeDelta start_time;
  std::string navigation_id;
};
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_PERFORMANCE_PERFORMANCE_TIMELINE_CONSTANTS_H_
