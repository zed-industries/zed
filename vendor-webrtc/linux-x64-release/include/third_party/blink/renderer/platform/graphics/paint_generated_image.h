// Copyright 2016 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GENERATED_IMAGE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GENERATED_IMAGE_H_

#include "third_party/blink/renderer/platform/graphics/generated_image.h"
#include "third_party/blink/renderer/platform/graphics/paint/paint_record.h"

namespace blink {

class PLATFORM_EXPORT PaintGeneratedImage : public GeneratedImage {
 public:
  static scoped_refptr<PaintGeneratedImage> Create(PaintRecord record,
                                                   const gfx::SizeF& size) {
    return base::AdoptRef(new PaintGeneratedImage(std::move(record), size));
  }
  ~PaintGeneratedImage() override = default;

 protected:
  void Draw(cc::PaintCanvas*,
            const cc::PaintFlags&,
            const gfx::RectF&,
            const gfx::RectF&,
            const ImageDrawOptions& draw_options) override;
  void DrawTile(cc::PaintCanvas*,
                const gfx::RectF&,
                const ImageDrawOptions&) final;

  PaintGeneratedImage(PaintRecord record, const gfx::SizeF& size)
      : GeneratedImage(size), record_(std::move(record)) {}

  PaintRecord record_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_PAINT_GENERATED_IMAGE_H_
