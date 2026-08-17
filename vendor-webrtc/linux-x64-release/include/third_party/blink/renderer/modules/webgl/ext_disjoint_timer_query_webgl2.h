// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_EXT_DISJOINT_TIMER_QUERY_WEBGL2_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_EXT_DISJOINT_TIMER_QUERY_WEBGL2_H_

#include "third_party/blink/renderer/bindings/core/v8/script_value.h"
#include "third_party/blink/renderer/modules/webgl/webgl_extension.h"
#include "third_party/blink/renderer/modules/webgl/webgl_query.h"

namespace blink {

class WebGLRenderingContextBase;
class WebGLQuery;

class EXTDisjointTimerQueryWebGL2 final : public WebGLExtension {
  DEFINE_WRAPPERTYPEINFO();

 public:
  static bool Supported(WebGLRenderingContextBase*);
  static const char* ExtensionName();

  explicit EXTDisjointTimerQueryWebGL2(WebGLRenderingContextBase*);

  WebGLExtensionName GetName() const override;

  void queryCounterEXT(WebGLQuery*, GLenum);

  void Trace(Visitor*) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_WEBGL_EXT_DISJOINT_TIMER_QUERY_WEBGL2_H_
