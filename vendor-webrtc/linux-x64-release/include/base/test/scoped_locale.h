// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_LOCALE_H_
#define BASE_TEST_SCOPED_LOCALE_H_

#include <string>

namespace base {

// Sets the given |locale| on construction, and restores the previous locale
// on destruction.
class ScopedLocale {
 public:
  explicit ScopedLocale(const std::string& locale);

  ScopedLocale(const ScopedLocale&) = delete;
  ScopedLocale& operator=(const ScopedLocale&) = delete;

  ~ScopedLocale();

 private:
  std::string prev_locale_;
};

}  // namespace base

#endif  // BASE_TEST_SCOPED_LOCALE_H_
