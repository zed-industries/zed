// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_TEXTURE_ARRAY_SWAP_CHAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_TEXTURE_ARRAY_SWAP_CHAIN_H_

#include "third_party/blink/renderer/modules/xr/xr_webgl_swap_chain.h"

namespace blink {

// This swap chain is a shim which wraps another swap chain that uses a
// side-by-side layout and produces a texture array instead. When the frame ends
// the contents of each array layer are copied to the corresponding viewports of
// the wrapped swap chain. This obviously incurs undesirable overhead, and it
// would be ideal if we could use the wrapped swap chains directly, but that
// isn't possible until we add texture array support to SharedImages.
// TODO(crbug.com/359418629): Remove once array SharedImages are available.
class XRWebGLTextureArraySwapChain final : public XRWebGLSwapChain {
 public:
  XRWebGLTextureArraySwapChain(XRWebGLSwapChain* wrapped_swap_chain,
                               uint32_t layers);
  ~XRWebGLTextureArraySwapChain() override;

  WebGLUnownedTexture* ProduceTexture() override;

  void OnFrameStart() override;
  void OnFrameEnd() override;

  void SetLayer(XRCompositionLayer* layer) override;

  scoped_refptr<StaticBitmapImage> TransferToStaticBitmapImage() override;

  void Trace(Visitor* visitor) const override;

 private:
  GLuint GetCopyProgram();

  Member<XRWebGLSwapChain> wrapped_swap_chain_;
  GLuint owned_texture_;

  GLuint copy_program_;
  GLuint texture_uniform_;
  GLuint layer_count_uniform_;
  GLuint vao_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_TEXTURE_ARRAY_SWAP_CHAIN_H_
