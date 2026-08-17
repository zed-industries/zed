// Copyright 2019 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_LAUNCHER_TEST_LAUNCHER_TEST_UTILS_H_
#define BASE_TEST_LAUNCHER_TEST_LAUNCHER_TEST_UTILS_H_

#include <stddef.h>

#include <optional>
#include <string>

#include "base/values.h"
#include "testing/gtest/include/gtest/gtest.h"

namespace base {

class FilePath;

namespace test_launcher_utils {

// Validate |dict_value| value in |key| is equal to |expected_value|
bool ValidateKeyValue(const Value::Dict& dict,
                      const std::string& key,
                      const std::string& expected_value);

// Validate |dict_value| value in |key| is equal to |expected_value|
bool ValidateKeyValue(const Value::Dict& dict,
                      const std::string& key,
                      int64_t expected_value);

// Validate |iteration_data| contains one test result under |test_name|
// with |status|, |result_part_count| number of result parts and additional
// fields that only generated after execution when |have_running_info|.
bool ValidateTestResult(const Value::Dict& iteration_data,
                        const std::string& test_name,
                        const std::string& status,
                        size_t result_part_count,
                        bool have_running_info = true);

// Validate test_locations contains all tests in |test_case_name|.
bool ValidateTestLocations(const Value::Dict& test_locations,
                           const std::string& test_case_name);

// Validate test_locations contains the correct file name and line number.
bool ValidateTestLocation(const Value::Dict& test_locations,
                          const std::string& test_name,
                          const std::string& file,
                          int line);

// Read json output file of test launcher.
std::optional<Value::Dict> ReadSummary(const FilePath& path);

}  // namespace test_launcher_utils

}  // namespace base

#endif  // BASE_TEST_LAUNCHER_TEST_LAUNCHER_TEST_UTILS_H_
