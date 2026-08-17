// Copyright 2022 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GTEST_TAGS_H_
#define BASE_TEST_GTEST_TAGS_H_

#include <string>

namespace base {

// Add a tag in the gtest xml output.
// Must be called on the thread where gtest is running the test case.
// Only works on desktop, which uses the test launcher.
// A test can call this function when the test generates a tag and save it
// as part of the test result.
// Example: AddTagToTestResult("tag_name", "tag_value")
// `name` is the tag name, should not be empty.
// `value` the actual tag value.
void AddTagToTestResult(const std::string& name, const std::string& value);

// Add a "feature_id" tag in gtest xml output.
// Must be called on the thread where gtest is running the test case.
void AddFeatureIdTagToTestResult(const std::string& value);

}  // namespace base

#endif  // BASE_TEST_GTEST_TAGS_H_
