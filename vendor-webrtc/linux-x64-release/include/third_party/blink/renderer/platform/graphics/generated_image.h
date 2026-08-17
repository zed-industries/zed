/*
 * Copyright (C) 2008 Apple Computer, Inc.  All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE COMPUTER, INC. ``AS IS'' AND ANY
 * EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR
 * PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL APPLE COMPUTER, INC. OR
 * CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
 * EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
 * PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
 * PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY
 * OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GENERATED_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GENERATED_IMAGE_H_

#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/skia/include/core/SkRefCnt.h"
#include "ui/gfx/geometry/size_conversions.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class PLATFORM_EXPORT GeneratedImage : public Image {
 public:
  bool CurrentFrameHasSingleSecurityOrigin() const override { return true; }

  bool HasIntrinsicSize() const override { return false; }

  gfx::Size SizeWithConfig(SizeConfig) const override {
    return gfx::ToRoundedSize(size_);
  }
  gfx::SizeF SizeWithConfigAsFloat(SizeConfig) const override { return size_; }

  // Assume that generated content has no decoded data we need to worry about
  void DestroyDecodedData() override {}

  PaintImage PaintImageForCurrentFrame() override;

 protected:
  void DrawPattern(GraphicsContext&,
                   const cc::PaintFlags&,
                   const gfx::RectF& dest_rect,
                   const ImageTilingInfo&,
                   const ImageDrawOptions& draw_options) final;

  // Implementation hook for the `DrawPattern()` implementation. `tile_rect` is
  // a single tile rectangle including any spacing. `pattern_matrix` contains
  // the transform from tile space to destination space. `src_rect` is the
  // rectangle containing actual content (`tile_rect` minus any spacing).
  //
  // Provide an implementation of this for a subclass if it can generate a more
  // efficient PaintShader than the default PaintRecord-based shader. If this
  // is overridden, then the `DrawTile()` implementation can be empty since it
  // won't be used.
  virtual sk_sp<cc::PaintShader> CreateShader(
      const gfx::RectF& tile_rect,
      const SkMatrix* pattern_matrix,
      const gfx::RectF& src_rect,
      const ImageDrawOptions& draw_options);

  // FIXME: Implement this to be less conservative.
  bool CurrentFrameKnownToBeOpaque() override { return false; }

  GeneratedImage(const gfx::SizeF& size) : size_(size) {}

  // Implementation hook for `CreateShader()`. Is passed a source rectangle
  // (see `CreateShader()` above) that should be painted onto the provided
  // PaintCanvas.
  virtual void DrawTile(cc::PaintCanvas*,
                        const gfx::RectF&,
                        const ImageDrawOptions&) = 0;

  gfx::SizeF size_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GENERATED_IMAGE_H_
