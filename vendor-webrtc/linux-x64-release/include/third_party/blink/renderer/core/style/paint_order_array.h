/*
 * Copyright (C) 1999 Antti Koivisto (koivisto@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
 * Copyright (C) 2011 Adobe Systems Incorporated. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_PAINT_ORDER_ARRAY_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_PAINT_ORDER_ARRAY_H_

#include "third_party/blink/renderer/core/style/computed_style_constants.h"

namespace blink {

// Helper that converts a paint-order encoded into an `EPaintOrder` into a form
// that can be index as an array.
class PaintOrderArray {
 public:
  explicit PaintOrderArray(EPaintOrder paint_order)
      : data_(Explode(paint_order)) {}
  enum Type { kNoMarkers };
  // Construct a paint-order array where the 'markers' paint type has been
  // moved to the last position, allowing it to be trivially skipped where it
  // will not have an effect.
  PaintOrderArray(EPaintOrder paint_order, Type)
      : PaintOrderArray(ReorderMarkerPaintType(paint_order)) {
    DCHECK_EQ((*this)[2], PT_MARKERS);
  }

  static constexpr unsigned kMaxItems = 3;

  EPaintOrderType operator[](unsigned index) const {
    DCHECK_LT(index, kMaxItems);
    const unsigned field_shift = kFieldBitwidth * index;
    return static_cast<EPaintOrderType>((data_ >> field_shift) & kFieldMask);
  }

  // Number of keywords in the shortest (canonical) form of a paint order list.
  //
  // Per spec, if any keyword is omitted it will be added last using
  // the standard ordering. So "stroke" implies an order "stroke fill
  // markers" etc. From a serialization PoV this means we never need
  // to emit the last keyword.
  //
  // https://svgwg.org/svg2-draft/painting.html#PaintOrder
  static constexpr unsigned CanonicalLength(EPaintOrder paint_order) {
    switch (paint_order) {
      case kPaintOrderNormal:
      case kPaintOrderFillStrokeMarkers:
      case kPaintOrderStrokeFillMarkers:
      case kPaintOrderMarkersFillStroke:
        return 1;
      case kPaintOrderFillMarkersStroke:
      case kPaintOrderStrokeMarkersFill:
      case kPaintOrderMarkersStrokeFill:
        return 2;
    }
  }

 private:
  static constexpr unsigned kFieldBitwidth = 2;
  static constexpr unsigned kFieldMask = (1u << kFieldBitwidth) - 1;

  static constexpr unsigned EncodeFields(EPaintOrderType first,
                                         EPaintOrderType second,
                                         EPaintOrderType third) {
    return (((third << kFieldBitwidth) | second) << kFieldBitwidth) | first;
  }
  static constexpr unsigned Explode(EPaintOrder paint_order) {
    switch (paint_order) {
      case kPaintOrderNormal:
      case kPaintOrderFillStrokeMarkers:
        return EncodeFields(PT_FILL, PT_STROKE, PT_MARKERS);
      case kPaintOrderFillMarkersStroke:
        return EncodeFields(PT_FILL, PT_MARKERS, PT_STROKE);
      case kPaintOrderStrokeFillMarkers:
        return EncodeFields(PT_STROKE, PT_FILL, PT_MARKERS);
      case kPaintOrderStrokeMarkersFill:
        return EncodeFields(PT_STROKE, PT_MARKERS, PT_FILL);
      case kPaintOrderMarkersFillStroke:
        return EncodeFields(PT_MARKERS, PT_FILL, PT_STROKE);
      case kPaintOrderMarkersStrokeFill:
        return EncodeFields(PT_MARKERS, PT_STROKE, PT_FILL);
    }
  }
  static constexpr EPaintOrder ReorderMarkerPaintType(EPaintOrder paint_order) {
    // Move 'markers' to the last position.
    if (paint_order == kPaintOrderFillMarkersStroke ||
        paint_order == kPaintOrderMarkersFillStroke) {
      return kPaintOrderFillStrokeMarkers;
    }
    if (paint_order == kPaintOrderStrokeMarkersFill ||
        paint_order == kPaintOrderMarkersStrokeFill) {
      return kPaintOrderStrokeFillMarkers;
    }
    return paint_order;
  }

  const unsigned data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_STYLE_PAINT_ORDER_ARRAY_H_
