/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_INTERFACE_H_
#define API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_INTERFACE_H_

#include <map>
#include <memory>
#include <string>
#include <utility>
#include <variant>
#include <vector>

#include "api/units/time_delta.h"
#include "api/video/resolution.h"
#include "api/video/video_frame_buffer.h"
#include "api/video_codecs/video_encoder_interface.h"
#include "api/video_codecs/video_encoding_general.h"
#include "rtc_base/numerics/rational.h"

namespace webrtc {
using FrameType = VideoEncoderInterface::FrameType;

// NOTE: This class is still under development and may change without notice.
class VideoEncoderFactoryInterface {
 public:
  enum class RateControlMode { kCqp, kCbr };

  struct Capabilities {
    struct PredictionConstraints {
      enum class BufferSpaceType {
        kMultiInstance,  // multiple independent sets of buffers
        kMultiKeyframe,  // single set of buffers, but can store multiple
                         // keyframes simultaneously.
        kSingleKeyframe  // single set of buffers, can only store one keyframe
                         // at a time.
      };

      int num_buffers;
      int max_references;
      int max_temporal_layers;

      BufferSpaceType buffer_space_type;
      int max_spatial_layers;
      std::vector<Rational> scaling_factors;

      std::vector<FrameType> supported_frame_types;
    } prediction_constraints;

    struct InputConstraints {
      Resolution min;
      Resolution max;
      int pixel_alignment;
      std::vector<VideoFrameBuffer::Type> input_formats;
    } input_constraints;

    std::vector<EncodingFormat> encoding_formats;

    struct BitrateControl {
      std::pair<int, int> qp_range;
      std::vector<RateControlMode> rc_modes;
    } rate_control;

    struct Performance {
      bool encode_on_calling_thread;
      std::pair<int, int> min_max_effort_level;
    } performance;
  };

  struct StaticEncoderSettings {
    struct Cqp {};
    struct Cbr {
      // TD: Should there be an intial buffer size?
      TimeDelta max_buffer_size;
      TimeDelta target_buffer_size;
    };

    Resolution max_encode_dimensions;
    EncodingFormat encoding_format;
    std::variant<Cqp, Cbr> rc_mode;
    int max_number_of_threads;
  };

  virtual ~VideoEncoderFactoryInterface() = default;

  virtual std::string CodecName() const = 0;
  virtual std::string ImplementationName() const = 0;
  virtual std::map<std::string, std::string> CodecSpecifics() const = 0;

  virtual Capabilities GetEncoderCapabilities() const = 0;
  virtual std::unique_ptr<VideoEncoderInterface> CreateEncoder(
      const StaticEncoderSettings& settings,
      const std::map<std::string, std::string>& encoder_specific_settings) = 0;
};

}  // namespace webrtc
#endif  // API_VIDEO_CODECS_VIDEO_ENCODER_FACTORY_INTERFACE_H_
