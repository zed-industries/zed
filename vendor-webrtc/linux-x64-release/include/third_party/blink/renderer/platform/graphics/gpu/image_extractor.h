// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_IMAGE_EXTRACTOR_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_IMAGE_EXTRACTOR_H_

#include "base/memory/stack_allocated.h"
#include "third_party/blink/renderer/platform/graphics/image.h"
#include "third_party/blink/renderer/platform/platform_export.h"

namespace blink {
class Image;

class PLATFORM_EXPORT ImageExtractor final {
  STACK_ALLOCATED();

 public:
  // Extract an SkImage from an Image. If the alpha channel will ultimately
  // be premultiplied, then `premultiply_alpha` should be true. If the color
  // space of image is to be ignored then `target_color_space` is to be
  // nullptr. Otherwise, `target_color_space` should be set to the color space
  // that the image will ultimately be converted to.
  ImageExtractor(Image*,
                 SkAlphaType target_alpha_type,
                 sk_sp<SkColorSpace> target_color_space);
  ImageExtractor(const ImageExtractor&) = delete;
  ImageExtractor& operator=(const ImageExtractor&) = delete;

  sk_sp<SkImage> GetSkImage() { return sk_image_; }

 private:
  sk_sp<SkImage> sk_image_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_GPU_IMAGE_EXTRACTOR_H_
