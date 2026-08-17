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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_BASE_TEMPORAL_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_BASE_TEMPORAL_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/html/forms/date_time_edit_element.h"
#include "third_party/blink/renderer/core/html/forms/input_type.h"
#include "third_party/blink/renderer/platform/text/date_components.h"

namespace blink {

class ExceptionState;

// A super class of date, datetime, datetime-local, month, time, and week types.
// TODO(tkent): A single temporal input type creates two InputTypeView instances
// unnecessarily.  One is ChooserOnlyTemporalInputTypeView or
// MultipleFieldsTemporalInputType, and another is BaseTemporalInputType, which
// inherits from InputTypeView through InputType.  The latter is not used.
class BaseTemporalInputType : public InputType {
 public:
  String VisibleValue() const override;
  String SanitizeValue(const String&) const override;
  // Parses the specified string for this InputType, and returns true if it
  // is successfully parsed. An instance pointed by the DateComponents*
  // parameter will have parsed values and be modified even if the parsing
  // fails. The DateComponents* parameter may be 0.
  bool ParseToDateComponents(const String&, DateComponents*) const;
  virtual bool SetMillisecondToDateComponents(double,
                                              DateComponents*) const = 0;

  // Provide some helpers for MultipleFieldsTemporalInputTypeView.
  virtual String FormatDateTimeFieldsState(
      const DateTimeFieldsState&) const = 0;
  virtual void SetupLayoutParameters(DateTimeEditElement::LayoutParameters&,
                                     const DateComponents&) const = 0;
  virtual bool IsValidFormat(bool has_year,
                             bool has_month,
                             bool has_week,
                             bool has_day,
                             bool has_ampm,
                             bool has_hour,
                             bool has_minute,
                             bool has_second) const = 0;
  virtual String AriaLabelForPickerIndicator() const = 0;
  bool TypeMismatchFor(const String&) const;
  bool ValueMissing(const String&) const;

 protected:
  BaseTemporalInputType(Type type, HTMLInputElement& element)
      : InputType(type, element) {}
  Decimal ParseToNumber(const String&, const Decimal&) const override;
  String Serialize(const Decimal&) const override;
  String SerializeWithComponents(const DateComponents&) const;
  bool ShouldHaveSecondField(const DateComponents&) const;

 private:
  virtual bool ParseToDateComponentsInternal(const String&,
                                             DateComponents*) const = 0;

  String BadInputText() const override;
  InputTypeView* CreateView() override;
  ValueMode GetValueMode() const override;
  double ValueAsDate() const override;
  void SetValueAsDate(const std::optional<base::Time>&,
                      ExceptionState&) const override;
  double ValueAsDouble() const override;
  void SetValueAsDouble(double,
                        TextFieldEventBehavior,
                        ExceptionState&) const override;
  bool TypeMismatch() const override;
  String ValueNotEqualText(const Decimal& value) const override;
  String RangeOverflowText(const Decimal& maximum) const override;
  String RangeUnderflowText(const Decimal& minimum) const override;
  String RangeInvalidText(const Decimal& minimum,
                          const Decimal& maximum) const override;
  Decimal DefaultValueForStepUp() const override;
  virtual String SerializeWithDate(const std::optional<base::Time>&) const;
  String LocalizeValue(const String&) const override;
  bool SupportsReadOnly() const override;
  bool ShouldRespectListAttribute() override;
  bool MayTriggerVirtualKeyboard() const override;
};

template <>
struct DowncastTraits<BaseTemporalInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsBaseTemporalInputType();
  }
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_BASE_TEMPORAL_INPUT_TYPE_H_
