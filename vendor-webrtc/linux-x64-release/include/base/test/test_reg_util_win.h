// Copyright 2011 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TEST_TEST_REG_UTIL_WIN_H_
#define BASE_TEST_TEST_REG_UTIL_WIN_H_

// Registry utility functions used only by tests.
#include <memory>
#include <string>
#include <vector>

#include "base/time/time.h"
#include "base/win/registry.h"

namespace content {
class BrowserTestBase;
}

namespace registry_util {

// Allows a test to easily override registry hives so that it can start from a
// known good state, or make sure to not leave any side effects once the test
// completes. This supports parallel tests. All the overrides are scoped to the
// lifetime of the override manager. Destroy the manager to undo the overrides.
//
// Overridden hives use keys stored at, for instance:
//   HKCU\Software\Chromium\TempTestKeys\
//       13028145911617809$02AB211C-CF73-478D-8D91-618E11998AED
// The key path are comprises of:
//   - The test key root, HKCU\Software\Chromium\TempTestKeys\
//   - The base::Time::ToInternalValue of the creation time. This is used to
//     delete stale keys left over from crashed tests.
//   - A GUID used for preventing name collisions (although unlikely) between
//     two RegistryOverrideManagers created with the same timestamp.
class RegistryOverrideManager {
 public:
  RegistryOverrideManager();

  RegistryOverrideManager(const RegistryOverrideManager&) = delete;
  RegistryOverrideManager& operator=(const RegistryOverrideManager&) = delete;

  ~RegistryOverrideManager();

  // Override the given registry hive using a randomly generated temporary key.
  // Multiple overrides to the same hive are not supported and lead to undefined
  // behavior.
  // Optional return of the registry override path.
  // Calls to these functions must be wrapped in ASSERT_NO_FATAL_FAILURE to
  // ensure that tests do not proceed in case of failure to override.
  // HKEY_LOCAL_MACHINE should not be overridden in initialization for tests
  // that launch sandboxed processes e.g. browser tests. It is safe to use from
  // within a text fixture, and in unit tests.
  void OverrideRegistry(HKEY override);
  void OverrideRegistry(HKEY override, std::wstring* override_path);

 private:
  friend class RegistryOverrideManagerTest;
  friend class content::BrowserTestBase;

  // Keeps track of one override.
  class ScopedRegistryKeyOverride {
   public:
    ScopedRegistryKeyOverride(HKEY override, const std::wstring& key_path);

    ScopedRegistryKeyOverride(const ScopedRegistryKeyOverride&) = delete;
    ScopedRegistryKeyOverride& operator=(const ScopedRegistryKeyOverride&) =
        delete;

    ~ScopedRegistryKeyOverride();

   private:
    HKEY override_;
    std::wstring key_path_;
  };

  // Used for testing only.
  RegistryOverrideManager(const base::Time& timestamp,
                          const std::wstring& test_key_root);

  // Whether or not to allow using the RegistryOverrideManager for HKLM (e.g. in
  // browser_tests).
  static void SetAllowHKLMRegistryOverrideForIntegrationTests(bool allow);

  base::Time timestamp_;
  std::wstring guid_;

  std::wstring test_key_root_;
  std::vector<std::unique_ptr<ScopedRegistryKeyOverride>> overrides_;
};

// Generates a temporary key path that will be eventually deleted
// automatically if the process crashes.
std::wstring GenerateTempKeyPath();

}  // namespace registry_util

#endif  // BASE_TEST_TEST_REG_UTIL_WIN_H_
