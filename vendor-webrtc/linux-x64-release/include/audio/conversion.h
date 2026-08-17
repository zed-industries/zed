/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef AUDIO_CONVERSION_H_
#define AUDIO_CONVERSION_H_

#include <stddef.h>
#include <stdint.h>

namespace webrtc {

// Convert fixed point number with 8 bit fractional part, to floating point.
inline float Q8ToFloat(uint32_t v) {
  return static_cast<float>(v) / (1 << 8);
}

// Convert fixed point number with 14 bit fractional part, to floating point.
inline float Q14ToFloat(uint32_t v) {
  return static_cast<float>(v) / (1 << 14);
}
}  // namespace webrtc

#endif  // AUDIO_CONVERSION_H_
