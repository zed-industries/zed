/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2010 Apple Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_BUTTON_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_BUTTON_ELEMENT_H_

#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/forms/html_form_control_element.h"

namespace blink {

class CORE_EXPORT HTMLButtonElement final : public HTMLFormControlElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLButtonElement(Document&);

  void setType(const AtomicString&);

  const AtomicString& Value() const;

  bool WillRespondToMouseClickEvents() override;

  void DispatchBlurEvent(Element*,
                         mojom::blink::FocusType,
                         InputDeviceCapabilities*) override;

  // This returns a <select> if this button has type=select and is a direct
  // child of a <select>.
  HTMLSelectElement* OwnerSelect() const;

  // Invoker Commands (https://github.com/whatwg/html/pull/9841)
  Element* commandForElement() const;
  AtomicString command() const;
  void setCommand(const AtomicString& type);
  CommandEventType GetCommandEventType(const AtomicString& type) const;

  // Override for inertness in order to make customizable <select> button inert.
  // TODO(crbug.com/1511354): Replace this with interactivity:inert in
  // UA stylesheet after CSSInert feature has been enabled by default and remove
  // virtual from HTMLElement::IsInertRoot.
  bool IsInertRoot() const override;

 private:
  // The type attribute of HTMLButtonElement is an enumerated attribute:
  // https://html.spec.whatwg.org/multipage/form-elements.html#attr-button-type
  // These values are a subset of the `FormControlType` enum. They have the same
  // binary representation so that FormControlType() reduces to a type cast.
  enum Type : std::underlying_type_t<mojom::blink::FormControlType> {
    kSubmit = base::to_underlying(mojom::blink::FormControlType::kButtonSubmit),
    kReset = base::to_underlying(mojom::blink::FormControlType::kButtonReset),
    kButton = base::to_underlying(mojom::blink::FormControlType::kButtonButton),
  };

  mojom::blink::FormControlType FormControlType() const override;
  const AtomicString& FormControlTypeAsString() const override;

  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void AdjustStyle(ComputedStyleBuilder&) override;

  // HTMLFormControlElement always creates one, but buttons don't need it.
  bool AlwaysCreateUserAgentShadowRoot() const override { return false; }

  Node::InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void DefaultEventHandler(Event&) override;
  bool HasActivationBehavior() const override;

  // Buttons can trigger popovers.
  PopoverTriggerSupport SupportsPopoverTriggering() const override {
    return PopoverTriggerSupport::kSupported;
  }

  void AppendToFormData(FormData&) override;

  bool IsEnumeratable() const override { return true; }
  bool IsLabelable() const override { return true; }
  bool IsInteractiveContent() const override;
  bool MatchesDefaultPseudoClass() const override;

  bool CanBeSuccessfulSubmitButton() const override;
  bool IsActivatedSubmit() const override;
  void SetActivatedSubmit(bool flag) override;

  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;
  bool IsURLAttribute(const Attribute&) const override;

  bool CanStartSelection() const override { return false; }

  bool IsOptionalFormControl() const override { return true; }
  bool RecalcWillValidate() const override;

  int DefaultTabIndex() const override;

  Type type_ = kSubmit;
  bool is_activated_submit_ = false;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_BUTTON_ELEMENT_H_
