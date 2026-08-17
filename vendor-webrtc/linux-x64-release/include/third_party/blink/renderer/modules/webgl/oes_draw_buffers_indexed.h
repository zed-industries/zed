// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OES_DRAW_BUFFERS_INDEXED_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OES_DRAW_BUFFERS_INDEXED_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/khronos/GLES2/gl2.h"

namespace blink {

class OESDrawBuffersIndexed final : public WebGLExtension {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit OESDrawBuffersIndexed(WebGLRenderingContextBase*);

  WebGLExtensionName GetName() const override;

  void enableiOES(GLenum target, GLuint index);

  void disableiOES(GLenum target, GLuint index);

  void blendEquationiOES(GLuint buf, GLenum mode);

  void blendEquationSeparateiOES(GLuint buf, GLenum modeRGB, GLenum modeAlpha);

  void blendFunciOES(GLuint buf, GLenum src, GLenum dst);

  void blendFuncSeparateiOES(GLuint buf,
                             GLenum srcRGB,
                             GLenum dstRGB,
                             GLenum srcAlpha,
                             GLenum dstAlpha);

  void colorMaskiOES(GLuint buf,
                     GLboolean r,
                     GLboolean g,
                     GLboolean b,
                     GLboolean a);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OES_DRAW_BUFFERS_INDEXED_H_
