// Copyright 2021 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACING_TRACING_TLS_H_
#define BASE_TRACING_TRACING_TLS_H_

#include "base/base_export.h"

namespace base {
namespace tracing {

// Returns a thread-local flag that records whether the calling thread is
// running trace event related code. This is used to avoid writing trace events
// re-entrantly.
BASE_EXPORT bool* GetThreadIsInTraceEvent();

}  // namespace tracing
}  // namespace base

#endif  // BASE_TRACING_TRACING_TLS_H_
