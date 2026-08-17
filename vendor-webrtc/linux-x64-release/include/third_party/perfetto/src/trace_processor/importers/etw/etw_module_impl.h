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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_MODULE_IMPL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_MODULE_IMPL_H_

#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/etw/etw_module.h"
#include "src/trace_processor/importers/etw/etw_parser.h"
#include "src/trace_processor/importers/etw/etw_tokenizer.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto {
namespace trace_processor {

class TraceBlobView;

class EtwModuleImpl : public EtwModule {
 public:
  explicit EtwModuleImpl(TraceProcessorContext* context);

  ModuleResult TokenizePacket(
      const protos::pbzero::TracePacket::Decoder& decoder,
      TraceBlobView* packet,
      int64_t packet_timestamp,
      RefPtr<PacketSequenceStateGeneration> state,
      uint32_t field_id) override;

  void ParseEtwEventData(uint32_t cpu,
                         int64_t ts,
                         const TracePacketData& data) override {
    base::Status res = parser_.ParseEtwEvent(cpu, ts, data);
    if (!res.ok()) {
      PERFETTO_ELOG("%s", res.message().c_str());
    }
  }

 private:
  EtwTokenizer tokenizer_;
  EtwParser parser_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_MODULE_IMPL_H_
