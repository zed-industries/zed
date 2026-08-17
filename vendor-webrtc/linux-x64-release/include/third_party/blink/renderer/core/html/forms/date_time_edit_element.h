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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_EDIT_ELEMENT_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_EDIT_ELEMENT_H_

#include "third_party/blink/public/mojom/input/focus_type.mojom-blink-forward.h"
#include "third_party/blink/renderer/core/html/forms/date_time_field_element.h"
#include "third_party/blink/renderer/core/html/forms/step_range.h"
#include "third_party/blink/renderer/platform/text/date_components.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class DateTimeFieldsState;
class Locale;
class StepRange;
enum class DateTimeField;

// DateTimeEditElement class contains numberic field and symbolc field for
// representing date and time, such as
//  - Year, Month, Day Of Month
//  - Hour, Minute, Second, Millisecond, AM/PM
class DateTimeEditElement final : public HTMLDivElement,
                                  public DateTimeFieldElement::FieldOwner {
 public:
  // EditControlOwner implementer must call removeEditControlOwner when
  // it doesn't handle event, e.g. at destruction.
  class EditControlOwner : public GarbageCollectedMixin {
   public:
    virtual ~EditControlOwner();
    virtual void DidBlurFromControl(mojom::blink::FocusType) = 0;
    virtual void DidFocusOnControl(mojom::blink::FocusType) = 0;
    virtual void EditControlValueChanged() = 0;
    virtual String FormatDateTimeFieldsState(
        const DateTimeFieldsState&) const = 0;
    virtual bool IsEditControlOwnerDisabled() const = 0;
    virtual bool IsEditControlOwnerReadOnly() const = 0;
    virtual AtomicString LocaleIdentifier() const = 0;
    virtual void EditControlDidChangeValueByKeyboard() = 0;
  };

  struct LayoutParameters {
    STACK_ALLOCATED();

   public:
    String date_time_format;
    String fallback_date_time_format;
    Locale& locale;
    const StepRange step_range;
    DateComponents minimum;
    DateComponents maximum;
    String placeholder_for_day;
    String placeholder_for_month;
    String placeholder_for_year;

    LayoutParameters(Locale& locale, const StepRange& step_range)
        : locale(locale), step_range(step_range) {}
  };

  DateTimeEditElement(Document&, EditControlOwner&);
  DateTimeEditElement(const DateTimeEditElement&) = delete;
  DateTimeEditElement& operator=(const DateTimeEditElement&) = delete;
  ~DateTimeEditElement() override;
  void Trace(Visitor*) const override;

  void AddField(DateTimeFieldElement*);
  bool AnyEditableFieldsHaveValues() const;
  void BlurByOwner();
  void DefaultEventHandler(Event&) override;
  void DisabledStateChanged();
  Element* FieldsWrapperElement() const;
  void FocusIfNoFocus();
  // If oldFocusedNode is one of sub-fields, focus on it. Otherwise focus on
  // the first sub-field.
  void FocusByOwner(Element* old_focused_element = nullptr);
  bool HasFocusedField();
  void ReadOnlyStateChanged();
  void RemoveEditControlOwner() { edit_control_owner_ = nullptr; }
  void ResetFields();
  void SetEmptyValue(const LayoutParameters&,
                     const DateComponents& date_for_read_only_field);
  void SetValueAsDate(const LayoutParameters&, const DateComponents&);
  void SetValueAsDateTimeFieldsState(const DateTimeFieldsState&);
  void SetOnlyYearMonthDay(const DateComponents&);
  void SetOnlyTime(const DateComponents&);
  void SetDateTimeLocal(const DateComponents&);
  void StepDown();
  void StepUp();
  String Value() const;
  DateTimeFieldsState ValueAsDateTimeFieldsState() const;
  bool HasField(DateTimeField) const;
  bool IsFirstFieldAMPM() const;
  wtf_size_t FocusedFieldIndex() const;

 private:
  static const wtf_size_t kInvalidFieldIndex = UINT_MAX;

  // Datetime can be represent at most five fields, such as
  //  1. year
  //  2. month
  //  3. day-of-month
  //  4. hour
  //  5. minute
  //  6. second
  //  7. millisecond
  //  8. AM/PM
  static const int kMaximumNumberOfFields = 8;

  DateTimeFieldElement* GetField(DateTimeField) const;
  DateTimeFieldElement* FieldAt(wtf_size_t) const;
  wtf_size_t FieldIndexOf(const DateTimeFieldElement&) const;
  DateTimeFieldElement* FocusedField() const;
  bool FocusOnNextFocusableField(wtf_size_t start_index);
  bool IsDisabled() const;
  bool IsReadOnly() const;
  void GetLayout(const LayoutParameters&, const DateComponents&);
  void UpdateUIState();

  // Element function.
  const ComputedStyle* CustomStyleForLayoutObject(
      const StyleRecalcContext&) override;
  bool IsDateTimeEditElement() const override;

  // DateTimeFieldElement::FieldOwner functions.
  void DidBlurFromField(mojom::blink::FocusType) override;
  void DidFocusOnField(mojom::blink::FocusType) override;
  void FieldValueChanged() override;
  bool FocusOnNextField(const DateTimeFieldElement&) override;
  bool FocusOnPreviousField(const DateTimeFieldElement&) override;
  void HandleAmPmRollover(DateTimeFieldElement::FieldRolloverType) override;
  bool IsFieldOwnerDisabled() const override;
  bool IsFieldOwnerReadOnly() const override;
  AtomicString LocaleIdentifier() const override;
  void FieldDidChangeValueByKeyboard() override;

  HeapVector<Member<DateTimeFieldElement>, kMaximumNumberOfFields> fields_;
  Member<EditControlOwner> edit_control_owner_;
};

template <>
struct DowncastTraits<DateTimeEditElement> {
  static bool AllowFrom(const Element& element) {
    return element.IsDateTimeEditElement();
  }
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_EDIT_ELEMENT_H_
