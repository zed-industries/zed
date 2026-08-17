// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_GTEST_LINKS_H_
#define BASE_TEST_GTEST_LINKS_H_

#include <string>

namespace base {

// Add a link in the gtest xml output.
// Only call this from a gtest test body with the same thread as the test.
// Only works on desktop.
// A test can call this function when the test generates a link and save it
// as part of the test result.
// Example: AddLinkToTestResult("image_link",
// "https://example_googlestorage/test.png") can mean a test generates an image
// with the url.
// |name| is the link name. It should be unique in one test case. Name will
// be displayed on test result page(Milo). |name| should only contains
// ascii-letters, ascii-digits, '/' and '_'.
// |url| the actual url.
void AddLinkToTestResult(const std::string& name, const std::string& url);

}  // namespace base

#endif  // BASE_TEST_GTEST_LINKS_H_
