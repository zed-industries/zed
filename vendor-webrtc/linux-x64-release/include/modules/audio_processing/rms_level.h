/*
 *  Copyright (c) 2014 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_RMS_LEVEL_H_
#define MODULES_AUDIO_PROCESSING_RMS_LEVEL_H_

#include <stddef.h>
#include <stdint.h>

#include <optional>

#include "api/array_view.h"

namespace webrtc {

// Computes the root mean square (RMS) level in dBFs (decibels from digital
// full-scale) of audio data. The computation follows RFC 6465:
// https://tools.ietf.org/html/rfc6465
// with the intent that it can provide the RTP audio level indication.
//
// The expected approach is to provide constant-sized chunks of audio to
// Analyze(). When enough chunks have been accumulated to form a packet, call
// Average() to get the audio level indicator for the RTP header.
class RmsLevel {
 public:
  struct Levels {
    int average;
    int peak;
  };

  enum : int { kMinLevelDb = 127, kInaudibleButNotMuted = 126 };

  RmsLevel();
  ~RmsLevel();

  // Can be called to reset internal states, but is not required during normal
  // operation.
  void Reset();

  // Pass each chunk of audio to Analyze() to accumulate the level.
  void Analyze(ArrayView<const int16_t> data);
  void Analyze(ArrayView<const float> data);

  // If all samples with the given `length` have a magnitude of zero, this is
  // a shortcut to avoid some computation.
  void AnalyzeMuted(size_t length);

  // Computes the RMS level over all data passed to Analyze() since the last
  // call to Average(). The returned value is positive but should be interpreted
  // as negative as per the RFC. It is constrained to [0, 127]. Resets the
  // internal state to start a new measurement period.
  int Average();

  // Like Average() above, but also returns the RMS peak value. Resets the
  // internal state to start a new measurement period.
  Levels AverageAndPeak();

 private:
  // Compares `block_size` with `block_size_`. If they are different, calls
  // Reset() and stores the new size.
  void CheckBlockSize(size_t block_size);

  float sum_square_;
  size_t sample_count_;
  float max_sum_square_;
  std::optional<size_t> block_size_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_RMS_LEVEL_H_
