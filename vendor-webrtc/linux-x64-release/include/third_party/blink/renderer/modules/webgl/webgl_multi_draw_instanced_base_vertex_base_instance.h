// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_MULTI_DRAW_INSTANCED_BASE_VERTEX_BASE_INSTANCE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_MULTI_DRAW_INSTANCED_BASE_VERTEX_BASE_INSTANCE_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/blink/renderer/modules/webgl/webgl_multi_draw_common.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

namespace blink {

class WebGLMultiDrawInstancedBaseVertexBaseInstance final
    : public WebGLExtension,
      public WebGLMultiDrawCommon {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit WebGLMultiDrawInstancedBaseVertexBaseInstance(
      WebGLRenderingContextBase*);
  WebGLExtensionName GetName() const override;

  void multiDrawArraysInstancedBaseInstanceWEBGL(
      GLenum mode,
      base::span<const int32_t> firsts_list,
      GLuint firsts_offset,
      base::span<const int32_t> counts_list,
      GLuint counts_offset,
      base::span<const int32_t> instance_counts_list,
      GLuint instance_counts_offset,
      base::span<const uint32_t> baseinstances_list,
      GLuint baseinstances_offset,
      GLsizei drawcount);

  void multiDrawElementsInstancedBaseVertexBaseInstanceWEBGL(
      GLenum mode,
      base::span<const int32_t> counts_list,
      GLuint counts_offset,
      GLenum type,
      base::span<const int32_t> offsets_list,
      GLuint offsets_offset,
      base::span<const int32_t> instance_counts_list,
      GLuint instance_counts_offset,
      base::span<const int32_t> basevertices_list,
      GLuint basevertices_offset,
      base::span<const uint32_t> baseinstances_list,
      GLuint baseinstances_offset,
      GLsizei drawcount);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_WEBGL_MULTI_DRAW_INSTANCED_BASE_VERTEX_BASE_INSTANCE_H_
