/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_DIFFER_BLOCK_H_
#define MODULES_DESKTOP_CAPTURE_DIFFER_BLOCK_H_

#include <stdint.h>

namespace webrtc {

// Size (in pixels) of each square block used for diffing. This must be a
// multiple of sizeof(uint64)/8.
const int kBlockSize = 32;

// Format: BGRA 32 bit.
const int kBytesPerPixel = 4;

// Low level function to compare 2 vectors of pixels of size kBlockSize. Returns
// whether the blocks differ.
bool VectorDifference(const uint8_t* image1, const uint8_t* image2);

// Low level function to compare 2 blocks of pixels of size
// (kBlockSize, `height`).  Returns whether the blocks differ.
bool BlockDifference(const uint8_t* image1,
                     const uint8_t* image2,
                     int height,
                     int stride);

// Low level function to compare 2 blocks of pixels of size
// (kBlockSize, kBlockSize).  Returns whether the blocks differ.
bool BlockDifference(const uint8_t* image1, const uint8_t* image2, int stride);

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_DIFFER_BLOCK_H_
