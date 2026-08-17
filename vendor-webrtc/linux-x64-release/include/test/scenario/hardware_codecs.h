/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_SCENARIO_HARDWARE_CODECS_H_
#define TEST_SCENARIO_HARDWARE_CODECS_H_

#include <memory>

#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

namespace webrtc {
namespace test {
std::unique_ptr<VideoEncoderFactory> CreateHardwareEncoderFactory();
std::unique_ptr<VideoDecoderFactory> CreateHardwareDecoderFactory();
}  // namespace test
}  // namespace webrtc
#endif  // TEST_SCENARIO_HARDWARE_CODECS_H_
