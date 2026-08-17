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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_GECKO_GECKO_TRACE_PARSER_IMPL_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_GECKO_GECKO_TRACE_PARSER_IMPL_H_

#include <cstdint>

#include "src/trace_processor/importers/common/trace_parser.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto::trace_processor::gecko_importer {

class GeckoTraceParserImpl : public GeckoTraceParser {
 public:
  explicit GeckoTraceParserImpl(TraceProcessorContext*);
  ~GeckoTraceParserImpl() override;

  void ParseGeckoEvent(int64_t ts, GeckoEvent) override;

 private:
  TraceProcessorContext* const context_;
};

}  // namespace perfetto::trace_processor::gecko_importer

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_GECKO_GECKO_TRACE_PARSER_IMPL_H_
