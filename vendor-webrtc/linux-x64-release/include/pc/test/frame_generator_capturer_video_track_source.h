/*
 *  Copyright 2018 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef PC_TEST_FRAME_GENERATOR_CAPTURER_VIDEO_TRACK_SOURCE_H_
#define PC_TEST_FRAME_GENERATOR_CAPTURER_VIDEO_TRACK_SOURCE_H_

#include <memory>
#include <optional>
#include <utility>

#include "api/media_stream_interface.h"
#include "api/task_queue/default_task_queue_factory.h"
#include "api/task_queue/task_queue_factory.h"
#include "api/test/create_frame_generator.h"
#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "pc/video_track_source.h"
#include "system_wrappers/include/clock.h"
#include "test/frame_generator_capturer.h"

namespace webrtc {

// Implements a VideoTrackSourceInterface to be used for creating VideoTracks.
// The video source is generated using a FrameGeneratorCapturer, specifically
// a SquareGenerator that generates frames with randomly sized and colored
// squares.
class FrameGeneratorCapturerVideoTrackSource : public VideoTrackSource {
 public:
  static const int kDefaultFramesPerSecond = 30;
  static const int kDefaultWidth = 640;
  static const int kDefaultHeight = 480;
  static const int kNumSquaresGenerated = 50;

  struct Config {
    int frames_per_second = kDefaultFramesPerSecond;
    int width = kDefaultWidth;
    int height = kDefaultHeight;
    int num_squares_generated = 50;
  };

  FrameGeneratorCapturerVideoTrackSource(Config config,
                                         Clock* clock,
                                         bool is_screencast)
      : VideoTrackSource(false /* remote */),
        task_queue_factory_(CreateDefaultTaskQueueFactory()),
        is_screencast_(is_screencast) {
    video_capturer_ = std::make_unique<test::FrameGeneratorCapturer>(
        clock,
        test::CreateSquareFrameGenerator(config.width, config.height,
                                         std::nullopt,
                                         config.num_squares_generated),
        config.frames_per_second, *task_queue_factory_);
    video_capturer_->Init();
  }

  FrameGeneratorCapturerVideoTrackSource(
      std::unique_ptr<test::FrameGeneratorCapturer> video_capturer,
      bool is_screencast)
      : VideoTrackSource(false /* remote */),
        video_capturer_(std::move(video_capturer)),
        is_screencast_(is_screencast) {}

  ~FrameGeneratorCapturerVideoTrackSource() = default;

  void Start() {
    SetState(kLive);
    video_capturer_->Start();
  }

  void Stop() {
    SetState(kMuted);
    video_capturer_->Stop();
  }

  bool is_screencast() const override { return is_screencast_; }

 protected:
  VideoSourceInterface<VideoFrame>* source() override {
    return video_capturer_.get();
  }

 private:
  const std::unique_ptr<TaskQueueFactory> task_queue_factory_;
  std::unique_ptr<test::FrameGeneratorCapturer> video_capturer_;
  const bool is_screencast_;
};

}  // namespace webrtc

#endif  // PC_TEST_FRAME_GENERATOR_CAPTURER_VIDEO_TRACK_SOURCE_H_
