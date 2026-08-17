/*
 *  Copyright (c) 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_SVC_SCALABILITY_STRUCTURE_SIMULCAST_H_
#define MODULES_VIDEO_CODING_SVC_SCALABILITY_STRUCTURE_SIMULCAST_H_

#include <bitset>
#include <vector>

#include "api/transport/rtp/dependency_descriptor.h"
#include "api/video/video_bitrate_allocation.h"
#include "common_video/generic_frame_descriptor/generic_frame_info.h"
#include "modules/video_coding/svc/scalable_video_controller.h"

namespace webrtc {

// Scalability structure with multiple independent spatial layers each with the
// same temporal layering.
class ScalabilityStructureSimulcast : public ScalableVideoController {
 public:
  struct ScalingFactor {
    int num = 1;
    int den = 2;
  };
  ScalabilityStructureSimulcast(int num_spatial_layers,
                                int num_temporal_layers,
                                ScalingFactor resolution_factor);
  ~ScalabilityStructureSimulcast() override;

  StreamLayersConfig StreamConfig() const override;
  std::vector<LayerFrameConfig> NextFrameConfig(bool restart) override;
  GenericFrameInfo OnEncodeDone(const LayerFrameConfig& config) override;
  void OnRatesUpdated(const VideoBitrateAllocation& bitrates) override;

 private:
  enum FramePattern {
    kNone,
    kDeltaT2A,
    kDeltaT1,
    kDeltaT2B,
    kDeltaT0,
  };
  static constexpr int kMaxNumSpatialLayers = 3;
  static constexpr int kMaxNumTemporalLayers = 3;

  // Index of the buffer to store last frame for layer (`sid`, `tid`)
  int BufferIndex(int sid, int tid) const {
    return tid * num_spatial_layers_ + sid;
  }
  bool DecodeTargetIsActive(int sid, int tid) const {
    return active_decode_targets_[sid * num_temporal_layers_ + tid];
  }
  void SetDecodeTargetIsActive(int sid, int tid, bool value) {
    active_decode_targets_.set(sid * num_temporal_layers_ + tid, value);
  }
  FramePattern NextPattern() const;
  bool TemporalLayerIsActive(int tid) const;

  const int num_spatial_layers_;
  const int num_temporal_layers_;
  const ScalingFactor resolution_factor_;

  FramePattern last_pattern_ = kNone;
  std::bitset<kMaxNumSpatialLayers> can_reference_t0_frame_for_spatial_id_ = 0;
  std::bitset<kMaxNumSpatialLayers> can_reference_t1_frame_for_spatial_id_ = 0;
  std::bitset<32> active_decode_targets_;
};

// S1  0--0--0-
//             ...
// S0  0--0--0-
class ScalabilityStructureS2T1 : public ScalabilityStructureSimulcast {
 public:
  explicit ScalabilityStructureS2T1(ScalingFactor resolution_factor = {})
      : ScalabilityStructureSimulcast(2, 1, resolution_factor) {}
  ~ScalabilityStructureS2T1() override = default;

  FrameDependencyStructure DependencyStructure() const override;
};

class ScalabilityStructureS2T2 : public ScalabilityStructureSimulcast {
 public:
  explicit ScalabilityStructureS2T2(ScalingFactor resolution_factor = {})
      : ScalabilityStructureSimulcast(2, 2, resolution_factor) {}
  ~ScalabilityStructureS2T2() override = default;

  FrameDependencyStructure DependencyStructure() const override;
};

// S1T2       3   7
//            |  /
// S1T1       / 5
//           |_/
// S1T0     1-------9...
//
// S0T2       2   6
//            |  /
// S0T1       / 4
//           |_/
// S0T0     0-------8...
// Time->   0 1 2 3 4
class ScalabilityStructureS2T3 : public ScalabilityStructureSimulcast {
 public:
  explicit ScalabilityStructureS2T3(ScalingFactor resolution_factor = {})
      : ScalabilityStructureSimulcast(2, 3, resolution_factor) {}
  ~ScalabilityStructureS2T3() override = default;

  FrameDependencyStructure DependencyStructure() const override;
};

class ScalabilityStructureS3T1 : public ScalabilityStructureSimulcast {
 public:
  explicit ScalabilityStructureS3T1(ScalingFactor resolution_factor = {})
      : ScalabilityStructureSimulcast(3, 1, resolution_factor) {}
  ~ScalabilityStructureS3T1() override = default;

  FrameDependencyStructure DependencyStructure() const override;
};

class ScalabilityStructureS3T2 : public ScalabilityStructureSimulcast {
 public:
  explicit ScalabilityStructureS3T2(ScalingFactor resolution_factor = {})
      : ScalabilityStructureSimulcast(3, 2, resolution_factor) {}
  ~ScalabilityStructureS3T2() override = default;

  FrameDependencyStructure DependencyStructure() const override;
};

class ScalabilityStructureS3T3 : public ScalabilityStructureSimulcast {
 public:
  explicit ScalabilityStructureS3T3(ScalingFactor resolution_factor = {})
      : ScalabilityStructureSimulcast(3, 3, resolution_factor) {}
  ~ScalabilityStructureS3T3() override = default;

  FrameDependencyStructure DependencyStructure() const override;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_SCALABILITY_STRUCTURE_SIMULCAST_H_
