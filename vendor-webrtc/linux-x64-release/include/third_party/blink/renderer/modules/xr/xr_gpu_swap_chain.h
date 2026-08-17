// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_SWAP_CHAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_SWAP_CHAIN_H_

#include "third_party/blink/renderer/modules/webgpu/gpu_texture.h"
#include "third_party/blink/renderer/modules/xr/xr_swap_chain.h"
#include "third_party/blink/renderer/platform/graphics/gpu/webgpu_swap_buffer_provider.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class GPUDevice;
class GPUTexture;

class XRGPUSwapChain : public XRSwapChain<GPUTexture> {
 public:
  explicit XRGPUSwapChain(GPUDevice*);
  ~XRGPUSwapChain() override = default;

  GPUDevice* device() { return device_.Get(); }
  virtual const wgpu::TextureDescriptor& descriptor() const = 0;

  void Trace(Visitor* visitor) const override;

 protected:
  void ClearCurrentTexture(wgpu::CommandEncoder);

 private:
  Member<GPUDevice> device_;
};

// A texture swap chain that is not communicated back to the compositor, used
// for things like depth/stencil attachments that don't assist reprojection.
class XRGPUStaticSwapChain final : public XRGPUSwapChain {
 public:
  XRGPUStaticSwapChain(GPUDevice*, const wgpu::TextureDescriptor&);

  GPUTexture* ProduceTexture() override;

  void OnFrameEnd() override;

  const wgpu::TextureDescriptor& descriptor() const override {
    return descriptor_;
  }

 private:
  wgpu::TextureDescriptor descriptor_;
};

// A swap chain backed by Mailboxes
class XRGPUMailboxSwapChain final : public XRGPUSwapChain {
 public:
  XRGPUMailboxSwapChain(GPUDevice* device, const wgpu::TextureDescriptor& desc);
  ~XRGPUMailboxSwapChain() override = default;

  GPUTexture* ProduceTexture() override;

  void OnFrameEnd() override;

  const wgpu::TextureDescriptor& descriptor() const override {
    return descriptor_;
  }

 private:
  wgpu::TextureDescriptor descriptor_;
  wgpu::DawnTextureInternalUsageDescriptor texture_internal_usage_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_GPU_SWAP_CHAIN_H_
