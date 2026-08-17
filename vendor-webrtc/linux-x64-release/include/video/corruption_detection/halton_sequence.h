/*
 * Copyright 2024 The WebRTC project authors. All rights reserved.
 *
 * Use of this source code is governed by a BSD-style license
 * that can be found in the LICENSE file in the root of the source
 * tree. An additional intellectual property rights grant can be found
 * in the file PATENTS.  All contributing project authors may
 * be found in the AUTHORS file in the root of the source tree.
 */

#ifndef VIDEO_CORRUPTION_DETECTION_HALTON_SEQUENCE_H_
#define VIDEO_CORRUPTION_DETECTION_HALTON_SEQUENCE_H_

#include <vector>

namespace webrtc {

// Generates the Halton sequence: a low discrepancy sequence of doubles in the
// half-open interval [0,1). See https://en.wikipedia.org/wiki/Halton_sequence
// for information on how the sequence is constructed.
class HaltonSequence {
 public:
  // Creates a sequence in `num_dimensions` number of dimensions. Possible
  // values are [1, 5].
  explicit HaltonSequence(int num_dimensions);
  // Creates a default sequence in a single dimension.
  HaltonSequence() = default;
  HaltonSequence(const HaltonSequence&) = default;
  HaltonSequence(HaltonSequence&&) = default;
  HaltonSequence& operator=(const HaltonSequence&) = default;
  HaltonSequence& operator=(HaltonSequence&&) = default;
  ~HaltonSequence() = default;

  // Gets the next point in the sequence where each value is in the half-open
  // interval [0,1).
  std::vector<double> GetNext();
  int GetCurrentIndex() const { return current_idx_; }
  void SetCurrentIndex(int idx);
  void Reset();

 private:
  int num_dimensions_ = 1;
  int current_idx_ = 0;
};

}  // namespace webrtc

#endif  // VIDEO_CORRUPTION_DETECTION_HALTON_SEQUENCE_H_
