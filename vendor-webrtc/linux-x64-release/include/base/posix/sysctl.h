// Copyright 2023 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_POSIX_SYSCTL_H_
#define BASE_POSIX_SYSCTL_H_

#include <initializer_list>
#include <optional>
#include <string>

#include "base/base_export.h"
#include "build/build_config.h"

// NB: While a BSD utility file, this lives in /base/posix/ for simplicity as
// there is no /base/bsd/.

namespace base {

// Returns the value returned by `sysctl` as a std::string, or nullopt on error.
BASE_EXPORT std::optional<std::string> StringSysctl(
    const std::initializer_list<int>& mib);

#if !BUILDFLAG(IS_OPENBSD)
// Returns the value returned by `sysctlbyname` as a std::string, or nullopt
// on error.
BASE_EXPORT std::optional<std::string> StringSysctlByName(const char* name);
#endif

}  // namespace base

#endif  // BASE_POSIX_SYSCTL_H_
