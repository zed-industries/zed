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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ANDROID_KERNEL_WAKELOCKS_STATE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ANDROID_KERNEL_WAKELOCKS_STATE_H_

#include <cstdint>
#include <string>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"

#include "protos/perfetto/trace/android/kernel_wakelock_data.pbzero.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

struct AndroidKernelWakelockState : PacketSequenceStateGeneration::CustomState {
  struct Metadata {
    std::string name;
    protos::pbzero::KernelWakelockData::Wakelock::Type type;
  };

  struct LastValue {
    uint64_t value;
    protos::pbzero::KernelWakelockData::Wakelock::Type type;
  };

  explicit AndroidKernelWakelockState(TraceProcessorContext*);
  ~AndroidKernelWakelockState() override;

  base::FlatHashMap<uint32_t, Metadata> wakelocks;
  base::FlatHashMap<std::string, LastValue> wakelock_last_values;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_ANDROID_KERNEL_WAKELOCKS_STATE_H_
