/*
 * Copyright (C) 2007 Eric Seidel <eric@webkit.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_MPATH_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_MPATH_ELEMENT_H_

#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/core/svg/svg_uri_reference.h"

namespace blink {

class SVGPathElement;

class SVGMPathElement final : public SVGElement, public SVGURIReference {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit SVGMPathElement(Document&);
  ~SVGMPathElement() override;

  SVGPathElement* PathElement();

  void TargetPathChanged();

  void Trace(Visitor*) const override;

 private:
  void BuildPendingResource() override;
  void ClearResourceReferences();
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  bool LayoutObjectIsNeeded(const DisplayStyle&) const override {
    return false;
  }
  void NotifyParentOfPathChange(ContainerNode*);

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  Member<IdTargetObserver> target_id_observer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_MPATH_ELEMENT_H_
