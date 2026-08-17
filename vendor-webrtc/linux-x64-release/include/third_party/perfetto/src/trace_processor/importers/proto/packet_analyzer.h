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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_ANALYZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_ANALYZER_H_

#include <utility>
#include <vector>

#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/destructible.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

// Interface for processing packet information.
class PacketAnalyzer : public Destructible {
 public:
  using SampleAnnotation = std::vector<std::pair<StringId, StringId>>;

  PacketAnalyzer() = default;
  ~PacketAnalyzer() override;

  static PacketAnalyzer* Get(TraceProcessorContext* context) {
    if (!context->content_analyzer)
      return nullptr;
    return static_cast<PacketAnalyzer*>(context->content_analyzer.get());
  }

  virtual void ProcessPacket(const TraceBlobView& packet,
                             const SampleAnnotation& packet_annotation) = 0;

  virtual void NotifyEndOfFile() = 0;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PACKET_ANALYZER_H_
