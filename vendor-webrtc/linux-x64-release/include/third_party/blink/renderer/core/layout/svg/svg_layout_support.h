/**
 * Copyright (C) 2007 Rob Buis <buis@kde.org>
 * Copyright (C) 2007 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
 * Copyright (C) 2009 Google, Inc.  All rights reserved.
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
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
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_LAYOUT_SUPPORT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_LAYOUT_SUPPORT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/platform/geometry/dash_array.h"
#include "third_party/blink/renderer/platform/transforms/affine_transform.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class PointF;
class RectF;
}  // namespace gfx

namespace blink {

class ComputedStyle;
class LayoutBoxModelObject;
class SVGViewportResolver;
class StrokeData;
class TransformState;

class CORE_EXPORT SVGLayoutSupport {
  STATIC_ONLY(SVGLayoutSupport);

 public:
  // Helper function determining whether overflow is hidden.
  static bool IsOverflowHidden(const LayoutObject&);
  static bool IsOverflowHidden(const ComputedStyle&);

  // Adjusts the visual rect with clipper and masker in local coordinates.
  static void AdjustWithClipPathAndMask(const LayoutObject& layout_object,
                                        const gfx::RectF& object_bounding_box,
                                        gfx::RectF& visual_rect);

  // Add any contribution from 'stroke' to a text content bounding rect.
  static gfx::RectF ExtendTextBBoxWithStroke(const LayoutObject&,
                                             const gfx::RectF& text_bounds);

  // Compute the visual rect for the a text content LayoutObject.
  static gfx::RectF ComputeVisualRectForText(const LayoutObject&,
                                             const gfx::RectF& text_bounds);

  // Important functions used by nearly all SVG layoutObjects centralizing
  // coordinate transformations / visual rect calculations
  static gfx::RectF LocalVisualRect(const LayoutObject&);
  static PhysicalRect VisualRectInAncestorSpace(
      const LayoutObject&,
      const LayoutBoxModelObject& ancestor,
      VisualRectFlags = kDefaultVisualRectFlags);
  static bool MapToVisualRectInAncestorSpace(
      const LayoutObject&,
      const LayoutBoxModelObject* ancestor,
      const gfx::RectF& local_visual_rect,
      PhysicalRect& result_rect,
      VisualRectFlags = kDefaultVisualRectFlags);
  static void MapLocalToAncestor(const LayoutObject*,
                                 const LayoutBoxModelObject* ancestor,
                                 TransformState&,
                                 MapCoordinatesFlags);
  static void MapAncestorToLocal(const LayoutObject&,
                                 const LayoutBoxModelObject* ancestor,
                                 TransformState&,
                                 MapCoordinatesFlags);

  // Shared between SVG layoutObjects and resources.
  static void ApplyStrokeStyleToStrokeData(StrokeData&,
                                           const ComputedStyle&,
                                           const LayoutObject&,
                                           float dash_scale_factor);

  static DashArray ResolveSVGDashArray(const SVGDashArray&,
                                       const ComputedStyle&,
                                       const SVGViewportResolver&);

  // Helper method for determining if a LayoutObject marked as text (isText()==
  // true) can/will be laid out as part of a <text>.
  static bool IsLayoutableTextNode(const LayoutObject*);

  // Determines whether a svg node should isolate or not based on ComputedStyle.
  static bool WillIsolateBlendingDescendantsForStyle(const ComputedStyle&);
  static bool WillIsolateBlendingDescendantsForObject(const LayoutObject*);
  static bool IsIsolationRequired(const LayoutObject*);

  static float CalculateScreenFontSizeScalingFactor(const LayoutObject*);

  // This returns a LayoutSVGText or nullptr.
  static LayoutObject* FindClosestLayoutSVGText(const LayoutObject*,
                                                const gfx::PointF&);
};

class SubtreeContentTransformScope {
  STACK_ALLOCATED();

 public:
  SubtreeContentTransformScope(const AffineTransform&);
  ~SubtreeContentTransformScope();

  static const AffineTransform& CurrentContentTransformation() {
    return current_content_transformation_;
  }

 private:
  static AffineTransform current_content_transformation_;
  AffineTransform saved_content_transformation_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_LAYOUT_SUPPORT_H_
