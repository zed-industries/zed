/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_VP8_CONSTANTS_H_
#define MODULES_VIDEO_CODING_UTILITY_VP8_CONSTANTS_H_

#include <stddef.h>
#include <stdint.h>


namespace webrtc {

// QP level below which VP8 variable framerate and zero hertz screencast reduces
// framerate due to diminishing quality enhancement returns.
constexpr int kVp8SteadyStateQpThreshold = 15;

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_VP8_CONSTANTS_H_
