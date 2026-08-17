/*
 *  Copyright (c) 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_AUDIO_CODECS_G722_AUDIO_ENCODER_G722_CONFIG_H_
#define API_AUDIO_CODECS_G722_AUDIO_ENCODER_G722_CONFIG_H_

#include "api/audio_codecs/audio_encoder.h"

namespace webrtc {

struct AudioEncoderG722Config {
  bool IsOk() const {
    return frame_size_ms > 0 && frame_size_ms % 10 == 0 && num_channels >= 1 &&
           num_channels <= AudioEncoder::kMaxNumberOfChannels;
  }
  int frame_size_ms = 20;
  int num_channels = 1;
};

}  // namespace webrtc

#endif  // API_AUDIO_CODECS_G722_AUDIO_ENCODER_G722_CONFIG_H_
