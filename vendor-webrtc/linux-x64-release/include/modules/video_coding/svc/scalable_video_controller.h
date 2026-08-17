/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef MODULES_VIDEO_CODING_SVC_SCALABLE_VIDEO_CONTROLLER_H_
#define MODULES_VIDEO_CODING_SVC_SCALABLE_VIDEO_CONTROLLER_H_

#include <vector>

#include "absl/container/inlined_vector.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/video/video_bitrate_allocation.h"
#include "api/video/video_codec_constants.h"
#include "common_video/generic_frame_descriptor/generic_frame_info.h"

namespace webrtc {

// Controls how video should be encoded to be scalable. Outputs results as
// buffer usage configuration for encoder and enough details to communicate the
// scalability structure via dependency descriptor rtp header extension.
class ScalableVideoController {
 public:
  struct StreamLayersConfig {
    int num_spatial_layers = 1;
    int num_temporal_layers = 1;
    // Indicates if frames can reference frames of a different resolution.
    bool uses_reference_scaling = true;
    // Spatial layers scaling. Frames with spatial_id = i expected to be encoded
    // with original_resolution * scaling_factor_num[i] / scaling_factor_den[i].
    int scaling_factor_num[DependencyDescriptor::kMaxSpatialIds] = {1, 1, 1, 1};
    int scaling_factor_den[DependencyDescriptor::kMaxSpatialIds] = {1, 1, 1, 1};
  };
  class LayerFrameConfig {
   public:
    // Builders/setters.
    LayerFrameConfig& Id(int value);
    LayerFrameConfig& Keyframe();
    LayerFrameConfig& S(int value);
    LayerFrameConfig& T(int value);
    LayerFrameConfig& Reference(int buffer_id);
    LayerFrameConfig& Update(int buffer_id);
    LayerFrameConfig& ReferenceAndUpdate(int buffer_id);

    // Getters.
    int Id() const { return id_; }
    bool IsKeyframe() const { return is_keyframe_; }
    int SpatialId() const { return spatial_id_; }
    int TemporalId() const { return temporal_id_; }
    const absl::InlinedVector<CodecBufferUsage, kMaxEncoderBuffers>& Buffers()
        const {
      return buffers_;
    }

   private:
    // Id to match configuration returned by NextFrameConfig with
    // (possibly modified) configuration passed back via OnEncoderDone.
    // The meaning of the id is an implementation detail of
    // the ScalableVideoController.
    int id_ = 0;

    // Indication frame should be encoded as a key frame. In particular when
    // `is_keyframe=true` property `CodecBufferUsage::referenced` should be
    // ignored and treated as false.
    bool is_keyframe_ = false;

    int spatial_id_ = 0;
    int temporal_id_ = 0;
    // Describes how encoder which buffers encoder allowed to reference and
    // which buffers encoder should update.
    absl::InlinedVector<CodecBufferUsage, kMaxEncoderBuffers> buffers_;
  };

  virtual ~ScalableVideoController() = default;

  // Returns video structure description for encoder to configure itself.
  virtual StreamLayersConfig StreamConfig() const = 0;

  // Returns video structure description in format compatible with
  // dependency descriptor rtp header extension.
  virtual FrameDependencyStructure DependencyStructure() const = 0;

  // Notifies Controller with updated bitrates per layer. In particular notifies
  // when certain layers should be disabled.
  // Controller shouldn't produce LayerFrameConfig for disabled layers.
  virtual void OnRatesUpdated(const VideoBitrateAllocation& bitrates) = 0;

  // When `restart` is true, first `LayerFrameConfig` should have `is_keyframe`
  // set to true.
  // Returned vector shouldn't be empty.
  virtual std::vector<LayerFrameConfig> NextFrameConfig(bool restart) = 0;

  // Returns configuration to pass to EncoderCallback.
  virtual GenericFrameInfo OnEncodeDone(const LayerFrameConfig& config) = 0;
};

// Below are implementation details.
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::Id(int value) {
  id_ = value;
  return *this;
}
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::Keyframe() {
  is_keyframe_ = true;
  return *this;
}
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::S(int value) {
  spatial_id_ = value;
  return *this;
}
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::T(int value) {
  temporal_id_ = value;
  return *this;
}
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::Reference(int buffer_id) {
  buffers_.emplace_back(buffer_id, /*referenced=*/true, /*updated=*/false);
  return *this;
}
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::Update(int buffer_id) {
  buffers_.emplace_back(buffer_id, /*referenced=*/false, /*updated=*/true);
  return *this;
}
inline ScalableVideoController::LayerFrameConfig&
ScalableVideoController::LayerFrameConfig::ReferenceAndUpdate(int buffer_id) {
  buffers_.emplace_back(buffer_id, /*referenced=*/true, /*updated=*/true);
  return *this;
}

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_SCALABLE_VIDEO_CONTROLLER_H_
