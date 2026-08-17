// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GTEST_XML_UTIL_H_
#define BASE_TEST_GTEST_XML_UTIL_H_

#include <vector>

namespace base {

class FilePath;
struct TestResult;

// Produces a vector of test results based on GTest output file.
// Returns true iff the output file exists and has been successfully parsed.
// On successful return and if non-null, |crashed| is set to true if the test
// results are valid but incomplete.
[[nodiscard]] bool ProcessGTestOutput(const base::FilePath& output_file,
                                      std::vector<TestResult>* results,
                                      bool* crashed);

}  // namespace base

#endif  // BASE_TEST_GTEST_XML_UTIL_H_
