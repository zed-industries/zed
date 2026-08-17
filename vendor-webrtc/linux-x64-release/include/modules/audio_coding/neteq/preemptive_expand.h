/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_CODING_NETEQ_PREEMPTIVE_EXPAND_H_
#define MODULES_AUDIO_CODING_NETEQ_PREEMPTIVE_EXPAND_H_

#include <stddef.h>
#include <stdint.h>

#include "modules/audio_coding/neteq/time_stretch.h"

namespace webrtc {

class AudioMultiVector;
class BackgroundNoise;

// This class implements the PreemptiveExpand operation. Most of the work is
// done in the base class TimeStretch, which is shared with the Accelerate
// operation. In the PreemptiveExpand class, the operations that are specific to
// PreemptiveExpand are implemented.
class PreemptiveExpand : public TimeStretch {
 public:
  PreemptiveExpand(int sample_rate_hz,
                   size_t num_channels,
                   const BackgroundNoise& background_noise,
                   size_t overlap_samples)
      : TimeStretch(sample_rate_hz, num_channels, background_noise),
        old_data_length_per_channel_(0),
        overlap_samples_(overlap_samples) {}

  PreemptiveExpand(const PreemptiveExpand&) = delete;
  PreemptiveExpand& operator=(const PreemptiveExpand&) = delete;

  // This method performs the actual PreemptiveExpand operation. The samples are
  // read from `input`, of length `input_length` elements, and are written to
  // `output`. The number of samples added through time-stretching is
  // is provided in the output `length_change_samples`. The method returns
  // the outcome of the operation as an enumerator value.
  ReturnCodes Process(const int16_t* pw16_decoded,
                      size_t len,
                      size_t old_data_len,
                      AudioMultiVector* output,
                      size_t* length_change_samples);

 protected:
  // Sets the parameters `best_correlation` and `peak_index` to suitable
  // values when the signal contains no active speech.
  void SetParametersForPassiveSpeech(size_t input_length,
                                     int16_t* best_correlation,
                                     size_t* peak_index) const override;

  // Checks the criteria for performing the time-stretching operation and,
  // if possible, performs the time-stretching.
  ReturnCodes CheckCriteriaAndStretch(const int16_t* input,
                                      size_t input_length,
                                      size_t peak_index,
                                      int16_t best_correlation,
                                      bool active_speech,
                                      bool /*fast_mode*/,
                                      AudioMultiVector* output) const override;

 private:
  size_t old_data_length_per_channel_;
  size_t overlap_samples_;
};

struct PreemptiveExpandFactory {
  PreemptiveExpandFactory() {}
  virtual ~PreemptiveExpandFactory() {}

  virtual PreemptiveExpand* Create(int sample_rate_hz,
                                   size_t num_channels,
                                   const BackgroundNoise& background_noise,
                                   size_t overlap_samples) const;
};

}  // namespace webrtc
#endif  // MODULES_AUDIO_CODING_NETEQ_PREEMPTIVE_EXPAND_H_
