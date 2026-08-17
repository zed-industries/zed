/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_CREATE_FRAME_GENERATOR_H_
#define API_TEST_CREATE_FRAME_GENERATOR_H_

#include <cstddef>
#include <cstdint>
#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "absl/base/nullability.h"
#include "absl/strings/string_view.h"
#include "api/environment/environment.h"
#include "api/test/frame_generator_interface.h"
#include "system_wrappers/include/clock.h"

namespace webrtc {
namespace test {

// Creates a frame generator that produces frames with small squares that
// move randomly towards the lower right corner.
// `type` has the default value FrameGeneratorInterface::OutputType::I420.
// `num_squares` has the default value 10.
std::unique_ptr<FrameGeneratorInterface> CreateSquareFrameGenerator(
    int width,
    int height,
    std::optional<FrameGeneratorInterface::OutputType> type,
    std::optional<int> num_squares);

// Creates a frame generator that repeatedly plays a set of yuv files.
// The frame_repeat_count determines how many times each frame is shown,
// with 1 = show each frame once, etc.
std::unique_ptr<FrameGeneratorInterface> CreateFromYuvFileFrameGenerator(
    std::vector<std::string> filenames,
    size_t width,
    size_t height,
    int frame_repeat_count);

// Creates a frame generator that repeatedly plays a set of nv12 files.
// The frame_repeat_count determines how many times each frame is shown,
// with 1 = show each frame once, etc.
std::unique_ptr<FrameGeneratorInterface> CreateFromNV12FileFrameGenerator(
    std::vector<std::string> filenames,
    size_t width,
    size_t height,
    int frame_repeat_count = 1);

absl_nonnull std::unique_ptr<FrameGeneratorInterface>
CreateFromIvfFileFrameGenerator(const Environment& env,
                                absl::string_view filename,
                                std::optional<int> fps_hint = std::nullopt);

// Creates a frame generator which takes a set of yuv files (wrapping a
// frame generator created by CreateFromYuvFile() above), but outputs frames
// that have been cropped to specified resolution: source_width/source_height
// is the size of the source images, target_width/target_height is the size of
// the cropped output. For each source image read, the cropped viewport will
// be scrolled top to bottom/left to right for scroll_tim_ms milliseconds.
// After that the image will stay in place for pause_time_ms milliseconds,
// and then this will be repeated with the next file from the input set.
std::unique_ptr<FrameGeneratorInterface>
CreateScrollingInputFromYuvFilesFrameGenerator(
    Clock* clock,
    std::vector<std::string> filenames,
    size_t source_width,
    size_t source_height,
    size_t target_width,
    size_t target_height,
    int64_t scroll_time_ms,
    int64_t pause_time_ms);

// Creates a frame generator that produces randomly generated slides. It fills
// the frames with randomly sized and colored squares.
// `frame_repeat_count` determines how many times each slide is shown.
std::unique_ptr<FrameGeneratorInterface>
CreateSlideFrameGenerator(int width, int height, int frame_repeat_count);

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_CREATE_FRAME_GENERATOR_H_
