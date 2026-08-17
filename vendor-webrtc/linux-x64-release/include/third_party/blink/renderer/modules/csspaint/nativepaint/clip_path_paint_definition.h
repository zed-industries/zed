// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_CLIP_PATH_PAINT_DEFINITION_H_
#define THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_CLIP_PATH_PAINT_DEFINITION_H_

#include "third_party/blink/renderer/modules/csspaint/nativepaint/native_css_paint_definition.h"
#include "third_party/blink/renderer/modules/modules_export.h"

namespace gfx {
class RectF;
}

namespace blink {

class Animation;
class Image;
class LocalFrame;
class Node;

class MODULES_EXPORT ClipPathPaintDefinition final
    : public GarbageCollected<ClipPathPaintDefinition>,
      public NativeCssPaintDefinition {
 public:
  static ClipPathPaintDefinition* Create(LocalFrame& local_root);

  explicit ClipPathPaintDefinition(LocalFrame& local_root);
  ~ClipPathPaintDefinition() final = default;
  ClipPathPaintDefinition(const ClipPathPaintDefinition&) = delete;
  ClipPathPaintDefinition& operator=(const ClipPathPaintDefinition&) = delete;

  // PaintDefinition override
  PaintRecord Paint(
      const CompositorPaintWorkletInput*,
      const CompositorPaintWorkletJob::AnimatedPropertyValues&) override;

  // static method that accepts a worklet id, for use in tests. The instance
  // version of this method calls this
  static scoped_refptr<Image> Paint(float zoom,
                                    const gfx::RectF& reference_box,
                                    const gfx::SizeF& clip_area_size,
                                    const Node&,
                                    int worklet_id);
  static Animation* GetAnimationIfCompositable(const Element* element);
  void Trace(Visitor* visitor) const override;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_MODULES_CSSPAINT_NATIVEPAINT_CLIP_PATH_PAINT_DEFINITION_H_
