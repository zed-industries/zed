/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_SATURATION_PROTECTOR_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_SATURATION_PROTECTOR_BUFFER_H_

#include <array>
#include <optional>

#include "modules/audio_processing/agc2/agc2_common.h"

namespace webrtc {

// Ring buffer for the saturation protector which only supports (i) push back
// and (ii) read oldest item.
class SaturationProtectorBuffer {
 public:
  SaturationProtectorBuffer();
  ~SaturationProtectorBuffer();

  bool operator==(const SaturationProtectorBuffer& b) const;
  inline bool operator!=(const SaturationProtectorBuffer& b) const {
    return !(*this == b);
  }

  // Maximum number of values that the buffer can contain.
  int Capacity() const;

  // Number of values in the buffer.
  int Size() const;

  void Reset();

  // Pushes back `v`. If the buffer is full, the oldest value is replaced.
  void PushBack(float v);

  // Returns the oldest item in the buffer. Returns an empty value if the
  // buffer is empty.
  std::optional<float> Front() const;

 private:
  int FrontIndex() const;
  // `buffer_` has `size_` elements (up to the size of `buffer_`) and `next_` is
  // the position where the next new value is written in `buffer_`.
  std::array<float, kSaturationProtectorBufferSize> buffer_;
  int next_ = 0;
  int size_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_SATURATION_PROTECTOR_BUFFER_H_
