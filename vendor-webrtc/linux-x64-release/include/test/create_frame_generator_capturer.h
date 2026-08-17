/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_CREATE_FRAME_GENERATOR_CAPTURER_H_
#define TEST_CREATE_FRAME_GENERATOR_CAPTURER_H_

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "api/task_queue/task_queue_factory.h"
#include "api/test/frame_generator_interface.h"
#include "api/units/time_delta.h"
#include "system_wrappers/include/clock.h"
#include "test/frame_generator_capturer.h"

namespace webrtc {
namespace test {

namespace frame_gen_cap_impl {
template <typename T>
class AutoOpt : public std::optional<T> {
 public:
  using std::optional<T>::optional;
  T* operator->() {
    if (!std::optional<T>::has_value())
      this->emplace(T());
    return std::optional<T>::operator->();
  }
};
}  // namespace frame_gen_cap_impl

struct FrameGeneratorCapturerConfig {
  struct SquaresVideo {
    int framerate = 30;
    FrameGeneratorInterface::OutputType pixel_format =
        FrameGeneratorInterface::OutputType::kI420;
    int width = 320;
    int height = 180;
    int num_squares = 10;
  };

  struct SquareSlides {
    int framerate = 30;
    TimeDelta change_interval = TimeDelta::Seconds(10);
    int width = 1600;
    int height = 1200;
  };

  struct VideoFile {
    int framerate = 30;
    std::string name;
    // Must be set to width and height of the source video file.
    int width = 0;
    int height = 0;
  };

  struct ImageSlides {
    int framerate = 30;
    TimeDelta change_interval = TimeDelta::Seconds(10);
    struct Crop {
      TimeDelta scroll_duration = TimeDelta::Seconds(0);
      std::optional<int> width;
      std::optional<int> height;
    } crop;
    int width = 1850;
    int height = 1110;
    std::vector<std::string> paths = {
        "web_screenshot_1850_1110",
        "presentation_1850_1110",
        "photo_1850_1110",
        "difficult_photo_1850_1110",
    };
  };

  frame_gen_cap_impl::AutoOpt<SquaresVideo> squares_video;
  frame_gen_cap_impl::AutoOpt<SquareSlides> squares_slides;
  frame_gen_cap_impl::AutoOpt<VideoFile> video_file;
  frame_gen_cap_impl::AutoOpt<ImageSlides> image_slides;
  bool allow_zero_hertz = false;
};

std::unique_ptr<FrameGeneratorCapturer> CreateFrameGeneratorCapturer(
    Clock* clock,
    TaskQueueFactory& task_queue_factory,
    const FrameGeneratorCapturerConfig& config);

}  // namespace test
}  // namespace webrtc

#endif  // TEST_CREATE_FRAME_GENERATOR_CAPTURER_H_
