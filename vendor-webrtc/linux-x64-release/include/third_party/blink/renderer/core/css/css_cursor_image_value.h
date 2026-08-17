/*
 * Copyright (C) 2006 Rob Buis <buis@kde.org>
 * Copyright (C) 2008 Apple Inc. All right reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CURSOR_IMAGE_VALUE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CURSOR_IMAGE_VALUE_H_

#include "third_party/blink/renderer/core/css/css_value.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"
#include "ui/gfx/geometry/point.h"

namespace WTF {
class String;
}  // namespace WTF

namespace blink {

namespace cssvalue {

class CSSCursorImageValue : public CSSValue {
 public:
  CSSCursorImageValue(const CSSValue& image_value,
                      bool hot_spot_specified,
                      const gfx::Point& hot_spot);

  bool HotSpotSpecified() const { return hot_spot_specified_; }
  const gfx::Point& HotSpot() const { return hot_spot_; }
  const CSSValue& ImageValue() const { return *image_value_; }

  WTF::String CustomCSSText() const;

  bool Equals(const CSSCursorImageValue&) const;

  void TraceAfterDispatch(blink::Visitor*) const;

 private:
  Member<const CSSValue> image_value_;
  gfx::Point hot_spot_;
  bool hot_spot_specified_;
};

}  // namespace cssvalue

template <>
struct DowncastTraits<cssvalue::CSSCursorImageValue> {
  static bool AllowFrom(const CSSValue& value) {
    return value.IsCursorImageValue();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_CSS_CSS_CURSOR_IMAGE_VALUE_H_
