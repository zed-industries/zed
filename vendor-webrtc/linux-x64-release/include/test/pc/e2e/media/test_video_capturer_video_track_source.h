/*
 *  Copyright (c) 2020 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_PC_E2E_MEDIA_TEST_VIDEO_CAPTURER_VIDEO_TRACK_SOURCE_H_
#define TEST_PC_E2E_MEDIA_TEST_VIDEO_CAPTURER_VIDEO_TRACK_SOURCE_H_

#include <memory>
#include <optional>
#include <utility>

#include "api/sequence_checker.h"
#include "api/test/video/test_video_track_source.h"
#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "test/test_video_capturer.h"

namespace webrtc {
namespace webrtc_pc_e2e {

class TestVideoCapturerVideoTrackSource : public test::TestVideoTrackSource {
 public:
  TestVideoCapturerVideoTrackSource(
      std::unique_ptr<test::TestVideoCapturer> video_capturer,
      bool is_screencast,
      std::optional<std::string> stream_label = std::nullopt)
      : TestVideoTrackSource(/*remote=*/false, std::move(stream_label)),
        video_capturer_(std::move(video_capturer)),
        is_screencast_(is_screencast) {
    sequence_checker_.Detach();
  }

  TestVideoCapturerVideoTrackSource(const TestVideoCapturerVideoTrackSource&) =
      delete;
  TestVideoCapturerVideoTrackSource& operator=(
      const TestVideoCapturerVideoTrackSource&) = delete;
  TestVideoCapturerVideoTrackSource(TestVideoCapturerVideoTrackSource&&) =
      delete;
  TestVideoCapturerVideoTrackSource& operator=(
      TestVideoCapturerVideoTrackSource&&) = delete;

  ~TestVideoCapturerVideoTrackSource() = default;

  void Start() override {
    SetState(kLive);
    video_capturer_->Start();
  }

  void Stop() override {
    SetState(kMuted);
    video_capturer_->Stop();
  }

  int GetFrameWidth() const override {
    return video_capturer_->GetFrameWidth();
  }

  int GetFrameHeight() const override {
    return video_capturer_->GetFrameHeight();
  }

  bool is_screencast() const override {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    return is_screencast_;
  }

  void SetEnableAdaptation(bool enable_adaptation) {
    video_capturer_->SetEnableAdaptation(enable_adaptation);
  }

  void OnOutputFormatRequest(int width,
                             int height,
                             const std::optional<int>& max_fps) override {
    video_capturer_->OnOutputFormatRequest(width, height, max_fps);
  }

  void SetScreencast(bool is_screencast) override {
    RTC_DCHECK_RUN_ON(&sequence_checker_);
    is_screencast_ = is_screencast;
  }

 protected:
  VideoSourceInterface<VideoFrame>* source() override {
    return video_capturer_.get();
  }

 private:
  const std::unique_ptr<test::TestVideoCapturer> video_capturer_;
  SequenceChecker sequence_checker_;
  bool is_screencast_ RTC_GUARDED_BY(sequence_checker_);
};

}  // namespace webrtc_pc_e2e
}  // namespace webrtc

#endif  // TEST_PC_E2E_MEDIA_TEST_VIDEO_CAPTURER_VIDEO_TRACK_SOURCE_H_
