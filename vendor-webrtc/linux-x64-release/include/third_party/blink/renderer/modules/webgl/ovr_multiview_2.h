// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OVR_MULTIVIEW_2_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OVR_MULTIVIEW_2_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/khronos/GLES2/gl2.h"

namespace blink {

class WebGLTexture;

class OVRMultiview2 final : public WebGLExtension {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit OVRMultiview2(WebGLRenderingContextBase*);

  WebGLExtensionName GetName() const override;

  void framebufferTextureMultiviewOVR(GLenum target,
                                      GLenum attachment,
                                      WebGLTexture* texture,
                                      GLint level,
                                      GLint baseViewIndex,
                                      GLsizei numViews);

 private:
  GLsizei max_views_ovr_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OVR_MULTIVIEW_2_H_
