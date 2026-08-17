/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */
#ifndef TEST_VIDEO_RENDERER_H_
#define TEST_VIDEO_RENDERER_H_

#include <stddef.h>

#include "api/video/video_sink_interface.h"

namespace webrtc {
class VideoFrame;

namespace test {
class VideoRenderer : public VideoSinkInterface<VideoFrame> {
 public:
  // Creates a platform-specific renderer if possible, or a null implementation
  // if failing.
  static VideoRenderer* Create(const char* window_title,
                               size_t width,
                               size_t height);
  // Returns a renderer rendering to a platform specific window if possible,
  // NULL if none can be created.
  // Creates a platform-specific renderer if possible, returns NULL if a
  // platform renderer could not be created. This occurs, for instance, when
  // running without an X environment on Linux.
  static VideoRenderer* CreatePlatformRenderer(const char* window_title,
                                               size_t width,
                                               size_t height);
  virtual ~VideoRenderer() {}

 protected:
  VideoRenderer() {}
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_VIDEO_RENDERER_H_
