/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_TEST_VIDEO_CAPTURER_H_
#define TEST_TEST_VIDEO_CAPTURER_H_

#include <stddef.h>

#include <memory>

#include "api/video/video_frame.h"
#include "api/video/video_source_interface.h"
#include "media/base/video_adapter.h"
#include "media/base/video_broadcaster.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {
namespace test {

class TestVideoCapturer : public VideoSourceInterface<VideoFrame> {
 public:
  class FramePreprocessor {
   public:
    virtual ~FramePreprocessor() = default;

    virtual VideoFrame Preprocess(const VideoFrame& frame) = 0;
  };

  ~TestVideoCapturer() override;

  void AddOrUpdateSink(VideoSinkInterface<VideoFrame>* sink,
                       const VideoSinkWants& wants) override;
  void RemoveSink(VideoSinkInterface<VideoFrame>* sink) override;
  void SetFramePreprocessor(std::unique_ptr<FramePreprocessor> preprocessor) {
    MutexLock lock(&lock_);
    preprocessor_ = std::move(preprocessor);
  }
  void SetEnableAdaptation(bool enable_adaptation) {
    MutexLock lock(&lock_);
    enable_adaptation_ = enable_adaptation;
  }
  void OnOutputFormatRequest(int width,
                             int height,
                             const std::optional<int>& max_fps);

  // Starts or resumes video capturing. Can be called multiple times during
  // lifetime of this object.
  virtual void Start() = 0;
  // Stops or pauses video capturing. Can be called multiple times during
  // lifetime of this object.
  virtual void Stop() = 0;

  virtual int GetFrameWidth() const = 0;
  virtual int GetFrameHeight() const = 0;

 protected:
  void OnFrame(const VideoFrame& frame);
  VideoSinkWants GetSinkWants();

 private:
  void UpdateVideoAdapter();
  VideoFrame MaybePreprocess(const VideoFrame& frame);

  Mutex lock_;
  std::unique_ptr<FramePreprocessor> preprocessor_ RTC_GUARDED_BY(lock_);
  bool enable_adaptation_ RTC_GUARDED_BY(lock_) = true;
  VideoBroadcaster broadcaster_;
  VideoAdapter video_adapter_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_TEST_VIDEO_CAPTURER_H_
