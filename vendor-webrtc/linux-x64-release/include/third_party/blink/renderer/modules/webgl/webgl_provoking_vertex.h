// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_PROVOKING_VERTEX_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_PROVOKING_VERTEX_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/khronos/GLES2/gl2.h"

namespace blink {

class WebGLProvokingVertex final : public WebGLExtension {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit WebGLProvokingVertex(WebGLRenderingContextBase*);

  WebGLExtensionName GetName() const override;

  void provokingVertexWEBGL(GLenum provokeMode);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_PROVOKING_VERTEX_H_
