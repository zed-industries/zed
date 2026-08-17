/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_PLATFORM_VIDEO_CAPTURER_H_
#define TEST_PLATFORM_VIDEO_CAPTURER_H_

#include <memory>

#include "test/test_video_capturer.h"

namespace webrtc {
namespace test {

std::unique_ptr<TestVideoCapturer> CreateVideoCapturer(
    size_t width,
    size_t height,
    size_t target_fps,
    size_t capture_device_index);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_PLATFORM_VIDEO_CAPTURER_H_
