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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_TEXT_AREA_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_TEXT_AREA_ELEMENT_H_

#include "base/gtest_prod_util.h"
#include "third_party/blink/renderer/core/core_export.h"
#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/forms/text_control_element.h"

namespace blink {

class BeforeTextInsertedEvent;

class CORE_EXPORT HTMLTextAreaElement final : public TextControlElement {
  DEFINE_WRAPPERTYPEINFO();

 public:
  explicit HTMLTextAreaElement(Document&);

  unsigned cols() const { return cols_; }
  unsigned rows() const { return rows_; }

  bool ShouldWrapText() const { return wrap_ != kNoWrap; }

  String Value() const override;
  void SetValue(
      const String&,
      TextFieldEventBehavior = TextFieldEventBehavior::kDispatchNoEvent,
      TextControlSetValueSelection =
          TextControlSetValueSelection::kSetSelectionToEnd,
      WebAutofillState = WebAutofillState::kNotFilled) override;
  String valueForBinding() const { return Value(); }
  void setValueForBinding(const String&);
  String defaultValue() const;
  void setDefaultValue(const String&);
  int textLength() const { return Value().length(); }

  // Sets the suggested value and puts the element into
  // WebAutofillState::kPreviewed state if |value| is non-empty, or
  // WebAutofillState::kNotFilled otherwise.
  // A null value indicates that the suggested value should be hidden.
  void SetSuggestedValue(const String& value) override;

  // For ValidityState
  String validationMessage() const override;
  bool ValueMissing() const override;
  bool TooLong() const override;
  bool TooShort() const override;
  bool IsValidValue(const String&) const;

  void setCols(unsigned);
  void setRows(unsigned);

  String DefaultToolTip() const override;

  void SetFocused(bool is_focused, mojom::blink::FocusType) override;

 private:
  FRIEND_TEST_ALL_PREFIXES(HTMLTextAreaElementTest, SanitizeUserInputValue);

  enum WrapMethod { kNoWrap, kSoftWrap, kHardWrap };

  void DidAddUserAgentShadowRoot(ShadowRoot&) override;
  bool AreAuthorShadowsAllowed() const override { return false; }

  void HandleBeforeTextInsertedEvent(BeforeTextInsertedEvent*);
  static String SanitizeUserInputValue(const String&, unsigned max_length);
  void UpdateValue();
  void SetNonDirtyValue(const String&, TextControlSetValueSelection);
  void SetValueCommon(const String&,
                      TextFieldEventBehavior,
                      TextControlSetValueSelection,
                      WebAutofillState autofill_state);

  bool IsPlaceholderVisible() const override { return is_placeholder_visible_; }
  void SetPlaceholderVisibility(bool) override;
  bool SupportsPlaceholder() const override { return true; }
  String GetPlaceholderValue() const final;
  HTMLElement* UpdatePlaceholderText() override;
  bool IsInnerEditorValueEmpty() const final;
  void CreateInnerEditorElementIfNecessary() const final;

  bool IsOptionalFormControl() const override {
    return !IsRequiredFormControl();
  }
  bool IsRequiredFormControl() const override { return IsRequired(); }

  void DefaultEventHandler(Event&) override;

  void SubtreeHasChanged() override;

  bool IsEnumeratable() const override { return true; }
  bool IsInteractiveContent() const override;
  bool IsLabelable() const override { return true; }

  mojom::blink::FormControlType FormControlType() const override;
  const AtomicString& FormControlTypeAsString() const override;

  FormControlState SaveFormControlState() const override;
  void RestoreFormControlState(const FormControlState&) override;

  bool IsTextControl() const override { return true; }
  bool IsAutoDirectionalityFormAssociated() const final { return true; }
  int scrollWidth() override;
  int scrollHeight() override;
  void ChildrenChanged(const ChildrenChange&) override;
  void ParseAttribute(const AttributeModificationParams&) override;
  bool IsPresentationAttribute(const QualifiedName&) const override;
  void CollectStyleForPresentationAttribute(
      const QualifiedName&,
      const AtomicString&,
      HeapVector<CSSPropertyValue, 8>&) override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) override;
  void AppendToFormData(FormData&) override;
  void ResetImpl() override;
  bool HasCustomFocusLogic() const override;
  bool MayTriggerVirtualKeyboard() const override;
  bool IsKeyboardFocusableSlow(
      UpdateBehavior update_behavior =
          UpdateBehavior::kStyleAndLayout) const override;
  void UpdateSelectionOnFocus(SelectionBehaviorOnFocus,
                              const FocusOptions*) override;

  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;

  bool MatchesReadOnlyPseudoClass() const override;
  bool MatchesReadWritePseudoClass() const override;
  void CloneNonAttributePropertiesFrom(const Element&, NodeCloningData&) final;

  // If the String* argument is 0, apply value().
  bool ValueMissing(const String*) const;
  bool TooLong(const String*, NeedsToCheckDirtyFlag) const;
  bool TooShort(const String*, NeedsToCheckDirtyFlag) const;

  unsigned rows_;
  unsigned cols_;
  WrapMethod wrap_;
  mutable String value_;
  mutable bool is_dirty_;
  unsigned is_placeholder_visible_ : 1;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_HTML_TEXT_AREA_ELEMENT_H_
