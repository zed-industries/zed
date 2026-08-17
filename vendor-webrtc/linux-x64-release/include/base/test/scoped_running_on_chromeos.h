// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_SCOPED_RUNNING_ON_CHROMEOS_H_
#define BASE_TEST_SCOPED_RUNNING_ON_CHROMEOS_H_

namespace base {
namespace test {

// Test helper that forces base::SysInfo::IsRunningOnChromeOS() to return true.
// NOTE: Must be created on the main thread before any other threads are
// started. Cannot be nested.
class ScopedRunningOnChromeOS {
 public:
  ScopedRunningOnChromeOS();
  ScopedRunningOnChromeOS(const ScopedRunningOnChromeOS&) = delete;
  ScopedRunningOnChromeOS& operator=(const ScopedRunningOnChromeOS&) = delete;
  ~ScopedRunningOnChromeOS();
};

}  // namespace test
}  // namespace base

#endif  //  BASE_TEST_SCOPED_RUNNING_ON_CHROMEOS_H_
