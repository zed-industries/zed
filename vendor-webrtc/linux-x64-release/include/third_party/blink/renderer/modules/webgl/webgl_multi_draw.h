// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_MULTI_DRAW_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_MULTI_DRAW_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/blink/renderer/modules/webgl/webgl_multi_draw_common.h"

namespace blink {

class WebGLMultiDraw final : public WebGLExtension,
                             public WebGLMultiDrawCommon {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit WebGLMultiDraw(WebGLRenderingContextBase*);
  WebGLExtensionName GetName() const override;

  void multiDrawArraysWEBGL(GLenum mode,
                            base::span<const int32_t> firstsList,
                            GLuint firstsOffset,
                            base::span<const int32_t> countsList,
                            GLuint countsOffset,
                            GLsizei drawcount);

  void multiDrawElementsWEBGL(GLenum mode,
                              base::span<const int32_t> countsList,
                              GLuint countsOffset,
                              GLenum type,
                              base::span<const int32_t> offsetsList,
                              GLuint offsetsOffset,
                              GLsizei drawcount);

  void multiDrawArraysInstancedWEBGL(
      GLenum mode,
      base::span<const int32_t> firstsList,
      GLuint firstsOffset,
      base::span<const int32_t> countsList,
      GLuint countsOffset,
      base::span<const int32_t> instanceCountsList,
      GLuint instanceCountsOffset,
      GLsizei drawcount);

  void multiDrawElementsInstancedWEBGL(
      GLenum mode,
      base::span<const int32_t> countsList,
      GLuint countsOffset,
      GLenum type,
      base::span<const int32_t> offsetsList,
      GLuint offsetsOffset,
      base::span<const int32_t> instanceCountsList,
      GLuint instanceCountsOffset,
      GLsizei drawcount);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_MULTI_DRAW_H_
