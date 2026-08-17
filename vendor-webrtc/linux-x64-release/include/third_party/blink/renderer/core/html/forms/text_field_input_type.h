/*
 * Copyright (C) 2010 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are
 * met:
 *
 *     * Redistributions of source code must retain the above copyright
 * notice, this list of conditions and the following disclaimer.
 *     * Redistributions in binary form must reproduce the above
 * copyright notice, this list of conditions and the following disclaimer
 * in the documentation and/or other materials provided with the
 * distribution.
 *     * Neither the name of Google Inc. nor the names of its
 * contributors may be used to endorse or promote products derived from
 * this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
 * A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
 * OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
 * SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
 * LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
 * DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
 * THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TEXT_FIELD_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TEXT_FIELD_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/html/forms/input_type.h"
#include "third_party/blink/renderer/core/html/forms/input_type_view.h"
#include "third_party/blink/renderer/core/html/forms/spin_button_element.h"

namespace blink {

// The class represents types of which UI contain text fields.
// It supports not only the types for BaseTextInputType but also type=number.
class TextFieldInputType : public InputType,
                           public InputTypeView,
                           protected SpinButtonElement::SpinButtonOwner {
 public:
  void Trace(Visitor*) const override;
  using InputType::GetElement;
  bool ValueMissing(const String&) const;

 protected:
  TextFieldInputType(Type, HTMLInputElement&);
  ~TextFieldInputType() override;
  bool CanSetSuggestedValue() override;
  void HandleKeydownEvent(KeyboardEvent&) override;

  bool IsInnerEditorValueEmpty() const final;
  void CreateShadowSubtree() override;
  void DestroyShadowSubtree() override;
  void ValueAttributeChanged() override;
  void DisabledAttributeChanged() override;
  void ReadonlyAttributeChanged() override;
  bool SupportsReadOnly() const override;
  void ForwardEvent(Event&) override;
  void HandleBlurEvent() override;
  void HandleBeforeTextInsertedEvent(BeforeTextInsertedEvent&) override;
  String SanitizeValue(const String&) const override;
  void SetValue(const String&,
                bool value_changed,
                TextFieldEventBehavior,
                TextControlSetValueSelection) override;
  void UpdateView() override;
  void AdjustStyle(ComputedStyleBuilder&) override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) const override;
  AppearanceValue AutoAppearance() const override;

  virtual bool NeedsContainer() const { return false; }
  virtual String ConvertFromVisibleValue(const String&) const;
  virtual void DidSetValueByUserEdit();

  void HandleKeydownEventForSpinButton(KeyboardEvent&);
  Element* ContainerElement() const;

 private:
  InputTypeView* CreateView() override;
  ValueMode GetValueMode() const override;
  bool MayTriggerVirtualKeyboard() const final;
  bool ShouldSubmitImplicitly(const Event&) final;
  bool ShouldRespectListAttribute() override;
  void ListAttributeTargetChanged() override;
  HTMLElement* UpdatePlaceholderText(bool is_suggested_value) final;
  void SubtreeHasChanged() final;
  void OpenPopupView() override;

  // SpinButtonElement::SpinButtonOwner functions.
  void FocusAndSelectSpinButtonOwner() final;
  bool ShouldSpinButtonRespondToMouseEvents() final;
  bool ShouldSpinButtonRespondToWheelEvents() final;
  void SpinButtonStepDown() final;
  void SpinButtonStepUp() final;
  void SpinButtonDidReleaseMouseCapture(SpinButtonElement::EventDispatch) final;

  SpinButtonElement* GetSpinButtonElement() const;
  void DisabledOrReadonlyAttributeChanged();
};

template <>
struct DowncastTraits<TextFieldInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsTextFieldInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TEXT_FIELD_INPUT_TYPE_H_
