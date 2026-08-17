/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_SVC_SCALABILITY_STRUCTURE_TEST_HELPERS_H_
#define MODULES_VIDEO_CODING_SVC_SCALABILITY_STRUCTURE_TEST_HELPERS_H_

#include <stdint.h>

#include <vector>

#include "api/array_view.h"
#include "api/video/video_bitrate_allocation.h"
#include "common_video/generic_frame_descriptor/generic_frame_info.h"
#include "modules/video_coding/chain_diff_calculator.h"
#include "modules/video_coding/frame_dependencies_calculator.h"
#include "modules/video_coding/svc/scalable_video_controller.h"

namespace webrtc {

// Creates bitrate allocation with non-zero bitrate for given number of temporal
// layers for each spatial layer.
VideoBitrateAllocation EnableTemporalLayers(int s0, int s1 = 0, int s2 = 0);

class ScalabilityStructureWrapper {
 public:
  explicit ScalabilityStructureWrapper(ScalableVideoController& structure)
      : structure_controller_(structure) {}

  std::vector<GenericFrameInfo> GenerateFrames(int num_temporal_units) {
    std::vector<GenericFrameInfo> frames;
    GenerateFrames(num_temporal_units, frames);
    return frames;
  }
  void GenerateFrames(int num_temporal_units,
                      std::vector<GenericFrameInfo>& frames);

  // Returns false and ADD_FAILUREs for frames with invalid references.
  // In particular validates no frame frame reference to frame before frames[0].
  // In error messages frames are indexed starting with 0.
  bool FrameReferencesAreValid(ArrayView<const GenericFrameInfo> frames) const;

 private:
  ScalableVideoController& structure_controller_;
  FrameDependenciesCalculator frame_deps_calculator_;
  ChainDiffCalculator chain_diff_calculator_;
  int64_t frame_id_ = 0;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_SCALABILITY_STRUCTURE_TEST_HELPERS_H_
