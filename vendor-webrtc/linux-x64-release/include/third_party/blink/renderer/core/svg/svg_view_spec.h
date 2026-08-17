/*
 * Copyright (C) 2007 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_VIEW_SPEC_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_VIEW_SPEC_H_

#include "third_party/blink/renderer/core/svg/svg_zoom_and_pan.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace blink {

class SVGPreserveAspectRatio;
class SVGRect;
class SVGTransformList;
class SVGViewElement;

class SVGViewSpec final : public GarbageCollected<SVGViewSpec> {
 public:
  static const SVGViewSpec* CreateFromFragment(const String&);
  static const SVGViewSpec* CreateForViewElement(const SVGViewElement&);
  static const SVGViewSpec* CreateFromAspectRatio(
      const SVGPreserveAspectRatio*);

  const SVGRect* ViewBox() const { return view_box_.Get(); }
  const SVGPreserveAspectRatio* PreserveAspectRatio() const {
    return preserve_aspect_ratio_.Get();
  }
  const SVGTransformList* Transform() const { return transform_.Get(); }
  SVGZoomAndPanType ZoomAndPan() const { return zoom_and_pan_; }

  void Trace(Visitor*) const;

 private:
  bool ParseViewSpec(const String&);
  template <typename CharType>
  bool ParseViewSpecInternal(base::span<const CharType> chars);

  Member<const SVGRect> view_box_;
  Member<const SVGPreserveAspectRatio> preserve_aspect_ratio_;
  Member<const SVGTransformList> transform_;
  SVGZoomAndPanType zoom_and_pan_ = kSVGZoomAndPanUnknown;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_VIEW_SPEC_H_
