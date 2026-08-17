/*
 * Copyright (C) 2012 Google Inc. All rights reserved.
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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_ICU_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_ICU_H_

#include <unicode/udat.h>
#include <unicode/unum.h>

#include <memory>
#include <optional>

#include "base/memory/raw_ptr.h"
#include "third_party/blink/renderer/platform/text/date_components.h"
#include "third_party/blink/renderer/platform/text/platform_locale.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

// We should use this class only for LocalizedNumberICU.cpp,
// LocalizedDateICU.cpp, and LocalizedNumberICUTest.cpp.
class PLATFORM_EXPORT LocaleICU : public Locale {
 public:
  explicit LocaleICU(const std::string&);
  ~LocaleICU() override;

  const Vector<String>& WeekDayShortLabels() override;
  unsigned FirstDayOfWeek() override;
  bool IsRTL() override;
  String DateFormat() override;
  String MonthFormat() override;
  String ShortMonthFormat() override;
  String TimeFormat() override;
  String ShortTimeFormat() override;
  String DateTimeFormatWithSeconds() override;
  String DateTimeFormatWithoutSeconds() override;
  const Vector<String>& MonthLabels() override;
  const Vector<String>& ShortMonthLabels() override;
  const Vector<String>& StandAloneMonthLabels() override;
  const Vector<String>& ShortStandAloneMonthLabels() override;
  const Vector<String>& TimeAMPMLabels() override;

 private:
  String DecimalSymbol(UNumberFormatSymbol);
  String DecimalTextAttribute(UNumberFormatTextAttribute);
  void InitializeLocaleData() override;

  bool DetectSignAndGetDigitRange(const String& input,
                                  bool& is_negative,
                                  unsigned& start_index,
                                  unsigned& end_index);
  unsigned MatchedDecimalSymbolIndex(const String& input, unsigned& position);

  bool InitializeShortDateFormat();
  UDateFormat* OpenDateFormat(UDateFormatStyle time_style,
                              UDateFormatStyle date_style) const;
  UDateFormat* OpenDateFormatForStandAloneMonthLabels(bool is_short) const;

  void InitializeCalendar();

  Vector<String> CreateLabelVector(const UDateFormat*,
                                   UDateFormatSymbolType,
                                   int32_t start_index,
                                   int32_t size);
  void InitializeDateTimeFormat();

  std::string locale_;
  raw_ptr<UNumberFormat, DanglingUntriaged> number_format_;
  raw_ptr<UDateFormat, DanglingUntriaged> short_date_format_;
  bool did_create_decimal_format_;
  bool did_create_short_date_format_;

  Vector<String> week_day_short_labels_;
  std::optional<unsigned> first_day_of_week_;
  Vector<String> month_labels_;
  String date_format_;
  String month_format_;
  String short_month_format_;
  String time_format_with_seconds_;
  String time_format_without_seconds_;
  String date_time_format_with_seconds_;
  String date_time_format_without_seconds_;
  raw_ptr<UDateFormat, DanglingUntriaged> medium_time_format_;
  raw_ptr<UDateFormat, DanglingUntriaged> short_time_format_;
  Vector<String> short_month_labels_;
  Vector<String> stand_alone_month_labels_;
  Vector<String> short_stand_alone_month_labels_;
  Vector<String> time_ampm_labels_;
  bool did_create_time_format_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_ICU_H_
