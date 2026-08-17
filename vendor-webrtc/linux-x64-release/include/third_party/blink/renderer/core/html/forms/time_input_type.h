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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TIME_INPUT_TYPE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TIME_INPUT_TYPE_H_

#include "third_party/blink/renderer/core/html/forms/base_temporal_input_type.h"

namespace blink {

class TimeInputType final : public BaseTemporalInputType {
 public:
  explicit TimeInputType(HTMLInputElement&);

 private:
  void CountUsage() override;
  Decimal DefaultValueForStepUp() const override;
  StepRange CreateStepRange(AnyStepHandling) const override;
  bool ParseToDateComponentsInternal(const String&,
                                     DateComponents*) const override;
  bool SetMillisecondToDateComponents(double, DateComponents*) const override;
  void WarnIfValueIsInvalid(const String&) const override;
  String LocalizeValue(const String&) const override;

  // BaseTemporalInputType functions
  String FormatDateTimeFieldsState(const DateTimeFieldsState&) const override;
  void SetupLayoutParameters(DateTimeEditElement::LayoutParameters&,
                             const DateComponents&) const override;
  bool IsValidFormat(bool has_year,
                     bool has_month,
                     bool has_week,
                     bool has_day,
                     bool has_ampm,
                     bool has_hour,
                     bool has_minute,
                     bool has_second) const override;
  String AriaLabelForPickerIndicator() const override;
  String ReversedRangeOutOfRangeText(const Decimal& minimum,
                                     const Decimal& maximum) const override;
};

template <>
struct DowncastTraits<TimeInputType> {
  static bool AllowFrom(const InputType& type) {
    return type.IsTimeInputType();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_TIME_INPUT_TYPE_H_
