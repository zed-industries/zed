// Copyright 2015 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_ICU_TEST_UTIL_H_
#define BASE_TEST_ICU_TEST_UTIL_H_

#include <memory>
#include <string>

#include "third_party/icu/source/common/unicode/uversion.h"

U_NAMESPACE_BEGIN
class TimeZone;
U_NAMESPACE_END

namespace base {
namespace test {

// In unit tests, prefer ScopedRestoreICUDefaultLocale over
// calling base::i18n::SetICUDefaultLocale() directly. This scoper makes it
// harder to accidentally forget to reset the locale.
class ScopedRestoreICUDefaultLocale {
 public:
  ScopedRestoreICUDefaultLocale();
  explicit ScopedRestoreICUDefaultLocale(const std::string& locale);
  ScopedRestoreICUDefaultLocale(const ScopedRestoreICUDefaultLocale&) = delete;
  ScopedRestoreICUDefaultLocale& operator=(
      const ScopedRestoreICUDefaultLocale&) = delete;
  ~ScopedRestoreICUDefaultLocale();

 private:
  const std::string default_locale_;
};

// In unit tests, prefer ScopedRestoreDefaultTimezone over
// calling icu::TimeZone::adoptDefault() directly. This scoper makes it
// harder to accidentally forget to reset the locale.
class ScopedRestoreDefaultTimezone {
 public:
  ScopedRestoreDefaultTimezone(const char* zoneid);
  ~ScopedRestoreDefaultTimezone();

  ScopedRestoreDefaultTimezone(const ScopedRestoreDefaultTimezone&) = delete;
  ScopedRestoreDefaultTimezone& operator=(const ScopedRestoreDefaultTimezone&) =
      delete;

 private:
  std::unique_ptr<icu::TimeZone> original_zone_;
};

void InitializeICUForTesting();

}  // namespace test
}  // namespace base

#endif  // BASE_TEST_ICU_TEST_UTIL_H_
