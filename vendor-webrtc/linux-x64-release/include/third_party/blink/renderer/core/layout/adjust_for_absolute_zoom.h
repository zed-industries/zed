/*
 * Copyright (C) 2000 Lars Knoll (knoll@kde.org)
 *           (C) 2000 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2003, 2005, 2006, 2007, 2008, 2009, 2010, 2011 Apple Inc. All
 * rights reserved.
 * Copyright (C) 2006 Graham Dennis (graham.dennis@gmail.com)
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ADJUST_FOR_ABSOLUTE_ZOOM_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ADJUST_FOR_ABSOLUTE_ZOOM_H_

#include "third_party/blink/renderer/core/execution_context/execution_context.h"
#include "third_party/blink/renderer/core/frame/local_frame.h"
#include "third_party/blink/renderer/core/layout/layout_object.h"
#include "third_party/blink/renderer/core/style/computed_style.h"
#include "third_party/blink/renderer/platform/geometry/layout_unit.h"
#include "third_party/blink/renderer/platform/geometry/physical_size.h"
#include "third_party/blink/renderer/platform/runtime_enabled_features.h"

namespace blink {

class AdjustForAbsoluteZoom {
  STATIC_ONLY(AdjustForAbsoluteZoom);

 public:
  // FIXME: Reduce/remove the dependency on zoom adjusted int values.
  // The float or LayoutUnit versions of layout values should be used.
  static int AdjustInt(int value, float zoom_factor) {
    if (zoom_factor == 1)
      return value;
    // Needed because computeLengthInt truncates (rather than rounds) when
    // scaling up.
    float fvalue = value;
    if (zoom_factor > 1) {
      if (value < 0)
        fvalue -= 0.5f;
      else
        fvalue += 0.5f;
    }
    return RoundForImpreciseConversion<int>(fvalue / zoom_factor);
  }
  inline static int AdjustInt(int value, const ComputedStyle& style) {
    float zoom_factor = style.EffectiveZoom();
    if (zoom_factor == 1)
      return value;
    return AdjustInt(value, zoom_factor);
  }
  inline static int AdjustInt(int value, LayoutObject* layout_object) {
    DCHECK(layout_object);
    return AdjustInt(value, layout_object->StyleRef());
  }

  inline static float AdjustFloat(float value, const ComputedStyle& style) {
    return value / style.EffectiveZoom();
  }

  inline static double AdjustDouble(double value, const ComputedStyle& style) {
    return value / style.EffectiveZoom();
  }

  inline static LayoutUnit AdjustLayoutUnit(LayoutUnit value,
                                            const ComputedStyle& style) {
    return LayoutUnit(value / style.EffectiveZoom());
  }
  inline static LayoutUnit AdjustLayoutUnit(LayoutUnit value,
                                            LayoutObject& layout_object) {
    return AdjustLayoutUnit(value, layout_object.StyleRef());
  }
  inline static PhysicalSize AdjustPhysicalSize(PhysicalSize size,
                                                const ComputedStyle& style) {
    return PhysicalSize(AdjustLayoutUnit(size.width, style),
                        AdjustLayoutUnit(size.height, style));
  }

  inline static void AdjustQuadMaybeExcludingCSSZoom(
      gfx::QuadF& quad,
      const LayoutObject& layout_object) {
    float zoom;
    if (layout_object.GetDocument().StandardizedBrowserZoomEnabled()) {
      zoom = layout_object.GetFrame()->LayoutZoomFactor();
    } else {
      zoom = layout_object.StyleRef().EffectiveZoom();
    }
    if (zoom != 1)
      quad.Scale(1 / zoom, 1 / zoom);
  }
  inline static void AdjustRectMaybeExcludingCSSZoom(
      gfx::RectF& rect,
      const LayoutObject& layout_object) {
    float zoom;
    if (layout_object.GetDocument().StandardizedBrowserZoomEnabled()) {
      zoom = layout_object.GetFrame()->LayoutZoomFactor();
    } else {
      zoom = layout_object.StyleRef().EffectiveZoom();
    }

    if (zoom != 1) {
      rect.Scale(1 / zoom, 1 / zoom);
    }
  }

  inline static float AdjustScroll(float scroll_offset, float zoom_factor) {
    return scroll_offset / zoom_factor;
  }
  inline static float AdjustScroll(float scroll_offset,
                                   const ComputedStyle& style) {
    return AdjustScroll(scroll_offset, style.EffectiveZoom());
  }
  inline static double AdjustScroll(double value, LayoutObject& layout_object) {
    return AdjustScroll(value, layout_object.StyleRef());
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_LAYOUT_ADJUST_FOR_ABSOLUTE_ZOOM_H_
