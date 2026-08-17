/*
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_RESOURCES_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_RESOURCES_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"
#include "third_party/blink/renderer/core/style/style_difference.h"
#include "third_party/blink/renderer/core/svg/svg_resource_client.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace gfx {
class RectF;
}

namespace blink {

class CompositorFilterOperations;
class ComputedStyle;
class FilterEffectBuilder;
class LayoutObject;
class ReferenceFilterOperation;
class SVGElement;
class SVGElementResourceClient;

// Holds a set of resources associated with a LayoutObject
class SVGResources {
  STATIC_ONLY(SVGResources);

 public:
  static SVGElementResourceClient* GetClient(const LayoutObject&);

  // Control what reference box is returned by ReferenceBoxForEffects() for a
  // <foreignObject> element.
  enum class ForeignObjectQuirk {
    // The reference box for <foreignObject> will have <0, 0> as its
    // origin. This is to be used when the local space of the effect already
    // includes the paint offset (== 'x' and 'y' for a <foreignObject>), and it
    // isn't easy to undo that.
    kEnabled,

    // The reference box for <foreignObject> will be "correct" i.e include the
    // 'x' and 'y' from the element and can thus be different from <0, 0>. This
    // should be used if the local space does not already include this offset.
    kDisabled,
  };
  static gfx::RectF ReferenceBoxForEffects(
      const LayoutObject&,
      GeometryBox geometry_box = GeometryBox::kFillBox,
      ForeignObjectQuirk foreign_object_quirk = ForeignObjectQuirk::kEnabled);

  static void UpdateEffects(LayoutObject&,
                            StyleDifference,
                            const ComputedStyle* old_style);
  static void ClearEffects(const LayoutObject&);
  static void UpdatePaints(const LayoutObject&,
                           const ComputedStyle* old_style,
                           const ComputedStyle& style);
  static void ClearPaints(const LayoutObject&, const ComputedStyle* style);
  static void UpdateMarkers(const LayoutObject&,
                            const ComputedStyle* old_style);
  static void ClearMarkers(const LayoutObject&, const ComputedStyle* style);

 private:
  static SVGElementResourceClient& EnsureClient(const LayoutObject&);
};

class SVGElementResourceClient final
    : public GarbageCollected<SVGElementResourceClient>,
      public SVGResourceClient {
 public:
  explicit SVGElementResourceClient(SVGElement*);

  void ResourceContentChanged(SVGResource*) override;

  void FilterPrimitiveChanged(SVGResource*,
                              SVGFilterPrimitiveStandardAttributes& primitive,
                              const QualifiedName& attribute) override;

  void UpdateFilterData(CompositorFilterOperations&);
  void InvalidateFilterData();
  void MarkFilterDataDirty();

  void Trace(Visitor*) const override;

 private:
  class FilterData;

  static FilterData* CreateFilterDataWithNodeMap(
      FilterEffectBuilder&,
      const ReferenceFilterOperation&);

  Member<SVGElement> element_;
  Member<FilterData> filter_data_;
  bool filter_data_dirty_;
};

// Helper class for handling invalidation of resources (generally after the
// reference box of a LayoutObject may have changed).
class SVGResourceInvalidator {
  STACK_ALLOCATED();

 public:
  explicit SVGResourceInvalidator(LayoutObject& object);

  // Invalidate any associated clip-path/mask/filter.
  void InvalidateEffects();

  // Invalidate any associated paints (fill/stroke).
  void InvalidatePaints();

 private:
  LayoutObject& object_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_SVG_RESOURCES_H_
