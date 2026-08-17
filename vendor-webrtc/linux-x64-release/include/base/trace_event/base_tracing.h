// Copyright 2020 The Chromium Authors
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef BASE_TRACE_EVENT_BASE_TRACING_H_
#define BASE_TRACE_EVENT_BASE_TRACING_H_

// Proxy header that provides tracing instrumentation for //base code. When
// tracing support is disabled via the gn flag enable_base_tracing, this header
// provides a mock implementation of the relevant trace macros instead, which
// causes the instrumentation in //base to be compiled into no-ops.

#include "base/tracing_buildflags.h"

#if BUILDFLAG(ENABLE_BASE_TRACING)
// Update the check in //base/PRESUBMIT.py when adding new headers here.
// TODO(crbug.com/42050015): Switch to perfetto for trace event implementation.
#include "base/trace_event/heap_profiler.h"  // IWYU pragma: export, nogncheck
#include "base/trace_event/interned_args_helper.h"  // IWYU pragma: export, nogncheck
#include "base/trace_event/memory_allocator_dump_guid.h"  // IWYU pragma: export, nogncheck
#include "base/trace_event/memory_dump_manager.h"  // IWYU pragma: export, nogncheck
#include "base/trace_event/memory_dump_provider.h"  // IWYU pragma: export, nogncheck
#include "base/trace_event/trace_event.h"      // IWYU pragma: export, nogncheck
#include "base/trace_event/trace_id_helper.h"  // IWYU pragma: export, nogncheck
#include "base/trace_event/traced_value.h"     // IWYU pragma: export, nogncheck
#include "base/trace_event/typed_macros.h"     // IWYU pragma: export, nogncheck
#include "third_party/perfetto/include/perfetto/tracing/traced_value.h"  // IWYU pragma: export, nogncheck
#include "third_party/perfetto/protos/perfetto/trace/track_event/chrome_process_descriptor.pbzero.h"  // IWYU pragma: export, nogncheck
#include "third_party/perfetto/protos/perfetto/trace/track_event/log_message.pbzero.h"  // IWYU pragma: export, nogncheck
#include "third_party/perfetto/protos/perfetto/trace/track_event/task_execution.pbzero.h"  // IWYU pragma: export, nogncheck
#else  // BUILDFLAG(ENABLE_BASE_TRACING)
#include "base/trace_event/trace_event_stub.h"  // IWYU pragma: export
#endif  // BUILDFLAG(ENABLE_BASE_TRACING)

#endif  // BASE_TRACE_EVENT_BASE_TRACING_H_
