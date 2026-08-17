// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_POLYGON_MODE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_POLYGON_MODE_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/khronos/GLES2/gl2.h"

namespace blink {

class WebGLPolygonMode final : public WebGLExtension {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit WebGLPolygonMode(WebGLRenderingContextBase*);

  WebGLExtensionName GetName() const override;

  void polygonModeWEBGL(GLenum face, GLenum mode);

 private:
  void EmitDeferredPortabilityWarning(WebGLRenderingContextBase* context);

  bool emitted_warning_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_POLYGON_MODE_H_
