/*
 * Copyright (C) 2004, 2005, 2007, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STOP_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STOP_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class Color;
class SVGAnimatedNumber;

class SVGStopElement final : public SVGElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGStopElement(Document&);

  Color StopColorIncludingOpacity() const;

  SVGAnimatedNumber* offset() const { return offset_.Get(); }

  void Trace(Visitor*) const override;

 protected:
  void DidRecalcStyle(const StyleRecalcChange) override;

 private:
  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  // Stop elements don't have associated layout objects.
  bool LayoutObjectIsNeeded(const DisplayStyle&) const override {
    return false;
  }

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedNumber> offset_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STOP_ELEMENT_H_
