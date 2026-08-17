// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_TIME_CLAMPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_TIME_CLAMPER_H_

#include "base/time/time.h"
#include "build/build_config.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

#include <stdint.h>

namespace blink {

class CORE_EXPORT TimeClamper {
  USING_FAST_MALLOC(TimeClamper);

 public:
  static constexpr int kCoarseResolutionMicroseconds = 100;
  static constexpr int kFineResolutionMicroseconds = 5;

  TimeClamper();
  TimeClamper(const TimeClamper&) = delete;
  TimeClamper& operator=(const TimeClamper&) = delete;

  // Deterministically clamp the time value |time_microseconds| to a fixed
  // interval to prevent timing attacks. See
  // http://www.w3.org/TR/hr-time-2/#privacy-security.
  //
  // For each clamped time interval, we compute a pseudorandom transition
  // threshold. The returned time will either be the start of that interval or
  // the next one depending on which side of the threshold |time_microseconds|
  // is.
  base::TimeDelta ClampTimeResolution(
      base::TimeDelta time,
      bool cross_origin_isolated_capability) const;

 private:
  inline double ThresholdFor(int64_t clamped_time, int resolution) const;
  static inline double ToDouble(uint64_t value);
  static inline uint64_t MurmurHash3(uint64_t value);

  uint64_t secret_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_TIME_CLAMPER_H_
