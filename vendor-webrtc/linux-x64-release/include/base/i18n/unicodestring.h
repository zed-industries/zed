// Copyright 2017 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_I18N_UNICODESTRING_H_
#define BASE_I18N_UNICODESTRING_H_

#include <string>

#include "third_party/icu/source/common/unicode/unistr.h"
#include "third_party/icu/source/common/unicode/uvernum.h"

#if U_ICU_VERSION_MAJOR_NUM >= 59
#include "third_party/icu/source/common/unicode/char16ptr.h"
#endif

namespace base {
namespace i18n {

inline std::u16string UnicodeStringToString16(
    const icu::UnicodeString& unistr) {
#if U_ICU_VERSION_MAJOR_NUM >= 59
  return std::u16string(icu::toUCharPtr(unistr.getBuffer()),
                        static_cast<size_t>(unistr.length()));
#else
  return std::u16string(unistr.getBuffer(),
                        static_cast<size_t>(unistr.length()));
#endif
}

}  // namespace i18n
}  // namespace base

#endif  // BASE_I18N_UNICODESTRING_H_
