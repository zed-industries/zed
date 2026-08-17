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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_MASKER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_MASKER_H_

#include <optional>

#include "third_party/blink/renderer/core/layout/svg/layout_svg_resource_container.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class SVGMaskElement;

class LayoutSVGResourceMasker final : public LayoutSVGResourceContainer {
 public:
  explicit LayoutSVGResourceMasker(SVGMaskElement*);
  ~LayoutSVGResourceMasker() override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGResourceMasker";
  }

  void RemoveAllClientsFromCache() override;

  gfx::RectF ResourceBoundingBox(const gfx::RectF& reference_box,
                                 float reference_box_zoom);

  SVGUnitTypes::SVGUnitType MaskUnits() const;
  SVGUnitTypes::SVGUnitType MaskContentUnits() const;

  static const LayoutSVGResourceType kResourceType = kMaskerResourceType;
  LayoutSVGResourceType ResourceType() const override {
    NOT_DESTROYED();
    return kResourceType;
  }

  PaintRecord CreatePaintRecord();

 private:
  std::optional<PaintRecord> cached_paint_record_;
};

template <>
struct DowncastTraits<LayoutSVGResourceMasker> {
  static bool AllowFrom(const LayoutSVGResourceContainer& container) {
    return container.ResourceType() == kMaskerResourceType;
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_MASKER_H_
