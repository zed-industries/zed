/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef API_TEST_CREATE_PEER_CONNECTION_QUALITY_TEST_FRAME_GENERATOR_H_
#define API_TEST_CREATE_PEER_CONNECTION_QUALITY_TEST_FRAME_GENERATOR_H_

#include <memory>
#include <optional>
#include <string>

#include "api/test/frame_generator_interface.h"
#include "api/test/pclf/media_configuration.h"

namespace webrtc {
namespace webrtc_pc_e2e {

// Creates a frame generator that produces frames with small squares that move
// randomly towards the lower right corner. `type` has the default value
// FrameGeneratorInterface::OutputType::I420. video_config specifies frame
// weight and height.
std::unique_ptr<test::FrameGeneratorInterface> CreateSquareFrameGenerator(
    const VideoConfig& video_config,
    std::optional<test::FrameGeneratorInterface::OutputType> type);

// Creates a frame generator that plays frames from the yuv file.
std::unique_ptr<test::FrameGeneratorInterface> CreateFromYuvFileFrameGenerator(
    const VideoConfig& video_config,
    std::string filename);

// Creates a proper frame generator for testing screen sharing.
std::unique_ptr<test::FrameGeneratorInterface> CreateScreenShareFrameGenerator(
    const VideoConfig& video_config,
    const ScreenShareConfig& screen_share_config);

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // API_TEST_CREATE_PEER_CONNECTION_QUALITY_TEST_FRAME_GENERATOR_H_
