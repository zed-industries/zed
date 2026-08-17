/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_DELAY_ESTIMATE_H_
#define MODULES_AUDIO_PROCESSING_AEC3_DELAY_ESTIMATE_H_

#include <stddef.h>

namespace webrtc {

// Stores delay_estimates.
struct DelayEstimate {
  enum class Quality { kCoarse, kRefined };

  DelayEstimate(Quality quality, size_t delay)
      : quality(quality), delay(delay) {}

  Quality quality;
  size_t delay;
  size_t blocks_since_last_change = 0;
  size_t blocks_since_last_update = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_DELAY_ESTIMATE_H_
