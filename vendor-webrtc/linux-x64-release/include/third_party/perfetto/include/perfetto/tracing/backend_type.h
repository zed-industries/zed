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

#ifndef INCLUDE_PERFETTO_TRACING_BACKEND_TYPE_H_
#define INCLUDE_PERFETTO_TRACING_BACKEND_TYPE_H_

#include <stdint.h>

namespace perfetto {

enum BackendType : uint32_t {
  kUnspecifiedBackend = 0,

  // Connects to a previously-initialized perfetto tracing backend for
  // in-process. If the in-process backend has not been previously initialized
  // it will do so and create the tracing service on a dedicated thread.
  kInProcessBackend = 1 << 0,

  // Connects to the system tracing service (e.g. on Linux/Android/Mac uses a
  // named UNIX socket).
  kSystemBackend = 1 << 1,

  // Used to provide a custom IPC transport to connect to the service.
  // TracingInitArgs::custom_backend must be non-null and point to an
  // indefinitely lived instance.
  kCustomBackend = 1 << 2,
};

}  // namespace perfetto

#endif  // INCLUDE_PERFETTO_TRACING_BACKEND_TYPE_H_
