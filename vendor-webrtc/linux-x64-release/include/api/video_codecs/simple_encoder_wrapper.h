/*
 *  Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_SIMPLE_ENCODER_WRAPPER_H_
#define API_VIDEO_CODECS_SIMPLE_ENCODER_WRAPPER_H_

#include <cstdint>
#include <functional>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/strings/string_view.h"
#include "api/scoped_refptr.h"
#include "api/transport/rtp/dependency_descriptor.h"
#include "api/units/timestamp.h"
#include "api/video/video_frame_buffer.h"
#include "api/video_codecs/video_encoder_factory_interface.h"
#include "api/video_codecs/video_encoder_interface.h"
#include "common_video/generic_frame_descriptor/generic_frame_info.h"
#include "modules/video_coding/svc/scalable_video_controller.h"

namespace webrtc {
class SimpleEncoderWrapper {
 public:
  struct EncodeResult {
    bool oh_no = false;
    std::vector<uint8_t> bitstream_data;
    FrameType frame_type;
    GenericFrameInfo generic_frame_info;
    std::optional<FrameDependencyStructure> dependency_structure;
  };

  using EncodeResultCallback = std::function<void(const EncodeResult& result)>;

  static std::vector<std::string> SupportedWebrtcSvcModes(
      const VideoEncoderFactoryInterface::Capabilities::PredictionConstraints&
          prediction_constraints);

  static std::unique_ptr<SimpleEncoderWrapper> Create(
      std::unique_ptr<VideoEncoderInterface> encoder,
      absl::string_view scalability_mode);

  // Should be private, use the Create function instead.
  SimpleEncoderWrapper(std::unique_ptr<VideoEncoderInterface> encoder,
                       std::unique_ptr<ScalableVideoController> svc_controller);

  // We should really only support CBR, but then we have to think about layer
  // allocations... eh... For this PoC just use CQP.
  void SetEncodeQp(int qp);

  void SetEncodeFps(int fps);

  void Encode(scoped_refptr<webrtc::VideoFrameBuffer> frame_buffer,
              bool force_keyframe,
              EncodeResultCallback callback);

 private:
  std::unique_ptr<VideoEncoderInterface> encoder_;
  std::unique_ptr<ScalableVideoController> svc_controller_;
  ScalableVideoController::StreamLayersConfig layer_configs_;
  int target_qp_ = 0;
  int fps_ = 0;
  Timestamp presentation_timestamp_ = Timestamp::Zero();
};

}  // namespace webrtc
#endif  // API_VIDEO_CODECS_SIMPLE_ENCODER_WRAPPER_H_
