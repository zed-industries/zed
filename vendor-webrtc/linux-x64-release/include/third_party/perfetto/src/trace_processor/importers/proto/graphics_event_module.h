/*
 * Copyright (C) 2019 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_GRAPHICS_EVENT_MODULE_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_GRAPHICS_EVENT_MODULE_H_

#include <cstdint>
#include "perfetto/base/build_config.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/proto/frame_timeline_event_parser.h"
#include "src/trace_processor/importers/proto/gpu_event_parser.h"
#include "src/trace_processor/importers/proto/graphics_frame_event_parser.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"

#include "protos/perfetto/trace/trace_packet.pbzero.h"

namespace perfetto {
namespace trace_processor {

class GraphicsEventModule : public ProtoImporterModule {
 public:
  explicit GraphicsEventModule(TraceProcessorContext* context);

  ~GraphicsEventModule() override;

  void ParseTracePacketData(const protos::pbzero::TracePacket::Decoder&,
                            int64_t ts,
                            const TracePacketData&,
                            uint32_t field_id) override;

 private:
  GpuEventParser parser_;
  GraphicsFrameEventParser frame_parser_;
  FrameTimelineEventParser frame_timeline_parser_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_GRAPHICS_EVENT_MODULE_H_
