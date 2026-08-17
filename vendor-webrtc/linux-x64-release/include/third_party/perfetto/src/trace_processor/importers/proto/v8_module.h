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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_MODULE_H_

#include <cstdint>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/protozero/field.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/tables/v8_tables_py.h"

namespace perfetto {
namespace protos {
namespace pbzero {
class TracePacket_Decoder;
}
}  // namespace protos
namespace trace_processor {

class PacketSequenceState;
struct TracePacketData;
class V8Tracker;

// Populates v8 related tables.
//
// This class processes v8 related trace packets and populates the various
// tables. In particular it keeps track of v8 Isolates and what code and
// associated debug information has been loaded in each of the isolates.
class V8Module : public ProtoImporterModule {
 public:
  explicit V8Module(TraceProcessorContext* context);

  ~V8Module() override;

  ModuleResult TokenizePacket(
      const protos::pbzero::TracePacket_Decoder& decoder,
      TraceBlobView* packet,
      int64_t packet_timestamp,
      RefPtr<PacketSequenceStateGeneration> state,
      uint32_t field_id) override;

  void ParseTracePacketData(const protos::pbzero::TracePacket_Decoder&,
                            int64_t ts,
                            const TracePacketData& packet_data,
                            uint32_t field_id) override;

 private:
  void ParseV8JsCode(protozero::ConstBytes bytes,
                     int64_t ts,
                     const TracePacketData& data);
  void ParseV8InternalCode(protozero::ConstBytes bytes,
                           int64_t ts,
                           const TracePacketData& data);
  void ParseV8WasmCode(protozero::ConstBytes bytes,
                       int64_t ts,
                       const TracePacketData& data);
  void ParseV8RegExpCode(protozero::ConstBytes bytes,
                         int64_t ts,
                         const TracePacketData& data);
  void ParseV8CodeMove(protozero::ConstBytes bytes,
                       int64_t ts,
                       const TracePacketData& data);

  // Determine the utid for a code event.
  // If the passed in decoder has no tid field this method will try the
  // TracePacketDefaults.
  template <typename CodeDecoder>
  std::optional<UniqueTid> GetUtid(PacketSequenceStateGeneration& generation,
                                   tables::V8IsolateTable::Id isolate_id,
                                   const CodeDecoder& code);
  std::optional<uint32_t> GetDefaultTid(
      PacketSequenceStateGeneration& generation) const;
  TraceProcessorContext* const context_;
  V8Tracker* const v8_tracker_;
  // Caches isolate to pid associations. Used to compute the utid for code
  // events.
  base::FlatHashMap<tables::V8IsolateTable::Id, uint32_t> isolate_to_pid_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_V8_MODULE_H_
