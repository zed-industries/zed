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
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH
 * DAMAGE.
 */

#ifndef THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_PLATFORM_LOCALE_H_
#define THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_PLATFORM_LOCALE_H_

#include <array>
#include <memory>

#include "third_party/blink/renderer/platform/language.h"
#include "third_party/blink/renderer/platform/text/date_components.h"
#include "third_party/blink/renderer/platform/wtf/allocator/allocator.h"
#include "third_party/blink/renderer/platform/wtf/text/wtf_string.h"

namespace blink {

class PLATFORM_EXPORT Locale {
  USING_FAST_MALLOC(Locale);

 public:
  static std::unique_ptr<Locale> Create(const String& locale_identifier);
  static Locale& DefaultLocale();
  static void ResetDefaultLocale();

  String QueryString(int resource_id);
  String QueryString(int resource_id, const String& parameter);
  String QueryString(int resource_id,
                     const String& parameter1,
                     const String& parameter2);
  String ValidationMessageTooLongText(unsigned value_length, int max_length);
  String ValidationMessageTooShortText(unsigned value_length, int min_length);

  // Converts the specified number string to another number string localized
  // for this Locale locale. The input string must conform to HTML
  // floating-point numbers, and is not empty.
  String ConvertToLocalizedNumber(const String&);

  // Converts the specified localized number string to a number string in the
  // HTML floating-point number format. The input string is provided by a end
  // user, and might not be a number string. It's ok that the function returns
  // a string which does not conform to the HTML floating-point number format,
  // callers of this function are responsible to check the format of the
  // resultant string.
  String ConvertFromLocalizedNumber(const String&);

  // Remove characters from |input| if a character is not included in
  // locale-specific number characters and |standardChars|.
  String StripInvalidNumberCharacters(const String& input,
                                      const String& standard_chars);

  // Returns localized decimal separator, e.g. "." for English, "," for French.
  String LocalizedDecimalSeparator();

  // Does the locale use single character filtering to do additional number
  // input validation?
  bool UsesSingleCharNumberFiltering();

  // Is the character a sign prefix? Accepts both of standard -+ and localized
  // signs.
  bool IsSignPrefix(UChar ch);

  // Are there 2 sign characters in a string? Accepts both of standard -+ and
  // localized signs.
  bool HasTwoSignChars(const String& str);

  // Is there a sign character that is not after an "E". Accepts both of
  // standard -+ and localized signs.
  bool HasSignNotAfterE(const String& str);

  // Is the character a digit? Accepts both of standard 0-9 and localized
  // digits.
  bool IsDigit(UChar ch);

  // Is the character a decimal separator? Accepts both of standard dot and
  // localized separator.
  bool IsDecimalSeparator(UChar ch);

  // Is there a decimal separator in a string? Accepts both of standard dot and
  // localized separator.
  bool HasDecimalSeparator(const String& str);

  // Returns date format in Unicode TR35 LDML[1] containing day of month,
  // month, and year, e.g. "dd/mm/yyyy"
  // [1] LDML http://unicode.org/reports/tr35/#Date_Format_Patterns
  virtual String DateFormat() = 0;

  // Returns a year-month format in Unicode TR35 LDML.
  virtual String MonthFormat() = 0;

  // Returns a year-month format using short month label in Unicode TR35 LDML.
  virtual String ShortMonthFormat() = 0;

  // Returns time format in Unicode TR35 LDML[1] containing hour, minute, and
  // second with optional period(AM/PM), e.g. "h:mm:ss a"
  // [1] LDML http://unicode.org/reports/tr35/#Date_Format_Patterns
  virtual String TimeFormat() = 0;

  // Returns time format in Unicode TR35 LDML containing hour, and minute
  // with optional period(AM/PM), e.g. "h:mm a"
  // Note: Some platforms return same value as timeFormat().
  virtual String ShortTimeFormat() = 0;

  // Returns a date-time format in Unicode TR35 LDML. It should have a seconds
  // field.
  virtual String DateTimeFormatWithSeconds() = 0;

