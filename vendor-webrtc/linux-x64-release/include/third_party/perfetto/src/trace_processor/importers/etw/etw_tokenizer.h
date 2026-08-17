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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_TOKENIZER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_TOKENIZER_H_

#include <optional>

#include "perfetto/base/status.h"
#include "perfetto/trace_processor/trace_blob_view.h"
#include "src/trace_processor/importers/proto/packet_sequence_state_generation.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

#include "protos/perfetto/trace/etw/etw_event_bundle.pbzero.h"

namespace perfetto {
namespace trace_processor {

class EtwTokenizer {
 public:
  explicit EtwTokenizer(TraceProcessorContext* context) : context_(context) {}

  base::Status TokenizeEtwBundle(TraceBlobView bundle,
                                 RefPtr<PacketSequenceStateGeneration> state);

 private:
  base::Status TokenizeEtwEvent(std::optional<uint32_t> fallback_cpu,
                                TraceBlobView event,
                                RefPtr<PacketSequenceStateGeneration> state);

  TraceProcessorContext* context_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_ETW_ETW_TOKENIZER_H_
