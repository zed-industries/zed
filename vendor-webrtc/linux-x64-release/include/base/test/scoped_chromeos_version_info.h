// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_CHROMEOS_VERSION_INFO_H_
#define BASE_TEST_SCOPED_CHROMEOS_VERSION_INFO_H_

#include <string_view>

#include "base/time/time.h"

namespace base {
namespace test {

// Test helper that temporarily overrides the cached lsb-release data.
// NOTE: Must be created on the main thread before any other threads are
// started. Cannot be nested.
class ScopedChromeOSVersionInfo {
 public:
  // Overrides |lsb_release| and |lsb_release_time|. For example, can be used to
  // simulate a specific OS version. Note that |lsb_release| must contain
  // CHROMEOS_RELEASE_NAME to make base::SysInfo::IsRunningOnChromeOS() return
  // true.
  ScopedChromeOSVersionInfo(std::string_view lsb_release,
                            Time lsb_release_time);
  ScopedChromeOSVersionInfo(const ScopedChromeOSVersionInfo&) = delete;
  ScopedChromeOSVersionInfo& operator=(const ScopedChromeOSVersionInfo&) =
      delete;
  ~ScopedChromeOSVersionInfo();
};

}  // namespace test
}  // namespace base

#endif  //  BASE_TEST_SCOPED_CHROMEOS_VERSION_INFO_H_
