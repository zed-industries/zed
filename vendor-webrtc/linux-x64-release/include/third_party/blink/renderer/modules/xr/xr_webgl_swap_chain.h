// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_SWAP_CHAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_SWAP_CHAIN_H_

#include "third_party/blink/renderer/modules/webgl/webgl_rendering_context_base.h"
#include "third_party/blink/renderer/modules/webgl/webgl_unowned_texture.h"
#include "third_party/blink/renderer/modules/xr/xr_swap_chain.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class WebGLRenderingContextBase;
class WebGLUnownedTexture;

class XRWebGLSwapChain : public XRSwapChain<WebGLUnownedTexture> {
 public:
  struct Descriptor {
    GLenum format;
    GLenum internal_format;
    GLenum type;
    GLenum attachment_target;
    uint16_t width;
    uint16_t height;
    uint16_t layers;
  };

  XRWebGLSwapChain(WebGLRenderingContextBase*,
                   const XRWebGLSwapChain::Descriptor&,
                   bool webgl_2);
  ~XRWebGLSwapChain() override = default;

  WebGLRenderingContextBase* context() { return webgl_context_.Get(); }
  const XRWebGLSwapChain::Descriptor& descriptor() const { return descriptor_; }
  bool webgl2() const { return webgl2_; }

  void Trace(Visitor* visitor) const override;

  void ClearCurrentTexture();

  virtual scoped_refptr<StaticBitmapImage> TransferToStaticBitmapImage() {
    return nullptr;
  }

 protected:
  WebGLFramebuffer* GetFramebuffer();

 private:
  Member<WebGLRenderingContextBase> webgl_context_;
  Member<WebGLFramebuffer> framebuffer_;

  XRWebGLSwapChain::Descriptor descriptor_;
  bool webgl2_;
};

// A texture swap chain that is not communicated back to the compositor, used
// for things like depth/stencil attachments that don't assist reprojection.
class XRWebGLStaticSwapChain final : public XRWebGLSwapChain {
 public:
  XRWebGLStaticSwapChain(WebGLRenderingContextBase*,
                         const XRWebGLSwapChain::Descriptor&,
                         bool webgl2);
  ~XRWebGLStaticSwapChain() override;

  WebGLUnownedTexture* ProduceTexture() override;

  void OnFrameEnd() override;

 private:
  GLuint owned_texture_;
};

// A swap chain backed by SharedImages
class XRWebGLSharedImageSwapChain final : public XRWebGLSwapChain {
 public:
  XRWebGLSharedImageSwapChain(WebGLRenderingContextBase*,
                              const XRWebGLSwapChain::Descriptor&,
                              bool webgl2);
  ~XRWebGLSharedImageSwapChain() override = default;

  WebGLUnownedTexture* ProduceTexture() override;

  void OnFrameEnd() override;

 private:
  std::unique_ptr<gpu::SharedImageTexture> shared_image_texture_;
  std::unique_ptr<gpu::SharedImageTexture::ScopedAccess>
      shared_image_scoped_access_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_SWAP_CHAIN_H_
