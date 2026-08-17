// Copyright 2012 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_IOS_IOS_UTIL_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_IOS_IOS_UTIL_H_

#include <cstdint>

#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base::ios {

// Returns whether the operating system is at the given version or later.
PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE)
bool IsRunningOnOrLater(int32_t major, int32_t minor, int32_t bug_fix);

}  // namespace partition_alloc::internal::base::ios

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_IOS_IOS_UTIL_H_
