/*
 *  Copyright (c) 2019 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_MAC_CAPTURER_H_
#define TEST_MAC_CAPTURER_H_

#include <cstddef>
#include <memory>
#include <vector>

#include "api/media_stream_interface.h"
#include "api/scoped_refptr.h"
#include "modules/video_capture/video_capture.h"
#include "rtc_base/logging.h"
#include "rtc_base/thread.h"
#include "test/test_video_capturer.h"

namespace webrtc {
namespace test {

class MacCapturer : public TestVideoCapturer,
                    public VideoSinkInterface<VideoFrame> {
 public:
  static MacCapturer* Create(size_t width,
                             size_t height,
                             size_t target_fps,
                             size_t capture_device_index);
  ~MacCapturer() override;

  void Start() override {
    RTC_LOG(LS_WARNING) << "Capturer doesn't support resume/pause and always "
                           "produces the video";
  }
  void Stop() override {
    RTC_LOG(LS_WARNING) << "Capturer doesn't support resume/pause and always "
                           "produces the video";
  }

  void OnFrame(const VideoFrame& frame) override;

  int GetFrameWidth() const override { return static_cast<int>(width_); }
  int GetFrameHeight() const override { return static_cast<int>(height_); }

 private:
  MacCapturer(size_t width,
              size_t height,
              size_t target_fps,
              size_t capture_device_index);
  void Destroy();

  size_t width_;
  size_t height_;
  void* capturer_;
  void* adapter_;
};

}  // namespace test
}  // namespace webrtc

#endif  // TEST_MAC_CAPTURER_H_
