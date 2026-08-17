// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_BACKGROUND_PAINT_CONTEXT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_BACKGROUND_PAINT_CONTEXT_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class RectF;
}  // namespace gfx

namespace blink {

class ComputedStyle;
class LayoutObject;

enum class GeometryBox;

class SVGBackgroundPaintContext {
  STACK_ALLOCATED();

 public:
  explicit SVGBackgroundPaintContext(const LayoutObject&);

  gfx::RectF VisualOverflowRect() const;
  gfx::RectF ReferenceBox(GeometryBox) const;
  const ComputedStyle& Style() const;

 private:
  const LayoutObject& object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_BACKGROUND_PAINT_CONTEXT_H_
