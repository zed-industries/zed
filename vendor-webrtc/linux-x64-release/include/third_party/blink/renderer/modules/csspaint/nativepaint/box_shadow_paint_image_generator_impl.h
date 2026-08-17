// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BOX_SHADOW_PAINT_IMAGE_GENERATOR_IMPL_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BOX_SHADOW_PAINT_IMAGE_GENERATOR_IMPL_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/css/box_shadow_paint_image_generator.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;
class BoxShadowPaintDefinition;

class MODULES_EXPORT BoxShadowPaintImageGeneratorImpl final
    : public BoxShadowPaintImageGenerator {
 public:
  static BoxShadowPaintImageGenerator* Create(LocalFrame&);

  explicit BoxShadowPaintImageGeneratorImpl(BoxShadowPaintDefinition*);
  ~BoxShadowPaintImageGeneratorImpl() override = default;

  scoped_refptr<Image> Paint() final;

  Animation* GetAnimationIfCompositable(const Element* element) final;

  void Shutdown() final;

  void Trace(Visitor*) const override;

 private:
  Member<BoxShadowPaintDefinition> box_shadow_paint_definition_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BOX_SHADOW_PAINT_IMAGE_GENERATOR_IMPL_H_
