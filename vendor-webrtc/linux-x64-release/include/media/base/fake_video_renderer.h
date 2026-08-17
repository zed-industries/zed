/*
 *  Copyright (c) 2011 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MEDIA_BASE_FAKE_VIDEO_RENDERER_H_
#define MEDIA_BASE_FAKE_VIDEO_RENDERER_H_

#include <stdint.h>

#include "api/video/video_frame.h"
#include "api/video/video_rotation.h"
#include "api/video/video_sink_interface.h"
#include "rtc_base/synchronization/mutex.h"

namespace webrtc {

// Faked video renderer that has a callback for actions on rendering.
class FakeVideoRenderer : public VideoSinkInterface<VideoFrame> {
 public:
  FakeVideoRenderer();

  void OnFrame(const VideoFrame& frame) override;

  int width() const {
    MutexLock lock(&mutex_);
    return width_;
  }
  int height() const {
    MutexLock lock(&mutex_);
    return height_;
  }

  VideoRotation rotation() const {
    MutexLock lock(&mutex_);
    return rotation_;
  }

  int64_t timestamp_us() const {
    MutexLock lock(&mutex_);
    return timestamp_us_;
  }

  int num_rendered_frames() const {
    MutexLock lock(&mutex_);
    return num_rendered_frames_;
  }

  bool black_frame() const {
    MutexLock lock(&mutex_);
    return black_frame_;
  }

 private:
  int width_ = 0;
  int height_ = 0;
  VideoRotation rotation_ = webrtc::kVideoRotation_0;
  int64_t timestamp_us_ = 0;
  int num_rendered_frames_ = 0;
  bool black_frame_ = false;
  mutable Mutex mutex_;
};

}  //  namespace webrtc

// Re-export symbols from the webrtc namespace for backwards compatibility.
// TODO(bugs.webrtc.org/4222596): Remove once all references are updated.
#ifdef WEBRTC_ALLOW_DEPRECATED_NAMESPACES
namespace cricket {
using ::webrtc::FakeVideoRenderer;
}  // namespace cricket
#endif  // WEBRTC_ALLOW_DEPRECATED_NAMESPACES

#endif  // MEDIA_BASE_FAKE_VIDEO_RENDERER_H_
