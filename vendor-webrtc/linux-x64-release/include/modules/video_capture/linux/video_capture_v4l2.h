/*
 *  Copyright (c) 2012 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_VIDEO_CAPTURE_LINUX_VIDEO_CAPTURE_V4L2_H_
#define MODULES_VIDEO_CAPTURE_LINUX_VIDEO_CAPTURE_V4L2_H_

#include <stddef.h>
#include <stdint.h>

#include <memory>

#include "modules/video_capture/video_capture_defines.h"
#include "modules/video_capture/video_capture_impl.h"
#include "rtc_base/platform_thread.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {
namespace videocapturemodule {
class VideoCaptureModuleV4L2 : public VideoCaptureImpl {
 public:
  VideoCaptureModuleV4L2();
  ~VideoCaptureModuleV4L2() override;
  int32_t Init(const char* deviceUniqueId);
  int32_t StartCapture(const VideoCaptureCapability& capability) override;
  int32_t StopCapture() override;
  bool CaptureStarted() override;
  int32_t CaptureSettings(VideoCaptureCapability& settings) override;

 private:
  enum { kNoOfV4L2Bufffers = 4 };

  static void CaptureThread(void*);
  bool CaptureProcess();
  bool AllocateVideoBuffers() RTC_EXCLUSIVE_LOCKS_REQUIRED(capture_lock_);
  bool DeAllocateVideoBuffers() RTC_EXCLUSIVE_LOCKS_REQUIRED(capture_lock_);

  PlatformThread _captureThread RTC_GUARDED_BY(api_checker_);
  Mutex capture_lock_ RTC_ACQUIRED_BEFORE(api_lock_);
  bool quit_ RTC_GUARDED_BY(capture_lock_);
  int32_t _deviceId RTC_GUARDED_BY(api_checker_);
  int32_t _deviceFd RTC_GUARDED_BY(capture_checker_);

  int32_t _buffersAllocatedByDevice RTC_GUARDED_BY(capture_lock_);
  VideoCaptureCapability configured_capability_
      RTC_GUARDED_BY(capture_checker_);
  bool _streaming RTC_GUARDED_BY(capture_checker_);
  bool _captureStarted RTC_GUARDED_BY(api_checker_);
  struct Buffer {
    void* start;
    size_t length;
  };
  Buffer* _pool RTC_GUARDED_BY(capture_lock_);
};
}  // namespace videocapturemodule
}  // namespace webrtc

#endif  // MODULES_VIDEO_CAPTURE_LINUX_VIDEO_CAPTURE_V4L2_H_
