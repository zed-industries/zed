/*
 * Copyright (C) 2021 The Android Open Source Project
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

#ifndef SRC_ANDROID_INTERNAL_TRACING_SERVICE_PROXY_H_
#define SRC_ANDROID_INTERNAL_TRACING_SERVICE_PROXY_H_

#include <stdint.h>

namespace perfetto {
namespace android_internal {

extern "C" {

bool __attribute__((visibility("default"))) NotifyTraceSessionEnded(
    bool session_stolen);

bool __attribute__((visibility("default"))) ReportTrace(
    const char* reporter_package_name,
    const char* reporter_class_name,
    int owned_trace_fd,
    int64_t uuid_lsb,
    int64_t uuid_msb,
    bool use_pipe_in_framework_for_testing);

}  // extern "C"

}  // namespace android_internal
}  // namespace perfetto
#endif  // SRC_ANDROID_INTERNAL_TRACING_SERVICE_PROXY_H_
