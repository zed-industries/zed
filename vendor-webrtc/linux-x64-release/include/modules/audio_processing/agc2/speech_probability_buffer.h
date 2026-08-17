/*
 *  Copyright (c) 2022 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AGC2_SPEECH_PROBABILITY_BUFFER_H_
#define MODULES_AUDIO_PROCESSING_AGC2_SPEECH_PROBABILITY_BUFFER_H_

#include <vector>

#include "rtc_base/gtest_prod_util.h"

namespace webrtc {

// This class implements a circular buffer that stores speech probabilities
// for a speech segment and estimates speech activity for that segment.
class SpeechProbabilityBuffer {
 public:
  // Ctor. The value of `low_probability_threshold` is required to be on the
  // range [0.0f, 1.0f].
  explicit SpeechProbabilityBuffer(float low_probability_threshold);
  ~SpeechProbabilityBuffer() {}
  SpeechProbabilityBuffer(const SpeechProbabilityBuffer&) = delete;
  SpeechProbabilityBuffer& operator=(const SpeechProbabilityBuffer&) = delete;

  // Adds `probability` in the buffer and computes an updatds sum of the buffer
  // probabilities. Value of `probability` is required to be on the range
  // [0.0f, 1.0f].
  void Update(float probability);

  // Resets the histogram, forgets the past.
  void Reset();

  // Returns true if the segment is active (a long enough segment with an
  // average speech probability above `low_probability_threshold`).
  bool IsActiveSegment() const;

 private:
  void RemoveTransient();

  // Use only for testing.
  float GetSumProbabilities() const { return sum_probabilities_; }

  FRIEND_TEST_ALL_PREFIXES(SpeechProbabilityBufferTest,
                           CheckSumAfterInitialization);
  FRIEND_TEST_ALL_PREFIXES(SpeechProbabilityBufferTest, CheckSumAfterUpdate);
  FRIEND_TEST_ALL_PREFIXES(SpeechProbabilityBufferTest, CheckSumAfterReset);
  FRIEND_TEST_ALL_PREFIXES(SpeechProbabilityBufferTest,
                           CheckSumAfterTransientNotRemoved);
  FRIEND_TEST_ALL_PREFIXES(SpeechProbabilityBufferTest,
                           CheckSumAfterTransientRemoved);

  const float low_probability_threshold_;

  // Sum of probabilities stored in `probabilities_`. Must be updated if
  // `probabilities_` is updated.
  float sum_probabilities_ = 0.0f;

  // Circular buffer for probabilities.
  std::vector<float> probabilities_;

  // Current index of the circular buffer, where the newest data will be written
  // to, therefore, pointing to the oldest data if buffer is full.
  int buffer_index_ = 0;

  // Indicates if the buffer is full and adding a new value removes the oldest
  // value.
  int buffer_is_full_ = false;

  int num_high_probability_observations_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AGC2_SPEECH_PROBABILITY_BUFFER_H_
