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

#ifndef INCLUDE_PERFETTO_PUBLIC_PRODUCER_H_
#define INCLUDE_PERFETTO_PUBLIC_PRODUCER_H_

#include <stdint.h>

#include "perfetto/public/abi/backend_type.h"
#include "perfetto/public/abi/producer_abi.h"
#include "perfetto/public/compiler.h"

// Arguments for PerfettoProducerInit. This struct is not ABI-stable, fields can
// be added and rearranged.
struct PerfettoProducerInitArgs {
  // Bitwise-or of backends that should be enabled.
  PerfettoBackendTypes backends;

  // [Optional] Tune the size of the shared memory buffer between the current
  // process and the service backend(s). This is a trade-off between memory
  // footprint and the ability to sustain bursts of trace writes (see comments
  // in shared_memory_abi.h).
  // If set, the value must be a multiple of 4KB. The value can be ignored if
  // larger than kMaxShmSize (32MB) or not a multiple of 4KB.
  uint32_t shmem_size_hint_kb;
};

// Initializes a PerfettoProducerInitArgs struct.
#define PERFETTO_PRODUCER_INIT_ARGS_INIT() {0, 0}

// Initializes the global perfetto producer.
//
// It's ok to call this function multiple times, but if a backend was already
// initialized, most of `args` would be ignored.
static inline void PerfettoProducerInit(struct PerfettoProducerInitArgs args) {
  struct PerfettoProducerBackendInitArgs* backend_args =
      PerfettoProducerBackendInitArgsCreate();

  PerfettoProducerBackendInitArgsSetShmemSizeHintKb(backend_args,
                                                    args.shmem_size_hint_kb);

  if (args.backends & PERFETTO_BACKEND_IN_PROCESS) {
    PerfettoProducerInProcessInit(backend_args);
  }
  if (args.backends & PERFETTO_BACKEND_SYSTEM) {
    PerfettoProducerSystemInit(backend_args);
  }

  PerfettoProducerBackendInitArgsDestroy(backend_args);
}

// Informs the tracing services to activate the single trigger `trigger_name` if
// any tracing session was waiting for it.
//
// Sends the trigger signal to all the initialized backends that are currently
// connected and that connect in the next `ttl_ms` milliseconds (but
// returns immediately anyway).
static inline void PerfettoProducerActivateTrigger(const char* trigger_name,
                                                   uint32_t ttl_ms) {
  const char* trigger_names[2];
  trigger_names[0] = trigger_name;
  trigger_names[1] = PERFETTO_NULL;
  PerfettoProducerActivateTriggers(trigger_names, ttl_ms);
}

#endif  // INCLUDE_PERFETTO_PUBLIC_PRODUCER_H_
