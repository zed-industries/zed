// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BACKGROUND_COLOR_PAINT_IMAGE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BACKGROUND_COLOR_PAINT_IMAGE_GENERATOR_H_

#include <optional>

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/native_paint_image_generator.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/skia/include/core/SkColor.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;
class LocalFrame;
class Node;

class CORE_EXPORT BackgroundColorPaintImageGenerator
    : public NativePaintImageGenerator {
 public:
  static BackgroundColorPaintImageGenerator* Create(LocalFrame&);

  ~BackgroundColorPaintImageGenerator() override = default;

  typedef BackgroundColorPaintImageGenerator* (
      *BackgroundColorPaintImageGeneratorCreateFunction)(LocalFrame&);
  static void Init(
      BackgroundColorPaintImageGeneratorCreateFunction create_function);

  virtual scoped_refptr<Image> Paint(const gfx::SizeF& container_size,
                                     const Node*) = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BACKGROUND_COLOR_PAINT_IMAGE_GENERATOR_H_
