/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_SVC_SVC_RATE_ALLOCATOR_H_
#define MODULES_VIDEO_CODING_SVC_SVC_RATE_ALLOCATOR_H_

#include <stddef.h>

#include <vector>

#include "absl/container/inlined_vector.h"
#include "api/field_trials_view.h"
#include "api/units/data_rate.h"
#include "api/video/video_bitrate_allocation.h"
#include "api/video/video_bitrate_allocator.h"
#include "api/video/video_codec_constants.h"
#include "api/video_codecs/video_codec.h"
#include "rtc_base/experiments/stable_target_rate_experiment.h"

namespace webrtc {

class SvcRateAllocator : public VideoBitrateAllocator {
 public:
  SvcRateAllocator(const VideoCodec& codec,
                   const FieldTrialsView& field_trials);

  VideoBitrateAllocation Allocate(
      VideoBitrateAllocationParameters parameters) override;

  static DataRate GetMaxBitrate(const VideoCodec& codec);
  static DataRate GetPaddingBitrate(const VideoCodec& codec);
  static absl::InlinedVector<DataRate, kMaxSpatialLayers> GetLayerStartBitrates(
      const VideoCodec& codec);

 private:
  struct NumLayers {
    size_t spatial = 1;
    size_t temporal = 1;
  };

  static NumLayers GetNumLayers(const VideoCodec& codec);
  std::vector<DataRate> DistributeAllocationToSpatialLayersNormalVideo(
      DataRate total_bitrate,
      size_t first_active_layer,
      size_t num_spatial_layers) const;

  std::vector<DataRate> DistributeAllocationToSpatialLayersScreenSharing(
      DataRate total_bitrate,
      size_t first_active_layer,
      size_t num_spatial_layers) const;

  // Returns the number of layers that are active and have enough bitrate to
  // actually be enabled.
  size_t FindNumEnabledLayers(DataRate target_rate) const;

  const VideoCodec codec_;
  const NumLayers num_layers_;
  const StableTargetRateExperiment experiment_settings_;
  const absl::InlinedVector<DataRate, kMaxSpatialLayers>
      cumulative_layer_start_bitrates_;
  size_t last_active_layer_count_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_SVC_RATE_ALLOCATOR_H_
