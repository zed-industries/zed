/*
 * Copyright (C) 2009 Dirk Schulze <krit@webkit.org>
 * Copyright (C) 2013 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FILTER_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FILTER_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/platform_export.h"
#include "ui/gfx/geometry/point3_f.h"
#include "ui/gfx/geometry/rect_f.h"

namespace blink {

class SourceGraphic;
class FilterEffect;

class PLATFORM_EXPORT Filter final : public GarbageCollected<Filter> {

 public:
  enum UnitScaling { kUserSpace, kBoundingBox };

  explicit Filter(float scale);
  Filter(const gfx::RectF& reference_box,
         const gfx::RectF& filter_region,
         float scale,
         UnitScaling);
  Filter(const Filter&) = delete;
  Filter& operator=(const Filter&) = delete;

  void Trace(Visitor*) const;

  float Scale() const { return scale_; }
  gfx::RectF MapLocalRectToAbsoluteRect(const gfx::RectF&) const;
  gfx::RectF MapAbsoluteRectToLocalRect(const gfx::RectF&) const;

  float ApplyHorizontalScale(float value) const;
  float ApplyVerticalScale(float value) const;

  gfx::Point3F Resolve3dPoint(gfx::Point3F) const;

  const gfx::RectF& FilterRegion() const { return filter_region_; }
  const gfx::RectF& ReferenceBox() const { return reference_box_; }

  void SetLastEffect(FilterEffect*);
  FilterEffect* LastEffect() const { return last_effect_.Get(); }

  SourceGraphic* GetSourceGraphic() const { return source_graphic_.Get(); }

 private:
  gfx::RectF reference_box_;
  gfx::RectF filter_region_;
  float scale_;
  UnitScaling unit_scaling_;

  Member<SourceGraphic> source_graphic_;
  Member<FilterEffect> last_effect_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_GRAPHICS_FILTERS_FILTER_H_
