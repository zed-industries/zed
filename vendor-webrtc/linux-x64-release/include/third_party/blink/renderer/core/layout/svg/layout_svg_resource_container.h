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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_CONTAINER_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_CONTAINER_H_

#include "third_party/blink/renderer/core/layout/svg/layout_svg_hidden_container.h"
#include "third_party/blink/renderer/core/style/style_svg_resource.h"
#include "third_party/blink/renderer/core/svg/svg_resource.h"
#include "third_party/blink/renderer/core/svg/svg_resource_client.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"

namespace blink {

class SVGLength;
class SVGLengthConversionData;
class SVGViewportResolver;

enum LayoutSVGResourceType {
  kMaskerResourceType,
  kMarkerResourceType,
  kPatternResourceType,
  kLinearGradientResourceType,
  kRadialGradientResourceType,
  kFilterResourceType,
  kClipperResourceType
};

class LayoutSVGResourceContainer : public LayoutSVGHiddenContainer {
 public:
  explicit LayoutSVGResourceContainer(SVGElement*);
  ~LayoutSVGResourceContainer() override;

  virtual void RemoveAllClientsFromCache() = 0;

  // Like RemoveAllClientsFromCache(), but predicated on if layout has been
  // performed at all.
  void InvalidateCache();

  // Remove any cached data for the |client|, and return true if so.
  virtual bool RemoveClientFromCache(SVGResourceClient&) {
    NOT_DESTROYED();
    return false;
  }

  SVGLayoutResult UpdateSVGLayout(const SVGLayoutInfo&) override;
  bool IsSVGResourceContainer() const final {
    NOT_DESTROYED();
    return true;
  }

  virtual LayoutSVGResourceType ResourceType() const = 0;

  bool IsSVGPaintServer() const {
    NOT_DESTROYED();
    LayoutSVGResourceType resource_type = ResourceType();
    return resource_type == kPatternResourceType ||
           resource_type == kLinearGradientResourceType ||
           resource_type == kRadialGradientResourceType;
  }

  bool FindCycle() const;

  static void InvalidateDependentElements(LayoutObject&, bool needs_layout);
  static void InvalidateAncestorChainResources(LayoutObject&,
                                               bool needs_layout);
  static void MarkForLayoutAndParentResourceInvalidation(
      LayoutObject&,
      bool needs_layout = true);
  static void StyleChanged(LayoutObject&, StyleDifference);

  void ClearInvalidationMask() {
    NOT_DESTROYED();
    completed_invalidations_mask_ = 0;
  }

  // Resolve the rectangle defined by `x`, `y`, `width` and `height` in the
  // unit space defined by `type` into user units.
  static gfx::RectF ResolveRectangle(
      const SVGViewportResolver&,
      const SVGLengthConversionData&,
      SVGUnitTypes::SVGUnitType type,
      const gfx::RectF& reference_box,
      const SVGLength& x,
      const SVGLength& y,
      const SVGLength& width,
      const SVGLength& height,
      const std::optional<gfx::SizeF>& override_viewport = std::nullopt);
  static gfx::RectF ResolveRectangle(
      const SVGElement& context,
      SVGUnitTypes::SVGUnitType type,
      const gfx::RectF& reference_box,
      const SVGLength& x,
      const SVGLength& y,
      const SVGLength& width,
      const SVGLength& height,
      const std::optional<gfx::SizeF>& override_viewport = std::nullopt);
  // Like the above, but pass `x()`, `y()`, `width()` and `height()` from the
  // context element for the corresponding arguments.
  template <typename T>
  static gfx::RectF ResolveRectangle(
      const T& context,
      SVGUnitTypes::SVGUnitType type,
      const gfx::RectF& reference_box,
      const std::optional<gfx::SizeF>& override_viewport = std::nullopt) {
    return ResolveRectangle(
        context, type, reference_box, *context.x()->CurrentValue(),
        *context.y()->CurrentValue(), *context.width()->CurrentValue(),
        *context.height()->CurrentValue(), override_viewport);
  }

  gfx::RectF ResolveRectangle(SVGUnitTypes::SVGUnitType type,
                              const gfx::RectF& reference_box,
                              const SVGLength& x,
                              const SVGLength& y,
                              const SVGLength& width,
                              const SVGLength& height) const;

 protected:
  typedef unsigned InvalidationModeMask;

  // When adding modes, make sure we don't overflow
  // |completed_invalidation_mask_|.
  enum InvalidationMode {
    kLayoutInvalidation = 1 << 0,
    kBoundariesInvalidation = 1 << 1,
    kPaintInvalidation = 1 << 2,
    kPaintPropertiesInvalidation = 1 << 3,
    kClipCacheInvalidation = 1 << 4,
    kFilterCacheInvalidation = 1 << 5,
    kInvalidateAll = kLayoutInvalidation | kBoundariesInvalidation |
                     kPaintInvalidation | kPaintPropertiesInvalidation |
                     kClipCacheInvalidation | kFilterCacheInvalidation,
  };

  // Used from RemoveAllClientsFromCache methods.
  void MarkAllClientsForInvalidation(InvalidationModeMask);

  virtual bool FindCycleFromSelf() const;
  static bool FindCycleInDescendants(const LayoutObject& root);
  static bool FindCycleInResources(const LayoutObject& object);
  static bool FindCycleInSubtree(const LayoutObject& root);

  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;
  void WillBeDestroyed() override;

 private:
  void InvalidateClientsIfActiveResource();

  // Track global (MarkAllClientsForInvalidation) invalidations to avoid
  // redundant crawls.
  unsigned completed_invalidations_mask_ : 8;

  unsigned is_invalidating_ : 1;
  // 23 padding bits available
};

template <>
struct DowncastTraits<LayoutSVGResourceContainer> {
  static bool AllowFrom(const LayoutObject& object) {
    return object.IsSVGResourceContainer();
  }
};

template <typename ContainerType>
inline ContainerType* GetSVGResourceAsType(SVGResourceClient& client,
                                           const SVGResource* resource) {
  if (!resource) {
    return nullptr;
  }
  return DynamicTo<ContainerType>(resource->ResourceContainer(client));
}

template <typename ContainerType>
inline ContainerType* GetSVGResourceAsType(
    SVGResourceClient& client,
    const StyleSVGResource* style_resource) {
  if (!style_resource) {
    return nullptr;
  }
  return GetSVGResourceAsType<ContainerType>(client,
                                             style_resource->Resource());
}

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_CONTAINER_H_
