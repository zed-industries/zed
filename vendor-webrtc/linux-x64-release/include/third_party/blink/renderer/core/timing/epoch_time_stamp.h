// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_EPOCH_TIME_STAMP_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_EPOCH_TIME_STAMP_H_

#include <stdint.h>

#include "base/time/time.h"

namespace blink {

typedef uint64_t EpochTimeStamp;

inline EpochTimeStamp ConvertTimeToEpochTimeStamp(base::Time time) {
  return static_cast<EpochTimeStamp>(time.InMillisecondsSinceUnixEpoch());
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_TIMING_EPOCH_TIME_STAMP_H_
