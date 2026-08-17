// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_STRINGS_STRING_NUMBER_CONVERSIONS_WIN_H_
#define BASE_STRINGS_STRING_NUMBER_CONVERSIONS_WIN_H_

#include <string>
#include <string_view>

#include "base/base_export.h"

namespace base {

BASE_EXPORT std::wstring NumberToWString(int value);
BASE_EXPORT std::wstring NumberToWString(unsigned int value);
BASE_EXPORT std::wstring NumberToWString(long value);
BASE_EXPORT std::wstring NumberToWString(unsigned long value);
BASE_EXPORT std::wstring NumberToWString(long long value);
BASE_EXPORT std::wstring NumberToWString(unsigned long long value);
BASE_EXPORT std::wstring NumberToWString(double value);

// The following section contains overloads of the cross-platform APIs for
// std::wstring and std::wstring_view.
BASE_EXPORT bool StringToInt(std::wstring_view input, int* output);
BASE_EXPORT bool StringToUint(std::wstring_view input, unsigned* output);
BASE_EXPORT bool StringToInt64(std::wstring_view input, int64_t* output);
BASE_EXPORT bool StringToUint64(std::wstring_view input, uint64_t* output);
BASE_EXPORT bool StringToSizeT(std::wstring_view input, size_t* output);
BASE_EXPORT bool StringToDouble(std::wstring_view input, double* output);

}  // namespace base

#endif  // BASE_STRINGS_STRING_NUMBER_CONVERSIONS_WIN_H_
