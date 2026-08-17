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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_APP_WAKELOCK_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_APP_WAKELOCK_MODULE_H_

#include "perfetto/protozero/scattered_heap_buffer.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor {

class AppWakelockModule : public ProtoImporterModule {
 public:
  explicit AppWakelockModule(TraceProcessorContext* context);
  ~AppWakelockModule() override = default;

  // Tokenize and de-intern WakelockBundles so that bundles of multiple
  // packets are sorted appropriately.
  ModuleResult TokenizePacket(
      const protos::pbzero::TracePacket::Decoder& decoder,
      TraceBlobView* packet,
      int64_t ts,
      RefPtr<PacketSequenceStateGeneration> state,
      uint32_t field_id) override;

  void ParseTracePacketData(const protos::pbzero::TracePacket::Decoder& decoder,
                            int64_t ts,
                            const TracePacketData&,
                            uint32_t field_id) override;

 private:
  void ParseWakelockBundle(int64_t ts, protozero::ConstBytes blob);

  // Helper to simplify pushing a TracePacket to the sorter. The caller fills in
  // the packet buffer and uses this to push for sorting and reset the buffer.
  void PushPacketBufferForSort(int64_t timestamp,
                               RefPtr<PacketSequenceStateGeneration> state);

  TraceProcessorContext* context_;
  protozero::HeapBuffered<protos::pbzero::TracePacket> packet_buffer_;

  const StringId arg_flags_;
  const StringId arg_owner_pid_;
  const StringId arg_owner_uid_;
  const StringId arg_work_uid_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_APP_WAKELOCK_MODULE_H_
