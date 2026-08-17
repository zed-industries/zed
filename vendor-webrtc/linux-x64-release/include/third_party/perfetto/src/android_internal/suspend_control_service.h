/*
 * Copyright (C) 2025 The Android Open Source Project
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

#ifndef SRC_ANDROID_INTERNAL_SUSPEND_CONTROL_SERVICE_H_
#define SRC_ANDROID_INTERNAL_SUSPEND_CONTROL_SERVICE_H_

#include <stddef.h>
#include <stdint.h>

// This header declares proxy functions defined in
// libperfetto_android_internal.so that allow traced_probes to access internal
// android functions (e.g., hwbinder).
// Do not add any include to either perfetto headers or android headers. See
// README.md for more.

namespace perfetto {
namespace android_internal {

struct KernelWakelock {
  char wakelock_name[64];
  bool is_kernel;
  uint64_t total_time_ms;
};

extern "C" {

// These functions are not thread safe unless specified otherwise.

bool __attribute__((visibility("default"))) GetKernelWakelocks(
    KernelWakelock* wakelocks,
    size_t* size_of_arr);

}  // extern "C"

}  // namespace android_internal
}  // namespace perfetto

#endif  // SRC_ANDROID_INTERNAL_SUSPEND_CONTROL_SERVICE_H_
