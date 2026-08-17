// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_FIDL_MATCHERS_H_
#define BASE_TEST_FIDL_MATCHERS_H_

#include <lib/fidl/cpp/comparison.h>

#include "testing/gmock/include/gmock/gmock-matchers.h"

namespace base::test {

// Matcher that verifies a fidl struct is equal to the expected fidl struct.
MATCHER_P(FidlEq,
          expected,
          "Matches if the expected fidl struct is equal to the argument per "
          "fidl::Equals().") {
  return fidl::Equals(arg, expected);
}

}  // namespace base::test

#endif  // BASE_TEST_FIDL_MATCHERS_H_
