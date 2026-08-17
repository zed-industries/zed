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

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_MAC_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_MAC_H_

#include <memory>

#include "third_party/blink/renderer/platform/text/platform_locale.h"
#include "third_party/blink/renderer/platform/wtf/forward.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"
#include "third_party/blink/renderer/platform/wtf/vector.h"

@class NSCalendar;
@class NSDateFormatter;
@class NSLocale;

namespace blink {

class PLATFORM_EXPORT LocaleMac : public Locale {
 public:
  static std::unique_ptr<LocaleMac> Create(const String&);
  static std::unique_ptr<LocaleMac> Create(NSLocale*);
  ~LocaleMac() override;

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
  explicit LocaleMac(NSLocale*);
  NSDateFormatter* ShortDateFormatter();
  void InitializeLocaleData() override;

  NSLocale* __strong locale_;
  NSCalendar* __strong gregorian_calendar_;
  Vector<String> week_day_short_labels_;
  Vector<String> month_labels_;
  NSDateFormatter* TimeFormatter();
  NSDateFormatter* ShortTimeFormatter();
  NSDateFormatter* DateTimeFormatterWithSeconds();
  NSDateFormatter* DateTimeFormatterWithoutSeconds();

  String date_format_;
  String month_format_;
  String short_month_format_;
  String time_format_with_seconds_;
  String time_format_without_seconds_;
  String date_time_format_with_seconds_;
  String date_time_format_without_seconds_;
  Vector<String> short_month_labels_;
  Vector<String> stand_alone_month_labels_;
  Vector<String> short_stand_alone_month_labels_;
  Vector<String> time_ampm_labels_;
  bool did_initialize_number_data_;
};

}  // namespace blink

#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_LOCALE_MAC_H_
