/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007, 2010 Rob Buis <buis@kde.org>
 * Copyright (C) 2014 Samsung Electronics. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FIT_TO_VIEW_BOX_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FIT_TO_VIEW_BOX_H_

#include "third_party/blink/renderer/platform/heap/garbage_collected.h"
#include "third_party/blink/renderer/platform/heap/member.h"

namespace gfx {
class RectF;
class SizeF;
}  // namespace gfx

namespace blink {

class AffineTransform;
class SVGAnimatedPropertyBase;
class QualifiedName;
class SVGAnimatedPreserveAspectRatio;
class SVGAnimatedRect;
class SVGElement;
class SVGPreserveAspectRatio;
class SVGRect;

class SVGFitToViewBox : public GarbageCollectedMixin {
 public:
  static AffineTransform ViewBoxToViewTransform(
      const gfx::RectF& view_box_rect,
      const SVGPreserveAspectRatio*,
      const gfx::SizeF& viewport_size);

  static bool IsKnownAttribute(const QualifiedName&);

  static bool HasValidViewBox(const SVGRect& view_box);
  bool HasValidViewBox() const;

  // JS API
  SVGAnimatedRect* viewBox() const { return view_box_.Get(); }
  SVGAnimatedPreserveAspectRatio* preserveAspectRatio() const {
    return preserve_aspect_ratio_.Get();
  }

  void Trace(Visitor*) const override;

 protected:
  explicit SVGFitToViewBox(SVGElement*);

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const;
  void SynchronizeAllSVGAttributes() const;

 private:
  Member<SVGAnimatedRect> view_box_;
  Member<SVGAnimatedPreserveAspectRatio> preserve_aspect_ratio_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_FIT_TO_VIEW_BOX_H_
