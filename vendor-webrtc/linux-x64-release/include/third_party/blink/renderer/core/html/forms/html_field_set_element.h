/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FIELD_SET_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FIELD_SET_ELEMENT_H_

#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element.h"

namespace blink {

class HTMLCollection;

class CORE_EXPORT HTMLFieldSetElement final : public HTMLFormControlElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLFieldSetElement(Document&);

  HTMLLegendElement* Legend() const;
  HTMLCollection* elements();

  bool IsDisabledFormControl() const override;

 protected:
  void DisabledAttributeChanged() override;
  void AncestorDisabledStateWasChanged() override;
  void DidMoveToNewDocument(Document& old_document) override;

 private:
  bool IsEnumeratable() const override { return true; }
  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void DidRecalcStyle(const StyleRecalcChange change) override;
  mojom::blink::FormControlType FormControlType() const override;
  const AtomicString& FormControlTypeAsString() const override;
  bool RecalcWillValidate() const override { return false; }
  bool MatchesValidityPseudoClasses() const final;
  bool IsValidElement() final;
  void ChildrenChanged(const ChildrenChange&) override;
  bool AreAuthorShadowsAllowed() const override { return false; }
  bool IsSubmittableElement() override;
  bool AlwaysCreateUserAgentShadowRoot() const override { return false; }
  bool MatchesEnabledPseudoClass() const final;

  Element* InvalidateDescendantDisabledStateAndFindFocusedOne(Element& base);
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FIELD_SET_ELEMENT_H_
