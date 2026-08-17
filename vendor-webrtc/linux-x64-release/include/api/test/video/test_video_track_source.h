/*
 *  Copyright (c) 2023 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef API_TEST_VIDEO_TEST_VIDEO_TRACK_SOURCE_H_
#define API_TEST_VIDEO_TEST_VIDEO_TRACK_SOURCE_H_

#include <optional>
#include <string>

#include "api/media_stream_interface.h"
#include "api/notifier.h"
#include "api/sequence_checker.h"
#include "api/video/recordable_encoded_frame.h"
#include "api/video/video_frame.h"
#include "api/video/video_sink_interface.h"
#include "api/video/video_source_interface.h"
#include "rtc_base/system/no_unique_address.h"
#include "rtc_base/thread_annotations.h"

namespace webrtc {
namespace test {

// Video source that can be used as input for tests.
class TestVideoTrackSource : public Notifier<VideoTrackSourceInterface> {
 public:
  explicit TestVideoTrackSource(
      bool remote,
      std::optional<std::string> stream_label = std::nullopt);
  ~TestVideoTrackSource() override = default;

  void SetState(SourceState new_state);

  SourceState state() const override;
  bool remote() const override { return remote_; }

  bool is_screencast() const override { return false; }
  std::optional<bool> needs_denoising() const override { return std::nullopt; }

  bool GetStats(Stats* /* stats */) override { return false; }

  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants) override;
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink) override;

  bool SupportsEncodedOutput() const override { return false; }
  void GenerateKeyFrame() override {}
  void AddEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* /* sink */) override {}
  void RemoveEncodedSink(
      VideoSinkInterface<RecordableEncodedFrame>* /* sink */) override {}

  // Starts producing video.
  virtual void Start() = 0;

  // Stops producing video.
  virtual void Stop() = 0;

  virtual void SetScreencast(bool is_screencast) = 0;

  // TODO(titovartem): make next 4 methods pure virtual.
  virtual void SetEnableAdaptation(bool /* enable_adaptation */) {}

  virtual int GetFrameWidth() const { return 0; }
  virtual int GetFrameHeight() const { return 0; }

  virtual void OnOutputFormatRequest(int /* width */,
                                     int /* height */,
                                     const std::optional<int>& /* max_fps */) {}

  // Returns stream label for this video source if present. Implementations
  // may override this method to increase debugability and testability.
  virtual std::optional<std::string> GetStreamLabel() { return stream_label_; }

 protected:
  virtual VideoSourceInterface<VideoFrame>* source() = 0;

 private:
  const std::optional<std::string> stream_label_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker worker_thread_checker_;
  RTC_NO_UNIQUE_ADDRESS SequenceChecker signaling_thread_checker_;
  SourceState state_ RTC_GUARDED_BY(&signaling_thread_checker_);
  const bool remote_;
};

}  // namespace test
}  // namespace webrtc

#endif  // API_TEST_VIDEO_TEST_VIDEO_TRACK_SOURCE_H_
