// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_MAC_MAC_UTIL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_MAC_MAC_UTIL_H_

#include <AvailabilityMacros.h>
#include <CoreGraphics/CoreGraphics.h>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base::mac {

// MacOSMajorVersion() returns the major version number (e.g. macOS 12.6.5
// returns 12) of the macOS currently running. Use for runtime OS version
// checking. Prefer to use @available in Objective-C files. Note that this does
// not include any Rapid Security Response (RSR) suffixes (the "(a)" at the end
// of version numbers.)
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
__attribute__((const)) int MacOSMajorVersion();

}  // namespace partition_alloc::internal::base::mac

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_MAC_MAC_UTIL_H_
