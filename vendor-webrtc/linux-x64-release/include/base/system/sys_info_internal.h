// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_SYSTEM_SYS_INFO_INTERNAL_H_
#define BASE_SYSTEM_SYS_INFO_INTERNAL_H_

#include "base/base_export.h"
#include "build/build_config.h"

#if BUILDFLAG(IS_APPLE)
#include <optional>
#endif

namespace base {

namespace internal {

template <typename T, T (*F)(void)>
class LazySysInfoValue {
 public:
  LazySysInfoValue() : value_(F()) {}

  LazySysInfoValue(const LazySysInfoValue&) = delete;
  LazySysInfoValue& operator=(const LazySysInfoValue&) = delete;

  ~LazySysInfoValue() = default;

  T value() { return value_; }

 private:
  const T value_;
};

#if BUILDFLAG(IS_MAC)
// Exposed for testing.
BASE_EXPORT std::optional<int> NumberOfPhysicalProcessors();

// When CPU security mitigation is enabled, return number of "physical"
// cores and not the number of "logical" cores. CPU security mitigations
// disables hyper-threading for the current application, which effectively
// limits the number of concurrently executing threads to the number of
// physical cores.
std::optional<int> NumberOfProcessorsWhenCpuSecurityMitigationEnabled();
#endif

#if BUILDFLAG(IS_APPLE)
std::optional<int> GetSysctlIntValue(const char* key_name);
#endif

}  // namespace internal

}  // namespace base

#endif  // BASE_SYSTEM_SYS_INFO_INTERNAL_H_
