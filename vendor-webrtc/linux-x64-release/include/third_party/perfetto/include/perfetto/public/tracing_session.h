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

#ifndef INCLUDE_PERFETTO_PUBLIC_TRACING_SESSION_H_
#define INCLUDE_PERFETTO_PUBLIC_TRACING_SESSION_H_

#include "perfetto/public/abi/backend_type.h"
#include "perfetto/public/abi/tracing_session_abi.h"

static inline struct PerfettoTracingSessionImpl* PerfettoTracingSessionCreate(
    PerfettoBackendTypes backend) {
  if (backend == PERFETTO_BACKEND_IN_PROCESS) {
    return PerfettoTracingSessionInProcessCreate();
  }
  if (backend == PERFETTO_BACKEND_SYSTEM) {
    return PerfettoTracingSessionSystemCreate();
  }
  return nullptr;
}

#endif  // INCLUDE_PERFETTO_PUBLIC_TRACING_SESSION_H_
