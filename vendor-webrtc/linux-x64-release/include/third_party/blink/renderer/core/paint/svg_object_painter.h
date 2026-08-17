// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_OBJECT_PAINTER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_OBJECT_PAINTER_H_

#include "cc/paint/paint_flags.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/paint/paint_info.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class AffineTransform;
class ComputedStyle;
class GraphicsContext;
struct SvgContextPaints;

enum LayoutSVGResourceMode {
  kApplyToFillMode,
  kApplyToStrokeMode,
};

class SVGObjectPainter {
  STACK_ALLOCATED();

 public:
  static bool HasVisibleStroke(const ComputedStyle&, const SvgContextPaints*);
  static bool HasFill(const ComputedStyle&, const SvgContextPaints*);

  SVGObjectPainter(const LayoutObject& layout_object,
                   const SvgContextPaints* context_paints)
      : layout_object_(layout_object), context_paints_(context_paints) {
    DCHECK(layout_object.IsSVG());
  }

  // Initializes |paint_flags| for painting an SVG object or a part of the
  // object. Returns true if successful, and the caller can continue to paint
  // using |paint_flags|.
  bool PreparePaint(
      PaintFlags,
      const ComputedStyle&,
      LayoutSVGResourceMode,
      cc::PaintFlags& paint_flags,
      const AffineTransform* additional_paint_server_transform = nullptr);

  void PaintResourceSubtree(GraphicsContext&,
                            PaintFlags additional_flags = PaintFlag::kNoFlag);

  SvgContextPaints::ContextPaint ResolveContextPaint(
      const SVGPaint& initial_paint);
  std::optional<AffineTransform> ResolveContextTransform(
      const SVGPaint& initial_paint,
      const AffineTransform* additional_paint_server_transform);

 private:
  const LayoutObject& layout_object_;
  const SvgContextPaints* context_paints_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_SVG_OBJECT_PAINTER_H_
