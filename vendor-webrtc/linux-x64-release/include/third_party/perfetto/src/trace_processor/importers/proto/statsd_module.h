/*
 * Copyright (C) 2022 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STATSD_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STATSD_MODULE_H_

#include <cstddef>
#include <cstdint>
#include <optional>

#include "perfetto/ext/base/flat_hash_map.h"
#include "perfetto/trace_processor/ref_counted.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"
#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"
#include "src/trace_processor/util/descriptors.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

namespace perfetto::trace_processor {

class StatsdModule : public ProtoImporterModule {
 public:
  explicit StatsdModule(TraceProcessorContext* context);

  ~StatsdModule() override;

  ModuleResult TokenizePacket(const protos::pbzero::TracePacket::Decoder&,
                              TraceBlobView* packet,
                              int64_t packet_timestamp,
                              RefPtr<PacketSequenceStateGeneration> state,
                              uint32_t field_id) override;

  void ParseTracePacketData(const protos::pbzero::TracePacket::Decoder& decoder,
                            int64_t ts,
                            const TracePacketData&,
                            uint32_t field_id) override;

 private:
  void ParseAtom(int64_t ts, protozero::ConstBytes bytes);
  StringId GetAtomName(uint32_t atom_field_id);
  TrackId InternTrackId();

  TraceProcessorContext* context_;
  base::FlatHashMap<uint32_t, StringId> atom_names_;
  const ProtoDescriptor* descriptor_ = nullptr;
  util::ProtoToArgsParser args_parser_;
  std::optional<TrackId> track_id_;
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_STATSD_MODULE_H_
