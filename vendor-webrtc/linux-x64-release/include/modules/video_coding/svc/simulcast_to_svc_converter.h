/* Copyright (c) 2024 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_SVC_SIMULCAST_TO_SVC_CONVERTER_H_
#define MODULES_VIDEO_CODING_SVC_SIMULCAST_TO_SVC_CONVERTER_H_

#include <stddef.h>

#include <memory>
#include <vector>

#include "api/video/encoded_image.h"
#include "api/video_codecs/scalability_mode.h"
#include "api/video_codecs/video_codec.h"
#include "modules/video_coding/include/video_codec_interface.h"
#include "modules/video_coding/svc/scalable_video_controller.h"
#include "rtc_base/system/rtc_export.h"

namespace webrtc {

class RTC_EXPORT SimulcastToSvcConverter {
 public:
  explicit SimulcastToSvcConverter(const VideoCodec&);
  SimulcastToSvcConverter(SimulcastToSvcConverter&&) = default;

  SimulcastToSvcConverter(const SimulcastToSvcConverter&) = delete;
  SimulcastToSvcConverter& operator=(const SimulcastToSvcConverter&) = delete;
  SimulcastToSvcConverter& operator=(SimulcastToSvcConverter&&) = default;

  ~SimulcastToSvcConverter() = default;

  static bool IsConfigSupported(const VideoCodec& codec);

  VideoCodec GetConfig() const;

  void EncodeStarted(bool force_keyframe);

  bool ConvertFrame(EncodedImage& encoded_image,
                    CodecSpecificInfo& codec_specific);

 private:
  struct LayerState {
    LayerState(ScalabilityMode scalability_mode, int num_temporal_layers);
    ~LayerState() = default;
    LayerState(const LayerState&) = delete;
    LayerState(LayerState&&) = default;

    std::unique_ptr<ScalableVideoController> video_controller;
    ScalableVideoController::LayerFrameConfig layer_config;
    bool awaiting_frame;
  };

  VideoCodec config_;

  std::vector<LayerState> layers_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_SVC_SIMULCAST_TO_SVC_CONVERTER_H_
