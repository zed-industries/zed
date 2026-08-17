// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BACKGROUND_COLOR_PAINT_IMAGE_GENERATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BACKGROUND_COLOR_PAINT_IMAGE_GENERATOR_IMPL_H_

#include "third_party/blink/renderer/core/css/background_color_paint_image_generator.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "ui/gfx/geometry/size_f.h"
#include "v8/include/v8.h"

namespace blink {

class Image;
class BackgroundColorPaintDefinition;

class MODULES_EXPORT BackgroundColorPaintImageGeneratorImpl final
    : public BackgroundColorPaintImageGenerator {
 public:
  static BackgroundColorPaintImageGenerator* Create(LocalFrame&);

  explicit BackgroundColorPaintImageGeneratorImpl(
      BackgroundColorPaintDefinition*);
  ~BackgroundColorPaintImageGeneratorImpl() override = default;

  // The |container_size| is without subpixel snapping.
  scoped_refptr<Image> Paint(const gfx::SizeF& container_size,
                             const Node*) final;

  Animation* GetAnimationIfCompositable(const Element* element) final;

  void Shutdown() final;

  void Trace(Visitor*) const override;

 private:
  Member<BackgroundColorPaintDefinition> background_color_paint_definition_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BACKGROUND_COLOR_PAINT_IMAGE_GENERATOR_IMPL_H_
