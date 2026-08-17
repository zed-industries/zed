// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_SUB_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_SUB_IMAGE_H_

#include <optional>

#include "third_party/blink/renderer/modules/webgl/webgl_texture.h"
#include "third_party/blink/renderer/modules/xr/xr_sub_image.h"
#include "third_party/blink/renderer/modules/xr/xr_webgl_swap_chain.h"

namespace blink {

class XRWebGLSubImage final : public XRSubImage {
  DEFINE_WRAPPERTYPEINFO();

 public:
  XRWebGLSubImage(const gfx::Rect& viewport,
                  std::optional<uint16_t> image_index,
                  XRWebGLSwapChain* color_swap_chain,
                  XRWebGLSwapChain* depth_stencil_swap_chain,
                  XRWebGLSwapChain* motion_vector_swap_chain);

  WebGLTexture* colorTexture() const { return color_texture_.Get(); }
  WebGLTexture* depthStencilTexture() const {
    return depth_stencil_texture_.Get();
  }
  WebGLTexture* motionVectorTexture() const {
    return motion_vector_texture_.Get();
  }

  std::optional<uint16_t> imageIndex() const { return image_index_; }
  uint16_t colorTextureWidth() const { return color_texture_width_; }
  uint16_t colorTextureHeight() const { return color_texture_height_; }
  std::optional<uint16_t> depthStencilTextureWidth() const {
    return depth_stencil_texture_width_;
  }
  std::optional<uint16_t> depthStencilTextureHeight() const {
    return depth_stencil_texture_height_;
  }
  std::optional<uint16_t> motionVectorTextureWidth() const {
    return motion_vector_texture_width_;
  }
  std::optional<uint16_t> motionVectorTextureHeight() const {
    return motion_vector_texture_height_;
  }

  void Trace(Visitor* visitor) const override;

 private:
  Member<WebGLTexture> color_texture_{nullptr};
  Member<WebGLTexture> depth_stencil_texture_{nullptr};
  Member<WebGLTexture> motion_vector_texture_{nullptr};

  std::optional<uint16_t> image_index_{std::nullopt};
  uint16_t color_texture_width_{0};
  uint16_t color_texture_height_{0};
  std::optional<uint16_t> depth_stencil_texture_width_{std::nullopt};
  std::optional<uint16_t> depth_stencil_texture_height_{std::nullopt};
  std::optional<uint16_t> motion_vector_texture_width_{std::nullopt};
  std::optional<uint16_t> motion_vector_texture_height_{std::nullopt};
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_XR_XR_WEBGL_SUB_IMAGE_H_
