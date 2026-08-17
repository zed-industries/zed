// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_PROJECTION_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_PROJECTION_LAYER_H_

#include "third_party/blink/renderer/modules/xr/xr_projection_layer.h"
#include "third_party/blink/renderer/modules/xr/xr_webgl_layer_client.h"

namespace blink {

class WebGLRenderingContextBase;
class XRWebGLBinding;
class XRWebGLSwapChain;

class XRWebGLProjectionLayer final : public XRProjectionLayer,
                                     public XRWebGLLayerClient {
 public:
  XRWebGLProjectionLayer(XRWebGLBinding*,
                         XRWebGLSwapChain* color_swap_chain,
                         XRWebGLSwapChain* depth_stencil_swap_chain);
  ~XRWebGLProjectionLayer() override = default;

  // XRWebGLLayerClient implementation
  const XRLayer* layer() const override { return this; }
  WebGLRenderingContextBase* context() const override {
    return webgl_context_.Get();
  }
  scoped_refptr<StaticBitmapImage> TransferToStaticBitmapImage() override;

  uint16_t textureWidth() const override;
  uint16_t textureHeight() const override;
  uint16_t textureArrayLength() const override;

  void OnFrameStart() override;
  void OnFrameEnd() override;
  void OnResize() override;

  XRWebGLSwapChain* color_swap_chain() const { return color_swap_chain_.Get(); }
  XRWebGLSwapChain* depth_stencil_swap_chain() const {
    return depth_stencil_swap_chain_.Get();
  }

  void MarkViewportUpdated() { viewport_updated_ = true; }

  void Trace(Visitor*) const override;

 private:
  Member<WebGLRenderingContextBase> webgl_context_;
  Member<XRWebGLSwapChain> color_swap_chain_;
  Member<XRWebGLSwapChain> depth_stencil_swap_chain_;
  bool viewport_updated_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_PROJECTION_LAYER_H_
