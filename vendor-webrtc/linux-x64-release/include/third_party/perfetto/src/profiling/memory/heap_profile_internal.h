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

#ifndef SRC_PROFILING_MEMORY_HEAP_PROFILE_INTERNAL_H_
#define SRC_PROFILING_MEMORY_HEAP_PROFILE_INTERNAL_H_

#include <stdlib.h>

#include <cinttypes>

#pragma GCC diagnostic push

#if defined(__clang__)
#pragma GCC diagnostic ignored "-Wnullability-extension"
#else
#define _Nullable
#define _Nonnull
#endif

#ifdef __cplusplus
extern "C" {
#endif

// Called by the standalone client or libc upon receipt of the profiling
// signal.
bool AHeapProfile_initSession(void* _Nullable (*_Nonnull malloc_fn)(size_t),
                              void (*_Nonnull free_fn)(void* _Nullable));

#ifdef __cplusplus
}
#endif

#pragma GCC diagnostic pop

#endif  // SRC_PROFILING_MEMORY_HEAP_PROFILE_INTERNAL_H_
