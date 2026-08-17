// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_TRACE_ID_HELPER_H_
#define BASE_TRACE_EVENT_TRACE_ID_HELPER_H_

#include <cstdint>

#include "base/base_export.h"

namespace base {
namespace trace_event {

// Returns an globally-unique id which can be used as a flow id or async event
// id. This is fast (powered by an memory-order-relaxed atomic int), so use this
// function instead of implementing your own counter and hashing it with a
// random value. However, consider using TRACE_ID_LOCAL(this) to avoid storing
// additional data if possible.
BASE_EXPORT uint64_t GetNextGlobalTraceId();

}  // namespace trace_event
}  // namespace base

#endif  // BASE_TRACE_EVENT_TRACE_ID_HELPER_H_
