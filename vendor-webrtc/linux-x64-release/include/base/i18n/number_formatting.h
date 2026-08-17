// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_NUMBER_FORMATTING_H_
#define BASE_I18N_NUMBER_FORMATTING_H_

#include <stdint.h>

#include <string>

#include "base/i18n/base_i18n_export.h"

namespace base {

// Return a number formatted with separators in the user's locale.
// Ex: FormatNumber(1234567) => "1,234,567" in English, "1.234.567" in German
BASE_I18N_EXPORT std::u16string FormatNumber(int64_t number);

// Return a number formatted with separators in the user's locale, with
// `fractional_digits` digits after the decimal point.
// Ex: FormatDouble(1234567.8, 1)
//         => "1,234,567.8" in English, "1.234.567,8" in German
BASE_I18N_EXPORT std::u16string FormatDouble(double number,
                                             int fractional_digits);

// Return a number formatted with separators in the user's locale, with up to
// `max_fractional_digits` digits after the decimal point, and eliminating
// trailing zeroes after `min_fractional_digits`.
// Ex: FormatDouble(1234567.8, 0, 4)
//         => "1,234,567.8" in English, "1.234.567,8" in German
// Ex: FormatDouble(1234567.888888, 0, 4)
//         => "1,234,567.8889" in English, "1.234.567,8889" in German
BASE_I18N_EXPORT std::u16string FormatDouble(double number,
                                             int min_fractional_digits,
                                             int max_fractional_digits);

// Return a percentage formatted with space and symbol in the user's locale.
// Ex: FormatPercent(12) => "12%" in English, "12 %" in Romanian
BASE_I18N_EXPORT std::u16string FormatPercent(int number);

// Causes cached formatters to be discarded and recreated. Only useful for
// testing.
BASE_I18N_EXPORT void ResetFormattersForTesting();

}  // namespace base

#endif  // BASE_I18N_NUMBER_FORMATTING_H_
