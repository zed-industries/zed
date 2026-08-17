// Copyright 2025 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_DRAWING_BUFFER_SWAP_CHAIN_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_DRAWING_BUFFER_SWAP_CHAIN_H_

#include "third_party/blink/renderer/modules/xr/xr_webgl_swap_chain.h"
#include "third_party/blink/renderer/platform/graphics/gpu/xr_webgl_drawing_buffer.h"

namespace blink {

// This swap chain wraps a XRWebGLDrawingBuffer, which enables it to behave more
// like a XRWebGLLayer and make use of the SUBMIT_AS_TEXTURE_HANDLE and
// SUBMIT_AS_MAILBOX_HOLDER transport methods.
// TODO(crbug.com/40700985): This swap chain variant can be removed once
// DRAW_INTO_TEXTURE_MAILBOX is the only remaining transport method.
class XRWebGLDrawingBufferSwapChain final : public XRWebGLSwapChain {
 public:
  XRWebGLDrawingBufferSwapChain(WebGLRenderingContextBase*,
                                const XRWebGLSwapChain::Descriptor&,
                                bool webgl2);
  ~XRWebGLDrawingBufferSwapChain() override;

  WebGLUnownedTexture* ProduceTexture() override;

  scoped_refptr<StaticBitmapImage> TransferToStaticBitmapImage() override;

  void OnFrameEnd() override;

  void Trace(Visitor* visitor) const override;

 private:
  GLuint GetCopyProgram();

  scoped_refptr<XRWebGLDrawingBuffer> drawing_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_DRAWING_BUFFER_SWAP_CHAIN_H_
