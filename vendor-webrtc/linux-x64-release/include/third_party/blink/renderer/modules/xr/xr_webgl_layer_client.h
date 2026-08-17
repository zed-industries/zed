// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_LAYER_CLIENT_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_LAYER_CLIENT_H_

#include "third_party/blink/renderer/platform/graphics/static_bitmap_image.h"
#include "third_party/blink/renderer/platform/wtf/ref_counted.h"

namespace blink {

class WebGLRenderingContextBase;
class XRLayer;

// Shared interface for XRWebGLLayer and WebGL-based XRProjectionLayers to
// submit frames with.
class XRWebGLLayerClient {
 public:
  virtual const XRLayer* layer() const = 0;
  virtual WebGLRenderingContextBase* context() const = 0;
  virtual scoped_refptr<StaticBitmapImage> TransferToStaticBitmapImage() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_LAYER_CLIENT_H_
