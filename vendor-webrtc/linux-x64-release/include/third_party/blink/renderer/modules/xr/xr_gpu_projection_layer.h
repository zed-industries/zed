// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_PROJECTION_LAYER_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_PROJECTION_LAYER_H_

#include "third_party/blink/renderer/modules/xr/xr_projection_layer.h"

namespace blink {

class GPUDevice;
class XRGPUBinding;
class XRGPUSwapChain;

class XRGPUProjectionLayer final : public XRProjectionLayer {
 public:
  XRGPUProjectionLayer(XRGPUBinding*,
                       XRGPUSwapChain* color_swap_chain,
                       XRGPUSwapChain* depth_stencil_swap_chain);
  ~XRGPUProjectionLayer() override = default;

  uint16_t textureWidth() const override;
  uint16_t textureHeight() const override;
  uint16_t textureArrayLength() const override;

  void OnFrameStart() override;
  void OnFrameEnd() override;
  void OnResize() override;

  XRGPUSwapChain* color_swap_chain() { return color_swap_chain_.Get(); }
  XRGPUSwapChain* depth_stencil_swap_chain() {
    return depth_stencil_swap_chain_.Get();
  }

  void MarkViewportUpdated() { viewport_updated_ = true; }

  void Trace(Visitor*) const override;

 private:
  Member<GPUDevice> device_;
  Member<XRGPUSwapChain> color_swap_chain_;
  Member<XRGPUSwapChain> depth_stencil_swap_chain_;
  bool viewport_updated_ = true;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_PROJECTION_LAYER_H_
