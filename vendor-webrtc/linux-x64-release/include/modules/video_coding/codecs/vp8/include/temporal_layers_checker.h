/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_TEMPORAL_LAYERS_CHECKER_H_
#define MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_TEMPORAL_LAYERS_CHECKER_H_

#include <stdint.h>

#include <memory>

#include "api/video_codecs/vp8_frame_config.h"
#include "api/video_codecs/vp8_temporal_layers.h"

namespace webrtc {

// Interface for a class that verifies correctness of temporal layer
// configurations (dependencies, sync flag, etc).
// Intended to be used in tests as well as with real apps in debug mode.
class TemporalLayersChecker {
 public:
  explicit TemporalLayersChecker(int num_temporal_layers);
  virtual ~TemporalLayersChecker() {}

  virtual bool CheckTemporalConfig(bool frame_is_keyframe,
                                   const Vp8FrameConfig& frame_config);

  static std::unique_ptr<TemporalLayersChecker> CreateTemporalLayersChecker(
      Vp8TemporalLayersType type,
      int num_temporal_layers);

 private:
  struct BufferState {
    BufferState() : is_keyframe(true), temporal_layer(0), sequence_number(0) {}
    bool is_keyframe;
    uint8_t temporal_layer;
    uint32_t sequence_number;
  };
  bool CheckAndUpdateBufferState(BufferState* state,
                                 bool* need_sync,
                                 bool frame_is_keyframe,
                                 uint8_t temporal_layer,
                                 Vp8FrameConfig::BufferFlags flags,
                                 uint32_t sequence_number,
                                 uint32_t* lowest_sequence_referenced);
  BufferState last_;
  BufferState arf_;
  BufferState golden_;
  int num_temporal_layers_;
  uint32_t sequence_number_;
  uint32_t last_sync_sequence_number_;
  uint32_t last_tl0_sequence_number_;
};

}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_VP8_INCLUDE_TEMPORAL_LAYERS_CHECKER_H_
