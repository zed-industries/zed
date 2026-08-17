/*
 * Copyright (C) 2022 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_TRACING_SESSION_ABI_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_TRACING_SESSION_ABI_H_

#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>

#include "perfetto/public/abi/export.h"

#ifdef __cplusplus
extern "C" {
#endif

// Opaque pointer to the internal representation of a tracing session.
struct PerfettoTracingSessionImpl;

PERFETTO_SDK_EXPORT struct PerfettoTracingSessionImpl*
PerfettoTracingSessionSystemCreate(void);

PERFETTO_SDK_EXPORT struct PerfettoTracingSessionImpl*
PerfettoTracingSessionInProcessCreate(void);

PERFETTO_SDK_EXPORT void PerfettoTracingSessionSetup(
    struct PerfettoTracingSessionImpl*,
    void* cfg_begin,
    size_t cfg_len);

typedef void (*PerfettoTracingSessionStopCb)(struct PerfettoTracingSessionImpl*,
                                             void* user_arg);

// Calls `*cb` with `user_arg` when the tracing session is stopped.
PERFETTO_SDK_EXPORT void PerfettoTracingSessionSetStopCb(
    struct PerfettoTracingSessionImpl*,
    PerfettoTracingSessionStopCb cb,
    void* user_arg);

PERFETTO_SDK_EXPORT void PerfettoTracingSessionStartAsync(
    struct PerfettoTracingSessionImpl*);

PERFETTO_SDK_EXPORT void PerfettoTracingSessionStartBlocking(
    struct PerfettoTracingSessionImpl*);

PERFETTO_SDK_EXPORT void PerfettoTracingSessionStopAsync(
    struct PerfettoTracingSessionImpl*);

PERFETTO_SDK_EXPORT void PerfettoTracingSessionStopBlocking(
    struct PerfettoTracingSessionImpl*);

// Called back to signal that a previous flush request has completed. `success`
// is true if every data source has acknowledged the flush request, false if the
// timeout has expired or there was an error.
typedef void (*PerfettoTracingSessionFlushCb)(
    struct PerfettoTracingSessionImpl*,
    bool success,
    void* user_arg);

// Issues a flush request, asking all data sources to ack the request, within
// the specified timeout. A "flush" is a fence to ensure visibility of data in
// the async tracing pipeline. It guarantees that all data written before the
// call will be visible in the trace buffer and hence by the
// PerfettoTracingSessionReadTraceBlocking() function. Returns immediately and
// invokes a callback when the flush request is complete.
// Args:
//  `cb`: will be invoked on an internal perfetto thread when all data
//    sources have acked, or the timeout is reached.
//  `user_arg`: passed as is to `cb`.
//  `timeout_ms`: how much time the service will wait for data source acks. If
//    0, the global timeout specified in the TraceConfig (flush_timeout_ms)
//    will be used. If flush_timeout_ms is also unspecified, a default value
//    of 5s will be used.
PERFETTO_SDK_EXPORT void PerfettoTracingSessionFlushAsync(
    struct PerfettoTracingSessionImpl*,
    uint32_t timeout_ms,
    PerfettoTracingSessionFlushCb cb,
    void* user_arg);

// Like PerfettoTracingSessionFlushAsync(), but blocks until the flush is
// complete (i.e. every data source has acknowledged or the timeout has
// expired).
PERFETTO_SDK_EXPORT bool PerfettoTracingSessionFlushBlocking(
    struct PerfettoTracingSessionImpl*,
    uint32_t timeout_ms);

// Called back to read pieces of tracing data. `data` points to a chunk of trace
// data, `size` bytes long. `has_more` is true if there is more tracing data and
// the callback will be invoked again.
typedef void (*PerfettoTracingSessionReadCb)(struct PerfettoTracingSessionImpl*,
                                             const void* data,
                                             size_t size,
                                             bool has_more,
                                             void* user_arg);

// Repeatedly calls cb with data from the tracing session. `user_arg` is passed
// as is to the callback.
PERFETTO_SDK_EXPORT void PerfettoTracingSessionReadTraceBlocking(
    struct PerfettoTracingSessionImpl*,
    PerfettoTracingSessionReadCb cb,
    void* user_arg);

PERFETTO_SDK_EXPORT void PerfettoTracingSessionDestroy(
    struct PerfettoTracingSessionImpl*);

#ifdef __cplusplus
}
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_TRACING_SESSION_ABI_H_
