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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_NUMERIC_FIELD_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_NUMERIC_FIELD_ELEMENT_H_

#include "third_party/blink/renderer/core/html/forms/date_time_field_element.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/string_builder.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// DateTimeNumericFieldElement represents numeric field of date time format,
// such as:
//  - hour
//  - minute
//  - millisecond
//  - second
//  - year
class DateTimeNumericFieldElement : public DateTimeFieldElement {
 public:
  struct Step {
    DISALLOW_NEW();
    Step(int step = 1, int step_base = 0) : step(step), step_base(step_base) {}
    int step;
    int step_base;
  };

  struct Range {
    DISALLOW_NEW();
    Range(int minimum, int maximum) : minimum(minimum), maximum(maximum) {}
    int ClampValue(int) const;
    bool IsInRange(int) const;
    bool IsSingleton() const { return minimum == maximum; }

    int minimum;
    int maximum;
  };

  DateTimeNumericFieldElement(const DateTimeNumericFieldElement&) = delete;
  DateTimeNumericFieldElement& operator=(const DateTimeNumericFieldElement&) =
      delete;

 protected:
  DateTimeNumericFieldElement(Document&,
                              FieldOwner&,
                              DateTimeField,
                              const Range&,
                              const Range& hard_limits,
                              const String& placeholder,
                              const Step& = Step());

  int ClampValue(int value) const { return range_.ClampValue(value); }
  virtual int DefaultValueForStepDown() const;
  virtual int DefaultValueForStepUp() const;
  virtual void NotifyOwnerIfStepDownRollOver(bool has_value,
                                             Step step,
                                             int old_value,
                                             int new_value) {}
  virtual void NotifyOwnerIfStepUpRollOver(bool has_value,
                                           Step step,
                                           int old_value,
                                           int new_value) {}
  const Range& GetRange() const { return range_; }

  // DateTimeFieldElement functions.
  bool HasValue() const final;
  void Initialize(const AtomicString& pseudo, const String& ax_help_text);
  int Maximum() const;
  String Placeholder() const override;
  void SetEmptyValue(EventBehavior = kDispatchNoEvent) final;
  void SetValueAsInteger(int, EventBehavior = kDispatchNoEvent) override;
  int ValueAsInteger() const final;
  String VisibleValue() const final;

 private:
  // DateTimeFieldElement functions.
  void HandleKeyboardEvent(KeyboardEvent&) final;
  float MaximumWidth(const ComputedStyle&) override;
  void StepDown() final;
  void StepUp() final;
  String Value() const final;

  // Node functions.
  void SetFocused(bool, mojom::blink::FocusType) final;

  String FormatValue(int) const;
  int RoundUp(int) const;
  int RoundDown(int) const;
  int TypeAheadValue() const;

  const String placeholder_;
  const Range range_;
  const Range hard_limits_;
  const Step step_;
  int value_;
  bool has_value_;
  mutable StringBuilder type_ahead_buffer_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_NUMERIC_FIELD_ELEMENT_H_
