/*
 *  Copyright (c) 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_CREATE_VIDEOCODEC_TEST_FIXTURE_H_
#define API_TEST_CREATE_VIDEOCODEC_TEST_FIXTURE_H_

#include <memory>

#include "api/test/videocodec_test_fixture.h"
#include "api/video_codecs/video_decoder_factory.h"
#include "api/video_codecs/video_encoder_factory.h"

namespace webrtc {
namespace test {

std::unique_ptr<VideoCodecTestFixture> CreateVideoCodecTestFixture(
    const VideoCodecTestFixture::Config& config);

std::unique_ptr<VideoCodecTestFixture> CreateVideoCodecTestFixture(
    const VideoCodecTestFixture::Config& config,
    std::unique_ptr<VideoDecoderFactory> decoder_factory,
    std::unique_ptr<VideoEncoderFactory> encoder_factory);

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_CREATE_VIDEOCODEC_TEST_FIXTURE_H_
