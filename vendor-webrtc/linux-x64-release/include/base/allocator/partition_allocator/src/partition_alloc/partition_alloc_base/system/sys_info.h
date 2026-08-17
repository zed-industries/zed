// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_SYSTEM_SYS_INFO_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_SYSTEM_SYS_INFO_H_

#include <cstdint>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base {

class PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) SysInfo {
 public:
  // Retrieves detailed numeric values for the OS version.
  // DON'T USE THIS ON THE MAC OR WINDOWS to determine the current OS release
  // for OS version-specific feature checks and workarounds. If you must use an
  // OS version check instead of a feature check, use
  // base::mac::MacOSMajorVersion() from base/mac/mac_util.h, or
  // base::win::GetVersion() from base/win/windows_version.h.
  static void OperatingSystemVersionNumbers(int32_t* major_version,
                                            int32_t* minor_version,
                                            int32_t* bugfix_version);
};

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_SYSTEM_SYS_INFO_H_
