/*
 * Copyright (C) 2023 The Android Open Source Project
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

#ifndef INCLUDE_PERFETTO_PUBLIC_ABI_THREAD_UTILS_ABI_H_
#define INCLUDE_PERFETTO_PUBLIC_ABI_THREAD_UTILS_ABI_H_

#include <stdint.h>

#include "perfetto/public/abi/export.h"

#ifdef __cplusplus
extern "C" {
#endif

#if defined(__Fuchsia__)
#include <zircon/types.h>
#elif defined(__linux__)
#include <sys/types.h>
#include <unistd.h>
#elif defined(__APPLE__) || defined(_WIN32)
#include <stdint.h>
#else
#include <pthread.h>
#endif

#if defined(__linux__) || defined(__native_client__)
typedef pid_t PerfettoThreadId;
#elif defined(__Fuchsia__)
typedef zx_koid_t PerfettoThreadId;
#elif defined(__APPLE__) || defined(_WIN32)
typedef uint64_t PerfettoThreadId;
#else
typedef pthread_t PerfettoThreadId;
#endif

PERFETTO_SDK_EXPORT PerfettoThreadId PerfettoGetThreadIdImpl(void);

#ifdef __cplusplus
}
#endif

#endif  // INCLUDE_PERFETTO_PUBLIC_ABI_THREAD_UTILS_ABI_H_
