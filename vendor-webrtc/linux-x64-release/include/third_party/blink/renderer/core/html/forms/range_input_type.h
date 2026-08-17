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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RANGE_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RANGE_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/dom/events/simulated_click_options.h"
#include "third_party/blink/renderer/core/html/forms/input_type.h"
#include "third_party/blink/renderer/core/html/forms/input_type_view.h"

namespace blink {

class ExceptionState;
class SliderThumbElement;

class RangeInputType final : public InputType, public InputTypeView {
 public:
  explicit RangeInputType(HTMLInputElement&);

  void Trace(Visitor*) const override;
  using InputType::GetElement;
  bool TypeMismatchFor(const String&) const;

 private:
  InputTypeView* CreateView() override;
  ValueMode GetValueMode() const override;
  void CountUsage() override;
  void DidRecalcStyle(const StyleRecalcChange) override;
  double ValueAsDouble() const override;
  void SetValueAsDouble(double,
                        TextFieldEventBehavior,
                        ExceptionState&) const override;
  bool SupportsRequired() const override;
  StepRange CreateStepRange(AnyStepHandling) const override;
  void HandleMouseDownEvent(MouseEvent&) override;
  void HandleKeydownEvent(KeyboardEvent&) override;
  LayoutObject* CreateLayoutObject(const ComputedStyle&) const override;
  void AdjustStyle(ComputedStyleBuilder&) override;
  void CreateShadowSubtree() override;
  Decimal ParseToNumber(const String&, const Decimal&) const override;
  String Serialize(const Decimal&) const override;
  void AccessKeyAction(SimulatedClickCreationScope creation_scope) override;
  void SanitizeValueInResponseToMinOrMaxAttributeChange() override;
  void StepAttributeChanged() override;
  void WarnIfValueIsInvalid(const String&) const override;
  String ValueNotEqualText(const Decimal& value) const override;
  String RangeOverflowText(const Decimal& maxmum) const override;
  String RangeUnderflowText(const Decimal& minimum) const override;
  String RangeInvalidText(const Decimal& minimum,
                          const Decimal& maximum) const override;
  void DidSetValue(const String&, bool value_changed) override;
  String SanitizeValue(const String& proposed_value) const override;
  bool ShouldRespectListAttribute() override;
  void DisabledAttributeChanged() override;
  void ListAttributeTargetChanged() override;
  Decimal FindClosestTickMarkValue(const Decimal&) override;

  SliderThumbElement* GetSliderThumbElement() const;
  Element* SliderTrackElement() const;
  void UpdateTickMarkValues();

  // InputTypeView function:
  AppearanceValue AutoAppearance() const override;
  void UpdateView() override;
  void ValueAttributeChanged() override;
  bool IsDraggedSlider() const override;

  bool tick_mark_values_dirty_;
  Vector<Decimal> tick_mark_values_;
};

template <>
struct DowncastTraits<RangeInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsRangeInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_RANGE_INPUT_TYPE_H_
