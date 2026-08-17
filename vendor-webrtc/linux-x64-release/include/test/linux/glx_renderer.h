/*
 *  Copyright (c) 2013 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef TEST_LINUX_GLX_RENDERER_H_
#define TEST_LINUX_GLX_RENDERER_H_

#include <GL/glx.h>
#include <X11/X.h>
#include <X11/Xlib.h>
#include <stddef.h>

#include "api/video/video_frame.h"
#include "test/gl/gl_renderer.h"

namespace webrtc {
namespace test {

class GlxRenderer : public GlRenderer {
 public:
  static GlxRenderer* Create(const char* window_title,
                             size_t width,
                             size_t height);
  virtual ~GlxRenderer();

  void OnFrame(const webrtc::VideoFrame& frame) override;

 private:
  GlxRenderer(size_t width, size_t height);

  bool Init(const char* window_title);
  void Resize(size_t width, size_t height);
  void Destroy();

  size_t width_, height_;

  Display* display_;
  Window window_;
  GLXContext context_;
};
}  // namespace test
}  // namespace webrtc

#endif  // TEST_LINUX_GLX_RENDERER_H_
