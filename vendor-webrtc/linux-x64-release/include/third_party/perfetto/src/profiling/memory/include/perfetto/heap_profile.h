/*
 * Copyright (C) 2020 The Android Open Source Project
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

// API to report allocations to heapprofd. This allows users to see the
// callstacks causing these allocations in heap profiles.
//
// In the context of this API, a "heap" is memory associated with an allocator.
// An example of an allocator is the malloc-family of libc functions (malloc /
// calloc / posix_memalign).
//
// A very simple custom allocator would look like this:
//
// void* my_malloc(size_t size) {
//   void* ptr = [code to somehow allocate get size bytes];
//   return ptr;
// }
//
// void my_free(void* ptr) {
//   [code to somehow free ptr]
// }
//
// To find out where in a program these two functions get called, we instrument
// the allocator using this API:
//
// static uint32_t g_heap_id =
//   AHeapProfile_registerHeap(AHeapInfo_create("invalid.example"));
//
// void* my_malloc(size_t size) {
//   void* ptr = [code to somehow allocate get size bytes];
//   AHeapProfile_reportAllocation(g_heap_id, static_cast<uintptr_t>(ptr),
//                                 size);
//   return ptr;
// }
//
// void my_free(void* ptr) {
//   AHeapProfile_reportFree(g_heap_id, static_cast<uintptr_t>(ptr));
//   [code to somehow free ptr]
// }
//
// This will allow users to get a flamegraph of the callstacks calling into
// these functions.
//
// See https://perfetto.dev/docs/data-sources/native-heap-profiler for more
// information on heapprofd in general.

#ifndef SRC_PROFILING_MEMORY_INCLUDE_PERFETTO_HEAP_PROFILE_H_
#define SRC_PROFILING_MEMORY_INCLUDE_PERFETTO_HEAP_PROFILE_H_

#include <stdlib.h>

#include <cinttypes>

#pragma GCC diagnostic push

#if defined(__clang__)
#pragma GCC diagnostic ignored "-Wnullability-extension"
#else
#define _Nullable
#define _Nonnull
#endif

// Maximum size of heap name, including NUL-byte.
#define HEAPPROFD_HEAP_NAME_SZ 64

#ifdef __cplusplus
extern "C" {
#endif

typedef struct AHeapInfo AHeapInfo;
typedef struct AHeapProfileEnableCallbackInfo AHeapProfileEnableCallbackInfo;
typedef struct AHeapProfileDisableCallbackInfo AHeapProfileDisableCallbackInfo;

typedef void (*_Nonnull AHeapInfo_EnableCallback)(
    void* _Nullable data,
    const AHeapProfileEnableCallbackInfo* _Nonnull session_info);

typedef void (*_Nonnull AHeapInfo_DisableCallback)(
    void* _Nullable data,
    const AHeapProfileDisableCallbackInfo* _Nonnull session_info);

// Get sampling interval (in bytes) of the profiling session that was started.
uint64_t AHeapProfileEnableCallbackInfo_getSamplingInterval(
    const AHeapProfileEnableCallbackInfo* _Nonnull session_info);

// Create new AHeapInfo, a struct describing a heap.
//
// Takes name of the heap, up to 64 bytes including null terminator. To
// guarantee uniqueness, this should include the caller's domain name,
// e.g. "dev.perfetto.largeobjects".
//
// On error, returns NULL.
// Errors are:
//  * Empty or too long (larger than 64 bytes including null terminator)
//    heap_name.
//  * Too many heaps have been registered in this process already.
//
// Must eventually be passed to AHeapProfile_registerHeap.
AHeapInfo* _Nullable AHeapInfo_create(const char* _Nonnull heap_name);

// Set enabled callback in AHeapInfo.
//
// If info is NULL, do nothing.
//
// After this AHeapInfo is registered via AHeapProfile_registerHeap,
// this callback is called when profiling of the heap is requested.
AHeapInfo* _Nullable AHeapInfo_setEnabledCallback(
    AHeapInfo* _Nullable info,
    AHeapInfo_EnableCallback callback,
    void* _Nullable data);

// Set disabled callback in AHeapInfo.
//
// If info is NULL, do nothing.
//
// After this AHeapInfo is registered via AHeapProfile_registerHeap,
// this callback is called when profiling of the heap ends.
AHeapInfo* _Nullable AHeapInfo_setDisabledCallback(
    AHeapInfo* _Nullable info,
    AHeapInfo_DisableCallback callback,
    void* _Nullable data);

// Register heap described in AHeapInfo.
//
// If info is NULL, return a no-op heap_id.
//
// The returned heap_id can be used in AHeapProfile_reportAllocation and
// AHeapProfile_reportFree.
//
// Takes ownership of |info|.
uint32_t AHeapProfile_registerHeap(AHeapInfo* _Nullable info);

// Reports an allocation of |size| on the given |heap_id|.
//
// The |alloc_id| needs to be a unique identifier for the allocation, and can
// can be used in AHeapProfile_reportFree to report the allocation has been
// freed.
//
// If a profiling session is active, this function decides whether the reported
// allocation should be sampled. If the allocation is sampled, it will be
// associated to the current callstack in the profile.
//
// Returns whether the allocation was sampled.
bool AHeapProfile_reportAllocation(uint32_t heap_id,
                                   uint64_t alloc_id,
                                   uint64_t size);

// Reports a sample of |size| on the given |heap_id|.
//
// If a profiling session is active, this function associates the sample with
// the current callstack in the profile.
//
// Returns whether the profiling session was active.
//
// THIS IS GENERALLY NOT WHAT YOU WANT. THIS IS ONLY NEEDED IF YOU NEED TO
// DO THE SAMPLING YOURSELF FOR PERFORMANCE REASONS.
// USE AHeapProfile_reportAllocation TO REPORT AN ALLOCATION AND LET
// HEAPPROFD DO THE SAMPLING.
//
// TODO(fmayer): Make this unavailable to non-Mainline.
bool AHeapProfile_reportSample(uint32_t heap_id,
                               uint64_t alloc_id,
                               uint64_t size);

// Report allocation was freed on the given heap.
//
// If |alloc_id| was sampled in a previous call to
// AHeapProfile_reportAllocation, this allocation is marked as freed in the
// profile.
//
// It is allowed to call with an |alloc_id| that was either not sampled or never
// passed to AHeapProfile_reportAllocation, in which case the call will not
// change the output.
void AHeapProfile_reportFree(uint32_t heap_id, uint64_t alloc_id);

#ifdef __cplusplus
}
#endif

#pragma GCC diagnostic pop

#endif  // SRC_PROFILING_MEMORY_INCLUDE_PERFETTO_HEAP_PROFILE_H_
