/*
 * Copyright (C) 2018 The Android Open Source Project
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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_JSON_JSON_TRACE_PARSER_IMPL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_JSON_JSON_TRACE_PARSER_IMPL_H_

#include <cstdint>
#include <string>

#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/importers/systrace/systrace_line.h"
#include "src/trace_processor/importers/systrace/systrace_line_parser.h"
#include "src/trace_processor/storage/trace_storage.h"

namespace Json {
class Value;
}

namespace perfetto::trace_processor {

class TraceProcessorContext;

// Parses legacy chrome JSON traces. The support for now is extremely rough
// and supports only explicit TRACE_EVENT_BEGIN/END events.
class JsonTraceParserImpl : public JsonTraceParser {
 public:
  explicit JsonTraceParserImpl(TraceProcessorContext*);
  ~JsonTraceParserImpl() override;

  // TraceParser implementation.
  void ParseJsonPacket(int64_t, std::string) override;
  void ParseSystraceLine(int64_t, SystraceLine) override;
  void ParseLegacyV8ProfileEvent(int64_t, LegacyV8CpuProfileEvent) override;

 private:
  TraceProcessorContext* const context_;
  SystraceLineParser systrace_line_parser_;

  void MaybeAddFlow(TrackId track_id, const Json::Value& event);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_JSON_JSON_TRACE_PARSER_IMPL_H_
