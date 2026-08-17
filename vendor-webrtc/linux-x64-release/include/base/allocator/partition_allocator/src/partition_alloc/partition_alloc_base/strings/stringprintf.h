// Copyright 2013 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef PARTITION_ALLOC_PARTITION_ALLOC_BASE_STRINGS_STRINGPRINTF_H_
#define PARTITION_ALLOC_PARTITION_ALLOC_BASE_STRINGS_STRINGPRINTF_H_

#include <cstdarg>  // va_list
#include <string>

#include "partition_alloc/build_config.h"
#include "partition_alloc/partition_alloc_base/compiler_specific.h"
#include "partition_alloc/partition_alloc_base/component_export.h"

namespace partition_alloc::internal::base {

// Since Only SystemErrorCodeToString and partition_alloc_perftests use
// StringPrintf, make StringPrintf not to support too long results.
// Instead, define max result length and truncate such results.
static constexpr size_t kMaxLengthOfTruncatingStringPrintfResult = 255U;

// Return a C++ string given printf-like input.
[[nodiscard]] PA_PRINTF_FORMAT(1, 2)
    PA_COMPONENT_EXPORT(PARTITION_ALLOC_BASE) std::string
    TruncatingStringPrintf(const char* format, ...);

}  // namespace partition_alloc::internal::base

#endif  // PARTITION_ALLOC_PARTITION_ALLOC_BASE_STRINGS_STRINGPRINTF_H_
