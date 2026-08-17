/*
 * Copyright (C) 2004, 2005 Nikolas Zimmermann <zimmermann@kde.org>
 * Copyright (C) 2004, 2005, 2006, 2007 Rob Buis <buis@kde.org>
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STYLE_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STYLE_ELEMENT_H_

#include "third_party/blink/renderer/core/css/style_element.h"
#include "third_party/blink/renderer/core/svg/svg_element.h"
#include "third_party/blink/renderer/platform/heap/garbage_collected.h"

namespace blink {

class SVGStyleElement final : public SVGElement, public StyleElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  SVGStyleElement(Document&, const CreateElementFlags);
  ~SVGStyleElement() override;

  using StyleElement::sheet;

  bool disabled() const;
  void setDisabled(bool);

  const AtomicString& type() const override;
  void setType(const AtomicString&);

  const AtomicString& media() const override;
  void setMedia(const AtomicString&);

  String title() const override;
  void setTitle(const AtomicString&);

  void DispatchPendingEvent();

  void Trace(Visitor*) const override;

 private:
  void ParseAttribute(const AttributeModificationParams&) override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void ChildrenChanged(const ChildrenChange&) override;

  void FinishParsingChildren() override;
  bool LayoutObjectIsNeeded(const DisplayStyle&) const override {
    return false;
  }

  bool SheetLoaded() override {
    return StyleElement::SheetLoaded(GetDocument());
  }
  void NotifyLoadedSheetAndAllCriticalSubresources(
      LoadedSheetErrorStatus) override;
  void SetToPendingState() override {
    StyleElement::SetToPendingState(GetDocument(), *this);
  }

  bool IsSameObject(const Node& node) const override { return this == &node; }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_SVG_SVG_STYLE_ELEMENT_H_
