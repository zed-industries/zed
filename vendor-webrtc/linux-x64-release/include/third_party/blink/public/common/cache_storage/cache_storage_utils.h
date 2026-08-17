// Copyright 2018 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef THIRD_PARTY_BLINK_PUBLIC_COMMON_CACHE_STORAGE_CACHE_STORAGE_UTILS_H_
#define THIRD_PARTY_BLINK_PUBLIC_COMMON_CACHE_STORAGE_CACHE_STORAGE_UTILS_H_

#include <cstdint>

#include "third_party/blink/public/common/common_export.h"

namespace blink {
namespace cache_storage {

// Create a trace ID for a cache_storage operation.  The ID value is
// guaranteed to be globally unique across all processes and threads.
// It can be used to trace across process boundaries.  When passing to
// a TRACE_EVENT* macro it should be wrapped in TRACE_ID_GLOBAL().
BLINK_COMMON_EXPORT int64_t CreateTraceId();

}  // namespace cache_storage
}  // namespace blink

#endif  // THIRD_PARTY_BLINK_PUBLIC_COMMON_CACHE_STORAGE_CACHE_STORAGE_UTILS_H_
