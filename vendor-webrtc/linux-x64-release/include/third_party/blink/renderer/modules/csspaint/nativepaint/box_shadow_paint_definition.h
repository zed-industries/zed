// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BOX_SHADOW_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BOX_SHADOW_PAINT_DEFINITION_H_

#include "third_party/blink/renderer/core/animation/animation.h"
#include "third_party/blink/renderer/core/dom/element.h"
#include "third_party/blink/renderer/modules/csspaint/nativepaint/native_css_paint_definition.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "third_party/blink/renderer/platform/graphics/image.h"

namespace blink {

class LocalFrame;

class MODULES_EXPORT BoxShadowPaintDefinition final
    : public GarbageCollected<BoxShadowPaintDefinition>,
      public NativeCssPaintDefinition {
 public:
  static BoxShadowPaintDefinition* Create(LocalFrame& local_root);

  explicit BoxShadowPaintDefinition(LocalFrame& local_root);
  ~BoxShadowPaintDefinition() final = default;
  BoxShadowPaintDefinition(const BoxShadowPaintDefinition&) = delete;
  BoxShadowPaintDefinition& operator=(const BoxShadowPaintDefinition&) = delete;

  static Animation* GetAnimationIfCompositable(const Element* element);

  // PaintDefinition override
  PaintRecord Paint(
      const CompositorPaintWorkletInput*,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&) override;

  scoped_refptr<Image> Paint();

  void Trace(Visitor* visitor) const override;

 private:
  friend class BoxShadowPaintDefinitionTest;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BOX_SHADOW_PAINT_DEFINITION_H_
