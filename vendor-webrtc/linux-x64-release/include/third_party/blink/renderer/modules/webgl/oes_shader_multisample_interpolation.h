// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OES_SHADER_MULTISAMPLE_INTERPOLATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OES_SHADER_MULTISAMPLE_INTERPOLATION_H_

#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/khronos/GLES2/gl2.h"

namespace blink {

class OESShaderMultisampleInterpolation final : public WebGLExtension {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit OESShaderMultisampleInterpolation(WebGLRenderingContextBase*);

  WebGLExtensionName GetName() const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_OES_SHADER_MULTISAMPLE_INTERPOLATION_H_
