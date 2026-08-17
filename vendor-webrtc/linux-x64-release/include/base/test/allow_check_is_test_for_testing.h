// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_ALLOW_CHECK_IS_TEST_FOR_TESTING_H_
#define BASE_TEST_ALLOW_CHECK_IS_TEST_FOR_TESTING_H_

namespace base::test {

// This is to be called exactly once when starting unit or browser tests to
// allow test-only code paths that contain `CHECK_IS_TEST()`. It must be called
// before we have other threads to avoid races with calls to `CHECK_IS_TEST()`.
//
// Note: This function must not be called in production code, but only in
// tests.
void AllowCheckIsTestForTesting();

}  // namespace base::test

#endif  // BASE_TEST_ALLOW_CHECK_IS_TEST_FOR_TESTING_H_
