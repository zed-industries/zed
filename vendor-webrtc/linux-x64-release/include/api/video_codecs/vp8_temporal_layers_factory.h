/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_VIDEO_CODECS_VP8_TEMPORAL_LAYERS_FACTORY_H_
#define API_VIDEO_CODECS_VP8_TEMPORAL_LAYERS_FACTORY_H_

#include <memory>

#include "api/fec_controller_override.h"
#include "api/video_codecs/video_codec.h"
#include "api/video_codecs/video_encoder.h"
#include "api/video_codecs/vp8_frame_buffer_controller.h"

namespace webrtc {

class Vp8TemporalLayersFactory : public Vp8FrameBufferControllerFactory {
 public:
  ~Vp8TemporalLayersFactory() override = default;

  std::unique_ptr<Vp8FrameBufferControllerFactory> Clone() const override;

  std::unique_ptr<Vp8FrameBufferController> Create(
      const VideoCodec& codec,
      const VideoEncoder::Settings& settings,
      FecControllerOverride* fec_controller_override) override;
};

}  // namespace webrtc

#endif  // API_VIDEO_CODECS_VP8_TEMPORAL_LAYERS_FACTORY_H_
