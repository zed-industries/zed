/*
 *  Copyright 2017 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CODING_CODECS_TEST_ANDROID_CODEC_FACTORY_HELPER_H_
#define MODULES_VIDEO_CODING_CODECS_TEST_ANDROID_CODEC_FACTORY_HELPER_H_

#include <memory>

#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

namespace webrtc {
namespace test {

void InitializeAndroidObjects();

std::unique_ptr<VideoEncoderFactory> CreateAndroidEncoderFactory();
std::unique_ptr<VideoDecoderFactory> CreateAndroidDecoderFactory();

}  // namespace test
}  // namespace webrtc

#endif  // MODULES_VIDEO_CODING_CODECS_TEST_ANDROID_CODEC_FACTORY_HELPER_H_
