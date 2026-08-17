// Copyright 2014 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_PATTERN_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_PATTERN_H_

#include "third_party/blink/renderer/platform/graphics/paint/paint_image.h"
#include "third_party/blink/renderer/platform/graphics/pattern.h"

namespace blink {

class Image;

class PLATFORM_EXPORT ImagePattern final : public Pattern {
 public:
  static scoped_refptr<ImagePattern> Create(scoped_refptr<Image>, RepeatMode);

  bool IsTextureBacked() const override;

 protected:
  sk_sp<PaintShader> CreateShader(const SkMatrix&) const override;

 private:
  ImagePattern(scoped_refptr<Image>, RepeatMode);

  PaintImage tile_image_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_IMAGE_PATTERN_H_