  // Returns a date-time format in Unicode TR35 LDML. It should have no seconds
  // field.
  virtual String DateTimeFormatWithoutSeconds() = 0;

  // weekFormatInLDML() returns week and year format in LDML, Unicode
  // technical standard 35, Locale Data Markup Language, e.g. "'Week' ww, yyyy"
  String WeekFormatInLDML();

  // Returns a vector of string of which size is 12. The first item is a
  // localized string of Jan and the last item is a localized string of
  // Dec. These strings should be short.
  virtual const Vector<String>& ShortMonthLabels() = 0;

  // Returns a vector of string of which size is 12. The first item is a
  // stand-alone localized string of January and the last item is a
  // stand-alone localized string of December. These strings should not be
  // abbreviations.
  virtual const Vector<String>& StandAloneMonthLabels() = 0;

  // Stand-alone month version of shortMonthLabels.
  virtual const Vector<String>& ShortStandAloneMonthLabels() = 0;

  // Returns localized period field(AM/PM) strings.
  virtual const Vector<String>& TimeAMPMLabels() = 0;

  // Returns a vector of string of which size is 12. The first item is a
  // localized string of January, and the last item is a localized string of
  // December. These strings should not be abbreviations.
  virtual const Vector<String>& MonthLabels() = 0;

  // Returns a vector of string of which size is 7. The first item is a
  // localized short string of Monday, and the last item is a localized
  // short string of Saturday. These strings should be short.
  virtual const Vector<String>& WeekDayShortLabels() = 0;

  // The first day of a week. 0 is Sunday, and 6 is Saturday.
  virtual unsigned FirstDayOfWeek() = 0;

  // Returns true if people use right-to-left writing in the locale for this
  // object.
  virtual bool IsRTL() = 0;

  enum FormatType {
    kFormatTypeUnspecified,
    kFormatTypeShort,
    kFormatTypeMedium
  };

  // Serializes the specified date into a formatted date string to
  // display to the user. If an implementation doesn't support
  // localized dates the function should return an empty string.
  // FormatType can be used to specify if you want the short format.
  String FormatDateTime(const DateComponents&,
                        FormatType = kFormatTypeUnspecified);

  Locale(const Locale&) = delete;
  Locale& operator=(const Locale&) = delete;
  virtual ~Locale();

 protected:
  enum {
    // 0-9 for digits.
    kDecimalSeparatorIndex = 10,
    kGroupSeparatorIndex = 11,
    kDecimalSymbolsSize
  };

  static constexpr char kFallbackWeekdayShortNames[7][4] = {
      "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"};
  static constexpr char kFallbackMonthShortNames[12][4] = {
      "Jan", "Feb", "Mar", "Apr", "May", "Jun",
      "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"};
  static constexpr const char* kFallbackMonthNames[12] = {
      "January", "February", "March",     "April",   "May",      "June",
      "July",    "August",   "September", "October", "November", "December"};

  Locale() : has_locale_data_(false) {}
  virtual void InitializeLocaleData() = 0;
  void SetLocaleData(const Vector<String, kDecimalSymbolsSize>&,
                     const String& positive_prefix,
                     const String& positive_suffix,
                     const String& negative_prefix,
                     const String& negative_suffix);

 private:
  bool DetectSignAndGetDigitRange(const String& input,
                                  bool& is_negative,
                                  unsigned& start_index,
                                  unsigned& end_index);
  unsigned MatchedDecimalSymbolIndex(const String& input, unsigned& position);

  std::array<String, kDecimalSymbolsSize> decimal_symbols_;
  String positive_prefix_;
  String positive_suffix_;
  String negative_prefix_;
  String negative_suffix_;
  String acceptable_number_characters_;
  bool has_locale_data_;
  // Does the locale use single character filtering to do additional number
  // input validation?
  bool uses_single_char_number_filtering_;
};

}  // namespace blink
#endif  // THIRD_PARTY_BLINK_RENDERER_PLATFORM_TEXT_PLATFORM_LOCALE_H_
