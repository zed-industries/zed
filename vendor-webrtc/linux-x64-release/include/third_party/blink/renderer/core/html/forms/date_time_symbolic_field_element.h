/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1.  Redistributions of source code must retain the above copyright
 *     notice, this list of conditions and the following disclaimer.
 * 2.  Redistributions in binary form must reproduce the above copyright
 *     notice, this list of conditions and the following disclaimer in the
 *     documentation and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY APPLE INC. AND ITS CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL APPLE INC. OR ITS CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_SYMBOLIC_FIELD_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_SYMBOLIC_FIELD_ELEMENT_H_

#include "third_party/blink/renderer/core/html/forms/date_time_field_element.h"
#include "third_party/blink/renderer/core/html/forms/type_ahead.h"

namespace blink {

// DateTimeSymbolicFieldElement represents non-numeric field of data time
// format, such as: AM/PM, and month.
class DateTimeSymbolicFieldElement : public DateTimeFieldElement,
                                     public TypeAheadDataSource {
 public:
  DateTimeSymbolicFieldElement(const DateTimeSymbolicFieldElement&) = delete;
  DateTimeSymbolicFieldElement& operator=(const DateTimeSymbolicFieldElement&) =
      delete;

 protected:
  DateTimeSymbolicFieldElement(Document&,
                               FieldOwner&,
                               DateTimeField,
                               const Vector<String>&,
                               int minimum,
                               int maximum);
  size_t SymbolsSize() const { return symbols_.size(); }
  bool HasValue() const final;
  void Initialize(const AtomicString& pseudo, const String& ax_help_text);
  void SetEmptyValue(EventBehavior = kDispatchNoEvent) final;
  void SetValueAsInteger(int, EventBehavior = kDispatchNoEvent) final;
  int ValueAsInteger() const final;

 private:
  static const int kInvalidIndex = -1;

  String VisibleEmptyValue() const;
  bool IndexIsInRange(int index) const {
    return index >= minimum_index_ && index <= maximum_index_;
  }

  // DateTimeFieldElement functions.
  void HandleKeyboardEvent(KeyboardEvent&) final;
  float MaximumWidth(const ComputedStyle&) override;
  String Placeholder() const override;
  void StepDown() final;
  void StepUp() final;
  String Value() const final;
  int ValueForARIAValueNow() const final;
  String VisibleValue() const final;

  // TypeAheadDataSource functions.
  int IndexOfSelectedOption() const override;
  int OptionCount() const override;
  String OptionAtIndex(int index) const override;

  const Vector<String> symbols_;

  // We use AtomicString to share visible empty value among multiple
  // DateTimeEditElements in the page.
  const AtomicString visible_empty_value_;
  int selected_index_;
  TypeAhead type_ahead_;
  const int minimum_index_;
  const int maximum_index_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_SYMBOLIC_FIELD_ELEMENT_H_
