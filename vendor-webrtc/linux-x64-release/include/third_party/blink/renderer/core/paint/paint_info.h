/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 *           (C) 2004 Allan Sandfeld Jensen (kde@carewolf.com)
 * Copyright (C) 2003, 2004, 2005, 2006, 2007, 2008, 2009 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2009 Google Inc. All rights reserved.
 *
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Library General Public
 * License as published by the Free Software Foundation; either
 * version 2 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this library; see the file COPYING.LIB.  If not, write to
 * the Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
 * Boston, MA 02110-1301, USA.
 *
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_INFO_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_INFO_H_

#include "base/check_op.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_box.h"
#include "third_party/blink/renderer/core/layout/physical_box_fragment.h"
#include "third_party/blink/renderer/core/paint/paint_flags.h"
#include "third_party/blink/renderer/core/paint/paint_phase.h"
#include "third_party/blink/renderer/platform/graphics/graphics_context.h"
#include "third_party/blink/renderer/platform/graphics/paint/cull_rect.h"
#include "third_party/blink/renderer/platform/graphics/paint/display_item.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "ui/gfx/geometry/rect.h"

namespace blink {

// To support context-fill and context-stroke:
//   https://svgwg.org/svg2-draft/painting.html#context-paint
struct CORE_EXPORT SvgContextPaints {
  STACK_ALLOCATED();

 public:
  struct CORE_EXPORT ContextPaint {
    STACK_ALLOCATED();

   public:
    ContextPaint(const LayoutObject& o, const SVGPaint& p)
        : object(o), paint(p) {}
    ContextPaint(const ContextPaint&) = default;
    ContextPaint(ContextPaint&&) = default;

    const LayoutObject& object;
    SVGPaint paint;
  };

  SvgContextPaints(const ContextPaint& f, const ContextPaint& s)
      : fill(f), stroke(s) {}
  SvgContextPaints(const ContextPaint& f,
                   const ContextPaint& s,
                   const AffineTransform& t)
      : fill(f), stroke(s), transform(t) {}
  SvgContextPaints(const SvgContextPaints&) = default;

  ContextPaint fill;
  ContextPaint stroke;
  AffineTransform transform;
};

struct CORE_EXPORT PaintInfo {
  STACK_ALLOCATED();

 public:
  PaintInfo(GraphicsContext& context,
            const CullRect& cull_rect,
            PaintPhase phase,
            bool descendant_painting_blocked,
            PaintFlags paint_flags = PaintFlag::kNoFlag,
            const SvgContextPaints* context_paints = nullptr)
      : context(context),
        phase(phase),
        cull_rect_(cull_rect),
        svg_context_paints_(context_paints),
        paint_flags_(paint_flags),
        descendant_painting_blocked_(descendant_painting_blocked) {}

  // Creates a PaintInfo for painting descendants. See comments about the paint
  // phases in PaintPhase.h for details.
  PaintInfo ForDescendants() const {
    PaintInfo result(*this);

    // We should never start to paint descendant when the flag is set.
    DCHECK(!result.is_painting_background_in_contents_space);

    if (phase == PaintPhase::kDescendantOutlinesOnly)
      result.phase = PaintPhase::kOutline;
    else if (phase == PaintPhase::kDescendantBlockBackgroundsOnly)
      result.phase = PaintPhase::kBlockBackground;

    result.fragment_data_override_ = nullptr;

    return result;
  }

  bool ShouldOmitCompositingInfo() const {
    return paint_flags_ & PaintFlag::kOmitCompositingInfo;
  }

  bool IsRenderingClipPathAsMaskImage() const {
    return paint_flags_ & PaintFlag::kPaintingClipPathAsMask;
  }
  bool IsRenderingResourceSubtree() const {
    return paint_flags_ & PaintFlag::kPaintingResourceSubtree;
  }

  bool ShouldSkipBackground() const { return skips_background_; }
  void SetSkipsBackground(bool b) { skips_background_ = b; }

  bool ShouldAddUrlMetadata() const {
    return paint_flags_ & PaintFlag::kAddUrlMetadata;
  }

  DisplayItem::Type DisplayItemTypeForClipping() const {
    return DisplayItem::PaintPhaseToClipType(phase);
  }

  PaintFlags GetPaintFlags() const { return paint_flags_; }

  const CullRect& GetCullRect() const { return cull_rect_; }
  void SetCullRect(const CullRect& cull_rect) { cull_rect_ = cull_rect; }

  bool IntersectsCullRect(
      const PhysicalRect& rect,
      const PhysicalOffset& offset = PhysicalOffset()) const {
    return cull_rect_.Intersects(
        ToEnclosingRect(PhysicalRect(rect.offset + offset, rect.size)));
  }

  void ApplyInfiniteCullRect() { cull_rect_ = CullRect::Infinite(); }

  void TransformCullRect(const TransformPaintPropertyNode& transform) {
    cull_rect_.ApplyTransform(transform);
  }

  void SetFragmentDataOverride(const FragmentData* fragment_data) {
    fragment_data_override_ = fragment_data;
  }
  const FragmentData* FragmentDataOverride() const {
    return fragment_data_override_;
  }

  const SvgContextPaints* GetSvgContextPaints() const {
    return svg_context_paints_;
  }
  void SetSvgContextPaints(const SvgContextPaints* context_paints) {
    svg_context_paints_ = context_paints;
  }

  bool IsPaintingBackgroundInContentsSpace() const {
    return is_painting_background_in_contents_space;
  }
  void SetIsPaintingBackgroundInContentsSpace(bool b) {
    is_painting_background_in_contents_space = b;
  }

  bool DescendantPaintingBlocked() const {
    return descendant_painting_blocked_;
  }
  void SetDescendantPaintingBlocked() { descendant_painting_blocked_ = true; }

  GraphicsContext& context;
  PaintPhase phase;

 private:
  CullRect cull_rect_;

  // Only set when entering legacy painters. Legacy painters are only used for
  // certain types of monolithic content, but there may still be multiple
  // fragments in such cases, due to repeated table headers/footers or repeated
  // fixed positioned objects when printing. The correct FragmentData is
  // typically obtained via an PhysicalBoxFragment object, but there are no
  // physical fragments passed to legacy painters.
  const FragmentData* fragment_data_override_ = nullptr;

  // This holds references to the SVGPaint values from an ancestor <use> or
  // LayoutSVGResourceMarker that are used when a descendant specifies
  // context-fill and/or context-paint paint values.
  const SvgContextPaints* svg_context_paints_ = nullptr;

  const PaintFlags paint_flags_;

  bool is_painting_background_in_contents_space = false;
  bool skips_background_ = false;
  bool descendant_painting_blocked_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_PAINT_PAINT_INFO_H_
