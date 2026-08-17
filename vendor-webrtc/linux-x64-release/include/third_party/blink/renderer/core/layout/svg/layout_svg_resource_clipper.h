/*
 * Copyright (C) Research In Motion Limited 2009-2010. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_CLIPPER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_CLIPPER_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/svg/layout_svg_resource_container.h"
#include "third_party/blink/renderer/core/style/reference_clip_path_operation.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"

namespace blink {

class SVGClipPathElement;

class LayoutSVGResourceClipper final : public LayoutSVGResourceContainer {
 public:
  explicit LayoutSVGResourceClipper(SVGClipPathElement*);
  ~LayoutSVGResourceClipper() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGResourceClipper";
  }

  void RemoveAllClientsFromCache() override;

  gfx::RectF ResourceBoundingBox(const gfx::RectF& reference_box);

  static const LayoutSVGResourceType kResourceType = kClipperResourceType;
  LayoutSVGResourceType ResourceType() const override {
    NOT_DESTROYED();
    return kResourceType;
  }

  bool HitTestClipContent(const gfx::RectF& reference_box,
                          const LayoutObject& reference_box_object,
                          const HitTestLocation&) const;

  SVGUnitTypes::SVGUnitType ClipPathUnits() const;
  AffineTransform CalculateClipTransform(const gfx::RectF& reference_box) const;

  std::optional<Path> AsPath();
  PaintRecord CreatePaintRecord();

 private:
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  void CalculateLocalClipBounds();
  bool FindCycleFromSelf() const override;

  // Cache of the clip path when using path clipping.
  enum ClipContentPathValidity {
    kClipContentPathUnknown,
    kClipContentPathValid,
    kClipContentPathInvalid
  } clip_content_path_validity_ = kClipContentPathUnknown;
  Path clip_content_path_;

  // Cache of the clip path paint record when falling back to masking for
  // clipping.
  std::optional<PaintRecord> cached_paint_record_;

  gfx::RectF local_clip_bounds_;
};

template <>
struct DowncastTraits<LayoutSVGResourceClipper> {
  static bool AllowFrom(const LayoutSVGResourceContainer& container) {
    return container.ResourceType() == kClipperResourceType;
  }
};

inline LayoutSVGResourceClipper* GetSVGResourceAsType(
    SVGResourceClient& client,
    const ReferenceClipPathOperation& reference_clip) {
  return GetSVGResourceAsType<LayoutSVGResourceClipper>(
      client, reference_clip.Resource());
}

inline LayoutSVGResourceClipper* GetSVGResourceAsType(
    SVGResourceClient& client,
    const ClipPathOperation* clip_path_operation) {
  const auto* reference_clip =
      DynamicTo<ReferenceClipPathOperation>(clip_path_operation);
  if (!reference_clip)
    return nullptr;
  return GetSVGResourceAsType(client, *reference_clip);
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_CLIPPER_H_
