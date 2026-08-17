/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_BLOCK_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_BLOCK_BUFFER_H_

#include <stddef.h>

#include <vector>

#include "modules/audio_processing/aec3/block.h"
#include "rtc_base/checks.h"

namespace webrtc {

// Struct for bundling a circular buffer of two dimensional vector objects
// together with the read and write indices.
struct BlockBuffer {
  BlockBuffer(size_t size, size_t num_bands, size_t num_channels);
  ~BlockBuffer();

  int IncIndex(int index) const {
    RTC_DCHECK_EQ(buffer.size(), static_cast<size_t>(size));
    return index < size - 1 ? index + 1 : 0;
  }

  int DecIndex(int index) const {
    RTC_DCHECK_EQ(buffer.size(), static_cast<size_t>(size));
    return index > 0 ? index - 1 : size - 1;
  }

  int OffsetIndex(int index, int offset) const {
    RTC_DCHECK_EQ(buffer.size(), static_cast<size_t>(size));
    RTC_DCHECK_GE(size, offset);
    return (size + index + offset) % size;
  }

  void UpdateWriteIndex(int offset) { write = OffsetIndex(write, offset); }
  void IncWriteIndex() { write = IncIndex(write); }
  void DecWriteIndex() { write = DecIndex(write); }
  void UpdateReadIndex(int offset) { read = OffsetIndex(read, offset); }
  void IncReadIndex() { read = IncIndex(read); }
  void DecReadIndex() { read = DecIndex(read); }

  const int size;
  std::vector<Block> buffer;
  int write = 0;
  int read = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_BLOCK_BUFFER_H_
