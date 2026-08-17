// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_CHECK_IS_TEST_H_
#define BASE_CHECK_IS_TEST_H_

#include "base/base_export.h"
#include "base/check.h"
#include "base/not_fatal_until.h"

// Code paths taken in tests are sometimes different from those taken in
// production. This might be because the respective tests do not initialize some
// objects that would be required for the "normal" code path.
//
// Ideally, such code constructs should be avoided, so that tests really test
// the production code and not something different.
//
// However, there already are hundreds of test-only paths in production code
// Cleaning up all these cases retroactively and completely avoiding such cases
// in the future seems unrealistic.
//
// Thus, it is useful to prevent the test code-only paths to be taken in
// production scenarios.
//
// `CHECK_IS_TEST` can be used to assert that a test-only path is actually taken
// only in tests. For instance:
//
//   // This only happens in unit tests:
//   if (!url_loader_factory)
//   {
//     // Assert that this code path is really only taken in tests.
//     CHECK_IS_TEST();
//     return;
//   }
//
// `CHECK_IS_TEST` should not be used within functions named `*ForTesting`,
// `*ForTests`, etc. because there is a presubmit check which warns against
// calling such functions in production code.
//
// `CHECK_IS_TEST` is thread safe.
//
// An optional base::NotFatalUntil argument can be provided to make the
// instance non-fatal (dumps without crashing) before a provided milestone.
// See base/check.h for details.

namespace base::internal {
BASE_EXPORT bool get_is_test_impl();
}  // namespace base::internal

#define CHECK_IS_TEST(...) \
  CHECK(base::internal::get_is_test_impl() __VA_OPT__(, ) __VA_ARGS__)

// In special cases, code should not execute in a test.
#define CHECK_IS_NOT_TEST() CHECK(!base::internal::get_is_test_impl())

#endif  // BASE_CHECK_IS_TEST_H_
