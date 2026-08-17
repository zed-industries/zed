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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_CLIP_PATH_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_CLIP_PATH_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_animated_enumeration.h"
#include "third_party/blink/renderer/core/svg/svg_transformable_element.h"
#include "third_party/blink/renderer/core/svg/svg_unit_types.h"

namespace blink {

class SVGClipPathElement final : public SVGTransformableElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGClipPathElement(Document&);

  SVGAnimatedEnumeration<SVGUnitTypes::SVGUnitType>* clipPathUnits() {
    return clip_path_units_.Get();
  }

  void Trace(Visitor*) const override;

 private:
  FocusableState SupportsFocus(UpdateBehavior) const override {
    return FocusableState::kNotFocusable;
  }

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;
  void ChildrenChanged(const ChildrenChange&) override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<SVGAnimatedEnumeration<SVGUnitTypes::SVGUnitType>> clip_path_units_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_CLIP_PATH_ELEMENT_H_
