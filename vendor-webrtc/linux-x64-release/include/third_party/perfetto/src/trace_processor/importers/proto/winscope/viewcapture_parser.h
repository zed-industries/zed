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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_VIEWCAPTURE_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_VIEWCAPTURE_PARSER_H_

#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/importers/proto/winscope/viewcapture_args_parser.h"
#include "src/trace_processor/util/descriptors.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

namespace perfetto::trace_processor {

class TraceProcessorContext;

class ViewCaptureParser {
 public:
  explicit ViewCaptureParser(TraceProcessorContext*);
  void Parse(int64_t timestamp,
             protozero::ConstBytes,
             PacketSequenceStateGeneration*);

 private:
  void ParseView(int64_t timestamp,
                 protozero::ConstBytes blob,
                 tables::ViewCaptureTable::Id,
                 PacketSequenceStateGeneration*);

  void AddDeinternedData(const ViewCaptureArgsParser&, uint32_t);

  TraceProcessorContext* const context_;
  util::ProtoToArgsParser args_parser_;
};
}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_VIEWCAPTURE_PARSER_H_
