// Copyright 2024 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_ANDROID_PMF_UTILS_H_
#define BASE_ANDROID_PMF_UTILS_H_

#include "base/files/file.h"
#include "base/gtest_prod_util.h"
#include "base/process/process.h"

namespace base::android {

class BASE_EXPORT PmfUtils {
 public:
  static std::optional<uint64_t> GetPrivateMemoryFootprintForCurrentProcess();

 private:
  FRIEND_TEST_ALL_PREFIXES(PmfUtilsTest, CalculatePrivateMemoryFootprint);
  static std::optional<uint64_t> CalculatePrivateMemoryFootprintForTesting(
      base::File& statm_file,
      base::File& status_file);
};

}  // namespace base::android

#endif  // BASE_ANDROID_PMF_UTILS_H_
