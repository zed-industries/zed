// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BOX_SHADOW_PAINT_IMAGE_GENERATOR_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BOX_SHADOW_PAINT_IMAGE_GENERATOR_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/css/native_paint_image_generator.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;
class LocalFrame;

class CORE_EXPORT BoxShadowPaintImageGenerator
    : public NativePaintImageGenerator {
 public:
  static BoxShadowPaintImageGenerator* Create(LocalFrame& local_root);

  ~BoxShadowPaintImageGenerator() override = default;

  using BoxShadowPaintImageGeneratorCreateFunction =
      BoxShadowPaintImageGenerator*(LocalFrame&);
  static void Init(BoxShadowPaintImageGeneratorCreateFunction* create_function);

  virtual scoped_refptr<Image> Paint() = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_BOX_SHADOW_PAINT_IMAGE_GENERATOR_H_
