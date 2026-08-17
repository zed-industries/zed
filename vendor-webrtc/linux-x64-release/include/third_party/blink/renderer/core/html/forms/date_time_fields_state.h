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

#ifndef THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_FIELDS_STATE_H_
#define THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_FIELDS_STATE_H_

#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"

namespace blink {

class FormControlState;

// DateTimeFieldsState represents fields in date/time for form state
// save/restore for input type "date", "datetime", "datetime-local", "month",
// "time", and "week" with multiple fields input UI.
//
// Each field can contain invalid value for date, e.g. day of month field can
// be 30 even if month field is February.
class DateTimeFieldsState {
  STACK_ALLOCATED();

 public:
  enum AMPMValue {
    kAMPMValueEmpty = -1,
    kAMPMValueAM,
    kAMPMValuePM,
  };

  static const unsigned kEmptyValue;

  DateTimeFieldsState();

  static DateTimeFieldsState RestoreFormControlState(const FormControlState&);
  FormControlState SaveFormControlState() const;

  AMPMValue Ampm() const { return ampm_; }
  unsigned DayOfMonth() const { return day_of_month_; }
  unsigned Hour() const { return hour_; }
  unsigned Hour24() const;
  unsigned Millisecond() const { return millisecond_; }
  unsigned Minute() const { return minute_; }
  unsigned Month() const { return month_; }
  unsigned Second() const { return second_; }
  unsigned WeekOfYear() const { return week_of_year_; }
  unsigned Year() const { return year_; }

  bool HasAMPM() const { return ampm_ != kAMPMValueEmpty; }
  bool HasDayOfMonth() const { return day_of_month_ != kEmptyValue; }
  bool HasHour() const { return hour_ != kEmptyValue; }
  bool HasMillisecond() const { return millisecond_ != kEmptyValue; }
  bool HasMinute() const { return minute_ != kEmptyValue; }
  bool HasMonth() const { return month_ != kEmptyValue; }
  bool HasSecond() const { return second_ != kEmptyValue; }
  bool HasWeekOfYear() const { return week_of_year_ != kEmptyValue; }
  bool HasYear() const { return year_ != kEmptyValue; }

  void SetAMPM(AMPMValue ampm) { ampm_ = ampm; }
  void SetDayOfMonth(unsigned day_of_month) { day_of_month_ = day_of_month; }
  void SetHour(unsigned hour12) { hour_ = hour12; }
  void SetHour24(unsigned hour24);
  void SetMillisecond(unsigned millisecond) { millisecond_ = millisecond; }
  void SetMinute(unsigned minute) { minute_ = minute; }
  void SetMonth(unsigned month) { month_ = month; }
  void SetSecond(unsigned second) { second_ = second; }
  void SetWeekOfYear(unsigned week_of_year) { week_of_year_ = week_of_year; }
  void SetYear(unsigned year) { year_ = year; }

 private:
  unsigned year_;
  unsigned month_;  // 1 to 12.
  unsigned day_of_month_;
  unsigned hour_;  // 1 to 12.
  unsigned minute_;
  unsigned second_;
  unsigned millisecond_;
  unsigned week_of_year_;
  AMPMValue ampm_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_CORE_HTML_FORMS_DATE_TIME_FIELDS_STATE_H_
