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

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_HEAP_BUFFER_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_HEAP_BUFFER_H_

#include "perfetto/public/abi/stream_writer_abi.h"

#ifdef __cplusplus
extern "C" {
#endif

// A PerfettoHeapBuffer can be used to serialize protobuf data using the
// PerfettoStreamWriter interface. Stores data on heap allocated buffers, which
// can be read back with PerfettoHeapBufferCopyInto().

struct PerfettoHeapBuffer;

// Creates a PerfettoHeapBuffer. Takes a pointer to an (uninitialized)
// PerfettoStreamWriter (owned by the caller). The stream writer can be user
// later to serialize protobuf data.
PERFETTO_SDK_EXPORT struct PerfettoHeapBuffer* PerfettoHeapBufferCreate(
    struct PerfettoStreamWriter*);

// Copies data from the heap buffer to `dst` (up to `size` bytes).
PERFETTO_SDK_EXPORT void PerfettoHeapBufferCopyInto(
    struct PerfettoHeapBuffer*,
    struct PerfettoStreamWriter*,
    void* dst,
    size_t size);

// Destroys the heap buffer.
PERFETTO_SDK_EXPORT void PerfettoHeapBufferDestroy(
    struct PerfettoHeapBuffer*,
    struct PerfettoStreamWriter*);

#ifdef __cplusplus
}
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_HEAP_BUFFER_H_
