/*
 * Copyright (C) 2024 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MULTI_MACHINE_TRACE_MANAGER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MULTI_MACHINE_TRACE_MANAGER_H_

#include <memory>
#include <vector>

#include "perfetto/ext/base/flat_hash_map.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;
class ProtoTraceReader;

// This class provides the get-or-create function for ProtoTraceReader to
// support multi-machine tracing. When the default ProtoTraceReader instance
// decodes a trace packet with a non-default machine ID:
//
// packet {
//    ftrace_events {
//    }
//    machine_id: 1001
// }
//
// An object graph rooted from a new ProtoTraceReader is created for the
// machine:
//
// ProtoTraceReader -> TraceProcessorContext (with a non-null machine_id).
//                     +--> TraceProcessorStorage (shared with the default
//                     instance)
//                     |--> TraceSorter (shared with the default instance)
//                     |--> TrackTracker (created for machine 1001)
//                     |--> ProcessTracker (created for machine 1001)
//                     |--> ... other data members rooted from
//                     TraceProcessorContext
//
// and the new ProtoTraceReader is used to parse all trace packet with the same
// machine ID. The context is used to insert the machine ID into the sqlite
// tables. for query in the trace processor or from the UI frontend.
class MultiMachineTraceManager {
 public:
  // RawMachineId is the value of 'machine_id' in trace packets.
  using RawMachineId = uint32_t;

  explicit MultiMachineTraceManager(TraceProcessorContext* default_context);
  ~MultiMachineTraceManager();

  // Get or create an instance of ProtoTraceReader for parsing the trace packets
  // with the RawMachineId from the trace packet.
  ProtoTraceReader* GetOrCreateReader(RawMachineId);

  using ProtoImporterModuleFactory = void (*)(TraceProcessorContext*);
  void EnableAdditionalModules(ProtoImporterModuleFactory);

 private:
  struct RemoteMachineContext {
    std::unique_ptr<TraceProcessorContext> context;
    std::unique_ptr<ProtoTraceReader> reader;
  };

  std::unique_ptr<TraceProcessorContext> CreateContext(RawMachineId);

  // The default TraceProcessorContext instance.
  TraceProcessorContext* default_context_;
  // Owns contexts for remote machines.
  base::FlatHashMap<RawMachineId, RemoteMachineContext>
      remote_machine_contexts_;

  ProtoImporterModuleFactory additional_modules_factory_ = nullptr;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MULTI_MACHINE_TRACE_MANAGER_H_
