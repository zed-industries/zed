/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_AUDIO_PROCESSING_AEC3_SUBTRACTOR_OUTPUT_ANALYZER_H_
#define MODULES_AUDIO_PROCESSING_AEC3_SUBTRACTOR_OUTPUT_ANALYZER_H_

#include <vector>

#include "modules/audio_processing/aec3/subtractor_output.h"

namespace webrtc {

// Class for analyzing the properties subtractor output.
class SubtractorOutputAnalyzer {
 public:
  explicit SubtractorOutputAnalyzer(size_t num_capture_channels);
  ~SubtractorOutputAnalyzer() = default;

  // Analyses the subtractor output.
  void Update(ArrayView<const SubtractorOutput> subtractor_output,
              bool* any_filter_converged,
              bool* any_coarse_filter_converged,
              bool* all_filters_diverged);

  const std::vector<bool>& ConvergedFilters() const {
    return filters_converged_;
  }

  // Handle echo path change.
  void HandleEchoPathChange();

 private:
  std::vector<bool> filters_converged_;
};

}  // namespace webrtc

#endif  // MODULES_AUDIO_PROCESSING_AEC3_SUBTRACTOR_OUTPUT_ANALYZER_H_
