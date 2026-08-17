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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MULTIPLE_FIELDS_TEMPORAL_INPUT_TYPE_VIEW_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MULTIPLE_FIELDS_TEMPORAL_INPUT_TYPE_VIEW_H_

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/html/forms/clear_button_element.h"
#include "third_party/blink/renderer/core/html/forms/date_time_edit_element.h"
#include "third_party/blink/renderer/core/html/forms/input_type_view.h"
#include "third_party/blink/renderer/core/html/forms/picker_indicator_element.h"
#include "third_party/blink/renderer/core/html/forms/spin_button_element.h"

namespace blink {

class BaseTemporalInputType;
struct DateTimeChooserParameters;

class MultipleFieldsTemporalInputTypeView final
    : public GarbageCollected<MultipleFieldsTemporalInputTypeView>,
      public InputTypeView,
      protected DateTimeEditElement::EditControlOwner,
      protected PickerIndicatorElement::PickerIndicatorOwner,
      protected SpinButtonElement::SpinButtonOwner,
      protected ClearButtonElement::ClearButtonOwner {
 public:
  MultipleFieldsTemporalInputTypeView(HTMLInputElement&,
                                      BaseTemporalInputType&);
  ~MultipleFieldsTemporalInputTypeView() override;
  void Trace(Visitor*) const override;

  wtf_size_t FocusedFieldIndex() const override;

  bool IsMultipleFieldsTemporal() const override { return true; }

 private:
  // DateTimeEditElement::EditControlOwner functions
  void DidBlurFromControl(mojom::blink::FocusType) final;
  void DidFocusOnControl(mojom::blink::FocusType) final;
  void EditControlValueChanged() final;
  String FormatDateTimeFieldsState(const DateTimeFieldsState&) const override;
  bool IsEditControlOwnerDisabled() const final;
  bool IsEditControlOwnerReadOnly() const final;
  AtomicString LocaleIdentifier() const final;
  void EditControlDidChangeValueByKeyboard() final;

  // SpinButtonElement::SpinButtonOwner functions.
  void FocusAndSelectSpinButtonOwner() override;
  bool ShouldSpinButtonRespondToMouseEvents() override;
  bool ShouldSpinButtonRespondToWheelEvents() override;
  void SpinButtonStepDown() override;
  void SpinButtonStepUp() override;
  void SpinButtonDidReleaseMouseCapture(
      SpinButtonElement::EventDispatch) override;

  // PickerIndicatorElement::PickerIndicatorOwner functions
  bool IsPickerIndicatorOwnerDisabledOrReadOnly() const final;
  void PickerIndicatorChooseValue(const String&) final;
  void PickerIndicatorChooseValue(double) final;
  Element& PickerOwnerElement() const final;
  bool SetupDateTimeChooserParameters(DateTimeChooserParameters&) final;
  void DidEndChooser() final;
  String AriaLabelForPickerIndicator() const final;

  // ClearButtonElement::ClearButtonOwner functions.
  void FocusAndSelectClearButtonOwner() override;
  bool ShouldClearButtonRespondToMouseEvents() override;
  void ClearValue() override;

  // InputTypeView functions
  void Blur() final;
  AppearanceValue AutoAppearance() const override;
  void OpenPopupView() override;
  void ClosePopupView() override;
  bool HasOpenedPopup() const override;
  bool IsPickerVisible() const override;
  void AdjustStyle(ComputedStyleBuilder&) override;
  void CreateShadowSubtree() final;
  void DestroyShadowSubtree() final;
  void DisabledAttributeChanged() final;
  void ForwardEvent(Event&) final;
  void HandleClickEvent(MouseEvent&) final;
  void HandleFocusInEvent(Element* old_focused_element,
                          mojom::blink::FocusType) final;
  void HandleKeydownEvent(KeyboardEvent&) final;
  bool HasBadInput() const override;
  bool HasCustomFocusLogic() const final;
  void MinOrMaxAttributeChanged() final;
  void ReadonlyAttributeChanged() final;
  void RequiredAttributeChanged() final;
  void RestoreFormControlState(const FormControlState&) final;
  FormControlState SaveFormControlState() const final;
  void DidSetValue(const String&, bool value_changed) final;
  void StepAttributeChanged() final;
  void UpdateView() final;
  void ValueAttributeChanged() override;
  void ListAttributeTargetChanged() final;
  void UpdateClearButtonVisibility() final;
  TextDirection ComputedTextDirection() final;
  AXObject* PopupRootAXObject() final;

  DateTimeEditElement* GetDateTimeEditElement() const;
  // Similar to GetDateTimeEditElement(), but does not force creation.
  DateTimeEditElement* GetDateTimeEditElementIfCreated() const;
  SpinButtonElement* GetSpinButtonElement() const;
  ClearButtonElement* GetClearButtonElement() const;
  PickerIndicatorElement* GetPickerIndicatorElement() const;
  bool ContainsFocusedShadowElement() const;
  void ShowPickerIndicator();
  void HidePickerIndicator();
  void UpdatePickerIndicatorVisibility();

  Member<BaseTemporalInputType> input_type_;
  bool is_destroying_shadow_subtree_;
  bool picker_indicator_is_visible_;
  bool picker_indicator_is_always_visible_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_MULTIPLE_FIELDS_TEMPORAL_INPUT_TYPE_VIEW_H_
