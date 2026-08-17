// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_DEPTH_INFORMATION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_DEPTH_INFORMATION_H_

#include "third_party/blink/renderer/modules/xr/xr_depth_information.h"

namespace blink {

class ExceptionState;
class WebGLTexture;

class XRWebGLDepthInformation final : public XRDepthInformation {
  DEFINE_WRAPPERTYPEINFO();

 public:
  WebGLTexture* texture(ExceptionState& exception_state);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_DEPTH_INFORMATION_H_
