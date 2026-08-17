/*
 *  Copyright (c) 2015 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_UTILITY_VP8_HEADER_PARSER_H_
#define MODULES_VIDEO_CODING_UTILITY_VP8_HEADER_PARSER_H_

#include <stdint.h>
#include <stdio.h>

namespace webrtc {

namespace vp8 {

typedef struct VP8BitReader VP8BitReader;
struct VP8BitReader {
  // Boolean decoder.
  uint32_t value_;  // Current value (2 bytes).
  uint32_t range_;  // Current range (always in [128..255] interval).
  int bits_;        // Number of bits shifted out of value, at most 7.
  // Read buffer.
  const uint8_t* buf_;      // Next byte to be read.
  const uint8_t* buf_end_;  // End of read buffer.
};

// Gets the QP, QP range: [0, 127].
// Returns true on success, false otherwise.
bool GetQp(const uint8_t* buf, size_t length, int* qp);

}  // namespace vp8

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_UTILITY_VP8_HEADER_PARSER_H_
