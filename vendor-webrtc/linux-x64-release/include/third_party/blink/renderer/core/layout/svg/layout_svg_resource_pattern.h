/*
 * Copyright (C) 2006 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) Research In Motion Limited 2010. All rights reserved.
 * Copyright 2014 The Chromium Authors
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_PATTERN_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_PATTERN_H_

#include <memory>

#include "third_party/blink/renderer/core/layout/svg/layout_svg_resource_paint_server.h"
#include "third_party/blink/renderer/core/svg/pattern_attributes.h"
#include "third_party/blink/renderer/platform/heap/collection_support/heap_hash_map.h"

namespace blink {

class AffineTransform;
class SVGPatternElement;
struct PatternData;

class LayoutSVGResourcePattern final : public LayoutSVGResourcePaintServer {
 public:
  explicit LayoutSVGResourcePattern(SVGPatternElement*);
  void Trace(Visitor*) const override;

  const char* GetName() const override {
    NOT_DESTROYED();
    return "LayoutSVGResourcePattern";
  }

  void RemoveAllClientsFromCache() override;
  bool RemoveClientFromCache(SVGResourceClient&) override;

  bool ApplyShader(const SVGResourceClient&,
                   const gfx::RectF& reference_box,
                   const AffineTransform* additional_transform,
                   const AutoDarkMode&,
                   cc::PaintFlags&) override;

  static const LayoutSVGResourceType kResourceType = kPatternResourceType;
  LayoutSVGResourceType ResourceType() const override {
    NOT_DESTROYED();
    return kResourceType;
  }

 private:
  void WillBeDestroyed() override;
  void StyleDidChange(StyleDifference, const ComputedStyle* old_style) override;

  bool FindCycleFromSelf() const override;
  std::unique_ptr<PatternData> BuildPatternData(
      const gfx::RectF& object_bounding_box);
  PaintRecord AsPaintRecord(const AffineTransform&) const;

  mutable bool should_collect_pattern_attributes_ : 1;
  mutable PatternAttributes attributes_;

  const PatternAttributes& EnsureAttributes() const;

  // FIXME: we can almost do away with this per-object map, but not quite: the
  // tile size can be relative to the client bounding box, and it gets captured
  // in the cached Pattern shader.
  // Hence, we need one Pattern shader per client. The display list OTOH is the
  // same => we should be able to cache a single display list per
  // LayoutSVGResourcePattern + one Pattern(shader) for each client -- this
  // would avoid re-recording when multiple clients share the same pattern.
  using PatternMap = HeapHashMap<Member<const SVGResourceClient>,
                                 std::unique_ptr<PatternData>>;
  PatternMap pattern_map_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_SVG_LAYOUT_SVG_RESOURCE_PATTERN_H_
