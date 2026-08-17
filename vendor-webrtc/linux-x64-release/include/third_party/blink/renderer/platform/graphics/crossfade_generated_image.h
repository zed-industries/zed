/*
 * Copyright (C) 2011 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CROSSFADE_GENERATED_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CROSSFADE_GENERATED_IMAGE_H_

#include "base/memory/scoped_refptr.h"
#include "third_party/blink/renderer/platform/graphics/generated_image.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "ui/gfx/geometry/size_conversions.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class PLATFORM_EXPORT CrossfadeGeneratedImage final : public GeneratedImage {
 public:
  struct WeightedImage {
    scoped_refptr<Image> image;
    float weight;  // Typically [0..1].
  };

  static scoped_refptr<CrossfadeGeneratedImage> Create(
      Vector<WeightedImage> images,
      const gfx::SizeF& size) {
    return base::AdoptRef(new CrossfadeGeneratedImage(std::move(images), size));
  }

  bool HasIntrinsicSize() const override { return true; }

  gfx::Size SizeWithConfig(SizeConfig) const override {
    return gfx::ToFlooredSize(size_);
  }

 protected:
  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF&,
            const gfx::RectF&,
            const ImageDrawOptions& draw_options) override;
  void DrawTile(cc::PaintCanvas*,
                const gfx::RectF&,
                const ImageDrawOptions&) final;

  CrossfadeGeneratedImage(Vector<WeightedImage> images, const gfx::SizeF&);

 private:
  void DrawCrossfade(cc::PaintCanvas*,
                     const cc::PaintFlags&,
                     const ImageDrawOptions&);

  Vector<WeightedImage> images_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_CROSSFADE_GENERATED_IMAGE_H_
