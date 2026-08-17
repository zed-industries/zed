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

#ifndef SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_ANDROID_INPUT_EVENT_PARSER_H_
#define SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_ANDROID_INPUT_EVENT_PARSER_H_

#include <cstdint>
#include "perfetto/base/build_config.h"
#include "protos/perfetto/trace/trace_packet.pbzero.h"
#include "src/trace_processor/importers/common/parser_types.h"
#include "src/trace_processor/importers/proto/proto_importer_module.h"
#include "src/trace_processor/util/descriptors.h"
#include "src/trace_processor/util/proto_to_args_parser.h"

namespace perfetto::trace_processor {

class AndroidInputEventParser {
 public:
  explicit AndroidInputEventParser(TraceProcessorContext* context);

  void ParseAndroidInputEvent(int64_t packet_ts,
                              const protozero::ConstBytes& bytes);

 private:
  TraceProcessorContext& context_;
  util::ProtoToArgsParser args_parser_;

  void ParseMotionEvent(int64_t packet_ts, const protozero::ConstBytes& bytes);
  void ParseKeyEvent(int64_t packet_ts, const protozero::ConstBytes& bytes);
  void ParseWindowDispatchEvent(int64_t packet_ts,
                                const protozero::ConstBytes& bytes);
};

}  // namespace perfetto::trace_processor

#endif  // SRC_TRACE_PROCESSOR_IMPORTERS_PROTO_WINSCOPE_ANDROID_INPUT_EVENT_PARSER_H_
