// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BACKGROUND_COLOR_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BACKGROUND_COLOR_PAINT_DEFINITION_H_

#include <optional>

#include "third_party/blink/renderer/core/animation/keyframe_effect_model.h"
#include "third_party/blink/renderer/modules/csspaint/nativepaint/native_css_paint_definition.h"
#include "third_party/blink/renderer/modules/modules_export.h"
#include "third_party/blink/renderer/platform/graphics/color.h"
#include "ui/gfx/geometry/size_f.h"

namespace blink {

class Image;
class LocalFrame;
class Node;

class MODULES_EXPORT BackgroundColorPaintDefinition final
    : public GarbageCollected<BackgroundColorPaintDefinition>,
      public NativeCssPaintDefinition {
  // Key strcut used for limiting access to the for testing default constructor.
  struct KeyForTest {};

 public:
  static BackgroundColorPaintDefinition* Create(LocalFrame&);
  explicit BackgroundColorPaintDefinition(LocalFrame&);
  ~BackgroundColorPaintDefinition() final = default;
  BackgroundColorPaintDefinition(const BackgroundColorPaintDefinition&) =
      delete;
  BackgroundColorPaintDefinition& operator=(
      const BackgroundColorPaintDefinition&) = delete;

  // PaintDefinition override
  PaintRecord Paint(
      const CompositorPaintWorkletInput*,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&) override;

  // The |container_size| is without subpixel snapping.
  scoped_refptr<Image> Paint(const gfx::SizeF& container_size, const Node*);

  static Animation* GetAnimationIfCompositable(const Element* element);

  void Trace(Visitor* visitor) const override;

  // Constructor for testing purpose only.
  explicit BackgroundColorPaintDefinition(KeyForTest) {}

 private:
  friend class BackgroundColorPaintDefinitionTest;

  PaintRecord PaintForTest(
      const Vector<Color>& animated_colors,
      const Vector<double>& offsets,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&
          animated_property_values);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_BACKGROUND_COLOR_PAINT_DEFINITION_H_
