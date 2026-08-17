// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_TEXTURE_ARRAY_SWAP_CHAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_TEXTURE_ARRAY_SWAP_CHAIN_H_

#include "third_party/blink/renderer/modules/xr/xr_gpu_swap_chain.h"

namespace blink {

// This swap chain is a shim which wraps another swap chain that uses a
// side-by-side layout and produces a texture array instead. When the frame ends
// the contents of each array layer are copied to the corresponding viewports of
// the wrapped swap chain. This obviously incurs undesirable overhead, and it
// would be ideal if we could use the wrapped swap chains directly, but that
// isn't possible until we add texture array support to SharedImages/Mailboxes.
// TODO(crbug.com/359418629): Remove once array Mailboxes are available.
class XRGPUTextureArraySwapChain final : public XRGPUSwapChain {
 public:
  XRGPUTextureArraySwapChain(GPUDevice* device,
                             XRGPUSwapChain* wrapped_swap_chain,
                             wgpu::TextureFormat format,
                             uint32_t layers);
  ~XRGPUTextureArraySwapChain() override = default;

  GPUTexture* ProduceTexture() override;

  void OnFrameStart() override;
  void OnFrameEnd() override;

  const wgpu::TextureDescriptor& descriptor() const override {
    return descriptor_;
  }

  void SetLayer(XRCompositionLayer* layer) override;

  void Trace(Visitor* visitor) const override;

 private:
  void DirectCopy(wgpu::CommandEncoder command_encoder,
                  GPUTexture* source_texture,
                  GPUTexture* dest_texture);
  void RenderCopy(wgpu::CommandEncoder command_encoder,
                  GPUTexture* source_texture,
                  GPUTexture* dest_texture);

  wgpu::RenderPipeline GetCopyPipelineForFormat(wgpu::TextureFormat format);

  Member<XRGPUSwapChain> wrapped_swap_chain_;
  wgpu::TextureDescriptor descriptor_;
  wgpu::DawnTextureInternalUsageDescriptor texture_internal_usage_;
  wgpu::RenderPipeline copy_pipeline_;
  wgpu::TextureFormat copy_pipeline_format_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_TEXTURE_ARRAY_SWAP_CHAIN_H_
