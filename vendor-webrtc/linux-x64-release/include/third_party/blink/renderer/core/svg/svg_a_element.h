/*
 * Copyright (C) 2004, 2005, 2008 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_A_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_A_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/rel_list.h"
#include "third_party/blink/renderer/core/svg/svg_graphics_element.h"
#include "third_party/blink/renderer/core/svg/svg_uri_reference.h"

namespace blink {

class CORE_EXPORT SVGAElement final : public SVGGraphicsElement,
                                      public SVGURIReference {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SVGAnimatedString* svgTarget() { return svg_target_.Get(); }

  explicit SVGAElement(Document&);

  Element* InterestTargetElement() const override;

  void Trace(Visitor*) const override;

  bool HasRel(uint32_t relation) const;
  DOMTokenList& relList() const { return *rel_list_; }

 private:
  String title() const override;

  void SvgAttributeChanged(const SvgAttributeChangedParams&) override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;

  void DefaultEventHandler(Event&) override;
  bool HasActivationBehavior() const override;

  bool IsLiveLink() const override { return IsLink(); }

  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override;
  bool ShouldHaveFocusAppearance() const final;
  bool IsKeyboardFocusableSlow(
      UpdateBehavior update_behavior =
          UpdateBehavior::kStyleAndLayout) const override;
  bool IsURLAttribute(const Attribute&) const override;
  bool CanStartSelection() const override;
  int DefaultTabIndex() const override;

  bool WillRespondToMouseClickEvents() override;

  SVGAnimatedPropertyBase* PropertyFromAttribute(
      const QualifiedName& attribute_name) const override;
  void SynchronizeAllSVGAttributes() const override;

  void ParseAttribute(const AttributeModificationParams&) override;

  void SetRel(const AtomicString&);

  Member<SVGAnimatedString> svg_target_;
  Member<RelList> rel_list_;
  uint32_t link_relations_ = 0;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_A_ELEMENT_H_
