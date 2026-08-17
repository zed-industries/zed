/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_TEST_ECHO_CANCELLER_TEST_TOOLS_H_
#define MODULES_AUDIO_PROCESSING_TEST_ECHO_CANCELLER_TEST_TOOLS_H_

#include <algorithm>
#include <vector>

#include "api/array_view.h"
#include "rtc_base/random.h"

namespace webrtc {

// Randomizes the elements in a vector with values -32767.f:32767.f.
void RandomizeSampleVector(Random* random_generator, ArrayView<float> v);

// Randomizes the elements in a vector with values -amplitude:amplitude.
void RandomizeSampleVector(Random* random_generator,
                           ArrayView<float> v,
                           float amplitude);

// Class for delaying a signal a fixed number of samples.
template <typename T>
class DelayBuffer {
 public:
  explicit DelayBuffer(size_t delay) : buffer_(delay) {}
  ~DelayBuffer() = default;

  // Produces a delayed signal copy of x.
  void Delay(ArrayView<const T> x, ArrayView<T> x_delayed);

 private:
  std::vector<T> buffer_;
  size_t next_insert_index_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_TEST_ECHO_CANCELLER_TEST_TOOLS_H_
