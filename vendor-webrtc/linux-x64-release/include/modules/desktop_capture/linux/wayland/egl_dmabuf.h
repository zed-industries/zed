/*
 *  Copyright 2021 The WebRTC project authors. All Rights Reserved.
 *
 *  Use of this source code is governed by a BSD-style license
 *  that can be found in the LICENSE file in the root of the source
 *  tree. An additional intellectual property rights grant can be found
 *  in the file PATENTS.  All contributing project authors may
 *  be found in the AUTHORS file in the root of the source tree.
 */

#ifndef MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_EGL_DMABUF_H_
#define MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_EGL_DMABUF_H_

#include <EGL/egl.h>
#include <GL/gl.h>
#include <gbm.h>

#include <memory>
#include <optional>
#include <string>
#include <vector>

#include "modules/desktop_capture/desktop_geometry.h"

namespace webrtc {

class EglDmaBuf {
 public:
  struct EGLStruct {
    std::vector<std::string> extensions;
    EGLDisplay display = EGL_NO_DISPLAY;
    EGLContext context = EGL_NO_CONTEXT;
  };

  struct PlaneData {
    int32_t fd;
    uint32_t stride;
    uint32_t offset;
  };

  EglDmaBuf();
  ~EglDmaBuf();

  // Returns whether the image was successfully imported from
  // given DmaBuf and its parameters
  bool ImageFromDmaBuf(const DesktopSize& size,
                       uint32_t format,
                       const std::vector<PlaneData>& plane_datas,
                       uint64_t modifiers,
                       const DesktopVector& offset,
                       const DesktopSize& buffer_size,
                       uint8_t* data);
  std::vector<uint64_t> QueryDmaBufModifiers(uint32_t format);

  bool IsEglInitialized() const { return egl_initialized_; }

 private:
  bool GetClientExtensions(EGLDisplay dpy, EGLint name);

  bool egl_initialized_ = false;
  bool has_image_dma_buf_import_ext_ = false;
  int32_t drm_fd_ = -1;               // for GBM buffer mmap
  gbm_device* gbm_device_ = nullptr;  // for passed GBM buffer retrieval

  GLuint fbo_ = 0;
  GLuint texture_ = 0;
  EGLStruct egl_;

  std::optional<std::string> GetRenderNode();
};

}  // namespace webrtc

#endif  // MODULES_DESKTOP_CAPTURE_LINUX_WAYLAND_EGL_DMABUF_H_
