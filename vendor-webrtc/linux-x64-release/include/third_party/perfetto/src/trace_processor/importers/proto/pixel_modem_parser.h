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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PIXEL_MODEM_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PIXEL_MODEM_PARSER_H_

#include "perfetto/protozero/field.h"
#include "src/trace_processor/importers/proto/pigweed_detokenizer.h"
#include "src/trace_processor/storage/trace_storage.h"
#include "src/trace_processor/types/trace_processor_context.h"

namespace perfetto {
namespace trace_processor {

class TraceProcessorContext;

class PixelModemParser {
 public:
  explicit PixelModemParser(TraceProcessorContext* context);
  ~PixelModemParser();

  base::Status SetDatabase(protozero::ConstBytes);
  base::Status ParseEvent(int64_t, uint64_t, protozero::ConstBytes);

 private:
  TraceProcessorContext* context_ = nullptr;
  pigweed::PigweedDetokenizer detokenizer_;

  const StringId template_id_;
  const StringId token_id_;
  const StringId token_id_hex_;
  const StringId packet_timestamp_id_;
};

}  // namespace trace_processor
}  // namespace perfetto

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_PIXEL_MODEM_PARSER_H_
