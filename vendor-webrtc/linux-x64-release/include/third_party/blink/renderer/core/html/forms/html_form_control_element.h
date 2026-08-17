/*
 * Copyright (C) 1999 Lars Knoll (knoll@kde.org)
 *           (C) 1999 Antti Koivisto (koivisto@kde.org)
 *           (C) 2000 Dirk Mueller (mueller@kde.org)
 * Copyright (C) 2004, 2005, 2006, 2007, 2008, 2009, 2010 Apple Inc. All rights
 * reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROL_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROL_ELEMENT_H_

#include "third_party/blink/public/mojom/forms/form_control_type.mojom-blink.h"
#include "third_party/blink/public/platform/web_string.h"
#include "third_party/blink/public/web/web_autofill_state.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/html/forms/form_associated.h"
#include "third_party/blink/renderer/core/html/forms/listed_element.h"
#include "third_party/blink/renderer/core/html/html_element.h"
#include "third_party/blink/renderer/platform/wtf/casting.h"

namespace blink {

class HTMLFormElement;

// HTMLFormControlElement is the default implementation of
// ListedElement, and listed element implementations should use
// HTMLFormControlElement unless there is a special reason.
class CORE_EXPORT HTMLFormControlElement : public HTMLElement,
                                           public ListedElement,
                                           public FormAssociated {
 public:
  ~HTMLFormControlElement() override;
  void Trace(Visitor*) const override;

  String formAction() const;
  void setFormAction(const AtomicString&);
  String formEnctype() const;
  void setFormEnctype(const AtomicString&);
  String formMethod() const;
  void setFormMethod(const AtomicString&);
  bool FormNoValidate() const;

  void Reset();

  void AttachLayoutTree(AttachContext& context) override;
  void DetachLayoutTree(bool performing_reattach) override;

  HTMLFormElement* formOwner() const final;
  HTMLElement* formForBinding() const final;

  bool IsDisabledFormControl() const override;

  bool MatchesEnabledPseudoClass() const override;

  bool IsEnumeratable() const override { return false; }

  bool IsRequired() const;

  const AtomicString& type() const { return FormControlTypeAsString(); }

  virtual mojom::blink::FormControlType FormControlType() const = 0;
  virtual const AtomicString& FormControlTypeAsString() const = 0;

  virtual bool CanTriggerImplicitSubmission() const { return false; }

  virtual bool IsSubmittableElement() { return true; }

  virtual String ResultForDialogSubmit();

  // Return true if this control type can be a submit button.  This doesn't
  // check |disabled|, and this doesn't check if this is the first submit
  // button.
  virtual bool CanBeSuccessfulSubmitButton() const { return false; }
  // Return true if this control can submit a form.
  // i.e. canBeSuccessfulSubmitButton() && !isDisabledFormControl().
  bool IsSuccessfulSubmitButton() const;
  virtual bool IsActivatedSubmit() const { return false; }
  virtual void SetActivatedSubmit(bool) {}

  struct PopoverTargetElement final {
   public:
    DISALLOW_NEW();
    WeakMember<HTMLElement> popover;
    PopoverTriggerAction action;
    void Trace(Visitor* visitor) const { visitor->Trace(popover); }
  };

  enum class PopoverTriggerSupport {
    kNone,
    kSupported,
  };

  // Retrieves the popover target element and triggering behavior.
  PopoverTargetElement popoverTargetElement();
  virtual PopoverTriggerSupport SupportsPopoverTriggering() const {
    return PopoverTriggerSupport::kNone;
  }

  Element* InterestTargetElement() const override;

  void DefaultEventHandler(Event&) override;

  bool willValidate() const override;

  bool IsReadOnly() const;
  bool IsDisabledOrReadOnly() const;

  bool MayTriggerVirtualKeyboard() const override;

  WebAutofillState GetAutofillState() const { return autofill_state_; }
  bool IsAutofilled() const {
    return autofill_state_ == WebAutofillState::kAutofilled;
  }
  bool IsPreviewed() const {
    return autofill_state_ == WebAutofillState::kPreviewed;
  }
  void SetAutofillState(WebAutofillState = WebAutofillState::kAutofilled);

  bool IsAutocompleteEmailUrlOrPassword() const;

  const AtomicString& autocapitalize() const final;

  static const HTMLFormControlElement* EnclosingFormControlElement(const Node*);

  String NameForAutofill() const;

  void CloneNonAttributePropertiesFrom(const Element&,
                                       NodeCloningData&) override;

  FormAssociated* ToFormAssociatedOrNull() override { return this; }
  void AssociateWith(HTMLFormElement*) override;

  bool BlocksFormSubmission() const { return blocks_form_submission_; }
  void SetBlocksFormSubmission(bool value) { blocks_form_submission_ = value; }

  int32_t GetAxId() const;

  bool MatchesValidityPseudoClasses() const override;

 protected:
  HTMLFormControlElement(const QualifiedName& tag_name, Document&);

  void AttributeChanged(const AttributeModificationParams&) override;
  void ParseAttribute(const AttributeModificationParams&) override;
  virtual void RequiredAttributeChanged();
  void DisabledAttributeChanged() override;
  InsertionNotificationRequest InsertedInto(ContainerNode&) override;
  void RemovedFrom(ContainerNode&) override;
  void WillChangeForm() override;
  void DidChangeForm() override;
  void DidMoveToNewDocument(Document& old_document) override;

  FocusableState SupportsFocus(UpdateBehavior update_behavior) const override;
  bool IsKeyboardFocusableSlow(
      UpdateBehavior update_behavior =
          UpdateBehavior::kStyleAndLayout) const override;
  bool ShouldHaveFocusAppearance() const override;

  virtual void ResetImpl() {}

 private:
  bool IsFormControlElement() const final { return true; }
  bool AlwaysCreateUserAgentShadowRoot() const override { return true; }

  bool IsValidElement() override;

  void HandlePopoverTriggering(HTMLElement* popover,
                               PopoverTriggerAction action);

  enum WebAutofillState autofill_state_;

  bool blocks_form_submission_ : 1;
};

template <>
struct DowncastTraits<HTMLFormControlElement> {
  static bool AllowFrom(const Node& node) {
    auto* element = DynamicTo<Element>(node);
    return element && AllowFrom(*element);
  }
  static bool AllowFrom(const ListedElement& control) {
    return control.IsFormControlElement();
  }
  static bool AllowFrom(const Element& element) {
    return element.IsFormControlElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_FORM_CONTROL_ELEMENT_H_
