/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_HIGH_PASS_FILTER_H_
#define MODULES_AUDIO_PROCESSING_HIGH_PASS_FILTER_H_

#include <memory>
#include <vector>

#include "api/array_view.h"
#include "modules/audio_processing/utility/cascaded_biquad_filter.h"

namespace webrtc {

class AudioBuffer;

class HighPassFilter {
 public:
  HighPassFilter(int sample_rate_hz, size_t num_channels);
  ~HighPassFilter();
  HighPassFilter(const HighPassFilter&) = delete;
  HighPassFilter& operator=(const HighPassFilter&) = delete;

  void Process(AudioBuffer* audio, bool use_split_band_data);
  void Process(std::vector<std::vector<float>>* audio);
  void Reset();
  void Reset(size_t num_channels);

  int sample_rate_hz() const { return sample_rate_hz_; }
  size_t num_channels() const { return filters_.size(); }

 private:
  const int sample_rate_hz_;
  std::vector<std::unique_ptr<CascadedBiQuadFilter>> filters_;
};
}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_HIGH_PASS_FILTER_H_
