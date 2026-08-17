// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_GOOGLETEST_CUSTOM_GTEST_INTERNAL_CUSTOM_CHROME_CUSTOM_TEMP_DIR_H_
#define THIRD_PARTY_GOOGLETEST_CUSTOM_GTEST_INTERNAL_CUSTOM_CHROME_CUSTOM_TEMP_DIR_H_

#include <string>

namespace testing {
// Returns alternate temp directory for gtest.
std::string ChromeCustomTempDir();
}  // namespace testing

#endif  // THIRD_PARTY_GOOGLETEST_CUSTOM_GTEST_INTERNAL_CUSTOM_CHROME_CUSTOM_TEMP_DIR_H_
