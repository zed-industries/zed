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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MEMORY_TRACKER_SNAPSHOT_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MEMORY_TRACKER_SNAPSHOT_MODULE_H_

#include "src/trace_processor/importers/proto/memory_tracker_snapshot_parser.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto {
namespace trace_processor {

class MemoryTrackerSnapshotModule : public ProtoImporterModule {
 public:
  explicit MemoryTrackerSnapshotModule(TraceProcessorContext* context);

  ~MemoryTrackerSnapshotModule() override;

  void ParseTracePacketData(const protos::pbzero::TracePacket_Decoder& decoder,
                            int64_t ts,
                            const TracePacketData&,
                            uint32_t field_id) override;

  void NotifyEndOfFile() override;

 private:
  MemoryTrackerSnapshotParser parser_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_MEMORY_TRACKER_SNAPSHOT_MODULE_H_
